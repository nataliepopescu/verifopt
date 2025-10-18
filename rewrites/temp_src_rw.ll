; example::run_src_rw
; Function Attrs: nonlazybind uwtable
define void @_ZN7example10run_src_rw17hebf68f20b7d2582aE(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, i64 noundef %num) unnamed_addr #1 personality ptr @rust_eh_personality {
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshDZgbIUcEv7_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #8, !noalias !3
; call __rustc::__rust_alloc
  %0 = tail call noundef dereferenceable_or_null(4) ptr @_RNvCshDZgbIUcEv7_7___rustc12___rust_alloc(i64 noundef range(i64 1, 0) 4, i64 noundef range(i64 1, -9223372036854775807) 1) #8, !noalias !3
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb3.i.i7.invoke, label %bb6, !prof !6

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

terminate:                                        ; preds = %cleanup1
  %4 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_ZN4core9panicking16panic_in_cleanup17h975505634b9400e7E() #11
  unreachable

bb8:                                              ; preds = %cleanup1
  resume { ptr, i32 } %2
}


;;;;;

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


