; ModuleID = 'stack_overflow.fb9a934af1a39ccd-cgu.0'
source_filename = "stack_overflow.fb9a934af1a39ccd-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "aarch64-unknown-linux-gnu"

%Big = type { [2000000 x i8] }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvNtCsjR135vb6FBx_3std2rt10lang_startuE0INtNtNtCs7STGCdL2atM_4core3ops8function6FnOnceuE9call_once6vtableCslBhqQiLn9nt_14stack_overflow, ptr @_RNCINvNtCsjR135vb6FBx_3std2rt10lang_startuE0CslBhqQiLn9nt_14stack_overflow, ptr @_RNCINvNtCsjR135vb6FBx_3std2rt10lang_startuE0CslBhqQiLn9nt_14stack_overflow }>, align 8
@alloc_476509aa7047bd805d630c6546685797 = private unnamed_addr constant [14 x i8] c"explicit panic", align 1
@alloc_10f227edfdb58242ee1bb3d8e8b52e5f = private unnamed_addr constant [22 x i8] c"src/stack_overflow.rs\00", align 1
@alloc_9887f478c096737b69952dd50f445217 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_10f227edfdb58242ee1bb3d8e8b52e5f, [16 x i8] c"\15\00\00\00\00\00\00\00\0C\00\00\00\09\00\00\00" }>, align 8
@alloc_7e80d81941cf5c819e3db4cff23967f9 = private unnamed_addr constant [72 x i8] c"`new_layout.size()` must be greater than or equal to `old_layout.size()`", align 1
@alloc_0bffe92e665ca7eb40305d3792d46f58 = private unnamed_addr constant [27 x i8] c"library/alloc/src/alloc.rs\00", align 1
@alloc_fff4fc10e7b599d802d0b863dc175ea8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0bffe92e665ca7eb40305d3792d46f58, [16 x i8] c"\1A\00\00\00\00\00\00\00\DF\00\00\00\09\00\00\00" }>, align 8
@anon.13aff46757640e471bf36f0f14e1b5b3.0 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@alloc_3d858ee5d55d08d896cafdf88815a8d0 = private unnamed_addr constant [58 x i8] c"assertion failed: old_layout.align() == new_layout.align()", align 1
@alloc_0a6a989224bce28eb2f5fb5f80443071 = private unnamed_addr constant [33 x i8] c"library/alloc/src/raw_vec/mod.rs\00", align 1
@alloc_744c66903f37c9b8dcfecbec02f9d5c4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0a6a989224bce28eb2f5fb5f80443071, [16 x i8] c" \00\00\00\00\00\00\00\18\02\00\00\0D\00\00\00" }>, align 8
@alloc_f618e816ca814de4958218055ad99397 = private unnamed_addr constant [32 x i8] c"assertion failed: additional > 0", align 1
@alloc_2e63d22236afbffcb89e2d0823f8afb0 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0a6a989224bce28eb2f5fb5f80443071, [16 x i8] c" \00\00\00\00\00\00\00\ED\01\00\00\09\00\00\00" }>, align 8
@anon.13aff46757640e471bf36f0f14e1b5b3.1 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] c"\01\00\00\00\00\00\00\80", [8 x i8] undef }>, align 8
@alloc_2b9352ac8aee68e075fb1d7daa438bea = private unnamed_addr constant [73 x i8] c"assertion failed: elem_layout.size() == elem_layout.pad_to_align().size()", align 1
@alloc_2fe82924fcf40f35157626e1ac5b5ac7 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0a6a989224bce28eb2f5fb5f80443071, [16 x i8] c" \00\00\00\00\00\00\00f\03\00\00\05\00\00\00" }>, align 8

; core::ptr::drop_in_place::<alloc::vec::Vec<stack_overflow::Big>>
; Function Attrs: uwtable
define internal void @_RINvNtCs7STGCdL2atM_4core3ptr13drop_in_placeINtNtCskPpRZcNh5S4_5alloc3vec3VecNtCslBhqQiLn9nt_14stack_overflow3BigEEB1f_(ptr noalias noundef align 8 dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
; invoke <alloc::vec::Vec<stack_overflow::Big> as core::ops::drop::Drop>::drop
  invoke void @_RNvXso_NtCskPpRZcNh5S4_5alloc3vecINtB5_3VecNtCslBhqQiLn9nt_14stack_overflow3BigENtNtNtCs7STGCdL2atM_4core3ops4drop4Drop4dropBH_(ptr noalias noundef align 8 dereferenceable(24) %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place::<alloc::raw_vec::RawVec<stack_overflow::Big>>
  invoke void @_RINvNtCs7STGCdL2atM_4core3ptr13drop_in_placeINtNtCskPpRZcNh5S4_5alloc7raw_vec6RawVecNtCslBhqQiLn9nt_14stack_overflow3BigEEB1m_(ptr noalias noundef align 8 dereferenceable(16) %_1) #21
          to label %bb1 unwind label %terminate

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  call void @llvm.lifetime.start.p0(ptr %0)
  store ptr %2, ptr %0, align 8
  %4 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %3, ptr %4, align 8
  br label %bb3

bb4:                                              ; preds = %start
; call core::ptr::drop_in_place::<alloc::raw_vec::RawVec<stack_overflow::Big>>
  call void @_RINvNtCs7STGCdL2atM_4core3ptr13drop_in_placeINtNtCskPpRZcNh5S4_5alloc7raw_vec6RawVecNtCslBhqQiLn9nt_14stack_overflow3BigEEB1m_(ptr noalias noundef align 8 dereferenceable(16) %_1)
  ret void

terminate:                                        ; preds = %bb3
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCs7STGCdL2atM_4core9panicking16panic_in_cleanup() #22
  unreachable

bb1:                                              ; preds = %bb3
  %6 = load ptr, ptr %0, align 8, !noundef !3
  %7 = getelementptr inbounds i8, ptr %0, i64 8
  %8 = load i32, ptr %7, align 8, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %0)
  %9 = insertvalue { ptr, i32 } poison, ptr %6, 0
  %10 = insertvalue { ptr, i32 } %9, i32 %8, 1
  resume { ptr, i32 } %10
}

; core::ptr::drop_in_place::<alloc::raw_vec::RawVec<stack_overflow::Big>>
; Function Attrs: uwtable
define internal void @_RINvNtCs7STGCdL2atM_4core3ptr13drop_in_placeINtNtCskPpRZcNh5S4_5alloc7raw_vec6RawVecNtCslBhqQiLn9nt_14stack_overflow3BigEEB1m_(ptr noalias noundef align 8 dereferenceable(16) %_1) unnamed_addr #0 {
start:
; call <alloc::raw_vec::RawVec<stack_overflow::Big> as core::ops::drop::Drop>::drop
  call void @_RNvXs1_NtCskPpRZcNh5S4_5alloc7raw_vecINtB5_6RawVecNtCslBhqQiLn9nt_14stack_overflow3BigENtNtNtCs7STGCdL2atM_4core3ops4drop4Drop4dropBO_(ptr noalias noundef align 8 dereferenceable(16) %_1)
  ret void
}

; std::rt::lang_start::<()>
; Function Attrs: uwtable
define hidden noundef i64 @_RINvNtCsjR135vb6FBx_3std2rt10lang_startuECslBhqQiLn9nt_14stack_overflow(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(ptr %_7)
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_RNvNtCsjR135vb6FBx_3std2rt19lang_start_internal(ptr noundef nonnull align 1 %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe)
  call void @llvm.lifetime.end.p0(ptr %_7)
  ret i64 %_0
}

; std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
; Function Attrs: noinline uwtable
define internal void @_RINvNtNtCsjR135vb6FBx_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECslBhqQiLn9nt_14stack_overflow(ptr noundef nonnull %f) unnamed_addr #1 {
start:
; call <fn() as core::ops::function::FnOnce<()>>::call_once
  call void @_RNvYFEuINtNtNtCs7STGCdL2atM_4core3ops8function6FnOnceuE9call_onceCslBhqQiLn9nt_14stack_overflow(ptr noundef nonnull %f) #23
  call void asm sideeffect "", "~{memory}"(), !srcloc !4
  ret void
}

; std::rt::lang_start::<()>::{closure#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNCINvNtCsjR135vb6FBx_3std2rt10lang_startuE0CslBhqQiLn9nt_14stack_overflow(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %_1) unnamed_addr #2 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  call void @_RINvNtNtCsjR135vb6FBx_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECslBhqQiLn9nt_14stack_overflow(ptr noundef nonnull %_4) #24
; call <() as std::process::Termination>::report
  %self = call noundef i8 @_RNvXsZ_NtCsjR135vb6FBx_3std7processuNtB5_11Termination6report() #23
  %_0 = zext i8 %self to i32
  ret i32 %_0
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNSNvYNCINvNtCsjR135vb6FBx_3std2rt10lang_startuE0INtNtNtCs7STGCdL2atM_4core3ops8function6FnOnceuE9call_once6vtableCslBhqQiLn9nt_14stack_overflow(ptr noundef %_1) unnamed_addr #2 {
start:
  %_2 = alloca [0 x i8], align 1
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once
  %_0 = call noundef i32 @_RNvYNCINvNtCsjR135vb6FBx_3std2rt10lang_startuE0INtNtNtCs7STGCdL2atM_4core3ops8function6FnOnceuE9call_onceCslBhqQiLn9nt_14stack_overflow(ptr noundef nonnull %0) #23
  ret i32 %_0
}

; stack_overflow::supersize_me
; Function Attrs: noinline uwtable
define internal void @_RNvCslBhqQiLn9nt_14stack_overflow12supersize_me(ptr noalias noundef align 8 dereferenceable(24) %out) unnamed_addr #1 {
start:
  %_6 = alloca [2000000 x i8], align 1
  %_4 = alloca [2000000 x i8], align 1
  %_2 = alloca [2000000 x i8], align 1
  call void @llvm.lifetime.start.p0(ptr %_2)
; call stack_overflow::meal
  call void @_RNvCslBhqQiLn9nt_14stack_overflow4meal(ptr noalias noundef sret([2000000 x i8]) align 1 captures(none) dereferenceable(2000000) %_2)
; call <alloc::vec::Vec<stack_overflow::Big>>::push_mut
  %_3 = call noundef nonnull align 1 ptr @_RNvMsF_NtCskPpRZcNh5S4_5alloc3vecINtB5_3VecNtCslBhqQiLn9nt_14stack_overflow3BigE8push_mutBH_(ptr noalias noundef align 8 dereferenceable(24) %out, ptr noalias noundef readonly align 1 captures(none) dereferenceable(2000000) %_2) #23
  call void @llvm.lifetime.end.p0(ptr %_2)
  call void @llvm.lifetime.start.p0(ptr %_4)
; call stack_overflow::meal
  call void @_RNvCslBhqQiLn9nt_14stack_overflow4meal(ptr noalias noundef sret([2000000 x i8]) align 1 captures(none) dereferenceable(2000000) %_4)
; call <alloc::vec::Vec<stack_overflow::Big>>::push_mut
  %_5 = call noundef nonnull align 1 ptr @_RNvMsF_NtCskPpRZcNh5S4_5alloc3vecINtB5_3VecNtCslBhqQiLn9nt_14stack_overflow3BigE8push_mutBH_(ptr noalias noundef align 8 dereferenceable(24) %out, ptr noalias noundef readonly align 1 captures(none) dereferenceable(2000000) %_4) #23
  call void @llvm.lifetime.end.p0(ptr %_4)
  call void @llvm.lifetime.start.p0(ptr %_6)
; call stack_overflow::meal
  call void @_RNvCslBhqQiLn9nt_14stack_overflow4meal(ptr noalias noundef sret([2000000 x i8]) align 1 captures(none) dereferenceable(2000000) %_6)
; call <alloc::vec::Vec<stack_overflow::Big>>::push_mut
  %_7 = call noundef nonnull align 1 ptr @_RNvMsF_NtCskPpRZcNh5S4_5alloc3vecINtB5_3VecNtCslBhqQiLn9nt_14stack_overflow3BigE8push_mutBH_(ptr noalias noundef align 8 dereferenceable(24) %out, ptr noalias noundef readonly align 1 captures(none) dereferenceable(2000000) %_6) #23
  call void @llvm.lifetime.end.p0(ptr %_6)
  ret void
}

; stack_overflow::main
; Function Attrs: uwtable
define hidden void @_RNvCslBhqQiLn9nt_14stack_overflow4main() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %1 = alloca [8 x i8], align 8
  %v = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(ptr %v)
  store i64 0, ptr %v, align 8
  %2 = getelementptr inbounds i8, ptr %v, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %v, i64 16
  store i64 0, ptr %3, align 8
  call void @llvm.lifetime.start.p0(ptr %1)
  store ptr %v, ptr %1, align 8
  call void asm sideeffect "", "r,~{memory}"(ptr %1), !srcloc !4
  %_2 = load ptr, ptr %1, align 8, !nonnull !3, !align !5, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %1)
; invoke stack_overflow::supersize_me
  invoke void @_RNvCslBhqQiLn9nt_14stack_overflow12supersize_me(ptr noalias noundef align 8 dereferenceable(24) %v)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<stack_overflow::Big>>
  invoke void @_RINvNtCs7STGCdL2atM_4core3ptr13drop_in_placeINtNtCskPpRZcNh5S4_5alloc3vec3VecNtCslBhqQiLn9nt_14stack_overflow3BigEEB1f_(ptr noalias noundef align 8 dereferenceable(24) %v) #21
          to label %bb4 unwind label %terminate

cleanup:                                          ; preds = %start
  %4 = landingpad { ptr, i32 }
          cleanup
  %5 = extractvalue { ptr, i32 } %4, 0
  %6 = extractvalue { ptr, i32 } %4, 1
  call void @llvm.lifetime.start.p0(ptr %0)
  store ptr %5, ptr %0, align 8
  %7 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %6, ptr %7, align 8
  br label %bb3

bb1:                                              ; preds = %start
; call core::ptr::drop_in_place::<alloc::vec::Vec<stack_overflow::Big>>
  call void @_RINvNtCs7STGCdL2atM_4core3ptr13drop_in_placeINtNtCskPpRZcNh5S4_5alloc3vec3VecNtCslBhqQiLn9nt_14stack_overflow3BigEEB1f_(ptr noalias noundef align 8 dereferenceable(24) %v)
  call void @llvm.lifetime.end.p0(ptr %v)
  ret void

terminate:                                        ; preds = %bb3
  %8 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCs7STGCdL2atM_4core9panicking16panic_in_cleanup() #22
  unreachable

bb4:                                              ; preds = %bb3
  %9 = load ptr, ptr %0, align 8, !noundef !3
  %10 = getelementptr inbounds i8, ptr %0, i64 8
  %11 = load i32, ptr %10, align 8, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %0)
  %12 = insertvalue { ptr, i32 } poison, ptr %9, 0
  %13 = insertvalue { ptr, i32 } %12, i32 %11, 1
  resume { ptr, i32 } %13
}

; stack_overflow::meal
; Function Attrs: uwtable
define internal void @_RNvCslBhqQiLn9nt_14stack_overflow4meal(ptr dead_on_unwind noalias noundef writable sret([2000000 x i8]) align 1 captures(none) dereferenceable(2000000) %_0) unnamed_addr #0 {
start:
  %0 = alloca [1 x i8], align 1
  %_3 = alloca [2000000 x i8], align 1
  call void @llvm.lifetime.start.p0(ptr %0)
  store i8 0, ptr %0, align 1
  call void asm sideeffect "", "r,~{memory}"(ptr %0), !srcloc !4
  %1 = load i8, ptr %0, align 1, !range !6, !noundef !3
  %_1 = trunc nuw i8 %1 to i1
  call void @llvm.lifetime.end.p0(ptr %0)
  %2 = call i1 @llvm.expect.i1(i1 %_1, i1 false)
  br i1 %2, label %bb1, label %bb2

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(ptr %_3)
  call void @llvm.memset.p0.i64(ptr align 1 %_3, i8 0, i64 2000000, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %_0, ptr align 1 %_3, i64 2000000, i1 false)
  call void @llvm.lifetime.end.p0(ptr %_3)
  ret void

bb1:                                              ; preds = %start
; call core::panicking::panic
  call void @_RNvNtCs7STGCdL2atM_4core9panicking5panic(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_476509aa7047bd805d630c6546685797, i64 noundef 14, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9887f478c096737b69952dd50f445217) #25
  unreachable
}

; <alloc::alloc::Global>::grow_impl_runtime
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @_RNvMNtCskPpRZcNh5S4_5alloc5allocNtB2_6Global17grow_impl_runtime(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self, ptr noundef nonnull %ptr, i64 noundef range(i64 1, -9223372036854775807) %old_layout.0, i64 noundef %old_layout.1, i64 noundef range(i64 1, -9223372036854775807) %new_layout.0, i64 noundef %new_layout.1, i1 noundef zeroext %zeroed) unnamed_addr #2 {
start:
  %self3 = alloca [16 x i8], align 8
  %_33 = alloca [16 x i8], align 8
  %self2 = alloca [8 x i8], align 8
  %self1 = alloca [8 x i8], align 8
  %_24 = alloca [8 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %_6 = icmp uge i64 %new_layout.1, %old_layout.1
  %0 = call i1 @llvm.expect.i1(i1 %_6, i1 true)
  br i1 %0, label %bb1, label %bb2

bb2:                                              ; preds = %start
  call void @llvm.assume(i1 true)
  %1 = icmp ne ptr inttoptr (i64 145 to ptr), null
  call void @llvm.assume(i1 %1)
; call core::panicking::panic_fmt
  call void @_RNvNtCs7STGCdL2atM_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_7e80d81941cf5c819e3db4cff23967f9, ptr noundef nonnull inttoptr (i64 145 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fff4fc10e7b599d802d0b863dc175ea8) #25
  unreachable

bb1:                                              ; preds = %start
  %2 = icmp eq i64 %old_layout.1, 0
  br i1 %2, label %bb4, label %bb3

bb4:                                              ; preds = %bb1
; call <alloc::alloc::Global>::alloc_impl_runtime
  %3 = call { ptr, i64 } @_RNvMNtCskPpRZcNh5S4_5alloc5allocNtB2_6Global18alloc_impl_runtime(i64 noundef %new_layout.0, i64 noundef %new_layout.1, i1 noundef zeroext %zeroed) #23
  %4 = extractvalue { ptr, i64 } %3, 0
  %5 = extractvalue { ptr, i64 } %3, 1
  store ptr %4, ptr %_0, align 8
  %6 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %5, ptr %6, align 8
  br label %bb10

bb3:                                              ; preds = %bb1
  %7 = sub i64 %old_layout.0, 1
  %8 = icmp ule i64 %7, 9223372036854775807
  call void @llvm.assume(i1 %8)
  %9 = sub i64 %new_layout.0, 1
  %10 = icmp ule i64 %9, 9223372036854775807
  call void @llvm.assume(i1 %10)
  %_15 = icmp eq i64 %old_layout.0, %new_layout.0
  br i1 %_15, label %bb5, label %bb6

bb10:                                             ; preds = %bb30, %bb16, %bb31, %bb35, %bb4
  %11 = load ptr, ptr %_0, align 8, !noundef !3
  %12 = getelementptr inbounds i8, ptr %_0, i64 8
  %13 = load i64, ptr %12, align 8
  %14 = insertvalue { ptr, i64 } poison, ptr %11, 0
  %15 = insertvalue { ptr, i64 } %14, i64 %13, 1
  ret { ptr, i64 } %15

bb6:                                              ; preds = %bb3
  call void @llvm.lifetime.start.p0(ptr %_33)
  call void @llvm.lifetime.start.p0(ptr %self3)
; call <alloc::alloc::Global>::alloc_impl_runtime
  %16 = call { ptr, i64 } @_RNvMNtCskPpRZcNh5S4_5alloc5allocNtB2_6Global18alloc_impl_runtime(i64 noundef %new_layout.0, i64 noundef %new_layout.1, i1 noundef zeroext %zeroed) #23
  %17 = extractvalue { ptr, i64 } %16, 0
  %18 = extractvalue { ptr, i64 } %16, 1
  store ptr %17, ptr %self3, align 8
  %19 = getelementptr inbounds i8, ptr %self3, i64 8
  store i64 %18, ptr %19, align 8
  %20 = load ptr, ptr %self3, align 8, !noundef !3
  %21 = getelementptr inbounds i8, ptr %self3, i64 8
  %22 = load i64, ptr %21, align 8
  %23 = ptrtoint ptr %20 to i64
  %24 = icmp eq i64 %23, 0
  %_74 = select i1 %24, i64 1, i64 0
  %25 = trunc nuw i64 %_74 to i1
  br i1 %25, label %bb31, label %bb32

bb5:                                              ; preds = %bb3
  br label %bb14

bb31:                                             ; preds = %bb6
  call void @llvm.lifetime.end.p0(ptr %self3)
  call void @llvm.assume(i1 true)
  %26 = load ptr, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, align 8, !noundef !3
  %27 = load i64, ptr getelementptr inbounds (i8, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, i64 8), align 8
  store ptr %26, ptr %_0, align 8
  %28 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %27, ptr %28, align 8
  call void @llvm.lifetime.end.p0(ptr %_33)
  br label %bb10

bb32:                                             ; preds = %bb6
  %v.0 = load ptr, ptr %self3, align 8, !nonnull !3, !noundef !3
  %29 = getelementptr inbounds i8, ptr %self3, i64 8
  %v.1 = load i64, ptr %29, align 8, !noundef !3
  store ptr %v.0, ptr %_33, align 8
  %30 = getelementptr inbounds i8, ptr %_33, i64 8
  store i64 %v.1, ptr %30, align 8
  call void @llvm.lifetime.end.p0(ptr %self3)
  %new_ptr.0 = load ptr, ptr %_33, align 8, !nonnull !3, !noundef !3
  %31 = getelementptr inbounds i8, ptr %_33, i64 8
  %new_ptr.1 = load i64, ptr %31, align 8, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %_33)
  %32 = icmp ne ptr %ptr, null
  call void @llvm.assume(i1 %32)
  %33 = icmp ne ptr %ptr, null
  call void @llvm.assume(i1 %33)
  %34 = icmp ne ptr %new_ptr.0, null
  call void @llvm.assume(i1 %34)
  br label %bb35

bb35:                                             ; preds = %bb32
  %35 = mul i64 %old_layout.1, 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %new_ptr.0, ptr align 1 %ptr, i64 %35, i1 false)
; call __rustc::__rust_dealloc
  call void @_RNvCs2fcwfXhWpkc_7___rustc14___rust_dealloc(ptr noundef %ptr, i64 noundef %old_layout.1, i64 noundef %old_layout.0) #26
  store ptr %new_ptr.0, ptr %_0, align 8
  %36 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %new_ptr.1, ptr %36, align 8
  br label %bb10

bb7:                                              ; No predecessors!
  unreachable

bb14:                                             ; preds = %bb5
  %37 = icmp ne ptr %ptr, null
  call void @llvm.assume(i1 %37)
; call __rustc::__rust_realloc
  %raw_ptr = call noundef ptr @_RNvCs2fcwfXhWpkc_7___rustc14___rust_realloc(ptr noundef %ptr, i64 noundef %old_layout.1, i64 noundef %old_layout.0, i64 noundef %new_layout.1) #26
  call void @llvm.lifetime.start.p0(ptr %_24)
  call void @llvm.lifetime.start.p0(ptr %self1)
  call void @llvm.lifetime.start.p0(ptr %self2)
  %_57 = ptrtoint ptr %raw_ptr to i64
  %38 = icmp eq i64 %_57, 0
  br i1 %38, label %bb16, label %bb17

bb16:                                             ; preds = %bb14
  store ptr null, ptr %self2, align 8
  store ptr null, ptr %self1, align 8
  call void @llvm.lifetime.end.p0(ptr %self2)
  call void @llvm.lifetime.end.p0(ptr %self1)
  call void @llvm.assume(i1 true)
  %39 = load ptr, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, align 8, !noundef !3
  %40 = load i64, ptr getelementptr inbounds (i8, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, i64 8), align 8
  store ptr %39, ptr %_0, align 8
  %41 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %40, ptr %41, align 8
  call void @llvm.lifetime.end.p0(ptr %_24)
  br label %bb10

bb17:                                             ; preds = %bb14
  br label %bb20

bb20:                                             ; preds = %bb17
  %42 = icmp ne ptr %raw_ptr, null
  call void @llvm.assume(i1 %42)
  store ptr %raw_ptr, ptr %self2, align 8
  %v = load ptr, ptr %self2, align 8, !nonnull !3, !noundef !3
  store ptr %v, ptr %self1, align 8
  call void @llvm.lifetime.end.p0(ptr %self2)
  %v4 = load ptr, ptr %self1, align 8, !nonnull !3, !noundef !3
  store ptr %v4, ptr %_24, align 8
  call void @llvm.lifetime.end.p0(ptr %self1)
  %ptr5 = load ptr, ptr %_24, align 8, !nonnull !3, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %_24)
  br i1 %zeroed, label %bb8, label %bb9

bb9:                                              ; preds = %bb27, %bb20
  %43 = icmp ne ptr %ptr5, null
  call void @llvm.assume(i1 %43)
  br label %bb30

bb8:                                              ; preds = %bb20
  br label %bb23

bb23:                                             ; preds = %bb8
  %self6 = getelementptr inbounds nuw i8, ptr %raw_ptr, i64 %old_layout.1
  %count = sub i64 %new_layout.1, %old_layout.1
  br label %bb27

bb27:                                             ; preds = %bb23
  %44 = mul i64 1, %count
  call void @llvm.memset.p0.i64(ptr align 1 %self6, i8 0, i64 %44, i1 false)
  br label %bb9

bb30:                                             ; preds = %bb9
  %45 = icmp ne ptr %ptr5, null
  call void @llvm.assume(i1 %45)
  store ptr %ptr5, ptr %_0, align 8
  %46 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %new_layout.1, ptr %46, align 8
  br label %bb10

bb13:                                             ; No predecessors!
  unreachable

bb18:                                             ; No predecessors!
  unreachable

bb19:                                             ; No predecessors!
  unreachable

bb21:                                             ; No predecessors!
  unreachable

bb22:                                             ; No predecessors!
  unreachable

bb25:                                             ; No predecessors!
  unreachable

bb26:                                             ; No predecessors!
  unreachable

bb28:                                             ; No predecessors!
  unreachable

bb29:                                             ; No predecessors!
  unreachable

bb33:                                             ; No predecessors!
  unreachable

bb34:                                             ; No predecessors!
  unreachable
}

; <alloc::alloc::Global>::alloc_impl_runtime
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @_RNvMNtCskPpRZcNh5S4_5alloc5allocNtB2_6Global18alloc_impl_runtime(i64 noundef range(i64 1, -9223372036854775807) %layout.0, i64 noundef %layout.1, i1 noundef zeroext %zeroed) unnamed_addr #2 {
start:
  %self1 = alloca [8 x i8], align 8
  %self = alloca [8 x i8], align 8
  %_9 = alloca [8 x i8], align 8
  %raw_ptr = alloca [8 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %0 = icmp eq i64 %layout.1, 0
  br i1 %0, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %1 = sub i64 %layout.0, 1
  %2 = icmp ule i64 %1, 9223372036854775807
  call void @llvm.assume(i1 %2)
  %_16 = inttoptr i64 %layout.0 to ptr
  %3 = icmp ne ptr %_16, null
  call void @llvm.assume(i1 %3)
  %4 = sub i64 %layout.0, 1
  %5 = icmp ule i64 %4, 9223372036854775807
  call void @llvm.assume(i1 %5)
  %data = inttoptr i64 %layout.0 to ptr
  br label %bb9

bb1:                                              ; preds = %start
  br i1 %zeroed, label %bb3, label %bb4

bb9:                                              ; preds = %bb2
  %6 = icmp ne ptr %data, null
  call void @llvm.assume(i1 %6)
  store ptr %data, ptr %_0, align 8
  %7 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 0, ptr %7, align 8
  br label %bb6

bb6:                                              ; preds = %bb21, %bb14, %bb9
  %8 = load ptr, ptr %_0, align 8, !noundef !3
  %9 = getelementptr inbounds i8, ptr %_0, i64 8
  %10 = load i64, ptr %9, align 8
  %11 = insertvalue { ptr, i64 } poison, ptr %8, 0
  %12 = insertvalue { ptr, i64 } %11, i64 %10, 1
  ret { ptr, i64 } %12

bb4:                                              ; preds = %bb1
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCs2fcwfXhWpkc_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #26
; call __rustc::__rust_alloc
  %13 = call noundef ptr @_RNvCs2fcwfXhWpkc_7___rustc12___rust_alloc(i64 noundef %layout.1, i64 noundef %layout.0) #26
  store ptr %13, ptr %raw_ptr, align 8
  br label %bb5

bb3:                                              ; preds = %bb1
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCs2fcwfXhWpkc_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #26
; call __rustc::__rust_alloc_zeroed
  %14 = call noundef ptr @_RNvCs2fcwfXhWpkc_7___rustc19___rust_alloc_zeroed(i64 noundef %layout.1, i64 noundef %layout.0) #26
  store ptr %14, ptr %raw_ptr, align 8
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  call void @llvm.lifetime.start.p0(ptr %_9)
  call void @llvm.lifetime.start.p0(ptr %self)
  call void @llvm.lifetime.start.p0(ptr %self1)
  %_31 = load ptr, ptr %raw_ptr, align 8, !noundef !3
  %15 = load ptr, ptr %raw_ptr, align 8, !noundef !3
  %_32 = ptrtoint ptr %15 to i64
  %16 = icmp eq i64 %_32, 0
  br i1 %16, label %bb14, label %bb15

bb14:                                             ; preds = %bb5
  store ptr null, ptr %self1, align 8
  store ptr null, ptr %self, align 8
  call void @llvm.lifetime.end.p0(ptr %self1)
  call void @llvm.lifetime.end.p0(ptr %self)
  call void @llvm.assume(i1 true)
  %17 = load ptr, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, align 8, !noundef !3
  %18 = load i64, ptr getelementptr inbounds (i8, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, i64 8), align 8
  store ptr %17, ptr %_0, align 8
  %19 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %18, ptr %19, align 8
  call void @llvm.lifetime.end.p0(ptr %_9)
  br label %bb6

bb15:                                             ; preds = %bb5
  br label %bb18

bb18:                                             ; preds = %bb15
  %20 = icmp ne ptr %_31, null
  call void @llvm.assume(i1 %20)
  store ptr %_31, ptr %self1, align 8
  %v = load ptr, ptr %self1, align 8, !nonnull !3, !noundef !3
  store ptr %v, ptr %self, align 8
  call void @llvm.lifetime.end.p0(ptr %self1)
  %v2 = load ptr, ptr %self, align 8, !nonnull !3, !noundef !3
  store ptr %v2, ptr %_9, align 8
  call void @llvm.lifetime.end.p0(ptr %self)
  %ptr = load ptr, ptr %_9, align 8, !nonnull !3, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %_9)
  %21 = icmp ne ptr %ptr, null
  call void @llvm.assume(i1 %21)
  br label %bb21

bb21:                                             ; preds = %bb18
  %22 = icmp ne ptr %ptr, null
  call void @llvm.assume(i1 %22)
  store ptr %ptr, ptr %_0, align 8
  %23 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %layout.1, ptr %23, align 8
  br label %bb6

bb7:                                              ; No predecessors!
  unreachable

bb8:                                              ; No predecessors!
  unreachable

bb16:                                             ; No predecessors!
  unreachable

bb17:                                             ; No predecessors!
  unreachable

bb19:                                             ; No predecessors!
  unreachable

bb20:                                             ; No predecessors!
  unreachable
}

; <alloc::raw_vec::RawVecInner>::deallocate
; Function Attrs: uwtable
define internal void @_RNvMs2_NtCskPpRZcNh5S4_5alloc7raw_vecNtB5_11RawVecInner10deallocateCslBhqQiLn9nt_14stack_overflow(ptr noalias noundef align 8 dereferenceable(16) %self, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0, i64 noundef %elem_layout.1) unnamed_addr #0 {
start:
  %_3 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(ptr %_3)
; call <alloc::raw_vec::RawVecInner>::current_memory
  call void @_RNvMs2_NtCskPpRZcNh5S4_5alloc7raw_vecNtB5_11RawVecInner14current_memoryCslBhqQiLn9nt_14stack_overflow(ptr noalias noundef sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, i64 noundef %elem_layout.0, i64 noundef %elem_layout.1) #23
  %0 = getelementptr inbounds i8, ptr %_3, i64 8
  %1 = load i64, ptr %0, align 8, !range !7, !noundef !3
  %2 = icmp eq i64 %1, 0
  %_5 = select i1 %2, i64 0, i64 1
  %3 = trunc nuw i64 %_5 to i1
  br i1 %3, label %bb2, label %bb4

bb2:                                              ; preds = %start
  %ptr = load ptr, ptr %_3, align 8, !nonnull !3, !noundef !3
  %4 = getelementptr inbounds i8, ptr %_3, i64 8
  %layout.0 = load i64, ptr %4, align 8, !range !8, !noundef !3
  %5 = getelementptr inbounds i8, ptr %4, i64 8
  %layout.1 = load i64, ptr %5, align 8, !noundef !3
  %_9 = getelementptr inbounds i8, ptr %self, i64 16
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @_RNvXs_NtCskPpRZcNh5S4_5alloc5allocNtB4_6GlobalNtNtCs7STGCdL2atM_4core5alloc9Allocator10deallocate(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_9, ptr noundef nonnull %ptr, i64 noundef %layout.0, i64 noundef %layout.1) #23
  call void @llvm.lifetime.end.p0(ptr %_3)
  br label %bb5

bb4:                                              ; preds = %start
  call void @llvm.lifetime.end.p0(ptr %_3)
  br label %bb5

bb5:                                              ; preds = %bb4, %bb2
  ret void

bb6:                                              ; No predecessors!
  unreachable
}

; <alloc::raw_vec::RawVecInner>::current_memory
; Function Attrs: inlinehint uwtable
define internal void @_RNvMs2_NtCskPpRZcNh5S4_5alloc7raw_vecNtB5_11RawVecInner14current_memoryCslBhqQiLn9nt_14stack_overflow(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0, i64 noundef %elem_layout.1) unnamed_addr #2 {
start:
  %_14 = alloca [24 x i8], align 8
  %0 = icmp eq i64 %elem_layout.1, 0
  br i1 %0, label %bb3, label %bb1

bb3:                                              ; preds = %bb2, %start
  %1 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 0, ptr %1, align 8
  br label %bb5

bb1:                                              ; preds = %start
  %self1 = load i64, ptr %self, align 8, !range !9, !noundef !3
  %2 = icmp ule i64 %self1, 9223372036854775807
  call void @llvm.assume(i1 %2)
  %3 = icmp eq i64 %self1, 0
  br i1 %3, label %bb2, label %bb4

bb2:                                              ; preds = %bb1
  br label %bb3

bb4:                                              ; preds = %bb1
  %self2 = load i64, ptr %self, align 8, !range !9, !noundef !3
  %4 = icmp ule i64 %self2, 9223372036854775807
  call void @llvm.assume(i1 %4)
  br label %bb7

bb5:                                              ; preds = %bb9, %bb3
  ret void

bb7:                                              ; preds = %bb4
  %alloc_size = mul nuw i64 %elem_layout.1, %self2
  %5 = sub i64 %elem_layout.0, 1
  %6 = icmp ule i64 %5, 9223372036854775807
  call void @llvm.assume(i1 %6)
  br label %bb9

bb9:                                              ; preds = %bb7
  call void @llvm.lifetime.start.p0(ptr %_14)
  %7 = getelementptr inbounds i8, ptr %self, i64 8
  %self3 = load ptr, ptr %7, align 8, !nonnull !3, !noundef !3
  store ptr %self3, ptr %_14, align 8
  %8 = getelementptr inbounds i8, ptr %_14, i64 8
  store i64 %elem_layout.0, ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 %alloc_size, ptr %9, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_14, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(ptr %_14)
  br label %bb5

bb6:                                              ; No predecessors!
  unreachable

bb8:                                              ; No predecessors!
  unreachable
}

; <alloc::raw_vec::RawVec<stack_overflow::Big>>::grow_one
; Function Attrs: noinline uwtable
define internal void @_RNvMs3_NtCskPpRZcNh5S4_5alloc7raw_vecINtB5_6RawVecNtCslBhqQiLn9nt_14stack_overflow3BigE8grow_oneBO_(ptr noalias noundef align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %_3 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(ptr %_3)
  %self1 = load i64, ptr %self, align 8, !range !9, !noundef !3
  %0 = icmp ule i64 %self1, 9223372036854775807
  call void @llvm.assume(i1 %0)
; call <alloc::raw_vec::RawVecInner>::grow_amortized
  %1 = call { i64, i64 } @_RNvMs4_NtCskPpRZcNh5S4_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCslBhqQiLn9nt_14stack_overflow(ptr noalias noundef align 8 dereferenceable(16) %self, i64 noundef %self1, i64 noundef 1, i64 noundef 1, i64 noundef 2000000)
  %2 = extractvalue { i64, i64 } %1, 0
  %3 = extractvalue { i64, i64 } %1, 1
  store i64 %2, ptr %_3, align 8
  %4 = getelementptr inbounds i8, ptr %_3, i64 8
  store i64 %3, ptr %4, align 8
  %5 = load i64, ptr %_3, align 8, !range !10, !noundef !3
  %6 = getelementptr inbounds i8, ptr %_3, i64 8
  %7 = load i64, ptr %6, align 8
  %8 = icmp eq i64 %5, -9223372036854775807
  %_6 = select i1 %8, i64 0, i64 1
  %9 = trunc nuw i64 %_6 to i1
  %10 = call i1 @llvm.expect.i1(i1 %9, i1 false)
  br i1 %10, label %bb2, label %bb3

bb2:                                              ; preds = %start
  %err.0 = load i64, ptr %_3, align 8, !range !7, !noundef !3
  %11 = getelementptr inbounds i8, ptr %_3, i64 8
  %err.1 = load i64, ptr %11, align 8
; call alloc::raw_vec::handle_error
  call void @_RNvNtCskPpRZcNh5S4_5alloc7raw_vec12handle_error(i64 noundef %err.0, i64 %err.1) #27
  unreachable

bb3:                                              ; preds = %start
  call void @llvm.lifetime.end.p0(ptr %_3)
  ret void

bb4:                                              ; No predecessors!
  unreachable
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold uwtable
define internal void @_RNvMs4_NtCskPpRZcNh5S4_5alloc7raw_vecNtB5_11RawVecInner11finish_growCslBhqQiLn9nt_14stack_overflow(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, i64 noundef %cap, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0, i64 noundef %elem_layout.1) unnamed_addr #3 {
start:
  %_9 = alloca [24 x i8], align 8
  %memory = alloca [16 x i8], align 8
  %self1 = alloca [24 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(ptr %_4)
  call void @llvm.lifetime.start.p0(ptr %self1)
; call alloc::raw_vec::layout_array
  call void @_RNvNtCskPpRZcNh5S4_5alloc7raw_vec12layout_array(ptr noalias noundef sret([24 x i8]) align 8 captures(none) dereferenceable(24) %self1, i64 noundef %cap, i64 noundef %elem_layout.0, i64 noundef %elem_layout.1) #23
  %_27 = load i64, ptr %self1, align 8, !range !11, !noundef !3
  %0 = trunc nuw i64 %_27 to i1
  br i1 %0, label %bb15, label %bb16

bb15:                                             ; preds = %start
  %1 = getelementptr inbounds i8, ptr %self1, i64 8
  %e.0 = load i64, ptr %1, align 8, !range !7, !noundef !3
  %2 = getelementptr inbounds i8, ptr %1, i64 8
  %e.1 = load i64, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %_4, i64 8
  store i64 %e.0, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  store i64 %e.1, ptr %4, align 8
  store i64 1, ptr %_4, align 8
  call void @llvm.lifetime.end.p0(ptr %self1)
  %5 = getelementptr inbounds i8, ptr %_4, i64 8
  %residual.0 = load i64, ptr %5, align 8, !range !7, !noundef !3
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  %residual.1 = load i64, ptr %6, align 8
  call void @llvm.assume(i1 true)
  %7 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %residual.0, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %7, i64 8
  store i64 %residual.1, ptr %8, align 8
  store i64 1, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(ptr %_4)
  br label %bb14

bb16:                                             ; preds = %start
  %9 = getelementptr inbounds i8, ptr %self1, i64 8
  %v.0 = load i64, ptr %9, align 8, !range !8, !noundef !3
  %10 = getelementptr inbounds i8, ptr %9, i64 8
  %v.1 = load i64, ptr %10, align 8, !noundef !3
  %11 = getelementptr inbounds i8, ptr %_4, i64 8
  store i64 %v.0, ptr %11, align 8
  %12 = getelementptr inbounds i8, ptr %11, i64 8
  store i64 %v.1, ptr %12, align 8
  store i64 0, ptr %_4, align 8
  call void @llvm.lifetime.end.p0(ptr %self1)
  %13 = getelementptr inbounds i8, ptr %_4, i64 8
  %new_layout.0 = load i64, ptr %13, align 8, !range !8, !noundef !3
  %14 = getelementptr inbounds i8, ptr %13, i64 8
  %new_layout.1 = load i64, ptr %14, align 8, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %_4)
  call void @llvm.lifetime.start.p0(ptr %memory)
  call void @llvm.lifetime.start.p0(ptr %_9)
; call <alloc::raw_vec::RawVecInner>::current_memory
  call void @_RNvMs2_NtCskPpRZcNh5S4_5alloc7raw_vecNtB5_11RawVecInner14current_memoryCslBhqQiLn9nt_14stack_overflow(ptr noalias noundef sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_9, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, i64 noundef %elem_layout.0, i64 noundef %elem_layout.1) #23
  %15 = getelementptr inbounds i8, ptr %_9, i64 8
  %16 = load i64, ptr %15, align 8, !range !7, !noundef !3
  %17 = icmp eq i64 %16, 0
  %_10 = select i1 %17, i64 0, i64 1
  %18 = trunc nuw i64 %_10 to i1
  br i1 %18, label %bb4, label %bb8

bb4:                                              ; preds = %bb16
  %ptr = load ptr, ptr %_9, align 8, !nonnull !3, !noundef !3
  %19 = getelementptr inbounds i8, ptr %_9, i64 8
  %old_layout.0 = load i64, ptr %19, align 8, !range !8, !noundef !3
  %20 = getelementptr inbounds i8, ptr %19, i64 8
  %old_layout.1 = load i64, ptr %20, align 8, !noundef !3
  %21 = sub i64 %old_layout.0, 1
  %22 = icmp ule i64 %21, 9223372036854775807
  call void @llvm.assume(i1 %22)
  %23 = sub i64 %new_layout.0, 1
  %24 = icmp ule i64 %23, 9223372036854775807
  call void @llvm.assume(i1 %24)
  %_13 = icmp eq i64 %old_layout.0, %new_layout.0
  %25 = call i1 @llvm.expect.i1(i1 %_13, i1 true)
  br i1 %25, label %bb5, label %bb6

bb8:                                              ; preds = %bb16
  call void @llvm.lifetime.end.p0(ptr %_9)
  %_22 = getelementptr inbounds i8, ptr %self, i64 16
; call <alloc::alloc::Global as core::alloc::Allocator>::allocate
  %26 = call { ptr, i64 } @_RNvXs_NtCskPpRZcNh5S4_5alloc5allocNtB4_6GlobalNtNtCs7STGCdL2atM_4core5alloc9Allocator8allocate(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_22, i64 noundef %new_layout.0, i64 noundef %new_layout.1) #23
  %27 = extractvalue { ptr, i64 } %26, 0
  %28 = extractvalue { ptr, i64 } %26, 1
  store ptr %27, ptr %memory, align 8
  %29 = getelementptr inbounds i8, ptr %memory, i64 8
  store i64 %28, ptr %29, align 8
  br label %bb10

bb6:                                              ; preds = %bb4
; call core::panicking::panic
  call void @_RNvNtCs7STGCdL2atM_4core9panicking5panic(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_3d858ee5d55d08d896cafdf88815a8d0, i64 noundef 58, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_744c66903f37c9b8dcfecbec02f9d5c4) #25
  unreachable

bb5:                                              ; preds = %bb4
  br label %bb18

bb18:                                             ; preds = %bb5
  %_21 = getelementptr inbounds i8, ptr %self, i64 16
; call <alloc::alloc::Global as core::alloc::Allocator>::grow
  %30 = call { ptr, i64 } @_RNvXs_NtCskPpRZcNh5S4_5alloc5allocNtB4_6GlobalNtNtCs7STGCdL2atM_4core5alloc9Allocator4grow(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_21, ptr noundef nonnull %ptr, i64 noundef %old_layout.0, i64 noundef %old_layout.1, i64 noundef %new_layout.0, i64 noundef %new_layout.1) #23
  %31 = extractvalue { ptr, i64 } %30, 0
  %32 = extractvalue { ptr, i64 } %30, 1
  store ptr %31, ptr %memory, align 8
  %33 = getelementptr inbounds i8, ptr %memory, i64 8
  store i64 %32, ptr %33, align 8
  call void @llvm.lifetime.end.p0(ptr %_9)
  br label %bb10

bb10:                                             ; preds = %bb8, %bb18
  %34 = load ptr, ptr %memory, align 8, !noundef !3
  %35 = getelementptr inbounds i8, ptr %memory, i64 8
  %36 = load i64, ptr %35, align 8
  %37 = ptrtoint ptr %34 to i64
  %38 = icmp eq i64 %37, 0
  %_23 = select i1 %38, i64 1, i64 0
  %39 = trunc nuw i64 %_23 to i1
  br i1 %39, label %bb11, label %bb12

bb11:                                             ; preds = %bb10
  %40 = sub i64 %new_layout.0, 1
  %41 = icmp ule i64 %40, 9223372036854775807
  call void @llvm.assume(i1 %41)
  %42 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %new_layout.0, ptr %42, align 8
  %43 = getelementptr inbounds i8, ptr %42, i64 8
  store i64 %new_layout.1, ptr %43, align 8
  store i64 1, ptr %_0, align 8
  br label %bb13

bb12:                                             ; preds = %bb10
  %memory.0 = load ptr, ptr %memory, align 8, !nonnull !3, !noundef !3
  %44 = getelementptr inbounds i8, ptr %memory, i64 8
  %memory.1 = load i64, ptr %44, align 8, !noundef !3
  %45 = getelementptr inbounds i8, ptr %_0, i64 8
  store ptr %memory.0, ptr %45, align 8
  %46 = getelementptr inbounds i8, ptr %45, i64 8
  store i64 %memory.1, ptr %46, align 8
  store i64 0, ptr %_0, align 8
  br label %bb13

bb13:                                             ; preds = %bb11, %bb12
  call void @llvm.lifetime.end.p0(ptr %memory)
  br label %bb14

bb14:                                             ; preds = %bb15, %bb13
  ret void

bb2:                                              ; No predecessors!
  unreachable

bb17:                                             ; No predecessors!
  unreachable
}

; <alloc::raw_vec::RawVecInner>::grow_amortized
; Function Attrs: uwtable
define internal { i64, i64 } @_RNvMs4_NtCskPpRZcNh5S4_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCslBhqQiLn9nt_14stack_overflow(ptr noalias noundef align 8 dereferenceable(16) %self, i64 noundef %len, i64 noundef %additional, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0, i64 noundef %elem_layout.1) unnamed_addr #0 {
start:
  %self3 = alloca [24 x i8], align 8
  %_21 = alloca [24 x i8], align 8
  %v1 = alloca [8 x i8], align 8
  %self2 = alloca [16 x i8], align 8
  %self1 = alloca [16 x i8], align 8
  %_9 = alloca [16 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %_5 = icmp ugt i64 %additional, 0
  %0 = call i1 @llvm.expect.i1(i1 %_5, i1 true)
  br i1 %0, label %bb1, label %bb2

bb2:                                              ; preds = %start
; call core::panicking::panic
  call void @_RNvNtCs7STGCdL2atM_4core9panicking5panic(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_f618e816ca814de4958218055ad99397, i64 noundef 32, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2e63d22236afbffcb89e2d0823f8afb0) #25
  unreachable

bb1:                                              ; preds = %start
  %1 = icmp eq i64 %elem_layout.1, 0
  %2 = call i1 @llvm.expect.i1(i1 %1, i1 true)
  br i1 %2, label %bb3, label %bb4

bb3:                                              ; preds = %bb1
  %3 = load i64, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, align 8, !range !10, !noundef !3
  %4 = load i64, ptr getelementptr inbounds (i8, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, i64 8), align 8
  store i64 %3, ptr %_0, align 8
  %5 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %4, ptr %5, align 8
  br label %bb8

bb4:                                              ; preds = %bb1
  call void @llvm.lifetime.start.p0(ptr %_9)
  call void @llvm.lifetime.start.p0(ptr %self1)
  call void @llvm.lifetime.start.p0(ptr %self2)
  %_27.0 = add i64 %len, %additional
  %_27.1 = icmp ult i64 %_27.0, %len
  br i1 %_27.1, label %bb9, label %bb11

bb8:                                              ; preds = %bb7, %bb21, %bb3
  %6 = load i64, ptr %_0, align 8, !range !10, !noundef !3
  %7 = getelementptr inbounds i8, ptr %_0, i64 8
  %8 = load i64, ptr %7, align 8
  %9 = insertvalue { i64, i64 } poison, i64 %6, 0
  %10 = insertvalue { i64, i64 } %9, i64 %8, 1
  ret { i64, i64 } %10

bb11:                                             ; preds = %bb4
  %_28 = add nuw i64 %len, %additional
  %11 = getelementptr inbounds i8, ptr %self2, i64 8
  store i64 %_28, ptr %11, align 8
  store i64 1, ptr %self2, align 8
  %12 = getelementptr inbounds i8, ptr %self2, i64 8
  %v = load i64, ptr %12, align 8, !noundef !3
  %13 = getelementptr inbounds i8, ptr %self1, i64 8
  store i64 %v, ptr %13, align 8
  store i64 -9223372036854775807, ptr %self1, align 8
  call void @llvm.lifetime.end.p0(ptr %self2)
  %14 = getelementptr inbounds i8, ptr %self1, i64 8
  %v4 = load i64, ptr %14, align 8, !noundef !3
  %15 = getelementptr inbounds i8, ptr %_9, i64 8
  store i64 %v4, ptr %15, align 8
  store i64 -9223372036854775807, ptr %_9, align 8
  call void @llvm.lifetime.end.p0(ptr %self1)
  %16 = getelementptr inbounds i8, ptr %_9, i64 8
  %required_cap = load i64, ptr %16, align 8, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %_9)
  %self5 = load i64, ptr %self, align 8, !range !9, !noundef !3
  %17 = icmp ule i64 %self5, 9223372036854775807
  call void @llvm.assume(i1 %17)
  %v16 = mul i64 %self5, 2
; call <usize as core::cmp::Ord>::max
  %cap = call noundef i64 @_RNvYjNtNtCs7STGCdL2atM_4core3cmp3Ord3maxCslBhqQiLn9nt_14stack_overflow(i64 noundef %v16, i64 noundef %required_cap) #23
  call void @llvm.lifetime.start.p0(ptr %v1)
  %18 = icmp eq i64 %elem_layout.1, 1
  br i1 %18, label %bb14, label %bb15

bb9:                                              ; preds = %bb4
  %19 = load i64, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, align 8, !range !11, !noundef !3
  %20 = load i64, ptr getelementptr inbounds (i8, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, i64 8), align 8
  store i64 %19, ptr %self2, align 8
  %21 = getelementptr inbounds i8, ptr %self2, i64 8
  store i64 %20, ptr %21, align 8
  %22 = load i64, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, align 8, !range !7, !noundef !3
  %23 = load i64, ptr getelementptr inbounds (i8, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, i64 8), align 8
  store i64 %22, ptr %self1, align 8
  %24 = getelementptr inbounds i8, ptr %self1, i64 8
  store i64 %23, ptr %24, align 8
  call void @llvm.lifetime.end.p0(ptr %self2)
  %e.08 = load i64, ptr %self1, align 8, !range !7, !noundef !3
  %25 = getelementptr inbounds i8, ptr %self1, i64 8
  %e.19 = load i64, ptr %25, align 8
  store i64 %e.08, ptr %_9, align 8
  %26 = getelementptr inbounds i8, ptr %_9, i64 8
  store i64 %e.19, ptr %26, align 8
  call void @llvm.lifetime.end.p0(ptr %self1)
  %residual.010 = load i64, ptr %_9, align 8, !range !7, !noundef !3
  %27 = getelementptr inbounds i8, ptr %_9, i64 8
  %residual.111 = load i64, ptr %27, align 8
  call void @llvm.assume(i1 true)
  store i64 %residual.010, ptr %_0, align 8
  %28 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %residual.111, ptr %28, align 8
  call void @llvm.lifetime.end.p0(ptr %_9)
  br label %bb7

bb14:                                             ; preds = %bb11
  store i64 8, ptr %v1, align 8
  br label %bb13

bb15:                                             ; preds = %bb11
  %_38 = icmp ule i64 %elem_layout.1, 1024
  br i1 %_38, label %bb16, label %bb17

bb13:                                             ; preds = %bb18, %bb14
  %29 = load i64, ptr %v1, align 8, !noundef !3
; call <usize as core::cmp::Ord>::max
  %cap7 = call noundef i64 @_RNvYjNtNtCs7STGCdL2atM_4core3cmp3Ord3maxCslBhqQiLn9nt_14stack_overflow(i64 noundef %29, i64 noundef %cap) #23
  call void @llvm.lifetime.end.p0(ptr %v1)
  call void @llvm.lifetime.start.p0(ptr %_21)
  call void @llvm.lifetime.start.p0(ptr %self3)
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call void @_RNvMs4_NtCskPpRZcNh5S4_5alloc7raw_vecNtB5_11RawVecInner11finish_growCslBhqQiLn9nt_14stack_overflow(ptr noalias noundef sret([24 x i8]) align 8 captures(none) dereferenceable(24) %self3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, i64 noundef %cap7, i64 noundef %elem_layout.0, i64 noundef %elem_layout.1)
  %_39 = load i64, ptr %self3, align 8, !range !11, !noundef !3
  %30 = trunc nuw i64 %_39 to i1
  br i1 %30, label %bb20, label %bb21

bb17:                                             ; preds = %bb15
  store i64 1, ptr %v1, align 8
  br label %bb18

bb16:                                             ; preds = %bb15
  store i64 4, ptr %v1, align 8
  br label %bb18

bb18:                                             ; preds = %bb16, %bb17
  br label %bb13

bb20:                                             ; preds = %bb13
  %31 = getelementptr inbounds i8, ptr %self3, i64 8
  %e.0 = load i64, ptr %31, align 8, !range !7, !noundef !3
  %32 = getelementptr inbounds i8, ptr %31, i64 8
  %e.1 = load i64, ptr %32, align 8
  %33 = getelementptr inbounds i8, ptr %_21, i64 8
  store i64 %e.0, ptr %33, align 8
  %34 = getelementptr inbounds i8, ptr %33, i64 8
  store i64 %e.1, ptr %34, align 8
  store i64 1, ptr %_21, align 8
  call void @llvm.lifetime.end.p0(ptr %self3)
  %35 = getelementptr inbounds i8, ptr %_21, i64 8
  %residual.0 = load i64, ptr %35, align 8, !range !7, !noundef !3
  %36 = getelementptr inbounds i8, ptr %35, i64 8
  %residual.1 = load i64, ptr %36, align 8
  call void @llvm.assume(i1 true)
  store i64 %residual.0, ptr %_0, align 8
  %37 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %residual.1, ptr %37, align 8
  call void @llvm.lifetime.end.p0(ptr %_21)
  br label %bb7

bb21:                                             ; preds = %bb13
  %38 = getelementptr inbounds i8, ptr %self3, i64 8
  %v.0 = load ptr, ptr %38, align 8, !nonnull !3, !noundef !3
  %39 = getelementptr inbounds i8, ptr %38, i64 8
  %v.1 = load i64, ptr %39, align 8, !noundef !3
  %40 = getelementptr inbounds i8, ptr %_21, i64 8
  store ptr %v.0, ptr %40, align 8
  %41 = getelementptr inbounds i8, ptr %40, i64 8
  store i64 %v.1, ptr %41, align 8
  store i64 0, ptr %_21, align 8
  call void @llvm.lifetime.end.p0(ptr %self3)
  %42 = getelementptr inbounds i8, ptr %_21, i64 8
  %ptr.0 = load ptr, ptr %42, align 8, !nonnull !3, !noundef !3
  %43 = getelementptr inbounds i8, ptr %42, i64 8
  %ptr.1 = load i64, ptr %43, align 8, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %_21)
  %44 = icmp ne ptr %ptr.0, null
  call void @llvm.assume(i1 %44)
  %45 = icmp ne ptr %ptr.0, null
  call void @llvm.assume(i1 %45)
  %46 = getelementptr inbounds i8, ptr %self, i64 8
  store ptr %ptr.0, ptr %46, align 8
  %47 = icmp ule i64 %cap7, 9223372036854775807
  call void @llvm.assume(i1 %47)
  store i64 %cap7, ptr %self, align 8
  %48 = load i64, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.1, align 8, !range !10, !noundef !3
  %49 = load i64, ptr getelementptr inbounds (i8, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.1, i64 8), align 8
  store i64 %48, ptr %_0, align 8
  %50 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %49, ptr %50, align 8
  br label %bb8

bb7:                                              ; preds = %bb9, %bb20
  br label %bb8

bb5:                                              ; No predecessors!
  unreachable
}

; <alloc::vec::Vec<stack_overflow::Big>>::push_mut
; Function Attrs: inlinehint uwtable
define internal noundef nonnull align 1 ptr @_RNvMsF_NtCskPpRZcNh5S4_5alloc3vecINtB5_3VecNtCslBhqQiLn9nt_14stack_overflow3BigE8push_mutBH_(ptr noalias noundef align 8 dereferenceable(24) %self, ptr dead_on_return noalias noundef readonly align 1 captures(none) dereferenceable(2000000) %value) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %src = alloca [2000000 x i8], align 1
  %_5 = alloca [8 x i8], align 8
  %1 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %1, align 8, !noundef !3
  call void @llvm.lifetime.start.p0(ptr %_5)
  br label %bb9

bb9:                                              ; preds = %start
  %self1 = load i64, ptr %self, align 8, !range !9, !noundef !3
  store i64 %self1, ptr %_5, align 8
  br label %bb7

bb7:                                              ; preds = %bb9
  %2 = load i64, ptr %_5, align 8, !noundef !3
  %_4 = icmp eq i64 %len, %2
  br i1 %_4, label %bb1, label %bb3

bb3:                                              ; preds = %bb7
  call void @llvm.lifetime.end.p0(ptr %_5)
  br label %bb4

bb1:                                              ; preds = %bb7
  call void @llvm.lifetime.end.p0(ptr %_5)
; invoke <alloc::raw_vec::RawVec<stack_overflow::Big>>::grow_one
  invoke void @_RNvMs3_NtCskPpRZcNh5S4_5alloc7raw_vecINtB5_6RawVecNtCslBhqQiLn9nt_14stack_overflow3BigE8grow_oneBO_(ptr noalias noundef align 8 dereferenceable(16) %self)
          to label %bb2 unwind label %cleanup

bb4:                                              ; preds = %bb2, %bb3
  %3 = getelementptr inbounds i8, ptr %self, i64 8
  %_14 = load ptr, ptr %3, align 8, !nonnull !3, !noundef !3
  %4 = icmp ne ptr %_14, null
  call void @llvm.assume(i1 %4)
  br label %bb12

bb6:                                              ; preds = %cleanup
  %5 = load ptr, ptr %0, align 8, !noundef !3
  %6 = getelementptr inbounds i8, ptr %0, i64 8
  %7 = load i32, ptr %6, align 8, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %0)
  %8 = insertvalue { ptr, i32 } poison, ptr %5, 0
  %9 = insertvalue { ptr, i32 } %8, i32 %7, 1
  resume { ptr, i32 } %9

cleanup:                                          ; preds = %bb1
  %10 = landingpad { ptr, i32 }
          cleanup
  %11 = extractvalue { ptr, i32 } %10, 0
  %12 = extractvalue { ptr, i32 } %10, 1
  call void @llvm.lifetime.start.p0(ptr %0)
  store ptr %11, ptr %0, align 8
  %13 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %12, ptr %13, align 8
  br label %bb6

bb2:                                              ; preds = %bb1
  br label %bb4

bb12:                                             ; preds = %bb4
  %end = getelementptr inbounds nuw %Big, ptr %_14, i64 %len
  call void @llvm.lifetime.start.p0(ptr %src)
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %src, ptr align 1 %value, i64 2000000, i1 false)
  br label %bb15

bb15:                                             ; preds = %bb12
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %end, ptr align 1 %src, i64 2000000, i1 false)
  call void @llvm.lifetime.end.p0(ptr %src)
  %14 = getelementptr inbounds i8, ptr %self, i64 16
  %15 = add i64 %len, 1
  store i64 %15, ptr %14, align 8
  ret ptr %end

bb8:                                              ; No predecessors!
  unreachable

bb10:                                             ; No predecessors!
  unreachable

bb11:                                             ; No predecessors!
  unreachable

bb13:                                             ; No predecessors!
  unreachable

bb14:                                             ; No predecessors!
  unreachable
}

; core::intrinsics::cold_path
; Function Attrs: cold nounwind uwtable
define internal void @_RNvNtCs7STGCdL2atM_4core10intrinsics9cold_path() unnamed_addr #4 {
start:
  ret void
}

; alloc::raw_vec::layout_array
; Function Attrs: inlinehint uwtable
define internal void @_RNvNtCskPpRZcNh5S4_5alloc7raw_vec12layout_array(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, i64 noundef %cap, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0, i64 noundef %elem_layout.1) unnamed_addr #2 {
start:
  %_9 = alloca [16 x i8], align 8
  %0 = sub i64 %elem_layout.0, 1
  %1 = icmp ule i64 %0, 9223372036854775807
  call void @llvm.assume(i1 %1)
  %_14 = sub nuw i64 %elem_layout.0, 1
  %_16 = add nuw i64 %elem_layout.1, %_14
  %_17 = xor i64 %_14, -1
  %new_size = and i64 %_16, %_17
  br label %bb5

bb5:                                              ; preds = %start
  %_3 = icmp eq i64 %elem_layout.1, %new_size
  %2 = call i1 @llvm.expect.i1(i1 %_3, i1 true)
  br i1 %2, label %bb1, label %bb2

bb2:                                              ; preds = %bb5
; call core::panicking::panic
  call void @_RNvNtCs7STGCdL2atM_4core9panicking5panic(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_2b9352ac8aee68e075fb1d7daa438bea, i64 noundef 73, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2fe82924fcf40f35157626e1ac5b5ac7) #25
  unreachable

bb1:                                              ; preds = %bb5
  call void @llvm.lifetime.start.p0(ptr %_9)
  %3 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %elem_layout.1, i64 %cap)
  %_21.0 = extractvalue { i64, i1 } %3, 0
  %_21.1 = extractvalue { i64, i1 } %3, 1
  %4 = call i1 @llvm.expect.i1(i1 %_21.1, i1 false)
  br i1 %4, label %bb6, label %bb8

bb8:                                              ; preds = %bb1
  %_28 = sub nuw i64 -9223372036854775808, %elem_layout.0
  %_26 = icmp ule i64 %_21.0, %_28
  br i1 %_26, label %bb9, label %bb10

bb6:                                              ; preds = %bb1
  br label %bb11

bb10:                                             ; preds = %bb8
  br label %bb11

bb9:                                              ; preds = %bb8
  store i64 %elem_layout.0, ptr %_9, align 8
  %5 = getelementptr inbounds i8, ptr %_9, i64 8
  store i64 %_21.0, ptr %5, align 8
  %layout.0 = load i64, ptr %_9, align 8, !range !8, !noundef !3
  %6 = getelementptr inbounds i8, ptr %_9, i64 8
  %layout.1 = load i64, ptr %6, align 8, !noundef !3
  %7 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %layout.0, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %7, i64 8
  store i64 %layout.1, ptr %8, align 8
  store i64 0, ptr %_0, align 8
  br label %bb3

bb11:                                             ; preds = %bb6, %bb10
  %9 = load i64, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, align 8, !range !7, !noundef !3
  %10 = load i64, ptr getelementptr inbounds (i8, ptr @anon.13aff46757640e471bf36f0f14e1b5b3.0, i64 8), align 8
  %11 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %9, ptr %11, align 8
  %12 = getelementptr inbounds i8, ptr %11, i64 8
  store i64 %10, ptr %12, align 8
  store i64 1, ptr %_0, align 8
  br label %bb3

bb3:                                              ; preds = %bb11, %bb9
  call void @llvm.lifetime.end.p0(ptr %_9)
  ret void

bb4:                                              ; No predecessors!
  unreachable
}

; <alloc::raw_vec::RawVec<stack_overflow::Big> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @_RNvXs1_NtCskPpRZcNh5S4_5alloc7raw_vecINtB5_6RawVecNtCslBhqQiLn9nt_14stack_overflow3BigENtNtNtCs7STGCdL2atM_4core3ops4drop4Drop4dropBO_(ptr noalias noundef align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
; call <alloc::raw_vec::RawVecInner>::deallocate
  call void @_RNvMs2_NtCskPpRZcNh5S4_5alloc7raw_vecNtB5_11RawVecInner10deallocateCslBhqQiLn9nt_14stack_overflow(ptr noalias noundef align 8 dereferenceable(16) %self, i64 noundef 1, i64 noundef 2000000)
  ret void
}

; <usize as core::cmp::PartialOrd>::lt
; Function Attrs: alwaysinline uwtable
define internal noundef zeroext i1 @_RNvXsU_NtNtCs7STGCdL2atM_4core3cmp5implsjNtB7_10PartialOrd2lt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %other) unnamed_addr #5 {
start:
  %_3 = load i64, ptr %self, align 8, !noundef !3
  %_4 = load i64, ptr %other, align 8, !noundef !3
  %_0 = icmp ult i64 %_3, %_4
  ret i1 %_0
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal noundef i8 @_RNvXsZ_NtCsjR135vb6FBx_3std7processuNtB5_11Termination6report() unnamed_addr #2 {
start:
  ret i8 0
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint uwtable
define internal void @_RNvXs_NtCskPpRZcNh5S4_5alloc5allocNtB4_6GlobalNtNtCs7STGCdL2atM_4core5alloc9Allocator10deallocate(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self, ptr noundef nonnull %ptr, i64 noundef range(i64 1, -9223372036854775807) %layout.0, i64 noundef %layout.1) unnamed_addr #2 {
start:
  %0 = icmp eq i64 %layout.1, 0
  br i1 %0, label %bb2, label %bb1

bb2:                                              ; preds = %bb1, %start
  ret void

bb1:                                              ; preds = %start
  %1 = icmp ne ptr %ptr, null
  call void @llvm.assume(i1 %1)
; call __rustc::__rust_dealloc
  call void @_RNvCs2fcwfXhWpkc_7___rustc14___rust_dealloc(ptr noundef %ptr, i64 noundef %layout.1, i64 noundef %layout.0) #26
  br label %bb2
}

; <alloc::alloc::Global as core::alloc::Allocator>::grow
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @_RNvXs_NtCskPpRZcNh5S4_5alloc5allocNtB4_6GlobalNtNtCs7STGCdL2atM_4core5alloc9Allocator4grow(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self, ptr noundef nonnull %ptr, i64 noundef range(i64 1, -9223372036854775807) %old_layout.0, i64 noundef %old_layout.1, i64 noundef range(i64 1, -9223372036854775807) %new_layout.0, i64 noundef %new_layout.1) unnamed_addr #2 {
start:
; call <alloc::alloc::Global>::grow_impl_runtime
  %0 = call { ptr, i64 } @_RNvMNtCskPpRZcNh5S4_5alloc5allocNtB2_6Global17grow_impl_runtime(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self, ptr noundef nonnull %ptr, i64 noundef %old_layout.0, i64 noundef %old_layout.1, i64 noundef %new_layout.0, i64 noundef %new_layout.1, i1 noundef zeroext false) #23
  %_0.0 = extractvalue { ptr, i64 } %0, 0
  %_0.1 = extractvalue { ptr, i64 } %0, 1
  %1 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %_0.1, 1
  ret { ptr, i64 } %2
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @_RNvXs_NtCskPpRZcNh5S4_5alloc5allocNtB4_6GlobalNtNtCs7STGCdL2atM_4core5alloc9Allocator8allocate(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self, i64 noundef range(i64 1, -9223372036854775807) %layout.0, i64 noundef %layout.1) unnamed_addr #2 {
start:
; call <alloc::alloc::Global>::alloc_impl_runtime
  %0 = call { ptr, i64 } @_RNvMNtCskPpRZcNh5S4_5alloc5allocNtB2_6Global18alloc_impl_runtime(i64 noundef %layout.0, i64 noundef %layout.1, i1 noundef zeroext false) #23
  %_0.0 = extractvalue { ptr, i64 } %0, 0
  %_0.1 = extractvalue { ptr, i64 } %0, 1
  %1 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %_0.1, 1
  ret { ptr, i64 } %2
}

; <alloc::vec::Vec<stack_overflow::Big> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @_RNvXso_NtCskPpRZcNh5S4_5alloc3vecINtB5_3VecNtCslBhqQiLn9nt_14stack_overflow3BigENtNtNtCs7STGCdL2atM_4core3ops4drop4Drop4dropBH_(ptr noalias noundef align 8 dereferenceable(24) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %_5 = load ptr, ptr %0, align 8, !nonnull !3, !noundef !3
  %1 = icmp ne ptr %_5, null
  call void @llvm.assume(i1 %1)
  %2 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %2, align 8, !noundef !3
  ret void
}

; <fn() as core::ops::function::FnOnce<()>>::call_once
; Function Attrs: inlinehint uwtable
define internal void @_RNvYFEuINtNtNtCs7STGCdL2atM_4core3ops8function6FnOnceuE9call_onceCslBhqQiLn9nt_14stack_overflow(ptr noundef nonnull %_1) unnamed_addr #2 {
start:
  %_2 = alloca [0 x i8], align 1
  call void %_1()
  ret void
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNvYNCINvNtCsjR135vb6FBx_3std2rt10lang_startuE0INtNtNtCs7STGCdL2atM_4core3ops8function6FnOnceuE9call_onceCslBhqQiLn9nt_14stack_overflow(ptr noundef nonnull %0) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %1 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [8 x i8], align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::<()>::{closure#0}
  %_0 = invoke noundef i32 @_RNCINvNtCsjR135vb6FBx_3std2rt10lang_startuE0CslBhqQiLn9nt_14stack_overflow(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %2 = load ptr, ptr %1, align 8, !noundef !3
  %3 = getelementptr inbounds i8, ptr %1, i64 8
  %4 = load i32, ptr %3, align 8, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %1)
  %5 = insertvalue { ptr, i32 } poison, ptr %2, 0
  %6 = insertvalue { ptr, i32 } %5, i32 %4, 1
  resume { ptr, i32 } %6

cleanup:                                          ; preds = %start
  %7 = landingpad { ptr, i32 }
          cleanup
  %8 = extractvalue { ptr, i32 } %7, 0
  %9 = extractvalue { ptr, i32 } %7, 1
  call void @llvm.lifetime.start.p0(ptr %1)
  store ptr %8, ptr %1, align 8
  %10 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %9, ptr %10, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %_0
}

; <usize as core::cmp::Ord>::max
; Function Attrs: inlinehint uwtable
define internal noundef i64 @_RNvYjNtNtCs7STGCdL2atM_4core3cmp3Ord3maxCslBhqQiLn9nt_14stack_overflow(i64 noundef %0, i64 noundef %1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %2 = alloca [16 x i8], align 8
  %_6 = alloca [1 x i8], align 1
  %_0 = alloca [8 x i8], align 8
  %other = alloca [8 x i8], align 8
  %self = alloca [8 x i8], align 8
  store i64 %0, ptr %self, align 8
  store i64 %1, ptr %other, align 8
  store i8 1, ptr %_6, align 1
; invoke <usize as core::cmp::PartialOrd>::lt
  %_3 = invoke noundef zeroext i1 @_RNvXsU_NtNtCs7STGCdL2atM_4core3cmp5implsjNtB7_10PartialOrd2lt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %other, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self)
          to label %bb1 unwind label %cleanup

bb5:                                              ; preds = %cleanup
  br label %bb8

cleanup:                                          ; preds = %start
  %3 = landingpad { ptr, i32 }
          cleanup
  %4 = extractvalue { ptr, i32 } %3, 0
  %5 = extractvalue { ptr, i32 } %3, 1
  call void @llvm.lifetime.start.p0(ptr %2)
  store ptr %4, ptr %2, align 8
  %6 = getelementptr inbounds i8, ptr %2, i64 8
  store i32 %5, ptr %6, align 8
  br label %bb5

bb1:                                              ; preds = %start
  br i1 %_3, label %bb2, label %bb3

bb3:                                              ; preds = %bb1
  %7 = load i64, ptr %other, align 8, !noundef !3
  store i64 %7, ptr %_0, align 8
  br label %bb4

bb2:                                              ; preds = %bb1
  store i8 0, ptr %_6, align 1
  %8 = load i64, ptr %self, align 8, !noundef !3
  store i64 %8, ptr %_0, align 8
  br label %bb4

bb4:                                              ; preds = %bb2, %bb3
  %9 = load i64, ptr %_0, align 8, !noundef !3
  ret i64 %9

bb8:                                              ; preds = %bb5
  %10 = load i8, ptr %_6, align 1, !range !6, !noundef !3
  %11 = trunc nuw i8 %10 to i1
  br i1 %11, label %bb7, label %bb6

bb6:                                              ; preds = %bb7, %bb8
  %12 = load ptr, ptr %2, align 8, !noundef !3
  %13 = getelementptr inbounds i8, ptr %2, i64 8
  %14 = load i32, ptr %13, align 8, !noundef !3
  call void @llvm.lifetime.end.p0(ptr %2)
  %15 = insertvalue { ptr, i32 } poison, ptr %12, 0
  %16 = insertvalue { ptr, i32 } %15, i32 %14, 1
  resume { ptr, i32 } %16

bb7:                                              ; preds = %bb8
  br label %bb6
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #6

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(ptr captures(none)) #7

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCs7STGCdL2atM_4core9panicking16panic_in_cleanup() unnamed_addr #8

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(ptr captures(none)) #7

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare noundef i64 @_RNvNtCsjR135vb6FBx_3std2rt19lang_start_internal(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #9

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #10

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #11

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCs7STGCdL2atM_4core9panicking5panic(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #12

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #13

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCs7STGCdL2atM_4core9panicking9panic_fmt(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #12

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCs2fcwfXhWpkc_7___rustc14___rust_dealloc(ptr allocptr noundef captures(address), i64 noundef, i64 noundef range(i64 1, -9223372036854775807)) unnamed_addr #14

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCs2fcwfXhWpkc_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #15

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCs2fcwfXhWpkc_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #6

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCs2fcwfXhWpkc_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef range(i64 1, -9223372036854775807)) unnamed_addr #16

; __rustc::__rust_alloc_zeroed
; Function Attrs: nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCs2fcwfXhWpkc_7___rustc19___rust_alloc_zeroed(i64 noundef, i64 allocalign noundef range(i64 1, -9223372036854775807)) unnamed_addr #17

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCskPpRZcNh5S4_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #18

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #19

define i32 @main(i32 %0, ptr %1) unnamed_addr #20 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start::<()>
  %3 = call i64 @_RINvNtCsjR135vb6FBx_3std2rt10lang_startuECslBhqQiLn9nt_14stack_overflow(ptr @_RNvCslBhqQiLn9nt_14stack_overflow4main, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #1 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #2 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #3 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #4 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #5 = { alwaysinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #6 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #7 = { nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #8 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #9 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #10 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #11 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #12 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #13 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #14 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #15 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #16 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCs2fcwfXhWpkc_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #17 = { nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #18 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #19 = { nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none) }
attributes #20 = { "frame-pointer"="non-leaf" "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" }
attributes #21 = { cold }
attributes #22 = { cold noreturn nounwind }
attributes #23 = { inlinehint }
attributes #24 = { noinline }
attributes #25 = { noinline noreturn }
attributes #26 = { nounwind }
attributes #27 = { noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.95.0-dev"}
!3 = !{}
!4 = !{i64 1208230135220926}
!5 = !{i64 8}
!6 = !{i8 0, i8 2}
!7 = !{i64 0, i64 -9223372036854775807}
!8 = !{i64 1, i64 -9223372036854775807}
!9 = !{i64 0, i64 -9223372036854775808}
!10 = !{i64 0, i64 -9223372036854775806}
!11 = !{i64 0, i64 2}
