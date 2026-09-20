	.file	"twovariant_ex.fbe7010531387dd8-cgu.0"
	.section	.text._RNvCslCRXCeWE6FM_13twovariant_ex10inner_main,"ax",@progbits
	.globl	_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main
	.p2align	4
	.type	_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main,@function
_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main:
	.cfi_startproc
	movq	$11111, -8(%rsp)
	leaq	-8(%rsp), %rax
	#APP
	#NO_APP
	retq
.Lfunc_end0:
	.size	_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main, .Lfunc_end0-_RNvCslCRXCeWE6FM_13twovariant_ex10inner_main
	.cfi_endproc

	.section	.text._RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call,"ax",@progbits
	.globl	_RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call
	.p2align	4
	.type	_RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call,@function
_RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call:
	.cfi_startproc
	movl	$11111, %eax
	retq
.Lfunc_end1:
	.size	_RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call, .Lfunc_end1-_RNvCslCRXCeWE6FM_13twovariant_ex13wrap_cat_call
	.cfi_endproc

	.section	.text._RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner,"ax",@progbits
	.globl	_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner
	.p2align	4
	.type	_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner,@function
_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner:
	.cfi_startproc
	movl	$11111, %eax
	retq
.Lfunc_end2:
	.size	_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner, .Lfunc_end2-_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner
	.cfi_endproc

	.globl	_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_outer
	.type	_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_outer,@function
_RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_outer = _RNvCslCRXCeWE6FM_13twovariant_ex24wrap_dyn_call_from_inner
	.ident	"rustc version 1.94.0-dev"
	.section	".note.GNU-stack","",@progbits
