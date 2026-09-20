	.file	"twovariant_ex.fbe7010531387dd8-cgu.0"
	.section	.text._RINvNtCs6f85o4NbpXU_4core3ptr13drop_in_placeINtNtCs40qiDxj6tkN_5alloc5boxed3BoxDNtCslCRXCeWE6FM_13twovariant_ex6AnimalEL_EEB1i_,"ax",@progbits
	.p2align	4
	.type	_RINvNtCs6f85o4NbpXU_4core3ptr13drop_in_placeINtNtCs40qiDxj6tkN_5alloc5boxed3BoxDNtCslCRXCeWE6FM_13twovariant_ex6AnimalEL_EEB1i_,@function
_RINvNtCs6f85o4NbpXU_4core3ptr13drop_in_placeINtNtCs40qiDxj6tkN_5alloc5boxed3BoxDNtCslCRXCeWE6FM_13twovariant_ex6AnimalEL_EEB1i_:
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
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	%rsi, %r14
	movq	%rdi, %rbx
	movq	(%rsi), %rax
	testq	%rax, %rax
	je	.LBB0_2
.Ltmp0:
	movq	%rbx, %rdi
	callq	*%rax
.Ltmp1:
.LBB0_2:
	movq	8(%r14), %rsi
	testq	%rsi, %rsi
	je	.LBB0_3
	movq	16(%r14), %rdx
	movq	%rbx, %rdi
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	jmpq	*_RNvCsdEVyRYziPs2_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB0_3:
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.LBB0_4:
	.cfi_def_cfa_offset 32
.Ltmp2:
	movq	%rax, %r15
	movq	8(%r14), %rsi
	testq	%rsi, %rsi
	je	.LBB0_6
	movq	16(%r14), %rdx
	movq	%rbx, %rdi
	callq	*_RNvCsdEVyRYziPs2_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB0_6:
	movq	%r15, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end0:
	.size	_RINvNtCs6f85o4NbpXU_4core3ptr13drop_in_placeINtNtCs40qiDxj6tkN_5alloc5boxed3BoxDNtCslCRXCeWE6FM_13twovariant_ex6AnimalEL_EEB1i_, .Lfunc_end0-_RINvNtCs6f85o4NbpXU_4core3ptr13drop_in_placeINtNtCs40qiDxj6tkN_5alloc5boxed3BoxDNtCslCRXCeWE6FM_13twovariant_ex6AnimalEL_EEB1i_
	.cfi_endproc
	.section	.gcc_except_table._RINvNtCs6f85o4NbpXU_4core3ptr13drop_in_placeINtNtCs40qiDxj6tkN_5alloc5boxed3BoxDNtCslCRXCeWE6FM_13twovariant_ex6AnimalEL_EEB1i_,"a",@progbits
	.p2align	2, 0x0
GCC_except_table0:
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
	.uleb128 .Lfunc_end0-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	.text._RNvCslCRXCeWE6FM_13twovariant_ex10inner_main,"ax",@progbits
	.globl	_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main
	.p2align	4
	.type	_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main,@function
_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main:
.Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception1
	pushq	%rbx
	.cfi_def_cfa_offset 16
	subq	$16, %rsp
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -16
.Ltmp3:
	leaq	.Lanon.da58bfa98c666fc6071512d4a4e1eb49.0(%rip), %rsi
	movl	$1, %edi
	callq	*_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner@GOTPCREL(%rip)
.Ltmp4:
	movq	%rax, 8(%rsp)
	leaq	8(%rsp), %rax
	#APP
	#NO_APP
	addq	$16, %rsp
	.cfi_def_cfa_offset 16
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.LBB1_3:
	.cfi_def_cfa_offset 32
.Ltmp5:
	movq	%rax, %rbx
.Ltmp6:
	leaq	.Lanon.da58bfa98c666fc6071512d4a4e1eb49.0(%rip), %rsi
	movl	$1, %edi
	callq	_RINvNtCs6f85o4NbpXU_4core3ptr13drop_in_placeINtNtCs40qiDxj6tkN_5alloc5boxed3BoxDNtCslCRXCeWE6FM_13twovariant_ex6AnimalEL_EEB1i_
.Ltmp7:
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
.LBB1_2:
.Ltmp8:
	callq	*_RNvNtCs6f85o4NbpXU_4core9panicking16panic_in_cleanup@GOTPCREL(%rip)
.Lfunc_end1:
	.size	_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main, .Lfunc_end1-_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main
	.cfi_endproc
	.section	.gcc_except_table._RNvCslCRXCeWE6FM_13twovariant_ex10inner_main,"a",@progbits
	.p2align	2, 0x0
GCC_except_table1:
.Lexception1:
	.byte	255
	.byte	155
	.uleb128 .Lttbase0-.Lttbaseref0
.Lttbaseref0:
	.byte	1
	.uleb128 .Lcst_end1-.Lcst_begin1
.Lcst_begin1:
	.uleb128 .Ltmp3-.Lfunc_begin1
	.uleb128 .Ltmp4-.Ltmp3
	.uleb128 .Ltmp5-.Lfunc_begin1
	.byte	0
	.uleb128 .Ltmp6-.Lfunc_begin1
	.uleb128 .Ltmp7-.Ltmp6
	.uleb128 .Ltmp8-.Lfunc_begin1
	.byte	1
	.uleb128 .Ltmp7-.Lfunc_begin1
	.uleb128 .Lfunc_end1-.Ltmp7
	.byte	0
	.byte	0
.Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase0:
	.byte	0
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
	jmpq	*24(%rsi)
.Lfunc_end3:
	.size	_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner, .Lfunc_end3-_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner
	.cfi_endproc

	.section	.text._RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak,"ax",@progbits
	.p2align	4
	.type	_RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak,@function
_RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak:
	.cfi_startproc
	movl	$11111, %eax
	retq
.Lfunc_end4:
	.size	_RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak, .Lfunc_end4-_RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak
	.cfi_endproc

	.type	.Lanon.da58bfa98c666fc6071512d4a4e1eb49.0,@object
	.section	.data.rel.ro..Lanon.da58bfa98c666fc6071512d4a4e1eb49.0,"aw",@progbits
	.p2align	3, 0x0
.Lanon.da58bfa98c666fc6071512d4a4e1eb49.0:
	.asciz	"\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\001\000\000\000\000\000\000"
	.quad	_RNvXCslCRXCeWE6FM_13twovariant_exNtB2_3CatNtB2_6Animal5speak
	.size	.Lanon.da58bfa98c666fc6071512d4a4e1eb49.0, 32

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
