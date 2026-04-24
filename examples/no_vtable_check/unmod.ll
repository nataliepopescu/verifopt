; ModuleID = '8vpau3st8l8djsqfdkpcm8j4u'
source_filename = "8vpau3st8l8djsqfdkpcm8j4u"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@vtable.0 = private constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0INtNtNtCshhmRwEtsTQ2_4core3ops8function6FnOnceuE9call_once6vtableCshm9upcCxmh6_15no_vtable_check, ptr @_RNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0Cshm9upcCxmh6_15no_vtable_check, ptr @_RNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0Cshm9upcCxmh6_15no_vtable_check }>, align 8, !dbg !0
@vtable.1 = private constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXCshm9upcCxmh6_15no_vtable_checkNtB2_8MyStructNtB2_7MyTrait10do_nothing }>, align 8, !dbg !24
@alloc_e3bcf19cc2379e63cf4ae2edddacd581 = private unnamed_addr constant [14 x i8] c"doing nothing\0A", align 1
@__rustc_debug_gdb_scripts_section__ = linkonce_odr unnamed_addr constant [34 x i8] c"\01gdb_load_rust_pretty_printers.py\00", section ".debug_gdb_scripts", align 1

; std::rt::lang_start::<()>
; Function Attrs: nonlazybind uwtable
define hidden i64 @_RINvNtCsefmIBSMl6ne_3std2rt10lang_startuECshm9upcCxmh6_15no_vtable_check(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #0 !dbg !43 {
start:
  %sigpipe.dbg.spill = alloca [1 x i8], align 1
  %argv.dbg.spill = alloca [8 x i8], align 8
  %argc.dbg.spill = alloca [8 x i8], align 8
  %main.dbg.spill = alloca [8 x i8], align 8
  %_7 = alloca [8 x i8], align 8
  store ptr %main, ptr %main.dbg.spill, align 8
    #dbg_declare(ptr %main.dbg.spill, !52, !DIExpression(), !58)
  store i64 %argc, ptr %argc.dbg.spill, align 8
    #dbg_declare(ptr %argc.dbg.spill, !53, !DIExpression(), !59)
  store ptr %argv, ptr %argv.dbg.spill, align 8
    #dbg_declare(ptr %argv.dbg.spill, !54, !DIExpression(), !60)
  store i8 %sigpipe, ptr %sigpipe.dbg.spill, align 1
    #dbg_declare(ptr %sigpipe.dbg.spill, !55, !DIExpression(), !61)
  store ptr %main, ptr %_7, align 8, !dbg !62
; call std::rt::lang_start_internal
  %_0 = call i64 @_RNvNtCsefmIBSMl6ne_3std2rt19lang_start_internal(ptr align 1 %_7, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe), !dbg !63
  ret i64 %_0, !dbg !64
}

; std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
; Function Attrs: noinline nonlazybind uwtable
define internal void @_RINvNtNtCsefmIBSMl6ne_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshm9upcCxmh6_15no_vtable_check(ptr %f) unnamed_addr #1 !dbg !65 {
start:
  %dummy.dbg.spill = alloca [0 x i8], align 1
  %f.dbg.spill = alloca [8 x i8], align 8
  %result.dbg.spill = alloca [0 x i8], align 1
    #dbg_declare(ptr %result.dbg.spill, !73, !DIExpression(), !77)
  store ptr %f, ptr %f.dbg.spill, align 8
    #dbg_declare(ptr %f.dbg.spill, !72, !DIExpression(), !78)
    #dbg_declare(ptr %dummy.dbg.spill, !79, !DIExpression(), !87)
; call <fn() as core::ops::function::FnOnce<()>>::call_once
  call void @_RNvYFEuINtNtNtCshhmRwEtsTQ2_4core3ops8function6FnOnceuE9call_onceCshm9upcCxmh6_15no_vtable_check(ptr %f) #5, !dbg !89
  call void asm sideeffect "", "~{memory}"(), !dbg !90, !srcloc !91
  ret void, !dbg !92
}

; std::rt::lang_start::<()>::{closure#0}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_RNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0Cshm9upcCxmh6_15no_vtable_check(ptr align 8 %_1) unnamed_addr #2 !dbg !93 {
start:
  %self.dbg.spill = alloca [1 x i8], align 1
  %_1.dbg.spill = alloca [8 x i8], align 8
  store ptr %_1, ptr %_1.dbg.spill, align 8
    #dbg_declare(ptr %_1.dbg.spill, !99, !DIExpression(DW_OP_deref), !100)
  %_4 = load ptr, ptr %_1, align 8, !dbg !101
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  call void @_RINvNtNtCsefmIBSMl6ne_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshm9upcCxmh6_15no_vtable_check(ptr %_4) #6, !dbg !102
; call <() as std::process::Termination>::report
  %self = call i8 @_RNvXsZ_NtCsefmIBSMl6ne_3std7processuNtB5_11Termination6reportCshm9upcCxmh6_15no_vtable_check() #5, !dbg !103
  store i8 %self, ptr %self.dbg.spill, align 1, !dbg !103
    #dbg_declare(ptr %self.dbg.spill, !104, !DIExpression(), !121)
  %_0 = zext i8 %self to i32, !dbg !123
  ret i32 %_0, !dbg !131
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_RNSNvYNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0INtNtNtCshhmRwEtsTQ2_4core3ops8function6FnOnceuE9call_once6vtableCshm9upcCxmh6_15no_vtable_check(ptr %_1) unnamed_addr #2 !dbg !132 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
    #dbg_declare(ptr %_1.dbg.spill, !141, !DIExpression(), !146)
    #dbg_declare(ptr %_2, !142, !DIExpression(), !146)
  %0 = load ptr, ptr %_1, align 8, !dbg !146
; call <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once
  %_0 = call i32 @_RNvYNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0INtNtNtCshhmRwEtsTQ2_4core3ops8function6FnOnceuE9call_onceCshm9upcCxmh6_15no_vtable_check(ptr %0) #5, !dbg !146
  ret i32 %_0, !dbg !146
}

; no_vtable_check::main
; Function Attrs: nonlazybind uwtable
define hidden void @_RNvCshm9upcCxmh6_15no_vtable_check4main() unnamed_addr #0 !dbg !147 {
start:
  %_s.dbg.spill = alloca [16 x i8], align 8
  store ptr inttoptr (i64 1 to ptr), ptr %_s.dbg.spill, align 8, !dbg !162
  %0 = getelementptr inbounds i8, ptr %_s.dbg.spill, i64 8, !dbg !162
  store ptr @vtable.1, ptr %0, align 8, !dbg !162
    #dbg_declare(ptr %_s.dbg.spill, !150, !DIExpression(), !163)
  ret void, !dbg !164
}

; <core::fmt::Arguments>::from_str
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, ptr } @_RNvMs4_NtCshhmRwEtsTQ2_4core3fmtNtB5_9Arguments8from_strCshm9upcCxmh6_15no_vtable_check(ptr align 1 %s.0, i64 %s.1) unnamed_addr #2 !dbg !165 {
start:
  %s.dbg.spill = alloca [16 x i8], align 8
  store ptr %s.0, ptr %s.dbg.spill, align 8
  %0 = getelementptr inbounds i8, ptr %s.dbg.spill, i64 8
  store i64 %s.1, ptr %0, align 8
    #dbg_declare(ptr %s.dbg.spill, !264, !DIExpression(), !265)
    #dbg_declare(ptr %s.dbg.spill, !266, !DIExpression(), !274)
    #dbg_declare(ptr %s.dbg.spill, !276, !DIExpression(), !281)
    #dbg_declare(ptr %s.dbg.spill, !283, !DIExpression(), !292)
  %_6 = shl i64 %s.1, 1, !dbg !294
  %_5 = or i64 %_6, 1, !dbg !294
  %_4 = inttoptr i64 %_5 to ptr, !dbg !295
  %1 = insertvalue { ptr, ptr } poison, ptr %s.0, 0, !dbg !296
  %2 = insertvalue { ptr, ptr } %1, ptr %_4, 1, !dbg !296
  ret { ptr, ptr } %2, !dbg !296
}

; <no_vtable_check::MyStruct as no_vtable_check::MyTrait>::do_nothing
; Function Attrs: nonlazybind uwtable
define internal void @_RNvXCshm9upcCxmh6_15no_vtable_checkNtB2_8MyStructNtB2_7MyTrait10do_nothing(ptr align 1 %self) unnamed_addr #0 !dbg !297 {
start:
  %self.dbg.spill = alloca [8 x i8], align 8
  store ptr %self, ptr %self.dbg.spill, align 8
    #dbg_declare(ptr %self.dbg.spill, !303, !DIExpression(), !304)
; call <core::fmt::Arguments>::from_str
  %0 = call { ptr, ptr } @_RNvMs4_NtCshhmRwEtsTQ2_4core3fmtNtB5_9Arguments8from_strCshm9upcCxmh6_15no_vtable_check(ptr align 1 @alloc_e3bcf19cc2379e63cf4ae2edddacd581, i64 14) #5, !dbg !305
  %_3.0 = extractvalue { ptr, ptr } %0, 0, !dbg !305
  %_3.1 = extractvalue { ptr, ptr } %0, 1, !dbg !305
; call std::io::stdio::_print
  call void @_RNvNtNtCsefmIBSMl6ne_3std2io5stdio6__print(ptr %_3.0, ptr %_3.1), !dbg !305
  ret void, !dbg !306
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @_RNvXsZ_NtCsefmIBSMl6ne_3std7processuNtB5_11Termination6reportCshm9upcCxmh6_15no_vtable_check() unnamed_addr #2 !dbg !307 {
start:
  %_1.dbg.spill = alloca [0 x i8], align 1
    #dbg_declare(ptr %_1.dbg.spill, !312, !DIExpression(), !313)
  ret i8 0, !dbg !314
}

; <fn() as core::ops::function::FnOnce<()>>::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_RNvYFEuINtNtNtCshhmRwEtsTQ2_4core3ops8function6FnOnceuE9call_onceCshm9upcCxmh6_15no_vtable_check(ptr %_1) unnamed_addr #2 !dbg !315 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
    #dbg_declare(ptr %_1.dbg.spill, !317, !DIExpression(), !321)
    #dbg_declare(ptr %_2, !318, !DIExpression(), !321)
  call void %_1(), !dbg !321
  ret void, !dbg !321
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_RNvYNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0INtNtNtCshhmRwEtsTQ2_4core3ops8function6FnOnceuE9call_onceCshm9upcCxmh6_15no_vtable_check(ptr %0) unnamed_addr #2 personality ptr @rust_eh_personality !dbg !322 {
start:
  %1 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [8 x i8], align 8
  store ptr %0, ptr %_1, align 8
    #dbg_declare(ptr %_1, !326, !DIExpression(), !328)
    #dbg_declare(ptr %_2, !327, !DIExpression(), !328)
; invoke std::rt::lang_start::<()>::{closure#0}
  %_0 = invoke i32 @_RNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0Cshm9upcCxmh6_15no_vtable_check(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup, !dbg !328

bb3:                                              ; preds = %cleanup
  %2 = load ptr, ptr %1, align 8, !dbg !328
  %3 = getelementptr inbounds i8, ptr %1, i64 8, !dbg !328
  %4 = load i32, ptr %3, align 8, !dbg !328
  %5 = insertvalue { ptr, i32 } poison, ptr %2, 0, !dbg !328
  %6 = insertvalue { ptr, i32 } %5, i32 %4, 1, !dbg !328
  resume { ptr, i32 } %6, !dbg !328

cleanup:                                          ; preds = %start
  %7 = landingpad { ptr, i32 }
          cleanup
  %8 = extractvalue { ptr, i32 } %7, 0
  %9 = extractvalue { ptr, i32 } %7, 1
  store ptr %8, ptr %1, align 8
  %10 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %9, ptr %10, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %_0, !dbg !328
}

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_RNvNtCsefmIBSMl6ne_3std2rt19lang_start_internal(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_RNvNtNtCsefmIBSMl6ne_3std2io5stdio6__print(ptr, ptr) unnamed_addr #0

; Function Attrs: nounwind nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #3

; Function Attrs: nonlazybind
define i32 @main(i32 %0, ptr %1) unnamed_addr #4 {
top:
  %2 = load volatile i8, ptr @__rustc_debug_gdb_scripts_section__, align 1
  %3 = sext i32 %0 to i64
; call std::rt::lang_start::<()>
  %4 = call i64 @_RINvNtCsefmIBSMl6ne_3std2rt10lang_startuECshm9upcCxmh6_15no_vtable_check(ptr @_RNvCshm9upcCxmh6_15no_vtable_check4main, i64 %3, ptr %1, i8 0)
  %5 = trunc i64 %4 to i32
  ret i32 %5
}

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #4 = { nonlazybind "target-cpu"="x86-64" }
attributes #5 = { inlinehint }
attributes #6 = { noinline }

!llvm.module.flags = !{!34, !35, !36, !37, !38}
!llvm.ident = !{!39}
!llvm.dbg.cu = !{!40}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable}", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable_type}", file: !2, size: 384, align: 64, flags: DIFlagArtificial, elements: !4, vtableHolder: !14, templateParams: !23, identifier: "44c99e9a2f1f1a2fa6b3433a5a25ed3b")
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
!14 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<()>", scope: !15, file: !2, size: 64, align: 64, elements: !18, templateParams: !23, identifier: "2c605c41b8c3cf1a90ad2eabd3753c19")
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
!25 = distinct !DIGlobalVariable(name: "<no_vtable_check::MyStruct as no_vtable_check::MyTrait>::{vtable}", scope: null, file: !2, type: !26, isLocal: true, isDefinition: true)
!26 = !DICompositeType(tag: DW_TAG_structure_type, name: "<no_vtable_check::MyStruct as no_vtable_check::MyTrait>::{vtable_type}", file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !27, vtableHolder: !32, templateParams: !23, identifier: "56cec68033d7453bfc15e2c33abd8c47")
!27 = !{!28, !29, !30, !31}
!28 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !26, file: !2, baseType: !6, size: 64, align: 64)
!29 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !26, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!30 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !26, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!31 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !26, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!32 = !DICompositeType(tag: DW_TAG_structure_type, name: "MyStruct", scope: !33, file: !2, align: 8, flags: DIFlagPrivate, elements: !23, identifier: "ecbced60c673847fe470e01c06113abe")
!33 = !DINamespace(name: "no_vtable_check", scope: null)
!34 = !{i32 8, !"PIC Level", i32 2}
!35 = !{i32 7, !"PIE Level", i32 2}
!36 = !{i32 2, !"RtLibUseGOT", i32 1}
!37 = !{i32 7, !"Dwarf Version", i32 4}
!38 = !{i32 2, !"Debug Info Version", i32 3}
!39 = !{!"rustc version 1.94.0-nightly (2f1bd3f37 2026-01-12)"}
!40 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !41, producer: "clang LLVM (rustc version 1.94.0-nightly (2f1bd3f37 2026-01-12))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !42, splitDebugInlining: false, nameTableKind: None)
!41 = !DIFile(filename: "src/main.rs/@/8vpau3st8l8djsqfdkpcm8j4u", directory: "/home/np/hack/verifopt/examples/no_vtable_check")
!42 = !{!0, !24}
!43 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_RINvNtCsefmIBSMl6ne_3std2rt10lang_startuECshm9upcCxmh6_15no_vtable_check", scope: !16, file: !44, line: 199, type: !45, scopeLine: 199, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !56, retainedNodes: !51)
!44 = !DIFile(filename: "/home/np/.rustup/toolchains/nightly-2026-01-13-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "cdcddc2bc903963a099042437d601c5f")
!45 = !DISubroutineType(types: !46)
!46 = !{!47, !20, !47, !48, !50}
!47 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!48 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !49, size: 64, align: 64, dwarfAddressSpace: 0)
!49 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !50, size: 64, align: 64, dwarfAddressSpace: 0)
!50 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!51 = !{!52, !53, !54, !55}
!52 = !DILocalVariable(name: "main", arg: 1, scope: !43, file: !44, line: 200, type: !20)
!53 = !DILocalVariable(name: "argc", arg: 2, scope: !43, file: !44, line: 201, type: !47)
!54 = !DILocalVariable(name: "argv", arg: 3, scope: !43, file: !44, line: 202, type: !48)
!55 = !DILocalVariable(name: "sigpipe", arg: 4, scope: !43, file: !44, line: 203, type: !50)
!56 = !{!57}
!57 = !DITemplateTypeParameter(name: "T", type: !7)
!58 = !DILocation(line: 200, column: 5, scope: !43)
!59 = !DILocation(line: 201, column: 5, scope: !43)
!60 = !DILocation(line: 202, column: 5, scope: !43)
!61 = !DILocation(line: 203, column: 5, scope: !43)
!62 = !DILocation(line: 206, column: 10, scope: !43)
!63 = !DILocation(line: 205, column: 5, scope: !43)
!64 = !DILocation(line: 211, column: 2, scope: !43)
!65 = distinct !DISubprogram(name: "__rust_begin_short_backtrace<fn(), ()>", linkageName: "_RINvNtNtCsefmIBSMl6ne_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshm9upcCxmh6_15no_vtable_check", scope: !67, file: !66, line: 162, type: !69, scopeLine: 162, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !75, retainedNodes: !71)
!66 = !DIFile(filename: "/home/np/.rustup/toolchains/nightly-2026-01-13-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs", directory: "", checksumkind: CSK_MD5, checksum: "0469076862be40bd9e65965440a24fae")
!67 = !DINamespace(name: "backtrace", scope: !68)
!68 = !DINamespace(name: "sys", scope: !17)
!69 = !DISubroutineType(types: !70)
!70 = !{null, !20}
!71 = !{!72, !73}
!72 = !DILocalVariable(name: "f", arg: 1, scope: !65, file: !66, line: 162, type: !20)
!73 = !DILocalVariable(name: "result", scope: !74, file: !66, line: 166, type: !7, align: 8)
!74 = distinct !DILexicalBlock(scope: !65, file: !66, line: 166, column: 5)
!75 = !{!76, !57}
!76 = !DITemplateTypeParameter(name: "F", type: !20)
!77 = !DILocation(line: 166, column: 9, scope: !74)
!78 = !DILocation(line: 162, column: 43, scope: !65)
!79 = !DILocalVariable(name: "dummy", scope: !80, file: !81, line: 481, type: !7, align: 8)
!80 = distinct !DISubprogram(name: "black_box<()>", linkageName: "_RINvNtCshhmRwEtsTQ2_4core4hint9black_boxuECshm9upcCxmh6_15no_vtable_check", scope: !82, file: !81, line: 481, type: !84, scopeLine: 481, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !56, retainedNodes: !86)
!81 = !DIFile(filename: "/home/np/.rustup/toolchains/nightly-2026-01-13-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/hint.rs", directory: "", checksumkind: CSK_MD5, checksum: "3abb805aa2da56dbef8488b9d76c6025")
!82 = !DINamespace(name: "hint", scope: !83)
!83 = !DINamespace(name: "core", scope: null)
!84 = !DISubroutineType(types: !85)
!85 = !{null, !7}
!86 = !{!79}
!87 = !DILocation(line: 481, column: 27, scope: !80, inlinedAt: !88)
!88 = !DILocation(line: 169, column: 5, scope: !74)
!89 = !DILocation(line: 166, column: 18, scope: !65)
!90 = !DILocation(line: 482, column: 5, scope: !80, inlinedAt: !88)
!91 = !{i64 5790835687068313}
!92 = !DILocation(line: 172, column: 2, scope: !65)
!93 = distinct !DISubprogram(name: "{closure#0}<()>", linkageName: "_RNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0Cshm9upcCxmh6_15no_vtable_check", scope: !15, file: !44, line: 206, type: !94, scopeLine: 206, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !56, retainedNodes: !98)
!94 = !DISubroutineType(types: !95)
!95 = !{!96, !97}
!96 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!97 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!98 = !{!99}
!99 = !DILocalVariable(name: "main", scope: !93, file: !44, line: 200, type: !20, align: 64)
!100 = !DILocation(line: 200, column: 5, scope: !93)
!101 = !DILocation(line: 206, column: 70, scope: !93)
!102 = !DILocation(line: 206, column: 18, scope: !93)
!103 = !DILocation(line: 206, column: 76, scope: !93)
!104 = !DILocalVariable(name: "self", arg: 1, scope: !105, file: !106, line: 2183, type: !107)
!105 = distinct !DISubprogram(name: "to_i32", linkageName: "_RNvMsT_NtCsefmIBSMl6ne_3std7processNtB5_8ExitCode6to_i32Cshm9upcCxmh6_15no_vtable_check", scope: !107, file: !106, line: 2183, type: !117, scopeLine: 2183, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !23, declaration: !119, retainedNodes: !120)
!106 = !DIFile(filename: "/home/np/.rustup/toolchains/nightly-2026-01-13-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/process.rs", directory: "", checksumkind: CSK_MD5, checksum: "a9965e4b9461ba3ac716cb006e954a28")
!107 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !108, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !109, templateParams: !23, identifier: "2b1f5de74c9c6b313b009fdba3deea46")
!108 = !DINamespace(name: "process", scope: !17)
!109 = !{!110}
!110 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !107, file: !2, baseType: !111, size: 8, align: 8, flags: DIFlagPrivate)
!111 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !112, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !115, templateParams: !23, identifier: "dea100e3340e6c7af8177f7367606540")
!112 = !DINamespace(name: "common", scope: !113)
!113 = !DINamespace(name: "unix", scope: !114)
!114 = !DINamespace(name: "process", scope: !68)
!115 = !{!116}
!116 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !111, file: !2, baseType: !50, size: 8, align: 8, flags: DIFlagPrivate)
!117 = !DISubroutineType(types: !118)
!118 = !{!96, !107}
!119 = !DISubprogram(name: "to_i32", linkageName: "_RNvMsT_NtCsefmIBSMl6ne_3std7processNtB5_8ExitCode6to_i32Cshm9upcCxmh6_15no_vtable_check", scope: !107, file: !106, line: 2183, type: !117, scopeLine: 2183, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!120 = !{!104}
!121 = !DILocation(line: 2183, column: 19, scope: !105, inlinedAt: !122)
!122 = !DILocation(line: 206, column: 85, scope: !93)
!123 = !DILocation(line: 588, column: 9, scope: !124, inlinedAt: !130)
!124 = distinct !DISubprogram(name: "as_i32", linkageName: "_RNvMs8_NtNtNtNtCsefmIBSMl6ne_3std3sys7process4unix6commonNtB5_8ExitCode6as_i32Cshm9upcCxmh6_15no_vtable_check", scope: !111, file: !125, line: 587, type: !126, scopeLine: 587, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !23, declaration: !129)
!125 = !DIFile(filename: "/home/np/.rustup/toolchains/nightly-2026-01-13-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/process/unix/common.rs", directory: "", checksumkind: CSK_MD5, checksum: "800043af4cb7280de1627c2d4895ccde")
!126 = !DISubroutineType(types: !127)
!127 = !{!96, !128}
!128 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::sys::process::unix::common::ExitCode", baseType: !111, size: 64, align: 64, dwarfAddressSpace: 0)
!129 = !DISubprogram(name: "as_i32", linkageName: "_RNvMs8_NtNtNtNtCsefmIBSMl6ne_3std3sys7process4unix6commonNtB5_8ExitCode6as_i32Cshm9upcCxmh6_15no_vtable_check", scope: !111, file: !125, line: 587, type: !126, scopeLine: 587, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!130 = !DILocation(line: 2184, column: 16, scope: !105, inlinedAt: !122)
!131 = !DILocation(line: 206, column: 93, scope: !93)
!132 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_RNSNvYNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0INtNtNtCshhmRwEtsTQ2_4core3ops8function6FnOnceuE9call_once6vtableCshm9upcCxmh6_15no_vtable_check", scope: !134, file: !133, line: 250, type: !137, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !143, retainedNodes: !140)
!133 = !DIFile(filename: "/home/np/.rustup/toolchains/nightly-2026-01-13-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs", directory: "", checksumkind: CSK_MD5, checksum: "7165aec212fc528edf645f7f5c1c91bb")
!134 = !DINamespace(name: "FnOnce", scope: !135)
!135 = !DINamespace(name: "function", scope: !136)
!136 = !DINamespace(name: "ops", scope: !83)
!137 = !DISubroutineType(types: !138)
!138 = !{!96, !139}
!139 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!140 = !{!141, !142}
!141 = !DILocalVariable(arg: 1, scope: !132, file: !133, line: 250, type: !139)
!142 = !DILocalVariable(arg: 2, scope: !132, file: !133, line: 250, type: !7)
!143 = !{!144, !145}
!144 = !DITemplateTypeParameter(name: "Self", type: !14)
!145 = !DITemplateTypeParameter(name: "Args", type: !7)
!146 = !DILocation(line: 250, column: 5, scope: !132)
!147 = distinct !DISubprogram(name: "main", linkageName: "_RNvCshm9upcCxmh6_15no_vtable_check4main", scope: !33, file: !148, line: 13, type: !21, scopeLine: 13, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagMainSubprogram, unit: !40, templateParams: !23, retainedNodes: !149)
!148 = !DIFile(filename: "src/main.rs", directory: "/home/np/hack/verifopt/examples/no_vtable_check", checksumkind: CSK_MD5, checksum: "5373a823c34eba1505fda20513f05b3d")
!149 = !{!150}
!150 = !DILocalVariable(name: "_s", scope: !151, file: !148, line: 14, type: !152, align: 64)
!151 = distinct !DILexicalBlock(scope: !147, file: !148, line: 14, column: 5)
!152 = !DICompositeType(tag: DW_TAG_structure_type, name: "&dyn no_vtable_check::MyTrait", file: !2, size: 128, align: 64, elements: !153, templateParams: !23, identifier: "f0389921042d0f7c31b1d8b4de669d2c")
!153 = !{!154, !157}
!154 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !152, file: !2, baseType: !155, size: 64, align: 64)
!155 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !156, size: 64, align: 64, dwarfAddressSpace: 0)
!156 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn no_vtable_check::MyTrait", file: !2, align: 8, elements: !23, identifier: "4abf8bb87f9d9e0f391d3caf4f76c5e7")
!157 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !152, file: !2, baseType: !158, size: 64, align: 64, offset: 64)
!158 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 4]", baseType: !159, size: 64, align: 64, dwarfAddressSpace: 0)
!159 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 256, align: 64, elements: !160)
!160 = !{!161}
!161 = !DISubrange(count: 4, lowerBound: 0)
!162 = !DILocation(line: 14, column: 28, scope: !147)
!163 = !DILocation(line: 14, column: 9, scope: !151)
!164 = !DILocation(line: 15, column: 2, scope: !147)
!165 = distinct !DISubprogram(name: "from_str", linkageName: "_RNvMs4_NtCshhmRwEtsTQ2_4core3fmtNtB5_9Arguments8from_strCshm9upcCxmh6_15no_vtable_check", scope: !167, file: !166, line: 815, type: !255, scopeLine: 815, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !23, declaration: !262, retainedNodes: !263)
!166 = !DIFile(filename: "/home/np/.rustup/toolchains/nightly-2026-01-13-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "66c54229528687a22c80807f245fa4df")
!167 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !168, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !169, templateParams: !23, identifier: "57d90a2427fc01502abe506c295775c8")
!168 = !DINamespace(name: "fmt", scope: !83)
!169 = !{!170, !178}
!170 = !DIDerivedType(tag: DW_TAG_member, name: "template", scope: !167, file: !2, baseType: !171, size: 64, align: 64, flags: DIFlagPrivate)
!171 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<u8>", scope: !172, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !174, templateParams: !176, identifier: "99656cd40a9f80545a094afedf9186aa")
!172 = !DINamespace(name: "non_null", scope: !173)
!173 = !DINamespace(name: "ptr", scope: !83)
!174 = !{!175}
!175 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !171, file: !2, baseType: !49, size: 64, align: 64, flags: DIFlagPrivate)
!176 = !{!177}
!177 = !DITemplateTypeParameter(name: "T", type: !50)
!178 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !167, file: !2, baseType: !179, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!179 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<core::fmt::rt::Argument>", scope: !172, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !180, templateParams: !253, identifier: "f10d5340d11d8aab1001b19d7e26b818")
!180 = !{!181}
!181 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !179, file: !2, baseType: !182, size: 64, align: 64, flags: DIFlagPrivate)
!182 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const core::fmt::rt::Argument", baseType: !183, size: 64, align: 64, dwarfAddressSpace: 0)
!183 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !184, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !185, templateParams: !23, identifier: "b2ebffab55b4fb7d7b38dcc103e66dfa")
!184 = !DINamespace(name: "rt", scope: !168)
!185 = !{!186}
!186 = !DIDerivedType(tag: DW_TAG_member, name: "ty", scope: !183, file: !2, baseType: !187, size: 128, align: 64, flags: DIFlagPrivate)
!187 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentType", scope: !184, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !188, templateParams: !23, identifier: "8d3a6047cfb87849bf9324b999ec942f")
!188 = !{!189}
!189 = !DICompositeType(tag: DW_TAG_variant_part, scope: !187, file: !2, size: 128, align: 64, elements: !190, templateParams: !23, identifier: "637ca2782781da3b67e4f01badb3bda9", discriminator: !251)
!190 = !{!191, !247}
!191 = !DIDerivedType(tag: DW_TAG_member, name: "Placeholder", scope: !189, file: !2, baseType: !192, size: 128, align: 64)
!192 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !187, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !193, templateParams: !23, identifier: "7066d80c770f8df9f3380e426e263b60")
!193 = !{!194, !198, !241}
!194 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !192, file: !2, baseType: !195, size: 64, align: 64, flags: DIFlagPrivate)
!195 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<()>", scope: !172, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !196, templateParams: !56, identifier: "447ec9d26e10e531e8f1e197616451e0")
!196 = !{!197}
!197 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !195, file: !2, baseType: !6, size: 64, align: 64, flags: DIFlagPrivate)
!198 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !192, file: !2, baseType: !199, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!199 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "unsafe fn(core::ptr::non_null::NonNull<()>, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !200, size: 64, align: 64, dwarfAddressSpace: 0)
!200 = !DISubroutineType(types: !201)
!201 = !{!202, !195, !219}
!202 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), core::fmt::Error>", scope: !203, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !204, templateParams: !23, identifier: "851f5b2e432414b2e7b3db86b4a1a8b4")
!203 = !DINamespace(name: "result", scope: !83)
!204 = !{!205}
!205 = !DICompositeType(tag: DW_TAG_variant_part, scope: !202, file: !2, size: 8, align: 8, elements: !206, templateParams: !23, identifier: "7f34f670cc7e5a709c598bd05139259", discriminator: !218)
!206 = !{!207, !214}
!207 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !205, file: !2, baseType: !208, size: 8, align: 8, extraData: i8 0)
!208 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !202, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !209, templateParams: !211, identifier: "139d5784a6bd79192f014a871d56cf48")
!209 = !{!210}
!210 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !208, file: !2, baseType: !7, align: 8, offset: 8, flags: DIFlagPublic)
!211 = !{!57, !212}
!212 = !DITemplateTypeParameter(name: "E", type: !213)
!213 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !168, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "d518cfd78b9122349a18c4d7288cbf03")
!214 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !205, file: !2, baseType: !215, size: 8, align: 8, extraData: i8 1)
!215 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !202, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !216, templateParams: !211, identifier: "e6f863c84697f76810826bd246fde978")
!216 = !{!217}
!217 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !215, file: !2, baseType: !213, align: 8, offset: 8, flags: DIFlagPublic)
!218 = !DIDerivedType(tag: DW_TAG_member, scope: !202, file: !2, baseType: !50, size: 8, align: 8, flags: DIFlagArtificial)
!219 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !220, size: 64, align: 64, dwarfAddressSpace: 0)
!220 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !168, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !221, templateParams: !23, identifier: "f12cf912c7a4bf9a3b0badb9f06a9eed")
!221 = !{!222, !230}
!222 = !DIDerivedType(tag: DW_TAG_member, name: "options", scope: !220, file: !2, baseType: !223, size: 64, align: 32, offset: 128, flags: DIFlagPrivate)
!223 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormattingOptions", scope: !168, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !224, templateParams: !23, identifier: "253c31bdce0fe7e6b0527dfecc6f05ef")
!224 = !{!225, !227, !229}
!225 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !223, file: !2, baseType: !226, size: 32, align: 32, flags: DIFlagPrivate)
!226 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!227 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !223, file: !2, baseType: !228, size: 16, align: 16, offset: 32, flags: DIFlagPrivate)
!228 = !DIBasicType(name: "u16", size: 16, encoding: DW_ATE_unsigned)
!229 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !223, file: !2, baseType: !228, size: 16, align: 16, offset: 48, flags: DIFlagPrivate)
!230 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !220, file: !2, baseType: !231, size: 128, align: 64, flags: DIFlagPrivate)
!231 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !2, size: 128, align: 64, elements: !232, templateParams: !23, identifier: "cd8eb8611e7ba10d74e7e1ce24a69117")
!232 = !{!233, !236}
!233 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !231, file: !2, baseType: !234, size: 64, align: 64)
!234 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !235, size: 64, align: 64, dwarfAddressSpace: 0)
!235 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !2, align: 8, elements: !23, identifier: "8a8fe7be3efc22cb2392cf1c20840095")
!236 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !231, file: !2, baseType: !237, size: 64, align: 64, offset: 64)
!237 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 6]", baseType: !238, size: 64, align: 64, dwarfAddressSpace: 0)
!238 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 384, align: 64, elements: !239)
!239 = !{!240}
!240 = !DISubrange(count: 6, lowerBound: 0)
!241 = !DIDerivedType(tag: DW_TAG_member, name: "_lifetime", scope: !192, file: !2, baseType: !242, align: 8, offset: 128, flags: DIFlagPrivate)
!242 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&()>", scope: !243, file: !2, align: 8, flags: DIFlagPublic, elements: !23, templateParams: !244, identifier: "f70b4bf177af8c0e1a5fa2b29aa1afd3")
!243 = !DINamespace(name: "marker", scope: !83)
!244 = !{!245}
!245 = !DITemplateTypeParameter(name: "T", type: !246)
!246 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&()", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 0)
!247 = !DIDerivedType(tag: DW_TAG_member, name: "Count", scope: !189, file: !2, baseType: !248, size: 128, align: 64, extraData: i64 0)
!248 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !187, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !249, templateParams: !23, identifier: "581f4ac5934e2962c73d67b384d5b4de")
!249 = !{!250}
!250 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !248, file: !2, baseType: !228, size: 16, align: 16, offset: 64, flags: DIFlagPrivate)
!251 = !DIDerivedType(tag: DW_TAG_member, scope: !187, file: !2, baseType: !252, size: 64, align: 64, flags: DIFlagArtificial)
!252 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!253 = !{!254}
!254 = !DITemplateTypeParameter(name: "T", type: !183)
!255 = !DISubroutineType(types: !256)
!256 = !{!167, !257}
!257 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 128, align: 64, elements: !258, templateParams: !23, identifier: "9277eecd40495f85161460476aacc992")
!258 = !{!259, !261}
!259 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !257, file: !2, baseType: !260, size: 64, align: 64)
!260 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !50, size: 64, align: 64, dwarfAddressSpace: 0)
!261 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !257, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!262 = !DISubprogram(name: "from_str", linkageName: "_RNvMs4_NtCshhmRwEtsTQ2_4core3fmtNtB5_9Arguments8from_strCshm9upcCxmh6_15no_vtable_check", scope: !167, file: !166, line: 815, type: !255, scopeLine: 815, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!263 = !{!264}
!264 = !DILocalVariable(name: "s", arg: 1, scope: !165, file: !166, line: 815, type: !257)
!265 = !DILocation(line: 815, column: 27, scope: !165)
!266 = !DILocalVariable(name: "self", arg: 1, scope: !267, file: !268, line: 562, type: !257)
!267 = distinct !DISubprogram(name: "as_ptr", linkageName: "_RNvMNtCshhmRwEtsTQ2_4core3stre6as_ptr", scope: !269, file: !268, line: 562, type: !271, scopeLine: 562, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !23, retainedNodes: !273)
!268 = !DIFile(filename: "/home/np/.rustup/toolchains/nightly-2026-01-13-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "1cc8f631cc94b123ec97ae0e0b2603be")
!269 = !DINamespace(name: "{impl#0}", scope: !270)
!270 = !DINamespace(name: "str", scope: !83)
!271 = !DISubroutineType(types: !272)
!272 = !{!49, !257}
!273 = !{!266}
!274 = !DILocation(line: 562, column: 25, scope: !267, inlinedAt: !275)
!275 = !DILocation(line: 819, column: 44, scope: !165)
!276 = !DILocalVariable(name: "self", arg: 1, scope: !277, file: !268, line: 141, type: !257)
!277 = distinct !DISubprogram(name: "len", linkageName: "_RNvMNtCshhmRwEtsTQ2_4core3stre3lenCshm9upcCxmh6_15no_vtable_check", scope: !269, file: !268, line: 141, type: !278, scopeLine: 141, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !23, retainedNodes: !280)
!278 = !DISubroutineType(types: !279)
!279 = !{!9, !257}
!280 = !{!276}
!281 = !DILocation(line: 141, column: 22, scope: !277, inlinedAt: !282)
!282 = !DILocation(line: 820, column: 40, scope: !165)
!283 = !DILocalVariable(name: "self", arg: 1, scope: !284, file: !268, line: 486, type: !257)
!284 = distinct !DISubprogram(name: "as_bytes", linkageName: "_RNvMNtCshhmRwEtsTQ2_4core3stre8as_bytes", scope: !269, file: !268, line: 486, type: !285, scopeLine: 486, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !23, retainedNodes: !291)
!285 = !DISubroutineType(types: !286)
!286 = !{!287, !257}
!287 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[u8]", file: !2, size: 128, align: 64, elements: !288, templateParams: !23, identifier: "31681e0c10b314f1f33e38b2779acbb4")
!288 = !{!289, !290}
!289 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !287, file: !2, baseType: !260, size: 64, align: 64)
!290 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !287, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!291 = !{!283}
!292 = !DILocation(line: 486, column: 27, scope: !284, inlinedAt: !293)
!293 = !DILocation(line: 142, column: 14, scope: !277, inlinedAt: !282)
!294 = !DILocation(line: 820, column: 38, scope: !165)
!295 = !DILocation(line: 820, column: 23, scope: !165)
!296 = !DILocation(line: 823, column: 6, scope: !165)
!297 = distinct !DISubprogram(name: "do_nothing", linkageName: "_RNvXCshm9upcCxmh6_15no_vtable_checkNtB2_8MyStructNtB2_7MyTrait10do_nothing", scope: !298, file: !148, line: 8, type: !299, scopeLine: 8, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !23, retainedNodes: !302)
!298 = !DINamespace(name: "{impl#0}", scope: !33)
!299 = !DISubroutineType(types: !300)
!300 = !{null, !301}
!301 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&no_vtable_check::MyStruct", baseType: !32, size: 64, align: 64, dwarfAddressSpace: 0)
!302 = !{!303}
!303 = !DILocalVariable(name: "self", arg: 1, scope: !297, file: !148, line: 8, type: !301)
!304 = !DILocation(line: 8, column: 19, scope: !297)
!305 = !DILocation(line: 9, column: 9, scope: !297)
!306 = !DILocation(line: 10, column: 6, scope: !297)
!307 = distinct !DISubprogram(name: "report", linkageName: "_RNvXsZ_NtCsefmIBSMl6ne_3std7processuNtB5_11Termination6reportCshm9upcCxmh6_15no_vtable_check", scope: !308, file: !106, line: 2581, type: !309, scopeLine: 2581, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !23, retainedNodes: !311)
!308 = !DINamespace(name: "{impl#63}", scope: !108)
!309 = !DISubroutineType(types: !310)
!310 = !{!107, !7}
!311 = !{!312}
!312 = !DILocalVariable(arg: 1, scope: !307, file: !106, line: 2581, type: !7)
!313 = !DILocation(line: 2581, column: 15, scope: !307)
!314 = !DILocation(line: 2583, column: 6, scope: !307)
!315 = distinct !DISubprogram(name: "call_once<fn(), ()>", linkageName: "_RNvYFEuINtNtNtCshhmRwEtsTQ2_4core3ops8function6FnOnceuE9call_onceCshm9upcCxmh6_15no_vtable_check", scope: !134, file: !133, line: 250, type: !69, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !319, retainedNodes: !316)
!316 = !{!317, !318}
!317 = !DILocalVariable(arg: 1, scope: !315, file: !133, line: 250, type: !20)
!318 = !DILocalVariable(arg: 2, scope: !315, file: !133, line: 250, type: !7)
!319 = !{!320, !145}
!320 = !DITemplateTypeParameter(name: "Self", type: !20)
!321 = !DILocation(line: 250, column: 5, scope: !315)
!322 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_RNvYNCINvNtCsefmIBSMl6ne_3std2rt10lang_startuE0INtNtNtCshhmRwEtsTQ2_4core3ops8function6FnOnceuE9call_onceCshm9upcCxmh6_15no_vtable_check", scope: !134, file: !133, line: 250, type: !323, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !40, templateParams: !143, retainedNodes: !325)
!323 = !DISubroutineType(types: !324)
!324 = !{!96, !14}
!325 = !{!326, !327}
!326 = !DILocalVariable(arg: 1, scope: !322, file: !133, line: 250, type: !14)
!327 = !DILocalVariable(arg: 2, scope: !322, file: !133, line: 250, type: !7)
!328 = !DILocation(line: 250, column: 5, scope: !322)
