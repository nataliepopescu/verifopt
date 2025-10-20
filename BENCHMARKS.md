# Benchmarks / Smoke Test Oddities

## `simple` + `pub_trait`: `best_norm` performs worse than `src_rw`

just looking at `simple` for simplicity... ha

the `src_rw` MIR is at least longer/more complicated than that of `best`,
but the LLVM IR actually ends up being more optimized...

recall that best is really just a static call (in addition to the `get_animal`
and `get_cat` calls that help it be a fairer comparison)
- if these additional calls are removed, then `best` still only performs just as
  well as `src_rw` for `simple`!
- that is wild

LLVM IR from godbolt, using `rustc nightly` and `--edition=2024 -Cdebuginfo=0
-Copt-level=3 --emit=llvm-ir`. 

`best_norm` LLVM IR:

<details>

<summary>main logic bbs</summary>

```llvm
; example::run_best
; Function Attrs: nonlazybind uwtable
define void @_ZN7example8run_best17hec4ce4feafcb64c5E(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, i64 noundef %num, ptr noalias noundef nonnull readonly align 1 captures(none) %cat) unnamed_addr #1 personality ptr @rust_eh_personality {
  %0 = icmp eq i64 %num, 0
  %spec.select.i = select i1 %0, ptr @vtable.0, ptr @vtable.1 ; 
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshDZgbIUcEv7_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #8, !noalias !6
; call __rustc::__rust_alloc
  %1 = tail call noundef dereferenceable_or_null(4) ptr @_RNvCshDZgbIUcEv7_7___rustc12___rust_alloc(i64 noundef range(i64 1, 0) 4, i64 noundef range(i64 1, -9223372036854775807) 1) #8, !noalias !6
  %2 = icmp eq ptr %1, null
  br i1 %2, label %bb3.i.i, label %bb3, !prof !11

bb3:                                              ; preds = %bb2
  store i32 2003789165, ptr %1, align 1, !noalias !12
  store i64 4, ptr %_0, align 8
  %_5.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %1, ptr %_5.sroa.4.0._0.sroa_idx, align 8
  %_5.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 4, ptr %_5.sroa.5.0._0.sroa_idx, align 8
  %4 = load ptr, ptr %spec.select.i, align 8, !invariant.load !3
  %.not.i9 = icmp eq ptr %4, null
  br i1 %.not.i9, label %"core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>::h0a0e2ba91be03f1b.exit16", label %is_not_null.i10

is_not_null.i10:                                  ; preds = %bb3
  tail call void %4(ptr noundef nonnull inttoptr (i64 1 to ptr))
  br label %"core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>::h0a0e2ba91be03f1b.exit16"

"core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>::h0a0e2ba91be03f1b.exit16": ; preds = %is_not_null.i10, %bb3
  ret void

;;;; cleanup/error bbs (can largely ignore for our purposes)
```

</details>

<details>

<summary>cleanup/error bbs</summary>

```llvm
bb3.i.i:                                          ; preds = %bb2
; invoke alloc::raw_vec::handle_error
  invoke void @_ZN5alloc7raw_vec12handle_error17h069bb0fee1c169b6E(i64 noundef 1, i64 4) #10
          to label %.noexc unwind label %cleanup1

cleanup1:                                         ; preds = %bb3.i.i
  %3 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>
  invoke fastcc void @"_ZN4core3ptr69drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$example..Animal$GT$$GT$17h0a0e2ba91be03f1bE"(ptr nonnull inttoptr (i64 1 to ptr), ptr nonnull @vtable.0) #9
          to label %bb6 unwind label %terminate

.noexc:                                           ; preds = %bb3.i.i
  unreachable

bb6:                                              ; preds = %cleanup1
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>
  invoke fastcc void @"_ZN4core3ptr69drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$example..Animal$GT$$GT$17h0a0e2ba91be03f1bE"(ptr nonnull inttoptr (i64 1 to ptr), ptr nonnull %spec.select.i) #9
          to label %common.resume unwind label %terminate

common.resume:                                    ; preds = %bb6
  resume { ptr, i32 } %3

terminate:                                        ; preds = %cleanup1, %bb6
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_ZN4core9panicking16panic_in_cleanup17h975505634b9400e7E() #11
  unreachable
}
```

</details>

start (~get_animal code):
- compare if %num == 0
- store either Cat or Dog vtable
- allocate mem

bb3 (mem alloc succeeds): 
- store a number/address (`2003789165` - probably the Cat vtable address) in %1
    - same constant as below IR's integer representation of "meow"
- store 4 in %_0 (offset 0)
- store %1 (Cat vtable address) in %_0 (offset 8)
- store 4 in %_0 (offset 16)

- load the selected vtable from `start` -> %4
- compare address (?) with null

is_not_null: 
- call offset vtable method
    - ah, this may actually be a `drop` function...


`src_rw` LLVM IR:

<details>

<summary>main logic bbs</summary>

```llvm
; example::run_src_rw
; Function Attrs: nonlazybind uwtable
define void @_ZN7example10run_src_rw17hebf68f20b7d2582aE(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, i64 noundef %num) unnamed_addr #1 personality ptr @rust_eh_personality {
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshDZgbIUcEv7_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #8, !noalias !3
; call __rustc::__rust_alloc
  %0 = tail call noundef dereferenceable_or_null(4) ptr @_RNvCshDZgbIUcEv7_7___rustc12___rust_alloc(i64 noundef range(i64 1, 0) 4, i64 noundef range(i64 1, -9223372036854775807) 1) #8, !noalias !3
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb3.i.i7.invoke, label %bb6, !prof !6

bb6:                                              ; preds = %bb2
  %_810 = icmp eq ptr @vtable.1, @vtable.0
  %3 = icmp eq i64 %num, 0
  %_8 = or i1 %_810, %3
  %. = select i1 %_8, i32 2003789165, i32 1718579063
  store i32 %., ptr %0, align 1, !noalias !3
  store i64 4, ptr %_0, align 8
  %_19.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %0, ptr %_19.sroa.4.0._0.sroa_idx, align 8
  %_19.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 4, ptr %_19.sroa.5.0._0.sroa_idx, align 8
  ret void

;;;; cleanup/error bbs (can largely ignore for our purposes)
```

</details>

<details>

<summary>cleanup/error bbs</summary>

```llvm
bb3.i.i7.invoke:                                  ; preds = %bb2
; invoke alloc::raw_vec::handle_error
  invoke void @_ZN5alloc7raw_vec12handle_error17h069bb0fee1c169b6E(i64 noundef 1, i64 4) #9
          to label %bb3.i.i7.cont unwind label %cleanup1

bb3.i.i7.cont:                                    ; preds = %bb3.i.i7.invoke
  unreachable

cleanup1:                                         ; preds = %bb3.i.i7.invoke
  %2 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>
  invoke fastcc void @"_ZN4core3ptr69drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$example..Animal$GT$$GT$17h0a0e2ba91be03f1bE"(ptr nonnull inttoptr (i64 1 to ptr), ptr nonnull @vtable.0) #10
          to label %bb8 unwind label %terminate

terminate:                                        ; preds = %cleanup1
  %4 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_ZN4core9panicking16panic_in_cleanup17h975505634b9400e7E() #11
  unreachable

bb8:                                              ; preds = %cleanup1
  resume { ptr, i32 } %2
}
```

</details>

func def line:
- return type == void
    - actual ret value is passed via hidden first pointer argument (sret)
    - 24 bytes - why?
- arg list:
    - %_0
        - hidden return pointer
    - %num
        - actual arg
- `unnamed_addr #1`
    - from chatgpt: "the address of this function is not significant (i.e., it
      can be merged with other identical functions)."
    - `attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }`
- `personality ptr @rust_eh_personality` = error handling

start:
- allocate 4 byte mem -> %0

bb6 (mem alloc succeeds): 
- compare if Cat vtable == Dog vtable
    - vtable.0 == cat's vtable
    - vtable.1 == dynamically-selected animal's vtable (you sure?)
- compare if %num == 0
- or the above two results (if any of the above are true...) -> %_8
- if any of the above are true, %. = Cat vtable address, otherwise Dog vtable
  address (just constants, assuming these are vtable addresses)
    - could hypothetically be the address of the respective strings to return
    - from chatgpt: "But instead of doing full string handling (which would be
      expensive in IR), it writes a specific constant — which is likely a
      precomputed hash or encoded string value for "meow" and "woof"."
    - this would explain why no function is actually being called
- store vtable address (%.) in %0

- store 4 in %_0 (offset 0) - String len
- `%_19.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8`
    - offset 8 into %_0
- store %0 (vtable address/constant) in %_0 (offset 8) - String ptr
- `%_19.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16`
    - offset 16 into %_0
- store 4 in %_0 (offset 16) - String capacity

- from chatgpt: "And knowing how String is laid out internally (len, ptr,
  capacity, 3 * 8 bytes), we now know that the 24-byte return struct being
  assembled in the IR is a Rust String."
    - in the above block of instructions, the IR is assembling a 3-field struct,
      i.e. the String return value


### observations/conclusions

- `best_norm` has more LOCs + branches, while `src_rw` is more streamlined
    - more potential optimization opportunities
- `src_rw` does not create/drop a trait object, while `best_norm` does (even
  though it isn't used)
    - less heap traffic / better CPU cache locality
    - manual dispatch enables elision of vtable/drop glue





## `struct_fields`: `src_rw` performs worse than `not_rw`

TODO


`not_rw`:

<details>

<summary>main logic bbs</summary>

```llvm
; example::run_not_rw
; Function Attrs: nonlazybind uwtable
define void @_ZN7example10run_not_rw17ha476ebdbbda63ed0E(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, i64 noundef %num) unnamed_addr #1 personality ptr @rust_eh_personality {
  %0 = icmp eq i64 %num, 0
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshDZgbIUcEv7_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #9, !noalias !3
; call __rustc::__rust_alloc
  %1 = tail call noundef align 8 dereferenceable_or_null(40) ptr @_RNvCshDZgbIUcEv7_7___rustc12___rust_alloc(i64 noundef 40, i64 noundef 8) #9, !noalias !3
  %2 = icmp eq ptr %1, null
  br i1 %0, label %bb1.i, label %bb3.i

bb1.i:                                            ; preds = %start
  br i1 %2, label %bb2.i.i, label %example::get_animal::h6dc0715541cf6bf2.exit, !prof !6

bb3.i:                                            ; preds = %start
  br i1 %2, label %bb2.i2.i, label %example::get_animal::h6dc0715541cf6bf2.exit, !prof !6

example::get_animal::h6dc0715541cf6bf2.exit: ; preds = %bb1.i, %bb3.i
  %alloc_21d17cef229e1474a90c9b348f8ab787.sink.i = phi ptr [ @alloc_0aeadd5be856a09c17c0276c83aa6ecb, %bb1.i ], [ @alloc_21d17cef229e1474a90c9b348f8ab787, %bb3.i ]
  %alloc_beba4de3f16ebcaee9706459d9100c63.sink.i = phi ptr [ @alloc_dc969eeaf2d16587d1d724a5ed80dc67, %bb1.i ], [ @alloc_beba4de3f16ebcaee9706459d9100c63, %bb3.i ]
  %_0.sroa.3.0.i = phi ptr [ @vtable.0, %bb1.i ], [ @vtable.1, %bb3.i ]
  %3 = phi <2 x i64> [ <i64 4, i64 9>, %bb1.i ], [ <i64 8, i64 7>, %bb3.i ]
  store ptr %alloc_21d17cef229e1474a90c9b348f8ab787.sink.i, ptr %1, align 8
  %_5.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %1, i64 8
  store i64 5, ptr %_5.sroa.4.0..sroa_idx.i, align 8
  %_5.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %1, i64 16
  store ptr %alloc_beba4de3f16ebcaee9706459d9100c63.sink.i, ptr %_5.sroa.5.0..sroa_idx.i, align 8
  %_5.sroa.6.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %1, i64 24
  store <2 x i64> %3, ptr %_5.sroa.6.0..sroa_idx.i, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshDZgbIUcEv7_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #9
; call __rustc::__rust_alloc
  %4 = tail call noundef align 8 dereferenceable_or_null(40) ptr @_RNvCshDZgbIUcEv7_7___rustc12___rust_alloc(i64 noundef 40, i64 noundef 8) #9
  %5 = icmp eq ptr %4, null
  br i1 %5, label %bb2.i, label %bb8, !prof !6

bb8:                                              ; preds = %example::get_animal::h6dc0715541cf6bf2.exit
  store ptr @alloc_22f94bb69379f300d5861c457fea3b28, ptr %4, align 8
  %_8.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %4, i64 8
  store i64 9, ptr %_8.sroa.4.0..sroa_idx, align 8
  %_8.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %4, i64 16
  store ptr @alloc_de6c8e55ac0f214c9a4376ea54159ba2, ptr %_8.sroa.5.0..sroa_idx, align 8
  %_8.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %4, i64 24
  store i64 7, ptr %_8.sroa.6.0..sroa_idx, align 8
  %_8.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %4, i64 32
  store i64 2, ptr %_8.sroa.7.0..sroa_idx, align 8
  %7 = getelementptr inbounds nuw i8, ptr %_0.sroa.3.0.i, i64 24
  %8 = load ptr, ptr %7, align 8, !invariant.load !3, !nonnull !3
  %9 = invoke { ptr, i64 } %8(ptr noundef nonnull align 1 %1)
          to label %bb2 unwind label %cleanup1

bb2:                                              ; preds = %bb8
  %_4.0 = extractvalue { ptr, i64 } %9, 0
  %_4.1 = extractvalue { ptr, i64 } %9, 1
  %_26.i.i.i.i = icmp slt i64 %_4.1, 0
  br i1 %_26.i.i.i.i, label %bb3.i.i, label %bb18.i.i.i, !prof !13

bb18.i.i.i:                                       ; preds = %bb2
  %11 = icmp eq i64 %_4.1, 0
  br i1 %11, label %bb3, label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb18.i.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshDZgbIUcEv7_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #9, !noalias !14
; call __rustc::__rust_alloc
  %12 = tail call noundef ptr @_RNvCshDZgbIUcEv7_7___rustc12___rust_alloc(i64 noundef range(i64 1, 0) %_4.1, i64 noundef range(i64 1, -9223372036854775807) 1) #9, !noalias !14
  %13 = icmp eq ptr %12, null
  br i1 %13, label %bb3.i.i, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %bb5.i.i.i
  %14 = ptrtoint ptr %12 to i64
  br label %bb3

bb3:                                              ; preds = %bb10.i.i.i, %bb18.i.i.i
  %_4.sroa.10.0.i.i = phi i64 [ %14, %bb10.i.i.i ], [ 1, %bb18.i.i.i ]
  %15 = inttoptr i64 %_4.sroa.10.0.i.i to ptr
  tail call void @llvm.memcpy.p0.p0.i64(ptr align 1 %15, ptr nonnull readonly align 1 %_4.0, i64 %_4.1, i1 false), !noalias !20
  store i64 %_4.1, ptr %_0, align 8
  %_9.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %15, ptr %_9.sroa.4.0._0.sroa_idx, align 8
  %_9.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %_4.1, ptr %_9.sroa.5.0._0.sroa_idx, align 8
; call __rustc::__rust_dealloc
  tail call void @_RNvCshDZgbIUcEv7_7___rustc14___rust_dealloc(ptr noundef nonnull %4, i64 noundef range(i64 1, -9223372036854775808) 40, i64 noundef range(i64 1, -9223372036854775807) 8) #9
  %16 = load ptr, ptr %_0.sroa.3.0.i, align 8, !invariant.load !3
  %.not.i14 = icmp eq ptr %16, null
  br i1 %.not.i14, label %bb3.i19, label %is_not_null.i15

is_not_null.i15:                                  ; preds = %bb3
  invoke void %16(ptr noundef nonnull %1)
          to label %bb3.i19 unwind label %cleanup.i16

;;;; cleanup/error bbs (can largely ignore for our purposes)
```

</details>

<details>

<summary>cleanup/error bbs</summary>

```llvm
bb2.i:                                            ; preds = %example::get_animal::h6dc0715541cf6bf2.exit
; invoke alloc::alloc::handle_alloc_error
  invoke void @_ZN5alloc5alloc18handle_alloc_error17h1c18310aad0b8a29E(i64 noundef 8, i64 noundef 40) #10
          to label %.noexc unwind label %cleanup

.noexc:                                           ; preds = %bb2.i
  unreachable

bb2.i.i:                                          ; preds = %bb1.i
; call alloc::alloc::handle_alloc_error
  tail call void @_ZN5alloc5alloc18handle_alloc_error17h1c18310aad0b8a29E(i64 noundef 8, i64 noundef 40) #10, !noalias !7
  unreachable

bb2.i2.i:                                         ; preds = %bb3.i
; call alloc::alloc::handle_alloc_error
  tail call void @_ZN5alloc5alloc18handle_alloc_error17h1c18310aad0b8a29E(i64 noundef 8, i64 noundef 40) #10, !noalias !10
  unreachable

bb3.i.i:                                          ; preds = %bb5.i.i.i, %bb2
  %_4.sroa.4.0.ph.i.i = phi i64 [ 1, %bb5.i.i.i ], [ 0, %bb2 ]
; invoke alloc::raw_vec::handle_error
  invoke void @_ZN5alloc7raw_vec12handle_error17h069bb0fee1c169b6E(i64 noundef %_4.sroa.4.0.ph.i.i, i64 %_4.1) #10
          to label %.noexc11 unwind label %cleanup1

.noexc11:                                         ; preds = %bb3.i.i
  unreachable

bb3.i19:                                          ; preds = %is_not_null.i15, %bb3
  %17 = getelementptr inbounds nuw i8, ptr %_0.sroa.3.0.i, i64 8
  %18 = load i64, ptr %17, align 8, !range !4, !invariant.load !3
  %19 = getelementptr inbounds nuw i8, ptr %_0.sroa.3.0.i, i64 16
  %20 = load i64, ptr %19, align 8, !range !5, !invariant.load !3
  %21 = add i64 %20, -1
  %22 = icmp sgt i64 %21, -1
  tail call void @llvm.assume(i1 %22)
  %23 = icmp eq i64 %18, 0
  br i1 %23, label %"core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>::h0a0e2ba91be03f1b.exit21", label %bb1.i.i20

bb1.i.i20:                                        ; preds = %bb3.i19
; call __rustc::__rust_dealloc
  tail call void @_RNvCshDZgbIUcEv7_7___rustc14___rust_dealloc(ptr noundef nonnull %1, i64 noundef range(i64 1, -9223372036854775808) %18, i64 noundef range(i64 1, -9223372036854775807) %20) #9
  br label %"core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>::h0a0e2ba91be03f1b.exit21"

cleanup1:                                         ; preds = %bb3.i.i, %bb8
  %10 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>
  invoke fastcc void @"_ZN4core3ptr69drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$example..Animal$GT$$GT$17h0a0e2ba91be03f1bE"(ptr nonnull %4, ptr nonnull @vtable.0) #11
          to label %bb6 unwind label %terminate

bb6:                                              ; preds = %cleanup, %cleanup1
  %.pn = phi { ptr, i32 } [ %10, %cleanup1 ], [ %6, %cleanup ]
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>
  invoke fastcc void @"_ZN4core3ptr69drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$example..Animal$GT$$GT$17h0a0e2ba91be03f1bE"(ptr nonnull %1, ptr nonnull %_0.sroa.3.0.i) #11
          to label %common.resume unwind label %terminate

common.resume:                                    ; preds = %bb6, %cleanup.i16, %bb1.i4.i17
  %common.resume.op = phi { ptr, i32 } [ %24, %bb1.i4.i17 ], [ %24, %cleanup.i16 ], [ %.pn, %bb6 ]
  resume { ptr, i32 } %common.resume.op

cleanup:                                          ; preds = %bb2.i
  %6 = landingpad { ptr, i32 }
          cleanup
  br label %bb6

cleanup.i16:                                      ; preds = %is_not_null.i15
  %24 = landingpad { ptr, i32 }
          cleanup
  %25 = getelementptr inbounds nuw i8, ptr %_0.sroa.3.0.i, i64 8
  %26 = load i64, ptr %25, align 8, !range !4, !invariant.load !3
  %27 = getelementptr inbounds nuw i8, ptr %_0.sroa.3.0.i, i64 16
  %28 = load i64, ptr %27, align 8, !range !5, !invariant.load !3
  %29 = add i64 %28, -1
  %30 = icmp sgt i64 %29, -1
  tail call void @llvm.assume(i1 %30)
  %31 = icmp eq i64 %26, 0
  br i1 %31, label %common.resume, label %bb1.i4.i17

bb1.i4.i17:                                       ; preds = %cleanup.i16
; call __rustc::__rust_dealloc
  tail call void @_RNvCshDZgbIUcEv7_7___rustc14___rust_dealloc(ptr noundef nonnull %1, i64 noundef range(i64 1, -9223372036854775808) %26, i64 noundef range(i64 1, -9223372036854775807) %28) #9
  br label %common.resume

"core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>::h0a0e2ba91be03f1b.exit21": ; preds = %bb3.i19, %bb1.i.i20
  ret void

terminate:                                        ; preds = %cleanup1, %bb6
  %32 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_ZN4core9panicking16panic_in_cleanup17h975505634b9400e7E() #12
  unreachable
}
```

</details>

start: 
- compare %num to 0
- alloc mem (40 bytes)

example::get_animal::... : (same as below)
- first alloc: either "kitty" or "doggo" (1st struct field)
- second alloc: either "shoe" or "anywhere" (3rd struct field)
- %_0.sroa...: Cat or Dog vtable ptr
- %3: phi node for 2nd struct field

- store 1st struct field -> alloc mem
- store 5 -> alloc mem + 8
    - 5 might correspond to the length of the 1st field's string, but then why
      doesn't the 3rd field's string have a similar length field?
- store 3rd struct field -> alloc mem + 16
- store 2nd struct field -> alloc mem + 24

    - ^ struct packing? why is the order mixed up?

- alloc more mem (40 bytes) -> %4

bb8: 
- store "catherine" -> 2nd alloc mem (name)
- store 9 -> 2nd alloc mem + 8 (name str len)
- store "shoebox" -> 2nd alloc mem + 16 (fav_toy)
- store 7 -> 2nd alloc mem + 24 (fav_toy str len)
- store 2 -> 2nd alloc mem + 32 (age)

- (^ same as below's bb10)

- %7: vtable 3rd method address
- call that method

bb2: 
- get retval of vtable method call
    - two parts likely b/c retval == &str (ptr + len)
    - %_4.0 : ptr
    - %_4.1 : len
- from chatgpt:
    - check length non-negative
    - check length non-zero?

bb5.i.i.i:
- alloc mem (bytes == length of retval) -> %12

bb10.i.i.i:
- %14: converts %12 ptr val to integer

bb3:
- phi node
- %15: converts %14 back to ptr (newly-alloc mem)
- memcopy retval into %15
- %_0 filled (len = %_4.1, ptr = %15, capacity = %_4.1) -> turn &str into String
- call dealloc/drop method


`src_rw`:

<details>

<summary>main logic bbs</summary>

```llvm
; example::run_src_rw
; Function Attrs: nonlazybind uwtable
define void @_ZN7example10run_src_rw17hebf68f20b7d2582aE(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, i64 noundef %num) unnamed_addr #1 personality ptr @rust_eh_personality {
  %0 = icmp eq i64 %num, 0
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshDZgbIUcEv7_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #8, !noalias !3
; call __rustc::__rust_alloc
  %1 = tail call noundef align 8 dereferenceable_or_null(40) ptr @_RNvCshDZgbIUcEv7_7___rustc12___rust_alloc(i64 noundef 40, i64 noundef 8) #8, !noalias !3
  %2 = icmp eq ptr %1, null
  br i1 %0, label %bb1.i, label %bb3.i

bb1.i:                                            ; preds = %start
  br i1 %2, label %bb2.i.i, label %example::get_animal::h6dc0715541cf6bf2.exit, !prof !6

bb3.i:                                            ; preds = %start
  br i1 %2, label %bb2.i2.i, label %example::get_animal::h6dc0715541cf6bf2.exit, !prof !6

example::get_animal::h6dc0715541cf6bf2.exit: ; preds = %bb1.i, %bb3.i
  %alloc_21d17cef229e1474a90c9b348f8ab787.sink.i = phi ptr [ @alloc_0aeadd5be856a09c17c0276c83aa6ecb, %bb1.i ], [ @alloc_21d17cef229e1474a90c9b348f8ab787, %bb3.i ]
  %alloc_beba4de3f16ebcaee9706459d9100c63.sink.i = phi ptr [ @alloc_dc969eeaf2d16587d1d724a5ed80dc67, %bb1.i ], [ @alloc_beba4de3f16ebcaee9706459d9100c63, %bb3.i ]
  %_0.sroa.3.0.i = phi ptr [ @vtable.0, %bb1.i ], [ @vtable.1, %bb3.i ]
  %3 = phi <2 x i64> [ <i64 4, i64 9>, %bb1.i ], [ <i64 8, i64 7>, %bb3.i ]
  store ptr %alloc_21d17cef229e1474a90c9b348f8ab787.sink.i, ptr %1, align 8
  %_5.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %1, i64 8
  store i64 5, ptr %_5.sroa.4.0..sroa_idx.i, align 8
  %_5.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %1, i64 16
  store ptr %alloc_beba4de3f16ebcaee9706459d9100c63.sink.i, ptr %_5.sroa.5.0..sroa_idx.i, align 8
  %_5.sroa.6.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %1, i64 24
  store <2 x i64> %3, ptr %_5.sroa.6.0..sroa_idx.i, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshDZgbIUcEv7_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #8
; call __rustc::__rust_alloc
  %4 = tail call noundef align 8 dereferenceable_or_null(40) ptr @_RNvCshDZgbIUcEv7_7___rustc12___rust_alloc(i64 noundef 40, i64 noundef 8) #8
  %5 = icmp eq ptr %4, null
  br i1 %5, label %bb2.i, label %bb10, !prof !6

bb10:                                             ; preds = %example::get_animal::h6dc0715541cf6bf2.exit
  store ptr @alloc_22f94bb69379f300d5861c457fea3b28, ptr %4, align 8
  %_15.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %4, i64 8
  store i64 9, ptr %_15.sroa.4.0..sroa_idx, align 8
  %_15.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %4, i64 16
  store ptr @alloc_de6c8e55ac0f214c9a4376ea54159ba2, ptr %_15.sroa.5.0..sroa_idx, align 8
  %_15.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %4, i64 24
  store i64 7, ptr %_15.sroa.6.0..sroa_idx, align 8
  %_15.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %4, i64 32
  store i64 2, ptr %_15.sroa.7.0..sroa_idx, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshDZgbIUcEv7_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #8, !noalias !3
; call __rustc::__rust_alloc
  %6 = tail call noundef dereferenceable_or_null(4) ptr @_RNvCshDZgbIUcEv7_7___rustc12___rust_alloc(i64 noundef range(i64 1, 0) 4, i64 noundef range(i64 1, -9223372036854775807) 1) #8, !noalias !3
  %7 = icmp eq ptr %6, null
  br i1 %7, label %bb3.i.i10.invoke, label %bb5, !prof !6

bb5:                                              ; preds = %bb10
  %_8 = icmp eq ptr %_0.sroa.3.0.i, @vtable.0
  %. = select i1 %_8, i32 2003789165, i32 1718579063
  store i32 %., ptr %6, align 1, !noalias !3
  store i64 4, ptr %_0, align 8
  %_19.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %6, ptr %_19.sroa.4.0._0.sroa_idx, align 8
  %_19.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 4, ptr %_19.sroa.5.0._0.sroa_idx, align 8
; call __rustc::__rust_dealloc
  tail call void @_RNvCshDZgbIUcEv7_7___rustc14___rust_dealloc(ptr noundef nonnull %4, i64 noundef range(i64 1, -9223372036854775808) 40, i64 noundef range(i64 1, -9223372036854775807) 8) #8
  ret void

;;;; cleanup/error bbs (can largely ignore for our purposes)
```

</details>

<details>

<summary>cleanup/error bbs</summary>

```llvm
bb2.i.i:                                          ; preds = %bb1.i
; call alloc::alloc::handle_alloc_error
  tail call void @_ZN5alloc5alloc18handle_alloc_error17h1c18310aad0b8a29E(i64 noundef 8, i64 noundef 40) #9, !noalias !7
  unreachable

bb2.i2.i:                                         ; preds = %bb3.i
; call alloc::alloc::handle_alloc_error
  tail call void @_ZN5alloc5alloc18handle_alloc_error17h1c18310aad0b8a29E(i64 noundef 8, i64 noundef 40) #9, !noalias !10
  unreachable

bb2.i:                                            ; preds = %example::get_animal::h6dc0715541cf6bf2.exit
; invoke alloc::alloc::handle_alloc_error
  invoke void @_ZN5alloc5alloc18handle_alloc_error17h1c18310aad0b8a29E(i64 noundef 8, i64 noundef 40) #9
          to label %.noexc unwind label %bb8

.noexc:                                           ; preds = %bb2.i
  unreachable

bb3.i.i10.invoke:                                 ; preds = %bb10
; invoke alloc::raw_vec::handle_error
  invoke void @_ZN5alloc7raw_vec12handle_error17h069bb0fee1c169b6E(i64 noundef 1, i64 4) #9
          to label %bb3.i.i10.cont unwind label %cleanup1

bb3.i.i10.cont:                                   ; preds = %bb3.i.i10.invoke
  unreachable

cleanup1:                                         ; preds = %bb3.i.i10.invoke
  %8 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>
  invoke fastcc void @"_ZN4core3ptr69drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$example..Animal$GT$$GT$17h0a0e2ba91be03f1bE"(ptr nonnull %4, ptr nonnull @vtable.0) #10
          to label %bb7 unwind label %terminate

terminate:                                        ; preds = %cleanup1, %bb8
  %9 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_ZN4core9panicking16panic_in_cleanup17h975505634b9400e7E() #11
  unreachable

bb7:                                              ; preds = %cleanup1, %bb8
  %.pn17 = phi { ptr, i32 } [ %10, %bb8 ], [ %8, %cleanup1 ]
  resume { ptr, i32 } %.pn17

bb8:                                              ; preds = %bb2.i
  %10 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn example::Animal>>
  invoke fastcc void @"_ZN4core3ptr69drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$example..Animal$GT$$GT$17h0a0e2ba91be03f1bE"(ptr nonnull %1, ptr nonnull %_0.sroa.3.0.i) #10
          to label %bb7 unwind label %terminate
}
```

</details>

start:
- compare %num to 0
- alloc mem (40 bytes) -> %1

example::get_animal::... : (same as above)
- first alloc: either "kitty" or "doggo" (1st struct field)
- second alloc: either "shoe" or "anywhere" (3rd struct field)
- %_0.sroa...: Cat or Dog vtable ptr
- %3: phi node for 2nd struct field

- store 1st struct field -> alloc mem
- store 5 -> alloc mem + 8
    - 5 might correspond to the length of the 1st field's string, but then why
      doesn't the 3rd field's string have a similar length field?
- store 3rd struct field -> alloc mem + 16
- store 2nd struct field -> alloc mem + 24

    - ^ struct packing? why is the order mixed up?

- alloc more mem (40 bytes) -> %4

bb10:
- store "catherine" -> 2nd alloc mem (name)
- store 9 -> 2nd alloc mem + 8 (name str len)
- store "shoebox" -> 2nd alloc mem + 16 (fav_toy)
- store 7 -> 2nd alloc mem + 24 (fav_toy str len)
- store 2 -> 2nd alloc mem + 32 (age)

- (^ same as above's bb8)

- alloc more mem (4 bytes) -> %6

bb5:
- vtable ptr comparison -> %_8
- %. = "meow" or "woof"
- store string in 4-byte mem
- store 4 -> %_0
- store 4-byte mem -> %_0 + 8
- store 4 -> %_0 + 16
- dealloc...


### observations/conclusions

contrary to the `simple` example observations, this `src_rw` IR seems more 
streamlined/has fewer LOCs but ends up being slower
- perhaps it has been optimized less, but if so, why?

`not_rw`
- calls the relevant vtable method
- memcopies the vtable retval (&str)
- then manually constructs the String retval using previous values

`src_rw`
- manually loads/stores the correct &str value (a constant depending on the
  relevant vtable) 
- then manually constructs the String retval

TODO would be interesting to see during which LLVM phase the two IRs diverge



## `vec`: `src_rw` performs worse than `not_rw`





### observations/conclusions

there might be something happening for _loops_ that reduces optimization
potential

TODO




