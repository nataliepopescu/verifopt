# Interesting Panics in Tock

- [ ] `src/platform/mpu.rs`
    - interface for configuring MPU (memory protection unit)
    - config is `Display` for panics -> TODO how does this compile?

- [ ] `src/platform/chip.rs`
    - interface for configuring MCU (microcontroller unit)
    - trait `Chip` has a `print_state` func used by panic
        - TODO who calls this?

- [ ] `src/utilities/copy_slice.rs`

- [ ] `src/utilities/static_init.rs`

- [ ] `src/utilities/leasable_buffer.rs`

- [ ] `src/deferred_call.rs`

- [ ] `src/kernel.rs`

- [ ] `src/process_printer.rs`

- [ ] `src/process_standard.rs`

- [ ] `src/processbuffer.rs`

- [ ] `src/process.rs`

- [ ] `src/grant.rs`

- [ ] `src/debug.rs`

- [ ] `src/scheduler/round_robin.rs`
    - unwraps some result

- [ ] `src/scheduler/mlfq.rs`
    - multilevel feedback queue scheduler
    - `MLFQSched::get_timeslice_us`
        - why only 3 allowed indices? 0, 1, or 2
    - only used by `Scheduler::next()` trait impl for `MLFQSched`
        - indexed by `queue_index` --> dynamic? limits?
        - can be as long as the number of processes in a given queue
    - how do MLFQ queues work / do they have any length limitations?
        - how is a process added to a queue?
        - actually, looks like there is a limit of 3
        - so realistically this panic should never be tripped/could be verified/optimized out
    - _is_ this panic actually compiled or is it optimized out?
        - `get_timeslice_us` does not appear in the symbols table
        - so might be optimized out

        - TODO how to compile a board _not_ in release mode?
            - debug mode/optimizations turned off
