; ModuleID = 'visitor_use.441f637470b8511d-cgu.0'
source_filename = "visitor_use.441f637470b8511d-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::sync::atomic::AtomicPtr<core::ffi::c_void>" = type { ptr }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h061e5e99f3d0d124E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h38b0e820d8cd1c48E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h38b0e820d8cd1c48E" }>, align 8
@alloc_85fc59111fd0cef7ef4093da3840b035 = private unnamed_addr constant <{ [8 x i8] }> zeroinitializer, align 1
@_ZN9getrandom8backends27linux_android_with_fallback12GETRANDOM_FN17hf208bec09c9234d7E = external local_unnamed_addr global %"core::sync::atomic::AtomicPtr<core::ffi::c_void>"
@alloc_c465275dc21ccddc5267a3b99fb75390 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"grrr\0A" }>, align 1
@alloc_d872c08bee5edae1ced83c18688a55f8 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_c465275dc21ccddc5267a3b99fb75390, [8 x i8] c"\05\00\00\00\00\00\00\00" }>, align 8
@vtable.4 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN76_$LT$visitor_use..SpeakBetterDogs$u20$as$u20$visitor_decl..AnimalVisitor$GT$7receive17hf779da33064c8beeE" }>, align 8

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden noundef i64 @_ZN3std2rt10lang_start17h3eaea2de9c7bfa26E(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7)
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_ZN3std2rt19lang_start_internal17h179a86731aa5c4aaE(ptr noundef nonnull align 1 %_7, ptr noalias noundef nonnull readonly align 8 dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7)
  ret i64 %_0
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h38b0e820d8cd1c48E"(ptr noalias nocapture noundef readonly align 8 dereferenceable(8) %_1) unnamed_addr #1 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf840ce6bc739f00aE(ptr noundef nonnull %_4)
  ret i32 0
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf840ce6bc739f00aE(ptr nocapture noundef nonnull readonly %f) unnamed_addr #2 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #11, !srcloc !5
  ret void
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h061e5e99f3d0d124E"(ptr nocapture noundef readonly %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17hf840ce6bc739f00aE(ptr noundef nonnull readonly %0), !noalias !6
  ret i32 0
}

; rand::rngs::reseeding::ReseedingCore<R,Rsdr>::reseed_and_generate
; Function Attrs: noinline nonlazybind uwtable
define internal fastcc void @"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17hd15adc99b7cb5ccbE"(ptr noalias noundef nonnull align 16 dereferenceable(64) %self, ptr noalias noundef nonnull align 4 dereferenceable(256) %results) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %_9.i.i = alloca [32 x i8], align 1
  %seed.i.i = alloca [32 x i8], align 1
  %_2.i = alloca [64 x i8], align 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9)
  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %_2.i), !noalias !9
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %seed.i.i), !noalias !12
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 1 dereferenceable(32) %seed.i.i, i8 0, i64 32, i1 false), !alias.scope !15, !noalias !12
  %0 = load atomic ptr, ptr @_ZN9getrandom8backends27linux_android_with_fallback12GETRANDOM_FN17hf208bec09c9234d7E acquire, align 8, !noalias !18
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb7.i.i.i.i.i, label %bb1.i.i.i.i.i, !prof !25

bb7.i.i.i.i.i:                                    ; preds = %start
; call getrandom::backends::linux_android_with_fallback::init
  %2 = tail call noundef nonnull ptr @_ZN9getrandom8backends27linux_android_with_fallback4init17h6d296174b3adafebE(), !noalias !18
  br label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %bb7.i.i.i.i.i, %start
  %fptr.sroa.0.0.i.i.i.i.i = phi ptr [ %2, %bb7.i.i.i.i.i ], [ %0, %start ]
  %_7.i.i.i.i.i = icmp eq ptr %fptr.sroa.0.0.i.i.i.i.i, inttoptr (i64 -1 to ptr)
  br i1 %_7.i.i.i.i.i, label %"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17h80ac1c8444049ac1E.exit.i.i", label %bb3.i.i.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i.i.i, %bb13.i.i.i.i.i.i
  %buf.sroa.0.012.i.i.i.i.i.i = phi ptr [ %buf.sroa.0.1.i.i.i.i.i.i, %bb13.i.i.i.i.i.i ], [ %seed.i.i, %bb1.i.i.i.i.i ]
  %buf.sroa.5.011.i.i.i.i.i.i = phi i64 [ %buf.sroa.5.1.i.i.i.i.i.i, %bb13.i.i.i.i.i.i ], [ 32, %bb1.i.i.i.i.i ]
  %_0.i.i.i.i.i.i.i = call noundef i64 %fptr.sroa.0.0.i.i.i.i.i(ptr noundef nonnull align 1 %buf.sroa.0.012.i.i.i.i.i.i, i64 noundef range(i64 1, 0) %buf.sroa.5.011.i.i.i.i.i.i, i32 noundef 0) #11, !noalias !26
  %_9.i.i.i.i.i.i = icmp sgt i64 %_0.i.i.i.i.i.i.i, 0
  br i1 %_9.i.i.i.i.i.i, label %bb19.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i

bb8.i.i.i.i.i.i:                                  ; preds = %bb3.i.i.i.i.i.i
  %3 = icmp eq i64 %_0.i.i.i.i.i.i.i, -1
  br i1 %3, label %bb6.i.i.i.i.i.i, label %bb4.i

bb6.i.i.i.i.i.i:                                  ; preds = %bb8.i.i.i.i.i.i
; call getrandom::backends::use_file::util_libc::last_os_error
  %err.i.i.i.i.i.i = call noundef i32 @_ZN9getrandom8backends8use_file9util_libc13last_os_error17hb85a3f52908be3f8E(), !noalias !26
  %.not.i.i.i.i.i.i = icmp eq i32 %err.i.i.i.i.i.i, -4
  br i1 %.not.i.i.i.i.i.i, label %bb13.i.i.i.i.i.i, label %bb4.i

bb13.i.i.i.i.i.i:                                 ; preds = %bb21.i.i.i.i.i.i, %bb6.i.i.i.i.i.i
  %buf.sroa.5.1.i.i.i.i.i.i = phi i64 [ %_38.i.i.i.i.i.i, %bb21.i.i.i.i.i.i ], [ %buf.sroa.5.011.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ]
  %buf.sroa.0.1.i.i.i.i.i.i = phi ptr [ %_39.i.i.i.i.i.i, %bb21.i.i.i.i.i.i ], [ %buf.sroa.0.012.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ]
  %4 = icmp eq i64 %buf.sroa.5.1.i.i.i.i.i.i, 0
  br i1 %4, label %bb5.i, label %bb3.i.i.i.i.i.i

bb19.i.i.i.i.i.i:                                 ; preds = %bb3.i.i.i.i.i.i
  %_37.i.i.i.i.i.i = icmp ult i64 %buf.sroa.5.011.i.i.i.i.i.i, %_0.i.i.i.i.i.i.i
  br i1 %_37.i.i.i.i.i.i, label %bb4.i, label %bb21.i.i.i.i.i.i

bb21.i.i.i.i.i.i:                                 ; preds = %bb19.i.i.i.i.i.i
  %_38.i.i.i.i.i.i = sub nuw i64 %buf.sroa.5.011.i.i.i.i.i.i, %_0.i.i.i.i.i.i.i
  %_39.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %buf.sroa.0.012.i.i.i.i.i.i, i64 %_0.i.i.i.i.i.i.i
  br label %bb13.i.i.i.i.i.i

"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17h80ac1c8444049ac1E.exit.i.i": ; preds = %bb1.i.i.i.i.i
; call getrandom::backends::linux_android_with_fallback::use_file_fallback
  %5 = call noundef i32 @_ZN9getrandom8backends27linux_android_with_fallback17use_file_fallback17hcd79e038d1f5293dE(ptr noalias noundef nonnull align 1 %seed.i.i, i64 noundef 32), !noalias !12
  %.not.i.i = icmp eq i32 %5, 0
  br i1 %.not.i.i, label %bb5.i, label %bb4.i

bb4.i:                                            ; preds = %bb19.i.i.i.i.i.i, %bb6.i.i.i.i.i.i, %bb8.i.i.i.i.i.i, %"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17h80ac1c8444049ac1E.exit.i.i"
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %seed.i.i), !noalias !12
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 48
  %_8.pre = load i64, ptr %.phi.trans.insert, align 16
  br label %"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17hc4a41e71dd820263E.exit"

bb5.i:                                            ; preds = %bb13.i.i.i.i.i.i, %"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17h80ac1c8444049ac1E.exit.i.i"
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9.i.i), !noalias !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(32) %_9.i.i, ptr noundef nonnull align 1 dereferenceable(32) %seed.i.i, i64 32, i1 false), !noalias !12
  %6 = getelementptr inbounds nuw i8, ptr %_2.i, i64 16
; call rand_chacha::guts::init_chacha
  call void @_ZN11rand_chacha4guts11init_chacha17h7afba8a5a87201a0E(ptr noalias nocapture noundef nonnull sret([48 x i8]) align 16 dereferenceable(48) %6, ptr noalias noundef nonnull readonly align 1 dereferenceable(32) %_9.i.i, ptr noalias noundef nonnull readonly align 1 @alloc_85fc59111fd0cef7ef4093da3840b035, i64 noundef 8), !noalias !9
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9.i.i), !noalias !12
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %seed.i.i), !noalias !12
  %_6.i = getelementptr inbounds nuw i8, ptr %self, i64 48
  %_3.i.i = load i64, ptr %_6.i, align 16, !alias.scope !9, !noalias !29, !noundef !4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(64) %self, ptr noundef nonnull align 16 dereferenceable(48) %6, i64 48, i1 false)
  br label %"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17hc4a41e71dd820263E.exit"

"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17hc4a41e71dd820263E.exit": ; preds = %bb4.i, %bb5.i
  %_8 = phi i64 [ %_8.pre, %bb4.i ], [ %_3.i.i, %bb5.i ]
  call void @llvm.lifetime.end.p0(i64 64, ptr nonnull %_2.i), !noalias !9
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 56
  %8 = add i64 %_8, -256
  store i64 %8, ptr %7, align 8
; call rand_chacha::guts::refill_wide
  call void @_ZN11rand_chacha4guts11refill_wide17hb00c8dd458bdc5c7E(ptr noalias noundef nonnull align 16 dereferenceable(48) %self, i32 noundef 6, ptr noalias noundef nonnull align 4 dereferenceable(256) %results)
  ret void
}

; <visitor_use::SpeakBetterDogs as visitor_decl::AnimalVisitor>::receive
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN76_$LT$visitor_use..SpeakBetterDogs$u20$as$u20$visitor_decl..AnimalVisitor$GT$7receive17hf779da33064c8beeE"(ptr noalias nocapture nonnull readonly align 1 %self, ptr noundef nonnull align 1 %a.0, ptr noalias nocapture noundef readonly align 8 dereferenceable(56) %a.1) unnamed_addr #0 {
start:
  %_5 = alloca [48 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %a.1, i64 48
  %1 = load ptr, ptr %0, align 8, !invariant.load !4, !nonnull !4
  %_3 = tail call noundef i32 %1(ptr noundef nonnull align 1 %a.0)
  %2 = icmp eq i32 %_3, 1
  br i1 %2, label %bb2, label %bb4

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_5)
  store ptr @alloc_d872c08bee5edae1ced83c18688a55f8, ptr %_5, align 8
  %3 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  store i64 1, ptr %3, align 8
  %4 = getelementptr inbounds nuw i8, ptr %_5, i64 32
  store ptr null, ptr %4, align 8
  %5 = getelementptr inbounds nuw i8, ptr %_5, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %5, align 8
  %6 = getelementptr inbounds nuw i8, ptr %_5, i64 24
  store i64 0, ptr %6, align 8
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_5)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_5)
  br label %bb5

bb4:                                              ; preds = %start
  %7 = getelementptr inbounds nuw i8, ptr %a.1, i64 32
  %8 = load ptr, ptr %7, align 8, !invariant.load !4, !nonnull !4
  tail call void %8(ptr noundef nonnull align 1 %a.0)
  br label %bb5

bb5:                                              ; preds = %bb4, %bb2
  ret void
}

; visitor_use::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN11visitor_use4main17h0335cc742f9b58a1E() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_4 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_4)
; call rand::rngs::thread::rng
  %0 = tail call noundef nonnull ptr @_ZN4rand4rngs6thread3rng17hfada0d982bfe9e01E()
  store ptr %0, ptr %_4, align 8
  %_5.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 16
  %1 = getelementptr inbounds nuw i8, ptr %0, i64 336
  %_3.i.i.i.i.i.i.i = load i64, ptr %1, align 16, !noalias !4, !noundef !4
  %_2.i.i.i.i.i.i.i = icmp ugt i64 %_3.i.i.i.i.i.i.i, 63
  br i1 %_2.i.i.i.i.i.i.i, label %bb2.i.i.i10.i.i.i.i, label %bb2

bb2.i.i.i10.i.i.i.i:                              ; preds = %start
  %_9.i.i.i.i11.i.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 272
  %2 = getelementptr inbounds nuw i8, ptr %0, i64 328
  %_4.i.i.i.i.i12.i.i.i.i = load i64, ptr %2, align 8, !alias.scope !33, !noalias !40, !noundef !4
  %_3.i.i.i.i.i13.i.i.i.i = icmp slt i64 %_4.i.i.i.i.i12.i.i.i.i, 1
  br i1 %_3.i.i.i.i.i13.i.i.i.i, label %bb1.i.i.i.i.i15.i.i.i.i, label %bb3.i.i.i.i.i14.i.i.i.i

bb3.i.i.i.i.i14.i.i.i.i:                          ; preds = %bb2.i.i.i10.i.i.i.i
  %3 = add nsw i64 %_4.i.i.i.i.i12.i.i.i.i, -256
  store i64 %3, ptr %2, align 8, !alias.scope !33, !noalias !40
; invoke rand_chacha::guts::refill_wide
  invoke void @_ZN11rand_chacha4guts11refill_wide17hb00c8dd458bdc5c7E(ptr noalias noundef nonnull align 16 dereferenceable(64) %_9.i.i.i.i11.i.i.i.i, i32 noundef 6, ptr noalias noundef nonnull align 16 dereferenceable(336) %_5.i.i.i.i.i.i.i)
          to label %bb2 unwind label %cleanup

bb1.i.i.i.i.i15.i.i.i.i:                          ; preds = %bb2.i.i.i10.i.i.i.i
; invoke rand::rngs::reseeding::ReseedingCore<R,Rsdr>::reseed_and_generate
  invoke fastcc void @"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17hd15adc99b7cb5ccbE"(ptr noalias noundef nonnull align 16 dereferenceable(64) %_9.i.i.i.i11.i.i.i.i, ptr noalias noundef nonnull align 16 dereferenceable(336) %_5.i.i.i.i.i.i.i)
          to label %bb2 unwind label %cleanup

cleanup:                                          ; preds = %bb1.i.i.i.i.i15.i.i.i.i, %bb3.i.i.i.i.i14.i.i.i.i
  %4 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.experimental.noalias.scope.decl(metadata !44)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !47)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !50)
  %_5.i.i.i = load ptr, ptr %_4, align 8, !alias.scope !53, !nonnull !4, !noundef !4
  %_8.i.i.i = load i64, ptr %_5.i.i.i, align 8, !noalias !53, !noundef !4
  %val.i.i.i = add i64 %_8.i.i.i, -1
  store i64 %val.i.i.i, ptr %_5.i.i.i, align 8, !noalias !53
  %5 = icmp eq i64 %val.i.i.i, 0
  br i1 %5, label %bb1.i.i.i, label %bb9

bb1.i.i.i:                                        ; preds = %cleanup
; invoke alloc::rc::Rc<T,A>::drop_slow
  invoke void @"_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h0182943288a25a80E"(ptr noalias noundef nonnull align 8 dereferenceable(8) %_4)
          to label %bb9 unwind label %terminate

bb2:                                              ; preds = %start, %bb3.i.i.i.i.i14.i.i.i.i, %bb1.i.i.i.i.i15.i.i.i.i
  %_10.i.i.i8.i.i.i.i = phi i64 [ %_3.i.i.i.i.i.i.i, %start ], [ 0, %bb3.i.i.i.i.i14.i.i.i.i ], [ 0, %bb1.i.i.i.i.i15.i.i.i.i ]
  %6 = getelementptr inbounds nuw i32, ptr %_5.i.i.i.i.i.i.i, i64 %_10.i.i.i8.i.i.i.i
  %value.i.i.i9.i.i.i.i = load i32, ptr %6, align 4, !alias.scope !54, !noalias !55, !noundef !4
  %7 = add nuw nsw i64 %_10.i.i.i8.i.i.i.i, 1
  store i64 %7, ptr %1, align 16, !alias.scope !54, !noalias !55
  tail call void @llvm.experimental.noalias.scope.decl(metadata !56)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !59)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !62)
  %_5.i.i.i3 = load ptr, ptr %_4, align 8, !alias.scope !65, !nonnull !4, !noundef !4
  %_8.i.i.i4 = load i64, ptr %_5.i.i.i3, align 8, !noalias !65, !noundef !4
  %val.i.i.i5 = add i64 %_8.i.i.i4, -1
  store i64 %val.i.i.i5, ptr %_5.i.i.i3, align 8, !noalias !65
  %8 = icmp eq i64 %val.i.i.i5, 0
  br i1 %8, label %bb1.i.i.i6, label %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17haf5126768f72e27fE.exit7"

bb1.i.i.i6:                                       ; preds = %bb2
; call alloc::rc::Rc<T,A>::drop_slow
  call void @"_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h0182943288a25a80E"(ptr noalias noundef nonnull align 8 dereferenceable(8) %_4)
  br label %"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17haf5126768f72e27fE.exit7"

"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17haf5126768f72e27fE.exit7": ; preds = %bb2, %bb1.i.i.i6
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_4)
  %9 = icmp sgt i32 %value.i.i.i9.i.i.i.i, -1
  %10 = select i1 %9, ptr @"_ZN58_$LT$visitor_decl..Cat$u20$as$u20$visitor_decl..Animal$GT$5visit17he353a6a328e6c372E", ptr @"_ZN58_$LT$visitor_decl..Dog$u20$as$u20$visitor_decl..Animal$GT$5visit17h3fff94aa131ecb55E"
  call void %10(ptr noundef nonnull align 1 inttoptr (i64 1 to ptr), ptr noundef nonnull align 1 inttoptr (i64 1 to ptr), ptr noalias noundef nonnull readonly align 8 dereferenceable(32) @vtable.4)
  ret void

terminate:                                        ; preds = %bb1.i.i.i
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h4898e2af8dc1f529E() #12
  unreachable

bb9:                                              ; preds = %cleanup, %bb1.i.i.i
  resume { ptr, i32 } %4
}

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #3

; Function Attrs: nounwind nonlazybind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef range(i32 1, 17), i64 noundef, ptr noundef, ptr noundef) unnamed_addr #4

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare noundef i64 @_ZN3std2rt19lang_start_internal17h179a86731aa5c4aaE(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; alloc::rc::Rc<T,A>::drop_slow
; Function Attrs: noinline nonlazybind uwtable
declare void @"_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h0182943288a25a80E"(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #2

; rand_chacha::guts::init_chacha
; Function Attrs: nonlazybind uwtable
declare void @_ZN11rand_chacha4guts11init_chacha17h7afba8a5a87201a0E(ptr dead_on_unwind noalias nocapture noundef writable sret([48 x i8]) align 16 dereferenceable(48), ptr noalias noundef readonly align 1 dereferenceable(32), ptr noalias noundef nonnull readonly align 1, i64 noundef) unnamed_addr #0

; rand_chacha::guts::refill_wide
; Function Attrs: nonlazybind uwtable
declare void @_ZN11rand_chacha4guts11refill_wide17hb00c8dd458bdc5c7E(ptr noalias noundef align 16 dereferenceable(48), i32 noundef, ptr noalias noundef align 4 dereferenceable(256)) unnamed_addr #0

; getrandom::backends::linux_android_with_fallback::init
; Function Attrs: cold noinline nonlazybind uwtable
declare noundef nonnull ptr @_ZN9getrandom8backends27linux_android_with_fallback4init17h6d296174b3adafebE() unnamed_addr #5

; getrandom::backends::linux_android_with_fallback::use_file_fallback
; Function Attrs: noinline nonlazybind uwtable
declare noundef i32 @_ZN9getrandom8backends27linux_android_with_fallback17use_file_fallback17hcd79e038d1f5293dE(ptr noalias noundef nonnull align 1, i64 noundef) unnamed_addr #2

; getrandom::backends::use_file::util_libc::last_os_error
; Function Attrs: nonlazybind uwtable
declare noundef range(i32 1, 0) i32 @_ZN9getrandom8backends8use_file9util_libc13last_os_error17hb85a3f52908be3f8E() unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef align 8 dereferenceable(48)) unnamed_addr #0

; rand::rngs::thread::rng
; Function Attrs: nonlazybind uwtable
declare noundef nonnull ptr @_ZN4rand4rngs6thread3rng17hfada0d982bfe9e01E() unnamed_addr #0

; <visitor_decl::Cat as visitor_decl::Animal>::visit
; Function Attrs: nonlazybind uwtable
declare void @"_ZN58_$LT$visitor_decl..Cat$u20$as$u20$visitor_decl..Animal$GT$5visit17he353a6a328e6c372E"(ptr noalias noundef nonnull readonly align 1, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 dereferenceable(32)) unnamed_addr #0

; <visitor_decl::Dog as visitor_decl::Animal>::visit
; Function Attrs: nonlazybind uwtable
declare void @"_ZN58_$LT$visitor_decl..Dog$u20$as$u20$visitor_decl..Animal$GT$5visit17h3fff94aa131ecb55E"(ptr noalias noundef nonnull readonly align 1, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 dereferenceable(32)) unnamed_addr #0

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind nonlazybind optsize uwtable
declare void @_ZN4core9panicking16panic_in_cleanup17h4898e2af8dc1f529E() unnamed_addr #6

; Function Attrs: nonlazybind
define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #7 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = sext i32 %0 to i64
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i)
  store ptr @_ZN11visitor_use4main17h0335cc742f9b58a1E, ptr %_7.i, align 8
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_ZN3std2rt19lang_start_internal17h179a86731aa5c4aaE(ptr noundef nonnull align 1 %_7.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(48) @vtable.0, i64 noundef %2, ptr noundef %1, i8 noundef 0)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i)
  %3 = trunc i64 %_0.i to i32
  ret i32 %3
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #8

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #8

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #9

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr nocapture writeonly, i8, i64, i1 immarg) #10

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #4 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #5 = { cold noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #6 = { cold minsize noinline noreturn nounwind nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #7 = { nonlazybind "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #8 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #9 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #10 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #11 = { nounwind }
attributes #12 = { cold noreturn nounwind }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{!"rustc version 1.87.0-nightly (ecade534c 2025-03-14)"}
!4 = !{}
!5 = !{i64 17106386592515592}
!6 = !{!7}
!7 = distinct !{!7, !8, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h38b0e820d8cd1c48E: %_1"}
!8 = distinct !{!8, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h38b0e820d8cd1c48E"}
!9 = !{!10}
!10 = distinct !{!10, !11, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17hc4a41e71dd820263E: %self"}
!11 = distinct !{!11, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17hc4a41e71dd820263E"}
!12 = !{!13, !10}
!13 = distinct !{!13, !14, !"_ZN9rand_core11SeedableRng12try_from_rng17h53577e744f438debE: %_0"}
!14 = distinct !{!14, !"_ZN9rand_core11SeedableRng12try_from_rng17h53577e744f438debE"}
!15 = !{!16}
!16 = distinct !{!16, !17, !"_ZN4core5array76_$LT$impl$u20$core..default..Default$u20$for$u20$$u5b$T$u3b$$u20$32$u5d$$GT$7default17h23edcffd8974c387E: %_0"}
!17 = distinct !{!17, !"_ZN4core5array76_$LT$impl$u20$core..default..Default$u20$for$u20$$u5b$T$u3b$$u20$32$u5d$$GT$7default17h23edcffd8974c387E"}
!18 = !{!19, !21, !23, !13, !10}
!19 = distinct !{!19, !20, !"_ZN9getrandom8backends27linux_android_with_fallback10fill_inner17h9d268e17ebe1020cE: %dest.0"}
!20 = distinct !{!20, !"_ZN9getrandom8backends27linux_android_with_fallback10fill_inner17h9d268e17ebe1020cE"}
!21 = distinct !{!21, !22, !"_ZN9getrandom4fill17h67b1a2f5a8ea0185E: %dest.0"}
!22 = distinct !{!22, !"_ZN9getrandom4fill17h67b1a2f5a8ea0185E"}
!23 = distinct !{!23, !24, !"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17h80ac1c8444049ac1E: %dest.0"}
!24 = distinct !{!24, !"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17h80ac1c8444049ac1E"}
!25 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!26 = !{!27, !13, !10}
!27 = distinct !{!27, !28, !"_ZN9getrandom8backends8use_file9util_libc14sys_fill_exact17hadcc2cba0310200aE: argument 1"}
!28 = distinct !{!28, !"_ZN9getrandom8backends8use_file9util_libc14sys_fill_exact17hadcc2cba0310200aE"}
!29 = !{!30, !32}
!30 = distinct !{!30, !31, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed28_$u7b$$u7b$closure$u7d$$u7d$17h87bf8ee02640d5edE: %_1"}
!31 = distinct !{!31, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed28_$u7b$$u7b$closure$u7d$$u7d$17h87bf8ee02640d5edE"}
!32 = distinct !{!32, !31, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed28_$u7b$$u7b$closure$u7d$$u7d$17h87bf8ee02640d5edE: %result"}
!33 = !{!34, !36, !38}
!34 = distinct !{!34, !35, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h7a0d7ed30134134fE: %self"}
!35 = distinct !{!35, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h7a0d7ed30134134fE"}
!36 = distinct !{!36, !37, !"_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h53307e0151505fc7E: %self"}
!37 = distinct !{!37, !"_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h53307e0151505fc7E"}
!38 = distinct !{!38, !39, !"_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h6857d40b541e4b82E: %self"}
!39 = distinct !{!39, !"_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h6857d40b541e4b82E"}
!40 = !{!41, !42}
!41 = distinct !{!41, !35, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h7a0d7ed30134134fE: %results"}
!42 = distinct !{!42, !43, !"_ZN68_$LT$rand..rngs..thread..ThreadRng$u20$as$u20$rand_core..RngCore$GT$8next_u3217h582253c5bbf76267E: %self"}
!43 = distinct !{!43, !"_ZN68_$LT$rand..rngs..thread..ThreadRng$u20$as$u20$rand_core..RngCore$GT$8next_u3217h582253c5bbf76267E"}
!44 = !{!45}
!45 = distinct !{!45, !46, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17haf5126768f72e27fE: %_1"}
!46 = distinct !{!46, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17haf5126768f72e27fE"}
!47 = !{!48}
!48 = distinct !{!48, !49, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17h124b1a95e0b8f0b7E: %_1"}
!49 = distinct !{!49, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17h124b1a95e0b8f0b7E"}
!50 = !{!51}
!51 = distinct !{!51, !52, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5d8225384da6960cE: %self"}
!52 = distinct !{!52, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5d8225384da6960cE"}
!53 = !{!51, !48, !45}
!54 = !{!38}
!55 = !{!42}
!56 = !{!57}
!57 = distinct !{!57, !58, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17haf5126768f72e27fE: %_1"}
!58 = distinct !{!58, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17haf5126768f72e27fE"}
!59 = !{!60}
!60 = distinct !{!60, !61, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17h124b1a95e0b8f0b7E: %_1"}
!61 = distinct !{!61, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17h124b1a95e0b8f0b7E"}
!62 = !{!63}
!63 = distinct !{!63, !64, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5d8225384da6960cE: %self"}
!64 = distinct !{!64, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5d8225384da6960cE"}
!65 = !{!63, !60, !57}
