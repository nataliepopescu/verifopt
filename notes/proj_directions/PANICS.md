# Panics in Tock Kernel

imix binary size with panics: 6.2MB

## How do panics work in (embedded) Rust?

panics signal that the program has entered an unrecoverable state
- dev choice: panic or Result<>

normally, when using the std lib, a default hook is registered (seems to be just for a panicking _thread_ and not a runtime; i do not think we need to think about a panicking runtime but look into just in case TODO)
- default: print message to stderr + backtrace

[panicking](https://doc.rust-lang.org/stable/embedded-book/start/panicking.html)

[#[panic_handler]](https://doc.rust-lang.org/nomicon/panic-handler.html)
- defines `panic!` behavior in `#![no_std]` envs
- apply to functions w signature `fn(&PanicInfo) -> !`
    - [PanicInfo](https://doc.rust-lang.org/core/panic/struct.PanicInfo.html)
    - [!](https://doc.rust-lang.org/std/primitive.never.html) aka the "never" type
        - "expressions with type ! will coerce into any other type"
- can only have _ONE_ in a bin/lib crate

board-specific implementations
- imix: `boards/imix/src/io.rs`
    - called `panic_fmt`

but disassembly still shows some calls to a `_RNvNtCs1omKOwJWJyg_4core9panicking5panic`, so what is that for?
- calls `_RNvNtCs1omKOwJWJyg_4core9panicking9panic_fmt`
- this must be the `#[panic_handler]` translation?
- 14 calls and one def
    - are these the calls to optimize out??

[unwinding](https://doc.rust-lang.org/nomicon/unwinding.html)

## Generating objdump

architecture-specific objdumps: 

| chip | arch | board | objdump | nix package |
| ---- | ---- | ---- | ---- | ---- |
| nrf52 | arm | imix | `arm-none-eabi-objdump` | `gcc-arm-embedded-9` |

flags
1. `-S`: intermix source code w disassembly (implies `-d`)
2. `-C`: demangle symbol names

```sh
<arch-specific objdump> -SC <ELF binary>
```

## How are names mangled?

https://rust-lang.github.io/rfcs/2603-rust-symbol-name-mangling-v0.html

https://doc.rust-lang.org/rustc/symbol-mangling/index.html

haven't read detail, just using [rustfilt](https://crates.io/crates/rustfilt) which seems to be working great

## Panic unwinding

guess of how this works (unconfirmed)
- any calls to `panic!` are translated to the `panic_fmt` function as labeled by the `#[panic_handler]` attribute

source code: `core::panicking::panic_fmt` calls `kernel::debug::panic`
- which calls `panic_print` and `panic_blink_forever`
    - latter = infinite loop
    - former calls `panic_process_info` which calls `printer.print_overview` (impl for `ProcessPrinter` trait)

disassembly
- calls to `panic_fmt`
    - calls `rust_begin_unwind`
        - calls `_RINvNtCshve5IrHWJ36_6kernel5debug5panicINtNtNtB4_3hil3led6LedLowNtNtCsjM2vLBECUFU_5sam4l4gpio7GPIOPinENtNtCsk1uL2O17nym_4imix2io6WriterINtNtB14_4chip5Sam4lNtB2c_23Sam4lDefaultPeripheralsENtNtCs76KSGgFM9gH_15capsules_system15process_printer18ProcessPrinterTextEB1G_` 
            - `kernel/src/debugs.rs` `panic` function
            - `capsules_system::process_printer::ProcessPrinterText` initialized at `boards/imix/src/main.rs:103`
                - prints out process state
            - calls (1) `_RINvNtCshve5IrHWJ36_6kernel5debug11panic_printNtNtCsk1uL2O17nym_4imix2io6WriterINtNtCsjM2vLBECUFU_5sam4l4chip5Sam4lNtB1i_23Sam4lDefaultPeripheralsENtNtCs76KSGgFM9gH_15capsules_system15process_printer18ProcessPrinterTextEBM_` 
                - (1) `kernel/src/debug.rs` `panic_print` function
                    - **fat** (2997 - 2371 = >600 LOAC)
                    - calls `_RNvXs_NtCsk1uL2O17nym_4imix2ioNtB4_6WriterNtNtCshve5IrHWJ36_6kernel5debug7IoWrite5write` many times
                        - `kernel/src/debug.rs` `IoWrite::write()`
                        - impls `std::io::write` for `no_std`
                    - calls `_RNvXs_NtCs76KSGgFM9gH_15capsules_system15process_printerNtB4_18ProcessPrinterTextNtNtCshve5IrHWJ36_6kernel15process_printer14ProcessPrinter14print_overview` once
                        - `capsules/system/src/process_printer.rs` `print_overview()`
                        - **also fat** (42893 - 42316 = >500 LOAC)
                        - calls `_RNvXs_NtNtCshve5IrHWJ36_6kernel9utilities12binary_writeNtB4_26WriteToBinaryOffsetWrapperNtNtCs1omKOwJWJyg_4core3fmt5Write9write_str`
                        - calls `_RNvYNtNtNtCshve5IrHWJ36_6kernel9utilities12binary_write26WriteToBinaryOffsetWrapperNtNtCs1omKOwJWJyg_4core3fmt5Write9write_fmtCs76KSGgFM9gH_15capsules_system`
                    - calls `_RNvNtNtCs1omKOwJWJyg_4core5slice5index24slice_end_index_len_fail`
                        - called by other code as well
                    - also calls panic_fmt again? (at very end)
            - and (2) `_RINvNtCshve5IrHWJ36_6kernel5debug19panic_blink_foreverINtNtNtB4_3hil3led6LedLowNtNtCsjM2vLBECUFU_5sam4l4gpio7GPIOPinEECsk1uL2O17nym_4imix`
                - (2) `kernel/src/debug.rs` `panic_blink_forever`
                    - less fat, but infinite loop

## How is objdump source code intermixed w disassembled output?

TODO

## Which panics end up in machine code?

### source -> assembly

`kernel` subdir

- [ ] `src/platform/mpu.rs`
    - interface for configuring MPU (memory protection unit)
    - config is `Display` for panics -> TODO how does this compile?

- [ ] `src/platform/chip.rs`
    - interface for configuring MCU (microcontroller unit)
    - trait `Chip` has a `print_state` func used by panic
        - TODO who calls this?

- [ ] `src/utilities/static_init.rs`
    - this might be very hard to remove / would require some other technique b/c
    panicking depends on the number of times a function is called (so in the
    hands of the caller)

- [ ] `src/deferred_call.rs`
    - array size default = 32 (supports up to 32 deferred calls), can be
      modified (manually) to support more
    - implementors of `DeferredCallClient` can set/receive deferred calls (aka 
      software interrups)
    - in comment/docs: "This function costs about 300 bytes, so you can remove 
      it if you are confident your setup will not exceed 32 deferred calls, and 
      that all of your components register their deferred calls."
    - where/how are deferred calls created? "verifying" would require a similar
      mechanism to ensuring that some function is called no more than X times
      (once in the `static_init.rs` case, or 32 here)

- [ ] `src/kernel.rs`
    - panic if grant is created after processes are initialized
    - in `create_grant()`
        - called from all the boards and `kernel/src/ipc.rs`
    - `self.grants_finalized` is a field, can maybe have a pre-condition on this
      but don't know how to write a contract on a struct subfield TODO
        - `self` == `Kernel`
        - `self.grants_finalized: Cell<bool>`
            - `Cell<T>` == shareable mutable container (single-threaded) ->
              interior mutability
                - `get()` copies the interior value
                - `set()` replaces the interior value, dropping the replaced
                  value
        - set to `true` in `get_grant_count_and_finalize()`
            - called by `get_grant_count_and_finalize_external()` but can't find
              any callers of that in the tock repo
            - `process_standard.rs` and `introspection.rs` call 
              `get_grant_count_and_finalize()` directly

- [ ] `src/process_standard.rs`
    - in `set_fault_state`
    - `self` == `ProcessStandard`
    - when `self.fault_policy.action(self)` == `FaultAction::Panic`
    - `fault_policy: &'a dyn ProcessFaultPolicy`
        - `dyn` = dynamic dispatch calls to trait methods
        - `ProcessFaultPolicy` is a trait
        - must impl `fn action(&self, process: &dyn Process) ->
          process::FaultAction`
    - does this mean some processes choose to respond with panics? 
        - capsules impl ProcessFaultPolicy (in
          `capsules/system/src/process_policies.rs`)
        - `PanicFaultPolicy` and `ThresholdRestartThenPanicFaultPolicy`
        - used in boards (but not in imix); imix == `StopFaultPolicy`
        - why is this a board-level decision? oh, its how the _kernel_ should
          respond when a process faults
    - FaultPolicy "popularity" (rough estimate = 
      `grep -rnI '<type>FaultPolicy' | wc -l`)
      - Panic: 81
      - Stop: 4
      - StopWithDebug: 12
      - Restart: 7
      - RestartWithDebug: 2
      - ThresholdRestart: 5
      - ThresholdRestartThenPanic: 7
    - what should be taken into consideration when deciding a fault policy for a
      board? TODO

- [ ] `src/processbuffer.rs`
    - several panics in `copy_to_slice` implementations (if 
      `self.len() != dest.len()`)
    - why is this a panic and, say, not an error? -> panic is optional
    - seems to be used in capsules
    - flux: try slice len preconditions TODO

- [ ] `src/process.rs`
    - defines `FaultAction` enum (explicitly says `Panic` variant is useful for
      debugging apps)

- [ ] `src/grant.rs`
    - in `access_grant_with_allocator()`
    - large comment that may help w some verification intuition, but again
      relies on the caller to not call a function on some object more than one
      time
    - if we have some dynamic counter somewhere, can that ever be verified
      statically?

- [ ] `src/debug.rs`
    - panic support routines

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

what is `if !config::CONFIG.debug_panics {` ?


`capsules` subdir

- [ ] `core/src/process_console.rs`

- [ ] `core/src/alarm.rs`

- [ ] `core/src/adc.rs`

- [ ] `core/src/virtualizers/virtual_aes_ccm.rs`

- [ ] `core/src/virtualizers/virtual_i2c.rs`

- [ ] `system/src/process_checker/basic.rs`

- [ ] `extra/src/ieee802154/framer.rs`

- [ ] `extra/src/net/ieee802154.rs`

- [ ] `extra/src/net/sixlowpan/sixlowpan_compression.rs`

- [ ] `extra/src/net/thread/driver.rs`

- [ ] `extra/src/log.rs`

- [ ] `extra/src/usb/ctap.rs`

- [ ] `extra/src/led_matrix.rs`

- [ ] `extra/src/st77xx.rs`

- [ ] `extra/src/panic_button.rs`

- [ ] `extra/src/sht4x.rs`

- [ ] `extra/src/seven_segment.rs`

- [ ] `extra/src/sht3x.rs`

- [ ] `extra/src/bus.rs`


`boards` subdir

- [ ] `wm1110dev/src/main.rs`

- [ ] `nano33ble/src/main.rs`

- [ ] `makepython-nrf52840/src/main.rs`

- [ ] `nano33ble_rev2/src/main.rs`

- [ ] `microbit_v2/src/main.rs`

- [ ] `clue_nrf52840/src/main.rs`

- [ ] `opentitan/earlgrey-cw310/build.rs`
    - actual

- [ ] `nano_rp2040_connect/src/main.rs`
    - actual

- [ ] `components/src/spi.rs`
    - actual

- [ ] `components/src/bus.rs`
    - actual

- [ ] `build_scripts/src/default.rs`
    - actual


`arch` subdir

- [ ] `rv32i/src/pmp.rs`

- [ ] `cortex-m0/src/lib.rs`

- [ ] `cortex-v7m/src/lib.rs`

- [ ] `cortex-m/src/lib.rs`

- [ ] `cortex-m/src/scb.rs`


lots in `chips` subdir TODO


`libraries` subdir

- [ ] `tock-cells/src/map_cell.rs`

- [ ] `tickv/src/async_ops.rs`


lots in `tools` subdir TODO

### assembly -> source

in reverse direction: identify a panic in the disassembled output + trace it to the source

```sh
arm-none-eabi-objdump -SC /path/to/imix/elf
```

~~looking for calls to `_RNvNtCs1omKOwJWJyg_4core9panicking9panic_fmt`~~

looking for calls to `_RNvNtCs1omKOwJWJyg_4core9panicking5panic`

14 total

demangling via [rustfilt](https://crates.io/crates/rustfilt)

- [ ] 0001019c <_RINvMNtCshve5IrHWJ36_6kernel6kernelNtB3_6Kernel11kernel_loopNtCsk1uL2O17nym_4imix4ImixINtNtCsjM2vLBECUFU_5sam4l4chip5Sam4lNtB1p_23Sam4lDefaultPeripheralsEKh4_EBY_>:
    - `<kernel::kernel::Kernel>::kernel_loop::<imix::Imix, sam4l::chip::Sam4l<sam4l::chip::Sam4lDefaultPeripherals>, 4>`
        - imix::Imix == KernelResources<C>
        - sam4l::chip::Sam4l<sam4l::chip::Sam4lDefaultPeripherals> == Chip
        - 4 == NUM_PROCS

- [ ] 00014a9c <_RNvMs_NtNtCsjMjWsJL4FQh_13capsules_core12virtualizers11virtual_i2cINtB4_6MuxI2CNtNtCsjM2vLBECUFU_5sam4l3i2c5I2CHwE10do_next_opCsk1uL2O17nym_4imix>:
    - `<capsules_core::virtualizers::virtual_i2c::MuxI2C<sam4l::i2c::I2CHw>>::do_next_op`

- [ ] 000166b8 <_RNvXNtNtNtCsjbBGJ2Hrjer_14capsules_extra3net4ipv69ipv6_sendINtB2_13IP6SendStructINtNtNtCsjMjWsJL4FQh_13capsules_core12virtualizers13virtual_alarm15VirtualMuxAlarmNtNtCsjM2vLBECUFU_5sam4l3ast3AstEENtB2_9IP6Sender7send_toCsk1uL2O17nym_4imix>:
    - `<capsules_extra::net::ipv6::ipv6_send::IP6SendStruct<capsules_core::virtualizers::virtual_alarm::VirtualMuxAlarm<sam4l::ast::Ast>> as capsules_extra::net::ipv6::ipv6_send::IP6Sender>::send_to`

- [ ] 000209c8 <_RNvXs_NtNtCsjbBGJ2Hrjer_14capsules_extra3usb11usbc_clientINtB4_6ClientNtNtCsjM2vLBECUFU_5sam4l4usbc4UsbcENtNtNtCshve5IrHWJ36_6kernel3hil3usb6Client10packet_outCsk1uL2O17nym_4imix>:
     - `<capsules_extra::usb::usbc_client::Client<sam4l::usbc::Usbc> as kernel::hil::usb::Client>::packet_out`

- [ ] 00020e18 <_RNvXs_NtNtCsjbBGJ2Hrjer_14capsules_extra3usb11usbc_clientINtB4_6ClientNtNtCsjM2vLBECUFU_5sam4l4usbc4UsbcENtNtNtCshve5IrHWJ36_6kernel3hil3usb6Client9packet_inCsk1uL2O17nym_4imix>:
    - `<capsules_extra::usb::usbc_client::Client<sam4l::usbc::Usbc> as kernel::hil::usb::Client>::packet_in`

- [ ] 00027d24 <_RNvNtNtNtCsjbBGJ2Hrjer_14capsules_extra3net9sixlowpan21sixlowpan_compression22decompress_iid_context>:
    - `capsules_extra::net::sixlowpan::sixlowpan_compression::decompress_iid_context`

- [ ] 000283b4 <_RNvMNtNtNtCsjbBGJ2Hrjer_14capsules_extra3net9sixlowpan15sixlowpan_stateNtB2_7TxState24write_additional_headers>:
    - `<capsules_extra::net::sixlowpan::sixlowpan_state::TxState>::write_additional_headers`

- [ ] 000285d8 <_RNvMs1_NtNtNtCsjbBGJ2Hrjer_14capsules_extra3net4ipv64ipv6NtB5_9IP6Packet18get_total_hdr_size>:
    - `<capsules_extra::net::ipv6::ipv6::IP6Packet>::get_total_hdr_size`

- [ ] 0002a930 <_RNvMs_NtNtCsjbBGJ2Hrjer_14capsules_extra10ieee8021546framerNtB4_9FrameInfo18ccm_encrypt_ranges>: (2)
    - `<capsules_extra::ieee802154::framer::FrameInfo>::ccm_encrypt_ranges`

- [ ] 0002b860 <_RNvXs_NtCsjbBGJ2Hrjer_14capsules_extra6sha256NtB4_14Sha256SoftwareINtNtNtCshve5IrHWJ36_6kernel3hil6digest10DigestDataKj20_E15set_data_client>:
    - `<capsules_extra::sha256::Sha256Software as kernel::hil::digest::DigestData<32>>::set_data_client`

- [ ] 0002b878 <_RNvXs1_NtCsjbBGJ2Hrjer_14capsules_extra6sha256NtB5_14Sha256SoftwareINtNtNtCshve5IrHWJ36_6kernel3hil6digest12DigestVerifyKj20_E17set_verify_client>:
    - `<capsules_extra::sha256::Sha256Software as kernel::hil::digest::DigestVerify<32>>::set_verify_client`

- [ ] 0002b890 <_RNvXs6_NtCsjbBGJ2Hrjer_14capsules_extra6sha256NtB5_14Sha256SoftwareINtNtNtCshve5IrHWJ36_6kernel3hil6digest16DigestDataVerifyKj20_E10set_client>:
    - `<capsules_extra::sha256::Sha256Software as kernel::hil::digest::DigestDataVerify<32>>::set_client`

- [ ] 0002d1b8 <_RNvNtCs1omKOwJWJyg_4core6option13unwrap_failed>:
    - `core::option::unwrap_failed`

### remove a panic and see how the binary/objdump changes

removing MLFQ panic does change the binary, but unclear _how_

might get more info from objdump diff
- same number of `_RNvNtCs1omKOwJWJyg_4core9panicking5panic` calls

### `-why_live` linker flag


### other techniques to try


check `strings` output

















