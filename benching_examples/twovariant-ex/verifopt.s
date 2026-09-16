	.file	"twovariant_ex.fbe7010531387dd8-cgu.0"
	.section	.text._RNvCslCRXCeWE6FM_13twovariant_ex10get_animal,"ax",@progbits
	.globl	_RNvCslCRXCeWE6FM_13twovariant_ex10get_animal
	.p2align	4
	.type	_RNvCslCRXCeWE6FM_13twovariant_ex10get_animal,@function
_RNvCslCRXCeWE6FM_13twovariant_ex10get_animal:
	.cfi_startproc
	testq	%rdi, %rdi
	leaq	.Lanon.a0f7dd0ec76c47d574bc46eb6f1772f5.0(%rip), %rax
	leaq	.Lanon.a0f7dd0ec76c47d574bc46eb6f1772f5.1(%rip), %rdx
	cmoveq	%rax, %rdx
	movl	$1, %eax
	retq
.Lfunc_end0:
	.size	_RNvCslCRXCeWE6FM_13twovariant_ex10get_animal, .Lfunc_end0-_RNvCslCRXCeWE6FM_13twovariant_ex10get_animal
	.cfi_endproc

	.section	.text._RNvCslCRXCeWE6FM_13twovariant_ex10inner_main,"ax",@progbits
	.globl	_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main
	.p2align	4
	.type	_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main,@function
_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	subq	$16, %rsp
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	xorl	%edi, %edi
	callq	*_RNvCslCRXCeWE6FM_13twovariant_ex10get_animal@GOTPCREL(%rip)
	movq	%rax, %rbx
	movq	%rdx, %r14
	movq	$11111, 8(%rsp)
	leaq	8(%rsp), %rax
	#APP
	#NO_APP
	movq	(%rdx), %rax
	testq	%rax, %rax
	je	.LBB1_2
.Ltmp0:
	movq	%rbx, %rdi
	callq	*%rax
.Ltmp1:
.LBB1_2:
	movq	8(%r14), %rsi
	testq	%rsi, %rsi
	je	.LBB1_4
	movq	16(%r14), %rdx
	movq	%rbx, %rdi
	callq	*_RNvCsdEVyRYziPs2_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB1_4:
	addq	$16, %rsp
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.LBB1_5:
	.cfi_def_cfa_offset 48
.Ltmp2:
	movq	%rax, %r15
	movq	8(%r14), %rsi
	testq	%rsi, %rsi
	je	.LBB1_7
	movq	16(%r14), %rdx
	movq	%rbx, %rdi
	callq	*_RNvCsdEVyRYziPs2_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB1_7:
	movq	%r15, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end1:
	.size	_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main, .Lfunc_end1-_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main
	.cfi_endproc
	.section	.gcc_except_table._RNvCslCRXCeWE6FM_13twovariant_ex10inner_main,"a",@progbits
	.p2align	2, 0x0
GCC_except_table1:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.uleb128 .Lfunc_end1-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	.text._RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call,"ax",@progbits
	.globl	_RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call
	.p2align	4
	.type	_RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call,@function
_RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call:
	.cfi_startproc
	movl	$11111, %eax
	retq
.Lfunc_end2:
	.size	_RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call, .Lfunc_end2-_RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call
	.cfi_endproc

	.section	.text._RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner,"ax",@progbits
	.globl	_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner
	.p2align	4
	.type	_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner,@function
_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner:
	.cfi_startproc
	movl	$11111, %eax
	retq
.Lfunc_end3:
	.size	_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner, .Lfunc_end3-_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner
	.cfi_endproc

	.section	.text._RNvCslCRXCeWE6FM_13twovariant_ex7get_cat,"ax",@progbits
	.globl	_RNvCslCRXCeWE6FM_13twovariant_ex7get_cat
	.p2align	4
	.type	_RNvCslCRXCeWE6FM_13twovariant_ex7get_cat,@function
_RNvCslCRXCeWE6FM_13twovariant_ex7get_cat:
	.cfi_startproc
	retq
.Lfunc_end4:
	.size	_RNvCslCRXCeWE6FM_13twovariant_ex7get_cat, .Lfunc_end4-_RNvCslCRXCeWE6FM_13twovariant_ex7get_cat
	.cfi_endproc

	.section	.text._RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak,"ax",@progbits
	.p2align	4
	.type	_RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak,@function
_RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak:
	.cfi_startproc
	movl	$11111, %eax
	retq
.Lfunc_end5:
	.size	_RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak, .Lfunc_end5-_RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak
	.cfi_endproc

	.section	.text._RNvXs_CslCRXCeWE6FM_13twovariant_exNtB4_3DogNtB4_6Animal5speak,"ax",@progbits
	.p2align	4
	.type	_RNvXs_CslCRXCeWE6FM_13twovariant_exNtB4_3DogNtB4_6Animal5speak,@function
_RNvXs_CslCRXCeWE6FM_13twovariant_exNtB4_3DogNtB4_6Animal5speak:
	.cfi_startproc
	movl	$22222, %eax
	retq
.Lfunc_end6:
	.size	_RNvXs_CslCRXCeWE6FM_13twovariant_exNtB4_3DogNtB4_6Animal5speak, .Lfunc_end6-_RNvXs_CslCRXCeWE6FM_13twovariant_exNtB4_3DogNtB4_6Animal5speak
	.cfi_endproc

	.type	.Lanon.a0f7dd0ec76c47d574bc46eb6f1772f5.0,@object
	.section	.data.rel.ro..Lanon.a0f7dd0ec76c47d574bc46eb6f1772f5.0,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a0f7dd0ec76c47d574bc46eb6f1772f5.0:
	.asciz	"\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\001\000\000\000\000\000\000"
	.quad	_RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak
	.size	.Lanon.a0f7dd0ec76c47d574bc46eb6f1772f5.0, 32

	.type	.Lanon.a0f7dd0ec76c47d574bc46eb6f1772f5.1,@object
	.section	.data.rel.ro..Lanon.a0f7dd0ec76c47d574bc46eb6f1772f5.1,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a0f7dd0ec76c47d574bc46eb6f1772f5.1:
	.asciz	"\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\001\000\000\000\000\000\000"
	.quad	_RNvXs_CslCRXCeWE6FM_13twovariant_exNtB4_3DogNtB4_6Animal5speak
	.size	.Lanon.a0f7dd0ec76c47d574bc46eb6f1772f5.1, 32

	.globl	_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_outer
	.type	_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_outer,@function
_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_outer = _RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner
	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.ident	"rustc version 1.94.0-dev"
	.section	".note.GNU-stack","",@progbits
