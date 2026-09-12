; ModuleID = 'visitor_decl.e28b6df344d9601f-cgu.0'
source_filename = "visitor_decl.e28b6df344d9601f-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@alloc_3f95fa5fe64159c0ca66f05c35f35619 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"meow\0A" }>, align 1
@alloc_000bc512779c9a763a8aa84ee52b6421 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_3f95fa5fe64159c0ca66f05c35f35619, [8 x i8] c"\05\00\00\00\00\00\00\00" }>, align 8
@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hb5495db68bd88c0dE", ptr @"_ZN58_$LT$visitor_decl..Cat$u20$as$u20$visitor_decl..Animal$GT$5speak17h7a4729798e14e403E", ptr @"_ZN58_$LT$visitor_decl..Cat$u20$as$u20$visitor_decl..Animal$GT$5visit17he353a6a328e6c372E", ptr @"_ZN58_$LT$visitor_decl..Cat$u20$as$u20$visitor_decl..Animal$GT$11get_type_id17hb05720de519fdd93E" }>, align 8
@alloc_2b182a67d4f9402d603ef3e7f72e2431 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"woof\0A" }>, align 1
@alloc_ec4fa215300b117d5ab20e2368000be2 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_2b182a67d4f9402d603ef3e7f72e2431, [8 x i8] c"\05\00\00\00\00\00\00\00" }>, align 8
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h813b384b8beaf007E", ptr @"_ZN58_$LT$visitor_decl..Dog$u20$as$u20$visitor_decl..Animal$GT$5speak17h2d33263061ead511E", ptr @"_ZN58_$LT$visitor_decl..Dog$u20$as$u20$visitor_decl..Animal$GT$5visit17h3fff94aa131ecb55E", ptr @"_ZN58_$LT$visitor_decl..Dog$u20$as$u20$visitor_decl..Animal$GT$11get_type_id17h09392c0088ca9cbfE" }>, align 8

; <T as core::any::Any>::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none) uwtable
define internal { i64, i64 } @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h813b384b8beaf007E"(ptr noalias nocapture nonnull readonly align 1 %self) unnamed_addr #0 {
start:
  ret { i64, i64 } { i64 4070058139456573159, i64 8903628646065446340 }
}

; <T as core::any::Any>::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none) uwtable
define internal { i64, i64 } @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hb5495db68bd88c0dE"(ptr noalias nocapture nonnull readonly align 1 %self) unnamed_addr #0 {
start:
  ret { i64, i64 } { i64 -549753294091840377, i64 5110801219166074874 }
}

; <visitor_decl::Cat as visitor_decl::Animal>::speak
; Function Attrs: nonlazybind uwtable
define void @"_ZN58_$LT$visitor_decl..Cat$u20$as$u20$visitor_decl..Animal$GT$5speak17h7a4729798e14e403E"(ptr noalias nocapture nonnull readonly align 1 %self) unnamed_addr #1 {
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
  call void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_3)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3)
  ret void
}

; <visitor_decl::Cat as visitor_decl::Animal>::visit
; Function Attrs: nonlazybind uwtable
define void @"_ZN58_$LT$visitor_decl..Cat$u20$as$u20$visitor_decl..Animal$GT$5visit17he353a6a328e6c372E"(ptr noalias noundef nonnull readonly align 1 %self, ptr noundef nonnull align 1 %av.0, ptr noalias nocapture noundef readonly align 8 dereferenceable(32) %av.1) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %av.1, i64 24
  %1 = load ptr, ptr %0, align 8, !invariant.load !3, !nonnull !3
  tail call void %1(ptr noundef nonnull align 1 %av.0, ptr noundef nonnull align 1 %self, ptr noalias noundef nonnull readonly align 8 dereferenceable(56) @vtable.0)
  ret void
}

; <visitor_decl::Cat as visitor_decl::Animal>::get_type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none) uwtable
define internal noundef i32 @"_ZN58_$LT$visitor_decl..Cat$u20$as$u20$visitor_decl..Animal$GT$11get_type_id17hb05720de519fdd93E"(ptr noalias nocapture nonnull readonly align 1 %self) unnamed_addr #0 {
start:
  ret i32 0
}

; <visitor_decl::Dog as visitor_decl::Animal>::speak
; Function Attrs: nonlazybind uwtable
define void @"_ZN58_$LT$visitor_decl..Dog$u20$as$u20$visitor_decl..Animal$GT$5speak17h2d33263061ead511E"(ptr noalias nocapture nonnull readonly align 1 %self) unnamed_addr #1 {
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
  call void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_3)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3)
  ret void
}

; <visitor_decl::Dog as visitor_decl::Animal>::visit
; Function Attrs: nonlazybind uwtable
define void @"_ZN58_$LT$visitor_decl..Dog$u20$as$u20$visitor_decl..Animal$GT$5visit17h3fff94aa131ecb55E"(ptr noalias noundef nonnull readonly align 1 %self, ptr noundef nonnull align 1 %av.0, ptr noalias nocapture noundef readonly align 8 dereferenceable(32) %av.1) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %av.1, i64 24
  %1 = load ptr, ptr %0, align 8, !invariant.load !3, !nonnull !3
  tail call void %1(ptr noundef nonnull align 1 %av.0, ptr noundef nonnull align 1 %self, ptr noalias noundef nonnull readonly align 8 dereferenceable(56) @vtable.1)
  ret void
}

; <visitor_decl::Dog as visitor_decl::Animal>::get_type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none) uwtable
define internal noundef i32 @"_ZN58_$LT$visitor_decl..Dog$u20$as$u20$visitor_decl..Animal$GT$11get_type_id17h09392c0088ca9cbfE"(ptr noalias nocapture nonnull readonly align 1 %self) unnamed_addr #0 {
start:
  ret i32 1
}

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef align 8 dereferenceable(48)) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #2

attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind willreturn memory(none) uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.87.0-nightly (ecade534c 2025-03-14)"}
!3 = !{}
