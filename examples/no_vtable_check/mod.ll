; ModuleID = '4ey4rzh9980vj5jz8zy4blkpf'
source_filename = "4ey4rzh9980vj5jz8zy4blkpf"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@vtable.0 = private constant <{}> zeroinitializer, align 8, !dbg !0
@anon.a9fb2210a212411c08c1cf522182d780.0 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@alloc_e3bcf19cc2379e63cf4ae2edddacd581 = private unnamed_addr constant [14 x i8] c"doing nothing\0A", align 1
@alloc_00a8b77a4978856e581317cd8f9c053e = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_e3bcf19cc2379e63cf4ae2edddacd581, [8 x i8] c"\0E\00\00\00\00\00\00\00" }>, align 8
@__rustc_debug_gdb_scripts_section__ = linkonce_odr unnamed_addr constant [34 x i8] c"\01gdb_load_rust_pretty_printers.py\00", section ".debug_gdb_scripts", align 1

; no_vtable_check::main
; Function Attrs: nonlazybind uwtable
define hidden void @_ZN15no_vtable_check4main17hb08df4f18b734676E() unnamed_addr #0 !dbg !43 {
start:
  %_s.dbg.spill = alloca [16 x i8], align 8
  store ptr inttoptr (i64 1 to ptr), ptr %_s.dbg.spill, align 8, !dbg !58
  %0 = getelementptr inbounds i8, ptr %_s.dbg.spill, i64 8, !dbg !58
  store ptr @vtable.0, ptr %0, align 8, !dbg !58
    #dbg_declare(ptr %_s.dbg.spill, !46, !DIExpression(), !59)
  ret void, !dbg !60
}

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17hb020ceb94d1a71b7E(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #0 !dbg !61 {
start:
  %sigpipe.dbg.spill = alloca [1 x i8], align 1
  %argv.dbg.spill = alloca [8 x i8], align 8
  %argc.dbg.spill = alloca [8 x i8], align 8
  %main.dbg.spill = alloca [8 x i8], align 8
  %_7 = alloca [8 x i8], align 8
  store ptr %main, ptr %main.dbg.spill, align 8
    #dbg_declare(ptr %main.dbg.spill, !70, !DIExpression(), !76)
  store i64 %argc, ptr %argc.dbg.spill, align 8
    #dbg_declare(ptr %argc.dbg.spill, !71, !DIExpression(), !77)
  store ptr %argv, ptr %argv.dbg.spill, align 8
    #dbg_declare(ptr %argv.dbg.spill, !72, !DIExpression(), !78)
  store i8 %sigpipe, ptr %sigpipe.dbg.spill, align 1
    #dbg_declare(ptr %sigpipe.dbg.spill, !73, !DIExpression(), !79)
  store ptr %main, ptr %_7, align 8, !dbg !80
; call std::rt::lang_start_internal
  %_0 = call i64 @_ZN3std2rt19lang_start_internal17had11ed1aa43f3d88E(ptr align 1 %_7, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe), !dbg !81
  ret i64 %_0, !dbg !82
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h534418dbdda9e4e1E"(ptr align 8 %_1) unnamed_addr #1 !dbg !83 {
start:
  %self.dbg.spill = alloca [1 x i8], align 1
  %_1.dbg.spill = alloca [8 x i8], align 8
  store ptr %_1, ptr %_1.dbg.spill, align 8
    #dbg_declare(ptr %_1.dbg.spill, !89, !DIExpression(DW_OP_deref), !90)
  %_4 = load ptr, ptr %_1, align 8, !dbg !91
; call std::sys::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h5ec997a8afe92091E(ptr %_4), !dbg !92
; call <() as std::process::Termination>::report
  %self = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h97b6eef8b5e86151E"(), !dbg !93
  store i8 %self, ptr %self.dbg.spill, align 1, !dbg !93
    #dbg_declare(ptr %self.dbg.spill, !94, !DIExpression(), !112)
  %_0 = zext i8 %self to i32, !dbg !114
  ret i32 %_0, !dbg !122
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h5ec997a8afe92091E(ptr %f) unnamed_addr #2 !dbg !123 {
start:
  %dummy.dbg.spill = alloca [0 x i8], align 1
  %f.dbg.spill = alloca [8 x i8], align 8
  %result.dbg.spill = alloca [0 x i8], align 1
    #dbg_declare(ptr %result.dbg.spill, !130, !DIExpression(), !134)
  store ptr %f, ptr %f.dbg.spill, align 8
    #dbg_declare(ptr %f.dbg.spill, !129, !DIExpression(), !135)
    #dbg_declare(ptr %dummy.dbg.spill, !136, !DIExpression(), !144)
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hb207130517f90b87E(ptr %f), !dbg !146
  call void asm sideeffect "", "~{memory}"(), !dbg !147, !srcloc !148
  ret void, !dbg !149
}

; core::fmt::rt::<impl core::fmt::Arguments>::new_const
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hb902ea13b0806123E"(ptr sret([48 x i8]) align 8 %_0, ptr align 8 %pieces) unnamed_addr #1 !dbg !150 {
start:
  %pieces.dbg.spill = alloca [8 x i8], align 8
  store ptr %pieces, ptr %pieces.dbg.spill, align 8
    #dbg_declare(ptr %pieces.dbg.spill, !293, !DIExpression(), !294)
  store ptr %pieces, ptr %_0, align 8, !dbg !295
  %0 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !295
  store i64 1, ptr %0, align 8, !dbg !295
  %1 = load ptr, ptr @anon.a9fb2210a212411c08c1cf522182d780.0, align 8, !dbg !295
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @anon.a9fb2210a212411c08c1cf522182d780.0, i64 8), align 8, !dbg !295
  %3 = getelementptr inbounds i8, ptr %_0, i64 32, !dbg !295
  store ptr %1, ptr %3, align 8, !dbg !295
  %4 = getelementptr inbounds i8, ptr %3, i64 8, !dbg !295
  store i64 %2, ptr %4, align 8, !dbg !295
  %5 = getelementptr inbounds i8, ptr %_0, i64 16, !dbg !295
  store ptr inttoptr (i64 8 to ptr), ptr %5, align 8, !dbg !295
  %6 = getelementptr inbounds i8, ptr %5, i64 8, !dbg !295
  store i64 0, ptr %6, align 8, !dbg !295
  ret void, !dbg !296
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h86cf8583adfcc402E"(ptr %_1) unnamed_addr #1 !dbg !297 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
    #dbg_declare(ptr %_1.dbg.spill, !306, !DIExpression(), !311)
    #dbg_declare(ptr %_2, !307, !DIExpression(), !311)
  %0 = load ptr, ptr %_1, align 8, !dbg !311
; call core::ops::function::FnOnce::call_once
  %_0 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h758439e98957f453E(ptr %0), !dbg !311
  ret i32 %_0, !dbg !311
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h758439e98957f453E(ptr %0) unnamed_addr #1 personality ptr @rust_eh_personality !dbg !312 {
start:
  %1 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [8 x i8], align 8
  store ptr %0, ptr %_1, align 8
    #dbg_declare(ptr %_1, !316, !DIExpression(), !318)
    #dbg_declare(ptr %_2, !317, !DIExpression(), !318)
; invoke std::rt::lang_start::{{closure}}
  %_0 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h534418dbdda9e4e1E"(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup, !dbg !318

bb3:                                              ; preds = %cleanup
  %2 = load ptr, ptr %1, align 8, !dbg !318
  %3 = getelementptr inbounds i8, ptr %1, i64 8, !dbg !318
  %4 = load i32, ptr %3, align 8, !dbg !318
  %5 = insertvalue { ptr, i32 } poison, ptr %2, 0, !dbg !318
  %6 = insertvalue { ptr, i32 } %5, i32 %4, 1, !dbg !318
  resume { ptr, i32 } %6, !dbg !318

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
  ret i32 %_0, !dbg !318
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17hb207130517f90b87E(ptr %_1) unnamed_addr #1 !dbg !319 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
    #dbg_declare(ptr %_1.dbg.spill, !321, !DIExpression(), !325)
    #dbg_declare(ptr %_2, !322, !DIExpression(), !325)
  call void %_1(), !dbg !325
  ret void, !dbg !325
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h97b6eef8b5e86151E"() unnamed_addr #1 !dbg !326 {
start:
  %_1.dbg.spill = alloca [0 x i8], align 1
    #dbg_declare(ptr %_1.dbg.spill, !331, !DIExpression(), !332)
  ret i8 0, !dbg !333
}

; <no_vtable_check::MyStruct as no_vtable_check::MyTrait>::do_nothing
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN70_$LT$no_vtable_check..MyStruct$u20$as$u20$no_vtable_check..MyTrait$GT$10do_nothing17he1fb507f934c50afE"(ptr align 1 %self) unnamed_addr #0 !dbg !334 {
start:
  %self.dbg.spill = alloca [8 x i8], align 8
  %_3 = alloca [48 x i8], align 8
  store ptr %self, ptr %self.dbg.spill, align 8
    #dbg_declare(ptr %self.dbg.spill, !340, !DIExpression(), !341)
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hb902ea13b0806123E"(ptr sret([48 x i8]) align 8 %_3, ptr align 8 @alloc_00a8b77a4978856e581317cd8f9c053e), !dbg !342
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17he837094b2ad61600E(ptr align 8 %_3), !dbg !342
  ret void, !dbg !343
}

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17had11ed1aa43f3d88E(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #0

; Function Attrs: nounwind nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #3

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17he837094b2ad61600E(ptr align 8) unnamed_addr #0

; Function Attrs: nonlazybind
define i32 @main(i32 %0, ptr %1) unnamed_addr #4 {
top:
  %2 = load volatile i8, ptr @__rustc_debug_gdb_scripts_section__, align 1
  %3 = sext i32 %0 to i64
; call std::rt::lang_start
  %4 = call i64 @_ZN3std2rt10lang_start17hb020ceb94d1a71b7E(ptr @_ZN15no_vtable_check4main17hb08df4f18b734676E, i64 %3, ptr %1, i8 0)
  %5 = trunc i64 %4 to i32
  ret i32 %5
}

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #4 = { nonlazybind "target-cpu"="x86-64" }

!llvm.module.flags = !{!24, !25, !26, !27, !28}
!llvm.ident = !{!29}
!llvm.dbg.cu = !{!30}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable}", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable_type}", file: !2, size: 384, align: 64, flags: DIFlagArtificial, elements: !4, vtableHolder: !14, templateParams: !23, identifier: "cd528c2334a61373803a2e876fd70669")
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
!14 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<()>", scope: !15, file: !2, size: 64, align: 64, elements: !18, templateParams: !23, identifier: "52bb55fb8e8a5acf1881e98a8190919f")
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
!29 = !{!"rustc version 1.92.0-dev"}
!30 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !31, producer: "clang LLVM (rustc version 1.92.0-dev)", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !32, splitDebugInlining: false, nameTableKind: None)
!31 = !DIFile(filename: "src/main.rs/@/4ey4rzh9980vj5jz8zy4blkpf", directory: "/home/np/hack/verifopt/examples/no_vtable_check")
!32 = !{!33, !0}
!33 = !DIGlobalVariableExpression(var: !34, expr: !DIExpression())
!34 = distinct !DIGlobalVariable(name: "<no_vtable_check::MyStruct as no_vtable_check::MyTrait>::{vtable}", scope: null, file: !2, type: !35, isLocal: true, isDefinition: true)
!35 = !DICompositeType(tag: DW_TAG_structure_type, name: "<no_vtable_check::MyStruct as no_vtable_check::MyTrait>::{vtable_type}", file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !36, vtableHolder: !41, templateParams: !23, identifier: "fa4d31a5d25c188d85a29261d94c125e")
!36 = !{!37, !38, !39, !40}
!37 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !35, file: !2, baseType: !6, size: 64, align: 64)
!38 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !35, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!39 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !35, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!40 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !35, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!41 = !DICompositeType(tag: DW_TAG_structure_type, name: "MyStruct", scope: !42, file: !2, align: 8, flags: DIFlagPrivate, elements: !23, identifier: "b9cbd2b95dd3bf84b2107198c5e8269e")
!42 = !DINamespace(name: "no_vtable_check", scope: null)
!43 = distinct !DISubprogram(name: "main", linkageName: "_ZN15no_vtable_check4main17hb08df4f18b734676E", scope: !42, file: !44, line: 13, type: !21, scopeLine: 13, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagMainSubprogram, unit: !30, templateParams: !23, retainedNodes: !45)
!44 = !DIFile(filename: "src/main.rs", directory: "/home/np/hack/verifopt/examples/no_vtable_check", checksumkind: CSK_MD5, checksum: "5373a823c34eba1505fda20513f05b3d")
!45 = !{!46}
!46 = !DILocalVariable(name: "_s", scope: !47, file: !44, line: 14, type: !48, align: 64)
!47 = distinct !DILexicalBlock(scope: !43, file: !44, line: 14, column: 5)
!48 = !DICompositeType(tag: DW_TAG_structure_type, name: "&dyn no_vtable_check::MyTrait", file: !2, size: 128, align: 64, elements: !49, templateParams: !23, identifier: "3f2ad70ec1efbac7eb446420bb8fed69")
!49 = !{!50, !53}
!50 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !48, file: !2, baseType: !51, size: 64, align: 64)
!51 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !52, size: 64, align: 64, dwarfAddressSpace: 0)
!52 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn no_vtable_check::MyTrait", file: !2, align: 8, elements: !23, identifier: "bfb9d6b3e61b6a1ae9234a7005452171")
!53 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !48, file: !2, baseType: !54, size: 64, align: 64, offset: 64)
!54 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 4]", baseType: !55, size: 64, align: 64, dwarfAddressSpace: 0)
!55 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 256, align: 64, elements: !56)
!56 = !{!57}
!57 = !DISubrange(count: 4, lowerBound: 0)
!58 = !DILocation(line: 14, column: 28, scope: !43)
!59 = !DILocation(line: 14, column: 9, scope: !47)
!60 = !DILocation(line: 15, column: 2, scope: !43)
!61 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_ZN3std2rt10lang_start17hb020ceb94d1a71b7E", scope: !16, file: !62, line: 199, type: !63, scopeLine: 199, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !74, retainedNodes: !69)
!62 = !DIFile(filename: "/home/np/hack/rust/library/std/src/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "89cabe9a51031f57147bca0574d2ccca")
!63 = !DISubroutineType(types: !64)
!64 = !{!65, !20, !65, !66, !68}
!65 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!66 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !67, size: 64, align: 64, dwarfAddressSpace: 0)
!67 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !68, size: 64, align: 64, dwarfAddressSpace: 0)
!68 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!69 = !{!70, !71, !72, !73}
!70 = !DILocalVariable(name: "main", arg: 1, scope: !61, file: !62, line: 200, type: !20)
!71 = !DILocalVariable(name: "argc", arg: 2, scope: !61, file: !62, line: 201, type: !65)
!72 = !DILocalVariable(name: "argv", arg: 3, scope: !61, file: !62, line: 202, type: !66)
!73 = !DILocalVariable(name: "sigpipe", arg: 4, scope: !61, file: !62, line: 203, type: !68)
!74 = !{!75}
!75 = !DITemplateTypeParameter(name: "T", type: !7)
!76 = !DILocation(line: 200, column: 5, scope: !61)
!77 = !DILocation(line: 201, column: 5, scope: !61)
!78 = !DILocation(line: 202, column: 5, scope: !61)
!79 = !DILocation(line: 203, column: 5, scope: !61)
!80 = !DILocation(line: 206, column: 10, scope: !61)
!81 = !DILocation(line: 205, column: 5, scope: !61)
!82 = !DILocation(line: 211, column: 2, scope: !61)
!83 = distinct !DISubprogram(name: "{closure#0}<()>", linkageName: "_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h534418dbdda9e4e1E", scope: !15, file: !62, line: 206, type: !84, scopeLine: 206, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !74, retainedNodes: !88)
!84 = !DISubroutineType(types: !85)
!85 = !{!86, !87}
!86 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!87 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!88 = !{!89}
!89 = !DILocalVariable(name: "main", scope: !83, file: !62, line: 200, type: !20, align: 64)
!90 = !DILocation(line: 200, column: 5, scope: !83)
!91 = !DILocation(line: 206, column: 70, scope: !83)
!92 = !DILocation(line: 206, column: 18, scope: !83)
!93 = !DILocation(line: 206, column: 76, scope: !83)
!94 = !DILocalVariable(name: "self", arg: 1, scope: !95, file: !96, line: 2161, type: !97)
!95 = distinct !DISubprogram(name: "to_i32", linkageName: "_ZN3std7process8ExitCode6to_i3217h22a886465a9817ddE", scope: !97, file: !96, line: 2161, type: !108, scopeLine: 2161, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !23, declaration: !110, retainedNodes: !111)
!96 = !DIFile(filename: "/home/np/hack/rust/library/std/src/process.rs", directory: "", checksumkind: CSK_MD5, checksum: "c1d0d8af031f70a317b8c3ee91aaa453")
!97 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !98, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !99, templateParams: !23, identifier: "3ba3f53275da0de98f558651527a1f5d")
!98 = !DINamespace(name: "process", scope: !17)
!99 = !{!100}
!100 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !97, file: !2, baseType: !101, size: 8, align: 8, flags: DIFlagPrivate)
!101 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !102, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !106, templateParams: !23, identifier: "54f03f884cf22ba7879ec3e5436c2419")
!102 = !DINamespace(name: "common", scope: !103)
!103 = !DINamespace(name: "unix", scope: !104)
!104 = !DINamespace(name: "process", scope: !105)
!105 = !DINamespace(name: "sys", scope: !17)
!106 = !{!107}
!107 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !101, file: !2, baseType: !68, size: 8, align: 8, flags: DIFlagPrivate)
!108 = !DISubroutineType(types: !109)
!109 = !{!86, !97}
!110 = !DISubprogram(name: "to_i32", linkageName: "_ZN3std7process8ExitCode6to_i3217h22a886465a9817ddE", scope: !97, file: !96, line: 2161, type: !108, scopeLine: 2161, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!111 = !{!94}
!112 = !DILocation(line: 2161, column: 19, scope: !95, inlinedAt: !113)
!113 = !DILocation(line: 206, column: 85, scope: !83)
!114 = !DILocation(line: 590, column: 9, scope: !115, inlinedAt: !121)
!115 = distinct !DISubprogram(name: "as_i32", linkageName: "_ZN3std3sys7process4unix6common8ExitCode6as_i3217h0f77e931232e466eE", scope: !101, file: !116, line: 589, type: !117, scopeLine: 589, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !23, declaration: !120)
!116 = !DIFile(filename: "/home/np/hack/rust/library/std/src/sys/process/unix/common.rs", directory: "", checksumkind: CSK_MD5, checksum: "1f22f9ba5abf68e24103f74be3b1fdf9")
!117 = !DISubroutineType(types: !118)
!118 = !{!86, !119}
!119 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::sys::process::unix::common::ExitCode", baseType: !101, size: 64, align: 64, dwarfAddressSpace: 0)
!120 = !DISubprogram(name: "as_i32", linkageName: "_ZN3std3sys7process4unix6common8ExitCode6as_i3217h0f77e931232e466eE", scope: !101, file: !116, line: 589, type: !117, scopeLine: 589, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!121 = !DILocation(line: 2162, column: 16, scope: !95, inlinedAt: !113)
!122 = !DILocation(line: 206, column: 93, scope: !83)
!123 = distinct !DISubprogram(name: "__rust_begin_short_backtrace<fn(), ()>", linkageName: "_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h5ec997a8afe92091E", scope: !125, file: !124, line: 154, type: !126, scopeLine: 154, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !132, retainedNodes: !128)
!124 = !DIFile(filename: "/home/np/hack/rust/library/std/src/sys/backtrace.rs", directory: "", checksumkind: CSK_MD5, checksum: "e2cc8cb6b8d66d5c0e73f449e0e721de")
!125 = !DINamespace(name: "backtrace", scope: !105)
!126 = !DISubroutineType(types: !127)
!127 = !{null, !20}
!128 = !{!129, !130}
!129 = !DILocalVariable(name: "f", arg: 1, scope: !123, file: !124, line: 154, type: !20)
!130 = !DILocalVariable(name: "result", scope: !131, file: !124, line: 158, type: !7, align: 8)
!131 = distinct !DILexicalBlock(scope: !123, file: !124, line: 158, column: 5)
!132 = !{!133, !75}
!133 = !DITemplateTypeParameter(name: "F", type: !20)
!134 = !DILocation(line: 158, column: 9, scope: !131)
!135 = !DILocation(line: 154, column: 43, scope: !123)
!136 = !DILocalVariable(name: "dummy", scope: !137, file: !138, line: 471, type: !7, align: 8)
!137 = distinct !DISubprogram(name: "black_box<()>", linkageName: "_ZN4core4hint9black_box17h456c4211574a7cb7E", scope: !139, file: !138, line: 471, type: !141, scopeLine: 471, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !74, retainedNodes: !143)
!138 = !DIFile(filename: "/home/np/hack/rust/library/core/src/hint.rs", directory: "", checksumkind: CSK_MD5, checksum: "56f659f9cbc57d60ad8ce456b7f06ccb")
!139 = !DINamespace(name: "hint", scope: !140)
!140 = !DINamespace(name: "core", scope: null)
!141 = !DISubroutineType(types: !142)
!142 = !{null, !7}
!143 = !{!136}
!144 = !DILocation(line: 471, column: 27, scope: !137, inlinedAt: !145)
!145 = !DILocation(line: 161, column: 5, scope: !131)
!146 = !DILocation(line: 158, column: 18, scope: !123)
!147 = !DILocation(line: 472, column: 5, scope: !137, inlinedAt: !145)
!148 = !{i64 11488672447254720}
!149 = !DILocation(line: 164, column: 2, scope: !123)
!150 = distinct !DISubprogram(name: "new_const<1>", linkageName: "_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hb902ea13b0806123E", scope: !152, file: !151, line: 194, type: !285, scopeLine: 194, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !23, declaration: !291, retainedNodes: !292)
!151 = !DIFile(filename: "/home/np/hack/rust/library/core/src/fmt/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "03cba3c9b7eca44212bc16adf1d5b1bc")
!152 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !153, file: !2, size: 384, align: 64, flags: DIFlagPublic, elements: !154, templateParams: !23, identifier: "7d58ac02c301f43baf9fb7dcbbb2de4d")
!153 = !DINamespace(name: "fmt", scope: !140)
!154 = !{!155, !166, !211}
!155 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !152, file: !2, baseType: !156, size: 128, align: 64, flags: DIFlagPrivate)
!156 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !2, size: 128, align: 64, elements: !157, templateParams: !23, identifier: "4e66b00a376d6af5b8765440fb2839f")
!157 = !{!158, !165}
!158 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !156, file: !2, baseType: !159, size: 64, align: 64)
!159 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !160, size: 64, align: 64, dwarfAddressSpace: 0)
!160 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 128, align: 64, elements: !161, templateParams: !23, identifier: "9277eecd40495f85161460476aacc992")
!161 = !{!162, !164}
!162 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !160, file: !2, baseType: !163, size: 64, align: 64)
!163 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !68, size: 64, align: 64, dwarfAddressSpace: 0)
!164 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !160, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!165 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !156, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!166 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !152, file: !2, baseType: !167, size: 128, align: 64, offset: 256, flags: DIFlagPrivate)
!167 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::Placeholder]>", scope: !168, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !169, templateParams: !23, identifier: "50c356ee7c09933c59e1758085f02c92")
!168 = !DINamespace(name: "option", scope: !140)
!169 = !{!170}
!170 = !DICompositeType(tag: DW_TAG_variant_part, scope: !167, file: !2, size: 128, align: 64, elements: !171, templateParams: !23, identifier: "88dc297f756b807ad1254dd92c03723", discriminator: !209)
!171 = !{!172, !205}
!172 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !170, file: !2, baseType: !173, size: 128, align: 64, extraData: i64 0)
!173 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !167, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !174, identifier: "2d8b1d07ec70713c4f0da616b2417136")
!174 = !{!175}
!175 = !DITemplateTypeParameter(name: "T", type: !176)
!176 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Placeholder]", file: !2, size: 128, align: 64, elements: !177, templateParams: !23, identifier: "a49b8ff40868a36890fdd7c7b107ffb0")
!177 = !{!178, !204}
!178 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !176, file: !2, baseType: !179, size: 64, align: 64)
!179 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !180, size: 64, align: 64, dwarfAddressSpace: 0)
!180 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !181, file: !2, size: 384, align: 64, flags: DIFlagPublic, elements: !182, templateParams: !23, identifier: "1fb5fa5ee31af3e2fccbdb8b2019e3c5")
!181 = !DINamespace(name: "rt", scope: !153)
!182 = !{!183, !184, !186, !203}
!183 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !180, file: !2, baseType: !9, size: 64, align: 64, offset: 256, flags: DIFlagPublic)
!184 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !180, file: !2, baseType: !185, size: 32, align: 32, offset: 320, flags: DIFlagPublic)
!185 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!186 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !180, file: !2, baseType: !187, size: 128, align: 64, flags: DIFlagPublic)
!187 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !181, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !188, templateParams: !23, identifier: "985549cb57f54a3d348f2104b3a0047b")
!188 = !{!189}
!189 = !DICompositeType(tag: DW_TAG_variant_part, scope: !187, file: !2, size: 128, align: 64, elements: !190, templateParams: !23, identifier: "bc676fa491009d568e79d1eb9405778e", discriminator: !202)
!190 = !{!191, !196, !200}
!191 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !189, file: !2, baseType: !192, size: 128, align: 64, extraData: i16 0)
!192 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !187, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !193, templateParams: !23, identifier: "fde2b4e0ae9c51688447454509865979")
!193 = !{!194}
!194 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !192, file: !2, baseType: !195, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!195 = !DIBasicType(name: "u16", size: 16, encoding: DW_ATE_unsigned)
!196 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !189, file: !2, baseType: !197, size: 128, align: 64, extraData: i16 1)
!197 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !187, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !198, templateParams: !23, identifier: "3cd0e25527f75cf082cb1f3b6b6d743f")
!198 = !{!199}
!199 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !197, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!200 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !189, file: !2, baseType: !201, size: 128, align: 64, extraData: i16 2)
!201 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !187, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, identifier: "12866993a6cd39be141bbdeab563335")
!202 = !DIDerivedType(tag: DW_TAG_member, scope: !187, file: !2, baseType: !195, size: 16, align: 16, flags: DIFlagArtificial)
!203 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !180, file: !2, baseType: !187, size: 128, align: 64, offset: 128, flags: DIFlagPublic)
!204 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !176, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!205 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !170, file: !2, baseType: !206, size: 128, align: 64)
!206 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !167, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !207, templateParams: !174, identifier: "c485de763449917dc3b7212beace901d")
!207 = !{!208}
!208 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !206, file: !2, baseType: !176, size: 128, align: 64, flags: DIFlagPublic)
!209 = !DIDerivedType(tag: DW_TAG_member, scope: !167, file: !2, baseType: !210, size: 64, align: 64, flags: DIFlagArtificial)
!210 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!211 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !152, file: !2, baseType: !212, size: 128, align: 64, offset: 128, flags: DIFlagPrivate)
!212 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Argument]", file: !2, size: 128, align: 64, elements: !213, templateParams: !23, identifier: "c75941db044b51302d1b9bbbdb0290ba")
!213 = !{!214, !284}
!214 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !212, file: !2, baseType: !215, size: 64, align: 64)
!215 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !216, size: 64, align: 64, dwarfAddressSpace: 0)
!216 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !181, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !217, templateParams: !23, identifier: "22a16b187855db5990b9cc63b2c61e88")
!217 = !{!218}
!218 = !DIDerivedType(tag: DW_TAG_member, name: "ty", scope: !216, file: !2, baseType: !219, size: 128, align: 64, flags: DIFlagPrivate)
!219 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentType", scope: !181, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !220, templateParams: !23, identifier: "580f2a9e9caf351eacbbe1df0b1849c9")
!220 = !{!221}
!221 = !DICompositeType(tag: DW_TAG_variant_part, scope: !219, file: !2, size: 128, align: 64, elements: !222, templateParams: !23, identifier: "8677a6a762e1c59536544a63a14d4431", discriminator: !283)
!222 = !{!223, !279}
!223 = !DIDerivedType(tag: DW_TAG_member, name: "Placeholder", scope: !221, file: !2, baseType: !224, size: 128, align: 64)
!224 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !219, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !225, templateParams: !23, identifier: "dcc6594d3b63b9af31916199421b33fa")
!225 = !{!226, !232, !273}
!226 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !224, file: !2, baseType: !227, size: 64, align: 64, flags: DIFlagPrivate)
!227 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<()>", scope: !228, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !230, templateParams: !74, identifier: "825b0c4314bfaa64a896f884702756c7")
!228 = !DINamespace(name: "non_null", scope: !229)
!229 = !DINamespace(name: "ptr", scope: !140)
!230 = !{!231}
!231 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !227, file: !2, baseType: !6, size: 64, align: 64, flags: DIFlagPrivate)
!232 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !224, file: !2, baseType: !233, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!233 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "unsafe fn(core::ptr::non_null::NonNull<()>, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !234, size: 64, align: 64, dwarfAddressSpace: 0)
!234 = !DISubroutineType(types: !235)
!235 = !{!236, !227, !253}
!236 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), core::fmt::Error>", scope: !237, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !238, templateParams: !23, identifier: "84caf198beeaee7d5d1cf628ac1b5cb0")
!237 = !DINamespace(name: "result", scope: !140)
!238 = !{!239}
!239 = !DICompositeType(tag: DW_TAG_variant_part, scope: !236, file: !2, size: 8, align: 8, elements: !240, templateParams: !23, identifier: "9addd34d593ea386bfe67eaddcb01605", discriminator: !252)
!240 = !{!241, !248}
!241 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !239, file: !2, baseType: !242, size: 8, align: 8, extraData: i8 0)
!242 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !236, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !243, templateParams: !245, identifier: "65cdfc1cae45de4cde4665072137a215")
!243 = !{!244}
!244 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !242, file: !2, baseType: !7, align: 8, offset: 8, flags: DIFlagPublic)
!245 = !{!75, !246}
!246 = !DITemplateTypeParameter(name: "E", type: !247)
!247 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !153, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "a7c1638c4a2ea7d2d48cb3faa110f99a")
!248 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !239, file: !2, baseType: !249, size: 8, align: 8, extraData: i8 1)
!249 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !236, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !250, templateParams: !245, identifier: "1a1bb3e0792fcc2b582d64739c9b02cc")
!250 = !{!251}
!251 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !249, file: !2, baseType: !247, align: 8, offset: 8, flags: DIFlagPublic)
!252 = !DIDerivedType(tag: DW_TAG_member, scope: !236, file: !2, baseType: !68, size: 8, align: 8, flags: DIFlagArtificial)
!253 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !254, size: 64, align: 64, dwarfAddressSpace: 0)
!254 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !153, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !255, templateParams: !23, identifier: "cee572ed0b7e6ab1a2bbaafc9046021d")
!255 = !{!256, !262}
!256 = !DIDerivedType(tag: DW_TAG_member, name: "options", scope: !254, file: !2, baseType: !257, size: 64, align: 32, offset: 128, flags: DIFlagPrivate)
!257 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormattingOptions", scope: !153, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !258, templateParams: !23, identifier: "2d93830c4e599e05b42e2d3080cecabd")
!258 = !{!259, !260, !261}
!259 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !257, file: !2, baseType: !185, size: 32, align: 32, flags: DIFlagPrivate)
!260 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !257, file: !2, baseType: !195, size: 16, align: 16, offset: 32, flags: DIFlagPrivate)
!261 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !257, file: !2, baseType: !195, size: 16, align: 16, offset: 48, flags: DIFlagPrivate)
!262 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !254, file: !2, baseType: !263, size: 128, align: 64, flags: DIFlagPrivate)
!263 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !2, size: 128, align: 64, elements: !264, templateParams: !23, identifier: "d7468ae9f63341a4a380a3dced0ded09")
!264 = !{!265, !268}
!265 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !263, file: !2, baseType: !266, size: 64, align: 64)
!266 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !267, size: 64, align: 64, dwarfAddressSpace: 0)
!267 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !2, align: 8, elements: !23, identifier: "3f11842946a6bb21fd082500f2de3999")
!268 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !263, file: !2, baseType: !269, size: 64, align: 64, offset: 64)
!269 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 6]", baseType: !270, size: 64, align: 64, dwarfAddressSpace: 0)
!270 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 384, align: 64, elements: !271)
!271 = !{!272}
!272 = !DISubrange(count: 6, lowerBound: 0)
!273 = !DIDerivedType(tag: DW_TAG_member, name: "_lifetime", scope: !224, file: !2, baseType: !274, align: 8, offset: 128, flags: DIFlagPrivate)
!274 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&()>", scope: !275, file: !2, align: 8, flags: DIFlagPublic, elements: !23, templateParams: !276, identifier: "bacc86ff826d96e05d094aa1e0207672")
!275 = !DINamespace(name: "marker", scope: !140)
!276 = !{!277}
!277 = !DITemplateTypeParameter(name: "T", type: !278)
!278 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&()", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 0)
!279 = !DIDerivedType(tag: DW_TAG_member, name: "Count", scope: !221, file: !2, baseType: !280, size: 128, align: 64, extraData: i64 0)
!280 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !219, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !281, templateParams: !23, identifier: "143d72d5a9966f12808f562da7a8d6bb")
!281 = !{!282}
!282 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !280, file: !2, baseType: !195, size: 16, align: 16, offset: 64, flags: DIFlagPrivate)
!283 = !DIDerivedType(tag: DW_TAG_member, scope: !219, file: !2, baseType: !210, size: 64, align: 64, flags: DIFlagArtificial)
!284 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !212, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!285 = !DISubroutineType(types: !286)
!286 = !{!152, !287}
!287 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[&str; 1]", baseType: !288, size: 64, align: 64, dwarfAddressSpace: 0)
!288 = !DICompositeType(tag: DW_TAG_array_type, baseType: !160, size: 128, align: 64, elements: !289)
!289 = !{!290}
!290 = !DISubrange(count: 1, lowerBound: 0)
!291 = !DISubprogram(name: "new_const<1>", linkageName: "_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hb902ea13b0806123E", scope: !152, file: !151, line: 194, type: !285, scopeLine: 194, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!292 = !{!293}
!293 = !DILocalVariable(name: "pieces", arg: 1, scope: !150, file: !151, line: 194, type: !287)
!294 = !DILocation(line: 194, column: 44, scope: !150)
!295 = !DILocation(line: 196, column: 9, scope: !150)
!296 = !DILocation(line: 197, column: 6, scope: !150)
!297 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h86cf8583adfcc402E", scope: !299, file: !298, line: 250, type: !302, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !308, retainedNodes: !305)
!298 = !DIFile(filename: "/home/np/hack/rust/library/core/src/ops/function.rs", directory: "", checksumkind: CSK_MD5, checksum: "f10f7c44ec86506ef01d8c34efe59fc0")
!299 = !DINamespace(name: "FnOnce", scope: !300)
!300 = !DINamespace(name: "function", scope: !301)
!301 = !DINamespace(name: "ops", scope: !140)
!302 = !DISubroutineType(types: !303)
!303 = !{!86, !304}
!304 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!305 = !{!306, !307}
!306 = !DILocalVariable(arg: 1, scope: !297, file: !298, line: 250, type: !304)
!307 = !DILocalVariable(arg: 2, scope: !297, file: !298, line: 250, type: !7)
!308 = !{!309, !310}
!309 = !DITemplateTypeParameter(name: "Self", type: !14)
!310 = !DITemplateTypeParameter(name: "Args", type: !7)
!311 = !DILocation(line: 250, column: 5, scope: !297)
!312 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17h758439e98957f453E", scope: !299, file: !298, line: 250, type: !313, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !308, retainedNodes: !315)
!313 = !DISubroutineType(types: !314)
!314 = !{!86, !14}
!315 = !{!316, !317}
!316 = !DILocalVariable(arg: 1, scope: !312, file: !298, line: 250, type: !14)
!317 = !DILocalVariable(arg: 2, scope: !312, file: !298, line: 250, type: !7)
!318 = !DILocation(line: 250, column: 5, scope: !312)
!319 = distinct !DISubprogram(name: "call_once<fn(), ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17hb207130517f90b87E", scope: !299, file: !298, line: 250, type: !126, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !323, retainedNodes: !320)
!320 = !{!321, !322}
!321 = !DILocalVariable(arg: 1, scope: !319, file: !298, line: 250, type: !20)
!322 = !DILocalVariable(arg: 2, scope: !319, file: !298, line: 250, type: !7)
!323 = !{!324, !310}
!324 = !DITemplateTypeParameter(name: "Self", type: !20)
!325 = !DILocation(line: 250, column: 5, scope: !319)
!326 = distinct !DISubprogram(name: "report", linkageName: "_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h97b6eef8b5e86151E", scope: !327, file: !96, line: 2559, type: !328, scopeLine: 2559, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !23, retainedNodes: !330)
!327 = !DINamespace(name: "{impl#63}", scope: !98)
!328 = !DISubroutineType(types: !329)
!329 = !{!97, !7}
!330 = !{!331}
!331 = !DILocalVariable(arg: 1, scope: !326, file: !96, line: 2559, type: !7)
!332 = !DILocation(line: 2559, column: 15, scope: !326)
!333 = !DILocation(line: 2561, column: 6, scope: !326)
!334 = distinct !DISubprogram(name: "do_nothing", linkageName: "_ZN70_$LT$no_vtable_check..MyStruct$u20$as$u20$no_vtable_check..MyTrait$GT$10do_nothing17he1fb507f934c50afE", scope: !335, file: !44, line: 8, type: !336, scopeLine: 8, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !30, templateParams: !23, retainedNodes: !339)
!335 = !DINamespace(name: "{impl#0}", scope: !42)
!336 = !DISubroutineType(types: !337)
!337 = !{null, !338}
!338 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&no_vtable_check::MyStruct", baseType: !41, size: 64, align: 64, dwarfAddressSpace: 0)
!339 = !{!340}
!340 = !DILocalVariable(name: "self", arg: 1, scope: !334, file: !44, line: 8, type: !338)
!341 = !DILocation(line: 8, column: 19, scope: !334)
!342 = !DILocation(line: 9, column: 9, scope: !334)
!343 = !DILocation(line: 10, column: 6, scope: !334)
