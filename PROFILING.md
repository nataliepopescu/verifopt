# Profiling Notes

Goal: how to differentiate between statically and
dynamically dispatched function calls?

All tests/experiments are conducted on a Linux machine (6.6.50) with an AMD Ryzen 7 
3800XT 8-Core Processor, running NixOS 24.05 (Uakari).

## Profiling tools

Options taken from [this
list](https://nnethercote.github.io/perf-book/profiling.html).

### [perf](https://perfwiki.github.io/main/)

potential cons: language-agnostic

`perf list` lists all symbolic event types. has some with "dispatch" keyword but
unsure what this is referring to in this context. e.g.: 
- `uops_dispatched`: [Micro-ops Dispatched]

`perf top`:

```sh
Error: 
Access to performance monitoring and observability operations is limited.
Consider adjusting /proc/sys/kernel/perf_event_paranoid setting to open
access to performance monitoring and observability operations for processes
without CAP_PERFMON, CAP_SYS_PTRACE or CAP_SYS_ADMIN Linux capability.
More information can be found at 'Perf events and tool security' document:
https://www.kernel.org/doc/html/latest/admin-guide/perf-security.html
perf_event_paranoid setting is 2:
  -1: Allow use of (almost) all events by all users
      Ignore mlock limit after perf_event_mlock_kb without CAP_IPC_LOCK
>= 0: Disallow raw and ftrace function tracepoint access
>= 1: Disallow CPU event access
>= 2: Disallow kernel profiling
To make the adjusted perf_event_paranoid setting permanent preserve it
in /etc/sysctl.conf (e.g. kernel.perf_event_paranoid = <setting>)
```

- can either grant specific processes (`perf`) with specific capabilities
  (CAP_PERFMON seems sufficient?), OR change the underlying system setting to be
  less "paranoid"
    - tried to go the former route, but when I try to change the group of perf
      (`sudo chgrp perf_users perf`) I get the following error: 
      `chgrp: changing group of 'perf': Read-only file system`
    - [modify paranoid
      setting](https://discourse.nixos.org/t/how-do-i-set-perf-event-paranoid/15869)
        - temp vs permanent
    - before getting too in the weeds, let's see if other perf-based tools have
      a workaround for this/are easier to set up

### [flamegraph](https://github.com/flamegraph-rs/flamegraph)

on NixOs: `nix-shell -p rustup` then `rustup update` then `cargo build
--release` then `cargo flamegraph --release`

generates `flamegraph.svg` and `perf.data` which we can inspect with `perf` tool

## Inspecting IR

### debug build

`dyn_dp`: (line 570)
- 68 lines long including whitespace + comments
- lines 607 and 613 (in bb4 and bb3, respectively) seem to initialize vtables
    - syntax: `store ptr @vtable`
- could 622 be the virtual call?
    - bb5 seems to be getting the "a" info for the virt call
    - syntax: `invoke void %x` (note `%x` signals a variable rather than a
      static string which would look like `@"_ZN4etcetcetc")

comparatively, `static_dp` (line 648) is a whopping 12 lines long (whitespace +
comments included)
- statically calls `Cat`'s `speak()` on line 656

### release build

no separate `dyn_dp` and `static_dp` funcs

but `invoke void %8` is used at line 393, and the line right before is:
`%8 = select i1 %_3.i, ptr @"Cat_as_Animal_speak", ptr @"Dog_as_Animal_speak", !dbg !1424` 
- select ~= ternary op: condition ? true : false
- %_3.i is related to the rand condition check

## Inspecting disassembled binary

be aware: https://users.rust-lang.org/t/emit-asm-changes-the-produced-machine-code/17701








