//! Writes a call makes into memory its caller can see, recorded so a memo hit
//! can replay them.
//!
//! A memoized call (exact_memo, virtual_call_memo) is keyed by its argument
//! *values*, and a pointer's value is its pointee's: two calls `f(&mut x)` and
//! `f(&mut y)` with `x` and `y` holding equal values have the same key. A hit
//! returns the cached result, but the callee's writes through its `&mut`
//! parameter went to `x` - the hit has to make them again, into `y`.
//!
//! Every such write leaves the callee along an alias path (see
//! ConstraintStore::resolve_mut_ref) that passes through one of the call's
//! *ports*: the caller-side places of its operands, e.g. the caller's `_5` in
//! `f(move _5)`, which aliases `x` in one call and `y` in the other. So while
//! a call is interpreted, a frame on a thread-local stack records each value
//! written through each port (by operand index), and through each static. A
//! hit replays them as writes to its own operands (InterpPass::replay_writes),
//! which the usual alias resolution routes to this call's places. Writes of
//! nested calls - interpreted or themselves replayed - pass through the
//! enclosing calls' ports too, so they land in every enclosing frame.
//!
//! Replays merge (weak update) even where the original write replaced the old
//! value: sound, if possibly less precise.
//!
//! A log keeps the distinct values written through each port, not their
//! merge: a write lands in every enclosing frame, so merging would allocate a
//! new value per frame per write (and memo entries keep them all alive - an
//! out-of-memory ripgrep run). Values are shared (`Constraints` is an `Rc`);
//! only a port with more than MAX_VALUES distinct values is merged.

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

use std::rc::Rc;

use rustc_public::mir::Place;
use rustc_public::DefId;

use crate::constraints::{Constraints, MapValue, VOID};
use crate::merge::merge_mapvals;

/// What one call wrote into memory its caller can see.
#[derive(Clone, Debug, Default)]
pub struct WriteLog {
    /// Operand index -> the distinct values written through it.
    pub ports: BTreeMap<usize, Vec<Constraints>>,
    /// Static -> the distinct values written to it.
    pub statics: HashMap<DefId, Vec<Constraints>>,
    /// The operands (by index) the call's return value aliases - e.g.
    /// `UnsafeCell::get(&self)`, `as_mut_ptr`, `RefMut::deref_mut` return a
    /// pointer to (part of) an argument's pointee. The caller's destination
    /// gets alias edges to them (InterpPass::take_return_aliases).
    pub returns: Vec<ReturnAlias>,
    /// Static -> its version (constraints::static_version) when the call
    /// first read or wrote it: the result is reusable only while every one
    /// is unchanged (reads_fresh).
    pub reads: HashMap<DefId, u64>,
}

/// What a call's return value points into (WriteLog::returns).
#[derive(Clone, Debug, PartialEq)]
pub enum ReturnAlias {
    /// The pointee of the call's operand at this index.
    Operand(usize),
    /// A place of a static (its synthetic scope, constraints::static_scope)
    /// - e.g. `static_init!`'s `&'static mut T` into its buffer; statics are
    /// the same place for every caller.
    Static(Place, VOID),
}

/// Up to how many distinct values a port keeps before they're merged.
const MAX_VALUES: usize = 8;

impl WriteLog {
    pub fn is_empty(&self) -> bool {
        self.ports.is_empty() && self.statics.is_empty() && self.returns.is_empty()
    }

    /// Number of logged values.
    pub fn len(&self) -> usize {
        self.ports.values().chain(self.statics.values()).map(Vec::len).sum()
    }
}

fn add(values: &mut Vec<Constraints>, value: &Constraints) {
    // Equality is a pointer check, then the cached hashes.
    if values.contains(value) {
        return;
    }
    values.push(value.clone());
    if values.len() > MAX_VALUES {
        let mut merged = values[0].clone();
        for v in &values[1..] {
            merged = match merge_mapvals(
                &MapValue::Constraints(merged),
                &MapValue::Constraints(v.clone()),
                None,
            ) {
                MapValue::Constraints(c) => c,
                MapValue::Store(..) => unreachable!("merging two values gave a store"),
            };
        }
        *values = vec![merged];
    }
}

struct Frame {
    id: u64,
    log: WriteLog,
}

#[derive(Default)]
struct Frames {
    stack: Vec<Frame>,
    /// Port -> (frame id, operand index) for every frame on the stack.
    ports: HashMap<(Place, VOID), Vec<(u64, usize)>>,
    /// Each frame's ports, to remove them when it's popped.
    port_keys: HashMap<u64, Vec<(Place, VOID)>>,
    next_id: u64,
}

thread_local! {
    static FRAMES: RefCell<Frames> = RefCell::new(Frames::default());
}

/// A call's frame on the stack: records its writes until `finish`, and is
/// removed when dropped (also on an error path or a recovered panic).
pub struct FrameGuard {
    id: u64,
    done: bool,
}

/// Starts recording the writes of a call whose operands are `ports` (the
/// caller-side places, in `caller`, of its place operands, by index).
pub fn push(ports: impl IntoIterator<Item = (usize, Place)>, caller: &VOID) -> FrameGuard {
    FRAMES.with(|f| {
        let mut f = f.borrow_mut();
        let id = f.next_id;
        f.next_id += 1;
        let mut keys = Vec::new();
        for (idx, place) in ports {
            let key = (place, caller.clone());
            f.ports.entry(key.clone()).or_default().push((id, idx));
            keys.push(key);
        }
        f.port_keys.insert(id, keys);
        f.stack.push(Frame { id, log: WriteLog::default() });
        FrameGuard { id, done: false }
    })
}

impl FrameGuard {
    /// Stops recording, returning what the call wrote.
    pub fn finish(mut self) -> WriteLog {
        self.done = true;
        pop(self.id)
    }
}

impl Drop for FrameGuard {
    fn drop(&mut self) {
        if !self.done {
            pop(self.id);
        }
    }
}

fn pop(id: u64) -> WriteLog {
    FRAMES.with(|f| {
        let mut f = f.borrow_mut();
        let mut out = WriteLog::default();
        // Normally the top frame; anything above it was left by a caller
        // that unwound without dropping its guard, and goes too.
        while let Some(frame) = f.stack.pop() {
            for key in f.port_keys.remove(&frame.id).unwrap_or_default() {
                if let Some(v) = f.ports.get_mut(&key) {
                    v.retain(|(fid, _)| *fid != frame.id);
                    if v.is_empty() {
                        f.ports.remove(&key);
                    }
                }
            }
            if frame.id == id {
                out = frame.log;
                break;
            }
        }
        out
    })
}

/// A write of `value` whose alias path visited `visited` (see
/// ConstraintStore::scoped_update): logged in every frame one of whose ports
/// is on the path.
pub fn record_write<'a>(visited: impl IntoIterator<Item = &'a (Place, VOID)>, value: &MapValue) {
    let MapValue::Constraints(value) = value else { return };
    FRAMES.with(|f| {
        let mut f = f.borrow_mut();
        if f.ports.is_empty() {
            return;
        }
        let hits: Vec<(u64, usize)> =
            visited.into_iter().filter_map(|p| f.ports.get(p)).flatten().copied().collect();
        if hits.is_empty() {
            return;
        }
        for frame in f.stack.iter_mut() {
            for &(fid, idx) in &hits {
                if fid == frame.id {
                    add(frame.log.ports.entry(idx).or_default(), value);
                }
            }
        }
    })
}

/// A write of `value` to the static `def`: logged in every frame.
pub fn record_static(def: DefId, value: &MapValue) {
    let MapValue::Constraints(value) = value else { return };
    FRAMES.with(|f| {
        for frame in f.borrow_mut().stack.iter_mut() {
            add(frame.log.statics.entry(def).or_default(), value);
        }
    })
}

/// A read of static `def` at `version` (or a write over that version):
/// noted in every frame that hasn't seen `def` yet. A frame's first touch of
/// a static sees its value from before the call - nothing else runs between.
pub fn record_static_read(def: DefId, version: u64) {
    FRAMES.with(|f| {
        for frame in f.borrow_mut().stack.iter_mut() {
            frame.log.reads.entry(def).or_insert(version);
        }
    })
}

/// A memo hit's reads, in the enclosing frames: their results depend on
/// them too.
pub fn replay_reads(log: &WriteLog) {
    if log.reads.is_empty() {
        return;
    }
    FRAMES.with(|f| {
        for frame in f.borrow_mut().stack.iter_mut() {
            for (def, v) in &log.reads {
                frame.log.reads.entry(*def).or_insert(*v);
            }
        }
    })
}

/// Whether every static the call read still has the version it read.
pub fn reads_fresh(log: &WriteLog) -> bool {
    log.reads.iter().all(|(def, v)| crate::constraints::static_version(*def) == *v)
}

/// Shared form stored in the memos.
pub type SharedLog = Rc<WriteLog>;
