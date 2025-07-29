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
@__rustc_debug_gdb_scripts_section__ = linkonce_odr unnamed_addr constant [34 x i8] c"\01gdb_load_rust_pretty_printers.py\00", section ".debug_gdb_scripts", align 1

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden noundef i64 @_ZN3std2rt10lang_start17h4be3234073074386E(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 !dbg !79 {
start:
  %_7 = alloca [8 x i8], align 8
    #dbg_value(ptr %main, !87, !DIExpression(), !93)
    #dbg_value(i64 %argc, !88, !DIExpression(), !93)
    #dbg_value(ptr %argv, !89, !DIExpression(), !93)
    #dbg_value(i8 %sigpipe, !90, !DIExpression(), !93)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7), !dbg !94
  store ptr %main, ptr %_7, align 8, !dbg !94
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_ZN3std2rt19lang_start_internal17h179a86731aa5c4aaE(ptr noundef nonnull align 1 %_7, ptr noalias noundef nonnull readonly align 8 dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe), !dbg !95
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7), !dbg !96
  ret i64 %_0, !dbg !97
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h35042c385834d7a7E"(ptr noalias nocapture noundef readonly align 8 dereferenceable(8) %_1) unnamed_addr #1 !dbg !98 {
start:
    #dbg_value(ptr %_1, !104, !DIExpression(DW_OP_deref), !105)
  %_4 = load ptr, ptr %_1, align 8, !dbg !106, !nonnull !23, !noundef !23
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4d6c13392805e6c7E(ptr noundef nonnull %_4), !dbg !107
  ret i32 0, !dbg !108
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4d6c13392805e6c7E(ptr nocapture noundef nonnull readonly %f) unnamed_addr #2 !dbg !109 {
start:
    #dbg_declare(ptr undef, !117, !DIExpression(), !121)
    #dbg_value(ptr %f, !116, !DIExpression(), !122)
    #dbg_declare(ptr undef, !123, !DIExpression(), !130)
    #dbg_value(ptr %f, !132, !DIExpression(), !143)
    #dbg_declare(ptr undef, !139, !DIExpression(), !145)
  tail call void %f(), !dbg !145
  tail call void asm sideeffect "", "~{memory}"() #11, !dbg !146, !srcloc !147
  ret void, !dbg !148
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17ha781119b1797b6ceE"(ptr nocapture noundef readonly %_1) unnamed_addr #1 personality ptr @rust_eh_personality !dbg !149 {
start:
    #dbg_value(ptr %_1, !154, !DIExpression(), !158)
    #dbg_declare(ptr undef, !155, !DIExpression(), !159)
  %0 = load ptr, ptr %_1, align 8, !dbg !159, !nonnull !23, !noundef !23
    #dbg_value(ptr %0, !104, !DIExpression(), !160)
    #dbg_value(ptr %0, !166, !DIExpression(), !169)
    #dbg_declare(ptr undef, !167, !DIExpression(), !170)
    #dbg_value(ptr undef, !104, !DIExpression(DW_OP_deref), !160)
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4d6c13392805e6c7E(ptr noundef nonnull readonly %0), !dbg !171, !noalias !172
  ret i32 0, !dbg !159
}

; rand::rngs::reseeding::ReseedingCore<R,Rsdr>::reseed_and_generate
; Function Attrs: noinline nonlazybind uwtable
define internal fastcc void @"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17ha6228666c915f093E"(ptr noalias noundef nonnull align 16 dereferenceable(64) %self, ptr noalias noundef nonnull align 4 dereferenceable(256) %0) unnamed_addr #2 personality ptr @rust_eh_personality !dbg !175 {
start:
  %_9.i.i = alloca [32 x i8], align 1
  %seed.i.i = alloca [32 x i8], align 1
  %self1.i = alloca [64 x i8], align 16
    #dbg_value(ptr %0, !245, !DIExpression(), !269)
    #dbg_value(ptr %self, !244, !DIExpression(), !269)
    #dbg_value(i64 256, !246, !DIExpression(), !270)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !271), !dbg !274
    #dbg_value(ptr %self, !275, !DIExpression(), !297)
    #dbg_declare(ptr %self1.i, !299, !DIExpression(), !339)
    #dbg_declare(ptr poison, !335, !DIExpression(), !341)
  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %self1.i), !dbg !342, !noalias !271
    #dbg_value(ptr poison, !343, !DIExpression(), !381)
    #dbg_declare(ptr %seed.i.i, !351, !DIExpression(), !383)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %seed.i.i), !dbg !384, !noalias !385
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 1 dereferenceable(32) %seed.i.i, i8 0, i64 32, i1 false), !dbg !388, !alias.scope !398, !noalias !385
    #dbg_value(ptr poison, !401, !DIExpression(), !414)
    #dbg_value(ptr %seed.i.i, !413, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !414)
    #dbg_value(i64 32, !413, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !414)
    #dbg_value(ptr %seed.i.i, !416, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !454)
    #dbg_value(i64 32, !416, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !454)
    #dbg_value(ptr %seed.i.i, !456, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !495)
    #dbg_value(i64 32, !456, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !495)
    #dbg_value(ptr undef, !497, !DIExpression(), !537)
    #dbg_value(ptr undef, !562, !DIExpression(), !572)
    #dbg_value(ptr %seed.i.i, !547, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !574)
    #dbg_value(i64 32, !547, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !574)
    #dbg_value(i8 2, !575, !DIExpression(), !593)
    #dbg_value(ptr @_ZN9getrandom8backends27linux_android_with_fallback12GETRANDOM_FN17hecc8036d0c0d1ff5E, !592, !DIExpression(), !593)
    #dbg_value(ptr @_ZN9getrandom8backends27linux_android_with_fallback12GETRANDOM_FN17hecc8036d0c0d1ff5E, !595, !DIExpression(), !602)
    #dbg_value(i8 2, !601, !DIExpression(), !602)
  %1 = load atomic ptr, ptr @_ZN9getrandom8backends27linux_android_with_fallback12GETRANDOM_FN17hecc8036d0c0d1ff5E acquire, align 8, !dbg !604, !noalias !605
    #dbg_value(ptr %1, !548, !DIExpression(), !612)
    #dbg_value(ptr %1, !613, !DIExpression(), !633)
  %2 = icmp eq ptr %1, null, !dbg !635
  br i1 %2, label %bb7.i.i.i.i.i, label %bb1.i.i.i.i.i, !dbg !635, !prof !636

bb7.i.i.i.i.i:                                    ; preds = %start
; call getrandom::backends::linux_android_with_fallback::init
  %3 = tail call noundef nonnull ptr @_ZN9getrandom8backends27linux_android_with_fallback4init17h04ff7c449f4a6b71E(), !dbg !637, !noalias !605
    #dbg_value(ptr %3, !549, !DIExpression(), !638)
  br label %bb1.i.i.i.i.i, !dbg !637

bb1.i.i.i.i.i:                                    ; preds = %bb7.i.i.i.i.i, %start
  %fptr.sroa.0.0.i.i.i.i.i = phi ptr [ %3, %bb7.i.i.i.i.i ], [ %1, %start ], !dbg !612
    #dbg_value(ptr %fptr.sroa.0.0.i.i.i.i.i, !549, !DIExpression(), !638)
    #dbg_value(ptr undef, !562, !DIExpression(), !572)
    #dbg_value(ptr poison, !571, !DIExpression(), !572)
  %_7.i.i.i.i.i = icmp eq ptr %fptr.sroa.0.0.i.i.i.i.i, inttoptr (i64 -1 to ptr), !dbg !639
  br i1 %_7.i.i.i.i.i, label %"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E.exit.i.i", label %bb3.i.i.i.i.i.i, !dbg !640

bb3.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i.i.i, %bb13.i.i.i.i.i.i
  %buf.sroa.0.038.i.i.i.i.i.i = phi ptr [ %buf.sroa.0.1.i.i.i.i.i.i, %bb13.i.i.i.i.i.i ], [ %seed.i.i, %bb1.i.i.i.i.i ]
  %buf.sroa.5.037.i.i.i.i.i.i = phi i64 [ %buf.sroa.5.1.i.i.i.i.i.i, %bb13.i.i.i.i.i.i ], [ 32, %bb1.i.i.i.i.i ]
    #dbg_value(ptr %buf.sroa.0.038.i.i.i.i.i.i, !516, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !537)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !516, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !537)
    #dbg_value(ptr poison, !641, !DIExpression(DW_OP_deref, DW_OP_deref), !648)
    #dbg_value(ptr %buf.sroa.0.038.i.i.i.i.i.i, !647, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !648)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !647, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !648)
  %_0.i.i.i.i.i.i.i = call noundef i64 %fptr.sroa.0.0.i.i.i.i.i(ptr noundef nonnull align 1 %buf.sroa.0.038.i.i.i.i.i.i, i64 noundef range(i64 1, 0) %buf.sroa.5.037.i.i.i.i.i.i, i32 noundef 0) #11, !dbg !650, !noalias !651
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !517, !DIExpression(), !654)
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !519, !DIExpression(), !655)
    #dbg_value(ptr undef, !521, !DIExpression(), !655)
  %_9.i.i.i.i.i.i = icmp sgt i64 %_0.i.i.i.i.i.i.i, 0, !dbg !656
  br i1 %_9.i.i.i.i.i.i, label %bb19.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i, !dbg !656

bb8.i.i.i.i.i.i:                                  ; preds = %bb3.i.i.i.i.i.i
  %4 = icmp eq i64 %_0.i.i.i.i.i.i.i, -1, !dbg !657
  br i1 %4, label %bb6.i.i.i.i.i.i, label %bb4.i, !dbg !657

bb6.i.i.i.i.i.i:                                  ; preds = %bb8.i.i.i.i.i.i
; call getrandom::backends::use_file::util_libc::last_os_error
  %err.i.i.i.i.i.i = call noundef i32 @_ZN9getrandom8backends8use_file9util_libc13last_os_error17h3226e5d4689f405dE(), !dbg !658, !noalias !651
    #dbg_value(i32 %err.i.i.i.i.i.i, !533, !DIExpression(), !659)
    #dbg_value(i32 %err.i.i.i.i.i.i, !660, !DIExpression(), !680)
    #dbg_value(i32 %err.i.i.i.i.i.i, !678, !DIExpression(), !682)
  %cond = icmp eq i32 %err.i.i.i.i.i.i, -4
  br i1 %cond, label %bb13.i.i.i.i.i.i, label %bb4.i, !dbg !683

bb13.i.i.i.i.i.i:                                 ; preds = %bb6.i.i.i.i.i.i, %bb19.i.i.i.i.i.i
  %buf.sroa.5.1.i.i.i.i.i.i = phi i64 [ %new_len.i.i.i.i.i.i, %bb19.i.i.i.i.i.i ], [ %buf.sroa.5.037.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ]
  %buf.sroa.0.1.i.i.i.i.i.i = phi ptr [ %_40.i.i.i.i.i.i, %bb19.i.i.i.i.i.i ], [ %buf.sroa.0.038.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ]
    #dbg_value(ptr %buf.sroa.0.1.i.i.i.i.i.i, !516, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !537)
    #dbg_value(i64 %buf.sroa.5.1.i.i.i.i.i.i, !516, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !537)
    #dbg_value(i64 poison, !684, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !726)
  %5 = icmp eq i64 %buf.sroa.5.1.i.i.i.i.i.i, 0, !dbg !728
  br i1 %5, label %bb5.i, label %bb3.i.i.i.i.i.i, !dbg !728

bb19.i.i.i.i.i.i:                                 ; preds = %bb3.i.i.i.i.i.i
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !523, !DIExpression(), !729)
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !730, !DIExpression(), !760)
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !762, !DIExpression(), !772)
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !774, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !786)
    #dbg_value(i64 %_0.i.i.i.i.i.i.i, !788, !DIExpression(), !809)
    #dbg_value(ptr %buf.sroa.0.038.i.i.i.i.i.i, !756, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !760)
    #dbg_value(ptr %buf.sroa.0.038.i.i.i.i.i.i, !770, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !772)
    #dbg_value(ptr %buf.sroa.0.038.i.i.i.i.i.i, !784, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !786)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !756, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !760)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !770, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !772)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !784, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !786)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !774, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !786)
    #dbg_value(i64 %buf.sroa.5.037.i.i.i.i.i.i, !808, !DIExpression(), !809)
  %_38.i.i.i.i.i.i = icmp ult i64 %buf.sroa.5.037.i.i.i.i.i.i, %_0.i.i.i.i.i.i.i, !dbg !811
  %new_len.i.i.i.i.i.i = sub nuw i64 %buf.sroa.5.037.i.i.i.i.i.i, %_0.i.i.i.i.i.i.i, !dbg !811
  %_40.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %buf.sroa.0.038.i.i.i.i.i.i, i64 %_0.i.i.i.i.i.i.i, !dbg !811
    #dbg_value(ptr undef, !684, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !726)
    #dbg_value(i64 undef, !684, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !726)
  br i1 %_38.i.i.i.i.i.i, label %bb4.i, label %bb13.i.i.i.i.i.i, !dbg !812

"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E.exit.i.i": ; preds = %bb1.i.i.i.i.i
; call getrandom::backends::linux_android_with_fallback::use_file_fallback
  %6 = call noundef i32 @_ZN9getrandom8backends27linux_android_with_fallback17use_file_fallback17h9211927a188feaf1E(ptr noalias noundef nonnull align 1 %seed.i.i, i64 noundef 32), !dbg !813, !noalias !385
    #dbg_value(i32 %6, !814, !DIExpression(), !839)
  %.not.i.i = icmp eq i32 %6, 0, !dbg !841
  br i1 %.not.i.i, label %bb5.i, label %bb4.i, !dbg !842

bb4.i:                                            ; preds = %bb19.i.i.i.i.i.i, %bb6.i.i.i.i.i.i, %bb8.i.i.i.i.i.i, %"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E.exit.i.i"
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %seed.i.i), !dbg !843, !noalias !385
    #dbg_value(ptr %self, !334, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !844)
    #dbg_value(ptr poison, !845, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !853)
    #dbg_value(ptr %self, !334, !DIExpression(DW_OP_plus_uconst, 48, DW_OP_stack_value, DW_OP_LLVM_fragment, 64, 64), !844)
    #dbg_value(ptr poison, !845, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 64, 64), !853)
    #dbg_value(ptr %self, !851, !DIExpression(DW_OP_plus_uconst, 48, DW_OP_deref, DW_OP_stack_value), !853)
    #dbg_value(ptr %self, !334, !DIExpression(DW_OP_plus_uconst, 56, DW_OP_stack_value, DW_OP_LLVM_fragment, 128, 64), !844)
    #dbg_value(ptr poison, !845, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 128, 64), !853)
    #dbg_value(ptr %self, !852, !DIExpression(DW_OP_plus_uconst, 56, DW_OP_deref, DW_OP_stack_value), !853)
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 48
  %_9.pre = load i64, ptr %.phi.trans.insert, align 16, !dbg !855
  br label %"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E.exit", !dbg !856

bb5.i:                                            ; preds = %bb13.i.i.i.i.i.i, %"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E.exit.i.i"
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9.i.i), !dbg !857, !noalias !385
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(32) %_9.i.i, ptr noundef nonnull align 1 dereferenceable(32) %seed.i.i, i64 32, i1 false), !dbg !857, !noalias !385
    #dbg_declare(ptr %_9.i.i, !858, !DIExpression(), !865)
    #dbg_value(ptr %_9.i.i, !867, !DIExpression(), !880)
    #dbg_value(ptr @alloc_85fc59111fd0cef7ef4093da3840b035, !879, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !880)
    #dbg_value(i64 8, !879, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !880)
  %7 = getelementptr inbounds nuw i8, ptr %self1.i, i64 16, !dbg !882
; call rand_chacha::guts::init_chacha
  call void @_ZN11rand_chacha4guts11init_chacha17he4f07b70577fd00dE(ptr noalias nocapture noundef nonnull sret([48 x i8]) align 16 dereferenceable(48) %7, ptr noalias noundef nonnull readonly align 1 dereferenceable(32) %_9.i.i, ptr noalias noundef nonnull readonly align 1 @alloc_85fc59111fd0cef7ef4093da3840b035, i64 noundef 8), !dbg !883, !noalias !271
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9.i.i), !dbg !884, !noalias !385
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %seed.i.i), !dbg !843, !noalias !385
  %_6.i = getelementptr inbounds nuw i8, ptr %self, i64 48, !dbg !885
    #dbg_value(ptr %self, !334, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !844)
    #dbg_value(ptr poison, !845, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 0, 64), !853)
    #dbg_value(ptr %_6.i, !334, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !844)
    #dbg_value(ptr poison, !845, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 64, 64), !853)
    #dbg_value(ptr %_6.i, !851, !DIExpression(DW_OP_deref), !853)
    #dbg_value(ptr %self, !334, !DIExpression(DW_OP_plus_uconst, 56, DW_OP_stack_value, DW_OP_LLVM_fragment, 128, 64), !844)
    #dbg_value(ptr poison, !845, !DIExpression(DW_OP_deref, DW_OP_LLVM_fragment, 128, 64), !853)
    #dbg_value(ptr %self, !852, !DIExpression(DW_OP_plus_uconst, 56, DW_OP_deref, DW_OP_stack_value), !853)
    #dbg_declare(ptr poison, !850, !DIExpression(), !886)
  %_3.i.i = load i64, ptr %_6.i, align 16, !dbg !887, !alias.scope !271, !noalias !888, !noundef !23
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(64) %self, ptr noundef nonnull align 16 dereferenceable(48) %7, i64 48, i1 false), !dbg !892
  br label %"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E.exit", !dbg !893

"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E.exit": ; preds = %bb4.i, %bb5.i
  %_9 = phi i64 [ %_9.pre, %bb4.i ], [ %_3.i.i, %bb5.i ], !dbg !855
  call void @llvm.lifetime.end.p0(i64 64, ptr nonnull %self1.i), !dbg !894, !noalias !271
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 56, !dbg !895
  %9 = add i64 %_9, -256, !dbg !895
  store i64 %9, ptr %8, align 8, !dbg !895
    #dbg_value(ptr %self, !896, !DIExpression(), !903)
    #dbg_value(ptr %0, !902, !DIExpression(), !903)
    #dbg_value(i32 6, !905, !DIExpression(), !915)
    #dbg_value(ptr %self, !913, !DIExpression(), !915)
    #dbg_value(ptr %0, !914, !DIExpression(), !915)
; call rand_chacha::guts::refill_wide
  call void @_ZN11rand_chacha4guts11refill_wide17h7bc07d422e567fdcE(ptr noalias noundef nonnull align 16 dereferenceable(48) %self, i32 noundef 6, ptr noalias noundef nonnull align 4 dereferenceable(256) %0), !dbg !917
  ret void, !dbg !918
}

; <dp_ex::Dog as dp_ex::Animal>::speak
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17h95a71617b8cd4b02E"(ptr noalias nocapture nonnull readonly align 1 %self) unnamed_addr #0 !dbg !919 {
start:
  %_3 = alloca [48 x i8], align 8
    #dbg_value(ptr poison, !926, !DIExpression(), !927)
    #dbg_value(ptr @alloc_ec4fa215300b117d5ab20e2368000be2, !928, !DIExpression(), !1092)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3), !dbg !1093
  store ptr @alloc_ec4fa215300b117d5ab20e2368000be2, ptr %_3, align 8, !dbg !1094
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 8, !dbg !1094
  store i64 1, ptr %0, align 8, !dbg !1094
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 32, !dbg !1094
  store ptr null, ptr %1, align 8, !dbg !1094
  %2 = getelementptr inbounds nuw i8, ptr %_3, i64 16, !dbg !1094
  store ptr inttoptr (i64 8 to ptr), ptr %2, align 8, !dbg !1094
  %3 = getelementptr inbounds nuw i8, ptr %_3, i64 24, !dbg !1094
  store i64 0, ptr %3, align 8, !dbg !1094
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_3), !dbg !1093
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3), !dbg !1093
  ret void, !dbg !1095
}

; <dp_ex::Cat as dp_ex::Animal>::speak
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17hfb6e878b47c18265E"(ptr noalias nocapture nonnull readonly align 1 %self) unnamed_addr #0 !dbg !1096 {
start:
  %_3 = alloca [48 x i8], align 8
    #dbg_value(ptr poison, !1102, !DIExpression(), !1103)
    #dbg_value(ptr @alloc_000bc512779c9a763a8aa84ee52b6421, !1104, !DIExpression(), !1107)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3), !dbg !1108
  store ptr @alloc_000bc512779c9a763a8aa84ee52b6421, ptr %_3, align 8, !dbg !1109
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 8, !dbg !1109
  store i64 1, ptr %0, align 8, !dbg !1109
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 32, !dbg !1109
  store ptr null, ptr %1, align 8, !dbg !1109
  %2 = getelementptr inbounds nuw i8, ptr %_3, i64 16, !dbg !1109
  store ptr inttoptr (i64 8 to ptr), ptr %2, align 8, !dbg !1109
  %3 = getelementptr inbounds nuw i8, ptr %_3, i64 24, !dbg !1109
  store i64 0, ptr %3, align 8, !dbg !1109
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_3), !dbg !1108
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3), !dbg !1108
  ret void, !dbg !1110
}

; dp_ex::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN5dp_ex4main17hc7990c7b9cee8a83E() unnamed_addr #0 personality ptr @rust_eh_personality !dbg !1111 {
start:
  %_3.i.i1 = alloca [48 x i8], align 8
  %rng.i = alloca [8 x i8], align 8
    #dbg_declare(ptr %rng.i, !1112, !DIExpression(), !1177)
    #dbg_value(ptr inttoptr (i64 1 to ptr), !1179, !DIExpression(), !1194)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %rng.i), !dbg !1208
; call rand::rngs::thread::rng
  %0 = tail call noundef nonnull ptr @_ZN4rand4rngs6thread3rng17h70cca7a3940ce3c4E(), !dbg !1209
  store ptr %0, ptr %rng.i, align 8, !dbg !1209
    #dbg_value(ptr %rng.i, !1203, !DIExpression(), !1210)
    #dbg_value(ptr %rng.i, !1191, !DIExpression(), !1194)
    #dbg_value(ptr %rng.i, !1211, !DIExpression(), !1221)
    #dbg_value(ptr %0, !1223, !DIExpression(DW_OP_plus_uconst, 16, DW_OP_stack_value), !1231)
  %_14.i = getelementptr inbounds nuw i8, ptr %0, i64 16, !dbg !1233
    #dbg_value(ptr %_14.i, !1218, !DIExpression(), !1234)
    #dbg_value(ptr %_14.i, !1235, !DIExpression(), !1242)
    #dbg_value(ptr %_14.i, !1244, !DIExpression(), !1254)
  %1 = getelementptr inbounds nuw i8, ptr %0, i64 336, !dbg !1256
  %_3.i.i = load i64, ptr %1, align 16, !dbg !1256, !alias.scope !1257, !noundef !23
  %_2.i.i = icmp ugt i64 %_3.i.i, 63, !dbg !1256
  br i1 %_2.i.i, label %bb2.i.i, label %bb9.i, !dbg !1256

bb2.i.i:                                          ; preds = %start
    #dbg_value(ptr %_14.i, !1260, !DIExpression(), !1267)
    #dbg_value(i64 0, !1266, !DIExpression(), !1267)
  %_9.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 272, !dbg !1269
    #dbg_value(ptr %_14.i, !1270, !DIExpression(), !1277)
    #dbg_value(ptr %_9.i.i.i, !1274, !DIExpression(), !1277)
  %2 = getelementptr inbounds nuw i8, ptr %0, i64 328, !dbg !1279
  %_4.i.i.i.i = load i64, ptr %2, align 8, !dbg !1279, !alias.scope !1280, !noalias !1285, !noundef !23
  %_3.i.i.i.i = icmp slt i64 %_4.i.i.i.i, 1, !dbg !1279
  br i1 %_3.i.i.i.i, label %bb1.i.i.i.i, label %bb3.i.i.i.i, !dbg !1279

bb3.i.i.i.i:                                      ; preds = %bb2.i.i
    #dbg_value(i64 256, !1275, !DIExpression(), !1287)
  %3 = add nsw i64 %_4.i.i.i.i, -256, !dbg !1288
  store i64 %3, ptr %2, align 8, !dbg !1288, !alias.scope !1280, !noalias !1285
    #dbg_value(ptr %_9.i.i.i, !896, !DIExpression(), !1289)
    #dbg_value(ptr %_14.i, !902, !DIExpression(), !1289)
    #dbg_value(i32 6, !905, !DIExpression(), !1291)
    #dbg_value(ptr %_9.i.i.i, !913, !DIExpression(), !1291)
    #dbg_value(ptr %_14.i, !914, !DIExpression(), !1291)
; invoke rand_chacha::guts::refill_wide
  invoke void @_ZN11rand_chacha4guts11refill_wide17h7bc07d422e567fdcE(ptr noalias noundef nonnull align 16 dereferenceable(64) %_9.i.i.i, i32 noundef 6, ptr noalias noundef nonnull align 16 dereferenceable(336) %_14.i)
          to label %bb9.i unwind label %cleanup.i, !dbg !1293

bb1.i.i.i.i:                                      ; preds = %bb2.i.i
; invoke rand::rngs::reseeding::ReseedingCore<R,Rsdr>::reseed_and_generate
  invoke fastcc void @"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17ha6228666c915f093E"(ptr noalias noundef nonnull align 16 dereferenceable(64) %_9.i.i.i, ptr noalias noundef nonnull align 16 dereferenceable(336) %_14.i)
          to label %bb9.i unwind label %cleanup.i, !dbg !1294

cleanup.i:                                        ; preds = %bb9.i, %bb1.i.i.i.i, %bb3.i.i.i.i
  %4 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1295), !dbg !1298
    #dbg_value(ptr %rng.i, !1299, !DIExpression(), !1308)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1310), !dbg !1313
    #dbg_value(ptr %rng.i, !1314, !DIExpression(), !1322)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1324), !dbg !1327
    #dbg_value(ptr %rng.i, !1328, !DIExpression(), !1336)
    #dbg_value(ptr %rng.i, !1338, !DIExpression(), !1346)
    #dbg_value(ptr %rng.i, !1338, !DIExpression(), !1348)
  %_5.i.i.i.i = load ptr, ptr %rng.i, align 8, !dbg !1350, !alias.scope !1357, !nonnull !23, !noundef !23
    #dbg_value(ptr %_5.i.i.i.i, !1358, !DIExpression(), !1366)
    #dbg_value(ptr %_5.i.i.i.i, !1368, !DIExpression(), !1373)
    #dbg_value(ptr %_5.i.i.i.i, !1375, !DIExpression(), !1383)
    #dbg_value(ptr %_5.i.i.i.i, !1385, !DIExpression(), !1392)
    #dbg_value(ptr %_5.i.i.i.i, !1394, !DIExpression(), !1400)
  %_8.i.i.i.i = load i64, ptr %_5.i.i.i.i, align 8, !dbg !1402, !noalias !1357, !noundef !23
  %val.i.i.i.i = add i64 %_8.i.i.i.i, -1, !dbg !1403
    #dbg_value(i64 %val.i.i.i.i, !1382, !DIExpression(), !1383)
    #dbg_value(i64 %val.i.i.i.i, !1391, !DIExpression(), !1392)
  store i64 %val.i.i.i.i, ptr %_5.i.i.i.i, align 8, !dbg !1404, !noalias !1357
    #dbg_value(ptr %_5.i.i.i.i, !1368, !DIExpression(), !1411)
    #dbg_value(ptr %_5.i.i.i.i, !1394, !DIExpression(), !1413)
  %5 = icmp eq i64 %val.i.i.i.i, 0, !dbg !1416
  br i1 %5, label %bb1.i.i.i5.i, label %bb8.i, !dbg !1416

bb1.i.i.i5.i:                                     ; preds = %cleanup.i
; invoke alloc::rc::Rc<T,A>::drop_slow
  invoke void @"_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h8d92a5ccbffeb99aE"(ptr noalias noundef nonnull align 8 dereferenceable(8) %rng.i)
          to label %bb8.i unwind label %terminate.i, !dbg !1417

bb9.i:                                            ; preds = %bb1.i.i.i.i, %bb3.i.i.i.i, %start
  %_10.i.i = phi i64 [ %_3.i.i, %start ], [ 0, %bb3.i.i.i.i ], [ 0, %bb1.i.i.i.i ], !dbg !1418
  %6 = getelementptr inbounds nuw i32, ptr %_14.i, i64 %_10.i.i, !dbg !1419
  %value.i.i = load i32, ptr %6, align 4, !dbg !1419, !alias.scope !1257, !noundef !23
    #dbg_value(i32 %value.i.i, !1252, !DIExpression(), !1420)
  %7 = add nuw nsw i64 %_10.i.i, 1, !dbg !1421
  store i64 %7, ptr %1, align 16, !dbg !1421, !alias.scope !1257
  %_3.i = icmp slt i32 %value.i.i, 0, !dbg !1422
    #dbg_value(ptr inttoptr (i64 1 to ptr), !1117, !DIExpression(DW_OP_LLVM_fragment, 0, 64), !1423)
    #dbg_value(ptr poison, !1117, !DIExpression(DW_OP_LLVM_fragment, 64, 64), !1423)
  %8 = select i1 %_3.i, ptr @"_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17hfb6e878b47c18265E", ptr @"_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17h95a71617b8cd4b02E", !dbg !1424
  invoke void %8(ptr noundef nonnull align 1 inttoptr (i64 1 to ptr))
          to label %bb5.i unwind label %cleanup.i, !dbg !1424

bb5.i:                                            ; preds = %bb9.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1425), !dbg !1298
    #dbg_value(ptr %rng.i, !1299, !DIExpression(), !1428)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1430), !dbg !1433
    #dbg_value(ptr %rng.i, !1314, !DIExpression(), !1434)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1436), !dbg !1439
    #dbg_value(ptr %rng.i, !1328, !DIExpression(), !1440)
    #dbg_value(ptr %rng.i, !1338, !DIExpression(), !1442)
    #dbg_value(ptr %rng.i, !1338, !DIExpression(), !1444)
  %_5.i.i.i7.i = load ptr, ptr %rng.i, align 8, !dbg !1446, !alias.scope !1448, !nonnull !23, !noundef !23
    #dbg_value(ptr %_5.i.i.i7.i, !1358, !DIExpression(), !1449)
    #dbg_value(ptr %_5.i.i.i7.i, !1368, !DIExpression(), !1451)
    #dbg_value(ptr %_5.i.i.i7.i, !1375, !DIExpression(), !1453)
    #dbg_value(ptr %_5.i.i.i7.i, !1385, !DIExpression(), !1455)
    #dbg_value(ptr %_5.i.i.i7.i, !1394, !DIExpression(), !1457)
  %_8.i.i.i8.i = load i64, ptr %_5.i.i.i7.i, align 8, !dbg !1459, !noalias !1448, !noundef !23
  %val.i.i.i9.i = add i64 %_8.i.i.i8.i, -1, !dbg !1460
    #dbg_value(i64 %val.i.i.i9.i, !1382, !DIExpression(), !1453)
    #dbg_value(i64 %val.i.i.i9.i, !1391, !DIExpression(), !1455)
  store i64 %val.i.i.i9.i, ptr %_5.i.i.i7.i, align 8, !dbg !1461, !noalias !1448
    #dbg_value(ptr %_5.i.i.i7.i, !1368, !DIExpression(), !1463)
    #dbg_value(ptr %_5.i.i.i7.i, !1394, !DIExpression(), !1465)
  %9 = icmp eq i64 %val.i.i.i9.i, 0, !dbg !1467
  br i1 %9, label %bb1.i.i.i10.i, label %_ZN5dp_ex6dyn_dp17h87c4dad2651e3135E.exit, !dbg !1467

bb1.i.i.i10.i:                                    ; preds = %bb5.i
; call alloc::rc::Rc<T,A>::drop_slow
  call void @"_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h8d92a5ccbffeb99aE"(ptr noalias noundef nonnull align 8 dereferenceable(8) %rng.i), !dbg !1468
  br label %_ZN5dp_ex6dyn_dp17h87c4dad2651e3135E.exit, !dbg !1468

terminate.i:                                      ; preds = %bb1.i.i.i5.i
  %10 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h4898e2af8dc1f529E() #12, !dbg !1469
  unreachable, !dbg !1469

bb8.i:                                            ; preds = %bb1.i.i.i5.i, %cleanup.i
  resume { ptr, i32 } %4, !dbg !1469

_ZN5dp_ex6dyn_dp17h87c4dad2651e3135E.exit:        ; preds = %bb5.i, %bb1.i.i.i10.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %rng.i), !dbg !1298
    #dbg_value(ptr inttoptr (i64 1 to ptr), !1470, !DIExpression(), !1474)
    #dbg_value(ptr inttoptr (i64 1 to ptr), !1102, !DIExpression(), !1476)
    #dbg_value(ptr @alloc_000bc512779c9a763a8aa84ee52b6421, !1104, !DIExpression(), !1478)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3.i.i1), !dbg !1480
  store ptr @alloc_000bc512779c9a763a8aa84ee52b6421, ptr %_3.i.i1, align 8, !dbg !1481
  %11 = getelementptr inbounds nuw i8, ptr %_3.i.i1, i64 8, !dbg !1481
  store i64 1, ptr %11, align 8, !dbg !1481
  %12 = getelementptr inbounds nuw i8, ptr %_3.i.i1, i64 32, !dbg !1481
  store ptr null, ptr %12, align 8, !dbg !1481
  %13 = getelementptr inbounds nuw i8, ptr %_3.i.i1, i64 16, !dbg !1481
  store ptr inttoptr (i64 8 to ptr), ptr %13, align 8, !dbg !1481
  %14 = getelementptr inbounds nuw i8, ptr %_3.i.i1, i64 24, !dbg !1481
  store i64 0, ptr %14, align 8, !dbg !1481
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17he3d109100110923bE(ptr noalias nocapture noundef nonnull align 8 dereferenceable(48) %_3.i.i1), !dbg !1480
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3.i.i1), !dbg !1480
  ret void, !dbg !1482
}

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare noundef i64 @_ZN3std2rt19lang_start_internal17h179a86731aa5c4aaE(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; Function Attrs: nounwind nonlazybind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef range(i32 1, 17), i64 noundef, ptr noundef, ptr noundef) unnamed_addr #3

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #4

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
declare noundef nonnull ptr @_ZN9getrandom8backends27linux_android_with_fallback4init17h04ff7c449f4a6b71E() unnamed_addr #5

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
declare void @_ZN4core9panicking16panic_in_cleanup17h4898e2af8dc1f529E() unnamed_addr #6

; Function Attrs: nonlazybind
define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #7 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = load volatile i8, ptr @__rustc_debug_gdb_scripts_section__, align 1
  %3 = sext i32 %0 to i64
    #dbg_value(ptr @_ZN5dp_ex4main17hc7990c7b9cee8a83E, !87, !DIExpression(), !93)
    #dbg_value(i64 %3, !88, !DIExpression(), !93)
    #dbg_value(ptr %1, !89, !DIExpression(), !93)
    #dbg_value(i8 0, !90, !DIExpression(), !93)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i), !dbg !94
  store ptr @_ZN5dp_ex4main17hc7990c7b9cee8a83E, ptr %_7.i, align 8, !dbg !94
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_ZN3std2rt19lang_start_internal17h179a86731aa5c4aaE(ptr noundef nonnull align 1 %_7.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(48) @vtable.0, i64 noundef %3, ptr noundef %1, i8 noundef 0), !dbg !95
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i), !dbg !96
  %4 = trunc i64 %_0.i to i32
  ret i32 %4
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
attributes #3 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #4 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #5 = { cold noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #6 = { cold minsize noinline noreturn nounwind nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #7 = { nonlazybind "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #8 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #9 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #10 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #11 = { nounwind }
attributes #12 = { cold noreturn nounwind }

!llvm.module.flags = !{!24, !25, !26, !27, !28}
!llvm.ident = !{!29}
!llvm.dbg.cu = !{!30}

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
!24 = !{i32 8, !"PIC Level", i32 2}
!25 = !{i32 7, !"PIE Level", i32 2}
!26 = !{i32 2, !"RtLibUseGOT", i32 1}
!27 = !{i32 7, !"Dwarf Version", i32 4}
!28 = !{i32 2, !"Debug Info Version", i32 3}
!29 = !{!"rustc version 1.87.0-nightly (ecade534c 2025-03-14)"}
!30 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !31, producer: "clang LLVM (rustc version 1.87.0-nightly (ecade534c 2025-03-14))", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !32, globals: !59, splitDebugInlining: false, nameTableKind: None)
!31 = !DIFile(filename: "src/main.rs/@/dp_ex.5c91ebfbd02dee49-cgu.0", directory: "/home/np/hack/verifopt/dp-ex")
!32 = !{!33, !40, !49, !57}
!33 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "c_void", scope: !34, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagEnumClass, elements: !37)
!34 = !DINamespace(name: "ffi", scope: !35)
!35 = !DINamespace(name: "core", scope: null)
!36 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!37 = !{!38, !39}
!38 = !DIEnumerator(name: "__variant1", value: 0, isUnsigned: true)
!39 = !DIEnumerator(name: "__variant2", value: 1, isUnsigned: true)
!40 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Ordering", scope: !41, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagEnumClass, elements: !43)
!41 = !DINamespace(name: "atomic", scope: !42)
!42 = !DINamespace(name: "sync", scope: !35)
!43 = !{!44, !45, !46, !47, !48}
!44 = !DIEnumerator(name: "Relaxed", value: 0, isUnsigned: true)
!45 = !DIEnumerator(name: "Release", value: 1, isUnsigned: true)
!46 = !DIEnumerator(name: "Acquire", value: 2, isUnsigned: true)
!47 = !DIEnumerator(name: "AcqRel", value: 3, isUnsigned: true)
!48 = !DIEnumerator(name: "SeqCst", value: 4, isUnsigned: true)
!49 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Alignment", scope: !50, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagEnumClass, elements: !52)
!50 = !DINamespace(name: "rt", scope: !51)
!51 = !DINamespace(name: "fmt", scope: !35)
!52 = !{!53, !54, !55, !56}
!53 = !DIEnumerator(name: "Left", value: 0, isUnsigned: true)
!54 = !DIEnumerator(name: "Right", value: 1, isUnsigned: true)
!55 = !DIEnumerator(name: "Center", value: 2, isUnsigned: true)
!56 = !DIEnumerator(name: "Unknown", value: 3, isUnsigned: true)
!57 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Alignment", scope: !51, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagEnumClass, elements: !58)
!58 = !{!53, !54, !55}
!59 = !{!0, !60, !70}
!60 = !DIGlobalVariableExpression(var: !61, expr: !DIExpression())
!61 = distinct !DIGlobalVariable(name: "<dp_ex::Dog as dp_ex::Animal>::{vtable}", scope: null, file: !2, type: !62, isLocal: true, isDefinition: true)
!62 = !DICompositeType(tag: DW_TAG_structure_type, name: "<dp_ex::Dog as dp_ex::Animal>::{vtable_type}", file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !63, vtableHolder: !68, templateParams: !23, identifier: "7b8c77dee4800bd473b9436f1e39b6b9")
!63 = !{!64, !65, !66, !67}
!64 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !62, file: !2, baseType: !6, size: 64, align: 64)
!65 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !62, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!66 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !62, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!67 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !62, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!68 = !DICompositeType(tag: DW_TAG_structure_type, name: "Dog", scope: !69, file: !2, align: 8, flags: DIFlagPrivate, elements: !23, identifier: "6c214b66f1636284dbf33c78927b13b5")
!69 = !DINamespace(name: "dp_ex", scope: null)
!70 = !DIGlobalVariableExpression(var: !71, expr: !DIExpression())
!71 = distinct !DIGlobalVariable(name: "<dp_ex::Cat as dp_ex::Animal>::{vtable}", scope: null, file: !2, type: !72, isLocal: true, isDefinition: true)
!72 = !DICompositeType(tag: DW_TAG_structure_type, name: "<dp_ex::Cat as dp_ex::Animal>::{vtable_type}", file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !73, vtableHolder: !78, templateParams: !23, identifier: "8aed5e37cb0c0bc2cc51d7d91c393e70")
!73 = !{!74, !75, !76, !77}
!74 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !72, file: !2, baseType: !6, size: 64, align: 64)
!75 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !72, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!76 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !72, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!77 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !72, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!78 = !DICompositeType(tag: DW_TAG_structure_type, name: "Cat", scope: !69, file: !2, align: 8, flags: DIFlagPrivate, elements: !23, identifier: "12efbea2fcd1686f8a149626cc63f3a3")
!79 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_ZN3std2rt10lang_start17h4be3234073074386E", scope: !16, file: !80, line: 192, type: !81, scopeLine: 192, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !91, retainedNodes: !86)
!80 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/std/src/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "5ed61ab28987f8860d5842313c6741b3")
!81 = !DISubroutineType(types: !82)
!82 = !{!83, !20, !83, !84, !36}
!83 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!84 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !85, size: 64, align: 64, dwarfAddressSpace: 0)
!85 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !36, size: 64, align: 64, dwarfAddressSpace: 0)
!86 = !{!87, !88, !89, !90}
!87 = !DILocalVariable(name: "main", arg: 1, scope: !79, file: !80, line: 193, type: !20)
!88 = !DILocalVariable(name: "argc", arg: 2, scope: !79, file: !80, line: 194, type: !83)
!89 = !DILocalVariable(name: "argv", arg: 3, scope: !79, file: !80, line: 195, type: !84)
!90 = !DILocalVariable(name: "sigpipe", arg: 4, scope: !79, file: !80, line: 196, type: !36)
!91 = !{!92}
!92 = !DITemplateTypeParameter(name: "T", type: !7)
!93 = !DILocation(line: 0, scope: !79)
!94 = !DILocation(line: 199, column: 10, scope: !79)
!95 = !DILocation(line: 198, column: 5, scope: !79)
!96 = !DILocation(line: 203, column: 5, scope: !79)
!97 = !DILocation(line: 204, column: 2, scope: !79)
!98 = distinct !DISubprogram(name: "{closure#0}<()>", linkageName: "_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h35042c385834d7a7E", scope: !15, file: !80, line: 199, type: !99, scopeLine: 199, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !91, retainedNodes: !103)
!99 = !DISubroutineType(types: !100)
!100 = !{!101, !102}
!101 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!102 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!103 = !{!104}
!104 = !DILocalVariable(name: "main", scope: !98, file: !80, line: 193, type: !20, align: 64)
!105 = !DILocation(line: 0, scope: !98)
!106 = !DILocation(line: 199, column: 70, scope: !98)
!107 = !DILocation(line: 199, column: 18, scope: !98)
!108 = !DILocation(line: 199, column: 93, scope: !98)
!109 = distinct !DISubprogram(name: "__rust_begin_short_backtrace<fn(), ()>", linkageName: "_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4d6c13392805e6c7E", scope: !111, file: !110, line: 148, type: !113, scopeLine: 148, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !119, retainedNodes: !115)
!110 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/std/src/sys/backtrace.rs", directory: "", checksumkind: CSK_MD5, checksum: "9e30c70624c3cf40238860e740bd696f")
!111 = !DINamespace(name: "backtrace", scope: !112)
!112 = !DINamespace(name: "sys", scope: !17)
!113 = !DISubroutineType(types: !114)
!114 = !{null, !20}
!115 = !{!116, !117}
!116 = !DILocalVariable(name: "f", arg: 1, scope: !109, file: !110, line: 148, type: !20)
!117 = !DILocalVariable(name: "result", scope: !118, file: !110, line: 152, type: !7, align: 8)
!118 = distinct !DILexicalBlock(scope: !109, file: !110, line: 152, column: 5)
!119 = !{!120, !92}
!120 = !DITemplateTypeParameter(name: "F", type: !20)
!121 = !DILocation(line: 152, column: 9, scope: !118)
!122 = !DILocation(line: 0, scope: !109)
!123 = !DILocalVariable(name: "dummy", scope: !124, file: !125, line: 476, type: !7, align: 8)
!124 = distinct !DISubprogram(name: "black_box<()>", linkageName: "_ZN4core4hint9black_box17hd82d6438fa6b0ff7E", scope: !126, file: !125, line: 476, type: !127, scopeLine: 476, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !91, retainedNodes: !129)
!125 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/hint.rs", directory: "", checksumkind: CSK_MD5, checksum: "53e417654697d2a6fdb3b165cec3a4bf")
!126 = !DINamespace(name: "hint", scope: !35)
!127 = !DISubroutineType(types: !128)
!128 = !{null, !7}
!129 = !{!123}
!130 = !DILocation(line: 476, column: 27, scope: !124, inlinedAt: !131)
!131 = !DILocation(line: 155, column: 5, scope: !118)
!132 = !DILocalVariable(arg: 1, scope: !133, file: !134, line: 250, type: !20)
!133 = distinct !DISubprogram(name: "call_once<fn(), ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17h63a06be7eb859f30E", scope: !135, file: !134, line: 250, type: !113, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !140, retainedNodes: !138)
!134 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/ops/function.rs", directory: "", checksumkind: CSK_MD5, checksum: "27f40bbdeb6cc525c0d0d7cf434d92c4")
!135 = !DINamespace(name: "FnOnce", scope: !136)
!136 = !DINamespace(name: "function", scope: !137)
!137 = !DINamespace(name: "ops", scope: !35)
!138 = !{!132, !139}
!139 = !DILocalVariable(arg: 2, scope: !133, file: !134, line: 250, type: !7)
!140 = !{!141, !142}
!141 = !DITemplateTypeParameter(name: "Self", type: !20)
!142 = !DITemplateTypeParameter(name: "Args", type: !7)
!143 = !DILocation(line: 0, scope: !133, inlinedAt: !144)
!144 = distinct !DILocation(line: 152, column: 18, scope: !109)
!145 = !DILocation(line: 250, column: 5, scope: !133, inlinedAt: !144)
!146 = !DILocation(line: 477, column: 5, scope: !124, inlinedAt: !131)
!147 = !{i64 16795246276619021}
!148 = !DILocation(line: 158, column: 2, scope: !109)
!149 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17ha781119b1797b6ceE", scope: !135, file: !134, line: 250, type: !150, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !156, retainedNodes: !153)
!150 = !DISubroutineType(types: !151)
!151 = !{!101, !152}
!152 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!153 = !{!154, !155}
!154 = !DILocalVariable(arg: 1, scope: !149, file: !134, line: 250, type: !152)
!155 = !DILocalVariable(arg: 2, scope: !149, file: !134, line: 250, type: !7)
!156 = !{!157, !142}
!157 = !DITemplateTypeParameter(name: "Self", type: !14)
!158 = !DILocation(line: 0, scope: !149)
!159 = !DILocation(line: 250, column: 5, scope: !149)
!160 = !DILocation(line: 0, scope: !98, inlinedAt: !161)
!161 = distinct !DILocation(line: 250, column: 5, scope: !162, inlinedAt: !168)
!162 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17h2ea33e7f40dac79eE", scope: !135, file: !134, line: 250, type: !163, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !156, retainedNodes: !165)
!163 = !DISubroutineType(types: !164)
!164 = !{!101, !14}
!165 = !{!166, !167}
!166 = !DILocalVariable(arg: 1, scope: !162, file: !134, line: 250, type: !14)
!167 = !DILocalVariable(arg: 2, scope: !162, file: !134, line: 250, type: !7)
!168 = distinct !DILocation(line: 250, column: 5, scope: !149)
!169 = !DILocation(line: 0, scope: !162, inlinedAt: !168)
!170 = !DILocation(line: 250, column: 5, scope: !162, inlinedAt: !168)
!171 = !DILocation(line: 199, column: 18, scope: !98, inlinedAt: !161)
!172 = !{!173}
!173 = distinct !{!173, !174, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h35042c385834d7a7E: %_1"}
!174 = distinct !{!174, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h35042c385834d7a7E"}
!175 = distinct !DISubprogram(name: "reseed_and_generate<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17ha6228666c915f093E", scope: !177, file: !176, line: 216, type: !230, scopeLine: 216, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !227, declaration: !242, retainedNodes: !243)
!176 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/rngs/reseeding.rs", directory: "", checksumkind: CSK_MD5, checksum: "b59ac7f2685a0e488478c11615cc2565")
!177 = !DICompositeType(tag: DW_TAG_structure_type, name: "ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", scope: !178, file: !2, size: 512, align: 128, flags: DIFlagPrivate, elements: !181, templateParams: !227, identifier: "592b3932b2f8878cd5d3f17e71dddcb2")
!178 = !DINamespace(name: "reseeding", scope: !179)
!179 = !DINamespace(name: "rngs", scope: !180)
!180 = !DINamespace(name: "rand", scope: null)
!181 = !{!182, !221, !225, !226}
!182 = !DIDerivedType(tag: DW_TAG_member, name: "inner", scope: !177, file: !2, baseType: !183, size: 384, align: 128, flags: DIFlagPrivate)
!183 = !DICompositeType(tag: DW_TAG_structure_type, name: "ChaCha12Core", scope: !184, file: !2, size: 384, align: 128, flags: DIFlagPublic, elements: !186, templateParams: !23, identifier: "2a2910802ded106bd4b70f79d2f9b222")
!184 = !DINamespace(name: "chacha", scope: !185)
!185 = !DINamespace(name: "rand_chacha", scope: null)
!186 = !{!187}
!187 = !DIDerivedType(tag: DW_TAG_member, name: "state", scope: !183, file: !2, baseType: !188, size: 384, align: 128, flags: DIFlagPrivate)
!188 = !DICompositeType(tag: DW_TAG_structure_type, name: "ChaCha", scope: !189, file: !2, size: 384, align: 128, flags: DIFlagPublic, elements: !190, templateParams: !23, identifier: "f0798a57f4b61d4970b465b77f0a9e05")
!189 = !DINamespace(name: "guts", scope: !185)
!190 = !{!191, !219, !220}
!191 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !188, file: !2, baseType: !192, size: 128, align: 128, flags: DIFlagProtected)
!192 = !DICompositeType(tag: DW_TAG_union_type, name: "vec128_storage", scope: !193, file: !2, size: 128, align: 128, elements: !195, templateParams: !23, identifier: "f389b8a1c6d54643d04728b3b45ccbef")
!193 = !DINamespace(name: "x86_64", scope: !194)
!194 = !DINamespace(name: "ppv_lite86", scope: null)
!195 = !{!196, !201, !206, !211}
!196 = !DIDerivedType(tag: DW_TAG_member, name: "u32x4", scope: !192, file: !2, baseType: !197, size: 128, align: 32)
!197 = !DICompositeType(tag: DW_TAG_array_type, baseType: !198, size: 128, align: 32, elements: !199)
!198 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!199 = !{!200}
!200 = !DISubrange(count: 4, lowerBound: 0)
!201 = !DIDerivedType(tag: DW_TAG_member, name: "u64x2", scope: !192, file: !2, baseType: !202, size: 128, align: 64)
!202 = !DICompositeType(tag: DW_TAG_array_type, baseType: !203, size: 128, align: 64, elements: !204)
!203 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!204 = !{!205}
!205 = !DISubrange(count: 2, lowerBound: 0)
!206 = !DIDerivedType(tag: DW_TAG_member, name: "u128x1", scope: !192, file: !2, baseType: !207, size: 128, align: 128)
!207 = !DICompositeType(tag: DW_TAG_array_type, baseType: !208, size: 128, align: 128, elements: !209)
!208 = !DIBasicType(name: "u128", size: 128, encoding: DW_ATE_unsigned)
!209 = !{!210}
!210 = !DISubrange(count: 1, lowerBound: 0)
!211 = !DIDerivedType(tag: DW_TAG_member, name: "sse2", scope: !192, file: !2, baseType: !212, size: 128, align: 128)
!212 = !DICompositeType(tag: DW_TAG_structure_type, name: "__m128i", scope: !213, file: !2, size: 128, align: 128, flags: DIFlagPublic, elements: !215, templateParams: !23, identifier: "ab2da2cf8a57b425a31dcc9ff8b98d2e")
!213 = !DINamespace(name: "x86", scope: !214)
!214 = !DINamespace(name: "core_arch", scope: !35)
!215 = !{!216}
!216 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !212, file: !2, baseType: !217, size: 128, align: 64, flags: DIFlagPrivate)
!217 = !DICompositeType(tag: DW_TAG_array_type, baseType: !218, size: 128, align: 64, elements: !204)
!218 = !DIBasicType(name: "i64", size: 64, encoding: DW_ATE_signed)
!219 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !188, file: !2, baseType: !192, size: 128, align: 128, offset: 128, flags: DIFlagProtected)
!220 = !DIDerivedType(tag: DW_TAG_member, name: "d", scope: !188, file: !2, baseType: !192, size: 128, align: 128, offset: 256, flags: DIFlagProtected)
!221 = !DIDerivedType(tag: DW_TAG_member, name: "reseeder", scope: !177, file: !2, baseType: !222, align: 8, offset: 512, flags: DIFlagPrivate)
!222 = !DICompositeType(tag: DW_TAG_structure_type, name: "OsRng", scope: !223, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "abe41dd699169f8d4175b6cf380563a2")
!223 = !DINamespace(name: "os", scope: !224)
!224 = !DINamespace(name: "rand_core", scope: null)
!225 = !DIDerivedType(tag: DW_TAG_member, name: "threshold", scope: !177, file: !2, baseType: !218, size: 64, align: 64, offset: 384, flags: DIFlagPrivate)
!226 = !DIDerivedType(tag: DW_TAG_member, name: "bytes_until_reseed", scope: !177, file: !2, baseType: !218, size: 64, align: 64, offset: 448, flags: DIFlagPrivate)
!227 = !{!228, !229}
!228 = !DITemplateTypeParameter(name: "R", type: !183)
!229 = !DITemplateTypeParameter(name: "Rsdr", type: !222)
!230 = !DISubroutineType(types: !231)
!231 = !{null, !232, !233}
!232 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", baseType: !177, size: 64, align: 64, dwarfAddressSpace: 0)
!233 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand_chacha::chacha::Array64<u32>", baseType: !234, size: 64, align: 64, dwarfAddressSpace: 0)
!234 = !DICompositeType(tag: DW_TAG_structure_type, name: "Array64<u32>", scope: !184, file: !2, size: 2048, align: 32, flags: DIFlagPublic, elements: !235, templateParams: !240, identifier: "426977da845494fc1dbde4983524ce31")
!235 = !{!236}
!236 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !234, file: !2, baseType: !237, size: 2048, align: 32, flags: DIFlagPrivate)
!237 = !DICompositeType(tag: DW_TAG_array_type, baseType: !198, size: 2048, align: 32, elements: !238)
!238 = !{!239}
!239 = !DISubrange(count: 64, lowerBound: 0)
!240 = !{!241}
!241 = !DITemplateTypeParameter(name: "T", type: !198)
!242 = !DISubprogram(name: "reseed_and_generate<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17ha6228666c915f093E", scope: !177, file: !176, line: 216, type: !230, scopeLine: 216, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !227)
!243 = !{!244, !245, !246, !248}
!244 = !DILocalVariable(name: "self", arg: 1, scope: !175, file: !176, line: 216, type: !232)
!245 = !DILocalVariable(name: "results", arg: 2, scope: !175, file: !176, line: 216, type: !233)
!246 = !DILocalVariable(name: "num_bytes", scope: !247, file: !176, line: 219, type: !9, align: 64)
!247 = distinct !DILexicalBlock(scope: !175, file: !176, line: 219, column: 9)
!248 = !DILocalVariable(name: "e", scope: !249, file: !176, line: 221, type: !250, align: 32)
!249 = distinct !DILexicalBlock(scope: !247, file: !176, line: 221, column: 39)
!250 = !DICompositeType(tag: DW_TAG_structure_type, name: "OsError", scope: !223, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !251, templateParams: !23, identifier: "4d5a6fcb535b7bb557d8bbb8131dad70")
!251 = !{!252}
!252 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !250, file: !2, baseType: !253, size: 32, align: 32, flags: DIFlagPrivate)
!253 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !254, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !256, templateParams: !23, identifier: "bc3dadadf2389ed099352a9bc0641d16")
!254 = !DINamespace(name: "error", scope: !255)
!255 = !DINamespace(name: "getrandom", scope: null)
!256 = !{!257}
!257 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !253, file: !2, baseType: !258, size: 32, align: 32, flags: DIFlagPrivate)
!258 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonZero<i32>", scope: !259, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !261, templateParams: !267, identifier: "32be11596a496ede934ffbdd9b95b3e8")
!259 = !DINamespace(name: "nonzero", scope: !260)
!260 = !DINamespace(name: "num", scope: !35)
!261 = !{!262}
!262 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !258, file: !2, baseType: !263, size: 32, align: 32, flags: DIFlagPrivate)
!263 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonZeroI32Inner", scope: !264, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !265, templateParams: !23, identifier: "7498a545642b9ccf8476c6ffe07e1717")
!264 = !DINamespace(name: "niche_types", scope: !260)
!265 = !{!266}
!266 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !263, file: !2, baseType: !101, size: 32, align: 32, flags: DIFlagPrivate)
!267 = !{!268}
!268 = !DITemplateTypeParameter(name: "T", type: !101)
!269 = !DILocation(line: 0, scope: !175)
!270 = !DILocation(line: 0, scope: !247)
!271 = !{!272}
!272 = distinct !{!272, !273, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E: %self"}
!273 = distinct !{!273, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E"}
!274 = !DILocation(line: 221, column: 25, scope: !249)
!275 = !DILocalVariable(name: "self", arg: 1, scope: !276, file: !176, line: 208, type: !232)
!276 = distinct !DISubprogram(name: "reseed<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E", scope: !177, file: !176, line: 208, type: !277, scopeLine: 208, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !227, declaration: !295, retainedNodes: !296)
!277 = !DISubroutineType(types: !278)
!278 = !{!279, !232}
!279 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), rand_core::os::OsError>", scope: !280, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !281, templateParams: !23, identifier: "ee302a6896271acd50a47cf25c1c8f76")
!280 = !DINamespace(name: "result", scope: !35)
!281 = !{!282}
!282 = !DICompositeType(tag: DW_TAG_variant_part, scope: !279, file: !2, size: 32, align: 32, elements: !283, templateParams: !23, identifier: "1cdf423128631b3680c1e059f5770637", discriminator: !294)
!283 = !{!284, !290}
!284 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !282, file: !2, baseType: !285, size: 32, align: 32, extraData: i32 0)
!285 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !279, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !286, templateParams: !288, identifier: "14c5d45082ecd4eeb838afc989facba8")
!286 = !{!287}
!287 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !285, file: !2, baseType: !7, align: 8, flags: DIFlagPublic)
!288 = !{!92, !289}
!289 = !DITemplateTypeParameter(name: "E", type: !250)
!290 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !282, file: !2, baseType: !291, size: 32, align: 32)
!291 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !279, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !292, templateParams: !288, identifier: "f383eed7d45070a1d992c59d2e554b28")
!292 = !{!293}
!293 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !291, file: !2, baseType: !250, size: 32, align: 32, flags: DIFlagPublic)
!294 = !DIDerivedType(tag: DW_TAG_member, scope: !279, file: !2, baseType: !198, size: 32, align: 32, flags: DIFlagArtificial)
!295 = !DISubprogram(name: "reseed<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h5a6427a801b62794E", scope: !177, file: !176, line: 208, type: !277, scopeLine: 208, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !227)
!296 = !{!275}
!297 = !DILocation(line: 0, scope: !276, inlinedAt: !298)
!298 = distinct !DILocation(line: 221, column: 25, scope: !249)
!299 = !DILocalVariable(name: "self", arg: 1, scope: !300, file: !301, line: 771, type: !302)
!300 = distinct !DISubprogram(name: "map<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsError, (), rand::rngs::reseeding::{impl#5}::reseed::{closure_env#0}<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN4core6result19Result$LT$T$C$E$GT$3map17h4156b7560f7a9c10E", scope: !302, file: !301, line: 771, type: !317, scopeLine: 771, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !330, declaration: !329, retainedNodes: !333)
!301 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/result.rs", directory: "", checksumkind: CSK_MD5, checksum: "b616a11c95aa850b4e5fe6b50aa03751")
!302 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsError>", scope: !280, file: !2, size: 512, align: 128, flags: DIFlagPublic, elements: !303, templateParams: !23, identifier: "5a68524507abe77b21abce2d59cb79f3")
!303 = !{!304}
!304 = !DICompositeType(tag: DW_TAG_variant_part, scope: !302, file: !2, size: 512, align: 128, elements: !305, templateParams: !23, identifier: "ae53ef526a0aa49668600a3ee4b26de5", discriminator: !316)
!305 = !{!306, !312}
!306 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !304, file: !2, baseType: !307, size: 512, align: 128, extraData: i32 0)
!307 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !302, file: !2, size: 512, align: 128, flags: DIFlagPublic, elements: !308, templateParams: !310, identifier: "c22ba6c20c5e86669f5a11b08ae418f")
!308 = !{!309}
!309 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !307, file: !2, baseType: !183, size: 384, align: 128, offset: 128, flags: DIFlagPublic)
!310 = !{!311, !289}
!311 = !DITemplateTypeParameter(name: "T", type: !183)
!312 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !304, file: !2, baseType: !313, size: 512, align: 128, extraData: i32 1)
!313 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !302, file: !2, size: 512, align: 128, flags: DIFlagPublic, elements: !314, templateParams: !310, identifier: "751e6797583144ab3d176d63bf35e4b9")
!314 = !{!315}
!315 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !313, file: !2, baseType: !250, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!316 = !DIDerivedType(tag: DW_TAG_member, scope: !302, file: !2, baseType: !198, size: 32, align: 32, flags: DIFlagArtificial)
!317 = !DISubroutineType(types: !318)
!318 = !{!279, !302, !319}
!319 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", scope: !320, file: !2, size: 192, align: 64, elements: !322, templateParams: !23, identifier: "d2c5aa1ace3eff00c74842752244a6ab")
!320 = !DINamespace(name: "reseed", scope: !321)
!321 = !DINamespace(name: "{impl#5}", scope: !178)
!322 = !{!323, !325, !327}
!323 = !DIDerivedType(tag: DW_TAG_member, name: "_ref__self__inner", scope: !319, file: !2, baseType: !324, size: 64, align: 64)
!324 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand_chacha::chacha::ChaCha12Core", baseType: !183, size: 64, align: 64, dwarfAddressSpace: 0)
!325 = !DIDerivedType(tag: DW_TAG_member, name: "_ref__self__threshold", scope: !319, file: !2, baseType: !326, size: 64, align: 64, offset: 64)
!326 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&i64", baseType: !218, size: 64, align: 64, dwarfAddressSpace: 0)
!327 = !DIDerivedType(tag: DW_TAG_member, name: "_ref__self__bytes_until_reseed", scope: !319, file: !2, baseType: !328, size: 64, align: 64, offset: 128)
!328 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut i64", baseType: !218, size: 64, align: 64, dwarfAddressSpace: 0)
!329 = !DISubprogram(name: "map<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsError, (), rand::rngs::reseeding::{impl#5}::reseed::{closure_env#0}<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN4core6result19Result$LT$T$C$E$GT$3map17h4156b7560f7a9c10E", scope: !302, file: !301, line: 771, type: !317, scopeLine: 771, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !330)
!330 = !{!311, !289, !331, !332}
!331 = !DITemplateTypeParameter(name: "U", type: !7)
!332 = !DITemplateTypeParameter(name: "F", type: !319)
!333 = !{!299, !334, !335, !337}
!334 = !DILocalVariable(name: "op", arg: 2, scope: !300, file: !301, line: 771, type: !319)
!335 = !DILocalVariable(name: "t", scope: !336, file: !301, line: 773, type: !183, align: 128)
!336 = distinct !DILexicalBlock(scope: !300, file: !301, line: 773, column: 13)
!337 = !DILocalVariable(name: "e", scope: !338, file: !301, line: 774, type: !250, align: 32)
!338 = distinct !DILexicalBlock(scope: !300, file: !301, line: 774, column: 13)
!339 = !DILocation(line: 771, column: 38, scope: !300, inlinedAt: !340)
!340 = distinct !DILocation(line: 209, column: 45, scope: !276, inlinedAt: !298)
!341 = !DILocation(line: 773, column: 16, scope: !336, inlinedAt: !340)
!342 = !DILocation(line: 209, column: 9, scope: !276, inlinedAt: !298)
!343 = !DILocalVariable(name: "rng", arg: 1, scope: !344, file: !345, line: 530, type: !349)
!344 = distinct !DISubprogram(name: "try_from_rng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN9rand_core11SeedableRng12try_from_rng17h529124245fe1aebcE", scope: !346, file: !345, line: 530, type: !347, scopeLine: 530, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !378, retainedNodes: !350)
!345 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_core-0.9.3/src/lib.rs", directory: "", checksumkind: CSK_MD5, checksum: "b6e5d656dad488a66c257847601969aa")
!346 = !DINamespace(name: "SeedableRng", scope: !224)
!347 = !DISubroutineType(types: !348)
!348 = !{!302, !349}
!349 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand_core::os::OsRng", baseType: !222, size: 64, align: 64, dwarfAddressSpace: 0)
!350 = !{!343, !351, !356, !376}
!351 = !DILocalVariable(name: "seed", scope: !352, file: !345, line: 531, type: !353, align: 8)
!352 = distinct !DILexicalBlock(scope: !344, file: !345, line: 531, column: 9)
!353 = !DICompositeType(tag: DW_TAG_array_type, baseType: !36, size: 256, align: 8, elements: !354)
!354 = !{!355}
!355 = !DISubrange(count: 32, lowerBound: 0)
!356 = !DILocalVariable(name: "residual", scope: !357, file: !345, line: 532, type: !358, align: 32)
!357 = distinct !DILexicalBlock(scope: !352, file: !345, line: 532, column: 42)
!358 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<core::convert::Infallible, rand_core::os::OsError>", scope: !280, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !359, templateParams: !23, identifier: "48a4bc2942fe3ff0b0ef592339a5854f")
!359 = !{!360}
!360 = !DICompositeType(tag: DW_TAG_variant_part, scope: !358, file: !2, size: 32, align: 32, elements: !361, templateParams: !23, identifier: "2d3be197a9eacbf9b36cebc5831dce97")
!361 = !{!362, !372}
!362 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !360, file: !2, baseType: !363, size: 32, align: 32)
!363 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !358, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !364, templateParams: !370, identifier: "ab9232abdde0e58b5f574c66d8cfafdb")
!364 = !{!365}
!365 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !363, file: !2, baseType: !366, align: 8, flags: DIFlagPublic)
!366 = !DICompositeType(tag: DW_TAG_structure_type, name: "Infallible", scope: !367, file: !2, align: 8, flags: DIFlagPublic, elements: !368, templateParams: !23, identifier: "e51c7217cff47e0428772b48782f2c6e")
!367 = !DINamespace(name: "convert", scope: !35)
!368 = !{!369}
!369 = !DICompositeType(tag: DW_TAG_variant_part, scope: !366, file: !2, align: 8, elements: !23, identifier: "578ba6599628555a2e7430e64d95f8d9")
!370 = !{!371, !289}
!371 = !DITemplateTypeParameter(name: "T", type: !366)
!372 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !360, file: !2, baseType: !373, size: 32, align: 32)
!373 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !358, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !374, templateParams: !370, identifier: "dc8c092dd34505ff40d3a1d90c4495d4")
!374 = !{!375}
!375 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !373, file: !2, baseType: !250, size: 32, align: 32, flags: DIFlagPublic)
!376 = !DILocalVariable(name: "val", scope: !377, file: !345, line: 532, type: !7, align: 8)
!377 = distinct !DILexicalBlock(scope: !352, file: !345, line: 532, column: 9)
!378 = !{!379, !380}
!379 = !DITemplateTypeParameter(name: "Self", type: !183)
!380 = !DITemplateTypeParameter(name: "R", type: !222)
!381 = !DILocation(line: 0, scope: !344, inlinedAt: !382)
!382 = distinct !DILocation(line: 209, column: 9, scope: !276, inlinedAt: !298)
!383 = !DILocation(line: 531, column: 13, scope: !352, inlinedAt: !382)
!384 = !DILocation(line: 531, column: 13, scope: !344, inlinedAt: !382)
!385 = !{!386, !272}
!386 = distinct !{!386, !387, !"_ZN9rand_core11SeedableRng12try_from_rng17h529124245fe1aebcE: %_0"}
!387 = distinct !{!387, !"_ZN9rand_core11SeedableRng12try_from_rng17h529124245fe1aebcE"}
!388 = !DILocation(line: 456, column: 17, scope: !389, inlinedAt: !397)
!389 = distinct !DISubprogram(name: "default<u8>", linkageName: "_ZN4core5array76_$LT$impl$u20$core..default..Default$u20$for$u20$$u5b$T$u3b$$u20$32$u5d$$GT$7default17h66f1ccdc17cac30eE", scope: !391, file: !390, line: 455, type: !393, scopeLine: 455, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !395)
!390 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/array/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "a5d4988c8e4a2c0226c27eed7d0d1450")
!391 = !DINamespace(name: "{impl#29}", scope: !392)
!392 = !DINamespace(name: "array", scope: !35)
!393 = !DISubroutineType(types: !394)
!394 = !{!353}
!395 = !{!396}
!396 = !DITemplateTypeParameter(name: "T", type: !36)
!397 = distinct !DILocation(line: 531, column: 24, scope: !344, inlinedAt: !382)
!398 = !{!399}
!399 = distinct !{!399, !400, !"_ZN4core5array76_$LT$impl$u20$core..default..Default$u20$for$u20$$u5b$T$u3b$$u20$32$u5d$$GT$7default17h66f1ccdc17cac30eE: %_0"}
!400 = distinct !{!400, !"_ZN4core5array76_$LT$impl$u20$core..default..Default$u20$for$u20$$u5b$T$u3b$$u20$32$u5d$$GT$7default17h66f1ccdc17cac30eE"}
!401 = !DILocalVariable(name: "self", arg: 1, scope: !402, file: !403, line: 97, type: !349)
!402 = distinct !DISubprogram(name: "try_fill_bytes", linkageName: "_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E", scope: !404, file: !403, line: 97, type: !405, scopeLine: 97, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !412)
!403 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_core-0.9.3/src/os.rs", directory: "", checksumkind: CSK_MD5, checksum: "a9b676e61e33e2a3ffa0c1bbc3d87d95")
!404 = !DINamespace(name: "{impl#3}", scope: !223)
!405 = !DISubroutineType(types: !406)
!406 = !{!279, !349, !407}
!407 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut [u8]", file: !2, size: 128, align: 64, elements: !408, templateParams: !23, identifier: "bdfeb4840e2373d8742974745efe30b6")
!408 = !{!409, !411}
!409 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !407, file: !2, baseType: !410, size: 64, align: 64)
!410 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !36, size: 64, align: 64, dwarfAddressSpace: 0)
!411 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !407, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!412 = !{!401, !413}
!413 = !DILocalVariable(name: "dest", arg: 2, scope: !402, file: !403, line: 97, type: !407)
!414 = !DILocation(line: 0, scope: !402, inlinedAt: !415)
!415 = distinct !DILocation(line: 532, column: 9, scope: !352, inlinedAt: !382)
!416 = !DILocalVariable(name: "dest", arg: 1, scope: !417, file: !418, line: 66, type: !407)
!417 = distinct !DISubprogram(name: "fill", linkageName: "_ZN9getrandom4fill17h11f2509b4e4fb8bcE", scope: !255, file: !418, line: 66, type: !419, scopeLine: 66, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !436)
!418 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.3/src/lib.rs", directory: "", checksumkind: CSK_MD5, checksum: "1faf0978fd7ba93e5b0c3d32f0a6715f")
!419 = !DISubroutineType(types: !420)
!420 = !{!421, !407}
!421 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), getrandom::error::Error>", scope: !280, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !422, templateParams: !23, identifier: "83e4a8b163ed8d485ec46c0869a1009a")
!422 = !{!423}
!423 = !DICompositeType(tag: DW_TAG_variant_part, scope: !421, file: !2, size: 32, align: 32, elements: !424, templateParams: !23, identifier: "29b45ebd9e1820ce85de3dcc5076dc9c", discriminator: !435)
!424 = !{!425, !431}
!425 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !423, file: !2, baseType: !426, size: 32, align: 32, extraData: i32 0)
!426 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !421, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !427, templateParams: !429, identifier: "d0a120dd63ad1ccf4335a0a05f38d8fa")
!427 = !{!428}
!428 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !426, file: !2, baseType: !7, align: 8, flags: DIFlagPublic)
!429 = !{!92, !430}
!430 = !DITemplateTypeParameter(name: "E", type: !253)
!431 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !423, file: !2, baseType: !432, size: 32, align: 32)
!432 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !421, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !433, templateParams: !429, identifier: "8e6e5cc4b064d87a864fd2209187218d")
!433 = !{!434}
!434 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !432, file: !2, baseType: !253, size: 32, align: 32, flags: DIFlagPublic)
!435 = !DIDerivedType(tag: DW_TAG_member, scope: !421, file: !2, baseType: !198, size: 32, align: 32, flags: DIFlagArtificial)
!436 = !{!416, !437, !452}
!437 = !DILocalVariable(name: "residual", scope: !438, file: !418, line: 70, type: !439, align: 32)
!438 = distinct !DILexicalBlock(scope: !417, file: !418, line: 70, column: 60)
!439 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<core::convert::Infallible, getrandom::error::Error>", scope: !280, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !440, templateParams: !23, identifier: "a0a20b2ab4efb093be088acdc4520dc")
!440 = !{!441}
!441 = !DICompositeType(tag: DW_TAG_variant_part, scope: !439, file: !2, size: 32, align: 32, elements: !442, templateParams: !23, identifier: "de36cef668fd27db27cedfcae36838f1")
!442 = !{!443, !448}
!443 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !441, file: !2, baseType: !444, size: 32, align: 32)
!444 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !439, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !445, templateParams: !447, identifier: "6ffcd2c901f08b8c522acfcf7317934a")
!445 = !{!446}
!446 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !444, file: !2, baseType: !366, align: 8, flags: DIFlagPublic)
!447 = !{!371, !430}
!448 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !441, file: !2, baseType: !449, size: 32, align: 32)
!449 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !439, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !450, templateParams: !447, identifier: "84c7c91ab022bcf3bb8a875f2b92cc4")
!450 = !{!451}
!451 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !449, file: !2, baseType: !253, size: 32, align: 32, flags: DIFlagPublic)
!452 = !DILocalVariable(name: "val", scope: !453, file: !418, line: 70, type: !407, align: 64)
!453 = distinct !DILexicalBlock(scope: !417, file: !418, line: 70, column: 5)
!454 = !DILocation(line: 0, scope: !417, inlinedAt: !455)
!455 = distinct !DILocation(line: 98, column: 9, scope: !402, inlinedAt: !415)
!456 = !DILocalVariable(name: "dest", arg: 1, scope: !457, file: !418, line: 97, type: !475)
!457 = distinct !DISubprogram(name: "fill_uninit", linkageName: "_ZN9getrandom11fill_uninit17h0bff8f15f1575c4eE", scope: !255, file: !418, line: 97, type: !458, scopeLine: 97, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !490)
!458 = !DISubroutineType(types: !459)
!459 = !{!460, !475}
!460 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<&mut [u8], getrandom::error::Error>", scope: !280, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !461, templateParams: !23, identifier: "d5b271eb8b6a9471c1c14843be9bb67a")
!461 = !{!462}
!462 = !DICompositeType(tag: DW_TAG_variant_part, scope: !460, file: !2, size: 128, align: 64, elements: !463, templateParams: !23, identifier: "f7ea3bd174bcc52c21117617ed826c", discriminator: !474)
!463 = !{!464, !470}
!464 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !462, file: !2, baseType: !465, size: 128, align: 64)
!465 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !460, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !466, templateParams: !468, identifier: "ff0a1fc0f460c66d01b4f70777872d9")
!466 = !{!467}
!467 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !465, file: !2, baseType: !407, size: 128, align: 64, flags: DIFlagPublic)
!468 = !{!469, !430}
!469 = !DITemplateTypeParameter(name: "T", type: !407)
!470 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !462, file: !2, baseType: !471, size: 128, align: 64, extraData: i64 0)
!471 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !460, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !472, templateParams: !468, identifier: "cb75b72154a5ea0567d23dcf4a9ebbde")
!472 = !{!473}
!473 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !471, file: !2, baseType: !253, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
!474 = !DIDerivedType(tag: DW_TAG_member, scope: !460, file: !2, baseType: !203, size: 64, align: 64, flags: DIFlagArtificial)
!475 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut [core::mem::maybe_uninit::MaybeUninit<u8>]", file: !2, size: 128, align: 64, elements: !476, templateParams: !23, identifier: "dde0e961468dcbc0c165acf044bfc7c3")
!476 = !{!477, !489}
!477 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !475, file: !2, baseType: !478, size: 64, align: 64)
!478 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !479, size: 64, align: 64, dwarfAddressSpace: 0)
!479 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<u8>", scope: !480, file: !2, size: 8, align: 8, elements: !482, templateParams: !395, identifier: "33985afa3139b64ff19925f3840c2e44")
!480 = !DINamespace(name: "maybe_uninit", scope: !481)
!481 = !DINamespace(name: "mem", scope: !35)
!482 = !{!483, !484}
!483 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !479, file: !2, baseType: !7, align: 8)
!484 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !479, file: !2, baseType: !485, size: 8, align: 8)
!485 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<u8>", scope: !486, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !487, templateParams: !395, identifier: "fd8a7b1ca73aa88217cd1fad649a76fb")
!486 = !DINamespace(name: "manually_drop", scope: !481)
!487 = !{!488}
!488 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !485, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagPrivate)
!489 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !475, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!490 = !{!456, !491, !493}
!491 = !DILocalVariable(name: "residual", scope: !492, file: !418, line: 99, type: !439, align: 32)
!492 = distinct !DILexicalBlock(scope: !457, file: !418, line: 99, column: 35)
!493 = !DILocalVariable(name: "val", scope: !494, file: !418, line: 99, type: !7, align: 8)
!494 = distinct !DILexicalBlock(scope: !457, file: !418, line: 99, column: 9)
!495 = !DILocation(line: 0, scope: !457, inlinedAt: !496)
!496 = distinct !DILocation(line: 70, column: 5, scope: !417, inlinedAt: !455)
!497 = !DILocalVariable(name: "sys_fill", arg: 2, scope: !498, file: !499, line: 58, type: !505)
!498 = distinct !DISubprogram(name: "sys_fill_exact<getrandom::backends::linux_android_with_fallback::fill_inner::{closure_env#0}>", linkageName: "_ZN9getrandom8backends8use_file9util_libc14sys_fill_exact17ha95d0b037dbe5276E", scope: !500, file: !499, line: 56, type: !503, scopeLine: 56, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !535, retainedNodes: !515)
!499 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.3/src/backends/../util_libc.rs", directory: "", checksumkind: CSK_MD5, checksum: "8e2d9aabad1e84344d4c4477ca7315d7")
!500 = !DINamespace(name: "util_libc", scope: !501)
!501 = !DINamespace(name: "use_file", scope: !502)
!502 = !DINamespace(name: "backends", scope: !255)
!503 = !DISubroutineType(types: !504)
!504 = !{!421, !475, !505}
!505 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}", scope: !506, file: !2, size: 64, align: 64, elements: !508, templateParams: !23, identifier: "2a868b422fb418a23e766acb60f7a64e")
!506 = !DINamespace(name: "fill_inner", scope: !507)
!507 = !DINamespace(name: "linux_android_with_fallback", scope: !502)
!508 = !{!509}
!509 = !DIDerivedType(tag: DW_TAG_member, name: "_ref__getrandom_fn", scope: !505, file: !2, baseType: !510, size: 64, align: 64)
!510 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&unsafe extern \22C\22 fn(*mut core::ffi::c_void, usize, u32) -> isize", baseType: !511, size: 64, align: 64, dwarfAddressSpace: 0)
!511 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "unsafe extern \22C\22 fn(*mut core::ffi::c_void, usize, u32) -> isize", baseType: !512, size: 64, align: 64, dwarfAddressSpace: 0)
!512 = !DISubroutineType(types: !513)
!513 = !{!83, !514, !9, !198}
!514 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut core::ffi::c_void", baseType: !33, size: 64, align: 64, dwarfAddressSpace: 0)
!515 = !{!516, !497, !517, !519, !521, !523, !525, !527, !529, !531, !533}
!516 = !DILocalVariable(name: "buf", arg: 1, scope: !498, file: !499, line: 57, type: !475)
!517 = !DILocalVariable(name: "res", scope: !518, file: !499, line: 61, type: !83, align: 64)
!518 = distinct !DILexicalBlock(scope: !498, file: !499, line: 61, column: 9)
!519 = !DILocalVariable(name: "res", scope: !520, file: !499, line: 63, type: !83, align: 64)
!520 = distinct !DILexicalBlock(scope: !518, file: !499, line: 63, column: 13)
!521 = !DILocalVariable(name: "res", scope: !520, file: !499, line: 63, type: !522, align: 64)
!522 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&isize", baseType: !83, size: 64, align: 64, dwarfAddressSpace: 0)
!523 = !DILocalVariable(name: "len", scope: !524, file: !499, line: 64, type: !9, align: 64)
!524 = distinct !DILexicalBlock(scope: !520, file: !499, line: 64, column: 17)
!525 = !DILocalVariable(name: "residual", scope: !526, file: !499, line: 64, type: !439, align: 32)
!526 = distinct !DILexicalBlock(scope: !520, file: !499, line: 64, column: 78)
!527 = !DILocalVariable(name: "val", scope: !528, file: !499, line: 64, type: !9, align: 64)
!528 = distinct !DILexicalBlock(scope: !520, file: !499, line: 64, column: 27)
!529 = !DILocalVariable(name: "residual", scope: !530, file: !499, line: 65, type: !439, align: 32)
!530 = distinct !DILexicalBlock(scope: !524, file: !499, line: 65, column: 66)
!531 = !DILocalVariable(name: "val", scope: !532, file: !499, line: 65, type: !475, align: 64)
!532 = distinct !DILexicalBlock(scope: !524, file: !499, line: 65, column: 23)
!533 = !DILocalVariable(name: "err", scope: !534, file: !499, line: 68, type: !253, align: 32)
!534 = distinct !DILexicalBlock(scope: !518, file: !499, line: 68, column: 17)
!535 = !{!536}
!536 = !DITemplateTypeParameter(name: "impl Fn(&mut [MaybeUninit<u8>]) -> libc::ssize_t", type: !505)
!537 = !DILocation(line: 0, scope: !498, inlinedAt: !538)
!538 = distinct !DILocation(line: 97, column: 9, scope: !539, inlinedAt: !561)
!539 = distinct !DILexicalBlock(scope: !541, file: !540, line: 96, column: 9)
!540 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.3/src/backends/linux_android_with_fallback.rs", directory: "", checksumkind: CSK_MD5, checksum: "f5389189c05d242ee07e7b95e4b47e19")
!541 = distinct !DILexicalBlock(scope: !542, file: !540, line: 87, column: 5)
!542 = distinct !DILexicalBlock(scope: !543, file: !540, line: 86, column: 5)
!543 = distinct !DISubprogram(name: "fill_inner", linkageName: "_ZN9getrandom8backends27linux_android_with_fallback10fill_inner17h5c1767f0f03d271eE", scope: !507, file: !540, line: 79, type: !544, scopeLine: 79, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !546)
!544 = !DISubroutineType(types: !545)
!545 = !{!421, !475}
!546 = !{!547, !548, !549, !558, !560}
!547 = !DILocalVariable(name: "dest", arg: 1, scope: !543, file: !540, line: 79, type: !475)
!548 = !DILocalVariable(name: "raw_ptr", scope: !542, file: !540, line: 86, type: !514, align: 64)
!549 = !DILocalVariable(name: "fptr", scope: !541, file: !540, line: 87, type: !550, align: 64)
!550 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<core::ffi::c_void>", scope: !551, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !553, templateParams: !556, identifier: "e361c219328224d298c16bd9a36b63e5")
!551 = !DINamespace(name: "non_null", scope: !552)
!552 = !DINamespace(name: "ptr", scope: !35)
!553 = !{!554}
!554 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !550, file: !2, baseType: !555, size: 64, align: 64, flags: DIFlagPrivate)
!555 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const core::ffi::c_void", baseType: !33, size: 64, align: 64, dwarfAddressSpace: 0)
!556 = !{!557}
!557 = !DITemplateTypeParameter(name: "T", type: !33)
!558 = !DILocalVariable(name: "p", scope: !559, file: !540, line: 88, type: !550, align: 64)
!559 = distinct !DILexicalBlock(scope: !542, file: !540, line: 88, column: 9)
!560 = !DILocalVariable(name: "getrandom_fn", scope: !539, file: !540, line: 96, type: !511, align: 64)
!561 = distinct !DILocation(line: 99, column: 9, scope: !457, inlinedAt: !496)
!562 = !DILocalVariable(name: "self", arg: 1, scope: !563, file: !564, line: 1618, type: !569)
!563 = distinct !DISubprogram(name: "eq<core::ffi::c_void>", linkageName: "_ZN78_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17h6cdf64c3162a4c6fE", scope: !565, file: !564, line: 1618, type: !566, scopeLine: 1618, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !556, retainedNodes: !570)
!564 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/ptr/non_null.rs", directory: "", checksumkind: CSK_MD5, checksum: "3b3cd84fd90af2705fa6d8309deb8eb9")
!565 = !DINamespace(name: "{impl#14}", scope: !551)
!566 = !DISubroutineType(types: !567)
!567 = !{!568, !569, !569}
!568 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!569 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::ptr::non_null::NonNull<core::ffi::c_void>", baseType: !550, size: 64, align: 64, dwarfAddressSpace: 0)
!570 = !{!562, !571}
!571 = !DILocalVariable(name: "other", arg: 2, scope: !563, file: !564, line: 1618, type: !569)
!572 = !DILocation(line: 0, scope: !563, inlinedAt: !573)
!573 = distinct !DILocation(line: 92, column: 8, scope: !541, inlinedAt: !561)
!574 = !DILocation(line: 0, scope: !543, inlinedAt: !561)
!575 = !DILocalVariable(name: "order", scope: !576, file: !577, line: 1567, type: !40, align: 8)
!576 = distinct !DISubprogram(name: "load<core::ffi::c_void>", linkageName: "_ZN4core4sync6atomic18AtomicPtr$LT$T$GT$4load17h8d3b225ec6367179E", scope: !578, file: !577, line: 1567, type: !587, scopeLine: 1567, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !556, declaration: !590, retainedNodes: !591)
!577 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/sync/atomic.rs", directory: "", checksumkind: CSK_MD5, checksum: "857639279079d3d11ec5f1aef3f6a77a")
!578 = !DICompositeType(tag: DW_TAG_structure_type, name: "AtomicPtr<core::ffi::c_void>", scope: !41, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !579, templateParams: !556, identifier: "940cc0fa8d3d46c1b04884c608fe053e")
!579 = !{!580}
!580 = !DIDerivedType(tag: DW_TAG_member, name: "p", scope: !578, file: !2, baseType: !581, size: 64, align: 64, flags: DIFlagPrivate)
!581 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<*mut core::ffi::c_void>", scope: !582, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !583, templateParams: !585, identifier: "476077520d2320d29a467bbf0a2828d")
!582 = !DINamespace(name: "cell", scope: !35)
!583 = !{!584}
!584 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !581, file: !2, baseType: !514, size: 64, align: 64, flags: DIFlagPrivate)
!585 = !{!586}
!586 = !DITemplateTypeParameter(name: "T", type: !514)
!587 = !DISubroutineType(types: !588)
!588 = !{!514, !589, !40}
!589 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::sync::atomic::AtomicPtr<core::ffi::c_void>", baseType: !578, size: 64, align: 64, dwarfAddressSpace: 0)
!590 = !DISubprogram(name: "load<core::ffi::c_void>", linkageName: "_ZN4core4sync6atomic18AtomicPtr$LT$T$GT$4load17h8d3b225ec6367179E", scope: !578, file: !577, line: 1567, type: !587, scopeLine: 1567, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !556)
!591 = !{!592, !575}
!592 = !DILocalVariable(name: "self", arg: 1, scope: !576, file: !577, line: 1567, type: !589)
!593 = !DILocation(line: 0, scope: !576, inlinedAt: !594)
!594 = distinct !DILocation(line: 86, column: 32, scope: !543, inlinedAt: !561)
!595 = !DILocalVariable(name: "dst", arg: 1, scope: !596, file: !577, line: 3728, type: !599)
!596 = distinct !DISubprogram(name: "atomic_load<*mut core::ffi::c_void>", linkageName: "_ZN4core4sync6atomic11atomic_load17h8f683aafed77e7a6E", scope: !41, file: !577, line: 3728, type: !597, scopeLine: 3728, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !585, retainedNodes: !600)
!597 = !DISubroutineType(types: !598)
!598 = !{!514, !599, !40}
!599 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *mut core::ffi::c_void", baseType: !514, size: 64, align: 64, dwarfAddressSpace: 0)
!600 = !{!595, !601}
!601 = !DILocalVariable(name: "order", arg: 2, scope: !596, file: !577, line: 3728, type: !40)
!602 = !DILocation(line: 0, scope: !596, inlinedAt: !603)
!603 = distinct !DILocation(line: 1569, column: 18, scope: !576, inlinedAt: !594)
!604 = !DILocation(line: 3733, column: 24, scope: !596, inlinedAt: !603)
!605 = !{!606, !608, !610, !386, !272}
!606 = distinct !{!606, !607, !"_ZN9getrandom8backends27linux_android_with_fallback10fill_inner17h5c1767f0f03d271eE: %dest.0"}
!607 = distinct !{!607, !"_ZN9getrandom8backends27linux_android_with_fallback10fill_inner17h5c1767f0f03d271eE"}
!608 = distinct !{!608, !609, !"_ZN9getrandom4fill17h11f2509b4e4fb8bcE: %dest.0"}
!609 = distinct !{!609, !"_ZN9getrandom4fill17h11f2509b4e4fb8bcE"}
!610 = distinct !{!610, !611, !"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E: %dest.0"}
!611 = distinct !{!611, !"_ZN62_$LT$rand_core..os..OsRng$u20$as$u20$rand_core..TryRngCore$GT$14try_fill_bytes17hf3d790d02097cbc9E"}
!612 = !DILocation(line: 0, scope: !542, inlinedAt: !561)
!613 = !DILocalVariable(name: "ptr", arg: 1, scope: !614, file: !564, line: 255, type: !514)
!614 = distinct !DISubprogram(name: "new<core::ffi::c_void>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17h80db20d9e0f6d719E", scope: !550, file: !564, line: 255, type: !615, scopeLine: 255, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !556, declaration: !631, retainedNodes: !632)
!615 = !DISubroutineType(types: !616)
!616 = !{!617, !514}
!617 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<core::ptr::non_null::NonNull<core::ffi::c_void>>", scope: !618, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !619, templateParams: !23, identifier: "edfa4e906f75924a8426ccf6f0c3ffaa")
!618 = !DINamespace(name: "option", scope: !35)
!619 = !{!620}
!620 = !DICompositeType(tag: DW_TAG_variant_part, scope: !617, file: !2, size: 64, align: 64, elements: !621, templateParams: !23, identifier: "f04d05b9f0681cae91cf66a1e9e23fc", discriminator: !630)
!621 = !{!622, !626}
!622 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !620, file: !2, baseType: !623, size: 64, align: 64, extraData: i64 0)
!623 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !617, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !624, identifier: "768783b45c98f68f1879709417b28c6c")
!624 = !{!625}
!625 = !DITemplateTypeParameter(name: "T", type: !550)
!626 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !620, file: !2, baseType: !627, size: 64, align: 64)
!627 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !617, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !628, templateParams: !624, identifier: "bace7ac2bd6060d6fe615060b57f13f3")
!628 = !{!629}
!629 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !627, file: !2, baseType: !550, size: 64, align: 64, flags: DIFlagPublic)
!630 = !DIDerivedType(tag: DW_TAG_member, scope: !617, file: !2, baseType: !203, size: 64, align: 64, flags: DIFlagArtificial)
!631 = !DISubprogram(name: "new<core::ffi::c_void>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17h80db20d9e0f6d719E", scope: !550, file: !564, line: 255, type: !615, scopeLine: 255, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !556)
!632 = !{!613}
!633 = !DILocation(line: 0, scope: !614, inlinedAt: !634)
!634 = distinct !DILocation(line: 87, column: 22, scope: !542, inlinedAt: !561)
!635 = !DILocation(line: 256, column: 13, scope: !614, inlinedAt: !634)
!636 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!637 = !DILocation(line: 89, column: 17, scope: !542, inlinedAt: !561)
!638 = !DILocation(line: 0, scope: !541, inlinedAt: !561)
!639 = !DILocation(line: 1619, column: 9, scope: !563, inlinedAt: !573)
!640 = !DILocation(line: 92, column: 8, scope: !541, inlinedAt: !561)
!641 = !DILocalVariable(name: "getrandom_fn", scope: !642, file: !540, line: 96, type: !511, align: 64)
!642 = distinct !DISubprogram(name: "{closure#0}", linkageName: "_ZN9getrandom8backends27linux_android_with_fallback10fill_inner28_$u7b$$u7b$closure$u7d$$u7d$17h7c5d5879a1df2957E", scope: !506, file: !540, line: 97, type: !643, scopeLine: 97, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !646)
!643 = !DISubroutineType(types: !644)
!644 = !{!83, !645, !475}
!645 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&getrandom::backends::linux_android_with_fallback::fill_inner::{closure_env#0}", baseType: !505, size: 64, align: 64, dwarfAddressSpace: 0)
!646 = !{!647, !641}
!647 = !DILocalVariable(name: "buf", arg: 2, scope: !642, file: !540, line: 97, type: !475)
!648 = !DILocation(line: 0, scope: !642, inlinedAt: !649)
!649 = distinct !DILocation(line: 61, column: 19, scope: !498, inlinedAt: !538)
!650 = !DILocation(line: 98, column: 13, scope: !642, inlinedAt: !649)
!651 = !{!652, !386, !272}
!652 = distinct !{!652, !653, !"_ZN9getrandom8backends8use_file9util_libc14sys_fill_exact17ha95d0b037dbe5276E: argument 1"}
!653 = distinct !{!653, !"_ZN9getrandom8backends8use_file9util_libc14sys_fill_exact17ha95d0b037dbe5276E"}
!654 = !DILocation(line: 0, scope: !518, inlinedAt: !538)
!655 = !DILocation(line: 0, scope: !520, inlinedAt: !538)
!656 = !DILocation(line: 63, column: 20, scope: !518, inlinedAt: !538)
!657 = !DILocation(line: 62, column: 9, scope: !518, inlinedAt: !538)
!658 = !DILocation(line: 68, column: 27, scope: !518, inlinedAt: !538)
!659 = !DILocation(line: 0, scope: !534, inlinedAt: !538)
!660 = !DILocalVariable(name: "self", arg: 1, scope: !661, file: !662, line: 88, type: !253)
!661 = distinct !DISubprogram(name: "raw_os_error", linkageName: "_ZN9getrandom5error5Error12raw_os_error17hb243f18ee1719933E", scope: !253, file: !662, line: 88, type: !663, scopeLine: 88, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, declaration: !676, retainedNodes: !677)
!662 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.3/src/error.rs", directory: "", checksumkind: CSK_MD5, checksum: "771456b8ebc6726476a73dc08ba5a3eb")
!663 = !DISubroutineType(types: !664)
!664 = !{!665, !253}
!665 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<i32>", scope: !618, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !666, templateParams: !23, identifier: "e1655327d1ce207c347490cdc4308e3f")
!666 = !{!667}
!667 = !DICompositeType(tag: DW_TAG_variant_part, scope: !665, file: !2, size: 64, align: 32, elements: !668, templateParams: !23, identifier: "1995b4680135c1de232f0b7acd3948b", discriminator: !675)
!668 = !{!669, !671}
!669 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !667, file: !2, baseType: !670, size: 64, align: 32, extraData: i32 0)
!670 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !665, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !23, templateParams: !267, identifier: "c89efb4fc5b954572e4641396a82323e")
!671 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !667, file: !2, baseType: !672, size: 64, align: 32, extraData: i32 1)
!672 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !665, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !673, templateParams: !267, identifier: "55ec302645c3ac12d11d4eb48b9b59c")
!673 = !{!674}
!674 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !672, file: !2, baseType: !101, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!675 = !DIDerivedType(tag: DW_TAG_member, scope: !665, file: !2, baseType: !198, size: 32, align: 32, flags: DIFlagArtificial)
!676 = !DISubprogram(name: "raw_os_error", linkageName: "_ZN9getrandom5error5Error12raw_os_error17hb243f18ee1719933E", scope: !253, file: !662, line: 88, type: !663, scopeLine: 88, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!677 = !{!660, !678}
!678 = !DILocalVariable(name: "code", scope: !679, file: !662, line: 89, type: !101, align: 32)
!679 = distinct !DILexicalBlock(scope: !661, file: !662, line: 89, column: 9)
!680 = !DILocation(line: 0, scope: !661, inlinedAt: !681)
!681 = distinct !DILocation(line: 70, column: 24, scope: !534, inlinedAt: !538)
!682 = !DILocation(line: 0, scope: !679, inlinedAt: !681)
!683 = !DILocation(line: 115, column: 16, scope: !679, inlinedAt: !681)
!684 = !DILocalVariable(name: "self", arg: 1, scope: !685, file: !301, line: 2008, type: !706)
!685 = distinct !DISubprogram(name: "branch<&mut [core::mem::maybe_uninit::MaybeUninit<u8>], getrandom::error::Error>", linkageName: "_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h8ab92a5330928040E", scope: !686, file: !301, line: 2008, type: !687, scopeLine: 2008, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !714, retainedNodes: !721)
!686 = !DINamespace(name: "{impl#27}", scope: !280)
!687 = !DISubroutineType(types: !688)
!688 = !{!689, !706}
!689 = !DICompositeType(tag: DW_TAG_structure_type, name: "ControlFlow<core::result::Result<core::convert::Infallible, getrandom::error::Error>, &mut [core::mem::maybe_uninit::MaybeUninit<u8>]>", scope: !690, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !691, templateParams: !23, identifier: "b2448ea5010dba4ecc53fcf9c981a7b")
!690 = !DINamespace(name: "control_flow", scope: !137)
!691 = !{!692}
!692 = !DICompositeType(tag: DW_TAG_variant_part, scope: !689, file: !2, size: 128, align: 64, elements: !693, templateParams: !23, identifier: "ce6e60f227e4fe532bc3537ab4378980", discriminator: !705)
!693 = !{!694, !701}
!694 = !DIDerivedType(tag: DW_TAG_member, name: "Continue", scope: !692, file: !2, baseType: !695, size: 128, align: 64)
!695 = !DICompositeType(tag: DW_TAG_structure_type, name: "Continue", scope: !689, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !696, templateParams: !698, identifier: "4902db8277564bdf6d774e7863675545")
!696 = !{!697}
!697 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !695, file: !2, baseType: !475, size: 128, align: 64, flags: DIFlagPublic)
!698 = !{!699, !700}
!699 = !DITemplateTypeParameter(name: "B", type: !439)
!700 = !DITemplateTypeParameter(name: "C", type: !475)
!701 = !DIDerivedType(tag: DW_TAG_member, name: "Break", scope: !692, file: !2, baseType: !702, size: 128, align: 64, extraData: i64 0)
!702 = !DICompositeType(tag: DW_TAG_structure_type, name: "Break", scope: !689, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !703, templateParams: !698, identifier: "cd12e69630180526d09dcfb41a58e0a0")
!703 = !{!704}
!704 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !702, file: !2, baseType: !439, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
!705 = !DIDerivedType(tag: DW_TAG_member, scope: !689, file: !2, baseType: !203, size: 64, align: 64, flags: DIFlagArtificial)
!706 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<&mut [core::mem::maybe_uninit::MaybeUninit<u8>], getrandom::error::Error>", scope: !280, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !707, templateParams: !23, identifier: "54875cdb341398217ea8f6fe73d06de1")
!707 = !{!708}
!708 = !DICompositeType(tag: DW_TAG_variant_part, scope: !706, file: !2, size: 128, align: 64, elements: !709, templateParams: !23, identifier: "945030fa0dd35e98e22a85243473ef52", discriminator: !720)
!709 = !{!710, !716}
!710 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !708, file: !2, baseType: !711, size: 128, align: 64)
!711 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !706, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !712, templateParams: !714, identifier: "e679703d7c0366fc5a04357dd1afe476")
!712 = !{!713}
!713 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !711, file: !2, baseType: !475, size: 128, align: 64, flags: DIFlagPublic)
!714 = !{!715, !430}
!715 = !DITemplateTypeParameter(name: "T", type: !475)
!716 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !708, file: !2, baseType: !717, size: 128, align: 64, extraData: i64 0)
!717 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !706, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !718, templateParams: !714, identifier: "badcde712446d198b67290eb4b82be9")
!718 = !{!719}
!719 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !717, file: !2, baseType: !253, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
!720 = !DIDerivedType(tag: DW_TAG_member, scope: !706, file: !2, baseType: !203, size: 64, align: 64, flags: DIFlagArtificial)
!721 = !{!684, !722, !724}
!722 = !DILocalVariable(name: "v", scope: !723, file: !301, line: 2010, type: !475, align: 64)
!723 = distinct !DILexicalBlock(scope: !685, file: !301, line: 2010, column: 13)
!724 = !DILocalVariable(name: "e", scope: !725, file: !301, line: 2011, type: !253, align: 32)
!725 = distinct !DILexicalBlock(scope: !685, file: !301, line: 2011, column: 13)
!726 = !DILocation(line: 0, scope: !685, inlinedAt: !727)
!727 = distinct !DILocation(line: 65, column: 23, scope: !524, inlinedAt: !538)
!728 = !DILocation(line: 60, column: 12, scope: !498, inlinedAt: !538)
!729 = !DILocation(line: 0, scope: !524, inlinedAt: !538)
!730 = !DILocalVariable(name: "index", scope: !731, file: !732, line: 619, type: !749, align: 64)
!731 = distinct !DISubprogram(name: "get_mut<core::mem::maybe_uninit::MaybeUninit<u8>, core::ops::range::RangeFrom<usize>>", linkageName: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7get_mut17hf0e0507db7039061E", scope: !733, file: !732, line: 619, type: !735, scopeLine: 619, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !757, retainedNodes: !755)
!732 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/slice/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "c0b5edce80aed843a079f65948cd2d97")
!733 = !DINamespace(name: "{impl#0}", scope: !734)
!734 = !DINamespace(name: "slice", scope: !35)
!735 = !DISubroutineType(types: !736)
!736 = !{!737, !475, !749}
!737 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&mut [core::mem::maybe_uninit::MaybeUninit<u8>]>", scope: !618, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !738, templateParams: !23, identifier: "e02c7f6170e888924a200289ab5dd29")
!738 = !{!739}
!739 = !DICompositeType(tag: DW_TAG_variant_part, scope: !737, file: !2, size: 128, align: 64, elements: !740, templateParams: !23, identifier: "8c73d15be9eb78d62f2f97fc6ab9552c", discriminator: !748)
!740 = !{!741, !744}
!741 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !739, file: !2, baseType: !742, size: 128, align: 64, extraData: i64 0)
!742 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !737, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !743, identifier: "990e3359395a2a9de56121807cac1a01")
!743 = !{!715}
!744 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !739, file: !2, baseType: !745, size: 128, align: 64)
!745 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !737, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !746, templateParams: !743, identifier: "38a7bfc194039282d403d0ca63c79e85")
!746 = !{!747}
!747 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !745, file: !2, baseType: !475, size: 128, align: 64, flags: DIFlagPublic)
!748 = !DIDerivedType(tag: DW_TAG_member, scope: !737, file: !2, baseType: !203, size: 64, align: 64, flags: DIFlagArtificial)
!749 = !DICompositeType(tag: DW_TAG_structure_type, name: "RangeFrom<usize>", scope: !750, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !751, templateParams: !753, identifier: "20dd8f89a39ed9c4ed1d74f21e2590e")
!750 = !DINamespace(name: "range", scope: !137)
!751 = !{!752}
!752 = !DIDerivedType(tag: DW_TAG_member, name: "start", scope: !749, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPublic)
!753 = !{!754}
!754 = !DITemplateTypeParameter(name: "Idx", type: !9)
!755 = !{!756, !730}
!756 = !DILocalVariable(name: "self", arg: 1, scope: !731, file: !732, line: 619, type: !475)
!757 = !{!758, !759}
!758 = !DITemplateTypeParameter(name: "T", type: !479)
!759 = !DITemplateTypeParameter(name: "I", type: !749)
!760 = !DILocation(line: 0, scope: !731, inlinedAt: !761)
!761 = distinct !DILocation(line: 65, column: 27, scope: !524, inlinedAt: !538)
!762 = !DILocalVariable(name: "self", scope: !763, file: !764, line: 542, type: !749, align: 64)
!763 = distinct !DISubprogram(name: "get_mut<core::mem::maybe_uninit::MaybeUninit<u8>>", linkageName: "_ZN110_$LT$core..ops..range..RangeFrom$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$7get_mut17hc6a29d167a67c21cE", scope: !765, file: !764, line: 542, type: !767, scopeLine: 542, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !771, retainedNodes: !769)
!764 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/slice/index.rs", directory: "", checksumkind: CSK_MD5, checksum: "baa2a238d3c3ce81e755e760737a6886")
!765 = !DINamespace(name: "{impl#7}", scope: !766)
!766 = !DINamespace(name: "index", scope: !734)
!767 = !DISubroutineType(types: !768)
!768 = !{!737, !749, !475}
!769 = !{!762, !770}
!770 = !DILocalVariable(name: "slice", arg: 2, scope: !763, file: !764, line: 542, type: !475)
!771 = !{!758}
!772 = !DILocation(line: 0, scope: !763, inlinedAt: !773)
!773 = distinct !DILocation(line: 623, column: 15, scope: !731, inlinedAt: !761)
!774 = !DILocalVariable(name: "self", scope: !775, file: !764, line: 377, type: !779, align: 64)
!775 = distinct !DISubprogram(name: "get_mut<core::mem::maybe_uninit::MaybeUninit<u8>>", linkageName: "_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$7get_mut17he480c3cf57ed840fE", scope: !776, file: !764, line: 377, type: !777, scopeLine: 377, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !771, retainedNodes: !783)
!776 = !DINamespace(name: "{impl#4}", scope: !766)
!777 = !DISubroutineType(types: !778)
!778 = !{!737, !779, !475}
!779 = !DICompositeType(tag: DW_TAG_structure_type, name: "Range<usize>", scope: !750, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !780, templateParams: !753, identifier: "9a3f37a980c09302627e6f483c3e1ec2")
!780 = !{!781, !782}
!781 = !DIDerivedType(tag: DW_TAG_member, name: "start", scope: !779, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPublic)
!782 = !DIDerivedType(tag: DW_TAG_member, name: "end", scope: !779, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!783 = !{!774, !774, !784, !785}
!784 = !DILocalVariable(name: "slice", arg: 2, scope: !775, file: !764, line: 377, type: !475)
!785 = !DILocalVariable(name: "new_len", scope: !775, file: !764, line: 378, type: !9, align: 64)
!786 = !DILocation(line: 0, scope: !775, inlinedAt: !787)
!787 = distinct !DILocation(line: 543, column: 35, scope: !763, inlinedAt: !773)
!788 = !DILocalVariable(name: "rhs", arg: 2, scope: !789, file: !790, line: 698, type: !9)
!789 = distinct !DISubprogram(name: "checked_sub", linkageName: "_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_sub17hb1cb57f58d49264dE", scope: !791, file: !790, line: 698, type: !792, scopeLine: 698, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !807)
!790 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/num/uint_macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "10cb569da042ef801acea143a0c73ffd")
!791 = !DINamespace(name: "{impl#11}", scope: !260)
!792 = !DISubroutineType(types: !793)
!793 = !{!794, !9, !9}
!794 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<usize>", scope: !618, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !795, templateParams: !23, identifier: "f68a2b7c3ee2cdf2fd7ff27937bd2af")
!795 = !{!796}
!796 = !DICompositeType(tag: DW_TAG_variant_part, scope: !794, file: !2, size: 128, align: 64, elements: !797, templateParams: !23, identifier: "dfb82d512c7342523dd52dc2924145e3", discriminator: !806)
!797 = !{!798, !802}
!798 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !796, file: !2, baseType: !799, size: 128, align: 64, extraData: i64 0)
!799 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !794, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !800, identifier: "4a42b54124b56d6195fb546eaea984ac")
!800 = !{!801}
!801 = !DITemplateTypeParameter(name: "T", type: !9)
!802 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !796, file: !2, baseType: !803, size: 128, align: 64, extraData: i64 1)
!803 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !794, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !804, templateParams: !800, identifier: "592f525925657159c4d0d34ea72ff0fe")
!804 = !{!805}
!805 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !803, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!806 = !DIDerivedType(tag: DW_TAG_member, scope: !794, file: !2, baseType: !203, size: 64, align: 64, flags: DIFlagArtificial)
!807 = !{!808, !788}
!808 = !DILocalVariable(name: "self", arg: 1, scope: !789, file: !790, line: 698, type: !9)
!809 = !DILocation(line: 0, scope: !789, inlinedAt: !810)
!810 = distinct !DILocation(line: 378, column: 32, scope: !775, inlinedAt: !787)
!811 = !DILocation(line: 704, column: 16, scope: !789, inlinedAt: !810)
!812 = !DILocation(line: 2009, column: 9, scope: !685, inlinedAt: !727)
!813 = !DILocation(line: 93, column: 9, scope: !541, inlinedAt: !561)
!814 = !DILocalVariable(name: "self", arg: 1, scope: !815, file: !301, line: 2008, type: !279)
!815 = distinct !DISubprogram(name: "branch<(), rand_core::os::OsError>", linkageName: "_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h1db3b00c1fb82457E", scope: !686, file: !301, line: 2008, type: !816, scopeLine: 2008, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !288, retainedNodes: !834)
!816 = !DISubroutineType(types: !817)
!817 = !{!818, !279}
!818 = !DICompositeType(tag: DW_TAG_structure_type, name: "ControlFlow<core::result::Result<core::convert::Infallible, rand_core::os::OsError>, ()>", scope: !690, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !819, templateParams: !23, identifier: "90d3b6de0c63b8e855faeb3aee55390")
!819 = !{!820}
!820 = !DICompositeType(tag: DW_TAG_variant_part, scope: !818, file: !2, size: 32, align: 32, elements: !821, templateParams: !23, identifier: "e99c02cf6c6f1bb799a23179814e0274", discriminator: !833)
!821 = !{!822, !829}
!822 = !DIDerivedType(tag: DW_TAG_member, name: "Continue", scope: !820, file: !2, baseType: !823, size: 32, align: 32, extraData: i32 0)
!823 = !DICompositeType(tag: DW_TAG_structure_type, name: "Continue", scope: !818, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !824, templateParams: !826, identifier: "d93396031251b881ccd4e21207912858")
!824 = !{!825}
!825 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !823, file: !2, baseType: !7, align: 8, flags: DIFlagPublic)
!826 = !{!827, !828}
!827 = !DITemplateTypeParameter(name: "B", type: !358)
!828 = !DITemplateTypeParameter(name: "C", type: !7)
!829 = !DIDerivedType(tag: DW_TAG_member, name: "Break", scope: !820, file: !2, baseType: !830, size: 32, align: 32)
!830 = !DICompositeType(tag: DW_TAG_structure_type, name: "Break", scope: !818, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !831, templateParams: !826, identifier: "bad8b7aa150ec32dc6f5526eb696b3c7")
!831 = !{!832}
!832 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !830, file: !2, baseType: !358, size: 32, align: 32, flags: DIFlagPublic)
!833 = !DIDerivedType(tag: DW_TAG_member, scope: !818, file: !2, baseType: !198, size: 32, align: 32, flags: DIFlagArtificial)
!834 = !{!814, !835, !837}
!835 = !DILocalVariable(name: "v", scope: !836, file: !301, line: 2010, type: !7, align: 8)
!836 = distinct !DILexicalBlock(scope: !815, file: !301, line: 2010, column: 13)
!837 = !DILocalVariable(name: "e", scope: !838, file: !301, line: 2011, type: !250, align: 32)
!838 = distinct !DILexicalBlock(scope: !815, file: !301, line: 2011, column: 13)
!839 = !DILocation(line: 0, scope: !815, inlinedAt: !840)
!840 = distinct !DILocation(line: 532, column: 9, scope: !352, inlinedAt: !382)
!841 = !DILocation(line: 2009, column: 15, scope: !815, inlinedAt: !840)
!842 = !DILocation(line: 2009, column: 9, scope: !815, inlinedAt: !840)
!843 = !DILocation(line: 534, column: 5, scope: !344, inlinedAt: !382)
!844 = !DILocation(line: 0, scope: !300, inlinedAt: !340)
!845 = !DILocalVariable(name: "self__inner", scope: !846, file: !176, line: 208, type: !183, align: 128)
!846 = distinct !DISubprogram(name: "{closure#0}<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed28_$u7b$$u7b$closure$u7d$$u7d$17h0ef576ded73a595eE", scope: !320, file: !176, line: 209, type: !847, scopeLine: 209, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !227, retainedNodes: !849)
!847 = !DISubroutineType(types: !848)
!848 = !{null, !319, !183}
!849 = !{!850, !845, !851, !852}
!850 = !DILocalVariable(name: "result", arg: 2, scope: !846, file: !176, line: 209, type: !183)
!851 = !DILocalVariable(name: "self__threshold", scope: !846, file: !176, line: 208, type: !218, align: 64)
!852 = !DILocalVariable(name: "self__bytes_until_reseed", scope: !846, file: !176, line: 208, type: !218, align: 64)
!853 = !DILocation(line: 0, scope: !846, inlinedAt: !854)
!854 = distinct !DILocation(line: 773, column: 25, scope: !336, inlinedAt: !340)
!855 = !DILocation(line: 226, column: 35, scope: !247)
!856 = !DILocation(line: 776, column: 5, scope: !300, inlinedAt: !340)
!857 = !DILocation(line: 533, column: 28, scope: !352, inlinedAt: !382)
!858 = !DILocalVariable(name: "seed", arg: 1, scope: !859, file: !860, line: 99, type: !353)
!859 = distinct !DISubprogram(name: "from_seed", linkageName: "_ZN76_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..SeedableRng$GT$9from_seed17h908954789d451da6E", scope: !861, file: !860, line: 99, type: !862, scopeLine: 99, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !864)
!860 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_chacha-0.9.0/src/chacha.rs", directory: "", checksumkind: CSK_MD5, checksum: "7f4e0005afead6890469401ecfa5f424")
!861 = !DINamespace(name: "{impl#24}", scope: !184)
!862 = !DISubroutineType(types: !863)
!863 = !{!183, !353}
!864 = !{!858}
!865 = !DILocation(line: 99, column: 26, scope: !859, inlinedAt: !866)
!866 = distinct !DILocation(line: 533, column: 12, scope: !352, inlinedAt: !382)
!867 = !DILocalVariable(name: "key", arg: 1, scope: !868, file: !869, line: 74, type: !872)
!868 = distinct !DISubprogram(name: "new", linkageName: "_ZN11rand_chacha4guts6ChaCha3new17hf3fc1524424c46feE", scope: !188, file: !869, line: 74, type: !870, scopeLine: 74, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, declaration: !877, retainedNodes: !878)
!869 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_chacha-0.9.0/src/guts.rs", directory: "", checksumkind: CSK_MD5, checksum: "d8991d5076408722f1c6dc41e4372770")
!870 = !DISubroutineType(types: !871)
!871 = !{!188, !872, !873}
!872 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[u8; 32]", baseType: !353, size: 64, align: 64, dwarfAddressSpace: 0)
!873 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[u8]", file: !2, size: 128, align: 64, elements: !874, templateParams: !23, identifier: "31681e0c10b314f1f33e38b2779acbb4")
!874 = !{!875, !876}
!875 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !873, file: !2, baseType: !410, size: 64, align: 64)
!876 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !873, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!877 = !DISubprogram(name: "new", linkageName: "_ZN11rand_chacha4guts6ChaCha3new17hf3fc1524424c46feE", scope: !188, file: !869, line: 74, type: !870, scopeLine: 74, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!878 = !{!867, !879}
!879 = !DILocalVariable(name: "nonce", arg: 2, scope: !868, file: !869, line: 74, type: !873)
!880 = !DILocation(line: 0, scope: !868, inlinedAt: !881)
!881 = distinct !DILocation(line: 101, column: 28, scope: !859, inlinedAt: !866)
!882 = !DILocation(line: 533, column: 9, scope: !352, inlinedAt: !382)
!883 = !DILocation(line: 75, column: 9, scope: !868, inlinedAt: !881)
!884 = !DILocation(line: 533, column: 32, scope: !352, inlinedAt: !382)
!885 = !DILocation(line: 209, column: 49, scope: !276, inlinedAt: !298)
!886 = !DILocation(line: 209, column: 50, scope: !846, inlinedAt: !854)
!887 = !DILocation(line: 210, column: 39, scope: !846, inlinedAt: !854)
!888 = !{!889, !891}
!889 = distinct !{!889, !890, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed28_$u7b$$u7b$closure$u7d$$u7d$17h0ef576ded73a595eE: %_1"}
!890 = distinct !{!890, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed28_$u7b$$u7b$closure$u7d$$u7d$17h0ef576ded73a595eE"}
!891 = distinct !{!891, !890, !"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed28_$u7b$$u7b$closure$u7d$$u7d$17h0ef576ded73a595eE: %result"}
!892 = !DILocation(line: 211, column: 13, scope: !846, inlinedAt: !854)
!893 = !DILocation(line: 773, column: 30, scope: !300, inlinedAt: !340)
!894 = !DILocation(line: 212, column: 10, scope: !276, inlinedAt: !298)
!895 = !DILocation(line: 226, column: 9, scope: !247)
!896 = !DILocalVariable(name: "self", arg: 1, scope: !897, file: !860, line: 90, type: !324)
!897 = distinct !DISubprogram(name: "generate", linkageName: "_ZN84_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17hcc73ae4b352d57fcE", scope: !898, file: !860, line: 90, type: !899, scopeLine: 90, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !901)
!898 = !DINamespace(name: "{impl#23}", scope: !184)
!899 = !DISubroutineType(types: !900)
!900 = !{null, !324, !233}
!901 = !{!896, !902}
!902 = !DILocalVariable(name: "r", arg: 2, scope: !897, file: !860, line: 90, type: !233)
!903 = !DILocation(line: 0, scope: !897, inlinedAt: !904)
!904 = distinct !DILocation(line: 227, column: 9, scope: !247)
!905 = !DILocalVariable(name: "drounds", scope: !906, file: !869, line: 80, type: !198, align: 32)
!906 = distinct !DISubprogram(name: "refill4", linkageName: "_ZN11rand_chacha4guts6ChaCha7refill417h8e27ff2a18dab7adE", scope: !188, file: !869, line: 80, type: !907, scopeLine: 80, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, declaration: !911, retainedNodes: !912)
!907 = !DISubroutineType(types: !908)
!908 = !{null, !909, !198, !910}
!909 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand_chacha::guts::ChaCha", baseType: !188, size: 64, align: 64, dwarfAddressSpace: 0)
!910 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut [u32; 64]", baseType: !237, size: 64, align: 64, dwarfAddressSpace: 0)
!911 = !DISubprogram(name: "refill4", linkageName: "_ZN11rand_chacha4guts6ChaCha7refill417h8e27ff2a18dab7adE", scope: !188, file: !869, line: 80, type: !907, scopeLine: 80, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!912 = !{!913, !905, !914}
!913 = !DILocalVariable(name: "self", arg: 1, scope: !906, file: !869, line: 80, type: !909)
!914 = !DILocalVariable(name: "out", arg: 3, scope: !906, file: !869, line: 80, type: !910)
!915 = !DILocation(line: 0, scope: !906, inlinedAt: !916)
!916 = distinct !DILocation(line: 91, column: 28, scope: !897, inlinedAt: !904)
!917 = !DILocation(line: 81, column: 9, scope: !906, inlinedAt: !916)
!918 = !DILocation(line: 228, column: 6, scope: !175)
!919 = distinct !DISubprogram(name: "speak", linkageName: "_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17h95a71617b8cd4b02E", scope: !921, file: !920, line: 10, type: !922, scopeLine: 10, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !925)
!920 = !DIFile(filename: "src/main.rs", directory: "/home/np/hack/verifopt/dp-ex", checksumkind: CSK_MD5, checksum: "b3f97947870f94c9202daaf6fb1ed201")
!921 = !DINamespace(name: "{impl#0}", scope: !69)
!922 = !DISubroutineType(types: !923)
!923 = !{null, !924}
!924 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&dp_ex::Dog", baseType: !68, size: 64, align: 64, dwarfAddressSpace: 0)
!925 = !{!926}
!926 = !DILocalVariable(name: "self", arg: 1, scope: !919, file: !920, line: 10, type: !924)
!927 = !DILocation(line: 0, scope: !919)
!928 = !DILocalVariable(name: "pieces", scope: !929, file: !930, line: 600, type: !1088, align: 64)
!929 = distinct !DISubprogram(name: "new_const<1>", linkageName: "_ZN4core3fmt9Arguments9new_const17h4071c16c1b3d8f01E", scope: !931, file: !930, line: 600, type: !1086, scopeLine: 600, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, declaration: !1090, retainedNodes: !1091)
!930 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/fmt/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "55150785045f6b77da421daee4eba248")
!931 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !51, file: !2, size: 384, align: 64, flags: DIFlagPublic, elements: !932, templateParams: !23, identifier: "24eb0fb5d76b97e2d20682aa7b755ab")
!932 = !{!933, !943, !987}
!933 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !931, file: !2, baseType: !934, size: 128, align: 64, flags: DIFlagPrivate)
!934 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !2, size: 128, align: 64, elements: !935, templateParams: !23, identifier: "4e66b00a376d6af5b8765440fb2839f")
!935 = !{!936, !942}
!936 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !934, file: !2, baseType: !937, size: 64, align: 64)
!937 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !938, size: 64, align: 64, dwarfAddressSpace: 0)
!938 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 128, align: 64, elements: !939, templateParams: !23, identifier: "9277eecd40495f85161460476aacc992")
!939 = !{!940, !941}
!940 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !938, file: !2, baseType: !410, size: 64, align: 64)
!941 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !938, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!942 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !934, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!943 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !931, file: !2, baseType: !944, size: 128, align: 64, offset: 256, flags: DIFlagPrivate)
!944 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::Placeholder]>", scope: !618, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !945, templateParams: !23, identifier: "2d1751345db4736156c9498f57ce083a")
!945 = !{!946}
!946 = !DICompositeType(tag: DW_TAG_variant_part, scope: !944, file: !2, size: 128, align: 64, elements: !947, templateParams: !23, identifier: "d11d7583563b78f79b7a9c1fddef5d53", discriminator: !986)
!947 = !{!948, !982}
!948 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !946, file: !2, baseType: !949, size: 128, align: 64, extraData: i64 0)
!949 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !944, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !950, identifier: "5dfa75cac8702857f2ccfde7135f7333")
!950 = !{!951}
!951 = !DITemplateTypeParameter(name: "T", type: !952)
!952 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Placeholder]", file: !2, size: 128, align: 64, elements: !953, templateParams: !23, identifier: "359448f2614cfcc0d0c1c56c841cf880")
!953 = !{!954, !981}
!954 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !952, file: !2, baseType: !955, size: 64, align: 64)
!955 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !956, size: 64, align: 64, dwarfAddressSpace: 0)
!956 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !50, file: !2, size: 448, align: 64, flags: DIFlagPublic, elements: !957, templateParams: !23, identifier: "37d51e956e565f2360e5336db010fb94")
!957 = !{!958, !959, !961, !962, !963, !980}
!958 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !956, file: !2, baseType: !9, size: 64, align: 64, offset: 256, flags: DIFlagPublic)
!959 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !956, file: !2, baseType: !960, size: 32, align: 32, offset: 352, flags: DIFlagPublic)
!960 = !DIBasicType(name: "char", size: 32, encoding: DW_ATE_UTF)
!961 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !956, file: !2, baseType: !49, size: 8, align: 8, offset: 384, flags: DIFlagPublic)
!962 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !956, file: !2, baseType: !198, size: 32, align: 32, offset: 320, flags: DIFlagPublic)
!963 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !956, file: !2, baseType: !964, size: 128, align: 64, flags: DIFlagPublic)
!964 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !50, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !965, templateParams: !23, identifier: "33fe681e4a4ed9828ac2f6a98ef48c92")
!965 = !{!966}
!966 = !DICompositeType(tag: DW_TAG_variant_part, scope: !964, file: !2, size: 128, align: 64, elements: !967, templateParams: !23, identifier: "17f74d77c68ee09fd7e4b3771d0838f4", discriminator: !979)
!967 = !{!968, !973, !977}
!968 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !966, file: !2, baseType: !969, size: 128, align: 64, extraData: i16 0)
!969 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !964, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !970, templateParams: !23, identifier: "b598892e00d12602a714369da84c5e40")
!970 = !{!971}
!971 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !969, file: !2, baseType: !972, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!972 = !DIBasicType(name: "u16", size: 16, encoding: DW_ATE_unsigned)
!973 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !966, file: !2, baseType: !974, size: 128, align: 64, extraData: i16 1)
!974 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !964, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !975, templateParams: !23, identifier: "5e02220b8c537696d6d32410ef9ad7cb")
!975 = !{!976}
!976 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !974, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!977 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !966, file: !2, baseType: !978, size: 128, align: 64, extraData: i16 2)
!978 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !964, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, identifier: "d2975225efab438f6e0bbcf92cacbea2")
!979 = !DIDerivedType(tag: DW_TAG_member, scope: !964, file: !2, baseType: !972, size: 16, align: 16, flags: DIFlagArtificial)
!980 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !956, file: !2, baseType: !964, size: 128, align: 64, offset: 128, flags: DIFlagPublic)
!981 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !952, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!982 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !946, file: !2, baseType: !983, size: 128, align: 64)
!983 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !944, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !984, templateParams: !950, identifier: "c738961c11197bf175c161a53fc8f300")
!984 = !{!985}
!985 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !983, file: !2, baseType: !952, size: 128, align: 64, flags: DIFlagPublic)
!986 = !DIDerivedType(tag: DW_TAG_member, scope: !944, file: !2, baseType: !203, size: 64, align: 64, flags: DIFlagArtificial)
!987 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !931, file: !2, baseType: !988, size: 128, align: 64, offset: 128, flags: DIFlagPrivate)
!988 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Argument]", file: !2, size: 128, align: 64, elements: !989, templateParams: !23, identifier: "7018af262b519fbf626cdbb35a367f12")
!989 = !{!990, !1085}
!990 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !988, file: !2, baseType: !991, size: 64, align: 64)
!991 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !992, size: 64, align: 64, dwarfAddressSpace: 0)
!992 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !50, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !993, templateParams: !23, identifier: "25620a2bf73f444f61cf887636e73c9c")
!993 = !{!994}
!994 = !DIDerivedType(tag: DW_TAG_member, name: "ty", scope: !992, file: !2, baseType: !995, size: 128, align: 64, flags: DIFlagPrivate)
!995 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentType", scope: !50, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !996, templateParams: !23, identifier: "6b7e829d90a5625593a17c12bac24a22")
!996 = !{!997}
!997 = !DICompositeType(tag: DW_TAG_variant_part, scope: !995, file: !2, size: 128, align: 64, elements: !998, templateParams: !23, identifier: "dbe2fa85d59ebf2abe6f5e2bf512dd10", discriminator: !1084)
!998 = !{!999, !1080}
!999 = !DIDerivedType(tag: DW_TAG_member, name: "Placeholder", scope: !997, file: !2, baseType: !1000, size: 128, align: 64)
!1000 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !995, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !1001, templateParams: !23, identifier: "9360cd60a524b377598c38d09f2ed0e")
!1001 = !{!1002, !1006, !1074}
!1002 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1000, file: !2, baseType: !1003, size: 64, align: 64, flags: DIFlagPrivate)
!1003 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<()>", scope: !551, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1004, templateParams: !91, identifier: "3e0ac18be859eeb0b4ae62bb9b847f81")
!1004 = !{!1005}
!1005 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !1003, file: !2, baseType: !6, size: 64, align: 64, flags: DIFlagPrivate)
!1006 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !1000, file: !2, baseType: !1007, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!1007 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "unsafe fn(core::ptr::non_null::NonNull<()>, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !1008, size: 64, align: 64, dwarfAddressSpace: 0)
!1008 = !DISubroutineType(types: !1009)
!1009 = !{!1010, !1003, !1026}
!1010 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), core::fmt::Error>", scope: !280, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !1011, templateParams: !23, identifier: "6816ab7da155524a7f7fa5256de3cf82")
!1011 = !{!1012}
!1012 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1010, file: !2, size: 8, align: 8, elements: !1013, templateParams: !23, identifier: "e8d0d2b8f2420234621de41ea2595fd1", discriminator: !1025)
!1013 = !{!1014, !1021}
!1014 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1012, file: !2, baseType: !1015, size: 8, align: 8, extraData: i8 0)
!1015 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1010, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !1016, templateParams: !1018, identifier: "e5f3207fdcf62eacaa53939734a74154")
!1016 = !{!1017}
!1017 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1015, file: !2, baseType: !7, align: 8, offset: 8, flags: DIFlagPublic)
!1018 = !{!92, !1019}
!1019 = !DITemplateTypeParameter(name: "E", type: !1020)
!1020 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !51, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "bd06eb806965e0e83cbfa92c8110c24d")
!1021 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1012, file: !2, baseType: !1022, size: 8, align: 8, extraData: i8 1)
!1022 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1010, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !1023, templateParams: !1018, identifier: "851959764360495ed30d5fff18ec423")
!1023 = !{!1024}
!1024 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1022, file: !2, baseType: !1020, align: 8, offset: 8, flags: DIFlagPublic)
!1025 = !DIDerivedType(tag: DW_TAG_member, scope: !1010, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagArtificial)
!1026 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !1027, size: 64, align: 64, dwarfAddressSpace: 0)
!1027 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !51, file: !2, size: 320, align: 64, flags: DIFlagPublic, elements: !1028, templateParams: !23, identifier: "3156aacd96f386f053e3f612f97c88")
!1028 = !{!1029, !1063}
!1029 = !DIDerivedType(tag: DW_TAG_member, name: "options", scope: !1027, file: !2, baseType: !1030, size: 160, align: 32, offset: 128, flags: DIFlagPrivate)
!1030 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormattingOptions", scope: !51, file: !2, size: 160, align: 32, flags: DIFlagPublic, elements: !1031, templateParams: !23, identifier: "fc1aad6aa3841ba2385d309ea9faf97f")
!1031 = !{!1032, !1033, !1034, !1048, !1062}
!1032 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !1030, file: !2, baseType: !198, size: 32, align: 32, offset: 96, flags: DIFlagPrivate)
!1033 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !1030, file: !2, baseType: !960, size: 32, align: 32, flags: DIFlagPrivate)
!1034 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !1030, file: !2, baseType: !1035, size: 8, align: 8, offset: 128, flags: DIFlagPrivate)
!1035 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<core::fmt::Alignment>", scope: !618, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !1036, templateParams: !23, identifier: "4db9caa4211fb655a1f558f9da6744dc")
!1036 = !{!1037}
!1037 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1035, file: !2, size: 8, align: 8, elements: !1038, templateParams: !23, identifier: "c9d5472e7f251313105cce4473782d1f", discriminator: !1047)
!1038 = !{!1039, !1043}
!1039 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !1037, file: !2, baseType: !1040, size: 8, align: 8, extraData: i8 3)
!1040 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !1035, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !23, templateParams: !1041, identifier: "5c74e537c166370284bc9075df990f51")
!1041 = !{!1042}
!1042 = !DITemplateTypeParameter(name: "T", type: !57)
!1043 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !1037, file: !2, baseType: !1044, size: 8, align: 8)
!1044 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !1035, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !1045, templateParams: !1041, identifier: "8063e44fd517a1d3ea1079db8136250")
!1045 = !{!1046}
!1046 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1044, file: !2, baseType: !57, size: 8, align: 8, flags: DIFlagPublic)
!1047 = !DIDerivedType(tag: DW_TAG_member, scope: !1035, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagArtificial)
!1048 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !1030, file: !2, baseType: !1049, size: 32, align: 16, offset: 32, flags: DIFlagPrivate)
!1049 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<u16>", scope: !618, file: !2, size: 32, align: 16, flags: DIFlagPublic, elements: !1050, templateParams: !23, identifier: "320c093e14f19e7dd75720678f877ce3")
!1050 = !{!1051}
!1051 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1049, file: !2, size: 32, align: 16, elements: !1052, templateParams: !23, identifier: "91b32d8e3c156f713174cb354a398aae", discriminator: !1061)
!1052 = !{!1053, !1057}
!1053 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !1051, file: !2, baseType: !1054, size: 32, align: 16, extraData: i16 0)
!1054 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !1049, file: !2, size: 32, align: 16, flags: DIFlagPublic, elements: !23, templateParams: !1055, identifier: "97265f141f100e86e7f3d477667dd131")
!1055 = !{!1056}
!1056 = !DITemplateTypeParameter(name: "T", type: !972)
!1057 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !1051, file: !2, baseType: !1058, size: 32, align: 16, extraData: i16 1)
!1058 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !1049, file: !2, size: 32, align: 16, flags: DIFlagPublic, elements: !1059, templateParams: !1055, identifier: "57de06c6dcc5971442200f74e602309c")
!1059 = !{!1060}
!1060 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1058, file: !2, baseType: !972, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!1061 = !DIDerivedType(tag: DW_TAG_member, scope: !1049, file: !2, baseType: !972, size: 16, align: 16, flags: DIFlagArtificial)
!1062 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !1030, file: !2, baseType: !1049, size: 32, align: 16, offset: 64, flags: DIFlagPrivate)
!1063 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !1027, file: !2, baseType: !1064, size: 128, align: 64, flags: DIFlagPrivate)
!1064 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !2, size: 128, align: 64, elements: !1065, templateParams: !23, identifier: "61781eb331eb2173128184c9b2bf02dc")
!1065 = !{!1066, !1069}
!1066 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !1064, file: !2, baseType: !1067, size: 64, align: 64)
!1067 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !1068, size: 64, align: 64, dwarfAddressSpace: 0)
!1068 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !2, align: 8, elements: !23, identifier: "285f69df1e9048ce486c1c48ca17de85")
!1069 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !1064, file: !2, baseType: !1070, size: 64, align: 64, offset: 64)
!1070 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 6]", baseType: !1071, size: 64, align: 64, dwarfAddressSpace: 0)
!1071 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 384, align: 64, elements: !1072)
!1072 = !{!1073}
!1073 = !DISubrange(count: 6, lowerBound: 0)
!1074 = !DIDerivedType(tag: DW_TAG_member, name: "_lifetime", scope: !1000, file: !2, baseType: !1075, align: 8, offset: 128, flags: DIFlagPrivate)
!1075 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&()>", scope: !1076, file: !2, align: 8, flags: DIFlagPublic, elements: !23, templateParams: !1077, identifier: "114a5780a36016c559f40910cc5f9248")
!1076 = !DINamespace(name: "marker", scope: !35)
!1077 = !{!1078}
!1078 = !DITemplateTypeParameter(name: "T", type: !1079)
!1079 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&()", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 0)
!1080 = !DIDerivedType(tag: DW_TAG_member, name: "Count", scope: !997, file: !2, baseType: !1081, size: 128, align: 64, extraData: i64 0)
!1081 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !995, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !1082, templateParams: !23, identifier: "54f8e57d2009f180917aae6b0e82c575")
!1082 = !{!1083}
!1083 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1081, file: !2, baseType: !972, size: 16, align: 16, offset: 64, flags: DIFlagPrivate)
!1084 = !DIDerivedType(tag: DW_TAG_member, scope: !995, file: !2, baseType: !203, size: 64, align: 64, flags: DIFlagArtificial)
!1085 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !988, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!1086 = !DISubroutineType(types: !1087)
!1087 = !{!931, !1088}
!1088 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[&str; 1]", baseType: !1089, size: 64, align: 64, dwarfAddressSpace: 0)
!1089 = !DICompositeType(tag: DW_TAG_array_type, baseType: !938, size: 128, align: 64, elements: !209)
!1090 = !DISubprogram(name: "new_const<1>", linkageName: "_ZN4core3fmt9Arguments9new_const17h4071c16c1b3d8f01E", scope: !931, file: !930, line: 600, type: !1086, scopeLine: 600, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!1091 = !{!928}
!1092 = !DILocation(line: 0, scope: !929, inlinedAt: !1093)
!1093 = !DILocation(line: 11, column: 9, scope: !919)
!1094 = !DILocation(line: 602, column: 9, scope: !929, inlinedAt: !1093)
!1095 = !DILocation(line: 12, column: 6, scope: !919)
!1096 = distinct !DISubprogram(name: "speak", linkageName: "_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17hfb6e878b47c18265E", scope: !1097, file: !920, line: 18, type: !1098, scopeLine: 18, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !1101)
!1097 = !DINamespace(name: "{impl#1}", scope: !69)
!1098 = !DISubroutineType(types: !1099)
!1099 = !{null, !1100}
!1100 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&dp_ex::Cat", baseType: !78, size: 64, align: 64, dwarfAddressSpace: 0)
!1101 = !{!1102}
!1102 = !DILocalVariable(name: "self", arg: 1, scope: !1096, file: !920, line: 18, type: !1100)
!1103 = !DILocation(line: 0, scope: !1096)
!1104 = !DILocalVariable(name: "pieces", scope: !1105, file: !930, line: 600, type: !1088, align: 64)
!1105 = distinct !DISubprogram(name: "new_const<1>", linkageName: "_ZN4core3fmt9Arguments9new_const17h4071c16c1b3d8f01E", scope: !931, file: !930, line: 600, type: !1086, scopeLine: 600, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, declaration: !1090, retainedNodes: !1106)
!1106 = !{!1104}
!1107 = !DILocation(line: 0, scope: !1105, inlinedAt: !1108)
!1108 = !DILocation(line: 19, column: 9, scope: !1096)
!1109 = !DILocation(line: 602, column: 9, scope: !1105, inlinedAt: !1108)
!1110 = !DILocation(line: 20, column: 6, scope: !1096)
!1111 = distinct !DISubprogram(name: "main", linkageName: "_ZN5dp_ex4main17hc7990c7b9cee8a83E", scope: !69, file: !920, line: 65, type: !21, scopeLine: 65, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized | DISPFlagMainSubprogram, unit: !30, templateParams: !23)
!1112 = !DILocalVariable(name: "rng", scope: !1113, file: !920, line: 26, type: !1126, align: 64)
!1113 = distinct !DILexicalBlock(scope: !1114, file: !920, line: 26, column: 5)
!1114 = distinct !DILexicalBlock(scope: !1115, file: !920, line: 24, column: 5)
!1115 = distinct !DISubprogram(name: "dyn_dp", linkageName: "_ZN5dp_ex6dyn_dp17h87c4dad2651e3135E", scope: !69, file: !920, line: 23, type: !21, scopeLine: 23, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !1116)
!1116 = !{!1117, !1112}
!1117 = !DILocalVariable(name: "a", scope: !1114, file: !920, line: 24, type: !1118, align: 64)
!1118 = !DICompositeType(tag: DW_TAG_structure_type, name: "&dyn dp_ex::Animal", file: !2, size: 128, align: 64, elements: !1119, templateParams: !23, identifier: "a178c33aa19ec831c4f9d4301ca5e8d5")
!1119 = !{!1120, !1123}
!1120 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !1118, file: !2, baseType: !1121, size: 64, align: 64)
!1121 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !1122, size: 64, align: 64, dwarfAddressSpace: 0)
!1122 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn dp_ex::Animal", file: !2, align: 8, elements: !23, identifier: "df9ac727fbc75c2d941b2128404024cc")
!1123 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !1118, file: !2, baseType: !1124, size: 64, align: 64, offset: 64)
!1124 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 4]", baseType: !1125, size: 64, align: 64, dwarfAddressSpace: 0)
!1125 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 256, align: 64, elements: !199)
!1126 = !DICompositeType(tag: DW_TAG_structure_type, name: "ThreadRng", scope: !1127, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1128, templateParams: !23, identifier: "4ab1881b8b411a5cadc96c64932ddfa7")
!1127 = !DINamespace(name: "thread", scope: !179)
!1128 = !{!1129}
!1129 = !DIDerivedType(tag: DW_TAG_member, name: "rng", scope: !1126, file: !2, baseType: !1130, size: 64, align: 64, flags: DIFlagPrivate)
!1130 = !DICompositeType(tag: DW_TAG_structure_type, name: "Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", scope: !1131, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1133, templateParams: !1175, identifier: "2441776dfe26f17ea1e97b5724f08ec3")
!1131 = !DINamespace(name: "rc", scope: !1132)
!1132 = !DINamespace(name: "alloc", scope: null)
!1133 = !{!1134, !1170, !1172}
!1134 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !1130, file: !2, baseType: !1135, size: 64, align: 64, flags: DIFlagPrivate)
!1135 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", scope: !551, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1136, templateParams: !1168, identifier: "6657d814155cb6a4fef2f2c8a5d2ed76")
!1136 = !{!1137}
!1137 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !1135, file: !2, baseType: !1138, size: 64, align: 64, flags: DIFlagPrivate)
!1138 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>", baseType: !1139, size: 64, align: 64, dwarfAddressSpace: 0)
!1139 = !DICompositeType(tag: DW_TAG_structure_type, name: "RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>", scope: !1131, file: !2, size: 2816, align: 128, flags: DIFlagPrivate, elements: !1140, templateParams: !1166, identifier: "2ba302f6965b308a386cfd5850553c95")
!1140 = !{!1141, !1148, !1149}
!1141 = !DIDerivedType(tag: DW_TAG_member, name: "strong", scope: !1139, file: !2, baseType: !1142, size: 64, align: 64, flags: DIFlagPrivate)
!1142 = !DICompositeType(tag: DW_TAG_structure_type, name: "Cell<usize>", scope: !582, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1143, templateParams: !800, identifier: "850821af38703e743ab2164fb52c6e1a")
!1143 = !{!1144}
!1144 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1142, file: !2, baseType: !1145, size: 64, align: 64, flags: DIFlagPrivate)
!1145 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<usize>", scope: !582, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1146, templateParams: !800, identifier: "13a2388050bdea621562362d97db0cb8")
!1146 = !{!1147}
!1147 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1145, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPrivate)
!1148 = !DIDerivedType(tag: DW_TAG_member, name: "weak", scope: !1139, file: !2, baseType: !1142, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!1149 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1139, file: !2, baseType: !1150, size: 2688, align: 128, offset: 128, flags: DIFlagPrivate)
!1150 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", scope: !582, file: !2, size: 2688, align: 128, flags: DIFlagPublic, elements: !1151, templateParams: !1164, identifier: "1e6fe10d094e887acf7e418c5201184a")
!1151 = !{!1152}
!1152 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1150, file: !2, baseType: !1153, size: 2688, align: 128, flags: DIFlagPrivate)
!1153 = !DICompositeType(tag: DW_TAG_structure_type, name: "ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", scope: !178, file: !2, size: 2688, align: 128, flags: DIFlagPublic, elements: !1154, templateParams: !227, identifier: "92ed0024a69ea761e3c8116b8e389515")
!1154 = !{!1155}
!1155 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1153, file: !2, baseType: !1156, size: 2688, align: 128, flags: DIFlagPrivate)
!1156 = !DICompositeType(tag: DW_TAG_structure_type, name: "BlockRng<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", scope: !1157, file: !2, size: 2688, align: 128, flags: DIFlagPublic, elements: !1158, templateParams: !1162, identifier: "a738b89e9f785734824d0ef2945771d6")
!1157 = !DINamespace(name: "block", scope: !224)
!1158 = !{!1159, !1160, !1161}
!1159 = !DIDerivedType(tag: DW_TAG_member, name: "results", scope: !1156, file: !2, baseType: !234, size: 2048, align: 32, flags: DIFlagPrivate)
!1160 = !DIDerivedType(tag: DW_TAG_member, name: "index", scope: !1156, file: !2, baseType: !9, size: 64, align: 64, offset: 2560, flags: DIFlagPrivate)
!1161 = !DIDerivedType(tag: DW_TAG_member, name: "core", scope: !1156, file: !2, baseType: !177, size: 512, align: 128, offset: 2048, flags: DIFlagPublic)
!1162 = !{!1163}
!1163 = !DITemplateTypeParameter(name: "R", type: !177)
!1164 = !{!1165}
!1165 = !DITemplateTypeParameter(name: "T", type: !1153)
!1166 = !{!1167}
!1167 = !DITemplateTypeParameter(name: "T", type: !1150)
!1168 = !{!1169}
!1169 = !DITemplateTypeParameter(name: "T", type: !1139)
!1170 = !DIDerivedType(tag: DW_TAG_member, name: "phantom", scope: !1130, file: !2, baseType: !1171, align: 8, offset: 64, flags: DIFlagPrivate)
!1171 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", scope: !1076, file: !2, align: 8, flags: DIFlagPublic, elements: !23, templateParams: !1168, identifier: "e9f1737b92b3623ac8b3aa7b210147f9")
!1172 = !DIDerivedType(tag: DW_TAG_member, name: "alloc", scope: !1130, file: !2, baseType: !1173, align: 8, offset: 64, flags: DIFlagPrivate)
!1173 = !DICompositeType(tag: DW_TAG_structure_type, name: "Global", scope: !1174, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "cc328e40098c35b06088ca0399a07f60")
!1174 = !DINamespace(name: "alloc", scope: !1132)
!1175 = !{!1167, !1176}
!1176 = !DITemplateTypeParameter(name: "A", type: !1173)
!1177 = !DILocation(line: 26, column: 9, scope: !1113, inlinedAt: !1178)
!1178 = distinct !DILocation(line: 80, column: 5, scope: !1111)
!1179 = !DILocalVariable(name: "self", scope: !1180, file: !1181, line: 198, type: !1187, align: 64)
!1180 = distinct !DISubprogram(name: "sample<rand::rngs::thread::ThreadRng>", linkageName: "_ZN4rand5distr5other110_$LT$impl$u20$rand..distr..distribution..Distribution$LT$bool$GT$$u20$for$u20$rand..distr..StandardUniform$GT$6sample17h8080265986bde9b0E", scope: !1182, file: !1181, line: 198, type: !1185, scopeLine: 198, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1192, retainedNodes: !1190)
!1181 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/distr/other.rs", directory: "", checksumkind: CSK_MD5, checksum: "ec48bf9fde3f2913ed48dceedb97c641")
!1182 = !DINamespace(name: "{impl#6}", scope: !1183)
!1183 = !DINamespace(name: "other", scope: !1184)
!1184 = !DINamespace(name: "distr", scope: !180)
!1185 = !DISubroutineType(types: !1186)
!1186 = !{!568, !1187, !1189}
!1187 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&rand::distr::StandardUniform", baseType: !1188, size: 64, align: 64, dwarfAddressSpace: 0)
!1188 = !DICompositeType(tag: DW_TAG_structure_type, name: "StandardUniform", scope: !1184, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "1cb1e543b6950708d7acc6b4a4f54ff3")
!1189 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand::rngs::thread::ThreadRng", baseType: !1126, size: 64, align: 64, dwarfAddressSpace: 0)
!1190 = !{!1179, !1191}
!1191 = !DILocalVariable(name: "rng", arg: 2, scope: !1180, file: !1181, line: 198, type: !1189)
!1192 = !{!1193}
!1193 = !DITemplateTypeParameter(name: "R", type: !1126)
!1194 = !DILocation(line: 0, scope: !1180, inlinedAt: !1195)
!1195 = distinct !DILocation(line: 99, column: 25, scope: !1196, inlinedAt: !1207)
!1196 = distinct !DISubprogram(name: "random<rand::rngs::thread::ThreadRng, bool>", linkageName: "_ZN4rand3rng3Rng6random17h860650e97b46471dE", scope: !1198, file: !1197, line: 95, type: !1200, scopeLine: 95, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1204, retainedNodes: !1202)
!1197 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/rng.rs", directory: "", checksumkind: CSK_MD5, checksum: "502aa58ff0bfc907d978a28fc3e03a36")
!1198 = !DINamespace(name: "Rng", scope: !1199)
!1199 = !DINamespace(name: "rng", scope: !180)
!1200 = !DISubroutineType(types: !1201)
!1201 = !{!568, !1189}
!1202 = !{!1203}
!1203 = !DILocalVariable(name: "self", arg: 1, scope: !1196, file: !1197, line: 95, type: !1189)
!1204 = !{!1205, !1206}
!1205 = !DITemplateTypeParameter(name: "Self", type: !1126)
!1206 = !DITemplateTypeParameter(name: "T", type: !568)
!1207 = distinct !DILocation(line: 27, column: 12, scope: !1113, inlinedAt: !1178)
!1208 = !DILocation(line: 26, column: 9, scope: !1114, inlinedAt: !1178)
!1209 = !DILocation(line: 26, column: 19, scope: !1114, inlinedAt: !1178)
!1210 = !DILocation(line: 0, scope: !1196, inlinedAt: !1207)
!1211 = !DILocalVariable(name: "self", arg: 1, scope: !1212, file: !1213, line: 170, type: !1189)
!1212 = distinct !DISubprogram(name: "next_u32", linkageName: "_ZN68_$LT$rand..rngs..thread..ThreadRng$u20$as$u20$rand_core..RngCore$GT$8next_u3217h46677650533b7ff9E", scope: !1214, file: !1213, line: 170, type: !1215, scopeLine: 170, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !1217)
!1213 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/rngs/thread.rs", directory: "", checksumkind: CSK_MD5, checksum: "842ea45780d9ccda805de377693390ed")
!1214 = !DINamespace(name: "{impl#3}", scope: !1127)
!1215 = !DISubroutineType(types: !1216)
!1216 = !{!198, !1189}
!1217 = !{!1211, !1218}
!1218 = !DILocalVariable(name: "rng", scope: !1219, file: !1213, line: 173, type: !1220, align: 64)
!1219 = distinct !DILexicalBlock(scope: !1212, file: !1213, line: 173, column: 9)
!1220 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", baseType: !1153, size: 64, align: 64, dwarfAddressSpace: 0)
!1221 = !DILocation(line: 0, scope: !1212, inlinedAt: !1222)
!1222 = distinct !DILocation(line: 203, column: 14, scope: !1180, inlinedAt: !1195)
!1223 = !DILocalVariable(name: "self", arg: 1, scope: !1224, file: !1225, line: 2194, type: !1228)
!1224 = distinct !DISubprogram(name: "get<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h9888cd175f29ba9eE", scope: !1150, file: !1225, line: 2194, type: !1226, scopeLine: 2194, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1164, declaration: !1229, retainedNodes: !1230)
!1225 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/cell.rs", directory: "", checksumkind: CSK_MD5, checksum: "64646115eb882926cf02189caaf4b938")
!1226 = !DISubroutineType(types: !1227)
!1227 = !{!1220, !1228}
!1228 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", baseType: !1150, size: 64, align: 64, dwarfAddressSpace: 0)
!1229 = !DISubprogram(name: "get<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN4core4cell19UnsafeCell$LT$T$GT$3get17h9888cd175f29ba9eE", scope: !1150, file: !1225, line: 2194, type: !1226, scopeLine: 2194, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1164)
!1230 = !{!1223}
!1231 = !DILocation(line: 0, scope: !1224, inlinedAt: !1232)
!1232 = distinct !DILocation(line: 173, column: 43, scope: !1212, inlinedAt: !1222)
!1233 = !DILocation(line: 2198, column: 9, scope: !1224, inlinedAt: !1232)
!1234 = !DILocation(line: 0, scope: !1219, inlinedAt: !1222)
!1235 = !DILocalVariable(name: "self", arg: 1, scope: !1236, file: !176, line: 113, type: !1220)
!1236 = distinct !DISubprogram(name: "next_u32<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN90_$LT$rand..rngs..reseeding..ReseedingRng$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h2012677de426454fE", scope: !1237, file: !176, line: 113, type: !1238, scopeLine: 113, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !227, retainedNodes: !1241)
!1237 = !DINamespace(name: "{impl#1}", scope: !178)
!1238 = !DISubroutineType(types: !1239)
!1239 = !{!198, !1240}
!1240 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", baseType: !1153, size: 64, align: 64, dwarfAddressSpace: 0)
!1241 = !{!1235}
!1242 = !DILocation(line: 0, scope: !1236, inlinedAt: !1243)
!1243 = distinct !DILocation(line: 174, column: 13, scope: !1219, inlinedAt: !1222)
!1244 = !DILocalVariable(name: "self", arg: 1, scope: !1245, file: !1246, line: 186, type: !1250)
!1245 = distinct !DISubprogram(name: "next_u32<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h0fdce2b914f75430E", scope: !1247, file: !1246, line: 186, type: !1248, scopeLine: 186, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1162, retainedNodes: !1251)
!1246 = !DIFile(filename: "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_core-0.9.3/src/block.rs", directory: "", checksumkind: CSK_MD5, checksum: "b5ef00244c22c51d50ecb2ae1bb4a1ae")
!1247 = !DINamespace(name: "{impl#2}", scope: !1157)
!1248 = !DISubroutineType(types: !1249)
!1249 = !{!198, !1250}
!1250 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut rand_core::block::BlockRng<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", baseType: !1156, size: 64, align: 64, dwarfAddressSpace: 0)
!1251 = !{!1244, !1252}
!1252 = !DILocalVariable(name: "value", scope: !1253, file: !1246, line: 191, type: !198, align: 32)
!1253 = distinct !DILexicalBlock(scope: !1245, file: !1246, line: 191, column: 9)
!1254 = !DILocation(line: 0, scope: !1245, inlinedAt: !1255)
!1255 = distinct !DILocation(line: 114, column: 9, scope: !1236, inlinedAt: !1243)
!1256 = !DILocation(line: 187, column: 12, scope: !1245, inlinedAt: !1255)
!1257 = !{!1258}
!1258 = distinct !{!1258, !1259, !"_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h0fdce2b914f75430E: %self"}
!1259 = distinct !{!1259, !"_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h0fdce2b914f75430E"}
!1260 = !DILocalVariable(name: "self", arg: 1, scope: !1261, file: !1246, line: 177, type: !1250)
!1261 = distinct !DISubprogram(name: "generate_and_set<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h8021faec2696411eE", scope: !1156, file: !1246, line: 177, type: !1262, scopeLine: 177, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1162, declaration: !1264, retainedNodes: !1265)
!1262 = !DISubroutineType(types: !1263)
!1263 = !{null, !1250, !9}
!1264 = !DISubprogram(name: "generate_and_set<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>", linkageName: "_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h8021faec2696411eE", scope: !1156, file: !1246, line: 177, type: !1262, scopeLine: 177, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1162)
!1265 = !{!1260, !1266}
!1266 = !DILocalVariable(name: "index", arg: 2, scope: !1261, file: !1246, line: 177, type: !9)
!1267 = !DILocation(line: 0, scope: !1261, inlinedAt: !1268)
!1268 = distinct !DILocation(line: 188, column: 13, scope: !1245, inlinedAt: !1255)
!1269 = !DILocation(line: 179, column: 9, scope: !1261, inlinedAt: !1268)
!1270 = !DILocalVariable(name: "results", arg: 2, scope: !1271, file: !176, line: 162, type: !233)
!1271 = distinct !DISubprogram(name: "generate<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>", linkageName: "_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h63618727820f9588E", scope: !1272, file: !176, line: 162, type: !230, scopeLine: 162, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !227, retainedNodes: !1273)
!1272 = !DINamespace(name: "{impl#4}", scope: !178)
!1273 = !{!1274, !1270, !1275}
!1274 = !DILocalVariable(name: "self", arg: 1, scope: !1271, file: !176, line: 162, type: !232)
!1275 = !DILocalVariable(name: "num_bytes", scope: !1276, file: !176, line: 169, type: !9, align: 64)
!1276 = distinct !DILexicalBlock(scope: !1271, file: !176, line: 169, column: 9)
!1277 = !DILocation(line: 0, scope: !1271, inlinedAt: !1278)
!1278 = distinct !DILocation(line: 179, column: 9, scope: !1261, inlinedAt: !1268)
!1279 = !DILocation(line: 163, column: 12, scope: !1271, inlinedAt: !1278)
!1280 = !{!1281, !1283, !1258}
!1281 = distinct !{!1281, !1282, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h63618727820f9588E: %self"}
!1282 = distinct !{!1282, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h63618727820f9588E"}
!1283 = distinct !{!1283, !1284, !"_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h8021faec2696411eE: %self"}
!1284 = distinct !{!1284, !"_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h8021faec2696411eE"}
!1285 = !{!1286}
!1286 = distinct !{!1286, !1282, !"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h63618727820f9588E: argument 1"}
!1287 = !DILocation(line: 0, scope: !1276, inlinedAt: !1278)
!1288 = !DILocation(line: 170, column: 9, scope: !1276, inlinedAt: !1278)
!1289 = !DILocation(line: 0, scope: !897, inlinedAt: !1290)
!1290 = distinct !DILocation(line: 171, column: 9, scope: !1276, inlinedAt: !1278)
!1291 = !DILocation(line: 0, scope: !906, inlinedAt: !1292)
!1292 = distinct !DILocation(line: 91, column: 28, scope: !897, inlinedAt: !1290)
!1293 = !DILocation(line: 81, column: 9, scope: !906, inlinedAt: !1292)
!1294 = !DILocation(line: 167, column: 20, scope: !1271, inlinedAt: !1278)
!1295 = !{!1296}
!1296 = distinct !{!1296, !1297, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hd497d5f9546794adE: %_1"}
!1297 = distinct !{!1297, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hd497d5f9546794adE"}
!1298 = !DILocation(line: 34, column: 1, scope: !1114, inlinedAt: !1178)
!1299 = !DILocalVariable(arg: 1, scope: !1300, file: !1301, line: 523, type: !1304)
!1300 = distinct !DISubprogram(name: "drop_in_place<rand::rngs::thread::ThreadRng>", linkageName: "_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hd497d5f9546794adE", scope: !552, file: !1301, line: 523, type: !1302, scopeLine: 523, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1306, retainedNodes: !1305)
!1301 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "a5cb8c8d2ea510166b9e600d925000e6")
!1302 = !DISubroutineType(types: !1303)
!1303 = !{null, !1304}
!1304 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut rand::rngs::thread::ThreadRng", baseType: !1126, size: 64, align: 64, dwarfAddressSpace: 0)
!1305 = !{!1299}
!1306 = !{!1307}
!1307 = !DITemplateTypeParameter(name: "T", type: !1126)
!1308 = !DILocation(line: 0, scope: !1300, inlinedAt: !1309)
!1309 = distinct !DILocation(line: 34, column: 1, scope: !1114, inlinedAt: !1178)
!1310 = !{!1311}
!1311 = distinct !{!1311, !1312, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17hc39867958365ed55E: %_1"}
!1312 = distinct !{!1312, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17hc39867958365ed55E"}
!1313 = !DILocation(line: 523, column: 1, scope: !1300, inlinedAt: !1309)
!1314 = !DILocalVariable(arg: 1, scope: !1315, file: !1301, line: 523, type: !1318)
!1315 = distinct !DISubprogram(name: "drop_in_place<alloc::rc::Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>>", linkageName: "_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17hc39867958365ed55E", scope: !552, file: !1301, line: 523, type: !1316, scopeLine: 523, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1320, retainedNodes: !1319)
!1316 = !DISubroutineType(types: !1317)
!1317 = !{null, !1318}
!1318 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut alloc::rc::Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", baseType: !1130, size: 64, align: 64, dwarfAddressSpace: 0)
!1319 = !{!1314}
!1320 = !{!1321}
!1321 = !DITemplateTypeParameter(name: "T", type: !1130)
!1322 = !DILocation(line: 0, scope: !1315, inlinedAt: !1323)
!1323 = distinct !DILocation(line: 523, column: 1, scope: !1300, inlinedAt: !1309)
!1324 = !{!1325}
!1325 = distinct !{!1325, !1326, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfce35e40ab15106fE: %self"}
!1326 = distinct !{!1326, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfce35e40ab15106fE"}
!1327 = !DILocation(line: 523, column: 1, scope: !1315, inlinedAt: !1323)
!1328 = !DILocalVariable(name: "self", arg: 1, scope: !1329, file: !1330, line: 2296, type: !1334)
!1329 = distinct !DISubprogram(name: "drop<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", linkageName: "_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfce35e40ab15106fE", scope: !1331, file: !1330, line: 2296, type: !1332, scopeLine: 2296, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1175, retainedNodes: !1335)
!1330 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/alloc/src/rc.rs", directory: "", checksumkind: CSK_MD5, checksum: "3970992b02512b107ee6d682a0d1a233")
!1331 = !DINamespace(name: "{impl#32}", scope: !1131)
!1332 = !DISubroutineType(types: !1333)
!1333 = !{null, !1334}
!1334 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut alloc::rc::Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", baseType: !1130, size: 64, align: 64, dwarfAddressSpace: 0)
!1335 = !{!1328}
!1336 = !DILocation(line: 0, scope: !1329, inlinedAt: !1337)
!1337 = distinct !DILocation(line: 523, column: 1, scope: !1315, inlinedAt: !1323)
!1338 = !DILocalVariable(name: "self", arg: 1, scope: !1339, file: !1330, line: 355, type: !1334)
!1339 = distinct !DISubprogram(name: "inner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", linkageName: "_ZN5alloc2rc15Rc$LT$T$C$A$GT$5inner17haea7fea26cd47993E", scope: !1130, file: !1330, line: 355, type: !1340, scopeLine: 355, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1175, declaration: !1344, retainedNodes: !1345)
!1340 = !DISubroutineType(types: !1341)
!1341 = !{!1342, !1343}
!1342 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>", baseType: !1139, size: 64, align: 64, dwarfAddressSpace: 0)
!1343 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&alloc::rc::Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", baseType: !1130, size: 64, align: 64, dwarfAddressSpace: 0)
!1344 = !DISubprogram(name: "inner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>", linkageName: "_ZN5alloc2rc15Rc$LT$T$C$A$GT$5inner17haea7fea26cd47993E", scope: !1130, file: !1330, line: 355, type: !1340, scopeLine: 355, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1175)
!1345 = !{!1338}
!1346 = !DILocation(line: 0, scope: !1339, inlinedAt: !1347)
!1347 = distinct !DILocation(line: 2298, column: 18, scope: !1329, inlinedAt: !1337)
!1348 = !DILocation(line: 0, scope: !1339, inlinedAt: !1349)
!1349 = distinct !DILocation(line: 2299, column: 21, scope: !1329, inlinedAt: !1337)
!1350 = !DILocation(line: 428, column: 20, scope: !1351, inlinedAt: !1356)
!1351 = distinct !DISubprogram(name: "as_ref<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ref17h5cb1a5262b93dcb6E", scope: !1135, file: !564, line: 424, type: !1352, scopeLine: 424, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1168, declaration: !1355)
!1352 = !DISubroutineType(types: !1353)
!1353 = !{!1342, !1354}
!1354 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::ptr::non_null::NonNull<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", baseType: !1135, size: 64, align: 64, dwarfAddressSpace: 0)
!1355 = !DISubprogram(name: "as_ref<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ref17h5cb1a5262b93dcb6E", scope: !1135, file: !564, line: 424, type: !1352, scopeLine: 424, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1168)
!1356 = distinct !DILocation(line: 358, column: 27, scope: !1339, inlinedAt: !1347)
!1357 = !{!1325, !1311, !1296}
!1358 = !DILocalVariable(name: "self", arg: 1, scope: !1359, file: !1330, line: 3570, type: !1138)
!1359 = distinct !DISubprogram(name: "dec_strong<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", linkageName: "_ZN5alloc2rc10RcInnerPtr10dec_strong17h358caec5e7ad9a10E", scope: !1360, file: !1330, line: 3570, type: !1361, scopeLine: 3570, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1364, retainedNodes: !1363)
!1360 = !DINamespace(name: "RcInnerPtr", scope: !1131)
!1361 = !DISubroutineType(types: !1362)
!1362 = !{null, !1342}
!1363 = !{!1358}
!1364 = !{!1365}
!1365 = !DITemplateTypeParameter(name: "Self", type: !1139)
!1366 = !DILocation(line: 0, scope: !1359, inlinedAt: !1367)
!1367 = distinct !DILocation(line: 2298, column: 26, scope: !1329, inlinedAt: !1337)
!1368 = !DILocalVariable(name: "self", arg: 1, scope: !1369, file: !1330, line: 3542, type: !1138)
!1369 = distinct !DISubprogram(name: "strong<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>", linkageName: "_ZN5alloc2rc10RcInnerPtr6strong17h149fbb4d0b213758E", scope: !1360, file: !1330, line: 3542, type: !1370, scopeLine: 3542, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !1364, retainedNodes: !1372)
!1370 = !DISubroutineType(types: !1371)
!1371 = !{!9, !1342}
!1372 = !{!1368}
!1373 = !DILocation(line: 0, scope: !1369, inlinedAt: !1374)
!1374 = distinct !DILocation(line: 3571, column: 36, scope: !1359, inlinedAt: !1367)
!1375 = !DILocalVariable(name: "self", arg: 1, scope: !1376, file: !1225, line: 428, type: !1379)
!1376 = distinct !DISubprogram(name: "set<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$3set17hf5e22642556ec098E", scope: !1142, file: !1225, line: 428, type: !1377, scopeLine: 428, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !800, declaration: !1380, retainedNodes: !1381)
!1377 = !DISubroutineType(types: !1378)
!1378 = !{null, !1379, !9}
!1379 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::cell::Cell<usize>", baseType: !1142, size: 64, align: 64, dwarfAddressSpace: 0)
!1380 = !DISubprogram(name: "set<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$3set17hf5e22642556ec098E", scope: !1142, file: !1225, line: 428, type: !1377, scopeLine: 428, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !800)
!1381 = !{!1375, !1382}
!1382 = !DILocalVariable(name: "val", arg: 2, scope: !1376, file: !1225, line: 428, type: !9)
!1383 = !DILocation(line: 0, scope: !1376, inlinedAt: !1384)
!1384 = distinct !DILocation(line: 3571, column: 27, scope: !1359, inlinedAt: !1367)
!1385 = !DILocalVariable(name: "self", arg: 1, scope: !1386, file: !1225, line: 500, type: !1379)
!1386 = distinct !DISubprogram(name: "replace<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$7replace17h5dd5c40532223491E", scope: !1142, file: !1225, line: 500, type: !1387, scopeLine: 500, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !800, declaration: !1389, retainedNodes: !1390)
!1387 = !DISubroutineType(types: !1388)
!1388 = !{!9, !1379, !9}
!1389 = !DISubprogram(name: "replace<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$7replace17h5dd5c40532223491E", scope: !1142, file: !1225, line: 500, type: !1387, scopeLine: 500, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !800)
!1390 = !{!1385, !1391}
!1391 = !DILocalVariable(name: "val", arg: 2, scope: !1386, file: !1225, line: 500, type: !9)
!1392 = !DILocation(line: 0, scope: !1386, inlinedAt: !1393)
!1393 = distinct !DILocation(line: 429, column: 14, scope: !1376, inlinedAt: !1384)
!1394 = !DILocalVariable(name: "self", arg: 1, scope: !1395, file: !1225, line: 541, type: !1379)
!1395 = distinct !DISubprogram(name: "get<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$3get17hfccf9b0edd6bd65dE", scope: !1142, file: !1225, line: 541, type: !1396, scopeLine: 541, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !800, declaration: !1398, retainedNodes: !1399)
!1396 = !DISubroutineType(types: !1397)
!1397 = !{!9, !1379}
!1398 = !DISubprogram(name: "get<usize>", linkageName: "_ZN4core4cell13Cell$LT$T$GT$3get17hfccf9b0edd6bd65dE", scope: !1142, file: !1225, line: 541, type: !1396, scopeLine: 541, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !800)
!1399 = !{!1394}
!1400 = !DILocation(line: 0, scope: !1395, inlinedAt: !1401)
!1401 = distinct !DILocation(line: 3543, column: 27, scope: !1369, inlinedAt: !1374)
!1402 = !DILocation(line: 544, column: 18, scope: !1395, inlinedAt: !1401)
!1403 = !DILocation(line: 3571, column: 31, scope: !1359, inlinedAt: !1367)
!1404 = !DILocation(line: 865, column: 9, scope: !1405, inlinedAt: !1410)
!1405 = distinct !DISubprogram(name: "replace<usize>", linkageName: "_ZN4core3mem7replace17hba84a865d2017893E", scope: !481, file: !1406, line: 850, type: !1407, scopeLine: 850, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !800)
!1406 = !DIFile(filename: "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/mem/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "f1c5365a2f0a05a42c44d469d395c8a1")
!1407 = !DISubroutineType(types: !1408)
!1408 = !{!9, !1409, !9}
!1409 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut usize", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 0)
!1410 = distinct !DILocation(line: 503, column: 9, scope: !1386, inlinedAt: !1393)
!1411 = !DILocation(line: 0, scope: !1369, inlinedAt: !1412)
!1412 = distinct !DILocation(line: 2299, column: 29, scope: !1329, inlinedAt: !1337)
!1413 = !DILocation(line: 0, scope: !1395, inlinedAt: !1414)
!1414 = distinct !DILocation(line: 3543, column: 27, scope: !1415, inlinedAt: !1412)
!1415 = !DILexicalBlockFile(scope: !1369, file: !1330, discriminator: 2)
!1416 = !DILocation(line: 2299, column: 16, scope: !1329, inlinedAt: !1337)
!1417 = !DILocation(line: 2300, column: 17, scope: !1329, inlinedAt: !1337)
!1418 = !DILocation(line: 191, column: 43, scope: !1245, inlinedAt: !1255)
!1419 = !DILocation(line: 191, column: 21, scope: !1245, inlinedAt: !1255)
!1420 = !DILocation(line: 0, scope: !1253, inlinedAt: !1255)
!1421 = !DILocation(line: 192, column: 9, scope: !1253, inlinedAt: !1255)
!1422 = !DILocation(line: 203, column: 9, scope: !1180, inlinedAt: !1195)
!1423 = !DILocation(line: 0, scope: !1114, inlinedAt: !1178)
!1424 = !DILocation(line: 33, column: 5, scope: !1113, inlinedAt: !1178)
!1425 = !{!1426}
!1426 = distinct !{!1426, !1427, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hd497d5f9546794adE: %_1"}
!1427 = distinct !{!1427, !"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17hd497d5f9546794adE"}
!1428 = !DILocation(line: 0, scope: !1300, inlinedAt: !1429)
!1429 = distinct !DILocation(line: 34, column: 1, scope: !1114, inlinedAt: !1178)
!1430 = !{!1431}
!1431 = distinct !{!1431, !1432, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17hc39867958365ed55E: %_1"}
!1432 = distinct !{!1432, !"_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17hc39867958365ed55E"}
!1433 = !DILocation(line: 523, column: 1, scope: !1300, inlinedAt: !1429)
!1434 = !DILocation(line: 0, scope: !1315, inlinedAt: !1435)
!1435 = distinct !DILocation(line: 523, column: 1, scope: !1300, inlinedAt: !1429)
!1436 = !{!1437}
!1437 = distinct !{!1437, !1438, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfce35e40ab15106fE: %self"}
!1438 = distinct !{!1438, !"_ZN68_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfce35e40ab15106fE"}
!1439 = !DILocation(line: 523, column: 1, scope: !1315, inlinedAt: !1435)
!1440 = !DILocation(line: 0, scope: !1329, inlinedAt: !1441)
!1441 = distinct !DILocation(line: 523, column: 1, scope: !1315, inlinedAt: !1435)
!1442 = !DILocation(line: 0, scope: !1339, inlinedAt: !1443)
!1443 = distinct !DILocation(line: 2298, column: 18, scope: !1329, inlinedAt: !1441)
!1444 = !DILocation(line: 0, scope: !1339, inlinedAt: !1445)
!1445 = distinct !DILocation(line: 2299, column: 21, scope: !1329, inlinedAt: !1441)
!1446 = !DILocation(line: 428, column: 20, scope: !1351, inlinedAt: !1447)
!1447 = distinct !DILocation(line: 358, column: 27, scope: !1339, inlinedAt: !1443)
!1448 = !{!1437, !1431, !1426}
!1449 = !DILocation(line: 0, scope: !1359, inlinedAt: !1450)
!1450 = distinct !DILocation(line: 2298, column: 26, scope: !1329, inlinedAt: !1441)
!1451 = !DILocation(line: 0, scope: !1369, inlinedAt: !1452)
!1452 = distinct !DILocation(line: 3571, column: 36, scope: !1359, inlinedAt: !1450)
!1453 = !DILocation(line: 0, scope: !1376, inlinedAt: !1454)
!1454 = distinct !DILocation(line: 3571, column: 27, scope: !1359, inlinedAt: !1450)
!1455 = !DILocation(line: 0, scope: !1386, inlinedAt: !1456)
!1456 = distinct !DILocation(line: 429, column: 14, scope: !1376, inlinedAt: !1454)
!1457 = !DILocation(line: 0, scope: !1395, inlinedAt: !1458)
!1458 = distinct !DILocation(line: 3543, column: 27, scope: !1369, inlinedAt: !1452)
!1459 = !DILocation(line: 544, column: 18, scope: !1395, inlinedAt: !1458)
!1460 = !DILocation(line: 3571, column: 31, scope: !1359, inlinedAt: !1450)
!1461 = !DILocation(line: 865, column: 9, scope: !1405, inlinedAt: !1462)
!1462 = distinct !DILocation(line: 503, column: 9, scope: !1386, inlinedAt: !1456)
!1463 = !DILocation(line: 0, scope: !1369, inlinedAt: !1464)
!1464 = distinct !DILocation(line: 2299, column: 29, scope: !1329, inlinedAt: !1441)
!1465 = !DILocation(line: 0, scope: !1395, inlinedAt: !1466)
!1466 = distinct !DILocation(line: 3543, column: 27, scope: !1415, inlinedAt: !1464)
!1467 = !DILocation(line: 2299, column: 16, scope: !1329, inlinedAt: !1441)
!1468 = !DILocation(line: 2300, column: 17, scope: !1329, inlinedAt: !1441)
!1469 = !DILocation(line: 23, column: 1, scope: !1115, inlinedAt: !1178)
!1470 = !DILocalVariable(name: "cat", scope: !1471, file: !920, line: 61, type: !1100, align: 64)
!1471 = distinct !DILexicalBlock(scope: !1472, file: !920, line: 61, column: 5)
!1472 = distinct !DISubprogram(name: "static_dp", linkageName: "_ZN5dp_ex9static_dp17hdfe239b3264d42c0E", scope: !69, file: !920, line: 60, type: !21, scopeLine: 60, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !30, templateParams: !23, retainedNodes: !1473)
!1473 = !{!1470}
!1474 = !DILocation(line: 0, scope: !1471, inlinedAt: !1475)
!1475 = distinct !DILocation(line: 81, column: 5, scope: !1111)
!1476 = !DILocation(line: 0, scope: !1096, inlinedAt: !1477)
!1477 = distinct !DILocation(line: 62, column: 5, scope: !1471, inlinedAt: !1475)
!1478 = !DILocation(line: 0, scope: !1105, inlinedAt: !1479)
!1479 = distinct !DILocation(line: 19, column: 9, scope: !1096, inlinedAt: !1477)
!1480 = !DILocation(line: 19, column: 9, scope: !1096, inlinedAt: !1477)
!1481 = !DILocation(line: 602, column: 9, scope: !1105, inlinedAt: !1479)
!1482 = !DILocation(line: 82, column: 2, scope: !1111)
