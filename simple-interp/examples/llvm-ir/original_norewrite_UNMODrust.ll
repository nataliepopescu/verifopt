; ModuleID = 'original_norewrite.5d41c54750b794d-cgu.0'
source_filename = "original_norewrite.5d41c54750b794d-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"std::ffi::os_str::OsString" = type { %"std::sys::os_str::bytes::Buf" }
%"std::sys::os_str::bytes::Buf" = type { %"alloc::vec::Vec<u8>" }
%"alloc::vec::Vec<u8>" = type { %"alloc::raw_vec::RawVec<u8>", i64 }
%"alloc::raw_vec::RawVec<u8>" = type { %"alloc::raw_vec::RawVecInner", %"core::marker::PhantomData<u8>" }
%"alloc::raw_vec::RawVecInner" = type { i64, ptr, %"alloc::alloc::Global" }
%"alloc::alloc::Global" = type {}
%"core::marker::PhantomData<u8>" = type {}
%"alloc::string::String" = type { %"alloc::vec::Vec<u8>" }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN70_$LT$original_norewrite..Cat$u20$as$u20$original_norewrite..Animal$GT$5kaeps17h04e3b6378d9a5731E" }>, align 8
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN70_$LT$original_norewrite..Dog$u20$as$u20$original_norewrite..Animal$GT$5kaeps17hcc0497b26c88a419E" }>, align 8
@alloc_c85489ac7d65f986716cfdb1dc3aed62 = private unnamed_addr constant [39 x i8] c"Pass in a number and see what happens!\0A", align 1
@alloc_da03007fb5fe1aab10d9ffd81ef29605 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_c85489ac7d65f986716cfdb1dc3aed62, [8 x i8] c"'\00\00\00\00\00\00\00" }>, align 8
@alloc_689a4e84816bbca00519182524d3f9c1 = private unnamed_addr constant [22 x i8] c"original_norewrite.rs\00", align 1
@alloc_be681e3131e727a983aaa4cb9118ba6c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_689a4e84816bbca00519182524d3f9c1, [16 x i8] c"\15\00\00\00\00\00\00\007\00\00\00\16\00\00\00" }>, align 8
@alloc_068ab455f5bf47d6087b5a8187125903 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_689a4e84816bbca00519182524d3f9c1, [16 x i8] c"\15\00\00\00\00\00\00\007\00\00\00\22\00\00\00" }>, align 8
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17haf9d08ae32638f21E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64790d7c15d240fbE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64790d7c15d240fbE" }>, align 8
@vtable.3 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN68_$LT$core..num..error..ParseIntError$u20$as$u20$core..fmt..Debug$GT$3fmt17h0f6568d5dfa9eaefE" }>, align 8
@alloc_00ae4b301f7fab8ac9617c03fcbd7274 = private unnamed_addr constant [43 x i8] c"called `Result::unwrap()` on an `Err` value", align 1
@alloc_7e80d81941cf5c819e3db4cff23967f9 = private unnamed_addr constant [72 x i8] c"`new_layout.size()` must be greater than or equal to `old_layout.size()`", align 1
@alloc_04056f6d76887c0653320aca2f1cbe49 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_7e80d81941cf5c819e3db4cff23967f9, [8 x i8] c"H\00\00\00\00\00\00\00" }>, align 8
@alloc_2d21cb22856eece06d16ef6ccb78ad88 = private unnamed_addr constant [52 x i8] c"/home/np/hack/rust_unmod/library/alloc/src/alloc.rs\00", align 1
@alloc_7f38a57380c4169a099c5ec78ee6b9ee = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_2d21cb22856eece06d16ef6ccb78ad88, [16 x i8] c"3\00\00\00\00\00\00\00\CF\00\00\00\09\00\00\00" }>, align 8
@alloc_09b45082787fd361308df8a45104af2e = private unnamed_addr constant [58 x i8] c"/home/np/hack/rust_unmod/library/alloc/src/raw_vec/mod.rs\00", align 1
@alloc_cf7a2c70ecf5a70905a3b8fb705afc60 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_09b45082787fd361308df8a45104af2e, [16 x i8] c"9\00\00\00\00\00\00\00J\03\00\00\09\00\00\00" }>, align 8
@alloc_59ba7b9f7211443cd55a366616eef46a = private unnamed_addr constant [5 x i8] c"Empty", align 1
@alloc_00315c78e51d29fe6b3102a4c1ecf6ef = private unnamed_addr constant [12 x i8] c"InvalidDigit", align 1
@alloc_bd3a3f3879e0d5f64554753e977f58d4 = private unnamed_addr constant [11 x i8] c"PosOverflow", align 1
@alloc_0964bb2a4870637395c77a018495bd5c = private unnamed_addr constant [11 x i8] c"NegOverflow", align 1
@alloc_6566120a3a17f930e960a0863fcbd591 = private unnamed_addr constant [4 x i8] c"Zero", align 1
@vtable.6 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he91003c1d8bf684aE" }>, align 8
@alloc_f62df14955f7d78bca139b0a7668683d = private unnamed_addr constant [13 x i8] c"ParseIntError", align 1
@alloc_a5d866b1768ad3f826bccdb004a1a8ae = private unnamed_addr constant [4 x i8] c"kind", align 1
@alloc_3f95fa5fe64159c0ca66f05c35f35619 = private unnamed_addr constant [5 x i8] c"meow\0A", align 1
@alloc_000bc512779c9a763a8aa84ee52b6421 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_3f95fa5fe64159c0ca66f05c35f35619, [8 x i8] c"\05\00\00\00\00\00\00\00" }>, align 8
@alloc_2b182a67d4f9402d603ef3e7f72e2431 = private unnamed_addr constant [5 x i8] c"woof\0A", align 1
@alloc_ec4fa215300b117d5ab20e2368000be2 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_2b182a67d4f9402d603ef3e7f72e2431, [8 x i8] c"\05\00\00\00\00\00\00\00" }>, align 8
@"switch.table._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he91003c1d8bf684aE" = private unnamed_addr constant [5 x i64] [i64 5, i64 12, i64 11, i64 11, i64 4], align 8
@"switch.table._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he91003c1d8bf684aE.30.rel" = private unnamed_addr constant [5 x i32] [i32 trunc (i64 sub (i64 ptrtoint (ptr @alloc_59ba7b9f7211443cd55a366616eef46a to i64), i64 ptrtoint (ptr @"switch.table._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he91003c1d8bf684aE.30.rel" to i64)) to i32), i32 trunc (i64 sub (i64 ptrtoint (ptr @alloc_00315c78e51d29fe6b3102a4c1ecf6ef to i64), i64 ptrtoint (ptr @"switch.table._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he91003c1d8bf684aE.30.rel" to i64)) to i32), i32 trunc (i64 sub (i64 ptrtoint (ptr @alloc_bd3a3f3879e0d5f64554753e977f58d4 to i64), i64 ptrtoint (ptr @"switch.table._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he91003c1d8bf684aE.30.rel" to i64)) to i32), i32 trunc (i64 sub (i64 ptrtoint (ptr @alloc_0964bb2a4870637395c77a018495bd5c to i64), i64 ptrtoint (ptr @"switch.table._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he91003c1d8bf684aE.30.rel" to i64)) to i32), i32 trunc (i64 sub (i64 ptrtoint (ptr @alloc_6566120a3a17f930e960a0863fcbd591 to i64), i64 ptrtoint (ptr @"switch.table._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he91003c1d8bf684aE.30.rel" to i64)) to i32)], align 4

; original_norewrite::main
; Function Attrs: nonlazybind uwtable
define hidden void @_ZN18original_norewrite4main17hb146e61664e51d9bE() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3.i.i.i.i.i.i = alloca [24 x i8], align 8
  %_19.i.i.i.i = alloca [32 x i8], align 8
  %_3.i.i.i.i = alloca [24 x i8], align 8
  %vector.i.i.i.i = alloca [24 x i8], align 8
  %_2.i.i = alloca [32 x i8], align 8
  %e.i = alloca [1 x i8], align 1
  %_5 = alloca [48 x i8], align 8
  %_2 = alloca [32 x i8], align 8
  %args = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %args)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_2)
; call std::env::args
  call void @_ZN3std3env4args17hdc2bb5ce4515f66cE(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(address) dereferenceable(32) %_2)
  call void @llvm.experimental.noalias.scope.decl(metadata !4)
  call void @llvm.experimental.noalias.scope.decl(metadata !7)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_2.i.i), !noalias !10
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_2.i.i, ptr noundef nonnull readonly align 8 dereferenceable(32) %_2, i64 32, i1 false), !alias.scope !13, !noalias !17
  call void @llvm.experimental.noalias.scope.decl(metadata !18)
  call void @llvm.experimental.noalias.scope.decl(metadata !21)
  call void @llvm.experimental.noalias.scope.decl(metadata !23)
  call void @llvm.experimental.noalias.scope.decl(metadata !26)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i.i.i.i), !noalias !28
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3.i.i.i.i), !noalias !28
; invoke <std::env::Args as core::iter::traits::iterator::Iterator>::next
  invoke void @"_ZN73_$LT$std..env..Args$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h98c081630f6700f9E"(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_3.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(32) %_2.i.i)
          to label %bb1.i.i.i.i unwind label %cleanup.i.i.i.i, !noalias !29

cleanup.i.i.i.i:                                  ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
  br label %bb10.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %start
  %1 = load i64, ptr %_3.i.i.i.i, align 8, !range !30, !noalias !28, !noundef !31
  %.not.i.i.i.i = icmp eq i64 %1, -9223372036854775808
  br i1 %.not.i.i.i.i, label %bb12.i.i.i.i, label %bb14.i.i.i.i

bb12.i.i.i.i:                                     ; preds = %bb1.i.i.i.i
  store i64 0, ptr %args, align 8, !alias.scope !32, !noalias !33
  %2 = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr inttoptr (i64 8 to ptr), ptr %2, align 8, !alias.scope !32, !noalias !33
  %3 = getelementptr inbounds nuw i8, ptr %args, i64 16
  store i64 0, ptr %3, align 8, !alias.scope !32, !noalias !33
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i.i.i.i), !noalias !28
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i.i.i), !noalias !28
  call void @llvm.experimental.noalias.scope.decl(metadata !34)
  call void @llvm.experimental.noalias.scope.decl(metadata !37)
  call void @llvm.experimental.noalias.scope.decl(metadata !40)
  call void @llvm.experimental.noalias.scope.decl(metadata !43)
  call void @llvm.experimental.noalias.scope.decl(metadata !46)
  %4 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 8
  %self2.i.i.i.i.i.i.i.i.i = load ptr, ptr %4, align 8, !alias.scope !49, !noalias !29, !nonnull !31, !noundef !31
  %5 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 24
  %self.val3.i.i.i.i.i.i.i.i.i = load ptr, ptr %5, align 8, !alias.scope !49, !noalias !29, !nonnull !31, !noundef !31
  %6 = ptrtoint ptr %self.val3.i.i.i.i.i.i.i.i.i to i64
  %7 = ptrtoint ptr %self2.i.i.i.i.i.i.i.i.i to i64
  %8 = sub nuw i64 %6, %7
  %9 = udiv exact i64 %8, 24
  call void @llvm.experimental.noalias.scope.decl(metadata !50)
  %_710.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %self.val3.i.i.i.i.i.i.i.i.i, %self2.i.i.i.i.i.i.i.i.i
  br i1 %_710.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb12.i.i.i.i, %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i"
  %_3.sroa.0.011.i.i.i.i.i.i.i.i.i.i = phi i64 [ %10, %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i" ], [ 0, %bb12.i.i.i.i ]
  %_6.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self2.i.i.i.i.i.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i.i
  %10 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i.i.i.i.i.i, align 8, !alias.scope !50, !noalias !53
  %11 = icmp eq i64 %_6.val.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %11, label %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i", label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i:             ; preds = %bb5.i.i.i.i.i.i.i.i.i.i
  %12 = getelementptr i8, ptr %_6.i.i.i.i.i.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %12, align 8, !alias.scope !50, !noalias !53, !nonnull !31, !noundef !31
; call __rustc::__rust_dealloc
  call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #20, !noalias !54
  br label %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i"

"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i": ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %10, %9
  br i1 %_7.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i.i:                            ; preds = %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i", %bb12.i.i.i.i
  %13 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 16
  %capacity1.i.i5.i.i.i.i.i.i.i.i.i = load i64, ptr %13, align 8, !alias.scope !49, !noalias !29, !noundef !31
  %14 = icmp eq i64 %capacity1.i.i5.i.i.i.i.i.i.i.i.i, 0
  br i1 %14, label %_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE.exit.thread, label %bb2.i.i.i.i.i6.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i6.i.i.i.i.i.i.i.i.i:                 ; preds = %bb1.i.i.i.i.i.i.i.i.i
  %ptr.i.i7.i.i.i.i.i.i.i.i.i = load ptr, ptr %_2.i.i, align 8, !alias.scope !49, !noalias !29, !nonnull !31, !noundef !31
  %15 = mul nuw i64 %capacity1.i.i5.i.i.i.i.i.i.i.i.i, 24
; call __rustc::__rust_dealloc
  call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i7.i.i.i.i.i.i.i.i.i, i64 noundef %15, i64 noundef range(i64 1, -9223372036854775807) 8) #20, !noalias !53
  br label %_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE.exit.thread

cleanup2.i.i.i.i:                                 ; preds = %bb3.i.i.i.i.i
  %16 = landingpad { ptr, i32 }
          cleanup
  %17 = icmp eq i64 %1, 0
  br i1 %17, label %bb10.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i:                           ; preds = %cleanup2.i.i.i.i
  %18 = icmp ne ptr %element.sroa.5.0.copyload.i.i.i.i, null
  call void @llvm.assume(i1 %18)
; call __rustc::__rust_dealloc
  call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %element.sroa.5.0.copyload.i.i.i.i, i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) 1) #20, !noalias !29
  br label %bb10.i.i.i.i

bb14.i.i.i.i:                                     ; preds = %bb1.i.i.i.i
  %element.sroa.5.0._3.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i, i64 8
  %element.sroa.5.0.copyload.i.i.i.i = load ptr, ptr %element.sroa.5.0._3.sroa_idx.i.i.i.i, align 8, !noalias !28
  %element.sroa.6.0._3.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i, i64 16
  %element.sroa.6.0.copyload.i.i.i.i = load i64, ptr %element.sroa.6.0._3.sroa_idx.i.i.i.i, align 8, !noalias !28
  %19 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 8
  %iterator.val.i.i.i.i = load ptr, ptr %19, align 8, !alias.scope !55, !noalias !29, !nonnull !31, !noundef !31
  %20 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 24
  %iterator.val6.i.i.i.i = load ptr, ptr %20, align 8, !alias.scope !55, !noalias !29, !nonnull !31, !noundef !31
  %21 = ptrtoint ptr %iterator.val6.i.i.i.i to i64
  %22 = ptrtoint ptr %iterator.val.i.i.i.i to i64
  %23 = sub nuw i64 %21, %22
  %24 = udiv exact i64 %23, 24
  %25 = call i64 @llvm.umax.i64(i64 %24, i64 3)
  %_0.sroa.0.0.i.i.i.i.i = add nuw nsw i64 %25, 1
  %_21.0.i.i.i.i.i.i.i = mul i64 %_0.sroa.0.0.i.i.i.i.i, 24
  %or.cond.i.i.i.i.i.i.i = icmp ugt i64 %23, 9223372036854775776
  br i1 %or.cond.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb18.i.i.i.i.i.i, !prof !56

bb18.i.i.i.i.i.i:                                 ; preds = %bb14.i.i.i.i
  %26 = icmp eq i64 %_21.0.i.i.i.i.i.i.i, 0
  br i1 %26, label %bb15.i.i.i.i, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h16e1f8f28639b222E.exit.i.i.i.i.i.i"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h16e1f8f28639b222E.exit.i.i.i.i.i.i": ; preds = %bb18.i.i.i.i.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCscszlnfU7ykQ_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #20, !noalias !57
; call __rustc::__rust_alloc
  %27 = call noundef align 8 ptr @_RNvCscszlnfU7ykQ_7___rustc12___rust_alloc(i64 noundef %_21.0.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #20, !noalias !57
  %28 = icmp eq ptr %27, null
  br i1 %28, label %bb3.i.i.i.i.i, label %bb10.i.i.i.i.i.i

bb10.i.i.i.i.i.i:                                 ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h16e1f8f28639b222E.exit.i.i.i.i.i.i"
  %29 = ptrtoint ptr %27 to i64
  br label %bb15.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h16e1f8f28639b222E.exit.i.i.i.i.i.i", %bb14.i.i.i.i
  %_4.sroa.4.0.ph.i.i.i.i.i = phi i64 [ 8, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h16e1f8f28639b222E.exit.i.i.i.i.i.i" ], [ 0, %bb14.i.i.i.i ]
; invoke alloc::raw_vec::handle_error
  invoke void @_ZN5alloc7raw_vec12handle_error17h4842bb444d407eecE(i64 noundef %_4.sroa.4.0.ph.i.i.i.i.i, i64 %_21.0.i.i.i.i.i.i.i) #21
          to label %.noexc.i.i.i.i unwind label %cleanup2.i.i.i.i, !noalias !29

.noexc.i.i.i.i:                                   ; preds = %bb3.i.i.i.i.i
  unreachable

bb15.i.i.i.i:                                     ; preds = %bb10.i.i.i.i.i.i, %bb18.i.i.i.i.i.i
  %_4.sroa.4.0.i.i.i.i.i = phi i64 [ %_0.sroa.0.0.i.i.i.i.i, %bb10.i.i.i.i.i.i ], [ 0, %bb18.i.i.i.i.i.i ]
  %_4.sroa.10.0.i.i.i.i.i = phi i64 [ %29, %bb10.i.i.i.i.i.i ], [ 8, %bb18.i.i.i.i.i.i ]
  %30 = inttoptr i64 %_4.sroa.10.0.i.i.i.i.i to ptr
  %_8.i.i.i.i.i = icmp samesign ult i64 %25, %_4.sroa.4.0.i.i.i.i.i
  call void @llvm.assume(i1 %_8.i.i.i.i.i)
  store i64 %1, ptr %30, align 8, !noalias !29
  %src.sroa.4.0._28.1.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %30, i64 8
  store ptr %element.sroa.5.0.copyload.i.i.i.i, ptr %src.sroa.4.0._28.1.sroa_idx.i.i.i.i, align 8, !noalias !29
  %src.sroa.5.0._28.1.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %30, i64 16
  store i64 %element.sroa.6.0.copyload.i.i.i.i, ptr %src.sroa.5.0._28.1.sroa_idx.i.i.i.i, align 8, !noalias !29
  store i64 %_4.sroa.4.0.i.i.i.i.i, ptr %vector.i.i.i.i, align 8, !noalias !28
  %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i.i.i, i64 8
  store ptr %30, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !28
  %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i.i.i, i64 16
  store i64 1, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i, align 8, !noalias !28
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i.i.i.i), !noalias !28
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_19.i.i.i.i), !noalias !28
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_19.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) %_2.i.i, i64 32, i1 false), !noalias !29
  call void @llvm.experimental.noalias.scope.decl(metadata !60)
  call void @llvm.experimental.noalias.scope.decl(metadata !63)
  call void @llvm.experimental.noalias.scope.decl(metadata !65)
  call void @llvm.experimental.noalias.scope.decl(metadata !68)
  %element.sroa.5.0._3.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i, i64 8
  %element.sroa.6.0._3.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i, i64 16
  %31 = getelementptr inbounds nuw i8, ptr %_19.i.i.i.i, i64 24
  %32 = getelementptr inbounds nuw i8, ptr %_19.i.i.i.i, i64 8
  br label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %bb8.i.i.i.i.i.i, %bb15.i.i.i.i
  %_25.i.i18.i.i.i.i = phi ptr [ %_25.i.i.i.i.i.i, %bb8.i.i.i.i.i.i ], [ %30, %bb15.i.i.i.i ]
  %len.i.i.i.i.i.i = phi i64 [ %46, %bb8.i.i.i.i.i.i ], [ 1, %bb15.i.i.i.i ]
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3.i.i.i.i.i.i), !noalias !70
; invoke <std::env::Args as core::iter::traits::iterator::Iterator>::next
  invoke void @"_ZN73_$LT$std..env..Args$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h98c081630f6700f9E"(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_3.i.i.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(32) %_19.i.i.i.i)
          to label %bb2.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i, !noalias !71

bb11.i.i.i.i.i.i:                                 ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i, %cleanup3.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i
  %.pn.i.i.i.i.i.i = phi { ptr, i32 } [ %33, %cleanup.i.i.i.i.i.i ], [ %47, %cleanup3.i.i.i.i.i.i ], [ %47, %bb2.i.i.i4.i.i.i.i.i.i.i.i ]
; call core::ptr::drop_in_place<std::env::Args>
  call fastcc void @"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd9964c0c546f889dE"(ptr noalias noundef nonnull align 8 dereferenceable(32) %_19.i.i.i.i) #22, !noalias !71
; call core::ptr::drop_in_place<alloc::vec::Vec<alloc::string::String>>
  call fastcc void @"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hd1259156c9080265E"(ptr noalias noundef align 8 dereferenceable(24) %vector.i.i.i.i) #22, !noalias !29
  br label %common.resume

cleanup.i.i.i.i.i.i:                              ; preds = %bb1.i.i.i.i.i.i
  %33 = landingpad { ptr, i32 }
          cleanup
  br label %bb11.i.i.i.i.i.i

bb2.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i.i.i.i
  %34 = load i64, ptr %_3.i.i.i.i.i.i, align 8, !range !30, !noalias !70, !noundef !31
  %.not.i.i.i.i.i.i = icmp eq i64 %34, -9223372036854775808
  br i1 %.not.i.i.i.i.i.i, label %bb9.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %bb2.i.i.i.i.i.i
  %element.sroa.5.0.copyload.i.i.i.i.i.i = load ptr, ptr %element.sroa.5.0._3.sroa_idx.i.i.i.i.i.i, align 8, !noalias !70
  %element.sroa.6.0.copyload.i.i.i.i.i.i = load i64, ptr %element.sroa.6.0._3.sroa_idx.i.i.i.i.i.i, align 8, !noalias !70
  %_20.i.i.i.i.i.i = icmp samesign ult i64 %len.i.i.i.i.i.i, 384307168202282326
  call void @llvm.assume(i1 %_20.i.i.i.i.i.i)
  %self2.i.i.i.i.i.i = load i64, ptr %vector.i.i.i.i, align 8, !range !72, !alias.scope !73, !noalias !74, !noundef !31
  %_8.i.i.i.i.i.i = icmp eq i64 %len.i.i.i.i.i.i, %self2.i.i.i.i.i.i
  br i1 %_8.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i

bb9.i.i.i.i.i.i:                                  ; preds = %bb2.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i.i.i.i.i.i), !noalias !70
  call void @llvm.experimental.noalias.scope.decl(metadata !75)
  call void @llvm.experimental.noalias.scope.decl(metadata !78)
  call void @llvm.experimental.noalias.scope.decl(metadata !81)
  call void @llvm.experimental.noalias.scope.decl(metadata !84)
  call void @llvm.experimental.noalias.scope.decl(metadata !87)
  %self2.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %32, align 8, !alias.scope !90, !noalias !91, !nonnull !31, !noundef !31
  %self.val3.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %31, align 8, !alias.scope !90, !noalias !91, !nonnull !31, !noundef !31
  %35 = ptrtoint ptr %self.val3.i.i.i.i.i.i.i.i.i.i.i to i64
  %36 = ptrtoint ptr %self2.i.i.i.i.i.i.i.i.i.i.i to i64
  %37 = sub nuw i64 %35, %36
  %38 = udiv exact i64 %37, 24
  call void @llvm.experimental.noalias.scope.decl(metadata !92)
  %_710.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %self.val3.i.i.i.i.i.i.i.i.i.i.i, %self2.i.i.i.i.i.i.i.i.i.i.i
  br i1 %_710.i.i.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i.i.i.i.i:                      ; preds = %bb9.i.i.i.i.i.i, %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i.i.i"
  %_3.sroa.0.011.i.i.i.i.i.i.i.i.i.i.i.i = phi i64 [ %39, %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i.i.i" ], [ 0, %bb9.i.i.i.i.i.i ]
  %_6.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self2.i.i.i.i.i.i.i.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i.i.i.i
  %39 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i.i.i.i.i.i.i.i, align 8, !alias.scope !92, !noalias !95
  %40 = icmp eq i64 %_6.val.i.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %40, label %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i.i.i", label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i:         ; preds = %bb5.i.i.i.i.i.i.i.i.i.i.i.i
  %41 = getelementptr i8, ptr %_6.i.i.i.i.i.i.i.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %41, align 8, !alias.scope !92, !noalias !95, !nonnull !31, !noundef !31
; call __rustc::__rust_dealloc
  call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #20, !noalias !96
  br label %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i.i.i"

"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i.i.i": ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %39, %38
  br i1 %_7.i.i.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i.i:                        ; preds = %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i.i.i.i", %bb9.i.i.i.i.i.i
  %42 = getelementptr inbounds nuw i8, ptr %_19.i.i.i.i, i64 16
  %capacity1.i.i5.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %42, align 8, !alias.scope !90, !noalias !91, !noundef !31
  %43 = icmp eq i64 %capacity1.i.i5.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %43, label %_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE.exit, label %bb2.i.i.i.i.i6.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i6.i.i.i.i.i.i.i.i.i.i.i:             ; preds = %bb1.i.i.i.i.i.i.i.i.i.i.i
  %ptr.i.i7.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %_19.i.i.i.i, align 8, !alias.scope !90, !noalias !91, !nonnull !31, !noundef !31
  %44 = mul nuw i64 %capacity1.i.i5.i.i.i.i.i.i.i.i.i.i.i, 24
; call __rustc::__rust_dealloc
  call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i7.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %44, i64 noundef range(i64 1, -9223372036854775807) 8) #20, !noalias !95
  br label %_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE.exit

bb8.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i.bb8.i.i_crit_edge.i.i.i.i, %bb3.i.i.i.i.i.i
  %_25.i.i.i.i.i.i = phi ptr [ %_25.i.i.pre.i.i.i.i, %bb1.i.i.i.bb8.i.i_crit_edge.i.i.i.i ], [ %_25.i.i18.i.i.i.i, %bb3.i.i.i.i.i.i ]
  %45 = getelementptr inbounds nuw %"alloc::string::String", ptr %_25.i.i.i.i.i.i, i64 %len.i.i.i.i.i.i
  store i64 %34, ptr %45, align 8, !noalias !71
  %src.sroa.4.0..sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %45, i64 8
  store ptr %element.sroa.5.0.copyload.i.i.i.i.i.i, ptr %src.sroa.4.0..sroa_idx.i.i.i.i.i.i, align 8, !noalias !71
  %src.sroa.5.0..sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %45, i64 16
  store i64 %element.sroa.6.0.copyload.i.i.i.i.i.i, ptr %src.sroa.5.0..sroa_idx.i.i.i.i.i.i, align 8, !noalias !71
  %46 = add nuw nsw i64 %len.i.i.i.i.i.i, 1
  store i64 %46, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i.i.i, align 8, !alias.scope !73, !noalias !74
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i.i.i.i.i.i), !noalias !70
  br label %bb1.i.i.i.i.i.i

cleanup3.i.i.i.i.i.i:                             ; preds = %bb1.i.i.i.i.i.i.i
  %47 = landingpad { ptr, i32 }
          cleanup
  %48 = icmp eq i64 %34, 0
  br i1 %48, label %bb11.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i:                       ; preds = %cleanup3.i.i.i.i.i.i
  %49 = icmp ne ptr %element.sroa.5.0.copyload.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %49)
; call __rustc::__rust_dealloc
  call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %element.sroa.5.0.copyload.i.i.i.i.i.i, i64 noundef %34, i64 noundef range(i64 1, -9223372036854775807) 1) #20, !noalias !71
  br label %bb11.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i:                                ; preds = %bb3.i.i.i.i.i.i
  %iterator.val7.i.i.i.i.i.i = load ptr, ptr %31, align 8, !alias.scope !97, !noalias !91, !nonnull !31, !noundef !31
  %50 = ptrtoint ptr %iterator.val7.i.i.i.i.i.i to i64
  %iterator.val.i.i.i.i.i.i = load ptr, ptr %32, align 8, !alias.scope !97, !noalias !91, !nonnull !31, !noundef !31
  %51 = ptrtoint ptr %iterator.val.i.i.i.i.i.i to i64
  %52 = sub nuw i64 %50, %51
  %53 = udiv exact i64 %52, 24
  %54 = add nuw nsw i64 %53, 1
; invoke alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle
  invoke fastcc void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he4790f211285b75bE"(ptr noalias noundef nonnull align 8 dereferenceable(24) %vector.i.i.i.i, i64 noundef %len.i.i.i.i.i.i, i64 noundef range(i64 1, 0) %54)
          to label %bb1.i.i.i.bb8.i.i_crit_edge.i.i.i.i unwind label %cleanup3.i.i.i.i.i.i

bb1.i.i.i.bb8.i.i_crit_edge.i.i.i.i:              ; preds = %bb1.i.i.i.i.i.i.i
  %_25.i.i.pre.i.i.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i.i, align 8, !alias.scope !73, !noalias !74
  br label %bb8.i.i.i.i.i.i

bb10.i.i.i.i:                                     ; preds = %bb2.i.i.i4.i.i.i.i.i.i, %cleanup2.i.i.i.i, %cleanup.i.i.i.i
  %.pn.ph.i.i.i.i = phi { ptr, i32 } [ %0, %cleanup.i.i.i.i ], [ %16, %cleanup2.i.i.i.i ], [ %16, %bb2.i.i.i4.i.i.i.i.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !98)
  call void @llvm.experimental.noalias.scope.decl(metadata !101), !noalias !23
  call void @llvm.experimental.noalias.scope.decl(metadata !104), !noalias !23
  call void @llvm.experimental.noalias.scope.decl(metadata !107), !noalias !23
  call void @llvm.experimental.noalias.scope.decl(metadata !110), !noalias !23
  %55 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 8
  %self2.i.i.i.i.i.i.i.i = load ptr, ptr %55, align 8, !alias.scope !113, !noalias !29, !nonnull !31, !noundef !31
  %56 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 24
  %self.val3.i.i.i.i.i.i.i.i = load ptr, ptr %56, align 8, !alias.scope !113, !noalias !29, !nonnull !31, !noundef !31
  %57 = ptrtoint ptr %self.val3.i.i.i.i.i.i.i.i to i64
  %58 = ptrtoint ptr %self2.i.i.i.i.i.i.i.i to i64
  %59 = sub nuw i64 %57, %58
  %60 = udiv exact i64 %59, 24
  call void @llvm.experimental.noalias.scope.decl(metadata !114), !noalias !23
  %_710.i.i.i.i.i.i.i.i.i = icmp eq ptr %self.val3.i.i.i.i.i.i.i.i, %self2.i.i.i.i.i.i.i.i
  br i1 %_710.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i.i:                            ; preds = %bb10.i.i.i.i, %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i"
  %_3.sroa.0.011.i.i.i.i.i.i.i.i.i = phi i64 [ %61, %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i" ], [ 0, %bb10.i.i.i.i ]
  %_6.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self2.i.i.i.i.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i
  %61 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i.i.i.i.i, align 8, !alias.scope !114, !noalias !117
  %62 = icmp eq i64 %_6.val.i.i.i.i.i.i.i.i.i, 0
  br i1 %62, label %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i", label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i:               ; preds = %bb5.i.i.i.i.i.i.i.i.i
  %63 = getelementptr i8, ptr %_6.i.i.i.i.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i.i.i.i.i = load ptr, ptr %63, align 8, !alias.scope !114, !noalias !117, !nonnull !31, !noundef !31
; call __rustc::__rust_dealloc
  call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #20, !noalias !118
  br label %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i"

"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i": ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = icmp eq i64 %61, %60
  br i1 %_7.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i:                              ; preds = %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i.i.i.i.i", %bb10.i.i.i.i
  %64 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 16
  %capacity1.i.i5.i.i.i.i.i.i.i.i = load i64, ptr %64, align 8, !alias.scope !113, !noalias !29, !noundef !31
  %65 = icmp eq i64 %capacity1.i.i5.i.i.i.i.i.i.i.i, 0
  br i1 %65, label %common.resume, label %bb2.i.i.i.i.i6.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i6.i.i.i.i.i.i.i.i:                   ; preds = %bb1.i.i.i.i.i.i.i.i
  %ptr.i.i7.i.i.i.i.i.i.i.i = load ptr, ptr %_2.i.i, align 8, !alias.scope !113, !noalias !29, !nonnull !31, !noundef !31
  %66 = mul nuw i64 %capacity1.i.i5.i.i.i.i.i.i.i.i, 24
; call __rustc::__rust_dealloc
  call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i7.i.i.i.i.i.i.i.i, i64 noundef %66, i64 noundef range(i64 1, -9223372036854775807) 8) #20, !noalias !117
  br label %common.resume

common.resume:                                    ; preds = %bb11.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i6.i.i.i.i.i.i.i.i, %cleanup.body
  %common.resume.op = phi { ptr, i32 } [ %eh.lpad-body, %cleanup.body ], [ %.pn.i.i.i.i.i.i, %bb11.i.i.i.i.i.i ], [ %.pn.ph.i.i.i.i, %bb1.i.i.i.i.i.i.i.i ], [ %.pn.ph.i.i.i.i, %bb2.i.i.i.i.i6.i.i.i.i.i.i.i.i ]
  resume { ptr, i32 } %common.resume.op

_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE.exit.thread: ; preds = %bb1.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i6.i.i.i.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_2.i.i), !noalias !10
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_2)
  br label %panic

_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE.exit: ; preds = %bb1.i.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i6.i.i.i.i.i.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_19.i.i.i.i), !noalias !28
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %args, ptr noundef nonnull align 8 dereferenceable(24) %vector.i.i.i.i, i64 24, i1 false), !noalias !33
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i.i.i), !noalias !28
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %args, i64 16
  %_3.pre = load i64, ptr %.phi.trans.insert, align 8
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_2.i.i), !noalias !10
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_2)
  %_10 = icmp ult i64 %_3.pre, 384307168202282326
  call void @llvm.assume(i1 %_10)
  switch i64 %_3.pre, label %bb12 [
    i64 1, label %bb4
    i64 0, label %panic
  ]

bb4:                                              ; preds = %_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE.exit
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_5)
  store ptr @alloc_da03007fb5fe1aab10d9ffd81ef29605, ptr %_5, align 8
  %67 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  store i64 1, ptr %67, align 8
  %68 = getelementptr inbounds nuw i8, ptr %_5, i64 32
  store ptr null, ptr %68, align 8
  %69 = getelementptr inbounds nuw i8, ptr %_5, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %69, align 8
  %70 = getelementptr inbounds nuw i8, ptr %_5, i64 24
  store i64 0, ptr %70, align 8
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h1a48aee8b229bf08E(ptr noalias noundef nonnull align 8 captures(address) dereferenceable(48) %_5)
          to label %bb8.thread unwind label %cleanup

cleanup:                                          ; preds = %is_not_null.i27.i, %bb2.i, %panic, %bb4
  %71 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body

cleanup.body:                                     ; preds = %bb8.i, %cleanup
  %eh.lpad-body = phi { ptr, i32 } [ %71, %cleanup ], [ %90, %bb8.i ]
; call core::ptr::drop_in_place<alloc::vec::Vec<alloc::string::String>>
  call fastcc void @"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hd1259156c9080265E"(ptr noalias noundef align 8 dereferenceable(24) %args) #22
  br label %common.resume

bb8.thread:                                       ; preds = %bb4
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_5)
  %.phi.trans.insert23 = getelementptr inbounds nuw i8, ptr %args, i64 8
  %_1.val.i.pre = load ptr, ptr %.phi.trans.insert23, align 8, !alias.scope !119
  br label %bb5.i.i.i.preheader

bb8:                                              ; preds = %bb4.i5, %is_not_null.i27.i
  call void @llvm.experimental.noalias.scope.decl(metadata !119)
  call void @llvm.experimental.noalias.scope.decl(metadata !122)
  %_710.i.i.i = icmp eq i64 %_3.pre, 0
  br i1 %_710.i.i.i, label %bb4.i, label %bb5.i.i.i.preheader

bb5.i.i.i.preheader:                              ; preds = %bb8.thread, %bb8
  %_1.val.i29 = phi ptr [ %_1.val.i.pre, %bb8.thread ], [ %_13, %bb8 ]
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb5.i.i.i.preheader, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i.i"
  %_3.sroa.0.011.i.i.i = phi i64 [ %72, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i.i" ], [ 0, %bb5.i.i.i.preheader ]
  %_6.i.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %_1.val.i29, i64 %_3.sroa.0.011.i.i.i
  %72 = add nuw nsw i64 %_3.sroa.0.011.i.i.i, 1
  %_6.val.i.i.i = load i64, ptr %_6.i.i.i, align 8, !alias.scope !122, !noalias !119
  %73 = icmp eq i64 %_6.val.i.i.i, 0
  br i1 %73, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i.i", label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %bb5.i.i.i
  %74 = getelementptr i8, ptr %_6.i.i.i, i64 8
  %_6.val7.i.i.i = load ptr, ptr %74, align 8, !alias.scope !122, !noalias !119, !nonnull !31, !noundef !31
; call __rustc::__rust_dealloc
  call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i, i64 noundef %_6.val.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #20, !noalias !125
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i.i"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i.i": ; preds = %bb2.i.i.i4.i.i.i.i.i, %bb5.i.i.i
  %_7.i.i.i = icmp eq i64 %72, %_3.pre
  br i1 %_7.i.i.i, label %bb4.i, label %bb5.i.i.i

bb4.i:                                            ; preds = %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i.i", %bb8
  %_1.val.i30 = phi ptr [ %_13, %bb8 ], [ %_1.val.i29, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i.i" ]
  %_1.val4.i = load i64, ptr %args, align 8, !range !72, !alias.scope !119, !noundef !31
  %75 = icmp eq i64 %_1.val4.i, 0
  br i1 %75, label %"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hd1259156c9080265E.exit", label %bb2.i.i.i6.i

bb2.i.i.i6.i:                                     ; preds = %bb4.i
  %76 = mul nuw i64 %_1.val4.i, 24
; call __rustc::__rust_dealloc
  call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i30, i64 noundef %76, i64 noundef range(i64 1, -9223372036854775807) 8) #20, !noalias !119
  br label %"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hd1259156c9080265E.exit"

"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hd1259156c9080265E.exit": ; preds = %bb4.i, %bb2.i.i.i6.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %args)
  ret void

bb12:                                             ; preds = %_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE.exit
  %77 = getelementptr inbounds nuw i8, ptr %args, i64 8
  %_13 = load ptr, ptr %77, align 8, !nonnull !31, !noundef !31
  %78 = getelementptr inbounds nuw i8, ptr %_13, i64 32
  %_21 = load ptr, ptr %78, align 8, !nonnull !31, !noundef !31
  %79 = getelementptr inbounds nuw i8, ptr %_13, i64 40
  %_20 = load i64, ptr %79, align 8, !noundef !31
  switch i64 %_20, label %bb9.i [
    i64 0, label %bb2.i
    i64 1, label %bb7.i
  ]

bb7.i:                                            ; preds = %bb12
  %80 = load i8, ptr %_21, align 1, !alias.scope !126, !noalias !129, !noundef !31
  switch i8 %80, label %bb14.i.preheader [
    i8 43, label %bb2.i
    i8 45, label %bb2.i
  ]

bb14.i.preheader:                                 ; preds = %bb11.i, %bb40.i, %bb7.i
  %src.sroa.0.156.i.ph = phi ptr [ %rest.0.i, %bb11.i ], [ %_21, %bb7.i ], [ %_21, %bb40.i ]
  %src.sroa.17.155.i.ph = phi i64 [ %rest.1.i, %bb11.i ], [ 1, %bb7.i ], [ %_20, %bb40.i ]
  br label %bb14.i

bb9.i:                                            ; preds = %bb12
  %.pr.i = load i8, ptr %_21, align 1, !alias.scope !126, !noalias !129
  %cond.i = icmp eq i8 %.pr.i, 43
  br i1 %cond.i, label %bb11.i, label %bb40.i

bb11.i:                                           ; preds = %bb9.i
  %rest.0.i = getelementptr inbounds nuw i8, ptr %_21, i64 1
  %rest.1.i = add i64 %_20, -1
  %81 = icmp ult i64 %_20, 18
  br i1 %81, label %bb14.i.preheader, label %bb19.i.preheader

bb19.i.preheader:                                 ; preds = %bb40.i, %bb11.i
  %src.sroa.17.0.i.ph = phi i64 [ %rest.1.i, %bb11.i ], [ %_20, %bb40.i ]
  %src.sroa.0.0.i.ph = phi ptr [ %rest.0.i, %bb11.i ], [ %_21, %bb40.i ]
  br label %bb19.i

bb19.i:                                           ; preds = %bb19.i.preheader, %bb23.i
  %result.sroa.0.0.i = phi i64 [ %_68.0.i, %bb23.i ], [ 0, %bb19.i.preheader ]
  %src.sroa.17.0.i = phi i64 [ %rest.13.i, %bb23.i ], [ %src.sroa.17.0.i.ph, %bb19.i.preheader ]
  %src.sroa.0.0.i = phi ptr [ %rest.02.i, %bb23.i ], [ %src.sroa.0.0.i.ph, %bb19.i.preheader ]
  %_31.not.i = icmp eq i64 %src.sroa.17.0.i, 0
  br i1 %_31.not.i, label %bb6, label %bb20.i

bb40.i:                                           ; preds = %bb9.i
  %82 = icmp ult i64 %_20, 17
  br i1 %82, label %bb14.i.preheader, label %bb19.i.preheader

bb20.i:                                           ; preds = %bb19.i
  %_39.i = load i8, ptr %src.sroa.0.0.i, align 1, !alias.scope !126, !noalias !129, !noundef !31
  %_38.i = zext i8 %_39.i to i32
  %83 = add nsw i32 %_38.i, -48
  %_14.i.i = icmp ult i32 %83, 10
  br i1 %_14.i.i, label %bb23.i, label %bb2.i

bb23.i:                                           ; preds = %bb20.i
  %84 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %result.sroa.0.0.i, i64 10)
  %_62.0.i = extractvalue { i64, i1 } %84, 0
  %rest.13.i = add i64 %src.sroa.17.0.i, -1
  %rest.02.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.0.i, i64 1
  %_62.1.i = extractvalue { i64, i1 } %84, 1
  %x.i = zext nneg i32 %83 to i64
  %_68.0.i = add i64 %_62.0.i, %x.i
  %_68.1.not.i = icmp ult i64 %_68.0.i, %_62.0.i
  %or.cond = select i1 %_62.1.i, i1 true, i1 %_68.1.not.i
  br i1 %or.cond, label %bb2.i, label %bb19.i

bb14.i:                                           ; preds = %bb14.i.preheader, %bb18.i
  %src.sroa.0.156.i = phi ptr [ %rest.05.i, %bb18.i ], [ %src.sroa.0.156.i.ph, %bb14.i.preheader ]
  %src.sroa.17.155.i = phi i64 [ %rest.16.i, %bb18.i ], [ %src.sroa.17.155.i.ph, %bb14.i.preheader ]
  %result.sroa.0.154.i = phi i64 [ %87, %bb18.i ], [ 0, %bb14.i.preheader ]
  %_21.i = load i8, ptr %src.sroa.0.156.i, align 1, !alias.scope !126, !noalias !129, !noundef !31
  %_20.i = zext i8 %_21.i to i32
  %85 = add nsw i32 %_20.i, -48
  %_14.i46.i = icmp ult i32 %85, 10
  br i1 %_14.i46.i, label %bb18.i, label %bb2.i

bb18.i:                                           ; preds = %bb14.i
  %86 = mul i64 %result.sroa.0.154.i, 10
  %rest.16.i = add nsw i64 %src.sroa.17.155.i, -1
  %rest.05.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.156.i, i64 1
  %_25.i = zext nneg i32 %85 to i64
  %87 = add i64 %86, %_25.i
  %_14.not.i = icmp eq i64 %rest.16.i, 0
  br i1 %_14.not.i, label %bb6, label %bb14.i

panic:                                            ; preds = %_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE.exit.thread, %_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE.exit
; invoke core::panicking::panic_bounds_check
  invoke void @_ZN4core9panicking18panic_bounds_check17h939af04696df73c0E(i64 noundef 1, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_be681e3131e727a983aaa4cb9118ba6c) #21
          to label %unreachable unwind label %cleanup

unreachable:                                      ; preds = %panic
  unreachable

bb2.i:                                            ; preds = %bb20.i, %bb23.i, %bb14.i, %bb12, %bb7.i, %bb7.i
  %_7.sroa.4.0.ph = phi i8 [ 1, %bb7.i ], [ 1, %bb7.i ], [ 0, %bb12 ], [ 1, %bb14.i ], [ 1, %bb20.i ], [ 2, %bb23.i ]
  call void @llvm.lifetime.start.p0(i64 1, ptr nonnull %e.i), !noalias !131
  store i8 %_7.sroa.4.0.ph, ptr %e.i, align 1, !noalias !131
; invoke core::result::unwrap_failed
  invoke void @_ZN4core6result13unwrap_failed17haff0608dd3f08819E(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_068ab455f5bf47d6087b5a8187125903) #21
          to label %.noexc unwind label %cleanup

.noexc:                                           ; preds = %bb2.i
  unreachable

bb6:                                              ; preds = %bb19.i, %bb18.i
  %_7.sroa.117.0 = phi i64 [ %87, %bb18.i ], [ %result.sroa.0.0.i, %bb19.i ]
  %88 = icmp eq i64 %_7.sroa.117.0, 0
  %spec.select.i.i = select i1 %88, ptr @vtable.0, ptr @vtable.1
  %89 = select i1 %88, ptr @"_ZN70_$LT$original_norewrite..Cat$u20$as$u20$original_norewrite..Animal$GT$5kaeps17h04e3b6378d9a5731E", ptr @"_ZN70_$LT$original_norewrite..Dog$u20$as$u20$original_norewrite..Animal$GT$5kaeps17hcc0497b26c88a419E"
  invoke void %89(ptr noundef nonnull align 1 inttoptr (i64 1 to ptr))
          to label %bb4.i5 unwind label %cleanup2.i

bb8.i:                                            ; preds = %bb7.i4
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn original_norewrite::Animal>>
  invoke fastcc void @"_ZN4core3ptr80drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$original_norewrite..Animal$GT$$GT$17h99635577d4c892bdE"(ptr nonnull inttoptr (i64 1 to ptr), ptr nonnull %spec.select.i.i) #22
          to label %cleanup.body unwind label %terminate.i

bb7.i4:                                           ; preds = %cleanup2.i
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn original_norewrite::Animal>>
  invoke fastcc void @"_ZN4core3ptr80drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$original_norewrite..Animal$GT$$GT$17h99635577d4c892bdE"(ptr nonnull inttoptr (i64 1 to ptr), ptr nonnull @vtable.0) #22
          to label %bb8.i unwind label %terminate.i

cleanup2.i:                                       ; preds = %bb6
  %90 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn original_norewrite::Animal>>
  invoke fastcc void @"_ZN4core3ptr80drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$original_norewrite..Animal$GT$$GT$17h99635577d4c892bdE"(ptr nonnull inttoptr (i64 1 to ptr), ptr nonnull @vtable.1) #22
          to label %bb7.i4 unwind label %terminate.i

bb4.i5:                                           ; preds = %bb6
  %91 = load ptr, ptr %spec.select.i.i, align 8, !invariant.load !31
  %.not.i26.i = icmp eq ptr %91, null
  br i1 %.not.i26.i, label %bb8, label %is_not_null.i27.i

is_not_null.i27.i:                                ; preds = %bb4.i5
  invoke void %91(ptr noundef nonnull inttoptr (i64 1 to ptr))
          to label %bb8 unwind label %cleanup

terminate.i:                                      ; preds = %cleanup2.i, %bb7.i4, %bb8.i
  %92 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h88c440a45b19874fE() #23
  unreachable
}

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden noundef i64 @_ZN3std2rt10lang_start17hb199dba3ce675e2aE(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7)
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_ZN3std2rt19lang_start_internal17h7d87538fea49e523E(ptr noundef nonnull align 1 %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.2, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7)
  ret i64 %_0
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64790d7c15d240fbE"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #1 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !31, !noundef !31
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17haa742f03ed74cee3E(ptr noundef nonnull %_4)
  ret i32 0
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17haa742f03ed74cee3E(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #2 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #20, !srcloc !134
  ret void
}

; <&T as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
define internal noundef zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he91003c1d8bf684aE"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !31, !align !135, !noundef !31
  %_3.val = load i8, ptr %_3, align 1, !range !136, !noundef !31
  %0 = zext nneg i8 %_3.val to i64
  %switch.gep = getelementptr inbounds nuw [5 x i64], ptr @"switch.table._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he91003c1d8bf684aE", i64 0, i64 %0
  %switch.load = load i64, ptr %switch.gep, align 8
  %1 = zext nneg i8 %_3.val to i64
  %reltable.shift = shl i64 %1, 2
  %reltable.intrinsic = call ptr @llvm.load.relative.i64(ptr @"switch.table._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he91003c1d8bf684aE.30.rel", i64 %reltable.shift)
; call core::fmt::Formatter::write_str
  %_0.i = tail call noundef zeroext i1 @_ZN4core3fmt9Formatter9write_str17h925affe97b034470E(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %reltable.intrinsic, i64 noundef %switch.load)
  ret i1 %_0.i
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17haf9d08ae32638f21E"(ptr noundef readonly captures(none) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !31, !noundef !31
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17haa742f03ed74cee3E(ptr noundef nonnull readonly %0), !noalias !137
  ret i32 0
}

; core::ptr::drop_in_place<std::env::Args>
; Function Attrs: nounwind nonlazybind uwtable
define internal fastcc void @"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd9964c0c546f889dE"(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !140)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !143)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !146)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !149)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %self2.i.i.i.i = load ptr, ptr %0, align 8, !alias.scope !152, !nonnull !31, !noundef !31
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %self.val3.i.i.i.i = load ptr, ptr %1, align 8, !alias.scope !152, !nonnull !31, !noundef !31
  %2 = ptrtoint ptr %self.val3.i.i.i.i to i64
  %3 = ptrtoint ptr %self2.i.i.i.i to i64
  %4 = sub nuw i64 %2, %3
  %5 = udiv exact i64 %4, 24
  tail call void @llvm.experimental.noalias.scope.decl(metadata !153)
  %_710.i.i.i.i.i = icmp eq ptr %self.val3.i.i.i.i, %self2.i.i.i.i
  br i1 %_710.i.i.i.i.i, label %bb1.i.i.i.i, label %bb5.i.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %start, %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i"
  %_3.sroa.0.011.i.i.i.i.i = phi i64 [ %6, %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i" ], [ 0, %start ]
  %_6.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self2.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i
  %6 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i, align 8, !alias.scope !153, !noalias !152
  %7 = icmp eq i64 %_6.val.i.i.i.i.i, 0
  br i1 %7, label %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i", label %bb2.i.i.i4.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i:                       ; preds = %bb5.i.i.i.i.i
  %8 = getelementptr i8, ptr %_6.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i = load ptr, ptr %8, align 8, !alias.scope !153, !noalias !152, !nonnull !31, !noundef !31
; call __rustc::__rust_dealloc
  tail call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #20, !noalias !156
  br label %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i"

"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i": ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i
  %_7.i.i.i.i.i = icmp eq i64 %6, %5
  br i1 %_7.i.i.i.i.i, label %bb1.i.i.i.i, label %bb5.i.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hce881811b18d2d2dE.exit.i.i.i.i.i", %start
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %capacity1.i.i5.i.i.i.i = load i64, ptr %9, align 8, !alias.scope !152, !noundef !31
  %10 = icmp eq i64 %capacity1.i.i5.i.i.i.i, 0
  br i1 %10, label %"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17ha76ffc5fa2a0256bE.exit", label %bb2.i.i.i.i.i6.i.i.i.i

bb2.i.i.i.i.i6.i.i.i.i:                           ; preds = %bb1.i.i.i.i
  %ptr.i.i7.i.i.i.i = load ptr, ptr %_1, align 8, !alias.scope !152, !nonnull !31, !noundef !31
  %11 = mul nuw i64 %capacity1.i.i5.i.i.i.i, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i7.i.i.i.i, i64 noundef %11, i64 noundef range(i64 1, -9223372036854775807) 8) #20, !noalias !152
  br label %"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17ha76ffc5fa2a0256bE.exit"

"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17ha76ffc5fa2a0256bE.exit": ; preds = %bb1.i.i.i.i, %bb2.i.i.i.i.i6.i.i.i.i
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<alloc::string::String>>
; Function Attrs: nounwind nonlazybind uwtable
define internal fastcc void @"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hd1259156c9080265E"(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val = load ptr, ptr %0, align 8, !nonnull !31, !noundef !31
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i64, ptr %1, align 8, !noundef !31
  tail call void @llvm.experimental.noalias.scope.decl(metadata !157)
  %_710.i.i = icmp eq i64 %_1.val1, 0
  br i1 %_710.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i"
  %_3.sroa.0.011.i.i = phi i64 [ %2, %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i" ], [ 0, %start ]
  %_6.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %_1.val, i64 %_3.sroa.0.011.i.i
  %2 = add nuw i64 %_3.sroa.0.011.i.i, 1
  %_6.val.i.i = load i64, ptr %_6.i.i, align 8, !alias.scope !157
  %3 = icmp eq i64 %_6.val.i.i, 0
  br i1 %3, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i", label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %bb5.i.i
  %4 = getelementptr i8, ptr %_6.i.i, i64 8
  %_6.val7.i.i = load ptr, ptr %4, align 8, !alias.scope !157, !nonnull !31, !noundef !31
; call __rustc::__rust_dealloc
  tail call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i, i64 noundef %_6.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #20, !noalias !157
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i": ; preds = %bb2.i.i.i4.i.i.i.i, %bb5.i.i
  %_7.i.i = icmp eq i64 %2, %_1.val1
  br i1 %_7.i.i, label %bb4, label %bb5.i.i

bb4:                                              ; preds = %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h2703bd204c2c153bE.exit.i.i", %start
  %_1.val4 = load i64, ptr %_1, align 8, !range !72, !noundef !31
  %5 = icmp eq i64 %_1.val4, 0
  br i1 %5, label %"_ZN4core3ptr72drop_in_place$LT$alloc..raw_vec..RawVec$LT$alloc..string..String$GT$$GT$17ha1a6da45e341f93fE.exit7", label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %6 = mul nuw i64 %_1.val4, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) 8) #20
  br label %"_ZN4core3ptr72drop_in_place$LT$alloc..raw_vec..RawVec$LT$alloc..string..String$GT$$GT$17ha1a6da45e341f93fE.exit7"

"_ZN4core3ptr72drop_in_place$LT$alloc..raw_vec..RawVec$LT$alloc..string..String$GT$$GT$17ha1a6da45e341f93fE.exit7": ; preds = %bb4, %bb2.i.i.i6
  ret void
}

; core::ptr::drop_in_place<alloc::boxed::Box<dyn original_norewrite::Animal>>
; Function Attrs: nonlazybind uwtable
define internal fastcc void @"_ZN4core3ptr80drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$original_norewrite..Animal$GT$$GT$17h99635577d4c892bdE"(ptr %_1.0.val, ptr readonly captures(address_is_null) %_1.8.val) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %0)
  %1 = load ptr, ptr %_1.8.val, align 8, !invariant.load !31
  %.not = icmp eq ptr %1, null
  br i1 %.not, label %bb3, label %is_not_null

is_not_null:                                      ; preds = %start
  %2 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %2)
  invoke void %1(ptr noundef nonnull %_1.0.val)
          to label %bb3 unwind label %cleanup

bb3:                                              ; preds = %is_not_null, %start
  %3 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %3)
  %4 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 8
  %5 = load i64, ptr %4, align 8, !range !72, !invariant.load !31
  %6 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 16
  %7 = load i64, ptr %6, align 8, !range !160, !invariant.load !31
  %8 = add i64 %7, -1
  %9 = icmp sgt i64 %8, -1
  tail call void @llvm.assume(i1 %9)
  %10 = icmp eq i64 %5, 0
  br i1 %10, label %"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1171bb232744c777E.exit", label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h430321929044cf73E.exit.i"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h430321929044cf73E.exit.i": ; preds = %bb3
; call __rustc::__rust_dealloc
  tail call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) %7) #20
  br label %"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1171bb232744c777E.exit"

"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1171bb232744c777E.exit": ; preds = %bb3, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h430321929044cf73E.exit.i"
  ret void

cleanup:                                          ; preds = %is_not_null
  %11 = landingpad { ptr, i32 }
          cleanup
  %12 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 8
  %13 = load i64, ptr %12, align 8, !range !72, !invariant.load !31
  %14 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 16
  %15 = load i64, ptr %14, align 8, !range !160, !invariant.load !31
  %16 = add i64 %15, -1
  %17 = icmp sgt i64 %16, -1
  tail call void @llvm.assume(i1 %17)
  %18 = icmp eq i64 %13, 0
  br i1 %18, label %bb1, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h430321929044cf73E.exit.i4"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h430321929044cf73E.exit.i4": ; preds = %cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %13, i64 noundef range(i64 1, -9223372036854775807) %15) #20
  br label %bb1

bb1:                                              ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h430321929044cf73E.exit.i4", %cleanup
  resume { ptr, i32 } %11
}

; alloc::raw_vec::finish_grow
; Function Attrs: cold nonlazybind uwtable
define internal fastcc void @_ZN5alloc7raw_vec11finish_grow17hd95b0468f037c5f0E(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, i64 noundef range(i64 1, -9223372036854775807) %0, i64 noundef %1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %current_memory) unnamed_addr #4 {
start:
  %_12.i.i = alloca [48 x i8], align 8
  %_16 = alloca [48 x i8], align 8
  %_12 = alloca [8 x i8], align 8
  %_9 = alloca [8 x i8], align 8
  %2 = getelementptr inbounds nuw i8, ptr %current_memory, i64 8
  %3 = load i64, ptr %2, align 8, !range !30, !noundef !31
  %.not = icmp eq i64 %3, 0
  br i1 %.not, label %bb5, label %bb1

bb1:                                              ; preds = %start
  %ptr = load ptr, ptr %current_memory, align 8, !nonnull !31, !noundef !31
  %4 = getelementptr inbounds nuw i8, ptr %current_memory, i64 16
  %5 = load i64, ptr %4, align 8, !noundef !31
  store i64 %3, ptr %_9, align 8
  store i64 %0, ptr %_12, align 8
  %_14 = icmp eq i64 %3, %0
  br i1 %_14, label %bb2, label %bb3, !prof !161

bb5:                                              ; preds = %start
  %6 = icmp eq i64 %1, 0
  br i1 %6, label %bb2.i.i, label %bb4.i.i

bb2.i.i:                                          ; preds = %bb5
  %_19.i.i = getelementptr i8, ptr null, i64 %0
  br label %bb7

bb4.i.i:                                          ; preds = %bb5
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCscszlnfU7ykQ_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #20
; call __rustc::__rust_alloc
  %7 = tail call noundef ptr @_RNvCscszlnfU7ykQ_7___rustc12___rust_alloc(i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) %0) #20
  br label %bb7

bb3:                                              ; preds = %bb1
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_16)
  store ptr null, ptr %_16, align 8
; call core::panicking::assert_failed
  call void @_ZN4core9panicking13assert_failed17hc0c6a54360149362E(i8 noundef 0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %_9, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %_12, ptr noalias noundef nonnull align 8 captures(address) dereferenceable(48) %_16, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cf7a2c70ecf5a70905a3b8fb705afc60) #21
  unreachable

bb2:                                              ; preds = %bb1
  %_6.not.i.i = icmp ult i64 %1, %5
  br i1 %_6.not.i.i, label %bb2.i.i14, label %bb1.i.i, !prof !162

bb2.i.i14:                                        ; preds = %bb2
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_12.i.i)
  store ptr @alloc_04056f6d76887c0653320aca2f1cbe49, ptr %_12.i.i, align 8
  %8 = getelementptr inbounds nuw i8, ptr %_12.i.i, i64 8
  store i64 1, ptr %8, align 8
  %9 = getelementptr inbounds nuw i8, ptr %_12.i.i, i64 32
  store ptr null, ptr %9, align 8
  %10 = getelementptr inbounds nuw i8, ptr %_12.i.i, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %10, align 8
  %11 = getelementptr inbounds nuw i8, ptr %_12.i.i, i64 24
  store i64 0, ptr %11, align 8
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h82731289384e3556E(ptr noalias noundef nonnull readonly align 8 captures(address) dereferenceable(48) %_12.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f38a57380c4169a099c5ec78ee6b9ee) #21
  unreachable

bb1.i.i:                                          ; preds = %bb2
  %12 = icmp eq i64 %5, 0
  br i1 %12, label %bb4.i.i13, label %bb5.i.i

bb4.i.i13:                                        ; preds = %bb1.i.i
  %13 = icmp eq i64 %1, 0
  br i1 %13, label %bb2.i.i.i, label %bb4.i.i.i

bb2.i.i.i:                                        ; preds = %bb4.i.i13
  %_19.i.i.i = getelementptr i8, ptr null, i64 %0
  br label %bb7

bb4.i.i.i:                                        ; preds = %bb4.i.i13
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCscszlnfU7ykQ_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #20
; call __rustc::__rust_alloc
  %14 = tail call noundef ptr @_RNvCscszlnfU7ykQ_7___rustc12___rust_alloc(i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) %0) #20
  br label %bb7

bb5.i.i:                                          ; preds = %bb1.i.i
; call __rustc::__rust_realloc
  %15 = tail call noundef ptr @_RNvCscszlnfU7ykQ_7___rustc14___rust_realloc(ptr noundef nonnull %ptr, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) %0, i64 noundef %1) #20
  br label %bb7

bb7:                                              ; preds = %bb5.i.i, %bb4.i.i.i, %bb2.i.i.i, %bb4.i.i, %bb2.i.i
  %_0.sroa.0.0.i.i12.pn = phi ptr [ %_19.i.i, %bb2.i.i ], [ %7, %bb4.i.i ], [ %15, %bb5.i.i ], [ %_19.i.i.i, %bb2.i.i.i ], [ %14, %bb4.i.i.i ]
  %16 = icmp eq ptr %_0.sroa.0.0.i.i12.pn, null
  %17 = inttoptr i64 %0 to ptr
  %spec.select = select i1 %16, ptr %17, ptr %_0.sroa.0.0.i.i12.pn
  %spec.select4 = zext i1 %16 to i64
  %18 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %spec.select, ptr %18, align 8
  %19 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %1, ptr %19, align 8
  store i64 %spec.select4, ptr %_0, align 8
  ret void
}

; alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle
; Function Attrs: cold nonlazybind uwtable
define internal fastcc void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17he4790f211285b75bE"(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef range(i64 1, 0) %additional) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %_28.i = alloca [24 x i8], align 8
  %self5.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !163)
  %_34.0.i = add i64 %additional, %len
  %_34.1.i = icmp ult i64 %_34.0.i, %len
  br i1 %_34.1.i, label %bb2, label %bb13.i, !prof !162

bb13.i:                                           ; preds = %start
  %self9.i = load i64, ptr %slf, align 8, !range !72, !alias.scope !163, !noundef !31
  %v110.i = shl nuw i64 %self9.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_34.0.i, i64 range(i64 0, -1) %v110.i)
  %_0.sroa.0.0.i36.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 4)
  %_21.0.i.i = mul i64 %_0.sroa.0.0.i36.i, 24
  %or.cond.i.i = icmp ugt i64 %_0.sroa.0.0.i.i, 384307168202282325
  br i1 %or.cond.i.i, label %bb2, label %bb24.i, !prof !56

bb24.i:                                           ; preds = %bb13.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self5.i), !noalias !163
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_28.i), !noalias !163
  %0 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %1 = icmp eq i64 %self9.i, 0
  br i1 %1, label %"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14current_memory17hf933bf23e667f361E.exit.i", label %bb4.i.i

bb4.i.i:                                          ; preds = %bb24.i
  %self.val34.i = load ptr, ptr %0, align 8, !alias.scope !163, !nonnull !31, !noundef !31
  %2 = mul nuw i64 %self9.i, 24
  store ptr %self.val34.i, ptr %_28.i, align 8, !alias.scope !166, !noalias !163
  %_15.sroa.5.0._0.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_28.i, i64 16
  store i64 %2, ptr %_15.sroa.5.0._0.sroa_idx.i.i, align 8, !alias.scope !166, !noalias !163
  br label %"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14current_memory17hf933bf23e667f361E.exit.i"

"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14current_memory17hf933bf23e667f361E.exit.i": ; preds = %bb4.i.i, %bb24.i
  %.sink.i.i = phi i64 [ 8, %bb4.i.i ], [ 0, %bb24.i ]
  %3 = getelementptr inbounds nuw i8, ptr %_28.i, i64 8
  store i64 %.sink.i.i, ptr %3, align 8, !alias.scope !166, !noalias !163
; call alloc::raw_vec::finish_grow
  call fastcc void @_ZN5alloc7raw_vec11finish_grow17hd95b0468f037c5f0E(ptr noalias noundef align 8 captures(address) dereferenceable(24) %self5.i, i64 noundef 8, i64 noundef %_21.0.i.i, ptr noalias noundef readonly align 8 captures(address) dereferenceable(24) %_28.i), !noalias !163
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_28.i), !noalias !163
  %_55.i = load i64, ptr %self5.i, align 8, !range !169, !noalias !163, !noundef !31
  %4 = trunc nuw i64 %_55.i to i1
  %5 = getelementptr inbounds nuw i8, ptr %self5.i, i64 8
  br i1 %4, label %bb25.i, label %bb3

bb25.i:                                           ; preds = %"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14current_memory17hf933bf23e667f361E.exit.i"
  %e.0.i = load i64, ptr %5, align 8, !range !30, !noalias !163, !noundef !31
  %6 = getelementptr inbounds nuw i8, ptr %self5.i, i64 16
  %e.1.i = load i64, ptr %6, align 8, !noalias !163
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self5.i), !noalias !163
  br label %bb2

bb2:                                              ; preds = %bb25.i, %start, %bb13.i
  %_0.sroa.6.0.i.ph = phi i64 [ undef, %bb13.i ], [ undef, %start ], [ %e.1.i, %bb25.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %bb13.i ], [ 0, %start ], [ %e.0.i, %bb25.i ]
; call alloc::raw_vec::handle_error
  tail call void @_ZN5alloc7raw_vec12handle_error17h4842bb444d407eecE(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.6.0.i.ph) #21
  unreachable

bb3:                                              ; preds = %"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14current_memory17hf933bf23e667f361E.exit.i"
  %v.014.i = load ptr, ptr %5, align 8, !noalias !163, !nonnull !31, !noundef !31
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self5.i), !noalias !163
  store ptr %v.014.i, ptr %0, align 8, !alias.scope !163
  %7 = icmp sgt i64 %_0.sroa.0.0.i36.i, -1
  tail call void @llvm.assume(i1 %7)
  store i64 %_0.sroa.0.0.i36.i, ptr %slf, align 8, !alias.scope !163
  ret void
}

; <core::num::error::ParseIntError as core::fmt::Debug>::fmt
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef zeroext i1 @"_ZN68_$LT$core..num..error..ParseIntError$u20$as$u20$core..fmt..Debug$GT$3fmt17h0f6568d5dfa9eaefE"(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %_5 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_5)
  store ptr %self, ptr %_5, align 8
; call core::fmt::Formatter::debug_struct_field1_finish
  %_0 = call noundef zeroext i1 @_ZN4core3fmt9Formatter26debug_struct_field1_finish17h51b6627e6baebca0E(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_f62df14955f7d78bca139b0a7668683d, i64 noundef 13, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a5d866b1768ad3f826bccdb004a1a8ae, i64 noundef 4, ptr noundef nonnull align 1 %_5, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.6)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_5)
  ret i1 %_0
}

; <original_norewrite::Cat as original_norewrite::Animal>::kaeps
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN70_$LT$original_norewrite..Cat$u20$as$u20$original_norewrite..Animal$GT$5kaeps17h04e3b6378d9a5731E"(ptr noalias nonnull readonly align 1 captures(none) %self) unnamed_addr #0 {
start:
  %_3 = alloca [48 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3)
  store ptr @alloc_000bc512779c9a763a8aa84ee52b6421, ptr %_3, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  store i64 1, ptr %0, align 8
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 32
  store ptr null, ptr %1, align 8
  %2 = getelementptr inbounds nuw i8, ptr %_3, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %2, align 8
  %3 = getelementptr inbounds nuw i8, ptr %_3, i64 24
  store i64 0, ptr %3, align 8
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h1a48aee8b229bf08E(ptr noalias noundef nonnull align 8 captures(address) dereferenceable(48) %_3)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3)
  ret void
}

; <original_norewrite::Dog as original_norewrite::Animal>::kaeps
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN70_$LT$original_norewrite..Dog$u20$as$u20$original_norewrite..Animal$GT$5kaeps17hcc0497b26c88a419E"(ptr noalias nonnull readonly align 1 captures(none) %self) unnamed_addr #0 {
start:
  %_3 = alloca [48 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3)
  store ptr @alloc_ec4fa215300b117d5ab20e2368000be2, ptr %_3, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  store i64 1, ptr %0, align 8
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 32
  store ptr null, ptr %1, align 8
  %2 = getelementptr inbounds nuw i8, ptr %_3, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %2, align 8
  %3 = getelementptr inbounds nuw i8, ptr %_3, i64 24
  store i64 0, ptr %3, align 8
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h1a48aee8b229bf08E(ptr noalias noundef nonnull align 8 captures(address) dereferenceable(48) %_3)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3)
  ret void
}

; Function Attrs: nounwind nonlazybind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #3

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #5

; <std::env::Args as core::iter::traits::iterator::Iterator>::next
; Function Attrs: nonlazybind uwtable
declare void @"_ZN73_$LT$std..env..Args$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h98c081630f6700f9E"(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(address) dereferenceable(24), ptr noalias noundef align 8 dereferenceable(32)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #6

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #5

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #7

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind nonlazybind optsize uwtable
declare void @_ZN4core9panicking16panic_in_cleanup17h88c440a45b19874fE() unnamed_addr #8

; std::env::args
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std3env4args17hdc2bb5ce4515f66cE(ptr dead_on_unwind noalias noundef writable sret([32 x i8]) align 8 captures(address) dereferenceable(32)) unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17h1a48aee8b229bf08E(ptr dead_on_return noalias noundef align 8 captures(address) dereferenceable(48)) unnamed_addr #0

; core::panicking::panic_bounds_check
; Function Attrs: cold minsize noinline noreturn nonlazybind optsize uwtable
declare void @_ZN4core9panicking18panic_bounds_check17h939af04696df73c0E(i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #9

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare noundef i64 @_ZN3std2rt19lang_start_internal17h7d87538fea49e523E(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #10

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking9panic_fmt17h82731289384e3556E(ptr dead_on_return noalias noundef readonly align 8 captures(address) dereferenceable(48), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core6result13unwrap_failed17haff0608dd3f08819E(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind nonlazybind uwtable
declare void @_RNvCscszlnfU7ykQ_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #3

; __rustc::__rust_alloc
; Function Attrs: nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCscszlnfU7ykQ_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #12

; __rustc::__rust_dealloc
; Function Attrs: nounwind nonlazybind allockind("free") uwtable
declare void @_RNvCscszlnfU7ykQ_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #13

; __rustc::__rust_realloc
; Function Attrs: nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCscszlnfU7ykQ_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #14

; core::panicking::assert_failed
; Function Attrs: cold minsize noinline noreturn nonlazybind optsize uwtable
declare void @_ZN4core9panicking13assert_failed17hc0c6a54360149362E(i8 noundef range(i8 0, 3), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr dead_on_return noalias noundef align 8 captures(address) dereferenceable(48), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #9

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn nonlazybind optsize uwtable
declare void @_ZN5alloc7raw_vec12handle_error17h4842bb444d407eecE(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #15

; core::fmt::Formatter::write_str
; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @_ZN4core3fmt9Formatter9write_str17h925affe97b034470E(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::fmt::Formatter::debug_struct_field1_finish
; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @_ZN4core3fmt9Formatter26debug_struct_field1_finish17h51b6627e6baebca0E(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #0

; Function Attrs: nonlazybind
define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #16 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = sext i32 %0 to i64
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i)
  store ptr @_ZN18original_norewrite4main17hb146e61664e51d9bE, ptr %_7.i, align 8
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_ZN3std2rt19lang_start_internal17h7d87538fea49e523E(ptr noundef nonnull align 1 %_7.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.2, i64 noundef %2, ptr noundef %1, i8 noundef 0)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i)
  %3 = trunc i64 %_0.i to i32
  ret i32 %3
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #17

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #18

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: read)
declare ptr @llvm.load.relative.i64(ptr, i64) #19

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #4 = { cold nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #5 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #6 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #7 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #8 = { cold minsize noinline noreturn nounwind nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #9 = { cold minsize noinline noreturn nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #10 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #11 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #12 = { nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCscszlnfU7ykQ_7___rustc19___rust_alloc_zeroed" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #13 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #14 = { nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #15 = { cold minsize noreturn nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #16 = { nonlazybind "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #17 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #18 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #19 = { nocallback nofree nosync nounwind willreturn memory(argmem: read) }
attributes #20 = { nounwind }
attributes #21 = { noreturn }
attributes #22 = { cold }
attributes #23 = { cold noreturn nounwind }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{!"rustc version 1.92.0-dev"}
!4 = !{!5}
!5 = distinct !{!5, !6, !"_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE: %_0"}
!6 = distinct !{!6, !"_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE"}
!7 = !{!8}
!8 = distinct !{!8, !9, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17he6336f1cccca3eb3E: %_0"}
!9 = distinct !{!9, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17he6336f1cccca3eb3E"}
!10 = !{!8, !11, !5, !12}
!11 = distinct !{!11, !9, !"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17he6336f1cccca3eb3E: %iter"}
!12 = distinct !{!12, !6, !"_ZN4core4iter6traits8iterator8Iterator7collect17h8722ddc4fa0f419cE: %self"}
!13 = !{!14, !16}
!14 = distinct !{!14, !15, !"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h3ac762d127b386d9E: %_0"}
!15 = distinct !{!15, !"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h3ac762d127b386d9E"}
!16 = distinct !{!16, !15, !"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h3ac762d127b386d9E: %self"}
!17 = !{!8, !5}
!18 = !{!19}
!19 = distinct !{!19, !20, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hc7d3d6438f0dad78E: %_0"}
!20 = distinct !{!20, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hc7d3d6438f0dad78E"}
!21 = !{!22}
!22 = distinct !{!22, !20, !"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hc7d3d6438f0dad78E: %iterator"}
!23 = !{!24}
!24 = distinct !{!24, !25, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h182e4ffe6b36aca5E: %_0"}
!25 = distinct !{!25, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h182e4ffe6b36aca5E"}
!26 = !{!27}
!27 = distinct !{!27, !25, !"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h182e4ffe6b36aca5E: %iterator"}
!28 = !{!24, !27, !19, !22, !8, !11, !5, !12}
!29 = !{!24, !19, !8, !11, !5, !12}
!30 = !{i64 0, i64 -9223372036854775807}
!31 = !{}
!32 = !{!24, !19, !8, !5}
!33 = !{!27, !22, !11, !12}
!34 = !{!35}
!35 = distinct !{!35, !36, !"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd9964c0c546f889dE: %_1"}
!36 = distinct !{!36, !"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd9964c0c546f889dE"}
!37 = !{!38}
!38 = distinct !{!38, !39, !"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17ha76ffc5fa2a0256bE: %_1"}
!39 = distinct !{!39, !"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17ha76ffc5fa2a0256bE"}
!40 = !{!41}
!41 = distinct !{!41, !42, !"_ZN4core3ptr49drop_in_place$LT$std..sys..args..common..Args$GT$17h20e3fbaa761c8d3aE: %_1"}
!42 = distinct !{!42, !"_ZN4core3ptr49drop_in_place$LT$std..sys..args..common..Args$GT$17h20e3fbaa761c8d3aE"}
!43 = !{!44}
!44 = distinct !{!44, !45, !"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17haafd9122f25a0f38E: %_1"}
!45 = distinct !{!45, !"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17haafd9122f25a0f38E"}
!46 = !{!47}
!47 = distinct !{!47, !48, !"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h00ed8e6ff16a925aE: %self"}
!48 = distinct !{!48, !"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h00ed8e6ff16a925aE"}
!49 = !{!47, !44, !41, !38, !35, !27, !22}
!50 = !{!51}
!51 = distinct !{!51, !52, !"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17hd5d6d339c550276eE: %_1.0"}
!52 = distinct !{!52, !"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17hd5d6d339c550276eE"}
!53 = !{!47, !44, !41, !38, !35, !24, !19, !8, !11, !5, !12}
!54 = !{!51, !47, !44, !41, !38, !35, !24, !19, !8, !11, !5, !12}
!55 = !{!27, !22}
!56 = !{!"branch_weights", i32 2002, i32 2000}
!57 = !{!58, !24, !19, !8, !11, !5, !12}
!58 = distinct !{!58, !59, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17hdef96cb08dece651E: %_0"}
!59 = distinct !{!59, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17hdef96cb08dece651E"}
!60 = !{!61}
!61 = distinct !{!61, !62, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17h6154859cc1e427e0E: %self"}
!62 = distinct !{!62, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17h6154859cc1e427e0E"}
!63 = !{!64}
!64 = distinct !{!64, !62, !"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17h6154859cc1e427e0E: %iter"}
!65 = !{!66}
!66 = distinct !{!66, !67, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17h93cfe20430857deeE: %self"}
!67 = distinct !{!67, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17h93cfe20430857deeE"}
!68 = !{!69}
!69 = distinct !{!69, !67, !"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17h93cfe20430857deeE: %iterator"}
!70 = !{!66, !69, !61, !64, !24, !27, !19, !22, !8, !11, !5, !12}
!71 = !{!66, !61, !24, !19, !8, !11, !5, !12}
!72 = !{i64 0, i64 -9223372036854775808}
!73 = !{!66, !61}
!74 = !{!69, !64, !24, !27, !19, !22, !8, !11, !5, !12}
!75 = !{!76}
!76 = distinct !{!76, !77, !"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd9964c0c546f889dE: %_1"}
!77 = distinct !{!77, !"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd9964c0c546f889dE"}
!78 = !{!79}
!79 = distinct !{!79, !80, !"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17ha76ffc5fa2a0256bE: %_1"}
!80 = distinct !{!80, !"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17ha76ffc5fa2a0256bE"}
!81 = !{!82}
!82 = distinct !{!82, !83, !"_ZN4core3ptr49drop_in_place$LT$std..sys..args..common..Args$GT$17h20e3fbaa761c8d3aE: %_1"}
!83 = distinct !{!83, !"_ZN4core3ptr49drop_in_place$LT$std..sys..args..common..Args$GT$17h20e3fbaa761c8d3aE"}
!84 = !{!85}
!85 = distinct !{!85, !86, !"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17haafd9122f25a0f38E: %_1"}
!86 = distinct !{!86, !"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17haafd9122f25a0f38E"}
!87 = !{!88}
!88 = distinct !{!88, !89, !"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h00ed8e6ff16a925aE: %self"}
!89 = distinct !{!89, !"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h00ed8e6ff16a925aE"}
!90 = !{!88, !85, !82, !79, !76, !69, !64}
!91 = !{!66, !61, !24, !27, !19, !22, !8, !11, !5, !12}
!92 = !{!93}
!93 = distinct !{!93, !94, !"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17hd5d6d339c550276eE: %_1.0"}
!94 = distinct !{!94, !"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17hd5d6d339c550276eE"}
!95 = !{!88, !85, !82, !79, !76, !66, !61, !24, !19, !8, !11, !5, !12}
!96 = !{!93, !88, !85, !82, !79, !76, !66, !61, !24, !19, !8, !11, !5, !12}
!97 = !{!69, !64}
!98 = !{!99}
!99 = distinct !{!99, !100, !"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd9964c0c546f889dE: %_1"}
!100 = distinct !{!100, !"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd9964c0c546f889dE"}
!101 = !{!102}
!102 = distinct !{!102, !103, !"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17ha76ffc5fa2a0256bE: %_1"}
!103 = distinct !{!103, !"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17ha76ffc5fa2a0256bE"}
!104 = !{!105}
!105 = distinct !{!105, !106, !"_ZN4core3ptr49drop_in_place$LT$std..sys..args..common..Args$GT$17h20e3fbaa761c8d3aE: %_1"}
!106 = distinct !{!106, !"_ZN4core3ptr49drop_in_place$LT$std..sys..args..common..Args$GT$17h20e3fbaa761c8d3aE"}
!107 = !{!108}
!108 = distinct !{!108, !109, !"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17haafd9122f25a0f38E: %_1"}
!109 = distinct !{!109, !"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17haafd9122f25a0f38E"}
!110 = !{!111}
!111 = distinct !{!111, !112, !"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h00ed8e6ff16a925aE: %self"}
!112 = distinct !{!112, !"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h00ed8e6ff16a925aE"}
!113 = !{!111, !108, !105, !102, !99, !22}
!114 = !{!115}
!115 = distinct !{!115, !116, !"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17hd5d6d339c550276eE: %_1.0"}
!116 = distinct !{!116, !"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17hd5d6d339c550276eE"}
!117 = !{!111, !108, !105, !102, !99, !24, !19, !8, !11, !5, !12}
!118 = !{!115, !111, !108, !105, !102, !99, !24, !19, !8, !11, !5, !12}
!119 = !{!120}
!120 = distinct !{!120, !121, !"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hd1259156c9080265E: %_1"}
!121 = distinct !{!121, !"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hd1259156c9080265E"}
!122 = !{!123}
!123 = distinct !{!123, !124, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17hde3e17e08480ca65E: %_1.0"}
!124 = distinct !{!124, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17hde3e17e08480ca65E"}
!125 = !{!123, !120}
!126 = !{!127}
!127 = distinct !{!127, !128, !"_ZN4core3num23_$LT$impl$u20$usize$GT$16from_ascii_radix17h014a6b40553bd955E: argument 1"}
!128 = distinct !{!128, !"_ZN4core3num23_$LT$impl$u20$usize$GT$16from_ascii_radix17h014a6b40553bd955E"}
!129 = !{!130}
!130 = distinct !{!130, !128, !"_ZN4core3num23_$LT$impl$u20$usize$GT$16from_ascii_radix17h014a6b40553bd955E: %_0"}
!131 = !{!132}
!132 = distinct !{!132, !133, !"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h2c35d1d4e19ef5bfE: %self"}
!133 = distinct !{!133, !"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h2c35d1d4e19ef5bfE"}
!134 = !{i64 17179508410747017}
!135 = !{i64 1}
!136 = !{i8 0, i8 5}
!137 = !{!138}
!138 = distinct !{!138, !139, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64790d7c15d240fbE: %_1"}
!139 = distinct !{!139, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64790d7c15d240fbE"}
!140 = !{!141}
!141 = distinct !{!141, !142, !"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17ha76ffc5fa2a0256bE: %_1"}
!142 = distinct !{!142, !"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17ha76ffc5fa2a0256bE"}
!143 = !{!144}
!144 = distinct !{!144, !145, !"_ZN4core3ptr49drop_in_place$LT$std..sys..args..common..Args$GT$17h20e3fbaa761c8d3aE: %_1"}
!145 = distinct !{!145, !"_ZN4core3ptr49drop_in_place$LT$std..sys..args..common..Args$GT$17h20e3fbaa761c8d3aE"}
!146 = !{!147}
!147 = distinct !{!147, !148, !"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17haafd9122f25a0f38E: %_1"}
!148 = distinct !{!148, !"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17haafd9122f25a0f38E"}
!149 = !{!150}
!150 = distinct !{!150, !151, !"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h00ed8e6ff16a925aE: %self"}
!151 = distinct !{!151, !"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h00ed8e6ff16a925aE"}
!152 = !{!150, !147, !144, !141}
!153 = !{!154}
!154 = distinct !{!154, !155, !"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17hd5d6d339c550276eE: %_1.0"}
!155 = distinct !{!155, !"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17hd5d6d339c550276eE"}
!156 = !{!154, !150, !147, !144, !141}
!157 = !{!158}
!158 = distinct !{!158, !159, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17hde3e17e08480ca65E: %_1.0"}
!159 = distinct !{!159, !"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17hde3e17e08480ca65E"}
!160 = !{i64 1, i64 0}
!161 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!162 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!163 = !{!164}
!164 = distinct !{!164, !165, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14grow_amortized17h4143c34a055c6fe5E: %self"}
!165 = distinct !{!165, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14grow_amortized17h4143c34a055c6fe5E"}
!166 = !{!167}
!167 = distinct !{!167, !168, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14current_memory17hf933bf23e667f361E: %_0"}
!168 = distinct !{!168, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14current_memory17hf933bf23e667f361E"}
!169 = !{i64 0, i64 2}
