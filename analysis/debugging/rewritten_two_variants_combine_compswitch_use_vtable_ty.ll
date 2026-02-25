; ModuleID = 'two_variants.e9f9aea9fd163765-cgu.0'
source_filename = "two_variants.e9f9aea9fd163765-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"alloc::string::String" = type { %"alloc::vec::Vec<u8>" }
%"alloc::vec::Vec<u8>" = type { %"alloc::raw_vec::RawVec<u8>", i64 }
%"alloc::raw_vec::RawVec<u8>" = type { %"alloc::raw_vec::RawVecInner", %"core::marker::PhantomData<u8>" }
%"alloc::raw_vec::RawVecInner" = type { i64, ptr, %"alloc::alloc::Global" }
%"alloc::alloc::Global" = type {}
%"core::marker::PhantomData<u8>" = type {}
%"std::ffi::os_str::OsString" = type { %"std::sys::os_str::bytes::Buf" }
%"std::sys::os_str::bytes::Buf" = type { %"alloc::vec::Vec<u8>" }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0INtNtNtCshhmRwEtsTQ2_4core3ops8function6FnOnceuE9call_once6vtableCsk5rB8Fy7VC3_12two_variants, ptr @_RNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0Csk5rB8Fy7VC3_12two_variants, ptr @_RNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0Csk5rB8Fy7VC3_12two_variants }>, align 8
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXCsk5rB8Fy7VC3_12two_variantsNtB2_3CatNtB2_6Animal5speak, ptr @_RNvXCsk5rB8Fy7VC3_12two_variantsNtB2_3CatNtB2_6Animal4walk }>, align 8
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXs_Csk5rB8Fy7VC3_12two_variantsNtB4_3DogNtB4_6Animal5speak, ptr @_RNvXs_Csk5rB8Fy7VC3_12two_variantsNtB4_3DogNtB4_6Animal4walk }>, align 8
@alloc_c85489ac7d65f986716cfdb1dc3aed62 = private unnamed_addr constant [39 x i8] c"Pass in a number and see what happens!\0A", align 1
@alloc_40721e4b46ef799d5fa7860d1c113640 = private unnamed_addr constant [25 x i8] c"examples/two_variants.rs\00", align 1
@alloc_7fa232d3fec0961e04b35ddd656c9b32 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_40721e4b46ef799d5fa7860d1c113640, [16 x i8] c"\18\00\00\00\00\00\00\005\00\00\00\19\00\00\00" }>, align 8
@alloc_9e4a0be258b7ac4e4e85080401a435ca = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_40721e4b46ef799d5fa7860d1c113640, [16 x i8] c"\18\00\00\00\00\00\00\005\00\00\00%\00\00\00" }>, align 8
@alloc_a8c4773e19f17bafbfce7be36b8ae92b = private unnamed_addr constant [8 x i8] c"\03x: \C0\01\0A\00", align 1
@alloc_b8a8ade38cd9a3b8b2bfcff364622fa1 = private unnamed_addr constant [20 x i8] c"\0Fanimal_vtable: \C0\01\0A\00", align 1
@alloc_18a9af093ad63d2a518738c428eb578e = private unnamed_addr constant [17 x i8] c"\0Ccat_vtable: \C0\01\0A\00", align 1
@alloc_a16f3e37bf6cf20be080dc24dc11b38e = private unnamed_addr constant [9 x i8] c"\04eq? \C0\01\0A\00", align 1
@alloc_0ea66dfa401483f8b62b25c396fda960 = private unnamed_addr constant [10 x i8] c"\05res: \C0\01\0A\00", align 1
@vtable.3 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXsc_NtNtCshhmRwEtsTQ2_4core3num5errorNtB5_13ParseIntErrorNtNtB9_3fmt5Debug3fmt }>, align 8
@alloc_00ae4b301f7fab8ac9617c03fcbd7274 = private unnamed_addr constant [43 x i8] c"called `Result::unwrap()` on an `Err` value", align 1
@alloc_9593adc5b471736a257f2c43c3399b67 = private unnamed_addr constant [11 x i8] c"DynMetadata", align 1
@vtable.4 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsp_NtCshhmRwEtsTQ2_4core3fmtPNtNtNtB7_3ptr8metadata6VTableNtB5_5Debug3fmtCsk5rB8Fy7VC3_12two_variants }>, align 8
@vtable.5 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCshhmRwEtsTQ2_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCsk5rB8Fy7VC3_12two_variants }>, align 8
@alloc_f62df14955f7d78bca139b0a7668683d = private unnamed_addr constant [13 x i8] c"ParseIntError", align 1
@alloc_a5d866b1768ad3f826bccdb004a1a8ae = private unnamed_addr constant [4 x i8] c"kind", align 1
@alloc_59ba7b9f7211443cd55a366616eef46a = private unnamed_addr constant [5 x i8] c"Empty", align 1
@alloc_00315c78e51d29fe6b3102a4c1ecf6ef = private unnamed_addr constant [12 x i8] c"InvalidDigit", align 1
@alloc_bd3a3f3879e0d5f64554753e977f58d4 = private unnamed_addr constant [11 x i8] c"PosOverflow", align 1
@alloc_0964bb2a4870637395c77a018495bd5c = private unnamed_addr constant [11 x i8] c"NegOverflow", align 1
@alloc_6566120a3a17f930e960a0863fcbd591 = private unnamed_addr constant [4 x i8] c"Zero", align 1
@switch.table._RNvXs1g_NtCshhmRwEtsTQ2_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCsk5rB8Fy7VC3_12two_variants = private unnamed_addr constant [5 x i64] [i64 5, i64 12, i64 11, i64 11, i64 4], align 8
@switch.table._RNvXs1g_NtCshhmRwEtsTQ2_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCsk5rB8Fy7VC3_12two_variants.30.rel = private unnamed_addr constant [5 x i32] [i32 trunc (i64 sub (i64 ptrtoint (ptr @alloc_59ba7b9f7211443cd55a366616eef46a to i64), i64 ptrtoint (ptr @switch.table._RNvXs1g_NtCshhmRwEtsTQ2_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCsk5rB8Fy7VC3_12two_variants.30.rel to i64)) to i32), i32 trunc (i64 sub (i64 ptrtoint (ptr @alloc_00315c78e51d29fe6b3102a4c1ecf6ef to i64), i64 ptrtoint (ptr @switch.table._RNvXs1g_NtCshhmRwEtsTQ2_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCsk5rB8Fy7VC3_12two_variants.30.rel to i64)) to i32), i32 trunc (i64 sub (i64 ptrtoint (ptr @alloc_bd3a3f3879e0d5f64554753e977f58d4 to i64), i64 ptrtoint (ptr @switch.table._RNvXs1g_NtCshhmRwEtsTQ2_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCsk5rB8Fy7VC3_12two_variants.30.rel to i64)) to i32), i32 trunc (i64 sub (i64 ptrtoint (ptr @alloc_0964bb2a4870637395c77a018495bd5c to i64), i64 ptrtoint (ptr @switch.table._RNvXs1g_NtCshhmRwEtsTQ2_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCsk5rB8Fy7VC3_12two_variants.30.rel to i64)) to i32), i32 trunc (i64 sub (i64 ptrtoint (ptr @alloc_6566120a3a17f930e960a0863fcbd591 to i64), i64 ptrtoint (ptr @switch.table._RNvXs1g_NtCshhmRwEtsTQ2_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCsk5rB8Fy7VC3_12two_variants.30.rel to i64)) to i32)], align 4

; std::rt::lang_start::<()>
; Function Attrs: nonlazybind uwtable
define hidden noundef i64 @_RINvNtCsefmIBSMl6ne_3std2rt10lang_startuECsk5rB8Fy7VC3_12two_variants(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7)
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_RNvNtCsefmIBSMl6ne_3std2rt19lang_start_internal(ptr noundef nonnull align 1 %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7)
  ret i64 %_0
}

; core::ptr::drop_in_place::<alloc::vec::Vec<alloc::string::String>>
; Function Attrs: nounwind nonlazybind uwtable
define internal fastcc void @_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc3vec3VecNtNtBL_6string6StringEECsk5rB8Fy7VC3_12two_variants(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val = load ptr, ptr %0, align 8, !nonnull !4, !noundef !4
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i64, ptr %1, align 8, !noundef !4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5)
  %_710.i.i = icmp eq i64 %_1.val1, 0
  br i1 %_710.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i
  %_3.sroa.0.011.i.i = phi i64 [ %2, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i ], [ 0, %start ]
  %_6.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %_1.val, i64 %_3.sroa.0.011.i.i
  %2 = add nuw i64 %_3.sroa.0.011.i.i, 1
  %_6.val.i.i = load i64, ptr %_6.i.i, align 8, !alias.scope !5
  %3 = icmp eq i64 %_6.val.i.i, 0
  br i1 %3, label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i, label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %bb5.i.i
  %4 = getelementptr i8, ptr %_6.i.i, i64 8
  %_6.val7.i.i = load ptr, ptr %4, align 8, !alias.scope !5, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i, i64 noundef %_6.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !5
  br label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i

_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i: ; preds = %bb2.i.i.i4.i.i.i.i, %bb5.i.i
  %_7.i.i = icmp eq i64 %2, %_1.val1
  br i1 %_7.i.i, label %bb4, label %bb5.i.i

bb4:                                              ; preds = %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i, %start
  %_1.val4 = load i64, ptr %_1, align 8, !range !8, !noundef !4
  %5 = icmp eq i64 %_1.val4, 0
  br i1 %5, label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc7raw_vec6RawVecNtNtBL_6string6StringEECsk5rB8Fy7VC3_12two_variants.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = mul nuw i64 %_1.val4, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #22
  br label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc7raw_vec6RawVecNtNtBL_6string6StringEECsk5rB8Fy7VC3_12two_variants.exit8

_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc7raw_vec6RawVecNtNtBL_6string6StringEECsk5rB8Fy7VC3_12two_variants.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void
}

; core::ptr::drop_in_place::<alloc::boxed::Box<dyn two_variants::Animal>>
; Function Attrs: nonlazybind uwtable
define internal fastcc void @_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc5boxed3BoxDNtCsk5rB8Fy7VC3_12two_variants6AnimalEL_EEB1i_(ptr %_1.0.val, ptr readonly captures(address_is_null) %_1.8.val) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %0)
  %1 = load ptr, ptr %_1.8.val, align 8, !invariant.load !4
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
  %5 = load i64, ptr %4, align 8, !range !8, !invariant.load !4
  %6 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 16
  %7 = load i64, ptr %6, align 8, !range !9, !invariant.load !4
  %8 = add i64 %7, -1
  %9 = icmp sgt i64 %8, -1
  tail call void @llvm.assume(i1 %9)
  %10 = icmp eq i64 %5, 0
  br i1 %10, label %_RNvXs8_NtCsavYRE6QVrtQ_5alloc5boxedINtB5_3BoxDNtCsk5rB8Fy7VC3_12two_variants6AnimalEL_ENtNtNtCshhmRwEtsTQ2_4core3ops4drop4Drop4dropBK_.exit, label %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i

_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i: ; preds = %bb3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) %7) #22
  br label %_RNvXs8_NtCsavYRE6QVrtQ_5alloc5boxedINtB5_3BoxDNtCsk5rB8Fy7VC3_12two_variants6AnimalEL_ENtNtNtCshhmRwEtsTQ2_4core3ops4drop4Drop4dropBK_.exit

_RNvXs8_NtCsavYRE6QVrtQ_5alloc5boxedINtB5_3BoxDNtCsk5rB8Fy7VC3_12two_variants6AnimalEL_ENtNtNtCshhmRwEtsTQ2_4core3ops4drop4Drop4dropBK_.exit: ; preds = %bb3, %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i
  ret void

cleanup:                                          ; preds = %is_not_null
  %11 = landingpad { ptr, i32 }
          cleanup
  %12 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 8
  %13 = load i64, ptr %12, align 8, !range !8, !invariant.load !4
  %14 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 16
  %15 = load i64, ptr %14, align 8, !range !9, !invariant.load !4
  %16 = add i64 %15, -1
  %17 = icmp sgt i64 %16, -1
  tail call void @llvm.assume(i1 %17)
  %18 = icmp eq i64 %13, 0
  br i1 %18, label %bb1, label %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i4

_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i4: ; preds = %cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %13, i64 noundef range(i64 1, -9223372036854775807) %15) #22
  br label %bb1

bb1:                                              ; preds = %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i4, %cleanup
  resume { ptr, i32 } %11
}

; core::ptr::drop_in_place::<std::env::Args>
; Function Attrs: nounwind nonlazybind uwtable
define internal fastcc void @_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env4ArgsECsk5rB8Fy7VC3_12two_variants(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %self.val.i.i.i.i = load ptr, ptr %0, align 8, !alias.scope !22, !nonnull !4, !noundef !4
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %self.val1.i.i.i.i = load ptr, ptr %1, align 8, !alias.scope !22, !nonnull !4, !noundef !4
  %2 = ptrtoint ptr %self.val1.i.i.i.i to i64
  %3 = ptrtoint ptr %self.val.i.i.i.i to i64
  %4 = sub nuw i64 %2, %3
  %5 = udiv exact i64 %4, 24
  tail call void @llvm.experimental.noalias.scope.decl(metadata !23)
  %_710.i.i.i.i.i = icmp eq ptr %self.val1.i.i.i.i, %self.val.i.i.i.i
  br i1 %_710.i.i.i.i.i, label %bb2.i.i.i.i, label %bb5.i.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %start, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i
  %_3.sroa.0.011.i.i.i.i.i = phi i64 [ %6, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i ], [ 0, %start ]
  %_6.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self.val.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i
  %6 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i, align 8, !alias.scope !23, !noalias !22
  %7 = icmp eq i64 %_6.val.i.i.i.i.i, 0
  br i1 %7, label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i:                       ; preds = %bb5.i.i.i.i.i
  %8 = getelementptr i8, ptr %_6.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i = load ptr, ptr %8, align 8, !alias.scope !23, !noalias !22, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !26
  br label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i

_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i: ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i
  %_7.i.i.i.i.i = icmp eq i64 %6, %5
  br i1 %_7.i.i.i.i.i, label %bb2.i.i.i.i, label %bb5.i.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i, %start
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %capacity1.i.i3.i.i.i.i = load i64, ptr %9, align 8, !alias.scope !22, !noundef !4
  %10 = icmp eq i64 %capacity1.i.i3.i.i.i.i, 0
  br i1 %10, label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env6ArgsOsECsk5rB8Fy7VC3_12two_variants.exit, label %bb2.i.i.i.i.i4.i.i.i.i

bb2.i.i.i.i.i4.i.i.i.i:                           ; preds = %bb2.i.i.i.i
  %ptr.i.i5.i.i.i.i = load ptr, ptr %_1, align 8, !alias.scope !22, !nonnull !4, !noundef !4
  %alloc_size.i.i.i.i.i.i6.i.i.i.i = mul nuw i64 %capacity1.i.i3.i.i.i.i, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i5.i.i.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22, !noalias !22
  br label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env6ArgsOsECsk5rB8Fy7VC3_12two_variants.exit

_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env6ArgsOsECsk5rB8Fy7VC3_12two_variants.exit: ; preds = %bb2.i.i.i.i, %bb2.i.i.i.i.i4.i.i.i.i
  ret void
}

; std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
; Function Attrs: noinline nonlazybind uwtable
define internal fastcc void @_RINvNtNtCsefmIBSMl6ne_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsk5rB8Fy7VC3_12two_variants(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #2 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #22, !srcloc !27
  ret void
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold nonlazybind uwtable
define internal fastcc void @_RINvNvMs2_NtCsavYRE6QVrtQ_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsk5rB8Fy7VC3_12two_variants(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef range(i64 1, 0) %additional) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28)
  %_25.0.i = add i64 %additional, %len
  %_25.1.i = icmp ult i64 %_25.0.i, %len
  br i1 %_25.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !8, !alias.scope !28, !noundef !4
  %v16.i = shl nuw i64 %self5.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_25.0.i, i64 range(i64 0, -1) %v16.i)
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !28
  %0 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %0, align 8, !alias.scope !28
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsavYRE6QVrtQ_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsk5rB8Fy7VC3_12two_variants(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i)
  %_35.i = load i64, ptr %self3.i, align 8, !range !31, !noalias !28, !noundef !4
  %1 = trunc nuw i64 %_35.i to i1
  %2 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %1, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %2, align 8, !range !32, !noalias !28, !noundef !4
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %3, align 8, !noalias !28
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !28
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsavYRE6QVrtQ_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #23
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %2, align 8, !noalias !28, !nonnull !4, !noundef !4
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !28
  store ptr %v.0.i, ptr %0, align 8, !alias.scope !28
  %4 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %4)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !28
  ret void
}

; std::rt::lang_start::<()>::{closure#0}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @_RNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0Csk5rB8Fy7VC3_12two_variants(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #4 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCsefmIBSMl6ne_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsk5rB8Fy7VC3_12two_variants(ptr noundef nonnull %_4) #24
  ret i32 0
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @_RNSNvYNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0INtNtNtCshhmRwEtsTQ2_4core3ops8function6FnOnceuE9call_once6vtableCsk5rB8Fy7VC3_12two_variants(ptr noundef readonly captures(none) %_1) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCsefmIBSMl6ne_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsk5rB8Fy7VC3_12two_variants(ptr noundef nonnull readonly %0) #24, !noalias !33
  ret i32 0
}

; two_variants::main
; Function Attrs: nonlazybind uwtable
define hidden void @_RNvCsk5rB8Fy7VC3_12two_variants4main() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3.i.i.i = alloca [24 x i8], align 8
  %_19.i = alloca [32 x i8], align 8
  %_3.i = alloca [24 x i8], align 8
  %vector.i = alloca [24 x i8], align 8
  %e.i = alloca [1 x i8], align 1
  %args5 = alloca [16 x i8], align 8
  %args4 = alloca [16 x i8], align 8
  %_37 = alloca [1 x i8], align 1
  %args3 = alloca [16 x i8], align 8
  %args2 = alloca [16 x i8], align 8
  %args1 = alloca [16 x i8], align 8
  %res = alloca [8 x i8], align 8
  %cat_vtable = alloca [8 x i8], align 8
  %animal_vtable = alloca [8 x i8], align 8
  %x = alloca [8 x i8], align 8
  %_2 = alloca [32 x i8], align 8
  %args = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %args)
; call std::env::args
  call void @_RNvNtCsefmIBSMl6ne_3std3env4args(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_2)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i), !noalias !41
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3.i), !noalias !41
; invoke <std::env::Args as core::iter::traits::iterator::Iterator>::next
  invoke void @_RNvXsa_NtCsefmIBSMl6ne_3std3envNtB5_4ArgsNtNtNtNtCshhmRwEtsTQ2_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_3.i, ptr noalias noundef nonnull align 8 dereferenceable(32) %_2)
          to label %bb1.i unwind label %cleanup.i, !noalias !36

cleanup.i:                                        ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
  br label %bb10.i

bb1.i:                                            ; preds = %start
  %1 = load i64, ptr %_3.i, align 8, !range !32, !noalias !41, !noundef !4
  %.not.i = icmp eq i64 %1, -9223372036854775808
  br i1 %.not.i, label %bb12.i, label %bb14.i

bb12.i:                                           ; preds = %bb1.i
  store i64 0, ptr %args, align 8, !alias.scope !36, !noalias !39
  %2 = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr inttoptr (i64 8 to ptr), ptr %2, align 8, !alias.scope !36, !noalias !39
  %3 = getelementptr inbounds nuw i8, ptr %args, i64 16
  store i64 0, ptr %3, align 8, !alias.scope !36, !noalias !39
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i), !noalias !41
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i), !noalias !41
  call void @llvm.experimental.noalias.scope.decl(metadata !42)
  call void @llvm.experimental.noalias.scope.decl(metadata !45)
  call void @llvm.experimental.noalias.scope.decl(metadata !48)
  call void @llvm.experimental.noalias.scope.decl(metadata !51)
  call void @llvm.experimental.noalias.scope.decl(metadata !54)
  %4 = getelementptr inbounds nuw i8, ptr %_2, i64 8
  %self.val.i.i.i.i.i.i = load ptr, ptr %4, align 8, !alias.scope !57, !noalias !36, !nonnull !4, !noundef !4
  %5 = getelementptr inbounds nuw i8, ptr %_2, i64 24
  %self.val1.i.i.i.i.i.i = load ptr, ptr %5, align 8, !alias.scope !57, !noalias !36, !nonnull !4, !noundef !4
  %6 = ptrtoint ptr %self.val1.i.i.i.i.i.i to i64
  %7 = ptrtoint ptr %self.val.i.i.i.i.i.i to i64
  %8 = sub nuw i64 %6, %7
  %9 = udiv exact i64 %8, 24
  call void @llvm.experimental.noalias.scope.decl(metadata !58)
  %_710.i.i.i.i.i.i.i = icmp eq ptr %self.val1.i.i.i.i.i.i, %self.val.i.i.i.i.i.i
  br i1 %_710.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i:                                ; preds = %bb12.i, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i
  %_3.sroa.0.011.i.i.i.i.i.i.i = phi i64 [ %10, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i ], [ 0, %bb12.i ]
  %_6.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self.val.i.i.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i.i.i
  %10 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i.i.i, align 8, !alias.scope !58, !noalias !61
  %11 = icmp eq i64 %_6.val.i.i.i.i.i.i.i, 0
  br i1 %11, label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i:                   ; preds = %bb5.i.i.i.i.i.i.i
  %12 = getelementptr i8, ptr %_6.i.i.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i.i.i = load ptr, ptr %12, align 8, !alias.scope !58, !noalias !61, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !62
  br label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i

_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i: ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = icmp eq i64 %10, %9
  br i1 %_7.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i:                                  ; preds = %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i, %bb12.i
  %13 = getelementptr inbounds nuw i8, ptr %_2, i64 16
  %capacity1.i.i3.i.i.i.i.i.i = load i64, ptr %13, align 8, !alias.scope !57, !noalias !36, !noundef !4
  %14 = icmp eq i64 %capacity1.i.i3.i.i.i.i.i.i, 0
  br i1 %14, label %panic, label %bb2.i.i.i.i.i4.i.i.i.i.i.i

bb2.i.i.i.i.i4.i.i.i.i.i.i:                       ; preds = %bb2.i.i.i.i.i.i
  %ptr.i.i5.i.i.i.i.i.i = load ptr, ptr %_2, align 8, !alias.scope !57, !noalias !36, !nonnull !4, !noundef !4
  %alloc_size.i.i.i.i.i.i6.i.i.i.i.i.i = mul nuw i64 %capacity1.i.i3.i.i.i.i.i.i, 24
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i5.i.i.i.i.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22, !noalias !61
  br label %panic

cleanup2.i:                                       ; preds = %bb3.i.i
  %15 = landingpad { ptr, i32 }
          cleanup
  %16 = icmp eq i64 %1, 0
  br i1 %16, label %bb10.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup2.i
  %17 = icmp ne ptr %element.sroa.5.0.copyload.i, null
  call void @llvm.assume(i1 %17)
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %element.sroa.5.0.copyload.i, i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !36
  br label %bb10.i

bb14.i:                                           ; preds = %bb1.i
  %element.sroa.5.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 8
  %element.sroa.5.0.copyload.i = load ptr, ptr %element.sroa.5.0._3.sroa_idx.i, align 8, !noalias !41
  %element.sroa.6.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 16
  %element.sroa.6.0.copyload.i = load i64, ptr %element.sroa.6.0._3.sroa_idx.i, align 8, !noalias !41
  %18 = getelementptr inbounds nuw i8, ptr %_2, i64 8
  %iterator.val.i = load ptr, ptr %18, align 8, !alias.scope !39, !noalias !36, !nonnull !4, !noundef !4
  %19 = getelementptr inbounds nuw i8, ptr %_2, i64 24
  %iterator.val6.i = load ptr, ptr %19, align 8, !alias.scope !39, !noalias !36, !nonnull !4, !noundef !4
  %20 = ptrtoint ptr %iterator.val6.i to i64
  %21 = ptrtoint ptr %iterator.val.i to i64
  %22 = sub nuw i64 %20, %21
  %23 = udiv exact i64 %22, 24
  %24 = call i64 @llvm.umax.i64(i64 %23, i64 3)
  %_0.sroa.0.0.i.i = add nuw nsw i64 %24, 1
  %_20.0.i.i.i.i = mul i64 %_0.sroa.0.0.i.i, 24
  %or.cond.i.i.i.i = icmp ugt i64 %22, 9223372036854775776
  br i1 %or.cond.i.i.i.i, label %bb3.i.i, label %bb17.i.i.i, !prof !63

bb17.i.i.i:                                       ; preds = %bb14.i
  %25 = icmp eq i64 %_20.0.i.i.i.i, 0
  br i1 %25, label %bb15.i, label %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator8allocate.exit.i.i.i

_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator8allocate.exit.i.i.i: ; preds = %bb17.i.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #22, !noalias !64
; call __rustc::__rust_alloc
  %26 = call noundef align 8 ptr @_RNvCsl5Zt2FtcpFZ_7___rustc12___rust_alloc(i64 noundef %_20.0.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22, !noalias !64
  %27 = icmp eq ptr %26, null
  br i1 %27, label %bb3.i.i, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator8allocate.exit.i.i.i
  %28 = ptrtoint ptr %26 to i64
  br label %bb15.i

bb3.i.i:                                          ; preds = %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator8allocate.exit.i.i.i, %bb14.i
  %_4.sroa.4.0.ph.i.i = phi i64 [ 8, %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator8allocate.exit.i.i.i ], [ 0, %bb14.i ]
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsavYRE6QVrtQ_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i.i, i64 %_20.0.i.i.i.i) #23
          to label %.noexc.i unwind label %cleanup2.i, !noalias !36

.noexc.i:                                         ; preds = %bb3.i.i
  unreachable

bb15.i:                                           ; preds = %bb10.i.i.i, %bb17.i.i.i
  %_4.sroa.4.0.i.i = phi i64 [ %_0.sroa.0.0.i.i, %bb10.i.i.i ], [ 0, %bb17.i.i.i ]
  %_4.sroa.10.0.i.i = phi i64 [ %28, %bb10.i.i.i ], [ 8, %bb17.i.i.i ]
  %29 = inttoptr i64 %_4.sroa.10.0.i.i to ptr
  %_7.i.i = icmp samesign ult i64 %24, %_4.sroa.4.0.i.i
  call void @llvm.assume(i1 %_7.i.i)
  store i64 %1, ptr %29, align 8, !noalias !36
  %src.sroa.4.0._28.1.sroa_idx.i = getelementptr inbounds nuw i8, ptr %29, i64 8
  store ptr %element.sroa.5.0.copyload.i, ptr %src.sroa.4.0._28.1.sroa_idx.i, align 8, !noalias !36
  %src.sroa.5.0._28.1.sroa_idx.i = getelementptr inbounds nuw i8, ptr %29, i64 16
  store i64 %element.sroa.6.0.copyload.i, ptr %src.sroa.5.0._28.1.sroa_idx.i, align 8, !noalias !36
  store i64 %_4.sroa.4.0.i.i, ptr %vector.i, align 8, !noalias !41
  %vector1.sroa.4.0.vector.sroa_idx.i = getelementptr inbounds nuw i8, ptr %vector.i, i64 8
  store ptr %29, ptr %vector1.sroa.4.0.vector.sroa_idx.i, align 8, !noalias !41
  %vector1.sroa.6.0.vector.sroa_idx.i = getelementptr inbounds nuw i8, ptr %vector.i, i64 16
  store i64 1, ptr %vector1.sroa.6.0.vector.sroa_idx.i, align 8, !noalias !41
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i), !noalias !41
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_19.i), !noalias !41
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_19.i, ptr noundef nonnull align 8 dereferenceable(32) %_2, i64 32, i1 false), !noalias !36
  call void @llvm.experimental.noalias.scope.decl(metadata !67)
  call void @llvm.experimental.noalias.scope.decl(metadata !70)
  call void @llvm.experimental.noalias.scope.decl(metadata !72)
  call void @llvm.experimental.noalias.scope.decl(metadata !75)
  %element.sroa.5.0._3.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i, i64 8
  %element.sroa.6.0._3.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i, i64 16
  %30 = getelementptr inbounds nuw i8, ptr %_19.i, i64 24
  %31 = getelementptr inbounds nuw i8, ptr %_19.i, i64 8
  br label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb8.i.i.i, %bb15.i
  %_23.i.i18.i = phi ptr [ %_23.i.i.i, %bb8.i.i.i ], [ %29, %bb15.i ]
  %len.i.i.i = phi i64 [ %new_len.i.i.i, %bb8.i.i.i ], [ 1, %bb15.i ]
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3.i.i.i), !noalias !77
; invoke <std::env::Args as core::iter::traits::iterator::Iterator>::next
  invoke void @_RNvXsa_NtCsefmIBSMl6ne_3std3envNtB5_4ArgsNtNtNtNtCshhmRwEtsTQ2_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_3.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(32) %_19.i)
          to label %bb2.i.i.i unwind label %cleanup.i.i.i, !noalias !78

bb11.i.i.i:                                       ; preds = %bb2.i.i.i4.i.i.i.i.i, %cleanup2.i.i.i, %cleanup.i.i.i
  %.pn.i.i.i = phi { ptr, i32 } [ %32, %cleanup.i.i.i ], [ %43, %cleanup2.i.i.i ], [ %43, %bb2.i.i.i4.i.i.i.i.i ]
; call core::ptr::drop_in_place::<std::env::Args>
  call fastcc void @_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env4ArgsECsk5rB8Fy7VC3_12two_variants(ptr noalias noundef nonnull align 8 dereferenceable(32) %_19.i) #25, !noalias !78
; call core::ptr::drop_in_place::<alloc::vec::Vec<alloc::string::String>>
  call fastcc void @_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc3vec3VecNtNtBL_6string6StringEECsk5rB8Fy7VC3_12two_variants(ptr noalias noundef align 8 dereferenceable(24) %vector.i) #25, !noalias !36
  br label %common.resume

cleanup.i.i.i:                                    ; preds = %bb1.i.i.i
  %32 = landingpad { ptr, i32 }
          cleanup
  br label %bb11.i.i.i

bb2.i.i.i:                                        ; preds = %bb1.i.i.i
  %33 = load i64, ptr %_3.i.i.i, align 8, !range !32, !noalias !77, !noundef !4
  %.not.i.i.i = icmp eq i64 %33, -9223372036854775808
  br i1 %.not.i.i.i, label %bb9.i.i.i, label %bb3.i.i.i

bb3.i.i.i:                                        ; preds = %bb2.i.i.i
  %element.sroa.5.0.copyload.i.i.i = load ptr, ptr %element.sroa.5.0._3.sroa_idx.i.i.i, align 8, !noalias !77
  %element.sroa.6.0.copyload.i.i.i = load i64, ptr %element.sroa.6.0._3.sroa_idx.i.i.i, align 8, !noalias !77
  %_19.i.i.i = icmp samesign ult i64 %len.i.i.i, 384307168202282326
  call void @llvm.assume(i1 %_19.i.i.i)
  %self1.i.i.i = load i64, ptr %vector.i, align 8, !range !8, !alias.scope !79, !noalias !80, !noundef !4
  %_8.i.i.i = icmp eq i64 %len.i.i.i, %self1.i.i.i
  br i1 %_8.i.i.i, label %bb1.i.i.i.i, label %bb8.i.i.i

bb9.i.i.i:                                        ; preds = %bb2.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i.i.i), !noalias !77
  call void @llvm.experimental.noalias.scope.decl(metadata !81)
  call void @llvm.experimental.noalias.scope.decl(metadata !84)
  call void @llvm.experimental.noalias.scope.decl(metadata !87)
  call void @llvm.experimental.noalias.scope.decl(metadata !90)
  call void @llvm.experimental.noalias.scope.decl(metadata !93)
  %self.val.i.i.i.i.i.i.i.i = load ptr, ptr %31, align 8, !alias.scope !96, !noalias !97, !nonnull !4, !noundef !4
  %self.val1.i.i.i.i.i.i.i.i = load ptr, ptr %30, align 8, !alias.scope !96, !noalias !97, !nonnull !4, !noundef !4
  %34 = ptrtoint ptr %self.val1.i.i.i.i.i.i.i.i to i64
  %35 = ptrtoint ptr %self.val.i.i.i.i.i.i.i.i to i64
  %36 = sub nuw i64 %34, %35
  %37 = udiv exact i64 %36, 24
  call void @llvm.experimental.noalias.scope.decl(metadata !98)
  %_710.i.i.i.i.i.i.i.i.i = icmp eq ptr %self.val1.i.i.i.i.i.i.i.i, %self.val.i.i.i.i.i.i.i.i
  br i1 %_710.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i.i:                            ; preds = %bb9.i.i.i, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i.i.i
  %_3.sroa.0.011.i.i.i.i.i.i.i.i.i = phi i64 [ %38, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i.i.i ], [ 0, %bb9.i.i.i ]
  %_6.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self.val.i.i.i.i.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i
  %38 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i.i.i.i.i, align 8, !alias.scope !98, !noalias !101
  %39 = icmp eq i64 %_6.val.i.i.i.i.i.i.i.i.i, 0
  br i1 %39, label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i:               ; preds = %bb5.i.i.i.i.i.i.i.i.i
  %40 = getelementptr i8, ptr %_6.i.i.i.i.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i.i.i.i.i = load ptr, ptr %40, align 8, !alias.scope !98, !noalias !101, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !102
  br label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i.i.i

_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i.i.i: ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = icmp eq i64 %38, %37
  br i1 %_7.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i:                              ; preds = %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i.i.i.i, %bb9.i.i.i
  %41 = getelementptr inbounds nuw i8, ptr %_19.i, i64 16
  %capacity1.i.i3.i.i.i.i.i.i.i.i = load i64, ptr %41, align 8, !alias.scope !96, !noalias !97, !noundef !4
  %42 = icmp eq i64 %capacity1.i.i3.i.i.i.i.i.i.i.i, 0
  br i1 %42, label %_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtB6_6string6StringEINtB2_18SpecFromIterNestedB11_NtNtCsefmIBSMl6ne_3std3env4ArgsE9from_iterCsk5rB8Fy7VC3_12two_variants.exit, label %bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i:                   ; preds = %bb2.i.i.i.i.i.i.i.i
  %ptr.i.i5.i.i.i.i.i.i.i.i = load ptr, ptr %_19.i, align 8, !alias.scope !96, !noalias !97, !nonnull !4, !noundef !4
  %alloc_size.i.i.i.i.i.i6.i.i.i.i.i.i.i.i = mul nuw i64 %capacity1.i.i3.i.i.i.i.i.i.i.i, 24
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i5.i.i.i.i.i.i.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22, !noalias !101
  br label %_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtB6_6string6StringEINtB2_18SpecFromIterNestedB11_NtNtCsefmIBSMl6ne_3std3env4ArgsE9from_iterCsk5rB8Fy7VC3_12two_variants.exit

bb8.i.i.i:                                        ; preds = %bb1.i.i.i.bb8.i.i_crit_edge.i, %bb3.i.i.i
  %_23.i.i.i = phi ptr [ %_23.i.i.pre.i, %bb1.i.i.i.bb8.i.i_crit_edge.i ], [ %_23.i.i18.i, %bb3.i.i.i ]
  %dst.i.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %_23.i.i.i, i64 %len.i.i.i
  store i64 %33, ptr %dst.i.i.i, align 8, !noalias !78
  %src.sroa.4.0.dst.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %dst.i.i.i, i64 8
  store ptr %element.sroa.5.0.copyload.i.i.i, ptr %src.sroa.4.0.dst.sroa_idx.i.i.i, align 8, !noalias !78
  %src.sroa.5.0.dst.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %dst.i.i.i, i64 16
  store i64 %element.sroa.6.0.copyload.i.i.i, ptr %src.sroa.5.0.dst.sroa_idx.i.i.i, align 8, !noalias !78
  %new_len.i.i.i = add nuw nsw i64 %len.i.i.i, 1
  store i64 %new_len.i.i.i, ptr %vector1.sroa.6.0.vector.sroa_idx.i, align 8, !alias.scope !79, !noalias !80
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i.i.i), !noalias !77
  br label %bb1.i.i.i

cleanup2.i.i.i:                                   ; preds = %bb1.i.i.i.i
  %43 = landingpad { ptr, i32 }
          cleanup
  %44 = icmp eq i64 %33, 0
  br i1 %44, label %bb11.i.i.i, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %cleanup2.i.i.i
  %45 = icmp ne ptr %element.sroa.5.0.copyload.i.i.i, null
  call void @llvm.assume(i1 %45)
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %element.sroa.5.0.copyload.i.i.i, i64 noundef %33, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !78
  br label %bb11.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb3.i.i.i
  %iterator.val5.i.i.i = load ptr, ptr %30, align 8, !alias.scope !103, !noalias !97, !nonnull !4, !noundef !4
  %46 = ptrtoint ptr %iterator.val5.i.i.i to i64
  %iterator.val.i.i.i = load ptr, ptr %31, align 8, !alias.scope !103, !noalias !97, !nonnull !4, !noundef !4
  %47 = ptrtoint ptr %iterator.val.i.i.i to i64
  %48 = sub nuw i64 %46, %47
  %49 = udiv exact i64 %48, 24
  %50 = add nuw nsw i64 %49, 1
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsavYRE6QVrtQ_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsk5rB8Fy7VC3_12two_variants(ptr noalias noundef nonnull align 8 dereferenceable(24) %vector.i, i64 noundef %len.i.i.i, i64 noundef range(i64 1, 0) %50)
          to label %bb1.i.i.i.bb8.i.i_crit_edge.i unwind label %cleanup2.i.i.i

bb1.i.i.i.bb8.i.i_crit_edge.i:                    ; preds = %bb1.i.i.i.i
  %_23.i.i.pre.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i, align 8, !alias.scope !79, !noalias !80
  br label %bb8.i.i.i

common.resume:                                    ; preds = %bb11.i.i.i, %bb2.i.i.i.i.i, %bb2.i.i.i.i.i4.i.i.i.i.i, %bb18
  %common.resume.op = phi { ptr, i32 } [ %.pn9, %bb18 ], [ %.pn.i.i.i, %bb11.i.i.i ], [ %.pn.ph.i, %bb2.i.i.i.i.i ], [ %.pn.ph.i, %bb2.i.i.i.i.i4.i.i.i.i.i ]
  resume { ptr, i32 } %common.resume.op

bb10.i:                                           ; preds = %bb2.i.i.i4.i.i.i, %cleanup2.i, %cleanup.i
  %.pn.ph.i = phi { ptr, i32 } [ %0, %cleanup.i ], [ %15, %cleanup2.i ], [ %15, %bb2.i.i.i4.i.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !104)
  call void @llvm.experimental.noalias.scope.decl(metadata !107), !noalias !36
  call void @llvm.experimental.noalias.scope.decl(metadata !110), !noalias !36
  call void @llvm.experimental.noalias.scope.decl(metadata !113), !noalias !36
  call void @llvm.experimental.noalias.scope.decl(metadata !116), !noalias !36
  %51 = getelementptr inbounds nuw i8, ptr %_2, i64 8
  %self.val.i.i.i.i.i = load ptr, ptr %51, align 8, !alias.scope !119, !noalias !36, !nonnull !4, !noundef !4
  %52 = getelementptr inbounds nuw i8, ptr %_2, i64 24
  %self.val1.i.i.i.i.i = load ptr, ptr %52, align 8, !alias.scope !119, !noalias !36, !nonnull !4, !noundef !4
  %53 = ptrtoint ptr %self.val1.i.i.i.i.i to i64
  %54 = ptrtoint ptr %self.val.i.i.i.i.i to i64
  %55 = sub nuw i64 %53, %54
  %56 = udiv exact i64 %55, 24
  call void @llvm.experimental.noalias.scope.decl(metadata !120), !noalias !36
  %_710.i.i.i.i.i.i = icmp eq ptr %self.val1.i.i.i.i.i, %self.val.i.i.i.i.i
  br i1 %_710.i.i.i.i.i.i, label %bb2.i.i.i.i.i, label %bb5.i.i.i.i.i.i

bb5.i.i.i.i.i.i:                                  ; preds = %bb10.i, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i
  %_3.sroa.0.011.i.i.i.i.i.i = phi i64 [ %57, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i ], [ 0, %bb10.i ]
  %_6.i.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self.val.i.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i.i
  %57 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i.i, align 8, !alias.scope !120, !noalias !123
  %58 = icmp eq i64 %_6.val.i.i.i.i.i.i, 0
  br i1 %58, label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i.i:                     ; preds = %bb5.i.i.i.i.i.i
  %59 = getelementptr i8, ptr %_6.i.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i.i = load ptr, ptr %59, align 8, !alias.scope !120, !noalias !123, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !124
  br label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i

_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i: ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i
  %_7.i.i.i.i.i.i = icmp eq i64 %57, %56
  br i1 %_7.i.i.i.i.i.i, label %bb2.i.i.i.i.i, label %bb5.i.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i.i.i.i, %bb10.i
  %60 = getelementptr inbounds nuw i8, ptr %_2, i64 16
  %capacity1.i.i3.i.i.i.i.i = load i64, ptr %60, align 8, !alias.scope !119, !noalias !36, !noundef !4
  %61 = icmp eq i64 %capacity1.i.i3.i.i.i.i.i, 0
  br i1 %61, label %common.resume, label %bb2.i.i.i.i.i4.i.i.i.i.i

bb2.i.i.i.i.i4.i.i.i.i.i:                         ; preds = %bb2.i.i.i.i.i
  %ptr.i.i5.i.i.i.i.i = load ptr, ptr %_2, align 8, !alias.scope !119, !noalias !36, !nonnull !4, !noundef !4
  %alloc_size.i.i.i.i.i.i6.i.i.i.i.i = mul nuw i64 %capacity1.i.i3.i.i.i.i.i, 24
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i5.i.i.i.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22, !noalias !123
  br label %common.resume

_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtB6_6string6StringEINtB2_18SpecFromIterNestedB11_NtNtCsefmIBSMl6ne_3std3env4ArgsE9from_iterCsk5rB8Fy7VC3_12two_variants.exit: ; preds = %bb2.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_19.i), !noalias !41
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %args, ptr noundef nonnull align 8 dereferenceable(24) %vector.i, i64 24, i1 false), !noalias !39
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i), !noalias !41
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %args, i64 16
  %_3.pre = load i64, ptr %.phi.trans.insert, align 8
  %_50 = icmp ult i64 %_3.pre, 384307168202282326
  call void @llvm.assume(i1 %_50)
  switch i64 %_3.pre, label %bb22 [
    i64 1, label %bb2
    i64 0, label %panic
  ]

bb2:                                              ; preds = %_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtB6_6string6StringEINtB2_18SpecFromIterNestedB11_NtNtCsefmIBSMl6ne_3std3env4ArgsE9from_iterCsk5rB8Fy7VC3_12two_variants.exit
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCsefmIBSMl6ne_3std2io5stdio6__print(ptr noundef nonnull @alloc_c85489ac7d65f986716cfdb1dc3aed62, ptr noundef nonnull inttoptr (i64 79 to ptr))
          to label %bb14.thread unwind label %cleanup

bb14.thread:                                      ; preds = %bb2
  %.phi.trans.insert61 = getelementptr inbounds nuw i8, ptr %args, i64 8
  %_1.val.i.pre = load ptr, ptr %.phi.trans.insert61, align 8, !alias.scope !125
  br label %bb5.i.i.i.preheader

bb18:                                             ; preds = %cleanup.i27, %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i4.i28, %cleanup, %bb17
  %.pn9 = phi { ptr, i32 } [ %82, %bb17 ], [ %62, %cleanup ], [ %94, %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i4.i28 ], [ %94, %cleanup.i27 ]
; call core::ptr::drop_in_place::<alloc::vec::Vec<alloc::string::String>>
  call fastcc void @_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc3vec3VecNtNtBL_6string6StringEECsk5rB8Fy7VC3_12two_variants(ptr noalias noundef align 8 dereferenceable(24) %args) #25
  br label %common.resume

cleanup:                                          ; preds = %bb2.i, %panic, %bb2
  %62 = landingpad { ptr, i32 }
          cleanup
  br label %bb18

bb14:                                             ; preds = %bb3.i30, %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i.i31
  call void @llvm.experimental.noalias.scope.decl(metadata !125)
  call void @llvm.experimental.noalias.scope.decl(metadata !128)
  %_710.i.i.i = icmp eq i64 %_3.pre, 0
  br i1 %_710.i.i.i, label %bb4.i, label %bb5.i.i.i.preheader

bb5.i.i.i.preheader:                              ; preds = %bb14.thread, %bb14
  %_1.val.i67 = phi ptr [ %_1.val.i.pre, %bb14.thread ], [ %_54, %bb14 ]
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb5.i.i.i.preheader, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i
  %_3.sroa.0.011.i.i.i = phi i64 [ %63, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i ], [ 0, %bb5.i.i.i.preheader ]
  %_6.i.i.i = getelementptr inbounds nuw %"alloc::string::String", ptr %_1.val.i67, i64 %_3.sroa.0.011.i.i.i
  %63 = add nuw nsw i64 %_3.sroa.0.011.i.i.i, 1
  %_6.val.i.i.i = load i64, ptr %_6.i.i.i, align 8, !alias.scope !128, !noalias !125
  %64 = icmp eq i64 %_6.val.i.i.i, 0
  br i1 %64, label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i, label %bb2.i.i.i4.i.i.i.i.i18

bb2.i.i.i4.i.i.i.i.i18:                           ; preds = %bb5.i.i.i
  %65 = getelementptr i8, ptr %_6.i.i.i, i64 8
  %_6.val7.i.i.i = load ptr, ptr %65, align 8, !alias.scope !128, !noalias !125, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i, i64 noundef %_6.val.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !131
  br label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i

_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i: ; preds = %bb2.i.i.i4.i.i.i.i.i18, %bb5.i.i.i
  %_7.i.i.i = icmp eq i64 %63, %_3.pre
  br i1 %_7.i.i.i, label %bb4.i, label %bb5.i.i.i

bb4.i:                                            ; preds = %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i, %bb14
  %_1.val.i68 = phi ptr [ %_54, %bb14 ], [ %_1.val.i67, %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants.exit.i.i.i ]
  %_1.val4.i = load i64, ptr %args, align 8, !range !8, !alias.scope !125, !noundef !4
  %66 = icmp eq i64 %_1.val4.i, 0
  br i1 %66, label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc3vec3VecNtNtBL_6string6StringEECsk5rB8Fy7VC3_12two_variants.exit, label %bb2.i.i.i6.i

bb2.i.i.i6.i:                                     ; preds = %bb4.i
  %alloc_size.i.i.i.i7.i = mul nuw i64 %_1.val4.i, 24
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i68, i64 noundef %alloc_size.i.i.i.i7.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22, !noalias !125
  br label %_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc3vec3VecNtNtBL_6string6StringEECsk5rB8Fy7VC3_12two_variants.exit

_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc3vec3VecNtNtBL_6string6StringEECsk5rB8Fy7VC3_12two_variants.exit: ; preds = %bb4.i, %bb2.i.i.i6.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %args)
  ret void

bb22:                                             ; preds = %_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtB6_6string6StringEINtB2_18SpecFromIterNestedB11_NtNtCsefmIBSMl6ne_3std3env4ArgsE9from_iterCsk5rB8Fy7VC3_12two_variants.exit
  %67 = getelementptr inbounds nuw i8, ptr %args, i64 8
  %_54 = load ptr, ptr %67, align 8, !nonnull !4, !noundef !4
  %68 = getelementptr inbounds nuw i8, ptr %_54, i64 32
  %_69 = load ptr, ptr %68, align 8, !nonnull !4, !noundef !4
  %69 = getelementptr inbounds nuw i8, ptr %_54, i64 40
  %_68 = load i64, ptr %69, align 8, !noundef !4
  switch i64 %_68, label %bb9thread-pre-split.i [
    i64 0, label %bb2.i
    i64 1, label %bb7.i
  ]

bb7.i:                                            ; preds = %bb22
  %70 = load i8, ptr %_69, align 1, !alias.scope !132, !noalias !135, !noundef !4
  switch i8 %70, label %bb9.i [
    i8 43, label %bb2.i
    i8 45, label %bb2.i
  ]

bb9thread-pre-split.i:                            ; preds = %bb22
  %.pr.i = load i8, ptr %_69, align 1, !alias.scope !132, !noalias !135
  br label %bb9.i

bb9.i:                                            ; preds = %bb9thread-pre-split.i, %bb7.i
  %71 = phi i8 [ %.pr.i, %bb9thread-pre-split.i ], [ %70, %bb7.i ]
  %cond.i = icmp eq i8 %71, 43
  %rest.1.i = sext i1 %cond.i to i64
  %src.sroa.15.0.i = add nsw i64 %_68, %rest.1.i
  %src.sroa.0.0.idx.i = zext i1 %cond.i to i64
  %src.sroa.0.0.i = getelementptr inbounds nuw i8, ptr %_69, i64 %src.sroa.0.0.idx.i
  %_10.i = icmp samesign ult i64 %src.sroa.15.0.i, 17
  br i1 %_10.i, label %bb15.preheader.i, label %bb22.i

bb15.preheader.i:                                 ; preds = %bb9.i
  %_13.not56.i = icmp eq i64 %src.sroa.15.0.i, 0
  br i1 %_13.not56.i, label %bb4.thread, label %bb16.i

bb4.thread:                                       ; preds = %bb15.preheader.i
  store i64 0, ptr %x, align 8
  br label %bb27.thread

bb22.i:                                           ; preds = %bb9.i, %bb40.i
  %result.sroa.0.0.i = phi i64 [ %_66.0.i, %bb40.i ], [ 0, %bb9.i ]
  %src.sroa.15.1.i = phi i64 [ %rest.12.i, %bb40.i ], [ %src.sroa.15.0.i, %bb9.i ]
  %src.sroa.0.1.i = phi ptr [ %rest.01.i, %bb40.i ], [ %src.sroa.0.0.i, %bb9.i ]
  %_30.not.i = icmp eq i64 %src.sroa.15.1.i, 0
  br i1 %_30.not.i, label %bb4, label %bb23.i

bb23.i:                                           ; preds = %bb22.i
  %rest.01.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i, i64 1
  %rest.12.i = add nsw i64 %src.sroa.15.1.i, -1
  %72 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %result.sroa.0.0.i, i64 10)
  %_60.0.i = extractvalue { i64, i1 } %72, 0
  %_60.1.i = extractvalue { i64, i1 } %72, 1
  %73 = load i8, ptr %src.sroa.0.1.i, align 1, !alias.scope !132, !noalias !135, !noundef !4
  br i1 %_60.1.i, label %bb31.i, label %bb33.i, !prof !137

bb33.i:                                           ; preds = %bb23.i
  %74 = zext i8 %73 to i32
  %75 = add nsw i32 %74, -48
  %_14.i.i = icmp ult i32 %75, 10
  br i1 %_14.i.i, label %bb40.i, label %bb2.i

bb31.i:                                           ; preds = %bb23.i
  %76 = add i8 %73, -48
  %_14.i44.i = icmp ult i8 %76, 10
  %spec.select = select i1 %_14.i44.i, i8 2, i8 1
  br label %bb2.i

bb40.i:                                           ; preds = %bb33.i
  %77 = zext nneg i32 %75 to i64
  %_66.0.i = add i64 %_60.0.i, %77
  %_66.1.i = icmp ult i64 %_66.0.i, %_60.0.i
  br i1 %_66.1.i, label %bb2.i, label %bb22.i, !prof !137

bb16.i:                                           ; preds = %bb15.preheader.i, %bb20.i
  %src.sroa.0.259.i = phi ptr [ %rest.05.i, %bb20.i ], [ %src.sroa.0.0.i, %bb15.preheader.i ]
  %src.sroa.15.258.i = phi i64 [ %rest.16.i, %bb20.i ], [ %src.sroa.15.0.i, %bb15.preheader.i ]
  %result.sroa.0.257.i = phi i64 [ %80, %bb20.i ], [ 0, %bb15.preheader.i ]
  %_20.i = load i8, ptr %src.sroa.0.259.i, align 1, !alias.scope !132, !noalias !135, !noundef !4
  %_19.i20 = zext i8 %_20.i to i32
  %78 = add nsw i32 %_19.i20, -48
  %_14.i46.i = icmp ult i32 %78, 10
  br i1 %_14.i46.i, label %bb20.i, label %bb2.i

bb20.i:                                           ; preds = %bb16.i
  %79 = mul i64 %result.sroa.0.257.i, 10
  %rest.16.i = add nsw i64 %src.sroa.15.258.i, -1
  %rest.05.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i, i64 1
  %_24.i = zext nneg i32 %78 to i64
  %80 = add i64 %79, %_24.i
  %_13.not.i = icmp eq i64 %rest.16.i, 0
  br i1 %_13.not.i, label %bb4, label %bb16.i

panic:                                            ; preds = %bb2.i.i.i.i.i4.i.i.i.i.i.i, %bb2.i.i.i.i.i.i, %_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtB6_6string6StringEINtB2_18SpecFromIterNestedB11_NtNtCsefmIBSMl6ne_3std3env4ArgsE9from_iterCsk5rB8Fy7VC3_12two_variants.exit
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCshhmRwEtsTQ2_4core9panicking18panic_bounds_check(i64 noundef 1, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fa232d3fec0961e04b35ddd656c9b32) #23
          to label %unreachable unwind label %cleanup

unreachable:                                      ; preds = %panic
  unreachable

bb2.i:                                            ; preds = %bb33.i, %bb40.i, %bb16.i, %bb31.i, %bb22, %bb7.i, %bb7.i
  %_7.sroa.4.0.ph = phi i8 [ 1, %bb7.i ], [ 1, %bb7.i ], [ 0, %bb22 ], [ %spec.select, %bb31.i ], [ 1, %bb16.i ], [ 1, %bb33.i ], [ 2, %bb40.i ]
  call void @llvm.lifetime.start.p0(i64 1, ptr nonnull %e.i), !noalias !138
  store i8 %_7.sroa.4.0.ph, ptr %e.i, align 1, !noalias !138
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCshhmRwEtsTQ2_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9e4a0be258b7ac4e4e85080401a435ca) #23
          to label %.noexc unwind label %cleanup

.noexc:                                           ; preds = %bb2.i
  unreachable

bb4:                                              ; preds = %bb22.i, %bb20.i
  %_7.sroa.1134.0 = phi i64 [ %80, %bb20.i ], [ %result.sroa.0.0.i, %bb22.i ]
  %_7.sroa.1134.0.fr = freeze i64 %_7.sroa.1134.0
  store i64 %_7.sroa.1134.0.fr, ptr %x, align 8
  %81 = icmp eq i64 %_7.sroa.1134.0.fr, 0
  br i1 %81, label %bb27.thread, label %bb27

bb27.thread:                                      ; preds = %bb4, %bb4.thread
  store ptr @vtable.1, ptr %animal_vtable, align 8
  store ptr @vtable.1, ptr %cat_vtable, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %res)
  br label %bb26

bb17:                                             ; preds = %cleanup7
; invoke core::ptr::drop_in_place::<alloc::boxed::Box<dyn two_variants::Animal>>
  invoke fastcc void @_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc5boxed3BoxDNtCsk5rB8Fy7VC3_12two_variants6AnimalEL_EEB1i_(ptr nonnull inttoptr (i64 1 to ptr), ptr nonnull %83) #25
          to label %bb18 unwind label %terminate

cleanup7:                                         ; preds = %bb10, %bb9, %bb8, %bb7, %bb6
  %82 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<alloc::boxed::Box<dyn two_variants::Animal>>
  invoke fastcc void @_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc5boxed3BoxDNtCsk5rB8Fy7VC3_12two_variants6AnimalEL_EEB1i_(ptr nonnull inttoptr (i64 1 to ptr), ptr nonnull @vtable.1) #25
          to label %bb17 unwind label %terminate

bb27:                                             ; preds = %bb4
  store ptr @vtable.2, ptr %animal_vtable, align 8
  store ptr @vtable.1, ptr %cat_vtable, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %res)
  %_123 = icmp eq ptr @vtable.2, @vtable.1
  br i1 %_123, label %bb26, label %bb6

bb26:                                             ; preds = %bb27.thread, %bb27
  br label %bb6

bb6:                                              ; preds = %bb27, %bb26
  %83 = phi ptr [ @vtable.1, %bb26 ], [ @vtable.2, %bb27 ]
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %res)
  store i64 22222, ptr %res, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args1)
  store ptr %x, ptr %args1, align 8
  %_20.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args1, i64 8
  store ptr @_RNvXsZ_NtNtCshhmRwEtsTQ2_4core3fmt3numjNtB7_5Debug3fmt, ptr %_20.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCsefmIBSMl6ne_3std2io5stdio6__print(ptr noundef nonnull @alloc_a8c4773e19f17bafbfce7be36b8ae92b, ptr noundef nonnull %args1)
          to label %bb7 unwind label %cleanup7

bb7:                                              ; preds = %bb6
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args1)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %animal_vtable, ptr %args2, align 8
  %_26.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args2, i64 8
  store ptr @_RNvXs1_NtNtCshhmRwEtsTQ2_4core3ptr8metadataINtB5_11DynMetadataDNtCsk5rB8Fy7VC3_12two_variants6AnimalEL_ENtNtB9_3fmt5Debug3fmtB11_, ptr %_26.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCsefmIBSMl6ne_3std2io5stdio6__print(ptr noundef nonnull @alloc_b8a8ade38cd9a3b8b2bfcff364622fa1, ptr noundef nonnull %args2)
          to label %bb8 unwind label %cleanup7

bb8:                                              ; preds = %bb7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args3)
  store ptr %cat_vtable, ptr %args3, align 8
  %_32.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args3, i64 8
  store ptr @_RNvXs1_NtNtCshhmRwEtsTQ2_4core3ptr8metadataINtB5_11DynMetadataDNtCsk5rB8Fy7VC3_12two_variants6AnimalEL_ENtNtB9_3fmt5Debug3fmtB11_, ptr %_32.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCsefmIBSMl6ne_3std2io5stdio6__print(ptr noundef nonnull @alloc_18a9af093ad63d2a518738c428eb578e, ptr noundef nonnull %args3)
          to label %bb9 unwind label %cleanup7

bb9:                                              ; preds = %bb8
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args3)
  call void @llvm.lifetime.start.p0(i64 1, ptr nonnull %_37)
  %_96 = load ptr, ptr %animal_vtable, align 8, !nonnull !4, !noundef !4
  %_97 = load ptr, ptr %cat_vtable, align 8, !nonnull !4, !noundef !4
  %84 = icmp eq ptr %_96, %_97
  %85 = zext i1 %84 to i8
  store i8 %85, ptr %_37, align 1
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args4)
  store ptr %_37, ptr %args4, align 8
  %_39.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args4, i64 8
  store ptr @_RNvXsf_NtCshhmRwEtsTQ2_4core3fmtbNtB5_5Debug3fmt, ptr %_39.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCsefmIBSMl6ne_3std2io5stdio6__print(ptr noundef nonnull @alloc_a16f3e37bf6cf20be080dc24dc11b38e, ptr noundef nonnull %args4)
          to label %bb10 unwind label %cleanup7

bb10:                                             ; preds = %bb9
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args4)
  call void @llvm.lifetime.end.p0(i64 1, ptr nonnull %_37)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args5)
  store ptr %res, ptr %args5, align 8
  %_45.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args5, i64 8
  store ptr @_RNvXsZ_NtNtCshhmRwEtsTQ2_4core3fmt3numjNtB7_5Debug3fmt, ptr %_45.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCsefmIBSMl6ne_3std2io5stdio6__print(ptr noundef nonnull @alloc_0ea66dfa401483f8b62b25c396fda960, ptr noundef nonnull %args5)
          to label %bb12 unwind label %cleanup7

bb12:                                             ; preds = %bb10
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args5)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %res)
  %86 = load ptr, ptr %83, align 8, !invariant.load !4
  %.not.i25 = icmp eq ptr %86, null
  br i1 %.not.i25, label %bb3.i30, label %is_not_null.i26

is_not_null.i26:                                  ; preds = %bb12
  invoke void %86(ptr noundef nonnull inttoptr (i64 1 to ptr))
          to label %bb3.i30 unwind label %cleanup.i27

bb3.i30:                                          ; preds = %is_not_null.i26, %bb12
  %87 = getelementptr inbounds nuw i8, ptr %83, i64 8
  %88 = load i64, ptr %87, align 8, !range !8, !invariant.load !4
  %89 = getelementptr inbounds nuw i8, ptr %83, i64 16
  %90 = load i64, ptr %89, align 8, !range !9, !invariant.load !4
  %91 = add i64 %90, -1
  %92 = icmp sgt i64 %91, -1
  call void @llvm.assume(i1 %92)
  %93 = icmp eq i64 %88, 0
  br i1 %93, label %bb14, label %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i.i31

_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i.i31: ; preds = %bb3.i30
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull inttoptr (i64 1 to ptr), i64 noundef %88, i64 noundef range(i64 1, -9223372036854775807) %90) #22
  br label %bb14

cleanup.i27:                                      ; preds = %is_not_null.i26
  %94 = landingpad { ptr, i32 }
          cleanup
  %95 = getelementptr inbounds nuw i8, ptr %83, i64 8
  %96 = load i64, ptr %95, align 8, !range !8, !invariant.load !4
  %97 = getelementptr inbounds nuw i8, ptr %83, i64 16
  %98 = load i64, ptr %97, align 8, !range !9, !invariant.load !4
  %99 = add i64 %98, -1
  %100 = icmp sgt i64 %99, -1
  call void @llvm.assume(i1 %100)
  %101 = icmp eq i64 %96, 0
  br i1 %101, label %bb18, label %_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i4.i28

_RNvXs_NtCsavYRE6QVrtQ_5alloc5allocNtB4_6GlobalNtNtCshhmRwEtsTQ2_4core5alloc9Allocator10deallocate.exit.i4.i28: ; preds = %cleanup.i27
; call __rustc::__rust_dealloc
  call void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr noundef nonnull inttoptr (i64 1 to ptr), i64 noundef %96, i64 noundef range(i64 1, -9223372036854775807) %98) #22
  br label %bb18

terminate:                                        ; preds = %cleanup7, %bb17
  %102 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCshhmRwEtsTQ2_4core9panicking16panic_in_cleanup() #26
  unreachable
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind nonlazybind uwtable
define internal fastcc void @_RNvMs4_NtCsavYRE6QVrtQ_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsk5rB8Fy7VC3_12two_variants(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap) unnamed_addr #5 {
start:
  %_20.0.i = mul i64 %cap, 24
  %or.cond.i = icmp ugt i64 %cap, 384307168202282325
  br i1 %or.cond.i, label %bb11, label %bb14, !prof !63

bb14:                                             ; preds = %start
  %0 = icmp eq i64 %self.0.val, 0
  br i1 %0, label %bb5, label %bb3

bb3:                                              ; preds = %bb14
  %alloc_size.i = mul nuw i64 %self.0.val, 24
  %1 = icmp ne ptr %self.8.val, null
  tail call void @llvm.assume(i1 %1)
  %cond.i.i = icmp uge i64 %_20.0.i, %alloc_size.i
  tail call void @llvm.assume(i1 %cond.i.i)
; call __rustc::__rust_realloc
  %raw_ptr.i.i = tail call noundef align 8 ptr @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %alloc_size.i, i64 noundef range(i64 1, -9223372036854775807) 8, i64 noundef %_20.0.i) #22
  br label %bb7

bb5:                                              ; preds = %bb14
  %2 = icmp eq i64 %_20.0.i, 0
  br i1 %2, label %bb9, label %bb4.i.i11

bb4.i.i11:                                        ; preds = %bb5
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsl5Zt2FtcpFZ_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #22
; call __rustc::__rust_alloc
  %3 = tail call noundef align 8 ptr @_RNvCsl5Zt2FtcpFZ_7___rustc12___rust_alloc(i64 noundef %_20.0.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22
  br label %bb7

bb7:                                              ; preds = %bb4.i.i11, %bb3
  %raw_ptr.i.i.pn = phi ptr [ %raw_ptr.i.i, %bb3 ], [ %3, %bb4.i.i11 ]
  %4 = icmp eq ptr %raw_ptr.i.i.pn, null
  br i1 %4, label %bb8, label %bb9

bb8:                                              ; preds = %bb7
  %5 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 8, ptr %5, align 8
  br label %bb11

bb9:                                              ; preds = %bb5, %bb7
  %raw_ptr.i.i.pn9 = phi ptr [ %raw_ptr.i.i.pn, %bb7 ], [ inttoptr (i64 8 to ptr), %bb5 ]
  %6 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %raw_ptr.i.i.pn9, ptr %6, align 8
  br label %bb11

bb11:                                             ; preds = %start, %bb9, %bb8
  %.sink10 = phi i64 [ 16, %bb9 ], [ 16, %bb8 ], [ 8, %start ]
  %_20.0.i.sink = phi i64 [ %_20.0.i, %bb9 ], [ %_20.0.i, %bb8 ], [ 0, %start ]
  %storemerge8 = phi i64 [ 0, %bb9 ], [ 1, %bb8 ], [ 1, %start ]
  %7 = getelementptr inbounds nuw i8, ptr %_0, i64 %.sink10
  store i64 %_20.0.i.sink, ptr %7, align 8
  store i64 %storemerge8, ptr %_0, align 8
  ret void
}

; <two_variants::Cat as two_variants::Animal>::walk
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none) uwtable
define internal noundef i64 @_RNvXCsk5rB8Fy7VC3_12two_variantsNtB2_3CatNtB2_6Animal4walk(ptr noalias nonnull readonly align 1 captures(none) %self) unnamed_addr #6 {
start:
  ret i64 33333
}

; <two_variants::Cat as two_variants::Animal>::speak
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none) uwtable
define internal noundef i64 @_RNvXCsk5rB8Fy7VC3_12two_variantsNtB2_3CatNtB2_6Animal5speak(ptr noalias nonnull readonly align 1 captures(none) %self) unnamed_addr #6 {
start:
  ret i64 11111
}

; <core::ptr::metadata::DynMetadata<dyn two_variants::Animal> as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
define internal noundef zeroext i1 @_RNvXs1_NtNtCshhmRwEtsTQ2_4core3ptr8metadataINtB5_11DynMetadataDNtCsk5rB8Fy7VC3_12two_variants6AnimalEL_ENtNtB9_3fmt5Debug3fmtB11_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_8 = alloca [8 x i8], align 8
  %_5 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5)
; call <core::fmt::Formatter>::debug_tuple
  call void @_RNvMsa_NtCshhmRwEtsTQ2_4core3fmtNtB5_9Formatter11debug_tuple(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_5, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9593adc5b471736a257f2c43c3399b67, i64 noundef 11)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_8)
  %self1 = load ptr, ptr %self, align 8, !nonnull !4, !noundef !4
  store ptr %self1, ptr %_8, align 8
; call <core::fmt::builders::DebugTuple>::field
  %_3 = call noundef nonnull align 8 ptr @_RNvMs2_NtNtCshhmRwEtsTQ2_4core3fmt8buildersNtB5_10DebugTuple5field(ptr noalias noundef nonnull align 8 dereferenceable(24) %_5, ptr noundef nonnull align 1 %_8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.4)
; call <core::fmt::builders::DebugTuple>::finish
  %_0 = call noundef zeroext i1 @_RNvMs2_NtNtCshhmRwEtsTQ2_4core3fmt8buildersNtB5_10DebugTuple6finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %_3)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_8)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5)
  ret i1 %_0
}

; <&core::num::error::IntErrorKind as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCshhmRwEtsTQ2_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCsk5rB8Fy7VC3_12two_variants(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !4, !align !141, !noundef !4
  %_3.val = load i8, ptr %_3, align 1, !range !142, !noundef !4
  %0 = zext nneg i8 %_3.val to i64
  %switch.gep = getelementptr inbounds nuw [5 x i64], ptr @switch.table._RNvXs1g_NtCshhmRwEtsTQ2_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCsk5rB8Fy7VC3_12two_variants, i64 0, i64 %0
  %switch.load = load i64, ptr %switch.gep, align 8
  %1 = zext nneg i8 %_3.val to i64
  %reltable.shift = shl i64 %1, 2
  %reltable.intrinsic = call ptr @llvm.load.relative.i64(ptr @switch.table._RNvXs1g_NtCshhmRwEtsTQ2_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCsk5rB8Fy7VC3_12two_variants.30.rel, i64 %reltable.shift)
; call <core::fmt::Formatter>::write_str
  %_0.i = tail call noundef zeroext i1 @_RNvMsa_NtCshhmRwEtsTQ2_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %reltable.intrinsic, i64 noundef %switch.load)
  ret i1 %_0.i
}

; <usize as core::fmt::Debug>::fmt
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef zeroext i1 @_RNvXsZ_NtNtCshhmRwEtsTQ2_4core3fmt3numjNtB7_5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %f, i64 16
  %_4 = load i32, ptr %0, align 8, !noundef !4
  %_3 = and i32 %_4, 33554432
  %1 = icmp eq i32 %_3, 0
  br i1 %1, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %_5 = and i32 %_4, 67108864
  %2 = icmp eq i32 %_5, 0
  br i1 %2, label %bb4, label %bb3

bb1:                                              ; preds = %start
; call <usize as core::fmt::LowerHex>::fmt
  %3 = tail call noundef zeroext i1 @_RNvXs6_NtNtCshhmRwEtsTQ2_4core3fmt3numjNtB7_8LowerHex3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb4:                                              ; preds = %bb2
; call <usize as core::fmt::Display>::fmt
  %4 = tail call noundef zeroext i1 @_RNvXsi_NtNtNtCshhmRwEtsTQ2_4core3fmt3num3impjNtB9_7Display3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb3:                                              ; preds = %bb2
; call <usize as core::fmt::UpperHex>::fmt
  %5 = tail call noundef zeroext i1 @_RNvXs8_NtNtCshhmRwEtsTQ2_4core3fmt3numjNtB7_8UpperHex3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb6:                                              ; preds = %bb4, %bb3, %bb1
  %_0.sroa.0.0.in = phi i1 [ %4, %bb4 ], [ %5, %bb3 ], [ %3, %bb1 ]
  ret i1 %_0.sroa.0.0.in
}

; <two_variants::Dog as two_variants::Animal>::walk
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none) uwtable
define internal noundef i64 @_RNvXs_Csk5rB8Fy7VC3_12two_variantsNtB4_3DogNtB4_6Animal4walk(ptr noalias nonnull readonly align 1 captures(none) %self) unnamed_addr #6 {
start:
  ret i64 44444
}

; <two_variants::Dog as two_variants::Animal>::speak
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none) uwtable
define internal noundef i64 @_RNvXs_Csk5rB8Fy7VC3_12two_variantsNtB4_3DogNtB4_6Animal5speak(ptr noalias nonnull readonly align 1 captures(none) %self) unnamed_addr #6 {
start:
  ret i64 22222
}

; <core::num::error::ParseIntError as core::fmt::Debug>::fmt
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef zeroext i1 @_RNvXsc_NtNtCshhmRwEtsTQ2_4core3num5errorNtB5_13ParseIntErrorNtNtB9_3fmt5Debug3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 {
start:
  %_5 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_5)
  store ptr %self, ptr %_5, align 8
; call <core::fmt::Formatter>::debug_struct_field1_finish
  %_0 = call noundef zeroext i1 @_RNvMsa_NtCshhmRwEtsTQ2_4core3fmtNtB5_9Formatter26debug_struct_field1_finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_f62df14955f7d78bca139b0a7668683d, i64 noundef 13, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a5d866b1768ad3f826bccdb004a1a8ae, i64 noundef 4, ptr noundef nonnull align 1 %_5, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.5)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_5)
  ret i1 %_0
}

; <bool as core::fmt::Debug>::fmt
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef zeroext i1 @_RNvXsf_NtCshhmRwEtsTQ2_4core3fmtbNtB5_5Debug3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 {
start:
; call <bool as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsg_NtCshhmRwEtsTQ2_4core3fmtbNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <*const core::ptr::metadata::VTable as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
define internal noundef zeroext i1 @_RNvXsp_NtCshhmRwEtsTQ2_4core3fmtPNtNtNtB7_3ptr8metadata6VTableNtB5_5Debug3fmtCsk5rB8Fy7VC3_12two_variants(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %self.val = load ptr, ptr %self, align 8, !noundef !4
  %_4.i = ptrtoint ptr %self.val to i64
; call core::fmt::pointer_fmt_inner
  %0 = tail call noundef zeroext i1 @_RNvNtCshhmRwEtsTQ2_4core3fmt17pointer_fmt_inner(i64 noundef %_4.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %0
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #7

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #8

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #7

; Function Attrs: nounwind nonlazybind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; <std::env::Args as core::iter::traits::iterator::Iterator>::next
; Function Attrs: nonlazybind uwtable
declare void @_RNvXsa_NtCsefmIBSMl6ne_3std3envNtB5_4ArgsNtNtNtNtCshhmRwEtsTQ2_4core4iter6traits8iterator8Iterator4next(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef align 8 dereferenceable(32)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #9

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind nonlazybind optsize uwtable
declare void @_RNvNtCshhmRwEtsTQ2_4core9panicking16panic_in_cleanup() unnamed_addr #10

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare noundef i64 @_RNvNtCsefmIBSMl6ne_3std2rt19lang_start_internal(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn nonlazybind optsize uwtable
declare void @_RNvNtCsavYRE6QVrtQ_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #11

; <core::fmt::Formatter>::write_str
; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @_RNvMsa_NtCshhmRwEtsTQ2_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::fmt::pointer_fmt_inner
; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @_RNvNtCshhmRwEtsTQ2_4core3fmt17pointer_fmt_inner(i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; std::env::args
; Function Attrs: nonlazybind uwtable
declare void @_RNvNtCsefmIBSMl6ne_3std3env4args(ptr dead_on_unwind noalias noundef writable sret([32 x i8]) align 8 captures(none) dereferenceable(32)) unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_RNvNtNtCsefmIBSMl6ne_3std2io5stdio6__print(ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; core::panicking::panic_bounds_check
; Function Attrs: cold minsize noinline noreturn nonlazybind optsize uwtable
declare void @_RNvNtCshhmRwEtsTQ2_4core9panicking18panic_bounds_check(i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #12

; __rustc::__rust_dealloc
; Function Attrs: nounwind nonlazybind allockind("free") uwtable
declare void @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #13

; __rustc::__rust_realloc
; Function Attrs: nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsl5Zt2FtcpFZ_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #14

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind nonlazybind uwtable
declare void @_RNvCsl5Zt2FtcpFZ_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #1

; __rustc::__rust_alloc
; Function Attrs: nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsl5Zt2FtcpFZ_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #15

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_RNvNtCshhmRwEtsTQ2_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #16

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #17

; <core::fmt::Formatter>::debug_tuple
; Function Attrs: nonlazybind uwtable
declare void @_RNvMsa_NtCshhmRwEtsTQ2_4core3fmtNtB5_9Formatter11debug_tuple(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(address) dereferenceable(24), ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <core::fmt::builders::DebugTuple>::field
; Function Attrs: nonlazybind uwtable
declare noundef nonnull align 8 ptr @_RNvMs2_NtNtCshhmRwEtsTQ2_4core3fmt8buildersNtB5_10DebugTuple5field(ptr noalias noundef align 8 dereferenceable(24), ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #0

; <core::fmt::builders::DebugTuple>::finish
; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @_RNvMs2_NtNtCshhmRwEtsTQ2_4core3fmt8buildersNtB5_10DebugTuple6finish(ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <usize as core::fmt::Display>::fmt
; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @_RNvXsi_NtNtNtCshhmRwEtsTQ2_4core3fmt3num3impjNtB9_7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <usize as core::fmt::UpperHex>::fmt
; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @_RNvXs8_NtNtCshhmRwEtsTQ2_4core3fmt3numjNtB7_8UpperHex3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <usize as core::fmt::LowerHex>::fmt
; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @_RNvXs6_NtNtCshhmRwEtsTQ2_4core3fmt3numjNtB7_8LowerHex3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <core::fmt::Formatter>::debug_struct_field1_finish
; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @_RNvMsa_NtCshhmRwEtsTQ2_4core3fmtNtB5_9Formatter26debug_struct_field1_finish(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #0

; <bool as core::fmt::Display>::fmt
; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @_RNvXsg_NtCshhmRwEtsTQ2_4core3fmtbNtB5_7Display3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; Function Attrs: nonlazybind
define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #18 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = sext i32 %0 to i64
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i)
  store ptr @_RNvCsk5rB8Fy7VC3_12two_variants4main, ptr %_7.i, align 8
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_RNvNtCsefmIBSMl6ne_3std2rt19lang_start_internal(ptr noundef nonnull align 1 %_7.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %2, ptr noundef %1, i8 noundef 0)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i)
  %3 = trunc i64 %_0.i to i32
  ret i32 %3
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #19

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #20

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: read)
declare ptr @llvm.load.relative.i64(ptr, i64) #21

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { cold nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #4 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #5 = { cold nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #6 = { mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none) uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #7 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #8 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #9 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #10 = { cold minsize noinline noreturn nounwind nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #11 = { cold minsize noreturn nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #12 = { cold minsize noinline noreturn nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #13 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #14 = { nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #15 = { nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsl5Zt2FtcpFZ_7___rustc19___rust_alloc_zeroed" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #16 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #17 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #18 = { nonlazybind "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #19 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #20 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #21 = { nocallback nofree nosync nounwind willreturn memory(argmem: read) }
attributes #22 = { nounwind }
attributes #23 = { noreturn }
attributes #24 = { noinline }
attributes #25 = { cold }
attributes #26 = { cold noreturn nounwind }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{!"rustc version 1.94.0-nightly (2f1bd3f37 2026-01-12)"}
!4 = !{}
!5 = !{!6}
!6 = distinct !{!6, !7, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants: %_1.0"}
!7 = distinct !{!7, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants"}
!8 = !{i64 0, i64 -9223372036854775808}
!9 = !{i64 1, i64 0}
!10 = !{!11}
!11 = distinct !{!11, !12, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env6ArgsOsECsk5rB8Fy7VC3_12two_variants: %_1"}
!12 = distinct !{!12, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env6ArgsOsECsk5rB8Fy7VC3_12two_variants"}
!13 = !{!14}
!14 = distinct !{!14, !15, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtNtCsefmIBSMl6ne_3std3sys4args6common4ArgsECsk5rB8Fy7VC3_12two_variants: %_1"}
!15 = distinct !{!15, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtNtCsefmIBSMl6ne_3std3sys4args6common4ArgsECsk5rB8Fy7VC3_12two_variants"}
!16 = !{!17}
!17 = distinct !{!17, !18, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtNtCsavYRE6QVrtQ_5alloc3vec9into_iter8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringEECsk5rB8Fy7VC3_12two_variants: %_1"}
!18 = distinct !{!18, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtNtCsavYRE6QVrtQ_5alloc3vec9into_iter8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringEECsk5rB8Fy7VC3_12two_variants"}
!19 = !{!20}
!20 = distinct !{!20, !21, !"_RNvXse_NtNtCsavYRE6QVrtQ_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringENtNtNtCshhmRwEtsTQ2_4core3ops4drop4Drop4dropCsk5rB8Fy7VC3_12two_variants: %self"}
!21 = distinct !{!21, !"_RNvXse_NtNtCsavYRE6QVrtQ_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringENtNtNtCshhmRwEtsTQ2_4core3ops4drop4Drop4dropCsk5rB8Fy7VC3_12two_variants"}
!22 = !{!20, !17, !14, !11}
!23 = !{!24}
!24 = distinct !{!24, !25, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants: %_1.0"}
!25 = distinct !{!25, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants"}
!26 = !{!24, !20, !17, !14, !11}
!27 = !{i64 15613108067727335}
!28 = !{!29}
!29 = distinct !{!29, !30, !"_RNvMs4_NtCsavYRE6QVrtQ_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsk5rB8Fy7VC3_12two_variants: %self"}
!30 = distinct !{!30, !"_RNvMs4_NtCsavYRE6QVrtQ_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsk5rB8Fy7VC3_12two_variants"}
!31 = !{i64 0, i64 2}
!32 = !{i64 0, i64 -9223372036854775807}
!33 = !{!34}
!34 = distinct !{!34, !35, !"_RNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0Csk5rB8Fy7VC3_12two_variants: %_1"}
!35 = distinct !{!35, !"_RNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0Csk5rB8Fy7VC3_12two_variants"}
!36 = !{!37}
!37 = distinct !{!37, !38, !"_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtB6_6string6StringEINtB2_18SpecFromIterNestedB11_NtNtCsefmIBSMl6ne_3std3env4ArgsE9from_iterCsk5rB8Fy7VC3_12two_variants: %_0"}
!38 = distinct !{!38, !"_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtB6_6string6StringEINtB2_18SpecFromIterNestedB11_NtNtCsefmIBSMl6ne_3std3env4ArgsE9from_iterCsk5rB8Fy7VC3_12two_variants"}
!39 = !{!40}
!40 = distinct !{!40, !38, !"_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtB6_6string6StringEINtB2_18SpecFromIterNestedB11_NtNtCsefmIBSMl6ne_3std3env4ArgsE9from_iterCsk5rB8Fy7VC3_12two_variants: %iterator"}
!41 = !{!37, !40}
!42 = !{!43}
!43 = distinct !{!43, !44, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env4ArgsECsk5rB8Fy7VC3_12two_variants: %_1"}
!44 = distinct !{!44, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env4ArgsECsk5rB8Fy7VC3_12two_variants"}
!45 = !{!46}
!46 = distinct !{!46, !47, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env6ArgsOsECsk5rB8Fy7VC3_12two_variants: %_1"}
!47 = distinct !{!47, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env6ArgsOsECsk5rB8Fy7VC3_12two_variants"}
!48 = !{!49}
!49 = distinct !{!49, !50, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtNtCsefmIBSMl6ne_3std3sys4args6common4ArgsECsk5rB8Fy7VC3_12two_variants: %_1"}
!50 = distinct !{!50, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtNtCsefmIBSMl6ne_3std3sys4args6common4ArgsECsk5rB8Fy7VC3_12two_variants"}
!51 = !{!52}
!52 = distinct !{!52, !53, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtNtCsavYRE6QVrtQ_5alloc3vec9into_iter8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringEECsk5rB8Fy7VC3_12two_variants: %_1"}
!53 = distinct !{!53, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtNtCsavYRE6QVrtQ_5alloc3vec9into_iter8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringEECsk5rB8Fy7VC3_12two_variants"}
!54 = !{!55}
!55 = distinct !{!55, !56, !"_RNvXse_NtNtCsavYRE6QVrtQ_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringENtNtNtCshhmRwEtsTQ2_4core3ops4drop4Drop4dropCsk5rB8Fy7VC3_12two_variants: %self"}
!56 = distinct !{!56, !"_RNvXse_NtNtCsavYRE6QVrtQ_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringENtNtNtCshhmRwEtsTQ2_4core3ops4drop4Drop4dropCsk5rB8Fy7VC3_12two_variants"}
!57 = !{!55, !52, !49, !46, !43, !40}
!58 = !{!59}
!59 = distinct !{!59, !60, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants: %_1.0"}
!60 = distinct !{!60, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants"}
!61 = !{!55, !52, !49, !46, !43, !37}
!62 = !{!59, !55, !52, !49, !46, !43, !37}
!63 = !{!"branch_weights", i32 2002, i32 2000}
!64 = !{!65, !37}
!65 = distinct !{!65, !66, !"_RNvMs4_NtCsavYRE6QVrtQ_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsk5rB8Fy7VC3_12two_variants: %_0"}
!66 = distinct !{!66, !"_RNvMs4_NtCsavYRE6QVrtQ_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsk5rB8Fy7VC3_12two_variants"}
!67 = !{!68}
!68 = distinct !{!68, !69, !"_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec11spec_extendINtB4_3VecNtNtB6_6string6StringEINtB2_10SpecExtendBR_NtNtCsefmIBSMl6ne_3std3env4ArgsE11spec_extendCsk5rB8Fy7VC3_12two_variants: %self"}
!69 = distinct !{!69, !"_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec11spec_extendINtB4_3VecNtNtB6_6string6StringEINtB2_10SpecExtendBR_NtNtCsefmIBSMl6ne_3std3env4ArgsE11spec_extendCsk5rB8Fy7VC3_12two_variants"}
!70 = !{!71}
!71 = distinct !{!71, !69, !"_RNvXNtNtCsavYRE6QVrtQ_5alloc3vec11spec_extendINtB4_3VecNtNtB6_6string6StringEINtB2_10SpecExtendBR_NtNtCsefmIBSMl6ne_3std3env4ArgsE11spec_extendCsk5rB8Fy7VC3_12two_variants: %iter"}
!72 = !{!73}
!73 = distinct !{!73, !74, !"_RINvMsj_NtCsavYRE6QVrtQ_5alloc3vecINtB6_3VecNtNtB8_6string6StringE16extend_desugaredNtNtCsefmIBSMl6ne_3std3env4ArgsECsk5rB8Fy7VC3_12two_variants: %self"}
!74 = distinct !{!74, !"_RINvMsj_NtCsavYRE6QVrtQ_5alloc3vecINtB6_3VecNtNtB8_6string6StringE16extend_desugaredNtNtCsefmIBSMl6ne_3std3env4ArgsECsk5rB8Fy7VC3_12two_variants"}
!75 = !{!76}
!76 = distinct !{!76, !74, !"_RINvMsj_NtCsavYRE6QVrtQ_5alloc3vecINtB6_3VecNtNtB8_6string6StringE16extend_desugaredNtNtCsefmIBSMl6ne_3std3env4ArgsECsk5rB8Fy7VC3_12two_variants: %iterator"}
!77 = !{!73, !76, !68, !71, !37, !40}
!78 = !{!73, !68, !37}
!79 = !{!73, !68}
!80 = !{!76, !71, !37, !40}
!81 = !{!82}
!82 = distinct !{!82, !83, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env4ArgsECsk5rB8Fy7VC3_12two_variants: %_1"}
!83 = distinct !{!83, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env4ArgsECsk5rB8Fy7VC3_12two_variants"}
!84 = !{!85}
!85 = distinct !{!85, !86, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env6ArgsOsECsk5rB8Fy7VC3_12two_variants: %_1"}
!86 = distinct !{!86, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env6ArgsOsECsk5rB8Fy7VC3_12two_variants"}
!87 = !{!88}
!88 = distinct !{!88, !89, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtNtCsefmIBSMl6ne_3std3sys4args6common4ArgsECsk5rB8Fy7VC3_12two_variants: %_1"}
!89 = distinct !{!89, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtNtCsefmIBSMl6ne_3std3sys4args6common4ArgsECsk5rB8Fy7VC3_12two_variants"}
!90 = !{!91}
!91 = distinct !{!91, !92, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtNtCsavYRE6QVrtQ_5alloc3vec9into_iter8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringEECsk5rB8Fy7VC3_12two_variants: %_1"}
!92 = distinct !{!92, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtNtCsavYRE6QVrtQ_5alloc3vec9into_iter8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringEECsk5rB8Fy7VC3_12two_variants"}
!93 = !{!94}
!94 = distinct !{!94, !95, !"_RNvXse_NtNtCsavYRE6QVrtQ_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringENtNtNtCshhmRwEtsTQ2_4core3ops4drop4Drop4dropCsk5rB8Fy7VC3_12two_variants: %self"}
!95 = distinct !{!95, !"_RNvXse_NtNtCsavYRE6QVrtQ_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringENtNtNtCshhmRwEtsTQ2_4core3ops4drop4Drop4dropCsk5rB8Fy7VC3_12two_variants"}
!96 = !{!94, !91, !88, !85, !82, !76, !71}
!97 = !{!73, !68, !37, !40}
!98 = !{!99}
!99 = distinct !{!99, !100, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants: %_1.0"}
!100 = distinct !{!100, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants"}
!101 = !{!94, !91, !88, !85, !82, !73, !68, !37}
!102 = !{!99, !94, !91, !88, !85, !82, !73, !68, !37}
!103 = !{!76, !71}
!104 = !{!105}
!105 = distinct !{!105, !106, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env4ArgsECsk5rB8Fy7VC3_12two_variants: %_1"}
!106 = distinct !{!106, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env4ArgsECsk5rB8Fy7VC3_12two_variants"}
!107 = !{!108}
!108 = distinct !{!108, !109, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env6ArgsOsECsk5rB8Fy7VC3_12two_variants: %_1"}
!109 = distinct !{!109, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtCsefmIBSMl6ne_3std3env6ArgsOsECsk5rB8Fy7VC3_12two_variants"}
!110 = !{!111}
!111 = distinct !{!111, !112, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtNtCsefmIBSMl6ne_3std3sys4args6common4ArgsECsk5rB8Fy7VC3_12two_variants: %_1"}
!112 = distinct !{!112, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeNtNtNtNtCsefmIBSMl6ne_3std3sys4args6common4ArgsECsk5rB8Fy7VC3_12two_variants"}
!113 = !{!114}
!114 = distinct !{!114, !115, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtNtCsavYRE6QVrtQ_5alloc3vec9into_iter8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringEECsk5rB8Fy7VC3_12two_variants: %_1"}
!115 = distinct !{!115, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtNtCsavYRE6QVrtQ_5alloc3vec9into_iter8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringEECsk5rB8Fy7VC3_12two_variants"}
!116 = !{!117}
!117 = distinct !{!117, !118, !"_RNvXse_NtNtCsavYRE6QVrtQ_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringENtNtNtCshhmRwEtsTQ2_4core3ops4drop4Drop4dropCsk5rB8Fy7VC3_12two_variants: %self"}
!118 = distinct !{!118, !"_RNvXse_NtNtCsavYRE6QVrtQ_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringENtNtNtCshhmRwEtsTQ2_4core3ops4drop4Drop4dropCsk5rB8Fy7VC3_12two_variants"}
!119 = !{!117, !114, !111, !108, !105}
!120 = !{!121}
!121 = distinct !{!121, !122, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants: %_1.0"}
!122 = distinct !{!122, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtNtCsefmIBSMl6ne_3std3ffi6os_str8OsStringECsk5rB8Fy7VC3_12two_variants"}
!123 = !{!117, !114, !111, !108, !105, !37}
!124 = !{!121, !117, !114, !111, !108, !105, !37}
!125 = !{!126}
!126 = distinct !{!126, !127, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc3vec3VecNtNtBL_6string6StringEECsk5rB8Fy7VC3_12two_variants: %_1"}
!127 = distinct !{!127, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeINtNtCsavYRE6QVrtQ_5alloc3vec3VecNtNtBL_6string6StringEECsk5rB8Fy7VC3_12two_variants"}
!128 = !{!129}
!129 = distinct !{!129, !130, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants: %_1.0"}
!130 = distinct !{!130, !"_RINvNtCshhmRwEtsTQ2_4core3ptr13drop_in_placeSNtNtCsavYRE6QVrtQ_5alloc6string6StringECsk5rB8Fy7VC3_12two_variants"}
!131 = !{!129, !126}
!132 = !{!133}
!133 = distinct !{!133, !134, !"_RNvMsv_NtCshhmRwEtsTQ2_4core3numj16from_ascii_radix: argument 1"}
!134 = distinct !{!134, !"_RNvMsv_NtCshhmRwEtsTQ2_4core3numj16from_ascii_radix"}
!135 = !{!136}
!136 = distinct !{!136, !134, !"_RNvMsv_NtCshhmRwEtsTQ2_4core3numj16from_ascii_radix: %_0"}
!137 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!138 = !{!139}
!139 = distinct !{!139, !140, !"_RNvMNtCshhmRwEtsTQ2_4core6resultINtB2_6ResultjNtNtNtB4_3num5error13ParseIntErrorE6unwrapCsk5rB8Fy7VC3_12two_variants: %self"}
!140 = distinct !{!140, !"_RNvMNtCshhmRwEtsTQ2_4core6resultINtB2_6ResultjNtNtNtB4_3num5error13ParseIntErrorE6unwrapCsk5rB8Fy7VC3_12two_variants"}
!141 = !{i64 1}
!142 = !{i8 0, i8 5}
