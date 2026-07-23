use std::cell::UnsafeCell;

trait Animal {
    fn legs(&self) -> u32;
}

struct Dog;
struct Duck;

impl Animal for Dog {
    fn legs(&self) -> u32 {
        4
    }
}

impl Animal for Duck {
    fn legs(&self) -> u32 {
        2
    }
}

static PET: Dog = Dog;
static MODE: u32 = 1;
static mut COUNTER: u32 = 0;

struct Cell {
    inner: UnsafeCell<u32>,
}

unsafe impl Sync for Cell {}

static CELL: Cell = Cell {
    inner: UnsafeCell::new(0),
};

fn pet_legs() -> u32 {
    let a: &dyn Animal = &PET;
    a.legs()
}

fn const_mode() -> u32 {
    let a: &dyn Animal = if MODE == 0 { &Dog } else { &Duck };
    a.legs()
}

fn mut_mode() -> u32 {
    let n = unsafe { COUNTER };
    let a: &dyn Animal = if n == 0 { &Dog } else { &Duck };
    a.legs()
}

fn cell_mode() -> u32 {
    let n = unsafe { *CELL.inner.get() };
    let a: &dyn Animal = if n == 0 { &Dog } else { &Duck };
    a.legs()
}

fn main() {
    let a = pet_legs();
    let b = const_mode();
    let c = mut_mode();
    let d = cell_mode();

    println!("{} {} {} {}", a, b, c, d);
}

// `a.legs()` in pet_legs    -- CHA: 2 {Dog, Duck}. FSA: 1 {Dog}.
// `a.legs()` in const_mode  -- CHA: 2 {Dog, Duck}. FSA: 1 {Duck}, Dog branch pruned.
// `a.legs()` in mut_mode    -- CHA: 2 {Dog, Duck}. FSA: 2, static mut is not constrainable.
// `a.legs()` in cell_mode   -- CHA: 2 {Dog, Duck}. FSA: 2, UnsafeCell is not constrainable.
//
// The last two are controls: FSA < 2 there means static_check_const let something through.
