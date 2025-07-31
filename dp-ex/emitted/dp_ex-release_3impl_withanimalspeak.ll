; ModuleID = 'dp_ex.5c91ebfbd02dee49-cgu.0'
source_filename = "dp_ex.5c91ebfbd02dee49-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::sync::atomic::AtomicPtr<core::ffi::c_void>" = type { ptr }

@vtable.0 = private constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17ha781119b1797b6ceE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h35042c385834d7a7E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h35042c385834d7a7E" }>, align 8, !dbg !0
@alloc_85fc59111fd0cef7ef4093da3840b035 = private unnamed_addr constant <{ [8 x i8] }> zeroinitializer, align 1
@_ZN9getrandom8backends27linux_android_with_fallback12GETRANDOM_FN17hecc8036d0c0d1ff5E = external local_unnamed_addr global %"core::sync::atomic::AtomicPtr<core::ffi::c_void>"
@alloc_2b182a67d4f9402d603ef3e7f72e2431 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"woof\0A" }>, align 1
@alloc_ec4fa215300b117d5ab20e2368000be2 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_2b182a67d4f9402d603ef3e7f72e2431, [8 x i8] c"\05\00\00\00\00\00\00\00" }>, align 8
@alloc_3f95fa5fe64159c0ca66f05c35f35619 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"meow\0A" }>, align 1
@alloc_000bc512779c9a763a8aa84ee52b6421 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_3f95fa5fe64159c0ca66f05c35f35619, [8 x i8] c"\05\00\00\00\00\00\00\00" }>, align 8
@alloc_97b892b1271fb0a1c9b8c8979f44bf66 = private unnamed_addr constant <{ [6 x i8] }> <{ [6 x i8] c"chirp\0A" }>, align 1
@alloc_aa690a7f645d07b0914dfbca7e9c692c = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_97b892b1271fb0a1c9b8c8979f44bf66, [8 x i8] c"\06\00\00\00\00\00\00\00" }>, align 8
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN45_$LT$dp_ex..Bird$u20$as$u20$dp_ex..Animal$GT$5speak17h900f4f573272ed1cE" }>, align 8, !dbg !24
@vtable.3 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17hfb6e878b47c18265E" }>, align 8, !dbg !34
@vtable.4 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17h95a71617b8cd4b02E" }>, align 8, !dbg !43
@__rustc_debug_gdb_scripts_section__ = linkonce_odr unnamed_addr constant [34 x i8] c"\01gdb_load_rust_pretty_printers.py\00", section ".debug_gdb_scripts", align 1

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden noundef i64 @_ZN3std2rt10lang_start17h4be3234073074386E(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 !dbg !103 {
start:
  %_7 = alloca [8 x i8], align 8
    #dbg_value(ptr %main, !111, !DIExpression(), !117)
    #dbg_value(i64 %argc, !112, !DIExpression(), !117)
    #dbg_value(ptr %argv, !113, !DIExpression(), !117)
    #dbg_value(i8 %sigpipe, !114, !DIExpression(), !117)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7), !dbg !118
  store ptr %main, ptr %_7, align 8, !dbg !118
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_ZN3std2rt19lang_start_internal17h179a86731aa5c4aaE(ptr noundef nonnull align 1 %_7, ptr noalias noundef nonnull readonly align 8 dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe), !dbg !119
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7), !dbg !120
  ret i64 %_0, !dbg !121
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h35042c385834d7a7E"(ptr noalias nocapture noundef readonly align 8 dereferenceable(8) %_1) unnamed_addr #1 !dbg !122 {
start:
    #dbg_value(ptr %_1, !128, !DIExpression(DW_OP_deref), !129)
  %_4 = load ptr, ptr %_1, align 8, !dbg !130, !nonnull !23, !noundef !23
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4d6c13392805e6c7E(ptr noundef nonnull %_4), !dbg !131
  ret i32 0, !dbg !132
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4d6c13392805e6c7E(ptr nocapture noundef nonnull readonly %f) unnamed_addr #2 !dbg !133 {
start:
    #dbg_declare(ptr undef, !141, !DIExpression(), !145)
    #dbg_value(ptr %f, !140, !DIExpression(), !146)
    #dbg_declare(ptr undef, !147, !DIExpression(), !154)
    #dbg_value(ptr %f, !156, !DIExpression(), !167)
    #dbg_declare(ptr undef, !163, !DIExpression(), !169)
  tail call void %f(), !dbg !169
  tail call void asm sideeffect "", "~{memory}"() #12, !dbg !170, !srcloc !171
  ret void, !dbg !172
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17ha781119b1797b6ceE"(ptr nocapture noundef readonly %_1) unnamed_addr #1 personality ptr @rust_eh_personality !dbg !173 {
start:
    #dbg_value(ptr %_1, !178, !DIExpression(), !182)
    #dbg_declare(ptr undef, !179, !DIExpression(), !183)
  %0 = load ptr, ptr %_1, align 8, !dbg !183, !nonnull !23, !noundef !23
    #dbg_value(ptr %0, !128, !DIExpression(), !184)
    #dbg_value(ptr %0, !190, !DIExpression(), !193)
    #dbg_declare(ptr undef, !191, !DIExpression(), !194)
    #dbg_value(ptr undef, !128, !DIExpression(DW_OP_deref), !184)
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4d6c13392805e6c7E(ptr noundef nonnull readonly %0), !dbg !195, !noalias !196
  ret i32 0, !dbg !183
}

; rand::rngs::reseeding::ReseedingCore<R,Rsdr>::reseed_and_generate
; Function Attrs: noinline nonlazybind uwtable
define internal fastcc void @"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17ha6228666c915f093E"(ptr noalias noundef nonnull align 16 dereferenceable(64) %self, ptr noalias noundef nonnull align 4 dereferenceable(256) %0) unnamed_addr #2 personality ptr @rust_eh_personality !dbg !199 {
start:
  %_9.i.i = alloca [32 x i8], align 1
  %seed.i.i = alloca [32 x i8], align 1
  %self1.i = alloca [64 x i8], align 16
    #dbg_value(ptr %0, !268, !DIExpression(), !292)
    #dbg_value(ptr %self, !267, !DIExpression(), !292)
    #dbg_value(i64 256, !269, !DIExpression(), !293)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !294), !dbg !297
    #dbg_value(ptr %self, !298, !DIExpression(), !320)
    #dbg_declare(ptr %self1.i, !322, !DIExpression(), !362)
    #dbg_declare(ptr poison, !358, !DIExpression(), !364)
  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %self1.i), !dbg !365, !noalias !294
    #dbg_value(ptr poison, !366, !DIExpression(), !404)
    #dbg_declare(ptr %seed.i.i, !374, !DIExpression(), !406)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %seed.i.i), !dbg !407, !noalias !408
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 1 dereferenceable(32) %seed.i.i, i8 0, i64 32, i1 false), !dbg !411, !alias.scope !421, !noalias !408
    #dbg_value(ptr poison, !424, !DIExpression(), !437)
    #dbg_value(ptr %seed.i.i, !436, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !437)
    #dbg_value(i64 32, !436, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !437)
    #dbg_value(ptr %seed.i.i, !439, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !477)
    #dbg_value(i64 32, !439, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !477)
    #dbg_value(ptr %seed.i.i, !479, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !518)
    #dbg_value(i64 32, !479, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !518)
    #dbg_value(ptr undef, !520, !DIExpression(), !560)
    #dbg_value(ptr undef, !585, !DIExpression(), !595)
    #dbg_value(ptr %seed.i.i, !570, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !597)
    #dbg_value(i64 32, !570, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !597)
    #dbg_value(i8 2, !598, !DIExpression(), !616)
    #dbg_value(ptr @_ZN9getrandom8backends27linux_android_with_fallback12GETRANDOM_FN17hecc8036d0c0d1ff5E, !615, !DIExpression(), !616)
    #dbg_value(ptr @_ZN9getrandom8backends27linux_android_with_fallback12GETRANDOM_FN17hecc8036d0c0d1ff5E, !618, !DIExpression(), !625)
    #dbg_value(i8 2, !624, !DIExpression(), !625)
  %1 = load atomic ptr, ptr @_ZN9getrandom8backends27linux_android_with_fallback12GETRANDOM_FN17hecc8036d0c0d1ff5E acquire, align 8, !dbg !627, !noalias !628
    #dbg_value(ptr %1, !571, !DIExpression(), !635)
    #dbg_value(ptr %1, !636, !DIExpression(), !656)
  %2 = icmp eq ptr %1, null, !dbg !658
  br i1 %2, label %bb7.i.i.i.i.i, label %bb1.i.i.i.i.i, !dbg !658, !prof !659

bb7.i.i.i.i.i:                                    ; preds = %start
; call getrandom::backends::linux_android_with_fallback::init
  %3 = tail call noundef nonnull ptr @_ZN9getrandom8backends27linux_android_with_fallback4init17h04ff7c449f4a6b71E(), !dbg !660, !noalias !628
    #dbg_value(ptr %3, !572, !DIExpression(), !661)
  br label %bb1.i.i.i.i.i, !dbg !660

bb1.i.i.i.i.i:                                    ; preds = %bb7.i.i.i.i.i, %start
  %fptr.sroa.0.0.i.i.i.i.i = phi ptr [ %3, %bb7.i.i.i.i.i ], [ %1, %start ], !dbg !635
    #dbg_value(ptr %fptr.sroa.0.0.i.i.i.i.i, !572, !DIExpression(), !661)
    #dbg_value(ptr undef, !585, !DIExpression(), !595)
    #dbg_value(ptr poison, !594, !DIExpression(), !595)
  %_7.i.i.i.i.i = icmp eq ptr %fptr.sroa.0.0.i.i.i.i.i, inttoptr (i64 -1 to ptr), !dbg !662
  br i1 %_7.i.i.i.i.i, label %"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E.exit.i.i", label %bb3.i.i.i.i.i.i, !dbg !663

bb3.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i.i.i, %bb13.i.i.i.i.i.i
  %buf.sroa.0.038.i.i.i.i.i.i = phi ptr [ %buf.sroa.0.1.i.i.i.i.i.i, %bb13.i.i.i.i.i.i ], [ %seed.i.i, %bb1.i.i.i.i.i ]
  %buf.sroa.5.037.i.i.i.i.i.i = phi i64 [ %buf.sroa.5.1.i.i.i.i.i.i, %bb13.i.i.i.i.i.i ], [ 32, %bb1.i.i.i.i.i ]
    #dbg_value(ptr %buf.sroa.0.038.i.i.i.i.i.i, !539, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !560)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !539, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !560)
    #dbg_value(ptr poison, !664, !DIExpression(DW_OP_deref, DW_OP_deref), !671)
    #dbg_value(ptr %buf.sroa.0.038.i.i.i.i.i.i, !670, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !671)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !670, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !671)
  %_0.i.i.i.i.i.i.i = call noundef i64 %fptr.sroa.0.0.i.i.i.i.i(ptr noundef nonnull align 1 %buf.sroa.0.038.i.i.i.i.i.i, i64 noundef range(i64 1, 0) %buf.sroa.5.037.i.i.i.i.i.i, i32 noundef 0) #12, !dbg !673, !noalias !674
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !540, !DIExpression(), !677)
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !542, !DIExpression(), !678)
    #dbg_value(ptr undef, !544, !DIExpression(), !678)
  %_9.i.i.i.i.i.i = icmp sgt i64 %_0.i.i.i.i.i.i.i, 0, !dbg !679
  br i1 %_9.i.i.i.i.i.i, label %bb19.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i, !dbg !679

bb8.i.i.i.i.i.i:                                  ; preds = %bb3.i.i.i.i.i.i
  %4 = icmp eq i64 %_0.i.i.i.i.i.i.i, -1, !dbg !680
  br i1 %4, label %bb6.i.i.i.i.i.i, label %bb4.i, !dbg !680

bb6.i.i.i.i.i.i:                                  ; preds = %bb8.i.i.i.i.i.i
; call getrandom::backends::use_file::util_libc::last_os_error
  %err.i.i.i.i.i.i = call noundef i32 @_ZN9getrandom8backends8use_file9util_libc13last_os_error17h3226e5d4689f405dE(), !dbg !681, !noalias !674
    #dbg_value(i32 %err.i.i.i.i.i.i, !556, !DIExpression(), !682)
    #dbg_value(i32 %err.i.i.i.i.i.i, !683, !DIExpression(), !703)
    #dbg_value(i32 %err.i.i.i.i.i.i, !701, !DIExpression(), !705)
  %cond = icmp eq i32 %err.i.i.i.i.i.i, -4
  br i1 %cond, label %bb13.i.i.i.i.i.i, label %bb4.i, !dbg !706

bb13.i.i.i.i.i.i:                                 ; preds = %bb6.i.i.i.i.i.i, %bb19.i.i.i.i.i.i
  %buf.sroa.5.1.i.i.i.i.i.i = phi i64 [ %new_len.i.i.i.i.i.i, %bb19.i.i.i.i.i.i ], [ %buf.sroa.5.037.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ]
  %buf.sroa.0.1.i.i.i.i.i.i = phi ptr [ %_40.i.i.i.i.i.i, %bb19.i.i.i.i.i.i ], [ %buf.sroa.0.038.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ]
    #dbg_value(ptr %buf.sroa.0.1.i.i.i.i.i.i, !539, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !560)
    #dbg_value(i64 %buf.sroa.5.1.i.i.i.i.i.i, !539, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !560)
    #dbg_value(i64 poison, !707, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !749)
  %5 = icmp eq i64 %buf.sroa.5.1.i.i.i.i.i.i, 0, !dbg !751
  br i1 %5, label %bb5.i, label %bb3.i.i.i.i.i.i, !dbg !751

bb19.i.i.i.i.i.i:                                 ; preds = %bb3.i.i.i.i.i.i
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !546, !DIExpression(), !752)
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !753, !DIExpression(), !783)
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !785, !DIExpression(), !795)
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !797, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !809)
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !811, !DIExpression(), !832)
    #dbg_value(ptr %buf.sroa.0.038.i.i.i.i.i.i, !779, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !783)
    #dbg_value(ptr %buf.sroa.0.038.i.i.i.i.i.i, !793, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !795)
    #dbg_value(ptr %buf.sroa.0.038.i.i.i.i.i.i, !807, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !809)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !779, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !783)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !793, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !795)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !807, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !809)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !797, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !809)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !831, !DIExpression(), !832)
  %_38.i.i.i.i.i.i = icmp ult i64 %buf.sroa.5.037.i.i.i.i.i.i, %_0.i.i.i.i.i.i.i, !dbg !834
  %new_len.i.i.i.i.i.i = sub nuw i64 %buf.sroa.5.037.i.i.i.i.i.i, %_0.i.i.i.i.i.i.i, !dbg !834
  %_40.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %buf.sroa.0.038.i.i.i.i.i.i, i64 %_0.i.i.i.i.i.i.i, !dbg !834
    #dbg_value(ptr undef, !707, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !749)
    #dbg_value(i64 undef, !707, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !749)
  br i1 %_38.i.i.i.i.i.i, label %bb4.i, label %bb13.i.i.i.i.i.i, !dbg !835

"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E.exit.i.i": ; preds = %bb1.i.i.i.i.i
; call getrandom::backends::linux_android_with_fallback::use_file_fallback
  %6 = call noundef i32 @_ZN9getrandom8backends27linux_android_with_fallback17use_file_fallback17h9211927a188feaf1E(ptr noalias noundef nonnull align 1 %seed.i.i, i64 noundef 32), !dbg !836, !noalias !408
    #dbg_value(i32 %6, !837, !DIExpression(), !862)
  %.not.i.i = icmp eq i32 %6, 0, !dbg !864
  br i1 %.not.i.i, label %bb5.i, label %bb4.i, !dbg !865

bb4.i:                                            ; preds = %bb19.i.i.i.i.i.i, %bb6.i.i.i.i.i.i, %bb8.i.i.i.i.i.i, %"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E.exit.i.i"
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %seed.i.i), !dbg !866, !noalias !408
    #dbg_value(ptr %self, !357, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !867)
    #dbg_value(ptr poison, !868, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !876)
    #dbg_value(ptr %self, !357, !DIExpression(DW_OP_plus_uconst, 48, DW_OP_stack_value, DW_OP_LLVM_fragment, 64, 64), !867)
    #dbg_value(ptr poison, !868, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 64, 64), !876)
    #dbg_value(ptr %self, !874, !DIExpression(DW_OP_plus_uconst, 48, DW_OP_deref, DW_OP_stack_value), !876)
    #dbg_value(ptr %self, !357, !DIExpression(DW_OP_plus_uconst, 56, DW_OP_stack_value, DW_OP_LLVM_fragment, 128, 64), !867)
    #dbg_value(ptr poison, !868, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 128, 64), !876)
    #dbg_value(ptr %self, !875, !DIExpression(DW_OP_plus_uconst, 56, DW_OP_deref, DW_OP_stack_value), !876)
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 48
  %_9.pre = load i64, ptr %.phi.trans.insert, align 16, !dbg !878
  br label %"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E.exit", !dbg !879

bb5.i:                                            ; preds = %bb13.i.i.i.i.i.i, %"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E.exit.i.i"
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9.i.i), !dbg !880, !noalias !408
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(32) %_9.i.i, ptr noundef nonnull align 1 dereferenceable(32) %seed.i.i, i64 32, i1 false), !dbg !880, !noalias !408
    #dbg_declare(ptr %_9.i.i, !881, !DIExpression(), !888)
    #dbg_value(ptr %_9.i.i, !890, !DIExpression(), !903)
    #dbg_value(ptr @alloc_85fc59111fd0cef7ef4093da3840b035, !902, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !903)
    #dbg_value(i64 8, !902, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !903)
  %7 = getelementptr inbounds nuw i8, ptr %self1.i, i64 16, !dbg !905
; call rand_chacha::guts::init_chacha
  call void @_ZN11rand_chacha4guts11init_chacha17he4f07b70577fd00dE(ptr noalias nocapture noundef nonnull sret([48 x i8]) align 16 dereferenceable(48) %7, ptr noalias noundef nonnull readonly align 1 dereferenceable(32) %_9.i.i, ptr noalias noundef nonnull readonly align 1 @alloc_85fc59111fd0cef7ef4093da3840b035, i64 noundef 8), !dbg !906, !noalias !294
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9.i.i), !dbg !907, !noalias !408
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %seed.i.i), !dbg !866, !noalias !408
  %_6.i = getelementptr inbounds nuw i8, ptr %self, i64 48, !dbg !908
    #dbg_value(ptr %self, !357, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !867)
    #dbg_value(ptr poison, !868, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !876)
    #dbg_value(ptr %_6.i, !357, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !867)
    #dbg_value(ptr poison, !868, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 64, 64), !876)
    #dbg_value(ptr %_6.i, !874, !DIExpression(DW_OP_deref), !876)
    #dbg_value(ptr %self, !357, !DIExpression(DW_OP_plus_uconst, 56, DW_OP_stack_value, DW_OP_LLVM_fragment, 128, 64), !867)
    #dbg_value(ptr poison, !868, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 128, 64), !876)
    #dbg_value(ptr %self, !875, !DIExpression(DW_OP_plus_uconst, 56, DW_OP_deref, DW_OP_stack_value), !876)
    #dbg_declare(ptr poison, !873, !DIExpression(), !909)
  %_3.i.i = load i64, ptr %_6.i, align 16, !dbg !910, !alias.scope !294, !noalias !911, !noundef !23
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(64) %self, ptr noundef nonnull align 16 dereferenceable(48) %7, i64 48, i1 false), !dbg !915
  br label %"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E.exit", !dbg !916

"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E.exit": ; preds = %bb4.i, %bb5.i
  %_9 = phi i64 [ %_9.pre, %bb4.i ], [ %_3.i.i, %bb5.i ], !dbg !878
  call void @llvm.lifetime.end.p0(i64 64, ptr nonnull %self1.i), !dbg !917, !noalias !294
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 56, !dbg !918
  %9 = add i64 %_9, -256, !dbg !918
  store i64 %9, ptr %8, align 8, !dbg !918
    #dbg_value(ptr %self, !919, !DIExpression(), !926)
    #dbg_value(ptr %0, !925, !DIExpression(), !926)
    #dbg_value(i32 6, !928, !DIExpression(), !938)
    #dbg_value(ptr %self, !936, !DIExpression(), !938)
    #dbg_value(ptr %0, !937, !DIExpression(), !938)
; call rand_chacha::guts::refill_wide
  call void @_ZN11rand_chacha4guts11refill_wide17h7bc07d422e567fdcE(ptr noalias noundef nonnull align 16 dereferenceable(48) %self, i32 noundef 6, ptr noalias noundef nonnull align 4 dereferenceable(256) %0), !dbg !940
  ret void, !dbg !941
}

; <dp_ex::Dog as dp_ex::Animal>::speak
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17h95a71617b8cd4b02E"(ptr noalias nocapture nonnull readonly align 1 %self) unnamed_addr #0 !dbg !942 {
start:
  %_3 = alloca [48 x i8], align 8
    #dbg_value(ptr poison, !949, !DIExpression(), !950)
    #dbg_value(ptr @alloc_ec4fa215300b117d5ab20e2368000be2, !951, !DIExpression(), !1115)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3), !dbg !1116
  store ptr @alloc_ec4fa215300b117d5ab20e2368000be2, ptr %_3, align 8, !dbg !1117
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 8, !dbg !1117
  store i64 1, ptr %0, align 8, !dbg !1117
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 32, !dbg !1117
  store ptr null, ptr %1, align 8, !dbg !1117
  %2 = getelementptr inbounds nuw i8, ptr %_3, i64 16, !dbg !1117
  store ptr inttoptr (i64 8 to ptr), ptr %2, align 8, !dbg !1117
  %3 = getelementptr inbounds nuw i8, ptr %_3, i64 24, !dbg !1117
  store i64 0, ptr %3, align 8, !dbg !1117
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_3), !dbg !1116
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3), !dbg !1116
  ret void, !dbg !1118
}

; <dp_ex::Cat as dp_ex::Animal>::speak
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17hfb6e878b47c18265E"(ptr noalias nocapture nonnull readonly align 1 %self) unnamed_addr #0 !dbg !1119 {
start:
  %_3 = alloca [48 x i8], align 8
    #dbg_value(ptr poison, !1125, !DIExpression(), !1126)
    #dbg_value(ptr @alloc_000bc512779c9a763a8aa84ee52b6421, !1127, !DIExpression(), !1130)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3), !dbg !1131
  store ptr @alloc_000bc512779c9a763a8aa84ee52b6421, ptr %_3, align 8, !dbg !1132
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 8, !dbg !1132
  store i64 1, ptr %0, align 8, !dbg !1132
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 32, !dbg !1132
  store ptr null, ptr %1, align 8, !dbg !1132
  %2 = getelementptr inbounds nuw i8, ptr %_3, i64 16, !dbg !1132
  store ptr inttoptr (i64 8 to ptr), ptr %2, align 8, !dbg !1132
  %3 = getelementptr inbounds nuw i8, ptr %_3, i64 24, !dbg !1132
  store i64 0, ptr %3, align 8, !dbg !1132
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_3), !dbg !1131
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3), !dbg !1131
  ret void, !dbg !1133
}

; <dp_ex::Bird as dp_ex::Animal>::speak
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN45_$LT$dp_ex..Bird$u20$as$u20$dp_ex..Animal$GT$5speak17h900f4f573272ed1cE"(ptr noalias nocapture nonnull readonly align 1 %self) unnamed_addr #0 !dbg !1134 {
start:
  %_3 = alloca [48 x i8], align 8
    #dbg_value(ptr poison, !1140, !DIExpression(), !1141)
    #dbg_value(ptr @alloc_aa690a7f645d07b0914dfbca7e9c692c, !1142, !DIExpression(), !1145)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3), !dbg !1146
  store ptr @alloc_aa690a7f645d07b0914dfbca7e9c692c, ptr %_3, align 8, !dbg !1147
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 8, !dbg !1147
  store i64 1, ptr %0, align 8, !dbg !1147
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 32, !dbg !1147
  store ptr null, ptr %1, align 8, !dbg !1147
  %2 = getelementptr inbounds nuw i8, ptr %_3, i64 16, !dbg !1147
  store ptr inttoptr (i64 8 to ptr), ptr %2, align 8, !dbg !1147
  %3 = getelementptr inbounds nuw i8, ptr %_3, i64 24, !dbg !1147
  store i64 0, ptr %3, align 8, !dbg !1147
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_3), !dbg !1146
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3), !dbg !1146
  ret void, !dbg !1148
}

; Function Attrs: nonlazybind uwtable
define dso_local void @animal_speak(ptr noundef nonnull align 1 %animal.0, ptr noalias nocapture noundef readonly align 8 dereferenceable(32) %animal.1) unnamed_addr #0 !dbg !1149 {
start:
    #dbg_value(ptr %animal.0, !1161, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !1162)
    #dbg_value(ptr %animal.1, !1161, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !1162)
  %0 = getelementptr inbounds nuw i8, ptr %animal.1, i64 24, !dbg !1163
  %1 = load ptr, ptr %0, align 8, !dbg !1163, !invariant.load !23, !nonnull !23
  tail call void %1(ptr noundef nonnull align 1 %animal.0), !dbg !1163
  ret void, !dbg !1164
}

; dp_ex::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN5dp_ex4main17hc7990c7b9cee8a83E() unnamed_addr #0 personality ptr @rust_eh_personality !dbg !1165 {
start:
  %_3.i.i = alloca [48 x i8], align 8
  %_4.i = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_4.i), !dbg !1166
; call rand::rngs::thread::rng
  %0 = tail call noundef nonnull ptr @_ZN4rand4rngs6thread3rng17h70cca7a3940ce3c4E(), !dbg !1166
  store ptr %0, ptr %_4.i, align 8, !dbg !1166
    #dbg_value(i32 3, !1174, !DIExpression(), !1251)
    #dbg_value(ptr poison, !1247, !DIExpression(), !1251)
    #dbg_value(i32 3, !1253, !DIExpression(), !1278)
    #dbg_value(ptr poison, !1275, !DIExpression(), !1278)
    #dbg_value(i32 0, !1280, !DIExpression(), !1297)
    #dbg_value(i32 3, !1288, !DIExpression(), !1297)
    #dbg_value(ptr poison, !1289, !DIExpression(), !1297)
    #dbg_value(i32 0, !1290, !DIExpression(), !1299)
    #dbg_value(i32 3, !1292, !DIExpression(), !1300)
    #dbg_value(i32 0, !1301, !DIExpression(), !1319)
    #dbg_value(i32 3, !1304, !DIExpression(DW_OP_constu, 1, DW_OP_minus, DW_OP_stack_value), !1319)
    #dbg_value(ptr poison, !1305, !DIExpression(), !1319)
    #dbg_value(i32 0, !1306, !DIExpression(), !1321)
    #dbg_value(i32 3, !1308, !DIExpression(DW_OP_constu, 1, DW_OP_minus, DW_OP_stack_value), !1322)
    #dbg_value(i32 3, !1310, !DIExpression(), !1323)
    #dbg_value(i32 3, !1324, !DIExpression(), !1341)
    #dbg_value(i32 3, !1324, !DIExpression(), !1343)
    #dbg_value(ptr poison, !1345, !DIExpression(), !1351)
    #dbg_value(ptr poison, !1345, !DIExpression(), !1353)
    #dbg_value(ptr poison, !1355, !DIExpression(), !1366)
    #dbg_value(ptr poison, !1355, !DIExpression(), !1368)
    #dbg_value(ptr poison, !1365, !DIExpression(), !1366)
    #dbg_value(ptr poison, !1365, !DIExpression(), !1368)
    #dbg_value(ptr poison, !1370, !DIExpression(), !1378)
    #dbg_value(ptr poison, !1370, !DIExpression(), !1380)
    #dbg_value(ptr %0, !1382, !DIExpression(DW_OP_plus_uconst, 16, DW_OP_stack_value), !1390)
    #dbg_value(ptr %0, !1382, !DIExpression(DW_OP_plus_uconst, 16, DW_OP_stack_value), !1392)
  %_7.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 16, !dbg !1394
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1375, !DIExpression(), !1399)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1375, !DIExpression(), !1400)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1401, !DIExpression(), !1408)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1401, !DIExpression(), !1410)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1412, !DIExpression(), !1422)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1412, !DIExpression(), !1424)
  %1 = getelementptr inbounds nuw i8, ptr %0, i64 336, !dbg !1426
  %_3.i.i.i.i.i.i.i.i = load i64, ptr %1, align 16, !dbg !1426, !noalias !23, !noundef !23
  %_2.i.i.i.i.i.i.i.i = icmp ugt i64 %_3.i.i.i.i.i.i.i.i, 63, !dbg !1426
  br i1 %_2.i.i.i.i.i.i.i.i, label %bb2.i.i.i18.i.i.i.i.i, label %_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit24.i.i.i.i.i, !dbg !1429

bb2.i.i.i18.i.i.i.i.i:                            ; preds = %start
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1430, !DIExpression(), !1437)
    #dbg_value(i64 0, !1436, !DIExpression(), !1437)
  %_9.i.i.i.i19.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 272, !dbg !1439
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1440, !DIExpression(), !1447)
    #dbg_value(ptr %_9.i.i.i.i19.i.i.i.i.i, !1444, !DIExpression(), !1447)
  %2 = getelementptr inbounds nuw i8, ptr %0, i64 328, !dbg !1449
  %_4.i.i.i.i.i20.i.i.i.i.i = load i64, ptr %2, align 8, !dbg !1449, !alias.scope !1450, !noalias !1457, !noundef !23
  %_3.i.i.i.i.i21.i.i.i.i.i = icmp slt i64 %_4.i.i.i.i.i20.i.i.i.i.i, 1, !dbg !1449
  br i1 %_3.i.i.i.i.i21.i.i.i.i.i, label %bb1.i.i.i.i.i23.i.i.i.i.i, label %bb3.i.i.i.i.i22.i.i.i.i.i, !dbg !1449

bb3.i.i.i.i.i22.i.i.i.i.i:                        ; preds = %bb2.i.i.i18.i.i.i.i.i
    #dbg_value(i64 256, !1445, !DIExpression(), !1461)
  %3 = add nsw i64 %_4.i.i.i.i.i20.i.i.i.i.i, -256, !dbg !1462
  store i64 %3, ptr %2, align 8, !dbg !1462, !alias.scope !1450, !noalias !1457
    #dbg_value(ptr %_9.i.i.i.i19.i.i.i.i.i, !919, !DIExpression(), !1463)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !925, !DIExpression(), !1463)
    #dbg_value(i32 6, !928, !DIExpression(), !1465)
    #dbg_value(ptr %_9.i.i.i.i19.i.i.i.i.i, !936, !DIExpression(), !1465)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !937, !DIExpression(), !1465)
; invoke rand_chacha::guts::refill_wide
  invoke void @_ZN11rand_chacha4guts11refill_wide17h7bc07d422e567fdcE(ptr noalias noundef nonnull align 16 dereferenceable(64) %_9.i.i.i.i19.i.i.i.i.i, i32 noundef 6, ptr noalias noundef nonnull align 16 dereferenceable(336) %_7.i.i.i.i.i.i.i.i)
          to label %_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit24.i.i.i.i.i unwind label %cleanup.i, !dbg !1467

bb1.i.i.i.i.i23.i.i.i.i.i:                        ; preds = %bb2.i.i.i18.i.i.i.i.i
; invoke rand::rngs::reseeding::ReseedingCore<R,Rsdr>::reseed_and_generate
  invoke fastcc void @"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17ha6228666c915f093E"(ptr noalias noundef nonnull align 16 dereferenceable(64) %_9.i.i.i.i19.i.i.i.i.i, ptr noalias noundef nonnull align 16 dereferenceable(336) %_7.i.i.i.i.i.i.i.i)
          to label %_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit24.i.i.i.i.i unwind label %cleanup.i, !dbg !1468

_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit24.i.i.i.i.i: ; preds = %bb1.i.i.i.i.i23.i.i.i.i.i, %bb3.i.i.i.i.i22.i.i.i.i.i, %start
  %_10.i.i.i16.i.i.i.i.i = phi i64 [ %_3.i.i.i.i.i.i.i.i, %start ], [ 0, %bb3.i.i.i.i.i22.i.i.i.i.i ], [ 0, %bb1.i.i.i.i.i23.i.i.i.i.i ], !dbg !1469
  %4 = getelementptr inbounds nuw i32, ptr %_7.i.i.i.i.i.i.i.i, i64 %_10.i.i.i16.i.i.i.i.i, !dbg !1470
  %value.i.i.i17.i.i.i.i.i = load i32, ptr %4, align 4, !dbg !1470, !alias.scope !1471, !noalias !1472, !noundef !23
    #dbg_value(i32 %value.i.i.i17.i.i.i.i.i, !1420, !DIExpression(), !1473)
  %5 = add nuw nsw i64 %_10.i.i.i16.i.i.i.i.i, 1, !dbg !1474
  store i64 %5, ptr %1, align 16, !dbg !1474, !alias.scope !1471, !noalias !1472
    #dbg_value(i32 %value.i.i.i17.i.i.i.i.i, !1336, !DIExpression(), !1341)
  %_26.i.i.i.i.i = zext i32 %value.i.i.i17.i.i.i.i.i to i64, !dbg !1475
  %tmp.i.i.i.i.i = mul nuw nsw i64 %_26.i.i.i.i.i, 3, !dbg !1475
    #dbg_value(i64 %tmp.i.i.i.i.i, !1337, !DIExpression(), !1476)
  %_29.i.i.i.i.i = lshr i64 %tmp.i.i.i.i.i, 32, !dbg !1477
  %lo_order.i.i.i.i.i = trunc i64 %tmp.i.i.i.i.i to i32, !dbg !1478
    #dbg_value(i32 %lo_order.i.i.i.i.i, !1314, !DIExpression(), !1479)
    #dbg_value(i32 %lo_order.i.i.i.i.i, !1480, !DIExpression(), !1498)
    #dbg_value(i64 %_29.i.i.i.i.i, !1312, !DIExpression(DW_OP_LLVM_convert, 64, DW_ATE_unsigned, DW_OP_LLVM_convert, 32, DW_ATE_unsigned, DW_OP_stack_value), !1479)
  %_16.i.i.i.i.i = icmp ugt i32 %lo_order.i.i.i.i.i, -3, !dbg !1500
  br i1 %_16.i.i.i.i.i, label %bb9.i.i.i.i.i, label %bb2.i, !dbg !1500

bb9.i.i.i.i.i:                                    ; preds = %_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit24.i.i.i.i.i
    #dbg_value(ptr poison, !1345, !DIExpression(), !1501)
    #dbg_value(ptr poison, !1355, !DIExpression(), !1503)
    #dbg_value(ptr poison, !1365, !DIExpression(), !1503)
    #dbg_value(ptr poison, !1370, !DIExpression(), !1505)
    #dbg_value(ptr %0, !1382, !DIExpression(DW_OP_plus_uconst, 16, DW_OP_stack_value), !1507)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1375, !DIExpression(), !1509)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1401, !DIExpression(), !1510)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1412, !DIExpression(), !1512)
  %_2.i.i.i27.i.i.i.i.i = icmp eq i64 %_10.i.i.i16.i.i.i.i.i, 63, !dbg !1514
  br i1 %_2.i.i.i27.i.i.i.i.i, label %bb2.i.i.i30.i.i.i.i.i, label %_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit36.i.i.i.i.i, !dbg !1514

bb2.i.i.i30.i.i.i.i.i:                            ; preds = %bb9.i.i.i.i.i
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1430, !DIExpression(), !1515)
    #dbg_value(i64 0, !1436, !DIExpression(), !1515)
  %_9.i.i.i.i31.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 272, !dbg !1517
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !1440, !DIExpression(), !1518)
    #dbg_value(ptr %_9.i.i.i.i31.i.i.i.i.i, !1444, !DIExpression(), !1518)
  %6 = getelementptr inbounds nuw i8, ptr %0, i64 328, !dbg !1520
  %_4.i.i.i.i.i32.i.i.i.i.i = load i64, ptr %6, align 8, !dbg !1520, !alias.scope !1521, !noalias !1528, !noundef !23
  %_3.i.i.i.i.i33.i.i.i.i.i = icmp slt i64 %_4.i.i.i.i.i32.i.i.i.i.i, 1, !dbg !1520
  br i1 %_3.i.i.i.i.i33.i.i.i.i.i, label %bb1.i.i.i.i.i35.i.i.i.i.i, label %bb3.i.i.i.i.i34.i.i.i.i.i, !dbg !1520

bb3.i.i.i.i.i34.i.i.i.i.i:                        ; preds = %bb2.i.i.i30.i.i.i.i.i
    #dbg_value(i64 256, !1445, !DIExpression(), !1532)
  %7 = add nsw i64 %_4.i.i.i.i.i32.i.i.i.i.i, -256, !dbg !1533
  store i64 %7, ptr %6, align 8, !dbg !1533, !alias.scope !1521, !noalias !1528
    #dbg_value(ptr %_9.i.i.i.i31.i.i.i.i.i, !919, !DIExpression(), !1534)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !925, !DIExpression(), !1534)
    #dbg_value(i32 6, !928, !DIExpression(), !1536)
    #dbg_value(ptr %_9.i.i.i.i31.i.i.i.i.i, !936, !DIExpression(), !1536)
    #dbg_value(ptr %_7.i.i.i.i.i.i.i.i, !937, !DIExpression(), !1536)
; invoke rand_chacha::guts::refill_wide
  invoke void @_ZN11rand_chacha4guts11refill_wide17h7bc07d422e567fdcE(ptr noalias noundef nonnull align 16 dereferenceable(64) %_9.i.i.i.i31.i.i.i.i.i, i32 noundef 6, ptr noalias noundef nonnull align 16 dereferenceable(336) %_7.i.i.i.i.i.i.i.i)
          to label %_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit36.i.i.i.i.i unwind label %cleanup.i, !dbg !1538

bb1.i.i.i.i.i35.i.i.i.i.i:                        ; preds = %bb2.i.i.i30.i.i.i.i.i
; invoke rand::rngs::reseeding::ReseedingCore<R,Rsdr>::reseed_and_generate
  invoke fastcc void @"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17ha6228666c915f093E"(ptr noalias noundef nonnull align 16 dereferenceable(64) %_9.i.i.i.i31.i.i.i.i.i, ptr noalias noundef nonnull align 16 dereferenceable(336) %_7.i.i.i.i.i.i.i.i)
          to label %_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit36.i.i.i.i.i unwind label %cleanup.i, !dbg !1539

_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit36.i.i.i.i.i: ; preds = %bb1.i.i.i.i.i35.i.i.i.i.i, %bb3.i.i.i.i.i34.i.i.i.i.i, %bb9.i.i.i.i.i
  %_10.i.i.i28.i.i.i.i.i = phi i64 [ %5, %bb9.i.i.i.i.i ], [ 0, %bb3.i.i.i.i.i34.i.i.i.i.i ], [ 0, %bb1.i.i.i.i.i35.i.i.i.i.i ], !dbg !1540
  %8 = getelementptr inbounds nuw i32, ptr %_7.i.i.i.i.i.i.i.i, i64 %_10.i.i.i28.i.i.i.i.i, !dbg !1541
  %value.i.i.i29.i.i.i.i.i = load i32, ptr %8, align 4, !dbg !1541, !alias.scope !1542, !noalias !1543, !noundef !23
    #dbg_value(i32 %value.i.i.i29.i.i.i.i.i, !1420, !DIExpression(), !1544)
  %9 = add nuw nsw i64 %_10.i.i.i28.i.i.i.i.i, 1, !dbg !1545
  store i64 %9, ptr %1, align 16, !dbg !1545, !alias.scope !1542, !noalias !1543
    #dbg_value(i32 %value.i.i.i29.i.i.i.i.i, !1336, !DIExpression(), !1343)
  %_32.i.i.i.i.i = zext i32 %value.i.i.i29.i.i.i.i.i to i64, !dbg !1546
  %tmp5.i.i.i.i.i = mul nuw nsw i64 %_32.i.i.i.i.i, 3, !dbg !1546
    #dbg_value(i64 %tmp5.i.i.i.i.i, !1339, !DIExpression(), !1547)
  %_34.i.i.i.i.i = lshr i64 %tmp5.i.i.i.i.i, 32, !dbg !1548
  %new_hi_order.i.i.i.i.i = trunc nuw nsw i64 %_34.i.i.i.i.i to i32, !dbg !1549
    #dbg_value(i32 %new_hi_order.i.i.i.i.i, !1315, !DIExpression(), !1550)
    #dbg_value(i32 %new_hi_order.i.i.i.i.i, !1497, !DIExpression(), !1498)
  %10 = tail call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %lo_order.i.i.i.i.i, i32 %new_hi_order.i.i.i.i.i), !dbg !1551
  %_36.1.i.i.i.i.i = extractvalue { i32, i1 } %10, 1, !dbg !1551
    #dbg_value(i1 %_36.1.i.i.i.i.i, !1317, !DIExpression(DW_OP_LLVM_convert, 1, DW_ATE_unsigned, DW_OP_LLVM_convert, 8, DW_ATE_unsigned, DW_OP_stack_value), !1552)
  %_22.i.i.i.i.i = zext i1 %_36.1.i.i.i.i.i to i64, !dbg !1553
  %11 = add nuw nsw i64 %_29.i.i.i.i.i, %_22.i.i.i.i.i, !dbg !1554
    #dbg_value(!DIArgList(i1 %_36.1.i.i.i.i.i, i64 %_29.i.i.i.i.i), !1312, !DIExpression(DW_OP_LLVM_arg, 0, DW_OP_LLVM_convert, 1, DW_ATE_unsigned, DW_OP_LLVM_convert, 32, DW_ATE_unsigned, DW_OP_LLVM_arg, 1, DW_OP_LLVM_convert, 64, DW_ATE_unsigned, DW_OP_LLVM_convert, 32, DW_ATE_unsigned, DW_OP_plus, DW_OP_stack_value), !1479)
  br label %bb2.i, !dbg !1555

cleanup.i:                                        ; preds = %bb1.i.i.i.i.i35.i.i.i.i.i, %bb3.i.i.i.i.i34.i.i.i.i.i, %bb1.i.i.i.i.i23.i.i.i.i.i, %bb3.i.i.i.i.i22.i.i.i.i.i
  %12 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1556), !dbg !1559
    #dbg_value(ptr %_4.i, !1560, !DIExpression(), !1569)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1571), !dbg !1574
    #dbg_value(ptr %_4.i, !1575, !DIExpression(), !1583)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1585), !dbg !1588
    #dbg_value(ptr %_4.i, !1589, !DIExpression(), !1597)
    #dbg_value(ptr %_4.i, !1599, !DIExpression(), !1607)
    #dbg_value(ptr %_4.i, !1599, !DIExpression(), !1609)
  %_5.i.i.i.i = load ptr, ptr %_4.i, align 8, !dbg !1611, !alias.scope !1618, !nonnull !23, !noundef !23
    #dbg_value(ptr %_5.i.i.i.i, !1619, !DIExpression(), !1627)
    #dbg_value(ptr %_5.i.i.i.i, !1629, !DIExpression(), !1634)
    #dbg_value(ptr %_5.i.i.i.i, !1636, !DIExpression(), !1644)
    #dbg_value(ptr %_5.i.i.i.i, !1646, !DIExpression(), !1653)
    #dbg_value(ptr %_5.i.i.i.i, !1655, !DIExpression(), !1661)
  %_8.i.i.i.i = load i64, ptr %_5.i.i.i.i, align 8, !dbg !1663, !noalias !1618, !noundef !23
  %val.i.i.i.i = add i64 %_8.i.i.i.i, -1, !dbg !1664
    #dbg_value(i64 %val.i.i.i.i, !1643, !DIExpression(), !1644)
    #dbg_value(i64 %val.i.i.i.i, !1652, !DIExpression(), !1653)
  store i64 %val.i.i.i.i, ptr %_5.i.i.i.i, align 8, !dbg !1665, !noalias !1618
    #dbg_value(ptr %_5.i.i.i.i, !1629, !DIExpression(), !1672)
    #dbg_value(ptr %_5.i.i.i.i, !1655, !DIExpression(), !1674)
  %13 = icmp eq i64 %val.i.i.i.i, 0, !dbg !1677
  br i1 %13, label %bb1.i.i.i.i, label %bb12.i, !dbg !1677

bb1.i.i.i.i:                                      ; preds = %cleanup.i
; invoke alloc::rc::Rc<T,A>::drop_slow
  invoke void @"_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h8d92a5ccbffeb99aE"(ptr noalias noundef nonnull align 8 dereferenceable(8) %_4.i)
          to label %bb12.i unwind label %terminate.i, !dbg !1678

bb2.i:                                            ; preds = %_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit36.i.i.i.i.i, %_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit24.i.i.i.i.i
  %result.sroa.0.0.i.i.i.i.i = phi i64 [ %11, %_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit36.i.i.i.i.i ], [ %_29.i.i.i.i.i, %_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE.exit24.i.i.i.i.i ], !dbg !1323
    #dbg_value(i64 %result.sroa.0.0.i.i.i.i.i, !1312, !DIExpression(), !1479)
    #dbg_value(i64 %result.sroa.0.0.i.i.i.i.i, !1171, !DIExpression(DW_OP_LLVM_convert, 64, DW_ATE_unsigned, DW_OP_LLVM_convert, 32, DW_ATE_unsigned, DW_OP_stack_value), !1679)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1680), !dbg !1559
    #dbg_value(ptr %_4.i, !1560, !DIExpression(), !1683)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1685), !dbg !1688
    #dbg_value(ptr %_4.i, !1575, !DIExpression(), !1689)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1691), !dbg !1694
    #dbg_value(ptr %_4.i, !1589, !DIExpression(), !1695)
    #dbg_value(ptr %_4.i, !1599, !DIExpression(), !1697)
    #dbg_value(ptr %_4.i, !1599, !DIExpression(), !1699)
  %_5.i.i.i7.i = load ptr, ptr %_4.i, align 8, !dbg !1701, !alias.scope !1703, !nonnull !23, !noundef !23
    #dbg_value(ptr %_5.i.i.i7.i, !1619, !DIExpression(), !1704)
    #dbg_value(ptr %_5.i.i.i7.i, !1629, !DIExpression(), !1706)
    #dbg_value(ptr %_5.i.i.i7.i, !1636, !DIExpression(), !1708)
    #dbg_value(ptr %_5.i.i.i7.i, !1646, !DIExpression(), !1710)
    #dbg_value(ptr %_5.i.i.i7.i, !1655, !DIExpression(), !1712)
  %_8.i.i.i8.i = load i64, ptr %_5.i.i.i7.i, align 8, !dbg !1714, !noalias !1703, !noundef !23
  %val.i.i.i9.i = add i64 %_8.i.i.i8.i, -1, !dbg !1715
    #dbg_value(i64 %val.i.i.i9.i, !1643, !DIExpression(), !1708)
    #dbg_value(i64 %val.i.i.i9.i, !1652, !DIExpression(), !1710)
  store i64 %val.i.i.i9.i, ptr %_5.i.i.i7.i, align 8, !dbg !1716, !noalias !1703
    #dbg_value(ptr %_5.i.i.i7.i, !1629, !DIExpression(), !1718)
    #dbg_value(ptr %_5.i.i.i7.i, !1655, !DIExpression(), !1720)
  %14 = icmp eq i64 %val.i.i.i9.i, 0, !dbg !1722
  br i1 %14, label %bb1.i.i.i10.i, label %_ZN5dp_ex6dyn_dp17h87c4dad2651e3135E.exit, !dbg !1722

bb1.i.i.i10.i:                                    ; preds = %bb2.i
; call alloc::rc::Rc<T,A>::drop_slow
  call void @"_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h8d92a5ccbffeb99aE"(ptr noalias noundef nonnull align 8 dereferenceable(8) %_4.i), !dbg !1723
  br label %_ZN5dp_ex6dyn_dp17h87c4dad2651e3135E.exit, !dbg !1723

terminate.i:                                      ; preds = %bb1.i.i.i.i
  %15 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h4898e2af8dc1f529E() #13, !dbg !1724
  unreachable, !dbg !1724

bb12.i:                                           ; preds = %bb1.i.i.i.i, %cleanup.i
  resume { ptr, i32 } %12, !dbg !1724

_ZN5dp_ex6dyn_dp17h87c4dad2651e3135E.exit:        ; preds = %bb2.i, %bb1.i.i.i10.i
    #dbg_value(i64 %result.sroa.0.0.i.i.i.i.i, !1171, !DIExpression(DW_OP_LLVM_convert, 64, DW_ATE_unsigned, DW_OP_LLVM_convert, 32, DW_ATE_unsigned, DW_OP_stack_value), !1679)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_4.i), !dbg !1559
  %switch.selectcmp.i = icmp eq i64 %result.sroa.0.0.i.i.i.i.i, 1, !dbg !1725
  %switch.select.i = select i1 %switch.selectcmp.i, ptr @vtable.3, ptr @vtable.4, !dbg !1725
  %switch.selectcmp1.i = icmp eq i64 %result.sroa.0.0.i.i.i.i.i, 0, !dbg !1725
  %switch.select2.i = select i1 %switch.selectcmp1.i, ptr @vtable.2, ptr %switch.select.i, !dbg !1725
    #dbg_value(ptr inttoptr (i64 1 to ptr), !1170, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !1726)
    #dbg_value(ptr %switch.select2.i, !1170, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !1726)
  %16 = getelementptr inbounds nuw i8, ptr %switch.select2.i, i64 24, !dbg !1727
  %17 = load ptr, ptr %16, align 8, !dbg !1727, !invariant.load !23, !nonnull !23
  call void %17(ptr noundef nonnull align 1 inttoptr (i64 1 to ptr)), !dbg !1727
    #dbg_value(ptr inttoptr (i64 1 to ptr), !1728, !DIExpression(), !1732)
    #dbg_value(ptr inttoptr (i64 1 to ptr), !1125, !DIExpression(), !1734)
    #dbg_value(ptr @alloc_000bc512779c9a763a8aa84ee52b6421, !1127, !DIExpression(), !1736)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3.i.i), !dbg !1738
  store ptr @alloc_000bc512779c9a763a8aa84ee52b6421, ptr %_3.i.i, align 8, !dbg !1739
  %18 = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 8, !dbg !1739
  store i64 1, ptr %18, align 8, !dbg !1739
  %19 = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 32, !dbg !1739
  store ptr null, ptr %19, align 8, !dbg !1739
  %20 = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 16, !dbg !1739
  store ptr inttoptr (i64 8 to ptr), ptr %20, align 8, !dbg !1739
  %21 = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 24, !dbg !1739
  store i64 0, ptr %21, align 8, !dbg !1739
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_3.i.i), !dbg !1738
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3.i.i), !dbg !1738
  ret void, !dbg !1740
}

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #3

; Function Attrs: nounwind nonlazybind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef range(i32 1, 17), i64 noundef, ptr noundef, ptr noundef) unnamed_addr #4

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #5

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare noundef i64 @_ZN3std2rt19lang_start_internal17h179a86731aa5c4aaE(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; alloc::rc::Rc<T,A>::drop_slow
; Function Attrs: noinline nonlazybind uwtable
declare void @"_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h8d92a5ccbffeb99aE"(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #2

; rand_chacha::guts::init_chacha
; Function Attrs: nonlazybind uwtable
declare void @_ZN11rand_chacha4guts11init_chacha17he4f07b70577fd00dE(ptr dead_on_unwind noalias nocapture noundef writable sret([48 x i8]) align 16 dereferenceable(48), ptr noalias noundef readonly align 1 dereferenceable(32), ptr noalias noundef nonnull readonly align 1, i64 noundef) unnamed_addr #0

; rand_chacha::guts::refill_wide
; Function Attrs: nonlazybind uwtable
declare void @_ZN11rand_chacha4guts11refill_wide17h7bc07d422e567fdcE(ptr noalias noundef align 16 dereferenceable(48), i32 noundef, ptr noalias noundef align 4 dereferenceable(256)) unnamed_addr #0

; getrandom::backends::linux_android_with_fallback::init
; Function Attrs: cold noinline nonlazybind uwtable
declare noundef nonnull ptr @_ZN9getrandom8backends27linux_android_with_fallback4init17h04ff7c449f4a6b71E() unnamed_addr #6

; getrandom::backends::linux_android_with_fallback::use_file_fallback
; Function Attrs: noinline nonlazybind uwtable
declare noundef i32 @_ZN9getrandom8backends27linux_android_with_fallback17use_file_fallback17h9211927a188feaf1E(ptr noalias noundef nonnull align 1, i64 noundef) unnamed_addr #2

; getrandom::backends::use_file::util_libc::last_os_error
; Function Attrs: nonlazybind uwtable
declare noundef range(i32 1, 0) i32 @_ZN9getrandom8backends8use_file9util_libc13last_os_error17h3226e5d4689f405dE() unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef align 8 dereferenceable(48)) unnamed_addr #0

; rand::rngs::thread::rng
; Function Attrs: nonlazybind uwtable
declare noundef nonnull ptr @_ZN4rand4rngs6thread3rng17h70cca7a3940ce3c4E() unnamed_addr #0

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind nonlazybind optsize uwtable
declare void @_ZN4core9panicking16panic_in_cleanup17h4898e2af8dc1f529E() unnamed_addr #7

; Function Attrs: nonlazybind
define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #8 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = load volatile i8, ptr @__rustc_debug_gdb_scripts_section__, align 1
  %3 = sext i32 %0 to i64
    #dbg_value(ptr @_ZN5dp_ex4main17hc7990c7b9cee8a83E, !111, !DIExpression(), !117)
    #dbg_value(i64 %3, !112, !DIExpression(), !117)
    #dbg_value(ptr %1, !113, !DIExpression(), !117)
    #dbg_value(i8 0, !114, !DIExpression(), !117)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i), !dbg !118
  store ptr @_ZN5dp_ex4main17hc7990c7b9cee8a83E, ptr %_7.i, align 8, !dbg !118
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_ZN3std2rt19lang_start_internal17h179a86731aa5c4aaE(ptr noundef nonnull align 1 %_7.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(48) @vtable.0, i64 noundef %3, ptr noundef %1, i8 noundef 0), !dbg !119
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i), !dbg !120
  %4 = trunc i64 %_0.i to i32
  ret i32 %4
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #9

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #9

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #10

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr nocapture writeonly, i8, i64, i1 immarg) #11

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #4 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #5 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #6 = { cold noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #7 = { cold minsize noinline noreturn nounwind nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #8 = { nonlazybind "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #9 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #10 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #11 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #12 = { nounwind }
attributes #13 = { cold noreturn nounwind }

!llvm.module.flags = !{!52, !53, !54, !55, !56}
!llvm.ident = !{!57}
!llvm.dbg.cu = !{!58}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable}", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable_type}", file: !2, size: 384, align: 64, flags: DIFlagArtificial, elements: !4, vtableHolder: !14, templateParams: !23, identifier: "8c89ff4b196217e389d8cd2a9b7cc0bd")
!4 = !{!5, !8, !10, !11, !12, !13}
!5 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !3, file: !2, baseType: !6, size: 64, align: 64)
!6 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const ()", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 0)
!7 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!8 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !3, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!9 = !DIBasicType(name: "usize", size: 64, encoding: DW_ATE_unsigned)
!10 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !3, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!11 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !3, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!12 = !DIDerivedType(tag: DW_TAG_member, name: "__method4", scope: !3, file: !2, baseType: !6, size: 64, align: 64, offset: 256)
!13 = !DIDerivedType(tag: DW_TAG_member, name: "__method5", scope: !3, file: !2, baseType: !6, size: 64, align: 64, offset: 320)
!14 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<()>", scope: !15, file: !2, size: 64, align: 64, elements: !18, templateParams: !23, identifier: "af6245bba15cb41add62c6071cc22fba")
!15 = !DINamespace(name: "lang_start", scope: !16)
!16 = !DINamespace(name: "rt", scope: !17)
!17 = !DINamespace(name: "std", scope: null)
!18 = !{!19}
!19 = !DIDerivedType(tag: DW_TAG_member, name: "main", scope: !14, file: !2, baseType: !20, size: 64, align: 64)
!20 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn()", baseType: !21, size: 64, align: 64, dwarfAddressSpace: 0)
!21 = !DISubroutineType(types: !22)
!22 = !{null}
!23 = !{}
!24 = !DIGlobalVariableExpression(var: !25, expr: !DIExpression())
!25 = distinct !DIGlobalVariable(name: "<dp_ex::Bird as dp_ex::Animal>::{vtable}", scope: null, file: !2, type: !26, isLocal: true, isDefinition: true)
!26 = !DICompositeType(tag: DW_TAG_structure_type, name: "<dp_ex::Bird as dp_ex::Animal>::{vtable_type}", file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !27, vtableHolder: !32, templateParams: !23, identifier: "3aabf6057d52011b8aa8f4102faed70c")
!27 = !{!28, !29, !30, !31}
!28 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !26, file: !2, baseType: !6, size: 64, align: 64)
!29 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !26, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!30 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !26, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!31 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !26, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!32 = !DICompositeType(tag: DW_TAG_structure_type, name: "Bird", scope: !33, file: !2, align: 8, flags: DIFlagPrivate, elements: !23, identifier: "857b71be6d6a563c67af310a295646b5")
!33 = !DINamespace(name: "dp_ex", scope: null)
!34 = !DIGlobalVariableExpression(var: !35, expr: !DIExpression())
!35 = distinct !DIGlobalVariable(name: "<dp_ex::Cat as dp_ex::Animal>::{vtable}", scope: null, file: !2, type: !36, isLocal: true, isDefinition: true)
!36 = !DICompositeType(tag: DW_TAG_structure_type, name: "<dp_ex::Cat as dp_ex::Animal>::{vtable_type}", file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !37, vtableHolder: !42, templateParams: !23, identifier: "8aed5e37cb0c0bc2cc51d7d91c393e70")
!37 = !{!38, !39, !40, !41}
!38 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !36, file: !2, baseType: !6, size: 64, align: 64)
!39 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !36, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!40 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !36, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!41 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !36, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!42 = !DICompositeType(tag: DW_TAG_structure_type, name: "Cat", scope: !33, file: !2, align: 8, flags: DIFlagPrivate, elements: !23, identifier: "12efbea2fcd1686f8a149626cc63f3a3")
!43 = !DIGlobalVariableExpression(var: !44, expr: !DIExpression())
!44 = distinct !DIGlobalVariable(name: "<dp_ex::Dog as dp_ex::Animal>::{vtable}", scope: null, file: !2, type: !45, isLocal: true, isDefinition: true)
!45 = !DICompositeType(tag: DW_TAG_structure_type, name: "<dp_ex::Dog as dp_ex::Animal>::{vtable_type}", file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !46, vtableHolder: !51, templateParams: !23, identifier: "7b8c77dee4800bd473b9436f1e39b6b9")
!46 = !{!47, !48, !49, !50}
!47 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !45, file: !2, baseType: !6, size: 64, align: 64)
!48 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !45, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!49 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !45, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!50 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !45, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!51 = !DICompositeType(tag: DW_TAG_structure_type, name: "Dog", scope: !33, file: !2, align: 8, flags: DIFlagPrivate, elements: !23, identifier: "6c214b66f1636284dbf33c78927b13b5")
!52 = !{i32 8, !"PIC Level", i32 2}
!53 = !{i32 7, !"PIE Level", i32 2}
!54 = !{i32 2, !"RtLibUseGOT", i32 1}
!55 = !{i32 7, !"Dwarf Version", i32 4}
!56 = !{i32 2, !"Debug Info Version", i32 3}
!57 = !{!"rustc version 1.87.0-nightly (ecade534c 2025-03-14)"}
!58 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !59, producer: "clang LLVM (rustc version 1.87.0-nightly (ecade534c 2025-03-14))", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !60, globals: !94, splitDebugInlining: false, nameTableKind: None)
!59 = !DIFile(filename: "src/main.rs/@/dp_ex.5c91ebfbd02dee49-cgu.0", directory: "/home/np/hack/verifopt/dp-ex")
!60 = !{!61, !69, !75, !84, !92}
!61 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Error", scope: !62, file: !2, baseType: !65, size: 8, align: 8, flags: DIFlagEnumClass, elements: !66)
!62 = !DINamespace(name: "uniform", scope: !63)
!63 = !DINamespace(name: "distr", scope: !64)
!64 = !DINamespace(name: "rand", scope: null)
!65 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!66 = !{!67, !68}
!67 = !DIEnumerator(name: "EmptyRange", value: 0, isUnsigned: true)
!68 = !DIEnumerator(name: "NonFinite", value: 1, isUnsigned: true)
!69 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "c_void", scope: !70, file: !2, baseType: !65, size: 8, align: 8, flags: DIFlagEnumClass, elements: !72)
!70 = !DINamespace(name: "ffi", scope: !71)
!71 = !DINamespace(name: "core", scope: null)
!72 = !{!73, !74}
!73 = !DIEnumerator(name: "__variant1", value: 0, isUnsigned: true)
!74 = !DIEnumerator(name: "__variant2", value: 1, isUnsigned: true)
!75 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Ordering", scope: !76, file: !2, baseType: !65, size: 8, align: 8, flags: DIFlagEnumClass, elements: !78)
!76 = !DINamespace(name: "atomic", scope: !77)
!77 = !DINamespace(name: "sync", scope: !71)
!78 = !{!79, !80, !81, !82, !83}
!79 = !DIEnumerator(name: "Relaxed", value: 0, isUnsigned: true)
!80 = !DIEnumerator(name: "Release", value: 1, isUnsigned: true)
!81 = !DIEnumerator(name: "Acquire", value: 2, isUnsigned: true)
!82 = !DIEnumerator(name: "AcqRel", value: 3, isUnsigned: true)
!83 = !DIEnumerator(name: "SeqCst", value: 4, isUnsigned: true)
!84 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Alignment", scope: !85, file: !2, baseType: !65, size: 8, align: 8, flags: DIFlagEnumClass, elements: !87)
!85 = !DINamespace(name: "rt", scope: !86)
!86 = !DINamespace(name: "fmt", scope: !71)
!87 = !{!88, !89, !90, !91}
!88 = !DIEnumerator(name: "Left", value: 0, isUnsigned: true)
!89 = !DIEnumerator(name: "Right", value: 1, isUnsigned: true)
!90 = !DIEnumerator(name: "Center", value: 2, isUnsigned: true)
!91 = !DIEnumerator(name: "Unknown", value: 3, isUnsigned: true)
!92 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Alignment", scope: !86, file: !2, baseType: !65, size: 8, align: 8, flags: DIFlagEnumClass, elements: !93)
!93 = !{!88, !89, !90}
!94 = !{!0, !95, !24, !34, !43}
!95 = !DIGlobalVariableExpression(var: !96, expr: !DIExpression())
!96 = distinct !DIGlobalVariable(name: "<rand::distr::uniform::Error as core::fmt::Debug>::{vtable}", scope: null, file: !2, type: !97, isLocal: true, isDefinition: true)
!97 = !DICompositeType(tag: DW_TAG_structure_type, name: "<rand::distr::uniform::Error as core::fmt::Debug>::{vtable_type}", file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !98, vtableHolder: !61, templateParams: !23, identifier: "76fbb543468239139f92e45b84443993")
!98 = !{!99, !100, !101, !102}
!99 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !97, file: !2, baseType: !6, size: 64, align: 64)
!100 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !97, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!101 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !97, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!102 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !97, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!103 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_ZN3std2rt10lang_start17h4be3234073074386E", scope: !16, file: !104, line: 192, type: !105, scopeLine: 192, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !115, retainedNodes: !110)
!104 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/std/src/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "5ed61ab28987f8860d5842313c6741b3")
!105 = !DISubroutineType(types: !106)
!106 = !{!107, !20, !107, !108, !65}
!107 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!108 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !109, size: 64, align: 64, dwarfAddressSpace: 0)
!109 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !65, size: 64, align: 64, dwarfAddressSpace: 0)
!110 = !{!111, !112, !113, !114}
!111 = !DILocalVariable(name: "main", arg: 1, scope: !103, file: !104, line: 193, type: !20)
!112 = !DILocalVariable(name: "argc", arg: 2, scope: !103, file: !104, line: 194, type: !107)
!113 = !DILocalVariable(name: "argv", arg: 3, scope: !103, file: !104, line: 195, type: !108)
!114 = !DILocalVariable(name: "sigpipe", arg: 4, scope: !103, file: !104, line: 196, type: !65)
!115 = !{!116}
!116 = !DITemplateTypeParameter(name: "T", type: !7)
!117 = !DILocation(line: 0, scope: !103)
!118 = !DILocation(line: 199, column: 10, scope: !103)
!119 = !DILocation(line: 198, column: 5, scope: !103)
!120 = !DILocation(line: 203, column: 5, scope: !103)
!121 = !DILocation(line: 204, column: 2, scope: !103)
!122 = distinct !DISubprogram(name: "{closure#0}<()>", linkageName: "_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h35042c385834d7a7E", scope: !15, file: !104, line: 199, type: !123, scopeLine: 199, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !115, retainedNodes: !127)
!123 = !DISubroutineType(types: !124)
!124 = !{!125, !126}
!125 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!126 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!127 = !{!128}
!128 = !DILocalVariable(name: "main", scope: !122, file: !104, line: 193, type: !20, align: 64)
!129 = !DILocation(line: 0, scope: !122)
!130 = !DILocation(line: 199, column: 70, scope: !122)
!131 = !DILocation(line: 199, column: 18, scope: !122)
!132 = !DILocation(line: 199, column: 93, scope: !122)
!133 = distinct !DISubprogram(name: "__rust_begin_short_backtrace<fn(), ()>", linkageName: "_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4d6c13392805e6c7E", scope: !135, file: !134, line: 148, type: !137, scopeLine: 148, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !143, retainedNodes: !139)
!134 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/std/src/sys/backtrace.rs", directory: "", checksumkind: CSK_MD5, checksum: "9e30c70624c3cf40238860e740bd696f")
!135 = !DINamespace(name: "backtrace", scope: !136)
!136 = !DINamespace(name: "sys", scope: !17)
!137 = !DISubroutineType(types: !138)
!138 = !{null, !20}
!139 = !{!140, !141}
!140 = !DILocalVariable(name: "f", arg: 1, scope: !133, file: !134, line: 148, type: !20)
!141 = !DILocalVariable(name: "result", scope: !142, file: !134, line: 152, type: !7, align: 8)
!142 = distinct !DILexicalBlock(scope: !133, file: !134, line: 152, column: 5)
!143 = !{!144, !116}
!144 = !DITemplateTypeParameter(name: "F", type: !20)
!145 = !DILocation(line: 152, column: 9, scope: !142)
!146 = !DILocation(line: 0, scope: !133)
!147 = !DILocalVariable(name: "dummy", scope: !148, file: !149, line: 476, type: !7, align: 8)
!148 = distinct !DISubprogram(name: "black_box<()>", linkageName: "_ZN4core4hint9black_box17hd82d6438fa6b0ff7E", scope: !150, file: !149, line: 476, type: !151, scopeLine: 476, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !115, retainedNodes: !153)
!149 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/hint.rs", directory: "", checksumkind: CSK_MD5, checksum: "53e417654697d2a6fdb3b165cec3a4bf")
!150 = !DINamespace(name: "hint", scope: !71)
!151 = !DISubroutineType(types: !152)
!152 = !{null, !7}
!153 = !{!147}
!154 = !DILocation(line: 476, column: 27, scope: !148, inlinedAt: !155)
!155 = !DILocation(line: 155, column: 5, scope: !142)
!156 = !DILocalVariable(arg: 1, scope: !157, file: !158, line: 250, type: !20)
!157 = distinct !DISubprogram(name: "call_once<fn(), ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17h63a06be7eb859f30E", scope: !159, file: !158, line: 250, type: !137, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !164, retainedNodes: !162)
!158 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/ops/function.rs", directory: "", checksumkind: CSK_MD5, checksum: "27f40bbdeb6cc525c0d0d7cf434d92c4")
!159 = !DINamespace(name: "FnOnce", scope: !160)
!160 = !DINamespace(name: "function", scope: !161)
!161 = !DINamespace(name: "ops", scope: !71)
!162 = !{!156, !163}
!163 = !DILocalVariable(arg: 2, scope: !157, file: !158, line: 250, type: !7)
!164 = !{!165, !166}
!165 = !DITemplateTypeParameter(name: "Self", type: !20)
!166 = !DITemplateTypeParameter(name: "Args", type: !7)
!167 = !DILocation(line: 0, scope: !157, inlinedAt: !168)
!168 = distinct !DILocation(line: 152, column: 18, scope: !133)
!169 = !DILocation(line: 250, column: 5, scope: !157, inlinedAt: !168)
!170 = !DILocation(line: 477, column: 5, scope: !148, inlinedAt: !155)
!171 = !{i64 16984190477948645}
!172 = !DILocation(line: 158, column: 2, scope: !133)
!173 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17ha781119b1797b6ceE", scope: !159, file: !158, line: 250, type: !174, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !180, retainedNodes: !177)
!174 = !DISubroutineType(types: !175)
!175 = !{!125, !176}
!176 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!177 = !{!178, !179}
!178 = !DILocalVariable(arg: 1, scope: !173, file: !158, line: 250, type: !176)
!179 = !DILocalVariable(arg: 2, scope: !173, file: !158, line: 250, type: !7)
!180 = !{!181, !166}
!181 = !DITemplateTypeParameter(name: "Self", type: !14)
!182 = !DILocation(line: 0, scope: !173)
!183 = !DILocation(line: 250, column: 5, scope: !173)
!184 = !DILocation(line: 0, scope: !122, inlinedAt: !185)
!185 = distinct !DILocation(line: 250, column: 5, scope: !186, inlinedAt: !192)
!186 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17h2ea33e7f40dac79eE", scope: !159, file: !158, line: 250, type: !187, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !180, retainedNodes: !189)
!187 = !DISubroutineType(types: !188)
!188 = !{!125, !14}
!189 = !{!190, !191}
!190 = !DILocalVariable(arg: 1, scope: !186, file: !158, line: 250, type: !14)
!191 = !DILocalVariable(arg: 2, scope: !186, file: !158, line: 250, type: !7)
!192 = distinct !DILocation(line: 250, column: 5, scope: !173)
!193 = !DILocation(line: 0, scope: !186, inlinedAt: !192)
!194 = !DILocation(line: 250, column: 5, scope: !186, inlinedAt: !192)
!195 = !DILocation(line: 199, column: 18, scope: !122, inlinedAt: !185)
!196 = !{!197}
!197 = distinct !{!197, !198, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h35042c385834d7a7E: %_1"}
!198 = distinct !{!198, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h35042c385834d7a7E"}
!199 = distinct !DISubprogram(name: "reseed_and_generate<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17ha6228666c915f093E", scope: !201, file: !200, line: 216, type: !253, scopeLine: 216, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !250, declaration: !265, retainedNodes: !266)
!200 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/rngs/reseeding.rs", directory: "", checksumkind: CSK_MD5, checksum: "b59ac7f2685a0e488478c11615cc2565")
!201 = !DICompositeType(tag: DW_TAG_structure_type, name: "ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", scope: !202, file: !2, size: 512, align: 128, flags: DIFlagPrivate, elements: !204, templateParams: !250, identifier: "592b3932b2f8878cd5d3f17e71dddcb2")
!202 = !DINamespace(name: "reseeding", scope: !203)
!203 = !DINamespace(name: "rngs", scope: !64)
!204 = !{!205, !244, !248, !249}
!205 = !DIDerivedType(tag: DW_TAG_member, name: "inner", scope: !201, file: !2, baseType: !206, size: 384, align: 128, flags: DIFlagPrivate)
!206 = !DICompositeType(tag: DW_TAG_structure_type, name: "ChaCha12Core", scope: !207, file: !2, size: 384, align: 128, flags: DIFlagPublic, elements: !209, templateParams: !23, identifier: "2a2910802ded106bd4b70f79d2f9b222")
!207 = !DINamespace(name: "chacha", scope: !208)
!208 = !DINamespace(name: "rand_chacha", scope: null)
!209 = !{!210}
!210 = !DIDerivedType(tag: DW_TAG_member, name: "state", scope: !206, file: !2, baseType: !211, size: 384, align: 128, flags: DIFlagPrivate)
!211 = !DICompositeType(tag: DW_TAG_structure_type, name: "ChaCha", scope: !212, file: !2, size: 384, align: 128, flags: DIFlagPublic, elements: !213, templateParams: !23, identifier: "f0798a57f4b61d4970b465b77f0a9e05")
!212 = !DINamespace(name: "guts", scope: !208)
!213 = !{!214, !242, !243}
!214 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !211, file: !2, baseType: !215, size: 128, align: 128, flags: DIFlagProtected)
!215 = !DICompositeType(tag: DW_TAG_union_type, name: "vec128_storage", scope: !216, file: !2, size: 128, align: 128, elements: !218, templateParams: !23, identifier: "f389b8a1c6d54643d04728b3b45ccbef")
!216 = !DINamespace(name: "x86_64", scope: !217)
!217 = !DINamespace(name: "ppv_lite86", scope: null)
!218 = !{!219, !224, !229, !234}
!219 = !DIDerivedType(tag: DW_TAG_member, name: "u32x4", scope: !215, file: !2, baseType: !220, size: 128, align: 32)
!220 = !DICompositeType(tag: DW_TAG_array_type, baseType: !221, size: 128, align: 32, elements: !222)
!221 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!222 = !{!223}
!223 = !DISubrange(count: 4, lowerBound: 0)
!224 = !DIDerivedType(tag: DW_TAG_member, name: "u64x2", scope: !215, file: !2, baseType: !225, size: 128, align: 64)
!225 = !DICompositeType(tag: DW_TAG_array_type, baseType: !226, size: 128, align: 64, elements: !227)
!226 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!227 = !{!228}
!228 = !DISubrange(count: 2, lowerBound: 0)
!229 = !DIDerivedType(tag: DW_TAG_member, name: "u128x1", scope: !215, file: !2, baseType: !230, size: 128, align: 128)
!230 = !DICompositeType(tag: DW_TAG_array_type, baseType: !231, size: 128, align: 128, elements: !232)
!231 = !DIBasicType(name: "u128", size: 128, encoding: DW_ATE_unsigned)
!232 = !{!233}
!233 = !DISubrange(count: 1, lowerBound: 0)
!234 = !DIDerivedType(tag: DW_TAG_member, name: "sse2", scope: !215, file: !2, baseType: !235, size: 128, align: 128)
!235 = !DICompositeType(tag: DW_TAG_structure_type, name: "__m128i", scope: !236, file: !2, size: 128, align: 128, flags: DIFlagPublic, elements: !238, templateParams: !23, identifier: "ab2da2cf8a57b425a31dcc9ff8b98d2e")
!236 = !DINamespace(name: "x86", scope: !237)
!237 = !DINamespace(name: "core_arch", scope: !71)
!238 = !{!239}
!239 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !235, file: !2, baseType: !240, size: 128, align: 64, flags: DIFlagPrivate)
!240 = !DICompositeType(tag: DW_TAG_array_type, baseType: !241, size: 128, align: 64, elements: !227)
!241 = !DIBasicType(name: "i64", size: 64, encoding: DW_ATE_signed)
!242 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !211, file: !2, baseType: !215, size: 128, align: 128, offset: 128, flags: DIFlagProtected)
!243 = !DIDerivedType(tag: DW_TAG_member, name: "d", scope: !211, file: !2, baseType: !215, size: 128, align: 128, offset: 256, flags: DIFlagProtected)
!244 = !DIDerivedType(tag: DW_TAG_member, name: "reseeder", scope: !201, file: !2, baseType: !245, align: 8, offset: 512, flags: DIFlagPrivate)
!245 = !DICompositeType(tag: DW_TAG_structure_type, name: "OsRng", scope: !246, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "abe41dd699169f8d4175b6cf380563a2")
!246 = !DINamespace(name: "os", scope: !247)
!247 = !DINamespace(name: "rand_core", scope: null)
!248 = !DIDerivedType(tag: DW_TAG_member, name: "threshold", scope: !201, file: !2, baseType: !241, size: 64, align: 64, offset: 384, flags: DIFlagPrivate)
!249 = !DIDerivedType(tag: DW_TAG_member, name: "bytes_until_reseed", scope: !201, file: !2, baseType: !241, size: 64, align: 64, offset: 448, flags: DIFlagPrivate)
!250 = !{!251, !252}
!251 = !DITemplateTypeParameter(name: "R", type: !206)
!252 = !DITemplateTypeParameter(name: "Rsdr", type: !245)
!253 = !DISubroutineType(types: !254)
!254 = !{null, !255, !256}
!255 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", baseType: !201, size: 64, align: 64, dwarfAddressSpace: 0)
!256 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand_chacha::chacha::Array64<u32>", baseType: !257, size: 64, align: 64, dwarfAddressSpace: 0)
!257 = !DICompositeType(tag: DW_TAG_structure_type, name: "Array64<u32>", scope: !207, file: !2, size: 2048, align: 32, flags: DIFlagPublic, elements: !258, templateParams: !263, identifier: "426977da845494fc1dbde4983524ce31")
!258 = !{!259}
!259 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !257, file: !2, baseType: !260, size: 2048, align: 32, flags: DIFlagPrivate)
!260 = !DICompositeType(tag: DW_TAG_array_type, baseType: !221, size: 2048, align: 32, elements: !261)
!261 = !{!262}
!262 = !DISubrange(count: 64, lowerBound: 0)
!263 = !{!264}
!264 = !DITemplateTypeParameter(name: "T", type: !221)
!265 = !DISubprogram(name: "reseed_and_generate<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17ha6228666c915f093E", scope: !201, file: !200, line: 216, type: !253, scopeLine: 216, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !250)
!266 = !{!267, !268, !269, !271}
!267 = !DILocalVariable(name: "self", arg: 1, scope: !199, file: !200, line: 216, type: !255)
!268 = !DILocalVariable(name: "results", arg: 2, scope: !199, file: !200, line: 216, type: !256)
!269 = !DILocalVariable(name: "num_bytes", scope: !270, file: !200, line: 219, type: !9, align: 64)
!270 = distinct !DILexicalBlock(scope: !199, file: !200, line: 219, column: 9)
!271 = !DILocalVariable(name: "e", scope: !272, file: !200, line: 221, type: !273, align: 32)
!272 = distinct !DILexicalBlock(scope: !270, file: !200, line: 221, column: 39)
!273 = !DICompositeType(tag: DW_TAG_structure_type, name: "OsError", scope: !246, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !274, templateParams: !23, identifier: "4d5a6fcb535b7bb557d8bbb8131dad70")
!274 = !{!275}
!275 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !273, file: !2, baseType: !276, size: 32, align: 32, flags: DIFlagPrivate)
!276 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !277, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !279, templateParams: !23, identifier: "bc3dadadf2389ed099352a9bc0641d16")
!277 = !DINamespace(name: "error", scope: !278)
!278 = !DINamespace(name: "getrandom", scope: null)
!279 = !{!280}
!280 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !276, file: !2, baseType: !281, size: 32, align: 32, flags: DIFlagPrivate)
!281 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonZero<i32>", scope: !282, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !284, templateParams: !290, identifier: "32be11596a496ede934ffbdd9b95b3e8")
!282 = !DINamespace(name: "nonzero", scope: !283)
!283 = !DINamespace(name: "num", scope: !71)
!284 = !{!285}
!285 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !281, file: !2, baseType: !286, size: 32, align: 32, flags: DIFlagPrivate)
!286 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonZeroI32Inner", scope: !287, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !288, templateParams: !23, identifier: "7498a545642b9ccf8476c6ffe07e1717")
!287 = !DINamespace(name: "niche_types", scope: !283)
!288 = !{!289}
!289 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !286, file: !2, baseType: !125, size: 32, align: 32, flags: DIFlagPrivate)
!290 = !{!291}
!291 = !DITemplateTypeParameter(name: "T", type: !125)
!292 = !DILocation(line: 0, scope: !199)
!293 = !DILocation(line: 0, scope: !270)
!294 = !{!295}
!295 = distinct !{!295, !296, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E: %self"}
!296 = distinct !{!296, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E"}
!297 = !DILocation(line: 221, column: 25, scope: !272)
!298 = !DILocalVariable(name: "self", arg: 1, scope: !299, file: !200, line: 208, type: !255)
!299 = distinct !DISubprogram(name: "reseed<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E", scope: !201, file: !200, line: 208, type: !300, scopeLine: 208, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !250, declaration: !318, retainedNodes: !319)
!300 = !DISubroutineType(types: !301)
!301 = !{!302, !255}
!302 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), rand_core::os::OsError>", scope: !303, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !304, templateParams: !23, identifier: "ee302a6896271acd50a47cf25c1c8f76")
!303 = !DINamespace(name: "result", scope: !71)
!304 = !{!305}
!305 = !DICompositeType(tag: DW_TAG_variant_part, scope: !302, file: !2, size: 32, align: 32, elements: !306, templateParams: !23, identifier: "1cdf423128631b3680c1e059f5770637", discriminator: !317)
!306 = !{!307, !313}
!307 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !305, file: !2, baseType: !308, size: 32, align: 32, extraData: i32 0)
!308 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !302, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !309, templateParams: !311, identifier: "14c5d45082ecd4eeb838afc989facba8")
!309 = !{!310}
!310 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !308, file: !2, baseType: !7, align: 8, flags: DIFlagPublic)
!311 = !{!116, !312}
!312 = !DITemplateTypeParameter(name: "E", type: !273)
!313 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !305, file: !2, baseType: !314, size: 32, align: 32)
!314 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !302, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !315, templateParams: !311, identifier: "f383eed7d45070a1d992c59d2e554b28")
!315 = !{!316}
!316 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !314, file: !2, baseType: !273, size: 32, align: 32, flags: DIFlagPublic)
!317 = !DIDerivedType(tag: DW_TAG_member, scope: !302, file: !2, baseType: !221, size: 32, align: 32, flags: DIFlagArtificial)
!318 = !DISubprogram(name: "reseed<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E", scope: !201, file: !200, line: 208, type: !300, scopeLine: 208, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !250)
!319 = !{!298}
!320 = !DILocation(line: 0, scope: !299, inlinedAt: !321)
!321 = distinct !DILocation(line: 221, column: 25, scope: !272)
!322 = !DILocalVariable(name: "self", arg: 1, scope: !323, file: !324, line: 771, type: !325)
!323 = distinct !DISubprogram(name: "map<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsError, (), rand::rngs::reseeding::{impl#5}::reseed::{closure_env#0}<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN4core6result19Result$LT$T$C$E$GT$3map17h4156b7560f7a9c10E", scope: !325, file: !324, line: 771, type: !340, scopeLine: 771, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !353, declaration: !352, retainedNodes: !356)
!324 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/result.rs", directory: "", checksumkind: CSK_MD5, checksum: "b616a11c95aa850b4e5fe6b50aa03751")
!325 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsError>", scope: !303, file: !2, size: 512, align: 128, flags: DIFlagPublic, elements: !326, templateParams: !23, identifier: "5a68524507abe77b21abce2d59cb79f3")
!326 = !{!327}
!327 = !DICompositeType(tag: DW_TAG_variant_part, scope: !325, file: !2, size: 512, align: 128, elements: !328, templateParams: !23, identifier: "ae53ef526a0aa49668600a3ee4b26de5", discriminator: !339)
!328 = !{!329, !335}
!329 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !327, file: !2, baseType: !330, size: 512, align: 128, extraData: i32 0)
!330 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !325, file: !2, size: 512, align: 128, flags: DIFlagPublic, elements: !331, templateParams: !333, identifier: "c22ba6c20c5e86669f5a11b08ae418f")
!331 = !{!332}
!332 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !330, file: !2, baseType: !206, size: 384, align: 128, offset: 128, flags: DIFlagPublic)
!333 = !{!334, !312}
!334 = !DITemplateTypeParameter(name: "T", type: !206)
!335 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !327, file: !2, baseType: !336, size: 512, align: 128, extraData: i32 1)
!336 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !325, file: !2, size: 512, align: 128, flags: DIFlagPublic, elements: !337, templateParams: !333, identifier: "751e6797583144ab3d176d63bf35e4b9")
!337 = !{!338}
!338 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !336, file: !2, baseType: !273, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!339 = !DIDerivedType(tag: DW_TAG_member, scope: !325, file: !2, baseType: !221, size: 32, align: 32, flags: DIFlagArtificial)
!340 = !DISubroutineType(types: !341)
!341 = !{!302, !325, !342}
!342 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", scope: !343, file: !2, size: 192, align: 64, elements: !345, templateParams: !23, identifier: "d2c5aa1ace3eff00c74842752244a6ab")
!343 = !DINamespace(name: "reseed", scope: !344)
!344 = !DINamespace(name: "{impl#5}", scope: !202)
!345 = !{!346, !348, !350}
!346 = !DIDerivedType(tag: DW_TAG_member, name: "_ref__self__inner", scope: !342, file: !2, baseType: !347, size: 64, align: 64)
!347 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand_chacha::chacha::ChaCha12Core", baseType: !206, size: 64, align: 64, dwarfAddressSpace: 0)
!348 = !DIDerivedType(tag: DW_TAG_member, name: "_ref__self__threshold", scope: !342, file: !2, baseType: !349, size: 64, align: 64, offset: 64)
!349 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&i64", baseType: !241, size: 64, align: 64, dwarfAddressSpace: 0)
!350 = !DIDerivedType(tag: DW_TAG_member, name: "_ref__self__bytes_until_reseed", scope: !342, file: !2, baseType: !351, size: 64, align: 64, offset: 128)
!351 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut i64", baseType: !241, size: 64, align: 64, dwarfAddressSpace: 0)
!352 = !DISubprogram(name: "map<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsError, (), rand::rngs::reseeding::{impl#5}::reseed::{closure_env#0}<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN4core6result19Result$LT$T$C$E$GT$3map17h4156b7560f7a9c10E", scope: !325, file: !324, line: 771, type: !340, scopeLine: 771, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !353)
!353 = !{!334, !312, !354, !355}
!354 = !DITemplateTypeParameter(name: "U", type: !7)
!355 = !DITemplateTypeParameter(name: "F", type: !342)
!356 = !{!322, !357, !358, !360}
!357 = !DILocalVariable(name: "op", arg: 2, scope: !323, file: !324, line: 771, type: !342)
!358 = !DILocalVariable(name: "t", scope: !359, file: !324, line: 773, type: !206, align: 128)
!359 = distinct !DILexicalBlock(scope: !323, file: !324, line: 773, column: 13)
!360 = !DILocalVariable(name: "e", scope: !361, file: !324, line: 774, type: !273, align: 32)
!361 = distinct !DILexicalBlock(scope: !323, file: !324, line: 774, column: 13)
!362 = !DILocation(line: 771, column: 38, scope: !323, inlinedAt: !363)
!363 = distinct !DILocation(line: 209, column: 45, scope: !299, inlinedAt: !321)
!364 = !DILocation(line: 773, column: 16, scope: !359, inlinedAt: !363)
!365 = !DILocation(line: 209, column: 9, scope: !299, inlinedAt: !321)
!366 = !DILocalVariable(name: "rng", arg: 1, scope: !367, file: !368, line: 530, type: !372)
!367 = distinct !DISubprogram(name: "try_from_rng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN9rand_core11SeedableRng12try_from_rng17h529124245fe1aebcE", scope: !369, file: !368, line: 530, type: !370, scopeLine: 530, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !401, retainedNodes: !373)
!368 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_core-0.9.3/src/lib.rs", directory: "", checksumkind: CSK_MD5, checksum: "b6e5d656dad488a66c257847601969aa")
!369 = !DINamespace(name: "SeedableRng", scope: !247)
!370 = !DISubroutineType(types: !371)
!371 = !{!325, !372}
!372 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand_core::os::OsRng", baseType: !245, size: 64, align: 64, dwarfAddressSpace: 0)
!373 = !{!366, !374, !379, !399}
!374 = !DILocalVariable(name: "seed", scope: !375, file: !368, line: 531, type: !376, align: 8)
!375 = distinct !DILexicalBlock(scope: !367, file: !368, line: 531, column: 9)
!376 = !DICompositeType(tag: DW_TAG_array_type, baseType: !65, size: 256, align: 8, elements: !377)
!377 = !{!378}
!378 = !DISubrange(count: 32, lowerBound: 0)
!379 = !DILocalVariable(name: "residual", scope: !380, file: !368, line: 532, type: !381, align: 32)
!380 = distinct !DILexicalBlock(scope: !375, file: !368, line: 532, column: 42)
!381 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<core::convert::Infallible, rand_core::os::OsError>", scope: !303, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !382, templateParams: !23, identifier: "48a4bc2942fe3ff0b0ef592339a5854f")
!382 = !{!383}
!383 = !DICompositeType(tag: DW_TAG_variant_part, scope: !381, file: !2, size: 32, align: 32, elements: !384, templateParams: !23, identifier: "2d3be197a9eacbf9b36cebc5831dce97")
!384 = !{!385, !395}
!385 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !383, file: !2, baseType: !386, size: 32, align: 32)
!386 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !381, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !387, templateParams: !393, identifier: "ab9232abdde0e58b5f574c66d8cfafdb")
!387 = !{!388}
!388 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !386, file: !2, baseType: !389, align: 8, flags: DIFlagPublic)
!389 = !DICompositeType(tag: DW_TAG_structure_type, name: "Infallible", scope: !390, file: !2, align: 8, flags: DIFlagPublic, elements: !391, templateParams: !23, identifier: "e51c7217cff47e0428772b48782f2c6e")
!390 = !DINamespace(name: "convert", scope: !71)
!391 = !{!392}
!392 = !DICompositeType(tag: DW_TAG_variant_part, scope: !389, file: !2, align: 8, elements: !23, identifier: "578ba6599628555a2e7430e64d95f8d9")
!393 = !{!394, !312}
!394 = !DITemplateTypeParameter(name: "T", type: !389)
!395 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !383, file: !2, baseType: !396, size: 32, align: 32)
!396 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !381, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !397, templateParams: !393, identifier: "dc8c092dd34505ff40d3a1d90c4495d4")
!397 = !{!398}
!398 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !396, file: !2, baseType: !273, size: 32, align: 32, flags: DIFlagPublic)
!399 = !DILocalVariable(name: "val", scope: !400, file: !368, line: 532, type: !7, align: 8)
!400 = distinct !DILexicalBlock(scope: !375, file: !368, line: 532, column: 9)
!401 = !{!402, !403}
!402 = !DITemplateTypeParameter(name: "Self", type: !206)
!403 = !DITemplateTypeParameter(name: "R", type: !245)
!404 = !DILocation(line: 0, scope: !367, inlinedAt: !405)
!405 = distinct !DILocation(line: 209, column: 9, scope: !299, inlinedAt: !321)
!406 = !DILocation(line: 531, column: 13, scope: !375, inlinedAt: !405)
!407 = !DILocation(line: 531, column: 13, scope: !367, inlinedAt: !405)
!408 = !{!409, !295}
!409 = distinct !{!409, !410, !"_ZN9rand_core11SeedableRng12try_from_rng17h529124245fe1aebcE: %_0"}
!410 = distinct !{!410, !"_ZN9rand_core11SeedableRng12try_from_rng17h529124245fe1aebcE"}
!411 = !DILocation(line: 456, column: 17, scope: !412, inlinedAt: !420)
!412 = distinct !DISubprogram(name: "default<u8>", linkageName: "_ZN4core5array76_$LT$impl$u20$core..default..Default$u20$for$u20$$u5b$T$u3b$$u20$32$u5d$$GT$7default17h66f1ccdc17cac30eE", scope: !414, file: !413, line: 455, type: !416, scopeLine: 455, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !418)
!413 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/array/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "a5d4988c8e4a2c0226c27eed7d0d1450")
!414 = !DINamespace(name: "{impl#29}", scope: !415)
!415 = !DINamespace(name: "array", scope: !71)
!416 = !DISubroutineType(types: !417)
!417 = !{!376}
!418 = !{!419}
!419 = !DITemplateTypeParameter(name: "T", type: !65)
!420 = distinct !DILocation(line: 531, column: 24, scope: !367, inlinedAt: !405)
!421 = !{!422}
!422 = distinct !{!422, !423, !"_ZN4core5array76_$LT$impl$u20$core..default..Default$u20$for$u20$$u5b$T$u3b$$u20$32$u5d$$GT$7default17h66f1ccdc17cac30eE: %_0"}
!423 = distinct !{!423, !"_ZN4core5array76_$LT$impl$u20$core..default..Default$u20$for$u20$$u5b$T$u3b$$u20$32$u5d$$GT$7default17h66f1ccdc17cac30eE"}
!424 = !DILocalVariable(name: "self", arg: 1, scope: !425, file: !426, line: 97, type: !372)
!425 = distinct !DISubprogram(name: "try_fill_bytes", linkageName: "_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E", scope: !427, file: !426, line: 97, type: !428, scopeLine: 97, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !435)
!426 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_core-0.9.3/src/os.rs", directory: "", checksumkind: CSK_MD5, checksum: "a9b676e61e33e2a3ffa0c1bbc3d87d95")
!427 = !DINamespace(name: "{impl#3}", scope: !246)
!428 = !DISubroutineType(types: !429)
!429 = !{!302, !372, !430}
!430 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut [u8]", file: !2, size: 128, align: 64, elements: !431, templateParams: !23, identifier: "bdfeb4840e2373d8742974745efe30b6")
!431 = !{!432, !434}
!432 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !430, file: !2, baseType: !433, size: 64, align: 64)
!433 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !65, size: 64, align: 64, dwarfAddressSpace: 0)
!434 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !430, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!435 = !{!424, !436}
!436 = !DILocalVariable(name: "dest", arg: 2, scope: !425, file: !426, line: 97, type: !430)
!437 = !DILocation(line: 0, scope: !425, inlinedAt: !438)
!438 = distinct !DILocation(line: 532, column: 9, scope: !375, inlinedAt: !405)
!439 = !DILocalVariable(name: "dest", arg: 1, scope: !440, file: !441, line: 66, type: !430)
!440 = distinct !DISubprogram(name: "fill", linkageName: "_ZN9getrandom4fill17h11f2509b4e4fb8bcE", scope: !278, file: !441, line: 66, type: !442, scopeLine: 66, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !459)
!441 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.3/src/lib.rs", directory: "", checksumkind: CSK_MD5, checksum: "1faf0978fd7ba93e5b0c3d32f0a6715f")
!442 = !DISubroutineType(types: !443)
!443 = !{!444, !430}
!444 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), getrandom::error::Error>", scope: !303, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !445, templateParams: !23, identifier: "83e4a8b163ed8d485ec46c0869a1009a")
!445 = !{!446}
!446 = !DICompositeType(tag: DW_TAG_variant_part, scope: !444, file: !2, size: 32, align: 32, elements: !447, templateParams: !23, identifier: "29b45ebd9e1820ce85de3dcc5076dc9c", discriminator: !458)
!447 = !{!448, !454}
!448 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !446, file: !2, baseType: !449, size: 32, align: 32, extraData: i32 0)
!449 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !444, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !450, templateParams: !452, identifier: "d0a120dd63ad1ccf4335a0a05f38d8fa")
!450 = !{!451}
!451 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !449, file: !2, baseType: !7, align: 8, flags: DIFlagPublic)
!452 = !{!116, !453}
!453 = !DITemplateTypeParameter(name: "E", type: !276)
!454 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !446, file: !2, baseType: !455, size: 32, align: 32)
!455 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !444, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !456, templateParams: !452, identifier: "8e6e5cc4b064d87a864fd2209187218d")
!456 = !{!457}
!457 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !455, file: !2, baseType: !276, size: 32, align: 32, flags: DIFlagPublic)
!458 = !DIDerivedType(tag: DW_TAG_member, scope: !444, file: !2, baseType: !221, size: 32, align: 32, flags: DIFlagArtificial)
!459 = !{!439, !460, !475}
!460 = !DILocalVariable(name: "residual", scope: !461, file: !441, line: 70, type: !462, align: 32)
!461 = distinct !DILexicalBlock(scope: !440, file: !441, line: 70, column: 60)
!462 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<core::convert::Infallible, getrandom::error::Error>", scope: !303, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !463, templateParams: !23, identifier: "a0a20b2ab4efb093be088acdc4520dc")
!463 = !{!464}
!464 = !DICompositeType(tag: DW_TAG_variant_part, scope: !462, file: !2, size: 32, align: 32, elements: !465, templateParams: !23, identifier: "de36cef668fd27db27cedfcae36838f1")
!465 = !{!466, !471}
!466 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !464, file: !2, baseType: !467, size: 32, align: 32)
!467 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !462, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !468, templateParams: !470, identifier: "6ffcd2c901f08b8c522acfcf7317934a")
!468 = !{!469}
!469 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !467, file: !2, baseType: !389, align: 8, flags: DIFlagPublic)
!470 = !{!394, !453}
!471 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !464, file: !2, baseType: !472, size: 32, align: 32)
!472 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !462, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !473, templateParams: !470, identifier: "84c7c91ab022bcf3bb8a875f2b92cc4")
!473 = !{!474}
!474 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !472, file: !2, baseType: !276, size: 32, align: 32, flags: DIFlagPublic)
!475 = !DILocalVariable(name: "val", scope: !476, file: !441, line: 70, type: !430, align: 64)
!476 = distinct !DILexicalBlock(scope: !440, file: !441, line: 70, column: 5)
!477 = !DILocation(line: 0, scope: !440, inlinedAt: !478)
!478 = distinct !DILocation(line: 98, column: 9, scope: !425, inlinedAt: !438)
!479 = !DILocalVariable(name: "dest", arg: 1, scope: !480, file: !441, line: 97, type: !498)
!480 = distinct !DISubprogram(name: "fill_uninit", linkageName: "_ZN9getrandom11fill_uninit17h0bff8f15f1575c4eE", scope: !278, file: !441, line: 97, type: !481, scopeLine: 97, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !513)
!481 = !DISubroutineType(types: !482)
!482 = !{!483, !498}
!483 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<&mut [u8], getrandom::error::Error>", scope: !303, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !484, templateParams: !23, identifier: "d5b271eb8b6a9471c1c14843be9bb67a")
!484 = !{!485}
!485 = !DICompositeType(tag: DW_TAG_variant_part, scope: !483, file: !2, size: 128, align: 64, elements: !486, templateParams: !23, identifier: "f7ea3bd174bcc52c21117617ed826c", discriminator: !497)
!486 = !{!487, !493}
!487 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !485, file: !2, baseType: !488, size: 128, align: 64)
!488 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !483, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !489, templateParams: !491, identifier: "ff0a1fc0f460c66d01b4f70777872d9")
!489 = !{!490}
!490 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !488, file: !2, baseType: !430, size: 128, align: 64, flags: DIFlagPublic)
!491 = !{!492, !453}
!492 = !DITemplateTypeParameter(name: "T", type: !430)
!493 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !485, file: !2, baseType: !494, size: 128, align: 64, extraData: i64 0)
!494 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !483, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !495, templateParams: !491, identifier: "cb75b72154a5ea0567d23dcf4a9ebbde")
!495 = !{!496}
!496 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !494, file: !2, baseType: !276, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
!497 = !DIDerivedType(tag: DW_TAG_member, scope: !483, file: !2, baseType: !226, size: 64, align: 64, flags: DIFlagArtificial)
!498 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut [core::mem::maybe_uninit::MaybeUninit<u8>]", file: !2, size: 128, align: 64, elements: !499, templateParams: !23, identifier: "dde0e961468dcbc0c165acf044bfc7c3")
!499 = !{!500, !512}
!500 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !498, file: !2, baseType: !501, size: 64, align: 64)
!501 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !502, size: 64, align: 64, dwarfAddressSpace: 0)
!502 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<u8>", scope: !503, file: !2, size: 8, align: 8, elements: !505, templateParams: !418, identifier: "33985afa3139b64ff19925f3840c2e44")
!503 = !DINamespace(name: "maybe_uninit", scope: !504)
!504 = !DINamespace(name: "mem", scope: !71)
!505 = !{!506, !507}
!506 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !502, file: !2, baseType: !7, align: 8)
!507 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !502, file: !2, baseType: !508, size: 8, align: 8)
!508 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<u8>", scope: !509, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !510, templateParams: !418, identifier: "fd8a7b1ca73aa88217cd1fad649a76fb")
!509 = !DINamespace(name: "manually_drop", scope: !504)
!510 = !{!511}
!511 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !508, file: !2, baseType: !65, size: 8, align: 8, flags: DIFlagPrivate)
!512 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !498, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!513 = !{!479, !514, !516}
!514 = !DILocalVariable(name: "residual", scope: !515, file: !441, line: 99, type: !462, align: 32)
!515 = distinct !DILexicalBlock(scope: !480, file: !441, line: 99, column: 35)
!516 = !DILocalVariable(name: "val", scope: !517, file: !441, line: 99, type: !7, align: 8)
!517 = distinct !DILexicalBlock(scope: !480, file: !441, line: 99, column: 9)
!518 = !DILocation(line: 0, scope: !480, inlinedAt: !519)
!519 = distinct !DILocation(line: 70, column: 5, scope: !440, inlinedAt: !478)
!520 = !DILocalVariable(name: "sys_fill", arg: 2, scope: !521, file: !522, line: 58, type: !528)
!521 = distinct !DISubprogram(name: "sys_fill_exact<getrandom::backends::linux_android_with_fallback::fill_inner::{closure_env#0}>", linkageName: "_ZN9getrandom8backends8use_file9util_libc14sys_fill_exact17ha95d0b037dbe5276E", scope: !523, file: !522, line: 56, type: !526, scopeLine: 56, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !558, retainedNodes: !538)
!522 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.3/src/backends/../util_libc.rs", directory: "", checksumkind: CSK_MD5, checksum: "8e2d9aabad1e84344d4c4477ca7315d7")
!523 = !DINamespace(name: "util_libc", scope: !524)
!524 = !DINamespace(name: "use_file", scope: !525)
!525 = !DINamespace(name: "backends", scope: !278)
!526 = !DISubroutineType(types: !527)
!527 = !{!444, !498, !528}
!528 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}", scope: !529, file: !2, size: 64, align: 64, elements: !531, templateParams: !23, identifier: "2a868b422fb418a23e766acb60f7a64e")
!529 = !DINamespace(name: "fill_inner", scope: !530)
!530 = !DINamespace(name: "linux_android_with_fallback", scope: !525)
!531 = !{!532}
!532 = !DIDerivedType(tag: DW_TAG_member, name: "_ref__getrandom_fn", scope: !528, file: !2, baseType: !533, size: 64, align: 64)
!533 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&unsafe extern \22C\22 fn(*mut core::ffi::c_void, usize, u32) -> isize", baseType: !534, size: 64, align: 64, dwarfAddressSpace: 0)
!534 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "unsafe extern \22C\22 fn(*mut core::ffi::c_void, usize, u32) -> isize", baseType: !535, size: 64, align: 64, dwarfAddressSpace: 0)
!535 = !DISubroutineType(types: !536)
!536 = !{!107, !537, !9, !221}
!537 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut core::ffi::c_void", baseType: !69, size: 64, align: 64, dwarfAddressSpace: 0)
!538 = !{!539, !520, !540, !542, !544, !546, !548, !550, !552, !554, !556}
!539 = !DILocalVariable(name: "buf", arg: 1, scope: !521, file: !522, line: 57, type: !498)
!540 = !DILocalVariable(name: "res", scope: !541, file: !522, line: 61, type: !107, align: 64)
!541 = distinct !DILexicalBlock(scope: !521, file: !522, line: 61, column: 9)
!542 = !DILocalVariable(name: "res", scope: !543, file: !522, line: 63, type: !107, align: 64)
!543 = distinct !DILexicalBlock(scope: !541, file: !522, line: 63, column: 13)
!544 = !DILocalVariable(name: "res", scope: !543, file: !522, line: 63, type: !545, align: 64)
!545 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&isize", baseType: !107, size: 64, align: 64, dwarfAddressSpace: 0)
!546 = !DILocalVariable(name: "len", scope: !547, file: !522, line: 64, type: !9, align: 64)
!547 = distinct !DILexicalBlock(scope: !543, file: !522, line: 64, column: 17)
!548 = !DILocalVariable(name: "residual", scope: !549, file: !522, line: 64, type: !462, align: 32)
!549 = distinct !DILexicalBlock(scope: !543, file: !522, line: 64, column: 78)
!550 = !DILocalVariable(name: "val", scope: !551, file: !522, line: 64, type: !9, align: 64)
!551 = distinct !DILexicalBlock(scope: !543, file: !522, line: 64, column: 27)
!552 = !DILocalVariable(name: "residual", scope: !553, file: !522, line: 65, type: !462, align: 32)
!553 = distinct !DILexicalBlock(scope: !547, file: !522, line: 65, column: 66)
!554 = !DILocalVariable(name: "val", scope: !555, file: !522, line: 65, type: !498, align: 64)
!555 = distinct !DILexicalBlock(scope: !547, file: !522, line: 65, column: 23)
!556 = !DILocalVariable(name: "err", scope: !557, file: !522, line: 68, type: !276, align: 32)
!557 = distinct !DILexicalBlock(scope: !541, file: !522, line: 68, column: 17)
!558 = !{!559}
!559 = !DITemplateTypeParameter(name: "impl Fn(&mut [MaybeUninit<u8>]) -> libc::ssize_t", type: !528)
!560 = !DILocation(line: 0, scope: !521, inlinedAt: !561)
!561 = distinct !DILocation(line: 97, column: 9, scope: !562, inlinedAt: !584)
!562 = distinct !DILexicalBlock(scope: !564, file: !563, line: 96, column: 9)
!563 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.3/src/backends/linux_android_with_fallback.rs", directory: "", checksumkind: CSK_MD5, checksum: "f5389189c05d242ee07e7b95e4b47e19")
!564 = distinct !DILexicalBlock(scope: !565, file: !563, line: 87, column: 5)
!565 = distinct !DILexicalBlock(scope: !566, file: !563, line: 86, column: 5)
!566 = distinct !DISubprogram(name: "fill_inner", linkageName: "_ZN9getrandom8backends27linux_android_with_fallback10fill_inner17h5c1767f0f03d271eE", scope: !530, file: !563, line: 79, type: !567, scopeLine: 79, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !569)
!567 = !DISubroutineType(types: !568)
!568 = !{!444, !498}
!569 = !{!570, !571, !572, !581, !583}
!570 = !DILocalVariable(name: "dest", arg: 1, scope: !566, file: !563, line: 79, type: !498)
!571 = !DILocalVariable(name: "raw_ptr", scope: !565, file: !563, line: 86, type: !537, align: 64)
!572 = !DILocalVariable(name: "fptr", scope: !564, file: !563, line: 87, type: !573, align: 64)
!573 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<core::ffi::c_void>", scope: !574, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !576, templateParams: !579, identifier: "e361c219328224d298c16bd9a36b63e5")
!574 = !DINamespace(name: "non_null", scope: !575)
!575 = !DINamespace(name: "ptr", scope: !71)
!576 = !{!577}
!577 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !573, file: !2, baseType: !578, size: 64, align: 64, flags: DIFlagPrivate)
!578 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const core::ffi::c_void", baseType: !69, size: 64, align: 64, dwarfAddressSpace: 0)
!579 = !{!580}
!580 = !DITemplateTypeParameter(name: "T", type: !69)
!581 = !DILocalVariable(name: "p", scope: !582, file: !563, line: 88, type: !573, align: 64)
!582 = distinct !DILexicalBlock(scope: !565, file: !563, line: 88, column: 9)
!583 = !DILocalVariable(name: "getrandom_fn", scope: !562, file: !563, line: 96, type: !534, align: 64)
!584 = distinct !DILocation(line: 99, column: 9, scope: !480, inlinedAt: !519)
!585 = !DILocalVariable(name: "self", arg: 1, scope: !586, file: !587, line: 1618, type: !592)
!586 = distinct !DISubprogram(name: "eq<core::ffi::c_void>", linkageName: "_ZN78_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17h6cdf64c3162a4c6fE", scope: !588, file: !587, line: 1618, type: !589, scopeLine: 1618, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !579, retainedNodes: !593)
!587 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/ptr/non_null.rs", directory: "", checksumkind: CSK_MD5, checksum: "3b3cd84fd90af2705fa6d8309deb8eb9")
!588 = !DINamespace(name: "{impl#14}", scope: !574)
!589 = !DISubroutineType(types: !590)
!590 = !{!591, !592, !592}
!591 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!592 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::ptr::non_null::NonNull<core::ffi::c_void>", baseType: !573, size: 64, align: 64, dwarfAddressSpace: 0)
!593 = !{!585, !594}
!594 = !DILocalVariable(name: "other", arg: 2, scope: !586, file: !587, line: 1618, type: !592)
!595 = !DILocation(line: 0, scope: !586, inlinedAt: !596)
!596 = distinct !DILocation(line: 92, column: 8, scope: !564, inlinedAt: !584)
!597 = !DILocation(line: 0, scope: !566, inlinedAt: !584)
!598 = !DILocalVariable(name: "order", scope: !599, file: !600, line: 1567, type: !75, align: 8)
!599 = distinct !DISubprogram(name: "load<core::ffi::c_void>", linkageName: "_ZN4core4sync6atomic18AtomicPtr$LT$T$GT$4load17h8d3b225ec6367179E", scope: !601, file: !600, line: 1567, type: !610, scopeLine: 1567, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !579, declaration: !613, retainedNodes: !614)
!600 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/sync/atomic.rs", directory: "", checksumkind: CSK_MD5, checksum: "857639279079d3d11ec5f1aef3f6a77a")
!601 = !DICompositeType(tag: DW_TAG_structure_type, name: "AtomicPtr<core::ffi::c_void>", scope: !76, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !602, templateParams: !579, identifier: "940cc0fa8d3d46c1b04884c608fe053e")
!602 = !{!603}
!603 = !DIDerivedType(tag: DW_TAG_member, name: "p", scope: !601, file: !2, baseType: !604, size: 64, align: 64, flags: DIFlagPrivate)
!604 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<*mut core::ffi::c_void>", scope: !605, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !606, templateParams: !608, identifier: "476077520d2320d29a467bbf0a2828d")
!605 = !DINamespace(name: "cell", scope: !71)
!606 = !{!607}
!607 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !604, file: !2, baseType: !537, size: 64, align: 64, flags: DIFlagPrivate)
!608 = !{!609}
!609 = !DITemplateTypeParameter(name: "T", type: !537)
!610 = !DISubroutineType(types: !611)
!611 = !{!537, !612, !75}
!612 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::sync::atomic::AtomicPtr<core::ffi::c_void>", baseType: !601, size: 64, align: 64, dwarfAddressSpace: 0)
!613 = !DISubprogram(name: "load<core::ffi::c_void>", linkageName: "_ZN4core4sync6atomic18AtomicPtr$LT$T$GT$4load17h8d3b225ec6367179E", scope: !601, file: !600, line: 1567, type: !610, scopeLine: 1567, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !579)
!614 = !{!615, !598}
!615 = !DILocalVariable(name: "self", arg: 1, scope: !599, file: !600, line: 1567, type: !612)
!616 = !DILocation(line: 0, scope: !599, inlinedAt: !617)
!617 = distinct !DILocation(line: 86, column: 32, scope: !566, inlinedAt: !584)
!618 = !DILocalVariable(name: "dst", arg: 1, scope: !619, file: !600, line: 3728, type: !622)
!619 = distinct !DISubprogram(name: "atomic_load<*mut core::ffi::c_void>", linkageName: "_ZN4core4sync6atomic11atomic_load17h8f683aafed77e7a6E", scope: !76, file: !600, line: 3728, type: !620, scopeLine: 3728, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !608, retainedNodes: !623)
!620 = !DISubroutineType(types: !621)
!621 = !{!537, !622, !75}
!622 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *mut core::ffi::c_void", baseType: !537, size: 64, align: 64, dwarfAddressSpace: 0)
!623 = !{!618, !624}
!624 = !DILocalVariable(name: "order", arg: 2, scope: !619, file: !600, line: 3728, type: !75)
!625 = !DILocation(line: 0, scope: !619, inlinedAt: !626)
!626 = distinct !DILocation(line: 1569, column: 18, scope: !599, inlinedAt: !617)
!627 = !DILocation(line: 3733, column: 24, scope: !619, inlinedAt: !626)
!628 = !{!629, !631, !633, !409, !295}
!629 = distinct !{!629, !630, !"_ZN9getrandom8backends27linux_android_with_fallback10fill_inner17h5c1767f0f03d271eE: %dest.0"}
!630 = distinct !{!630, !"_ZN9getrandom8backends27linux_android_with_fallback10fill_inner17h5c1767f0f03d271eE"}
!631 = distinct !{!631, !632, !"_ZN9getrandom4fill17h11f2509b4e4fb8bcE: %dest.0"}
!632 = distinct !{!632, !"_ZN9getrandom4fill17h11f2509b4e4fb8bcE"}
!633 = distinct !{!633, !634, !"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E: %dest.0"}
!634 = distinct !{!634, !"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E"}
!635 = !DILocation(line: 0, scope: !565, inlinedAt: !584)
!636 = !DILocalVariable(name: "ptr", arg: 1, scope: !637, file: !587, line: 255, type: !537)
!637 = distinct !DISubprogram(name: "new<core::ffi::c_void>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17h80db20d9e0f6d719E", scope: !573, file: !587, line: 255, type: !638, scopeLine: 255, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !579, declaration: !654, retainedNodes: !655)
!638 = !DISubroutineType(types: !639)
!639 = !{!640, !537}
!640 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<core::ptr::non_null::NonNull<core::ffi::c_void>>", scope: !641, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !642, templateParams: !23, identifier: "edfa4e906f75924a8426ccf6f0c3ffaa")
!641 = !DINamespace(name: "option", scope: !71)
!642 = !{!643}
!643 = !DICompositeType(tag: DW_TAG_variant_part, scope: !640, file: !2, size: 64, align: 64, elements: !644, templateParams: !23, identifier: "f04d05b9f0681cae91cf66a1e9e23fc", discriminator: !653)
!644 = !{!645, !649}
!645 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !643, file: !2, baseType: !646, size: 64, align: 64, extraData: i64 0)
!646 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !640, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !647, identifier: "768783b45c98f68f1879709417b28c6c")
!647 = !{!648}
!648 = !DITemplateTypeParameter(name: "T", type: !573)
!649 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !643, file: !2, baseType: !650, size: 64, align: 64)
!650 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !640, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !651, templateParams: !647, identifier: "bace7ac2bd6060d6fe615060b57f13f3")
!651 = !{!652}
!652 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !650, file: !2, baseType: !573, size: 64, align: 64, flags: DIFlagPublic)
!653 = !DIDerivedType(tag: DW_TAG_member, scope: !640, file: !2, baseType: !226, size: 64, align: 64, flags: DIFlagArtificial)
!654 = !DISubprogram(name: "new<core::ffi::c_void>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17h80db20d9e0f6d719E", scope: !573, file: !587, line: 255, type: !638, scopeLine: 255, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !579)
!655 = !{!636}
!656 = !DILocation(line: 0, scope: !637, inlinedAt: !657)
!657 = distinct !DILocation(line: 87, column: 22, scope: !565, inlinedAt: !584)
!658 = !DILocation(line: 256, column: 13, scope: !637, inlinedAt: !657)
!659 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!660 = !DILocation(line: 89, column: 17, scope: !565, inlinedAt: !584)
!661 = !DILocation(line: 0, scope: !564, inlinedAt: !584)
!662 = !DILocation(line: 1619, column: 9, scope: !586, inlinedAt: !596)
!663 = !DILocation(line: 92, column: 8, scope: !564, inlinedAt: !584)
!664 = !DILocalVariable(name: "getrandom_fn", scope: !665, file: !563, line: 96, type: !534, align: 64)
!665 = distinct !DISubprogram(name: "{closure#0}", linkageName: "_ZN9getrandom8backends27linux_android_with_fallback10fill_inner28_$u7b$$u7b$closure$u7d$$u7d$17h7c5d5879a1df2957E", scope: !529, file: !563, line: 97, type: !666, scopeLine: 97, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !669)
!666 = !DISubroutineType(types: !667)
!667 = !{!107, !668, !498}
!668 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&getrandom::backends::linux_android_with_fallback::fill_inner::{closure_env#0}", baseType: !528, size: 64, align: 64, dwarfAddressSpace: 0)
!669 = !{!670, !664}
!670 = !DILocalVariable(name: "buf", arg: 2, scope: !665, file: !563, line: 97, type: !498)
!671 = !DILocation(line: 0, scope: !665, inlinedAt: !672)
!672 = distinct !DILocation(line: 61, column: 19, scope: !521, inlinedAt: !561)
!673 = !DILocation(line: 98, column: 13, scope: !665, inlinedAt: !672)
!674 = !{!675, !409, !295}
!675 = distinct !{!675, !676, !"_ZN9getrandom8backends8use_file9util_libc14sys_fill_exact17ha95d0b037dbe5276E: argument 1"}
!676 = distinct !{!676, !"_ZN9getrandom8backends8use_file9util_libc14sys_fill_exact17ha95d0b037dbe5276E"}
!677 = !DILocation(line: 0, scope: !541, inlinedAt: !561)
!678 = !DILocation(line: 0, scope: !543, inlinedAt: !561)
!679 = !DILocation(line: 63, column: 20, scope: !541, inlinedAt: !561)
!680 = !DILocation(line: 62, column: 9, scope: !541, inlinedAt: !561)
!681 = !DILocation(line: 68, column: 27, scope: !541, inlinedAt: !561)
!682 = !DILocation(line: 0, scope: !557, inlinedAt: !561)
!683 = !DILocalVariable(name: "self", arg: 1, scope: !684, file: !685, line: 88, type: !276)
!684 = distinct !DISubprogram(name: "raw_os_error", linkageName: "_ZN9getrandom5error5Error12raw_os_error17hb243f18ee1719933E", scope: !276, file: !685, line: 88, type: !686, scopeLine: 88, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, declaration: !699, retainedNodes: !700)
!685 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.3/src/error.rs", directory: "", checksumkind: CSK_MD5, checksum: "771456b8ebc6726476a73dc08ba5a3eb")
!686 = !DISubroutineType(types: !687)
!687 = !{!688, !276}
!688 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<i32>", scope: !641, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !689, templateParams: !23, identifier: "e1655327d1ce207c347490cdc4308e3f")
!689 = !{!690}
!690 = !DICompositeType(tag: DW_TAG_variant_part, scope: !688, file: !2, size: 64, align: 32, elements: !691, templateParams: !23, identifier: "1995b4680135c1de232f0b7acd3948b", discriminator: !698)
!691 = !{!692, !694}
!692 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !690, file: !2, baseType: !693, size: 64, align: 32, extraData: i32 0)
!693 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !688, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !23, templateParams: !290, identifier: "c89efb4fc5b954572e4641396a82323e")
!694 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !690, file: !2, baseType: !695, size: 64, align: 32, extraData: i32 1)
!695 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !688, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !696, templateParams: !290, identifier: "55ec302645c3ac12d11d4eb48b9b59c")
!696 = !{!697}
!697 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !695, file: !2, baseType: !125, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!698 = !DIDerivedType(tag: DW_TAG_member, scope: !688, file: !2, baseType: !221, size: 32, align: 32, flags: DIFlagArtificial)
!699 = !DISubprogram(name: "raw_os_error", linkageName: "_ZN9getrandom5error5Error12raw_os_error17hb243f18ee1719933E", scope: !276, file: !685, line: 88, type: !686, scopeLine: 88, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!700 = !{!683, !701}
!701 = !DILocalVariable(name: "code", scope: !702, file: !685, line: 89, type: !125, align: 32)
!702 = distinct !DILexicalBlock(scope: !684, file: !685, line: 89, column: 9)
!703 = !DILocation(line: 0, scope: !684, inlinedAt: !704)
!704 = distinct !DILocation(line: 70, column: 24, scope: !557, inlinedAt: !561)
!705 = !DILocation(line: 0, scope: !702, inlinedAt: !704)
!706 = !DILocation(line: 115, column: 16, scope: !702, inlinedAt: !704)
!707 = !DILocalVariable(name: "self", arg: 1, scope: !708, file: !324, line: 2008, type: !729)
!708 = distinct !DISubprogram(name: "branch<&mut [core::mem::maybe_uninit::MaybeUninit<u8>], getrandom::error::Error>", linkageName: "_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h8ab92a5330928040E", scope: !709, file: !324, line: 2008, type: !710, scopeLine: 2008, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !737, retainedNodes: !744)
!709 = !DINamespace(name: "{impl#27}", scope: !303)
!710 = !DISubroutineType(types: !711)
!711 = !{!712, !729}
!712 = !DICompositeType(tag: DW_TAG_structure_type, name: "ControlFlow<core::result::Result<core::convert::Infallible, getrandom::error::Error>, &mut [core::mem::maybe_uninit::MaybeUninit<u8>]>", scope: !713, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !714, templateParams: !23, identifier: "b2448ea5010dba4ecc53fcf9c981a7b")
!713 = !DINamespace(name: "control_flow", scope: !161)
!714 = !{!715}
!715 = !DICompositeType(tag: DW_TAG_variant_part, scope: !712, file: !2, size: 128, align: 64, elements: !716, templateParams: !23, identifier: "ce6e60f227e4fe532bc3537ab4378980", discriminator: !728)
!716 = !{!717, !724}
!717 = !DIDerivedType(tag: DW_TAG_member, name: "Continue", scope: !715, file: !2, baseType: !718, size: 128, align: 64)
!718 = !DICompositeType(tag: DW_TAG_structure_type, name: "Continue", scope: !712, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !719, templateParams: !721, identifier: "4902db8277564bdf6d774e7863675545")
!719 = !{!720}
!720 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !718, file: !2, baseType: !498, size: 128, align: 64, flags: DIFlagPublic)
!721 = !{!722, !723}
!722 = !DITemplateTypeParameter(name: "B", type: !462)
!723 = !DITemplateTypeParameter(name: "C", type: !498)
!724 = !DIDerivedType(tag: DW_TAG_member, name: "Break", scope: !715, file: !2, baseType: !725, size: 128, align: 64, extraData: i64 0)
!725 = !DICompositeType(tag: DW_TAG_structure_type, name: "Break", scope: !712, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !726, templateParams: !721, identifier: "cd12e69630180526d09dcfb41a58e0a0")
!726 = !{!727}
!727 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !725, file: !2, baseType: !462, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
!728 = !DIDerivedType(tag: DW_TAG_member, scope: !712, file: !2, baseType: !226, size: 64, align: 64, flags: DIFlagArtificial)
!729 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<&mut [core::mem::maybe_uninit::MaybeUninit<u8>], getrandom::error::Error>", scope: !303, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !730, templateParams: !23, identifier: "54875cdb341398217ea8f6fe73d06de1")
!730 = !{!731}
!731 = !DICompositeType(tag: DW_TAG_variant_part, scope: !729, file: !2, size: 128, align: 64, elements: !732, templateParams: !23, identifier: "945030fa0dd35e98e22a85243473ef52", discriminator: !743)
!732 = !{!733, !739}
!733 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !731, file: !2, baseType: !734, size: 128, align: 64)
!734 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !729, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !735, templateParams: !737, identifier: "e679703d7c0366fc5a04357dd1afe476")
!735 = !{!736}
!736 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !734, file: !2, baseType: !498, size: 128, align: 64, flags: DIFlagPublic)
!737 = !{!738, !453}
!738 = !DITemplateTypeParameter(name: "T", type: !498)
!739 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !731, file: !2, baseType: !740, size: 128, align: 64, extraData: i64 0)
!740 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !729, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !741, templateParams: !737, identifier: "badcde712446d198b67290eb4b82be9")
!741 = !{!742}
!742 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !740, file: !2, baseType: !276, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
!743 = !DIDerivedType(tag: DW_TAG_member, scope: !729, file: !2, baseType: !226, size: 64, align: 64, flags: DIFlagArtificial)
!744 = !{!707, !745, !747}
!745 = !DILocalVariable(name: "v", scope: !746, file: !324, line: 2010, type: !498, align: 64)
!746 = distinct !DILexicalBlock(scope: !708, file: !324, line: 2010, column: 13)
!747 = !DILocalVariable(name: "e", scope: !748, file: !324, line: 2011, type: !276, align: 32)
!748 = distinct !DILexicalBlock(scope: !708, file: !324, line: 2011, column: 13)
!749 = !DILocation(line: 0, scope: !708, inlinedAt: !750)
!750 = distinct !DILocation(line: 65, column: 23, scope: !547, inlinedAt: !561)
!751 = !DILocation(line: 60, column: 12, scope: !521, inlinedAt: !561)
!752 = !DILocation(line: 0, scope: !547, inlinedAt: !561)
!753 = !DILocalVariable(name: "index", scope: !754, file: !755, line: 619, type: !772, align: 64)
!754 = distinct !DISubprogram(name: "get_mut<core::mem::maybe_uninit::MaybeUninit<u8>, core::ops::range::RangeFrom<usize>>", linkageName: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7get_mut17hf0e0507db7039061E", scope: !756, file: !755, line: 619, type: !758, scopeLine: 619, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !780, retainedNodes: !778)
!755 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/slice/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "c0b5edce80aed843a079f65948cd2d97")
!756 = !DINamespace(name: "{impl#0}", scope: !757)
!757 = !DINamespace(name: "slice", scope: !71)
!758 = !DISubroutineType(types: !759)
!759 = !{!760, !498, !772}
!760 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&mut [core::mem::maybe_uninit::MaybeUninit<u8>]>", scope: !641, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !761, templateParams: !23, identifier: "e02c7f6170e888924a200289ab5dd29")
!761 = !{!762}
!762 = !DICompositeType(tag: DW_TAG_variant_part, scope: !760, file: !2, size: 128, align: 64, elements: !763, templateParams: !23, identifier: "8c73d15be9eb78d62f2f97fc6ab9552c", discriminator: !771)
!763 = !{!764, !767}
!764 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !762, file: !2, baseType: !765, size: 128, align: 64, extraData: i64 0)
!765 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !760, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !766, identifier: "990e3359395a2a9de56121807cac1a01")
!766 = !{!738}
!767 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !762, file: !2, baseType: !768, size: 128, align: 64)
!768 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !760, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !769, templateParams: !766, identifier: "38a7bfc194039282d403d0ca63c79e85")
!769 = !{!770}
!770 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !768, file: !2, baseType: !498, size: 128, align: 64, flags: DIFlagPublic)
!771 = !DIDerivedType(tag: DW_TAG_member, scope: !760, file: !2, baseType: !226, size: 64, align: 64, flags: DIFlagArtificial)
!772 = !DICompositeType(tag: DW_TAG_structure_type, name: "RangeFrom<usize>", scope: !773, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !774, templateParams: !776, identifier: "20dd8f89a39ed9c4ed1d74f21e2590e")
!773 = !DINamespace(name: "range", scope: !161)
!774 = !{!775}
!775 = !DIDerivedType(tag: DW_TAG_member, name: "start", scope: !772, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPublic)
!776 = !{!777}
!777 = !DITemplateTypeParameter(name: "Idx", type: !9)
!778 = !{!779, !753}
!779 = !DILocalVariable(name: "self", arg: 1, scope: !754, file: !755, line: 619, type: !498)
!780 = !{!781, !782}
!781 = !DITemplateTypeParameter(name: "T", type: !502)
!782 = !DITemplateTypeParameter(name: "I", type: !772)
!783 = !DILocation(line: 0, scope: !754, inlinedAt: !784)
!784 = distinct !DILocation(line: 65, column: 27, scope: !547, inlinedAt: !561)
!785 = !DILocalVariable(name: "self", scope: !786, file: !787, line: 542, type: !772, align: 64)
!786 = distinct !DISubprogram(name: "get_mut<core::mem::maybe_uninit::MaybeUninit<u8>>", linkageName: "_ZN110_$LT$core..ops..range..RangeFrom$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$7get_mut17hc6a29d167a67c21cE", scope: !788, file: !787, line: 542, type: !790, scopeLine: 542, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !794, retainedNodes: !792)
!787 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/slice/index.rs", directory: "", checksumkind: CSK_MD5, checksum: "baa2a238d3c3ce81e755e760737a6886")
!788 = !DINamespace(name: "{impl#7}", scope: !789)
!789 = !DINamespace(name: "index", scope: !757)
!790 = !DISubroutineType(types: !791)
!791 = !{!760, !772, !498}
!792 = !{!785, !793}
!793 = !DILocalVariable(name: "slice", arg: 2, scope: !786, file: !787, line: 542, type: !498)
!794 = !{!781}
!795 = !DILocation(line: 0, scope: !786, inlinedAt: !796)
!796 = distinct !DILocation(line: 623, column: 15, scope: !754, inlinedAt: !784)
!797 = !DILocalVariable(name: "self", scope: !798, file: !787, line: 377, type: !802, align: 64)
!798 = distinct !DISubprogram(name: "get_mut<core::mem::maybe_uninit::MaybeUninit<u8>>", linkageName: "_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$7get_mut17he480c3cf57ed840fE", scope: !799, file: !787, line: 377, type: !800, scopeLine: 377, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !794, retainedNodes: !806)
!799 = !DINamespace(name: "{impl#4}", scope: !789)
!800 = !DISubroutineType(types: !801)
!801 = !{!760, !802, !498}
!802 = !DICompositeType(tag: DW_TAG_structure_type, name: "Range<usize>", scope: !773, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !803, templateParams: !776, identifier: "9a3f37a980c09302627e6f483c3e1ec2")
!803 = !{!804, !805}
!804 = !DIDerivedType(tag: DW_TAG_member, name: "start", scope: !802, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPublic)
!805 = !DIDerivedType(tag: DW_TAG_member, name: "end", scope: !802, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!806 = !{!797, !797, !807, !808}
!807 = !DILocalVariable(name: "slice", arg: 2, scope: !798, file: !787, line: 377, type: !498)
!808 = !DILocalVariable(name: "new_len", scope: !798, file: !787, line: 378, type: !9, align: 64)
!809 = !DILocation(line: 0, scope: !798, inlinedAt: !810)
!810 = distinct !DILocation(line: 543, column: 35, scope: !786, inlinedAt: !796)
!811 = !DILocalVariable(name: "rhs", arg: 2, scope: !812, file: !813, line: 698, type: !9)
!812 = distinct !DISubprogram(name: "checked_sub", linkageName: "_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_sub17hb1cb57f58d49264dE", scope: !814, file: !813, line: 698, type: !815, scopeLine: 698, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !830)
!813 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/num/uint_macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "10cb569da042ef801acea143a0c73ffd")
!814 = !DINamespace(name: "{impl#11}", scope: !283)
!815 = !DISubroutineType(types: !816)
!816 = !{!817, !9, !9}
!817 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<usize>", scope: !641, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !818, templateParams: !23, identifier: "f68a2b7c3ee2cdf2fd7ff27937bd2af")
!818 = !{!819}
!819 = !DICompositeType(tag: DW_TAG_variant_part, scope: !817, file: !2, size: 128, align: 64, elements: !820, templateParams: !23, identifier: "dfb82d512c7342523dd52dc2924145e3", discriminator: !829)
!820 = !{!821, !825}
!821 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !819, file: !2, baseType: !822, size: 128, align: 64, extraData: i64 0)
!822 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !817, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !823, identifier: "4a42b54124b56d6195fb546eaea984ac")
!823 = !{!824}
!824 = !DITemplateTypeParameter(name: "T", type: !9)
!825 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !819, file: !2, baseType: !826, size: 128, align: 64, extraData: i64 1)
!826 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !817, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !827, templateParams: !823, identifier: "592f525925657159c4d0d34ea72ff0fe")
!827 = !{!828}
!828 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !826, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!829 = !DIDerivedType(tag: DW_TAG_member, scope: !817, file: !2, baseType: !226, size: 64, align: 64, flags: DIFlagArtificial)
!830 = !{!831, !811}
!831 = !DILocalVariable(name: "self", arg: 1, scope: !812, file: !813, line: 698, type: !9)
!832 = !DILocation(line: 0, scope: !812, inlinedAt: !833)
!833 = distinct !DILocation(line: 378, column: 32, scope: !798, inlinedAt: !810)
!834 = !DILocation(line: 704, column: 16, scope: !812, inlinedAt: !833)
!835 = !DILocation(line: 2009, column: 9, scope: !708, inlinedAt: !750)
!836 = !DILocation(line: 93, column: 9, scope: !564, inlinedAt: !584)
!837 = !DILocalVariable(name: "self", arg: 1, scope: !838, file: !324, line: 2008, type: !302)
!838 = distinct !DISubprogram(name: "branch<(), rand_core::os::OsError>", linkageName: "_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h1db3b00c1fb82457E", scope: !709, file: !324, line: 2008, type: !839, scopeLine: 2008, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !311, retainedNodes: !857)
!839 = !DISubroutineType(types: !840)
!840 = !{!841, !302}
!841 = !DICompositeType(tag: DW_TAG_structure_type, name: "ControlFlow<core::result::Result<core::convert::Infallible, rand_core::os::OsError>, ()>", scope: !713, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !842, templateParams: !23, identifier: "90d3b6de0c63b8e855faeb3aee55390")
!842 = !{!843}
!843 = !DICompositeType(tag: DW_TAG_variant_part, scope: !841, file: !2, size: 32, align: 32, elements: !844, templateParams: !23, identifier: "e99c02cf6c6f1bb799a23179814e0274", discriminator: !856)
!844 = !{!845, !852}
!845 = !DIDerivedType(tag: DW_TAG_member, name: "Continue", scope: !843, file: !2, baseType: !846, size: 32, align: 32, extraData: i32 0)
!846 = !DICompositeType(tag: DW_TAG_structure_type, name: "Continue", scope: !841, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !847, templateParams: !849, identifier: "d93396031251b881ccd4e21207912858")
!847 = !{!848}
!848 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !846, file: !2, baseType: !7, align: 8, flags: DIFlagPublic)
!849 = !{!850, !851}
!850 = !DITemplateTypeParameter(name: "B", type: !381)
!851 = !DITemplateTypeParameter(name: "C", type: !7)
!852 = !DIDerivedType(tag: DW_TAG_member, name: "Break", scope: !843, file: !2, baseType: !853, size: 32, align: 32)
!853 = !DICompositeType(tag: DW_TAG_structure_type, name: "Break", scope: !841, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !854, templateParams: !849, identifier: "bad8b7aa150ec32dc6f5526eb696b3c7")
!854 = !{!855}
!855 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !853, file: !2, baseType: !381, size: 32, align: 32, flags: DIFlagPublic)
!856 = !DIDerivedType(tag: DW_TAG_member, scope: !841, file: !2, baseType: !221, size: 32, align: 32, flags: DIFlagArtificial)
!857 = !{!837, !858, !860}
!858 = !DILocalVariable(name: "v", scope: !859, file: !324, line: 2010, type: !7, align: 8)
!859 = distinct !DILexicalBlock(scope: !838, file: !324, line: 2010, column: 13)
!860 = !DILocalVariable(name: "e", scope: !861, file: !324, line: 2011, type: !273, align: 32)
!861 = distinct !DILexicalBlock(scope: !838, file: !324, line: 2011, column: 13)
!862 = !DILocation(line: 0, scope: !838, inlinedAt: !863)
!863 = distinct !DILocation(line: 532, column: 9, scope: !375, inlinedAt: !405)
!864 = !DILocation(line: 2009, column: 15, scope: !838, inlinedAt: !863)
!865 = !DILocation(line: 2009, column: 9, scope: !838, inlinedAt: !863)
!866 = !DILocation(line: 534, column: 5, scope: !367, inlinedAt: !405)
!867 = !DILocation(line: 0, scope: !323, inlinedAt: !363)
!868 = !DILocalVariable(name: "self__inner", scope: !869, file: !200, line: 208, type: !206, align: 128)
!869 = distinct !DISubprogram(name: "{closure#0}<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed28_$u7b$$u7b$closure$u7d$$u7d$17h0ef576ded73a595eE", scope: !343, file: !200, line: 209, type: !870, scopeLine: 209, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !250, retainedNodes: !872)
!870 = !DISubroutineType(types: !871)
!871 = !{null, !342, !206}
!872 = !{!873, !868, !874, !875}
!873 = !DILocalVariable(name: "result", arg: 2, scope: !869, file: !200, line: 209, type: !206)
!874 = !DILocalVariable(name: "self__threshold", scope: !869, file: !200, line: 208, type: !241, align: 64)
!875 = !DILocalVariable(name: "self__bytes_until_reseed", scope: !869, file: !200, line: 208, type: !241, align: 64)
!876 = !DILocation(line: 0, scope: !869, inlinedAt: !877)
!877 = distinct !DILocation(line: 773, column: 25, scope: !359, inlinedAt: !363)
!878 = !DILocation(line: 226, column: 35, scope: !270)
!879 = !DILocation(line: 776, column: 5, scope: !323, inlinedAt: !363)
!880 = !DILocation(line: 533, column: 28, scope: !375, inlinedAt: !405)
!881 = !DILocalVariable(name: "seed", arg: 1, scope: !882, file: !883, line: 99, type: !376)
!882 = distinct !DISubprogram(name: "from_seed", linkageName: "_ZN76_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..SeedableRng$GT$9from_seed17h908954789d451da6E", scope: !884, file: !883, line: 99, type: !885, scopeLine: 99, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !887)
!883 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_chacha-0.9.0/src/chacha.rs", directory: "", checksumkind: CSK_MD5, checksum: "7f4e0005afead6890469401ecfa5f424")
!884 = !DINamespace(name: "{impl#24}", scope: !207)
!885 = !DISubroutineType(types: !886)
!886 = !{!206, !376}
!887 = !{!881}
!888 = !DILocation(line: 99, column: 26, scope: !882, inlinedAt: !889)
!889 = distinct !DILocation(line: 533, column: 12, scope: !375, inlinedAt: !405)
!890 = !DILocalVariable(name: "key", arg: 1, scope: !891, file: !892, line: 74, type: !895)
!891 = distinct !DISubprogram(name: "new", linkageName: "_ZN11rand_chacha4guts6ChaCha3new17hf3fc1524424c46feE", scope: !211, file: !892, line: 74, type: !893, scopeLine: 74, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, declaration: !900, retainedNodes: !901)
!892 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_chacha-0.9.0/src/guts.rs", directory: "", checksumkind: CSK_MD5, checksum: "d8991d5076408722f1c6dc41e4372770")
!893 = !DISubroutineType(types: !894)
!894 = !{!211, !895, !896}
!895 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[u8; 32]", baseType: !376, size: 64, align: 64, dwarfAddressSpace: 0)
!896 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[u8]", file: !2, size: 128, align: 64, elements: !897, templateParams: !23, identifier: "31681e0c10b314f1f33e38b2779acbb4")
!897 = !{!898, !899}
!898 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !896, file: !2, baseType: !433, size: 64, align: 64)
!899 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !896, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!900 = !DISubprogram(name: "new", linkageName: "_ZN11rand_chacha4guts6ChaCha3new17hf3fc1524424c46feE", scope: !211, file: !892, line: 74, type: !893, scopeLine: 74, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!901 = !{!890, !902}
!902 = !DILocalVariable(name: "nonce", arg: 2, scope: !891, file: !892, line: 74, type: !896)
!903 = !DILocation(line: 0, scope: !891, inlinedAt: !904)
!904 = distinct !DILocation(line: 101, column: 28, scope: !882, inlinedAt: !889)
!905 = !DILocation(line: 533, column: 9, scope: !375, inlinedAt: !405)
!906 = !DILocation(line: 75, column: 9, scope: !891, inlinedAt: !904)
!907 = !DILocation(line: 533, column: 32, scope: !375, inlinedAt: !405)
!908 = !DILocation(line: 209, column: 49, scope: !299, inlinedAt: !321)
!909 = !DILocation(line: 209, column: 50, scope: !869, inlinedAt: !877)
!910 = !DILocation(line: 210, column: 39, scope: !869, inlinedAt: !877)
!911 = !{!912, !914}
!912 = distinct !{!912, !913, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed28_$u7b$$u7b$closure$u7d$$u7d$17h0ef576ded73a595eE: %_1"}
!913 = distinct !{!913, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed28_$u7b$$u7b$closure$u7d$$u7d$17h0ef576ded73a595eE"}
!914 = distinct !{!914, !913, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed28_$u7b$$u7b$closure$u7d$$u7d$17h0ef576ded73a595eE: %result"}
!915 = !DILocation(line: 211, column: 13, scope: !869, inlinedAt: !877)
!916 = !DILocation(line: 773, column: 30, scope: !323, inlinedAt: !363)
!917 = !DILocation(line: 212, column: 10, scope: !299, inlinedAt: !321)
!918 = !DILocation(line: 226, column: 9, scope: !270)
!919 = !DILocalVariable(name: "self", arg: 1, scope: !920, file: !883, line: 90, type: !347)
!920 = distinct !DISubprogram(name: "generate", linkageName: "_ZN84_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17hcc73ae4b352d57fcE", scope: !921, file: !883, line: 90, type: !922, scopeLine: 90, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !924)
!921 = !DINamespace(name: "{impl#23}", scope: !207)
!922 = !DISubroutineType(types: !923)
!923 = !{null, !347, !256}
!924 = !{!919, !925}
!925 = !DILocalVariable(name: "r", arg: 2, scope: !920, file: !883, line: 90, type: !256)
!926 = !DILocation(line: 0, scope: !920, inlinedAt: !927)
!927 = distinct !DILocation(line: 227, column: 9, scope: !270)
!928 = !DILocalVariable(name: "drounds", scope: !929, file: !892, line: 80, type: !221, align: 32)
!929 = distinct !DISubprogram(name: "refill4", linkageName: "_ZN11rand_chacha4guts6ChaCha7refill417h8e27ff2a18dab7adE", scope: !211, file: !892, line: 80, type: !930, scopeLine: 80, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, declaration: !934, retainedNodes: !935)
!930 = !DISubroutineType(types: !931)
!931 = !{null, !932, !221, !933}
!932 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand_chacha::guts::ChaCha", baseType: !211, size: 64, align: 64, dwarfAddressSpace: 0)
!933 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut [u32; 64]", baseType: !260, size: 64, align: 64, dwarfAddressSpace: 0)
!934 = !DISubprogram(name: "refill4", linkageName: "_ZN11rand_chacha4guts6ChaCha7refill417h8e27ff2a18dab7adE", scope: !211, file: !892, line: 80, type: !930, scopeLine: 80, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!935 = !{!936, !928, !937}
!936 = !DILocalVariable(name: "self", arg: 1, scope: !929, file: !892, line: 80, type: !932)
!937 = !DILocalVariable(name: "out", arg: 3, scope: !929, file: !892, line: 80, type: !933)
!938 = !DILocation(line: 0, scope: !929, inlinedAt: !939)
!939 = distinct !DILocation(line: 91, column: 28, scope: !920, inlinedAt: !927)
!940 = !DILocation(line: 81, column: 9, scope: !929, inlinedAt: !939)
!941 = !DILocation(line: 228, column: 6, scope: !199)
!942 = distinct !DISubprogram(name: "speak", linkageName: "_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17h95a71617b8cd4b02E", scope: !944, file: !943, line: 10, type: !945, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !948)
!943 = !DIFile(filename: "src/main.rs", directory: "/home/np/hack/verifopt/dp-ex", checksumkind: CSK_MD5, checksum: "ead3cb60495c9b0a2e5a76f4f58f39a0")
!944 = !DINamespace(name: "{impl#0}", scope: !33)
!945 = !DISubroutineType(types: !946)
!946 = !{null, !947}
!947 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&dp_ex::Dog", baseType: !51, size: 64, align: 64, dwarfAddressSpace: 0)
!948 = !{!949}
!949 = !DILocalVariable(name: "self", arg: 1, scope: !942, file: !943, line: 10, type: !947)
!950 = !DILocation(line: 0, scope: !942)
!951 = !DILocalVariable(name: "pieces", scope: !952, file: !953, line: 600, type: !1111, align: 64)
!952 = distinct !DISubprogram(name: "new_const<1>", linkageName: "_ZN4core3fmt9Arguments9new_const17h4071c16c1b3d8f01E", scope: !954, file: !953, line: 600, type: !1109, scopeLine: 600, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, declaration: !1113, retainedNodes: !1114)
!953 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/fmt/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "55150785045f6b77da421daee4eba248")
!954 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !86, file: !2, size: 384, align: 64, flags: DIFlagPublic, elements: !955, templateParams: !23, identifier: "24eb0fb5d76b97e2d20682aa7b755ab")
!955 = !{!956, !966, !1010}
!956 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !954, file: !2, baseType: !957, size: 128, align: 64, flags: DIFlagPrivate)
!957 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !2, size: 128, align: 64, elements: !958, templateParams: !23, identifier: "4e66b00a376d6af5b8765440fb2839f")
!958 = !{!959, !965}
!959 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !957, file: !2, baseType: !960, size: 64, align: 64)
!960 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !961, size: 64, align: 64, dwarfAddressSpace: 0)
!961 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 128, align: 64, elements: !962, templateParams: !23, identifier: "9277eecd40495f85161460476aacc992")
!962 = !{!963, !964}
!963 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !961, file: !2, baseType: !433, size: 64, align: 64)
!964 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !961, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!965 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !957, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!966 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !954, file: !2, baseType: !967, size: 128, align: 64, offset: 256, flags: DIFlagPrivate)
!967 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::Placeholder]>", scope: !641, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !968, templateParams: !23, identifier: "2d1751345db4736156c9498f57ce083a")
!968 = !{!969}
!969 = !DICompositeType(tag: DW_TAG_variant_part, scope: !967, file: !2, size: 128, align: 64, elements: !970, templateParams: !23, identifier: "d11d7583563b78f79b7a9c1fddef5d53", discriminator: !1009)
!970 = !{!971, !1005}
!971 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !969, file: !2, baseType: !972, size: 128, align: 64, extraData: i64 0)
!972 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !967, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !973, identifier: "5dfa75cac8702857f2ccfde7135f7333")
!973 = !{!974}
!974 = !DITemplateTypeParameter(name: "T", type: !975)
!975 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Placeholder]", file: !2, size: 128, align: 64, elements: !976, templateParams: !23, identifier: "359448f2614cfcc0d0c1c56c841cf880")
!976 = !{!977, !1004}
!977 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !975, file: !2, baseType: !978, size: 64, align: 64)
!978 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !979, size: 64, align: 64, dwarfAddressSpace: 0)
!979 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !85, file: !2, size: 448, align: 64, flags: DIFlagPublic, elements: !980, templateParams: !23, identifier: "37d51e956e565f2360e5336db010fb94")
!980 = !{!981, !982, !984, !985, !986, !1003}
!981 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !979, file: !2, baseType: !9, size: 64, align: 64, offset: 256, flags: DIFlagPublic)
!982 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !979, file: !2, baseType: !983, size: 32, align: 32, offset: 352, flags: DIFlagPublic)
!983 = !DIBasicType(name: "char", size: 32, encoding: DW_ATE_UTF)
!984 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !979, file: !2, baseType: !84, size: 8, align: 8, offset: 384, flags: DIFlagPublic)
!985 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !979, file: !2, baseType: !221, size: 32, align: 32, offset: 320, flags: DIFlagPublic)
!986 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !979, file: !2, baseType: !987, size: 128, align: 64, flags: DIFlagPublic)
!987 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !85, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !988, templateParams: !23, identifier: "33fe681e4a4ed9828ac2f6a98ef48c92")
!988 = !{!989}
!989 = !DICompositeType(tag: DW_TAG_variant_part, scope: !987, file: !2, size: 128, align: 64, elements: !990, templateParams: !23, identifier: "17f74d77c68ee09fd7e4b3771d0838f4", discriminator: !1002)
!990 = !{!991, !996, !1000}
!991 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !989, file: !2, baseType: !992, size: 128, align: 64, extraData: i16 0)
!992 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !987, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !993, templateParams: !23, identifier: "b598892e00d12602a714369da84c5e40")
!993 = !{!994}
!994 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !992, file: !2, baseType: !995, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!995 = !DIBasicType(name: "u16", size: 16, encoding: DW_ATE_unsigned)
!996 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !989, file: !2, baseType: !997, size: 128, align: 64, extraData: i16 1)
!997 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !987, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !998, templateParams: !23, identifier: "5e02220b8c537696d6d32410ef9ad7cb")
!998 = !{!999}
!999 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !997, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!1000 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !989, file: !2, baseType: !1001, size: 128, align: 64, extraData: i16 2)
!1001 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !987, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, identifier: "d2975225efab438f6e0bbcf92cacbea2")
!1002 = !DIDerivedType(tag: DW_TAG_member, scope: !987, file: !2, baseType: !995, size: 16, align: 16, flags: DIFlagArtificial)
!1003 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !979, file: !2, baseType: !987, size: 128, align: 64, offset: 128, flags: DIFlagPublic)
!1004 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !975, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!1005 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !969, file: !2, baseType: !1006, size: 128, align: 64)
!1006 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !967, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1007, templateParams: !973, identifier: "c738961c11197bf175c161a53fc8f300")
!1007 = !{!1008}
!1008 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1006, file: !2, baseType: !975, size: 128, align: 64, flags: DIFlagPublic)
!1009 = !DIDerivedType(tag: DW_TAG_member, scope: !967, file: !2, baseType: !226, size: 64, align: 64, flags: DIFlagArtificial)
!1010 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !954, file: !2, baseType: !1011, size: 128, align: 64, offset: 128, flags: DIFlagPrivate)
!1011 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Argument]", file: !2, size: 128, align: 64, elements: !1012, templateParams: !23, identifier: "7018af262b519fbf626cdbb35a367f12")
!1012 = !{!1013, !1108}
!1013 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !1011, file: !2, baseType: !1014, size: 64, align: 64)
!1014 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !1015, size: 64, align: 64, dwarfAddressSpace: 0)
!1015 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !85, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1016, templateParams: !23, identifier: "25620a2bf73f444f61cf887636e73c9c")
!1016 = !{!1017}
!1017 = !DIDerivedType(tag: DW_TAG_member, name: "ty", scope: !1015, file: !2, baseType: !1018, size: 128, align: 64, flags: DIFlagPrivate)
!1018 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentType", scope: !85, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !1019, templateParams: !23, identifier: "6b7e829d90a5625593a17c12bac24a22")
!1019 = !{!1020}
!1020 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1018, file: !2, size: 128, align: 64, elements: !1021, templateParams: !23, identifier: "dbe2fa85d59ebf2abe6f5e2bf512dd10", discriminator: !1107)
!1021 = !{!1022, !1103}
!1022 = !DIDerivedType(tag: DW_TAG_member, name: "Placeholder", scope: !1020, file: !2, baseType: !1023, size: 128, align: 64)
!1023 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !1018, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !1024, templateParams: !23, identifier: "9360cd60a524b377598c38d09f2ed0e")
!1024 = !{!1025, !1029, !1097}
!1025 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1023, file: !2, baseType: !1026, size: 64, align: 64, flags: DIFlagPrivate)
!1026 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<()>", scope: !574, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1027, templateParams: !115, identifier: "3e0ac18be859eeb0b4ae62bb9b847f81")
!1027 = !{!1028}
!1028 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !1026, file: !2, baseType: !6, size: 64, align: 64, flags: DIFlagPrivate)
!1029 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !1023, file: !2, baseType: !1030, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!1030 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "unsafe fn(core::ptr::non_null::NonNull<()>, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !1031, size: 64, align: 64, dwarfAddressSpace: 0)
!1031 = !DISubroutineType(types: !1032)
!1032 = !{!1033, !1026, !1049}
!1033 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), core::fmt::Error>", scope: !303, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !1034, templateParams: !23, identifier: "6816ab7da155524a7f7fa5256de3cf82")
!1034 = !{!1035}
!1035 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1033, file: !2, size: 8, align: 8, elements: !1036, templateParams: !23, identifier: "e8d0d2b8f2420234621de41ea2595fd1", discriminator: !1048)
!1036 = !{!1037, !1044}
!1037 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1035, file: !2, baseType: !1038, size: 8, align: 8, extraData: i8 0)
!1038 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1033, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !1039, templateParams: !1041, identifier: "e5f3207fdcf62eacaa53939734a74154")
!1039 = !{!1040}
!1040 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1038, file: !2, baseType: !7, align: 8, offset: 8, flags: DIFlagPublic)
!1041 = !{!116, !1042}
!1042 = !DITemplateTypeParameter(name: "E", type: !1043)
!1043 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !86, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "bd06eb806965e0e83cbfa92c8110c24d")
!1044 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1035, file: !2, baseType: !1045, size: 8, align: 8, extraData: i8 1)
!1045 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1033, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !1046, templateParams: !1041, identifier: "851959764360495ed30d5fff18ec423")
!1046 = !{!1047}
!1047 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1045, file: !2, baseType: !1043, align: 8, offset: 8, flags: DIFlagPublic)
!1048 = !DIDerivedType(tag: DW_TAG_member, scope: !1033, file: !2, baseType: !65, size: 8, align: 8, flags: DIFlagArtificial)
!1049 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !1050, size: 64, align: 64, dwarfAddressSpace: 0)
!1050 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !86, file: !2, size: 320, align: 64, flags: DIFlagPublic, elements: !1051, templateParams: !23, identifier: "3156aacd96f386f053e3f612f97c88")
!1051 = !{!1052, !1086}
!1052 = !DIDerivedType(tag: DW_TAG_member, name: "options", scope: !1050, file: !2, baseType: !1053, size: 160, align: 32, offset: 128, flags: DIFlagPrivate)
!1053 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormattingOptions", scope: !86, file: !2, size: 160, align: 32, flags: DIFlagPublic, elements: !1054, templateParams: !23, identifier: "fc1aad6aa3841ba2385d309ea9faf97f")
!1054 = !{!1055, !1056, !1057, !1071, !1085}
!1055 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !1053, file: !2, baseType: !221, size: 32, align: 32, offset: 96, flags: DIFlagPrivate)
!1056 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !1053, file: !2, baseType: !983, size: 32, align: 32, flags: DIFlagPrivate)
!1057 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !1053, file: !2, baseType: !1058, size: 8, align: 8, offset: 128, flags: DIFlagPrivate)
!1058 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<core::fmt::Alignment>", scope: !641, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !1059, templateParams: !23, identifier: "4db9caa4211fb655a1f558f9da6744dc")
!1059 = !{!1060}
!1060 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1058, file: !2, size: 8, align: 8, elements: !1061, templateParams: !23, identifier: "c9d5472e7f251313105cce4473782d1f", discriminator: !1070)
!1061 = !{!1062, !1066}
!1062 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !1060, file: !2, baseType: !1063, size: 8, align: 8, extraData: i8 3)
!1063 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !1058, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !23, templateParams: !1064, identifier: "5c74e537c166370284bc9075df990f51")
!1064 = !{!1065}
!1065 = !DITemplateTypeParameter(name: "T", type: !92)
!1066 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !1060, file: !2, baseType: !1067, size: 8, align: 8)
!1067 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !1058, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !1068, templateParams: !1064, identifier: "8063e44fd517a1d3ea1079db8136250")
!1068 = !{!1069}
!1069 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1067, file: !2, baseType: !92, size: 8, align: 8, flags: DIFlagPublic)
!1070 = !DIDerivedType(tag: DW_TAG_member, scope: !1058, file: !2, baseType: !65, size: 8, align: 8, flags: DIFlagArtificial)
!1071 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !1053, file: !2, baseType: !1072, size: 32, align: 16, offset: 32, flags: DIFlagPrivate)
!1072 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<u16>", scope: !641, file: !2, size: 32, align: 16, flags: DIFlagPublic, elements: !1073, templateParams: !23, identifier: "320c093e14f19e7dd75720678f877ce3")
!1073 = !{!1074}
!1074 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1072, file: !2, size: 32, align: 16, elements: !1075, templateParams: !23, identifier: "91b32d8e3c156f713174cb354a398aae", discriminator: !1084)
!1075 = !{!1076, !1080}
!1076 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !1074, file: !2, baseType: !1077, size: 32, align: 16, extraData: i16 0)
!1077 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !1072, file: !2, size: 32, align: 16, flags: DIFlagPublic, elements: !23, templateParams: !1078, identifier: "97265f141f100e86e7f3d477667dd131")
!1078 = !{!1079}
!1079 = !DITemplateTypeParameter(name: "T", type: !995)
!1080 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !1074, file: !2, baseType: !1081, size: 32, align: 16, extraData: i16 1)
!1081 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !1072, file: !2, size: 32, align: 16, flags: DIFlagPublic, elements: !1082, templateParams: !1078, identifier: "57de06c6dcc5971442200f74e602309c")
!1082 = !{!1083}
!1083 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1081, file: !2, baseType: !995, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!1084 = !DIDerivedType(tag: DW_TAG_member, scope: !1072, file: !2, baseType: !995, size: 16, align: 16, flags: DIFlagArtificial)
!1085 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !1053, file: !2, baseType: !1072, size: 32, align: 16, offset: 64, flags: DIFlagPrivate)
!1086 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !1050, file: !2, baseType: !1087, size: 128, align: 64, flags: DIFlagPrivate)
!1087 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !2, size: 128, align: 64, elements: !1088, templateParams: !23, identifier: "61781eb331eb2173128184c9b2bf02dc")
!1088 = !{!1089, !1092}
!1089 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !1087, file: !2, baseType: !1090, size: 64, align: 64)
!1090 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !1091, size: 64, align: 64, dwarfAddressSpace: 0)
!1091 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !2, align: 8, elements: !23, identifier: "285f69df1e9048ce486c1c48ca17de85")
!1092 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !1087, file: !2, baseType: !1093, size: 64, align: 64, offset: 64)
!1093 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 6]", baseType: !1094, size: 64, align: 64, dwarfAddressSpace: 0)
!1094 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 384, align: 64, elements: !1095)
!1095 = !{!1096}
!1096 = !DISubrange(count: 6, lowerBound: 0)
!1097 = !DIDerivedType(tag: DW_TAG_member, name: "_lifetime", scope: !1023, file: !2, baseType: !1098, align: 8, offset: 128, flags: DIFlagPrivate)
!1098 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&()>", scope: !1099, file: !2, align: 8, flags: DIFlagPublic, elements: !23, templateParams: !1100, identifier: "114a5780a36016c559f40910cc5f9248")
!1099 = !DINamespace(name: "marker", scope: !71)
!1100 = !{!1101}
!1101 = !DITemplateTypeParameter(name: "T", type: !1102)
!1102 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&()", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 0)
!1103 = !DIDerivedType(tag: DW_TAG_member, name: "Count", scope: !1020, file: !2, baseType: !1104, size: 128, align: 64, extraData: i64 0)
!1104 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !1018, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !1105, templateParams: !23, identifier: "54f8e57d2009f180917aae6b0e82c575")
!1105 = !{!1106}
!1106 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1104, file: !2, baseType: !995, size: 16, align: 16, offset: 64, flags: DIFlagPrivate)
!1107 = !DIDerivedType(tag: DW_TAG_member, scope: !1018, file: !2, baseType: !226, size: 64, align: 64, flags: DIFlagArtificial)
!1108 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !1011, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!1109 = !DISubroutineType(types: !1110)
!1110 = !{!954, !1111}
!1111 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[&str; 1]", baseType: !1112, size: 64, align: 64, dwarfAddressSpace: 0)
!1112 = !DICompositeType(tag: DW_TAG_array_type, baseType: !961, size: 128, align: 64, elements: !232)
!1113 = !DISubprogram(name: "new_const<1>", linkageName: "_ZN4core3fmt9Arguments9new_const17h4071c16c1b3d8f01E", scope: !954, file: !953, line: 600, type: !1109, scopeLine: 600, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!1114 = !{!951}
!1115 = !DILocation(line: 0, scope: !952, inlinedAt: !1116)
!1116 = !DILocation(line: 11, column: 9, scope: !942)
!1117 = !DILocation(line: 602, column: 9, scope: !952, inlinedAt: !1116)
!1118 = !DILocation(line: 12, column: 6, scope: !942)
!1119 = distinct !DISubprogram(name: "speak", linkageName: "_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17hfb6e878b47c18265E", scope: !1120, file: !943, line: 18, type: !1121, scopeLine: 18, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !1124)
!1120 = !DINamespace(name: "{impl#1}", scope: !33)
!1121 = !DISubroutineType(types: !1122)
!1122 = !{null, !1123}
!1123 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&dp_ex::Cat", baseType: !42, size: 64, align: 64, dwarfAddressSpace: 0)
!1124 = !{!1125}
!1125 = !DILocalVariable(name: "self", arg: 1, scope: !1119, file: !943, line: 18, type: !1123)
!1126 = !DILocation(line: 0, scope: !1119)
!1127 = !DILocalVariable(name: "pieces", scope: !1128, file: !953, line: 600, type: !1111, align: 64)
!1128 = distinct !DISubprogram(name: "new_const<1>", linkageName: "_ZN4core3fmt9Arguments9new_const17h4071c16c1b3d8f01E", scope: !954, file: !953, line: 600, type: !1109, scopeLine: 600, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, declaration: !1113, retainedNodes: !1129)
!1129 = !{!1127}
!1130 = !DILocation(line: 0, scope: !1128, inlinedAt: !1131)
!1131 = !DILocation(line: 19, column: 9, scope: !1119)
!1132 = !DILocation(line: 602, column: 9, scope: !1128, inlinedAt: !1131)
!1133 = !DILocation(line: 20, column: 6, scope: !1119)
!1134 = distinct !DISubprogram(name: "speak", linkageName: "_ZN45_$LT$dp_ex..Bird$u20$as$u20$dp_ex..Animal$GT$5speak17h900f4f573272ed1cE", scope: !1135, file: !943, line: 26, type: !1136, scopeLine: 26, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !1139)
!1135 = !DINamespace(name: "{impl#2}", scope: !33)
!1136 = !DISubroutineType(types: !1137)
!1137 = !{null, !1138}
!1138 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&dp_ex::Bird", baseType: !32, size: 64, align: 64, dwarfAddressSpace: 0)
!1139 = !{!1140}
!1140 = !DILocalVariable(name: "self", arg: 1, scope: !1134, file: !943, line: 26, type: !1138)
!1141 = !DILocation(line: 0, scope: !1134)
!1142 = !DILocalVariable(name: "pieces", scope: !1143, file: !953, line: 600, type: !1111, align: 64)
!1143 = distinct !DISubprogram(name: "new_const<1>", linkageName: "_ZN4core3fmt9Arguments9new_const17h4071c16c1b3d8f01E", scope: !954, file: !953, line: 600, type: !1109, scopeLine: 600, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, declaration: !1113, retainedNodes: !1144)
!1144 = !{!1142}
!1145 = !DILocation(line: 0, scope: !1143, inlinedAt: !1146)
!1146 = !DILocation(line: 27, column: 9, scope: !1134)
!1147 = !DILocation(line: 602, column: 9, scope: !1143, inlinedAt: !1146)
!1148 = !DILocation(line: 28, column: 6, scope: !1134)
!1149 = distinct !DISubprogram(name: "animal_speak", scope: !33, file: !943, line: 32, type: !1150, scopeLine: 32, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !1160)
!1150 = !DISubroutineType(types: !1151)
!1151 = !{null, !1152}
!1152 = !DICompositeType(tag: DW_TAG_structure_type, name: "&dyn dp_ex::Animal", file: !2, size: 128, align: 64, elements: !1153, templateParams: !23, identifier: "a178c33aa19ec831c4f9d4301ca5e8d5")
!1153 = !{!1154, !1157}
!1154 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !1152, file: !2, baseType: !1155, size: 64, align: 64)
!1155 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !1156, size: 64, align: 64, dwarfAddressSpace: 0)
!1156 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn dp_ex::Animal", file: !2, align: 8, elements: !23, identifier: "df9ac727fbc75c2d941b2128404024cc")
!1157 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !1152, file: !2, baseType: !1158, size: 64, align: 64, offset: 64)
!1158 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 4]", baseType: !1159, size: 64, align: 64, dwarfAddressSpace: 0)
!1159 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 256, align: 64, elements: !222)
!1160 = !{!1161}
!1161 = !DILocalVariable(name: "animal", arg: 1, scope: !1149, file: !943, line: 32, type: !1152)
!1162 = !DILocation(line: 0, scope: !1149)
!1163 = !DILocation(line: 33, column: 5, scope: !1149)
!1164 = !DILocation(line: 34, column: 2, scope: !1149)
!1165 = distinct !DISubprogram(name: "main", linkageName: "_ZN5dp_ex4main17hc7990c7b9cee8a83E", scope: !33, file: !943, line: 81, type: !21, scopeLine: 81, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized | DISPFlagMainSubprogram, unit: !58, templateParams: !23)
!1166 = !DILocation(line: 39, column: 20, scope: !1167, inlinedAt: !1173)
!1167 = distinct !DILexicalBlock(scope: !1168, file: !943, line: 37, column: 5)
!1168 = distinct !DISubprogram(name: "dyn_dp", linkageName: "_ZN5dp_ex6dyn_dp17h87c4dad2651e3135E", scope: !33, file: !943, line: 36, type: !21, scopeLine: 36, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !1169)
!1169 = !{!1170, !1171}
!1170 = !DILocalVariable(name: "a", scope: !1167, file: !943, line: 37, type: !1152, align: 64)
!1171 = !DILocalVariable(name: "num", scope: !1172, file: !943, line: 39, type: !221, align: 32)
!1172 = distinct !DILexicalBlock(scope: !1167, file: !943, line: 39, column: 5)
!1173 = distinct !DILocation(line: 96, column: 5, scope: !1165)
!1174 = !DILocalVariable(name: "range", arg: 2, scope: !1175, file: !1176, line: 161, type: !1233)
!1175 = distinct !DISubprogram(name: "random_range<rand::rngs::thread::ThreadRng, u32, core::ops::range::RangeTo<u32>>", linkageName: "_ZN4rand3rng3Rng12random_range17ha83b341ba699d1f8E", scope: !1177, file: !1176, line: 161, type: !1179, scopeLine: 161, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1248, retainedNodes: !1246)
!1176 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/rng.rs", directory: "", checksumkind: CSK_MD5, checksum: "502aa58ff0bfc907d978a28fc3e03a36")
!1177 = !DINamespace(name: "Rng", scope: !1178)
!1178 = !DINamespace(name: "rng", scope: !64)
!1179 = !DISubroutineType(types: !1180)
!1180 = !{!221, !1181, !1233, !1238}
!1181 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand::rngs::thread::ThreadRng", baseType: !1182, size: 64, align: 64, dwarfAddressSpace: 0)
!1182 = !DICompositeType(tag: DW_TAG_structure_type, name: "ThreadRng", scope: !1183, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1184, templateParams: !23, identifier: "4ab1881b8b411a5cadc96c64932ddfa7")
!1183 = !DINamespace(name: "thread", scope: !203)
!1184 = !{!1185}
!1185 = !DIDerivedType(tag: DW_TAG_member, name: "rng", scope: !1182, file: !2, baseType: !1186, size: 64, align: 64, flags: DIFlagPrivate)
!1186 = !DICompositeType(tag: DW_TAG_structure_type, name: "Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", scope: !1187, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1189, templateParams: !1231, identifier: "2441776dfe26f17ea1e97b5724f08ec3")
!1187 = !DINamespace(name: "rc", scope: !1188)
!1188 = !DINamespace(name: "alloc", scope: null)
!1189 = !{!1190, !1226, !1228}
!1190 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !1186, file: !2, baseType: !1191, size: 64, align: 64, flags: DIFlagPrivate)
!1191 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", scope: !574, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1192, templateParams: !1224, identifier: "6657d814155cb6a4fef2f2c8a5d2ed76")
!1192 = !{!1193}
!1193 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !1191, file: !2, baseType: !1194, size: 64, align: 64, flags: DIFlagPrivate)
!1194 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>", baseType: !1195, size: 64, align: 64, dwarfAddressSpace: 0)
!1195 = !DICompositeType(tag: DW_TAG_structure_type, name: "RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>", scope: !1187, file: !2, size: 2816, align: 128, flags: DIFlagPrivate, elements: !1196, templateParams: !1222, identifier: "2ba302f6965b308a386cfd5850553c95")
!1196 = !{!1197, !1204, !1205}
!1197 = !DIDerivedType(tag: DW_TAG_member, name: "strong", scope: !1195, file: !2, baseType: !1198, size: 64, align: 64, flags: DIFlagPrivate)
!1198 = !DICompositeType(tag: DW_TAG_structure_type, name: "Cell<usize>", scope: !605, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1199, templateParams: !823, identifier: "850821af38703e743ab2164fb52c6e1a")
!1199 = !{!1200}
!1200 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1198, file: !2, baseType: !1201, size: 64, align: 64, flags: DIFlagPrivate)
!1201 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<usize>", scope: !605, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1202, templateParams: !823, identifier: "13a2388050bdea621562362d97db0cb8")
!1202 = !{!1203}
!1203 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1201, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPrivate)
!1204 = !DIDerivedType(tag: DW_TAG_member, name: "weak", scope: !1195, file: !2, baseType: !1198, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!1205 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1195, file: !2, baseType: !1206, size: 2688, align: 128, offset: 128, flags: DIFlagPrivate)
!1206 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", scope: !605, file: !2, size: 2688, align: 128, flags: DIFlagPublic, elements: !1207, templateParams: !1220, identifier: "1e6fe10d094e887acf7e418c5201184a")
!1207 = !{!1208}
!1208 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1206, file: !2, baseType: !1209, size: 2688, align: 128, flags: DIFlagPrivate)
!1209 = !DICompositeType(tag: DW_TAG_structure_type, name: "ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", scope: !202, file: !2, size: 2688, align: 128, flags: DIFlagPublic, elements: !1210, templateParams: !250, identifier: "92ed0024a69ea761e3c8116b8e389515")
!1210 = !{!1211}
!1211 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1209, file: !2, baseType: !1212, size: 2688, align: 128, flags: DIFlagPrivate)
!1212 = !DICompositeType(tag: DW_TAG_structure_type, name: "BlockRng<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", scope: !1213, file: !2, size: 2688, align: 128, flags: DIFlagPublic, elements: !1214, templateParams: !1218, identifier: "a738b89e9f785734824d0ef2945771d6")
!1213 = !DINamespace(name: "block", scope: !247)
!1214 = !{!1215, !1216, !1217}
!1215 = !DIDerivedType(tag: DW_TAG_member, name: "results", scope: !1212, file: !2, baseType: !257, size: 2048, align: 32, flags: DIFlagPrivate)
!1216 = !DIDerivedType(tag: DW_TAG_member, name: "index", scope: !1212, file: !2, baseType: !9, size: 64, align: 64, offset: 2560, flags: DIFlagPrivate)
!1217 = !DIDerivedType(tag: DW_TAG_member, name: "core", scope: !1212, file: !2, baseType: !201, size: 512, align: 128, offset: 2048, flags: DIFlagPublic)
!1218 = !{!1219}
!1219 = !DITemplateTypeParameter(name: "R", type: !201)
!1220 = !{!1221}
!1221 = !DITemplateTypeParameter(name: "T", type: !1209)
!1222 = !{!1223}
!1223 = !DITemplateTypeParameter(name: "T", type: !1206)
!1224 = !{!1225}
!1225 = !DITemplateTypeParameter(name: "T", type: !1195)
!1226 = !DIDerivedType(tag: DW_TAG_member, name: "phantom", scope: !1186, file: !2, baseType: !1227, align: 8, offset: 64, flags: DIFlagPrivate)
!1227 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", scope: !1099, file: !2, align: 8, flags: DIFlagPublic, elements: !23, templateParams: !1224, identifier: "e9f1737b92b3623ac8b3aa7b210147f9")
!1228 = !DIDerivedType(tag: DW_TAG_member, name: "alloc", scope: !1186, file: !2, baseType: !1229, align: 8, offset: 64, flags: DIFlagPrivate)
!1229 = !DICompositeType(tag: DW_TAG_structure_type, name: "Global", scope: !1230, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "cc328e40098c35b06088ca0399a07f60")
!1230 = !DINamespace(name: "alloc", scope: !1188)
!1231 = !{!1223, !1232}
!1232 = !DITemplateTypeParameter(name: "A", type: !1229)
!1233 = !DICompositeType(tag: DW_TAG_structure_type, name: "RangeTo<u32>", scope: !773, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !1234, templateParams: !1236, identifier: "48f7e25e7c0f7b525bf4264ad22cd972")
!1234 = !{!1235}
!1235 = !DIDerivedType(tag: DW_TAG_member, name: "end", scope: !1233, file: !2, baseType: !221, size: 32, align: 32, flags: DIFlagPublic)
!1236 = !{!1237}
!1237 = !DITemplateTypeParameter(name: "Idx", type: !221)
!1238 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::panic::location::Location", baseType: !1239, size: 64, align: 64, dwarfAddressSpace: 0)
!1239 = !DICompositeType(tag: DW_TAG_structure_type, name: "Location", scope: !1240, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !1242, templateParams: !23, identifier: "632d586581923cc25c0b9d839e71393c")
!1240 = !DINamespace(name: "location", scope: !1241)
!1241 = !DINamespace(name: "panic", scope: !71)
!1242 = !{!1243, !1244, !1245}
!1243 = !DIDerivedType(tag: DW_TAG_member, name: "file", scope: !1239, file: !2, baseType: !961, size: 128, align: 64, flags: DIFlagPrivate)
!1244 = !DIDerivedType(tag: DW_TAG_member, name: "line", scope: !1239, file: !2, baseType: !221, size: 32, align: 32, offset: 128, flags: DIFlagPrivate)
!1245 = !DIDerivedType(tag: DW_TAG_member, name: "col", scope: !1239, file: !2, baseType: !221, size: 32, align: 32, offset: 160, flags: DIFlagPrivate)
!1246 = !{!1247, !1174}
!1247 = !DILocalVariable(name: "self", arg: 1, scope: !1175, file: !1176, line: 161, type: !1181)
!1248 = !{!1249, !264, !1250}
!1249 = !DITemplateTypeParameter(name: "Self", type: !1182)
!1250 = !DITemplateTypeParameter(name: "R", type: !1233)
!1251 = !DILocation(line: 0, scope: !1175, inlinedAt: !1252)
!1252 = distinct !DILocation(line: 39, column: 20, scope: !1167, inlinedAt: !1173)
!1253 = !DILocalVariable(name: "self", arg: 1, scope: !1254, file: !1255, line: 458, type: !1233)
!1254 = distinct !DISubprogram(name: "sample_single<rand::rngs::thread::ThreadRng>", linkageName: "_ZN101_$LT$core..ops..range..RangeTo$LT$u32$GT$$u20$as$u20$rand..distr..uniform..SampleRange$LT$u32$GT$$GT$13sample_single17h91b6f2b4113c1537E", scope: !1256, file: !1255, line: 458, type: !1257, scopeLine: 458, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1276, retainedNodes: !1274)
!1255 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/distr/uniform.rs", directory: "", checksumkind: CSK_MD5, checksum: "86316dcdf17b8e0b3477ac1272e2ff41")
!1256 = !DINamespace(name: "{impl#26}", scope: !62)
!1257 = !DISubroutineType(types: !1258)
!1258 = !{!1259, !1233, !1181}
!1259 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<u32, rand::distr::uniform::Error>", scope: !303, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !1260, templateParams: !23, identifier: "3c1cada4139609b5fcf1f55b1087f1fa")
!1260 = !{!1261}
!1261 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1259, file: !2, size: 64, align: 32, elements: !1262, templateParams: !23, identifier: "26e24ef08779c4b1b47d1562aeb5e6e7", discriminator: !1273)
!1262 = !{!1263, !1269}
!1263 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1261, file: !2, baseType: !1264, size: 64, align: 32, extraData: i8 0)
!1264 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1259, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !1265, templateParams: !1267, identifier: "731fefb81fcdce553077d4fbbf890f09")
!1265 = !{!1266}
!1266 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1264, file: !2, baseType: !221, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!1267 = !{!264, !1268}
!1268 = !DITemplateTypeParameter(name: "E", type: !61)
!1269 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1261, file: !2, baseType: !1270, size: 64, align: 32, extraData: i8 1)
!1270 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1259, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !1271, templateParams: !1267, identifier: "34b532e45f94b39f4d596f13241a9c83")
!1271 = !{!1272}
!1272 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1270, file: !2, baseType: !61, size: 8, align: 8, offset: 8, flags: DIFlagPublic)
!1273 = !DIDerivedType(tag: DW_TAG_member, scope: !1259, file: !2, baseType: !65, size: 8, align: 8, flags: DIFlagArtificial)
!1274 = !{!1253, !1275}
!1275 = !DILocalVariable(name: "rng", arg: 2, scope: !1254, file: !1255, line: 458, type: !1181)
!1276 = !{!1277}
!1277 = !DITemplateTypeParameter(name: "R", type: !1182)
!1278 = !DILocation(line: 0, scope: !1254, inlinedAt: !1279)
!1279 = distinct !DILocation(line: 167, column: 9, scope: !1175, inlinedAt: !1252)
!1280 = !DILocalVariable(name: "low_b", arg: 1, scope: !1281, file: !1282, line: 154, type: !221)
!1281 = distinct !DISubprogram(name: "sample_single<rand::rngs::thread::ThreadRng, u32, u32>", linkageName: "_ZN105_$LT$rand..distr..uniform..int..UniformInt$LT$u32$GT$$u20$as$u20$rand..distr..uniform..UniformSampler$GT$13sample_single17hb146a428033ec591E", scope: !1283, file: !1282, line: 153, type: !1285, scopeLine: 153, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1294, retainedNodes: !1287)
!1282 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/distr/uniform_int.rs", directory: "", checksumkind: CSK_MD5, checksum: "c4ac3345496b768f96867781b41a134c")
!1283 = !DINamespace(name: "{impl#23}", scope: !1284)
!1284 = !DINamespace(name: "int", scope: !62)
!1285 = !DISubroutineType(types: !1286)
!1286 = !{!1259, !221, !221, !1181}
!1287 = !{!1280, !1288, !1289, !1290, !1292}
!1288 = !DILocalVariable(name: "high_b", arg: 2, scope: !1281, file: !1282, line: 155, type: !221)
!1289 = !DILocalVariable(name: "rng", arg: 3, scope: !1281, file: !1282, line: 156, type: !1181)
!1290 = !DILocalVariable(name: "low", scope: !1291, file: !1282, line: 162, type: !221, align: 32)
!1291 = distinct !DILexicalBlock(scope: !1281, file: !1282, line: 162, column: 17)
!1292 = !DILocalVariable(name: "high", scope: !1293, file: !1282, line: 163, type: !221, align: 32)
!1293 = distinct !DILexicalBlock(scope: !1291, file: !1282, line: 163, column: 17)
!1294 = !{!1277, !1295, !1296}
!1295 = !DITemplateTypeParameter(name: "B1", type: !221)
!1296 = !DITemplateTypeParameter(name: "B2", type: !221)
!1297 = !DILocation(line: 0, scope: !1281, inlinedAt: !1298)
!1298 = distinct !DILocation(line: 459, column: 17, scope: !1254, inlinedAt: !1279)
!1299 = !DILocation(line: 0, scope: !1291, inlinedAt: !1298)
!1300 = !DILocation(line: 0, scope: !1293, inlinedAt: !1298)
!1301 = !DILocalVariable(name: "low_b", arg: 1, scope: !1302, file: !1282, line: 177, type: !221)
!1302 = distinct !DISubprogram(name: "sample_single_inclusive<rand::rngs::thread::ThreadRng, u32, u32>", linkageName: "_ZN105_$LT$rand..distr..uniform..int..UniformInt$LT$u32$GT$$u20$as$u20$rand..distr..uniform..UniformSampler$GT$23sample_single_inclusive17h0ca7f56543a6235cE", scope: !1283, file: !1282, line: 176, type: !1285, scopeLine: 176, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1294, retainedNodes: !1303)
!1303 = !{!1301, !1304, !1305, !1306, !1308, !1310, !1312, !1314, !1315, !1317}
!1304 = !DILocalVariable(name: "high_b", arg: 2, scope: !1302, file: !1282, line: 178, type: !221)
!1305 = !DILocalVariable(name: "rng", arg: 3, scope: !1302, file: !1282, line: 179, type: !1181)
!1306 = !DILocalVariable(name: "low", scope: !1307, file: !1282, line: 185, type: !221, align: 32)
!1307 = distinct !DILexicalBlock(scope: !1302, file: !1282, line: 185, column: 17)
!1308 = !DILocalVariable(name: "high", scope: !1309, file: !1282, line: 186, type: !221, align: 32)
!1309 = distinct !DILexicalBlock(scope: !1307, file: !1282, line: 186, column: 17)
!1310 = !DILocalVariable(name: "range", scope: !1311, file: !1282, line: 190, type: !221, align: 32)
!1311 = distinct !DILexicalBlock(scope: !1309, file: !1282, line: 190, column: 17)
!1312 = !DILocalVariable(name: "result", scope: !1313, file: !1282, line: 197, type: !221, align: 32)
!1313 = distinct !DILexicalBlock(scope: !1311, file: !1282, line: 197, column: 17)
!1314 = !DILocalVariable(name: "lo_order", scope: !1313, file: !1282, line: 197, type: !221, align: 32)
!1315 = !DILocalVariable(name: "new_hi_order", scope: !1316, file: !1282, line: 202, type: !221, align: 32)
!1316 = distinct !DILexicalBlock(scope: !1313, file: !1282, line: 202, column: 21)
!1317 = !DILocalVariable(name: "is_overflow", scope: !1318, file: !1282, line: 204, type: !591, align: 8)
!1318 = distinct !DILexicalBlock(scope: !1316, file: !1282, line: 204, column: 21)
!1319 = !DILocation(line: 0, scope: !1302, inlinedAt: !1320)
!1320 = distinct !DILocation(line: 167, column: 17, scope: !1293, inlinedAt: !1298)
!1321 = !DILocation(line: 0, scope: !1307, inlinedAt: !1320)
!1322 = !DILocation(line: 0, scope: !1309, inlinedAt: !1320)
!1323 = !DILocation(line: 0, scope: !1311, inlinedAt: !1320)
!1324 = !DILocalVariable(name: "x", arg: 2, scope: !1325, file: !1326, line: 28, type: !221)
!1325 = distinct !DISubprogram(name: "wmul", linkageName: "_ZN60_$LT$u32$u20$as$u20$rand..distr..utils..WideningMultiply$GT$4wmul17h290d0e28aef3d8b7E", scope: !1327, file: !1326, line: 28, type: !1329, scopeLine: 28, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !1335)
!1326 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/distr/utils.rs", directory: "", checksumkind: CSK_MD5, checksum: "c3a51cef73487558fdaefa4944537a68")
!1327 = !DINamespace(name: "{impl#5}", scope: !1328)
!1328 = !DINamespace(name: "utils", scope: !63)
!1329 = !DISubroutineType(types: !1330)
!1330 = !{!1331, !221, !221}
!1331 = !DICompositeType(tag: DW_TAG_structure_type, name: "(u32, u32)", file: !2, size: 64, align: 32, elements: !1332, templateParams: !23, identifier: "17be3570ed04f451565b4d5e18e8cfe4")
!1332 = !{!1333, !1334}
!1333 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1331, file: !2, baseType: !221, size: 32, align: 32)
!1334 = !DIDerivedType(tag: DW_TAG_member, name: "__1", scope: !1331, file: !2, baseType: !221, size: 32, align: 32, offset: 32)
!1335 = !{!1336, !1324, !1337, !1339}
!1336 = !DILocalVariable(name: "self", arg: 1, scope: !1325, file: !1326, line: 28, type: !221)
!1337 = !DILocalVariable(name: "tmp", scope: !1338, file: !1326, line: 29, type: !226, align: 64)
!1338 = distinct !DILexicalBlock(scope: !1325, file: !1326, line: 29, column: 17)
!1339 = !DILocalVariable(name: "tmp", scope: !1340, file: !1326, line: 29, type: !226, align: 64)
!1340 = distinct !DILexicalBlock(scope: !1325, file: !1326, line: 29, column: 17)
!1341 = !DILocation(line: 0, scope: !1325, inlinedAt: !1342)
!1342 = distinct !DILocation(line: 197, column: 73, scope: !1311, inlinedAt: !1320)
!1343 = !DILocation(line: 0, scope: !1325, inlinedAt: !1344)
!1344 = distinct !DILocation(line: 202, column: 74, scope: !1313, inlinedAt: !1320)
!1345 = !DILocalVariable(name: "self", arg: 1, scope: !1346, file: !1176, line: 95, type: !1181)
!1346 = distinct !DISubprogram(name: "random<rand::rngs::thread::ThreadRng, u32>", linkageName: "_ZN4rand3rng3Rng6random17h79f0c87c9997e7cdE", scope: !1177, file: !1176, line: 95, type: !1347, scopeLine: 95, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1350, retainedNodes: !1349)
!1347 = !DISubroutineType(types: !1348)
!1348 = !{!221, !1181}
!1349 = !{!1345}
!1350 = !{!1249, !264}
!1351 = !DILocation(line: 0, scope: !1346, inlinedAt: !1352)
!1352 = distinct !DILocation(line: 193, column: 31, scope: !1311, inlinedAt: !1320)
!1353 = !DILocation(line: 0, scope: !1346, inlinedAt: !1354)
!1354 = distinct !DILocation(line: 197, column: 46, scope: !1311, inlinedAt: !1320)
!1355 = !DILocalVariable(name: "self", arg: 1, scope: !1356, file: !1357, line: 44, type: !1362)
!1356 = distinct !DISubprogram(name: "sample<rand::rngs::thread::ThreadRng>", linkageName: "_ZN4rand5distr7integer109_$LT$impl$u20$rand..distr..distribution..Distribution$LT$u32$GT$$u20$for$u20$rand..distr..StandardUniform$GT$6sample17hffe9556e703a128cE", scope: !1358, file: !1357, line: 44, type: !1360, scopeLine: 44, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1276, retainedNodes: !1364)
!1357 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/distr/integer.rs", directory: "", checksumkind: CSK_MD5, checksum: "cfec955451e26eda8446b93515b251f1")
!1358 = !DINamespace(name: "{impl#2}", scope: !1359)
!1359 = !DINamespace(name: "integer", scope: !63)
!1360 = !DISubroutineType(types: !1361)
!1361 = !{!221, !1362, !1181}
!1362 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&rand::distr::StandardUniform", baseType: !1363, size: 64, align: 64, dwarfAddressSpace: 0)
!1363 = !DICompositeType(tag: DW_TAG_structure_type, name: "StandardUniform", scope: !63, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "1cb1e543b6950708d7acc6b4a4f54ff3")
!1364 = !{!1355, !1365}
!1365 = !DILocalVariable(name: "rng", arg: 2, scope: !1356, file: !1357, line: 44, type: !1181)
!1366 = !DILocation(line: 0, scope: !1356, inlinedAt: !1367)
!1367 = distinct !DILocation(line: 99, column: 9, scope: !1346, inlinedAt: !1352)
!1368 = !DILocation(line: 0, scope: !1356, inlinedAt: !1369)
!1369 = distinct !DILocation(line: 99, column: 9, scope: !1346, inlinedAt: !1354)
!1370 = !DILocalVariable(name: "self", arg: 1, scope: !1371, file: !1372, line: 170, type: !1181)
!1371 = distinct !DISubprogram(name: "next_u32", linkageName: "_ZN68_$LT$rand..rngs..thread..ThreadRng$u20$as$u20$rand_core..RngCore$GT$8next_u3217h46677650533b7ff9E", scope: !1373, file: !1372, line: 170, type: !1347, scopeLine: 170, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !1374)
!1372 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/rngs/thread.rs", directory: "", checksumkind: CSK_MD5, checksum: "842ea45780d9ccda805de377693390ed")
!1373 = !DINamespace(name: "{impl#3}", scope: !1183)
!1374 = !{!1370, !1375}
!1375 = !DILocalVariable(name: "rng", scope: !1376, file: !1372, line: 173, type: !1377, align: 64)
!1376 = distinct !DILexicalBlock(scope: !1371, file: !1372, line: 173, column: 9)
!1377 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", baseType: !1209, size: 64, align: 64, dwarfAddressSpace: 0)
!1378 = !DILocation(line: 0, scope: !1371, inlinedAt: !1379)
!1379 = distinct !DILocation(line: 45, column: 9, scope: !1356, inlinedAt: !1367)
!1380 = !DILocation(line: 0, scope: !1371, inlinedAt: !1381)
!1381 = distinct !DILocation(line: 45, column: 9, scope: !1356, inlinedAt: !1369)
!1382 = !DILocalVariable(name: "self", arg: 1, scope: !1383, file: !1384, line: 2194, type: !1387)
!1383 = distinct !DISubprogram(name: "get<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h9888cd175f29ba9eE", scope: !1206, file: !1384, line: 2194, type: !1385, scopeLine: 2194, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1220, declaration: !1388, retainedNodes: !1389)
!1384 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/cell.rs", directory: "", checksumkind: CSK_MD5, checksum: "64646115eb882926cf02189caaf4b938")
!1385 = !DISubroutineType(types: !1386)
!1386 = !{!1377, !1387}
!1387 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", baseType: !1206, size: 64, align: 64, dwarfAddressSpace: 0)
!1388 = !DISubprogram(name: "get<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h9888cd175f29ba9eE", scope: !1206, file: !1384, line: 2194, type: !1385, scopeLine: 2194, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1220)
!1389 = !{!1382}
!1390 = !DILocation(line: 0, scope: !1383, inlinedAt: !1391)
!1391 = distinct !DILocation(line: 173, column: 43, scope: !1371, inlinedAt: !1379)
!1392 = !DILocation(line: 0, scope: !1383, inlinedAt: !1393)
!1393 = distinct !DILocation(line: 173, column: 43, scope: !1371, inlinedAt: !1381)
!1394 = !DILocation(line: 2198, column: 9, scope: !1383, inlinedAt: !1395)
!1395 = distinct !DILocation(line: 173, column: 43, scope: !1371, inlinedAt: !1396)
!1396 = distinct !DILocation(line: 45, column: 9, scope: !1356, inlinedAt: !1397)
!1397 = distinct !DILocation(line: 99, column: 9, scope: !1346, inlinedAt: !1398)
!1398 = distinct !DILocation(line: 0, scope: !1311, inlinedAt: !1320)
!1399 = !DILocation(line: 0, scope: !1376, inlinedAt: !1379)
!1400 = !DILocation(line: 0, scope: !1376, inlinedAt: !1381)
!1401 = !DILocalVariable(name: "self", arg: 1, scope: !1402, file: !200, line: 113, type: !1377)
!1402 = distinct !DISubprogram(name: "next_u32<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN90_$LT$rand..rngs..reseeding..ReseedingRng$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h2012677de426454fE", scope: !1403, file: !200, line: 113, type: !1404, scopeLine: 113, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !250, retainedNodes: !1407)
!1403 = !DINamespace(name: "{impl#1}", scope: !202)
!1404 = !DISubroutineType(types: !1405)
!1405 = !{!221, !1406}
!1406 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", baseType: !1209, size: 64, align: 64, dwarfAddressSpace: 0)
!1407 = !{!1401}
!1408 = !DILocation(line: 0, scope: !1402, inlinedAt: !1409)
!1409 = distinct !DILocation(line: 174, column: 13, scope: !1376, inlinedAt: !1379)
!1410 = !DILocation(line: 0, scope: !1402, inlinedAt: !1411)
!1411 = distinct !DILocation(line: 174, column: 13, scope: !1376, inlinedAt: !1381)
!1412 = !DILocalVariable(name: "self", arg: 1, scope: !1413, file: !1414, line: 186, type: !1418)
!1413 = distinct !DISubprogram(name: "next_u32<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h0fdce2b914f75430E", scope: !1415, file: !1414, line: 186, type: !1416, scopeLine: 186, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1218, retainedNodes: !1419)
!1414 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_core-0.9.3/src/block.rs", directory: "", checksumkind: CSK_MD5, checksum: "b5ef00244c22c51d50ecb2ae1bb4a1ae")
!1415 = !DINamespace(name: "{impl#2}", scope: !1213)
!1416 = !DISubroutineType(types: !1417)
!1417 = !{!221, !1418}
!1418 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand_core::block::BlockRng<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", baseType: !1212, size: 64, align: 64, dwarfAddressSpace: 0)
!1419 = !{!1412, !1420}
!1420 = !DILocalVariable(name: "value", scope: !1421, file: !1414, line: 191, type: !221, align: 32)
!1421 = distinct !DILexicalBlock(scope: !1413, file: !1414, line: 191, column: 9)
!1422 = !DILocation(line: 0, scope: !1413, inlinedAt: !1423)
!1423 = distinct !DILocation(line: 114, column: 9, scope: !1402, inlinedAt: !1409)
!1424 = !DILocation(line: 0, scope: !1413, inlinedAt: !1425)
!1425 = distinct !DILocation(line: 114, column: 9, scope: !1402, inlinedAt: !1411)
!1426 = !DILocation(line: 187, column: 12, scope: !1413, inlinedAt: !1427)
!1427 = distinct !DILocation(line: 114, column: 9, scope: !1402, inlinedAt: !1428)
!1428 = distinct !DILocation(line: 174, column: 13, scope: !1376, inlinedAt: !1396)
!1429 = !DILocation(line: 187, column: 12, scope: !1413, inlinedAt: !1425)
!1430 = !DILocalVariable(name: "self", arg: 1, scope: !1431, file: !1414, line: 177, type: !1418)
!1431 = distinct !DISubprogram(name: "generate_and_set<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h8021faec2696411eE", scope: !1212, file: !1414, line: 177, type: !1432, scopeLine: 177, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1218, declaration: !1434, retainedNodes: !1435)
!1432 = !DISubroutineType(types: !1433)
!1433 = !{null, !1418, !9}
!1434 = !DISubprogram(name: "generate_and_set<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h8021faec2696411eE", scope: !1212, file: !1414, line: 177, type: !1432, scopeLine: 177, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1218)
!1435 = !{!1430, !1436}
!1436 = !DILocalVariable(name: "index", arg: 2, scope: !1431, file: !1414, line: 177, type: !9)
!1437 = !DILocation(line: 0, scope: !1431, inlinedAt: !1438)
!1438 = distinct !DILocation(line: 188, column: 13, scope: !1413, inlinedAt: !1425)
!1439 = !DILocation(line: 179, column: 9, scope: !1431, inlinedAt: !1438)
!1440 = !DILocalVariable(name: "results", arg: 2, scope: !1441, file: !200, line: 162, type: !256)
!1441 = distinct !DISubprogram(name: "generate<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h63618727820f9588E", scope: !1442, file: !200, line: 162, type: !253, scopeLine: 162, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !250, retainedNodes: !1443)
!1442 = !DINamespace(name: "{impl#4}", scope: !202)
!1443 = !{!1444, !1440, !1445}
!1444 = !DILocalVariable(name: "self", arg: 1, scope: !1441, file: !200, line: 162, type: !255)
!1445 = !DILocalVariable(name: "num_bytes", scope: !1446, file: !200, line: 169, type: !9, align: 64)
!1446 = distinct !DILexicalBlock(scope: !1441, file: !200, line: 169, column: 9)
!1447 = !DILocation(line: 0, scope: !1441, inlinedAt: !1448)
!1448 = distinct !DILocation(line: 179, column: 9, scope: !1431, inlinedAt: !1438)
!1449 = !DILocation(line: 163, column: 12, scope: !1441, inlinedAt: !1448)
!1450 = !{!1451, !1453, !1455}
!1451 = distinct !{!1451, !1452, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h63618727820f9588E: %self"}
!1452 = distinct !{!1452, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h63618727820f9588E"}
!1453 = distinct !{!1453, !1454, !"_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h8021faec2696411eE: %self"}
!1454 = distinct !{!1454, !"_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h8021faec2696411eE"}
!1455 = distinct !{!1455, !1456, !"_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h0fdce2b914f75430E: %self"}
!1456 = distinct !{!1456, !"_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h0fdce2b914f75430E"}
!1457 = !{!1458, !1459}
!1458 = distinct !{!1458, !1452, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h63618727820f9588E: argument 1"}
!1459 = distinct !{!1459, !1460, !"_ZN68_$LT$rand..rngs..thread..ThreadRng$u20$as$u20$rand_core..RngCore$GT$8next_u3217h46677650533b7ff9E: %self"}
!1460 = distinct !{!1460, !"_ZN68_$LT$rand..rngs..thread..ThreadRng$u20$as$u20$rand_core..RngCore$GT$8next_u3217h46677650533b7ff9E"}
!1461 = !DILocation(line: 0, scope: !1446, inlinedAt: !1448)
!1462 = !DILocation(line: 170, column: 9, scope: !1446, inlinedAt: !1448)
!1463 = !DILocation(line: 0, scope: !920, inlinedAt: !1464)
!1464 = distinct !DILocation(line: 171, column: 9, scope: !1446, inlinedAt: !1448)
!1465 = !DILocation(line: 0, scope: !929, inlinedAt: !1466)
!1466 = distinct !DILocation(line: 91, column: 28, scope: !920, inlinedAt: !1464)
!1467 = !DILocation(line: 81, column: 9, scope: !929, inlinedAt: !1466)
!1468 = !DILocation(line: 167, column: 20, scope: !1441, inlinedAt: !1448)
!1469 = !DILocation(line: 191, column: 43, scope: !1413, inlinedAt: !1425)
!1470 = !DILocation(line: 191, column: 21, scope: !1413, inlinedAt: !1425)
!1471 = !{!1455}
!1472 = !{!1459}
!1473 = !DILocation(line: 0, scope: !1421, inlinedAt: !1425)
!1474 = !DILocation(line: 192, column: 9, scope: !1421, inlinedAt: !1425)
!1475 = !DILocation(line: 29, column: 27, scope: !1325, inlinedAt: !1342)
!1476 = !DILocation(line: 0, scope: !1338, inlinedAt: !1342)
!1477 = !DILocation(line: 30, column: 18, scope: !1338, inlinedAt: !1342)
!1478 = !DILocation(line: 24, column: 35, scope: !1338, inlinedAt: !1342)
!1479 = !DILocation(line: 0, scope: !1313, inlinedAt: !1320)
!1480 = !DILocalVariable(name: "self", arg: 1, scope: !1481, file: !813, line: 531, type: !221)
!1481 = distinct !DISubprogram(name: "checked_add", linkageName: "_ZN4core3num21_$LT$impl$u20$u32$GT$11checked_add17h4191cb6962bcebe0E", scope: !1482, file: !813, line: 531, type: !1483, scopeLine: 531, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !1496)
!1482 = !DINamespace(name: "{impl#8}", scope: !283)
!1483 = !DISubroutineType(types: !1484)
!1484 = !{!1485, !221, !221}
!1485 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<u32>", scope: !641, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !1486, templateParams: !23, identifier: "dc3248abc74d192d1a7819fad3d96238")
!1486 = !{!1487}
!1487 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1485, file: !2, size: 64, align: 32, elements: !1488, templateParams: !23, identifier: "3ff9042edade1ef318c5fb0fec0d6f49", discriminator: !1495)
!1488 = !{!1489, !1491}
!1489 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !1487, file: !2, baseType: !1490, size: 64, align: 32, extraData: i32 0)
!1490 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !1485, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !23, templateParams: !263, identifier: "47e94d63712ffe12eba83494576b290a")
!1491 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !1487, file: !2, baseType: !1492, size: 64, align: 32, extraData: i32 1)
!1492 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !1485, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !1493, templateParams: !263, identifier: "c0d8d505e522905f8b06868e5eeabad0")
!1493 = !{!1494}
!1494 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1492, file: !2, baseType: !221, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!1495 = !DIDerivedType(tag: DW_TAG_member, scope: !1485, file: !2, baseType: !221, size: 32, align: 32, flags: DIFlagArtificial)
!1496 = !{!1480, !1497}
!1497 = !DILocalVariable(name: "rhs", arg: 2, scope: !1481, file: !813, line: 531, type: !221)
!1498 = !DILocation(line: 0, scope: !1481, inlinedAt: !1499)
!1499 = distinct !DILocation(line: 204, column: 48, scope: !1316, inlinedAt: !1320)
!1500 = !DILocation(line: 200, column: 20, scope: !1313, inlinedAt: !1320)
!1501 = !DILocation(line: 0, scope: !1346, inlinedAt: !1502)
!1502 = distinct !DILocation(line: 202, column: 45, scope: !1313, inlinedAt: !1320)
!1503 = !DILocation(line: 0, scope: !1356, inlinedAt: !1504)
!1504 = distinct !DILocation(line: 99, column: 9, scope: !1346, inlinedAt: !1502)
!1505 = !DILocation(line: 0, scope: !1371, inlinedAt: !1506)
!1506 = distinct !DILocation(line: 45, column: 9, scope: !1356, inlinedAt: !1504)
!1507 = !DILocation(line: 0, scope: !1383, inlinedAt: !1508)
!1508 = distinct !DILocation(line: 173, column: 43, scope: !1371, inlinedAt: !1506)
!1509 = !DILocation(line: 0, scope: !1376, inlinedAt: !1506)
!1510 = !DILocation(line: 0, scope: !1402, inlinedAt: !1511)
!1511 = distinct !DILocation(line: 174, column: 13, scope: !1376, inlinedAt: !1506)
!1512 = !DILocation(line: 0, scope: !1413, inlinedAt: !1513)
!1513 = distinct !DILocation(line: 114, column: 9, scope: !1402, inlinedAt: !1511)
!1514 = !DILocation(line: 187, column: 12, scope: !1413, inlinedAt: !1513)
!1515 = !DILocation(line: 0, scope: !1431, inlinedAt: !1516)
!1516 = distinct !DILocation(line: 188, column: 13, scope: !1413, inlinedAt: !1513)
!1517 = !DILocation(line: 179, column: 9, scope: !1431, inlinedAt: !1516)
!1518 = !DILocation(line: 0, scope: !1441, inlinedAt: !1519)
!1519 = distinct !DILocation(line: 179, column: 9, scope: !1431, inlinedAt: !1516)
!1520 = !DILocation(line: 163, column: 12, scope: !1441, inlinedAt: !1519)
!1521 = !{!1522, !1524, !1526}
!1522 = distinct !{!1522, !1523, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h63618727820f9588E: %self"}
!1523 = distinct !{!1523, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h63618727820f9588E"}
!1524 = distinct !{!1524, !1525, !"_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h8021faec2696411eE: %self"}
!1525 = distinct !{!1525, !"_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h8021faec2696411eE"}
!1526 = distinct !{!1526, !1527, !"_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h0fdce2b914f75430E: %self"}
!1527 = distinct !{!1527, !"_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h0fdce2b914f75430E"}
!1528 = !{!1529, !1530}
!1529 = distinct !{!1529, !1523, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h63618727820f9588E: argument 1"}
!1530 = distinct !{!1530, !1531, !"_ZN68_$LT$rand..rngs..thread..ThreadRng$u20$as$u20$rand_core..RngCore$GT$8next_u3217h46677650533b7ff9E: %self"}
!1531 = distinct !{!1531, !"_ZN68_$LT$rand..rngs..thread..ThreadRng$u20$as$u20$rand_core..RngCore$GT$8next_u3217h46677650533b7ff9E"}
!1532 = !DILocation(line: 0, scope: !1446, inlinedAt: !1519)
!1533 = !DILocation(line: 170, column: 9, scope: !1446, inlinedAt: !1519)
!1534 = !DILocation(line: 0, scope: !920, inlinedAt: !1535)
!1535 = distinct !DILocation(line: 171, column: 9, scope: !1446, inlinedAt: !1519)
!1536 = !DILocation(line: 0, scope: !929, inlinedAt: !1537)
!1537 = distinct !DILocation(line: 91, column: 28, scope: !920, inlinedAt: !1535)
!1538 = !DILocation(line: 81, column: 9, scope: !929, inlinedAt: !1537)
!1539 = !DILocation(line: 167, column: 20, scope: !1441, inlinedAt: !1519)
!1540 = !DILocation(line: 191, column: 43, scope: !1413, inlinedAt: !1513)
!1541 = !DILocation(line: 191, column: 21, scope: !1413, inlinedAt: !1513)
!1542 = !{!1526}
!1543 = !{!1530}
!1544 = !DILocation(line: 0, scope: !1421, inlinedAt: !1513)
!1545 = !DILocation(line: 192, column: 9, scope: !1421, inlinedAt: !1513)
!1546 = !DILocation(line: 29, column: 27, scope: !1325, inlinedAt: !1344)
!1547 = !DILocation(line: 0, scope: !1340, inlinedAt: !1344)
!1548 = !DILocation(line: 30, column: 18, scope: !1340, inlinedAt: !1344)
!1549 = !DILocation(line: 24, column: 35, scope: !1340, inlinedAt: !1344)
!1550 = !DILocation(line: 0, scope: !1316, inlinedAt: !1320)
!1551 = !DILocation(line: 539, column: 37, scope: !1481, inlinedAt: !1499)
!1552 = !DILocation(line: 0, scope: !1318, inlinedAt: !1320)
!1553 = !DILocation(line: 205, column: 31, scope: !1318, inlinedAt: !1320)
!1554 = !DILocation(line: 205, column: 21, scope: !1318, inlinedAt: !1320)
!1555 = !DILocation(line: 200, column: 17, scope: !1313, inlinedAt: !1320)
!1556 = !{!1557}
!1557 = distinct !{!1557, !1558, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hd497d5f9546794adE: %_1"}
!1558 = distinct !{!1558, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hd497d5f9546794adE"}
!1559 = !DILocation(line: 39, column: 49, scope: !1167, inlinedAt: !1173)
!1560 = !DILocalVariable(arg: 1, scope: !1561, file: !1562, line: 523, type: !1565)
!1561 = distinct !DISubprogram(name: "drop_in_place<rand::rngs::thread::ThreadRng>", linkageName: "_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hd497d5f9546794adE", scope: !575, file: !1562, line: 523, type: !1563, scopeLine: 523, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1567, retainedNodes: !1566)
!1562 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "a5cb8c8d2ea510166b9e600d925000e6")
!1563 = !DISubroutineType(types: !1564)
!1564 = !{null, !1565}
!1565 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut rand::rngs::thread::ThreadRng", baseType: !1182, size: 64, align: 64, dwarfAddressSpace: 0)
!1566 = !{!1560}
!1567 = !{!1568}
!1568 = !DITemplateTypeParameter(name: "T", type: !1182)
!1569 = !DILocation(line: 0, scope: !1561, inlinedAt: !1570)
!1570 = distinct !DILocation(line: 39, column: 49, scope: !1167, inlinedAt: !1173)
!1571 = !{!1572}
!1572 = distinct !{!1572, !1573, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17hc39867958365ed55E: %_1"}
!1573 = distinct !{!1573, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17hc39867958365ed55E"}
!1574 = !DILocation(line: 523, column: 1, scope: !1561, inlinedAt: !1570)
!1575 = !DILocalVariable(arg: 1, scope: !1576, file: !1562, line: 523, type: !1579)
!1576 = distinct !DISubprogram(name: "drop_in_place<alloc::rc::Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>>", linkageName: "_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17hc39867958365ed55E", scope: !575, file: !1562, line: 523, type: !1577, scopeLine: 523, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1581, retainedNodes: !1580)
!1577 = !DISubroutineType(types: !1578)
!1578 = !{null, !1579}
!1579 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut alloc::rc::Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", baseType: !1186, size: 64, align: 64, dwarfAddressSpace: 0)
!1580 = !{!1575}
!1581 = !{!1582}
!1582 = !DITemplateTypeParameter(name: "T", type: !1186)
!1583 = !DILocation(line: 0, scope: !1576, inlinedAt: !1584)
!1584 = distinct !DILocation(line: 523, column: 1, scope: !1561, inlinedAt: !1570)
!1585 = !{!1586}
!1586 = distinct !{!1586, !1587, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfce35e40ab15106fE: %self"}
!1587 = distinct !{!1587, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfce35e40ab15106fE"}
!1588 = !DILocation(line: 523, column: 1, scope: !1576, inlinedAt: !1584)
!1589 = !DILocalVariable(name: "self", arg: 1, scope: !1590, file: !1591, line: 2296, type: !1595)
!1590 = distinct !DISubprogram(name: "drop<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", linkageName: "_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfce35e40ab15106fE", scope: !1592, file: !1591, line: 2296, type: !1593, scopeLine: 2296, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1231, retainedNodes: !1596)
!1591 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/alloc/src/rc.rs", directory: "", checksumkind: CSK_MD5, checksum: "3970992b02512b107ee6d682a0d1a233")
!1592 = !DINamespace(name: "{impl#32}", scope: !1187)
!1593 = !DISubroutineType(types: !1594)
!1594 = !{null, !1595}
!1595 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut alloc::rc::Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", baseType: !1186, size: 64, align: 64, dwarfAddressSpace: 0)
!1596 = !{!1589}
!1597 = !DILocation(line: 0, scope: !1590, inlinedAt: !1598)
!1598 = distinct !DILocation(line: 523, column: 1, scope: !1576, inlinedAt: !1584)
!1599 = !DILocalVariable(name: "self", arg: 1, scope: !1600, file: !1591, line: 355, type: !1595)
!1600 = distinct !DISubprogram(name: "inner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", linkageName: "_ZN5alloc2rc15Rc$LT$T$C$A$GT$5inner17haea7fea26cd47993E", scope: !1186, file: !1591, line: 355, type: !1601, scopeLine: 355, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1231, declaration: !1605, retainedNodes: !1606)
!1601 = !DISubroutineType(types: !1602)
!1602 = !{!1603, !1604}
!1603 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>", baseType: !1195, size: 64, align: 64, dwarfAddressSpace: 0)
!1604 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&alloc::rc::Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", baseType: !1186, size: 64, align: 64, dwarfAddressSpace: 0)
!1605 = !DISubprogram(name: "inner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", linkageName: "_ZN5alloc2rc15Rc$LT$T$C$A$GT$5inner17haea7fea26cd47993E", scope: !1186, file: !1591, line: 355, type: !1601, scopeLine: 355, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1231)
!1606 = !{!1599}
!1607 = !DILocation(line: 0, scope: !1600, inlinedAt: !1608)
!1608 = distinct !DILocation(line: 2298, column: 18, scope: !1590, inlinedAt: !1598)
!1609 = !DILocation(line: 0, scope: !1600, inlinedAt: !1610)
!1610 = distinct !DILocation(line: 2299, column: 21, scope: !1590, inlinedAt: !1598)
!1611 = !DILocation(line: 428, column: 20, scope: !1612, inlinedAt: !1617)
!1612 = distinct !DISubprogram(name: "as_ref<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ref17h5cb1a5262b93dcb6E", scope: !1191, file: !587, line: 424, type: !1613, scopeLine: 424, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1224, declaration: !1616)
!1613 = !DISubroutineType(types: !1614)
!1614 = !{!1603, !1615}
!1615 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::ptr::non_null::NonNull<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", baseType: !1191, size: 64, align: 64, dwarfAddressSpace: 0)
!1616 = !DISubprogram(name: "as_ref<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ref17h5cb1a5262b93dcb6E", scope: !1191, file: !587, line: 424, type: !1613, scopeLine: 424, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1224)
!1617 = distinct !DILocation(line: 358, column: 27, scope: !1600, inlinedAt: !1608)
!1618 = !{!1586, !1572, !1557}
!1619 = !DILocalVariable(name: "self", arg: 1, scope: !1620, file: !1591, line: 3570, type: !1194)
!1620 = distinct !DISubprogram(name: "dec_strong<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", linkageName: "_ZN5alloc2rc10RcInnerPtr10dec_strong17h358caec5e7ad9a10E", scope: !1621, file: !1591, line: 3570, type: !1622, scopeLine: 3570, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1625, retainedNodes: !1624)
!1621 = !DINamespace(name: "RcInnerPtr", scope: !1187)
!1622 = !DISubroutineType(types: !1623)
!1623 = !{null, !1603}
!1624 = !{!1619}
!1625 = !{!1626}
!1626 = !DITemplateTypeParameter(name: "Self", type: !1195)
!1627 = !DILocation(line: 0, scope: !1620, inlinedAt: !1628)
!1628 = distinct !DILocation(line: 2298, column: 26, scope: !1590, inlinedAt: !1598)
!1629 = !DILocalVariable(name: "self", arg: 1, scope: !1630, file: !1591, line: 3542, type: !1194)
!1630 = distinct !DISubprogram(name: "strong<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", linkageName: "_ZN5alloc2rc10RcInnerPtr6strong17h149fbb4d0b213758E", scope: !1621, file: !1591, line: 3542, type: !1631, scopeLine: 3542, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !1625, retainedNodes: !1633)
!1631 = !DISubroutineType(types: !1632)
!1632 = !{!9, !1603}
!1633 = !{!1629}
!1634 = !DILocation(line: 0, scope: !1630, inlinedAt: !1635)
!1635 = distinct !DILocation(line: 3571, column: 36, scope: !1620, inlinedAt: !1628)
!1636 = !DILocalVariable(name: "self", arg: 1, scope: !1637, file: !1384, line: 428, type: !1640)
!1637 = distinct !DISubprogram(name: "set<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$3set17hf5e22642556ec098E", scope: !1198, file: !1384, line: 428, type: !1638, scopeLine: 428, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !823, declaration: !1641, retainedNodes: !1642)
!1638 = !DISubroutineType(types: !1639)
!1639 = !{null, !1640, !9}
!1640 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::cell::Cell<usize>", baseType: !1198, size: 64, align: 64, dwarfAddressSpace: 0)
!1641 = !DISubprogram(name: "set<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$3set17hf5e22642556ec098E", scope: !1198, file: !1384, line: 428, type: !1638, scopeLine: 428, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !823)
!1642 = !{!1636, !1643}
!1643 = !DILocalVariable(name: "val", arg: 2, scope: !1637, file: !1384, line: 428, type: !9)
!1644 = !DILocation(line: 0, scope: !1637, inlinedAt: !1645)
!1645 = distinct !DILocation(line: 3571, column: 27, scope: !1620, inlinedAt: !1628)
!1646 = !DILocalVariable(name: "self", arg: 1, scope: !1647, file: !1384, line: 500, type: !1640)
!1647 = distinct !DISubprogram(name: "replace<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$7replace17h5dd5c40532223491E", scope: !1198, file: !1384, line: 500, type: !1648, scopeLine: 500, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !823, declaration: !1650, retainedNodes: !1651)
!1648 = !DISubroutineType(types: !1649)
!1649 = !{!9, !1640, !9}
!1650 = !DISubprogram(name: "replace<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$7replace17h5dd5c40532223491E", scope: !1198, file: !1384, line: 500, type: !1648, scopeLine: 500, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !823)
!1651 = !{!1646, !1652}
!1652 = !DILocalVariable(name: "val", arg: 2, scope: !1647, file: !1384, line: 500, type: !9)
!1653 = !DILocation(line: 0, scope: !1647, inlinedAt: !1654)
!1654 = distinct !DILocation(line: 429, column: 14, scope: !1637, inlinedAt: !1645)
!1655 = !DILocalVariable(name: "self", arg: 1, scope: !1656, file: !1384, line: 541, type: !1640)
!1656 = distinct !DISubprogram(name: "get<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$3get17hfccf9b0edd6bd65dE", scope: !1198, file: !1384, line: 541, type: !1657, scopeLine: 541, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !823, declaration: !1659, retainedNodes: !1660)
!1657 = !DISubroutineType(types: !1658)
!1658 = !{!9, !1640}
!1659 = !DISubprogram(name: "get<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$3get17hfccf9b0edd6bd65dE", scope: !1198, file: !1384, line: 541, type: !1657, scopeLine: 541, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !823)
!1660 = !{!1655}
!1661 = !DILocation(line: 0, scope: !1656, inlinedAt: !1662)
!1662 = distinct !DILocation(line: 3543, column: 27, scope: !1630, inlinedAt: !1635)
!1663 = !DILocation(line: 544, column: 18, scope: !1656, inlinedAt: !1662)
!1664 = !DILocation(line: 3571, column: 31, scope: !1620, inlinedAt: !1628)
!1665 = !DILocation(line: 865, column: 9, scope: !1666, inlinedAt: !1671)
!1666 = distinct !DISubprogram(name: "replace<usize>", linkageName: "_ZN4core3mem7replace17hba84a865d2017893E", scope: !504, file: !1667, line: 850, type: !1668, scopeLine: 850, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !823)
!1667 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/mem/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "f1c5365a2f0a05a42c44d469d395c8a1")
!1668 = !DISubroutineType(types: !1669)
!1669 = !{!9, !1670, !9}
!1670 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut usize", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 0)
!1671 = distinct !DILocation(line: 503, column: 9, scope: !1647, inlinedAt: !1654)
!1672 = !DILocation(line: 0, scope: !1630, inlinedAt: !1673)
!1673 = distinct !DILocation(line: 2299, column: 29, scope: !1590, inlinedAt: !1598)
!1674 = !DILocation(line: 0, scope: !1656, inlinedAt: !1675)
!1675 = distinct !DILocation(line: 3543, column: 27, scope: !1676, inlinedAt: !1673)
!1676 = !DILexicalBlockFile(scope: !1630, file: !1591, discriminator: 2)
!1677 = !DILocation(line: 2299, column: 16, scope: !1590, inlinedAt: !1598)
!1678 = !DILocation(line: 2300, column: 17, scope: !1590, inlinedAt: !1598)
!1679 = !DILocation(line: 0, scope: !1172, inlinedAt: !1173)
!1680 = !{!1681}
!1681 = distinct !{!1681, !1682, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hd497d5f9546794adE: %_1"}
!1682 = distinct !{!1682, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hd497d5f9546794adE"}
!1683 = !DILocation(line: 0, scope: !1561, inlinedAt: !1684)
!1684 = distinct !DILocation(line: 39, column: 49, scope: !1167, inlinedAt: !1173)
!1685 = !{!1686}
!1686 = distinct !{!1686, !1687, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17hc39867958365ed55E: %_1"}
!1687 = distinct !{!1687, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17hc39867958365ed55E"}
!1688 = !DILocation(line: 523, column: 1, scope: !1561, inlinedAt: !1684)
!1689 = !DILocation(line: 0, scope: !1576, inlinedAt: !1690)
!1690 = distinct !DILocation(line: 523, column: 1, scope: !1561, inlinedAt: !1684)
!1691 = !{!1692}
!1692 = distinct !{!1692, !1693, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfce35e40ab15106fE: %self"}
!1693 = distinct !{!1693, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfce35e40ab15106fE"}
!1694 = !DILocation(line: 523, column: 1, scope: !1576, inlinedAt: !1690)
!1695 = !DILocation(line: 0, scope: !1590, inlinedAt: !1696)
!1696 = distinct !DILocation(line: 523, column: 1, scope: !1576, inlinedAt: !1690)
!1697 = !DILocation(line: 0, scope: !1600, inlinedAt: !1698)
!1698 = distinct !DILocation(line: 2298, column: 18, scope: !1590, inlinedAt: !1696)
!1699 = !DILocation(line: 0, scope: !1600, inlinedAt: !1700)
!1700 = distinct !DILocation(line: 2299, column: 21, scope: !1590, inlinedAt: !1696)
!1701 = !DILocation(line: 428, column: 20, scope: !1612, inlinedAt: !1702)
!1702 = distinct !DILocation(line: 358, column: 27, scope: !1600, inlinedAt: !1698)
!1703 = !{!1692, !1686, !1681}
!1704 = !DILocation(line: 0, scope: !1620, inlinedAt: !1705)
!1705 = distinct !DILocation(line: 2298, column: 26, scope: !1590, inlinedAt: !1696)
!1706 = !DILocation(line: 0, scope: !1630, inlinedAt: !1707)
!1707 = distinct !DILocation(line: 3571, column: 36, scope: !1620, inlinedAt: !1705)
!1708 = !DILocation(line: 0, scope: !1637, inlinedAt: !1709)
!1709 = distinct !DILocation(line: 3571, column: 27, scope: !1620, inlinedAt: !1705)
!1710 = !DILocation(line: 0, scope: !1647, inlinedAt: !1711)
!1711 = distinct !DILocation(line: 429, column: 14, scope: !1637, inlinedAt: !1709)
!1712 = !DILocation(line: 0, scope: !1656, inlinedAt: !1713)
!1713 = distinct !DILocation(line: 3543, column: 27, scope: !1630, inlinedAt: !1707)
!1714 = !DILocation(line: 544, column: 18, scope: !1656, inlinedAt: !1713)
!1715 = !DILocation(line: 3571, column: 31, scope: !1620, inlinedAt: !1705)
!1716 = !DILocation(line: 865, column: 9, scope: !1666, inlinedAt: !1717)
!1717 = distinct !DILocation(line: 503, column: 9, scope: !1647, inlinedAt: !1711)
!1718 = !DILocation(line: 0, scope: !1630, inlinedAt: !1719)
!1719 = distinct !DILocation(line: 2299, column: 29, scope: !1590, inlinedAt: !1696)
!1720 = !DILocation(line: 0, scope: !1656, inlinedAt: !1721)
!1721 = distinct !DILocation(line: 3543, column: 27, scope: !1676, inlinedAt: !1719)
!1722 = !DILocation(line: 2299, column: 16, scope: !1590, inlinedAt: !1696)
!1723 = !DILocation(line: 2300, column: 17, scope: !1590, inlinedAt: !1696)
!1724 = !DILocation(line: 36, column: 1, scope: !1168, inlinedAt: !1173)
!1725 = !DILocation(line: 41, column: 8, scope: !1172, inlinedAt: !1173)
!1726 = !DILocation(line: 0, scope: !1167, inlinedAt: !1173)
!1727 = !DILocation(line: 49, column: 5, scope: !1172, inlinedAt: !1173)
!1728 = !DILocalVariable(name: "cat", scope: !1729, file: !943, line: 77, type: !1123, align: 64)
!1729 = distinct !DILexicalBlock(scope: !1730, file: !943, line: 77, column: 5)
!1730 = distinct !DISubprogram(name: "static_dp", linkageName: "_ZN5dp_ex9static_dp17hdfe239b3264d42c0E", scope: !33, file: !943, line: 76, type: !21, scopeLine: 76, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !58, templateParams: !23, retainedNodes: !1731)
!1731 = !{!1728}
!1732 = !DILocation(line: 0, scope: !1729, inlinedAt: !1733)
!1733 = distinct !DILocation(line: 97, column: 5, scope: !1165)
!1734 = !DILocation(line: 0, scope: !1119, inlinedAt: !1735)
!1735 = distinct !DILocation(line: 78, column: 5, scope: !1729, inlinedAt: !1733)
!1736 = !DILocation(line: 0, scope: !1128, inlinedAt: !1737)
!1737 = distinct !DILocation(line: 19, column: 9, scope: !1119, inlinedAt: !1735)
!1738 = !DILocation(line: 19, column: 9, scope: !1119, inlinedAt: !1735)
!1739 = !DILocation(line: 602, column: 9, scope: !1128, inlinedAt: !1737)
!1740 = !DILocation(line: 98, column: 2, scope: !1165)
