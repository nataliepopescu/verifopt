# Benchmarks / Smoke Tests Oddities

## `simple` + `pub_trait`: `best` performs worse than `src_rw`

just looking at `simple` for simplicity... ha

the `src_rw` MIR is at least longer/more complicated than that of `best`,
but the LLVM IR actually ends up being more optimized...

recall that best is really just a static call (in addition to the `get_animal`
and `get_cat` calls that help it be a fairer comparison)
- if these additional calls are removed, then `best` still only performs just as
  well as `src_rw` for `simple`!
- that is wild

`best` LLVM IR:

```llvm
; example::run_best
; Function Attrs: nonlazybind uwtable
define void @_ZN7example8run_best17hec4ce4feafcb64c5E(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, i64 noundef %num, ptr noalias noundef nonnull readonly align 1 captures(none) %cat) unnamed_addr #1 personality ptr @rust_eh_personality {
  %0 = icmp eq i64 %num, 0
  %spec.select.i = select i1 %0, ptr @vtable.0, ptr @vtable.1
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


`src_rw` LLVM IR:

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











## `vec`: `src_rw` performs worse than `not_rw`

there might be something happening for _loops_ that reduces optimization
potential

TODO

