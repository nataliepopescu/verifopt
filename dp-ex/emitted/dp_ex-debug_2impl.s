	.file	"47mem9q2d6p1mvtu0iveyp842"
	.section	".text._ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h65f4881e2030d610E","ax",@progbits
	.p2align	4
	.type	_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h65f4881e2030d610E,@function
_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h65f4881e2030d610E:
.Lfunc_begin0:
	.file	1 "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/rngs" "reseeding.rs"
	.loc	1 162 0
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
	movq	%rsi, 16(%rsp)
	movq	%rdi, 24(%rsp)
.Ltmp0:
	.loc	1 163 12 prologue_end
	cmpq	$0, 56(%rdi)
	jle	.LBB0_2
	.loc	1 169 37
	leaq	16(%rsp), %rdi
	callq	_ZN63_$LT$$RF$mut$u20$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17hcf4f85feb7206a28E
	movq	%rax, %rdi
	movq	%rdx, %rsi
	.loc	1 169 25 is_stmt 0
	callq	_ZN4core3mem11size_of_val17h6d9e552afa58affdE
	movq	%rax, %rcx
	movq	8(%rsp), %rax
	movq	%rcx, 32(%rsp)
.Ltmp1:
	.loc	1 170 9 is_stmt 1
	movq	56(%rax), %rax
	subq	%rcx, %rax
	movq	%rax, (%rsp)
	seto	%al
	jo	.LBB0_4
	jmp	.LBB0_3
.Ltmp2:
.LBB0_2:
	.loc	1 0 9 is_stmt 0
	movq	8(%rsp), %rdi
	.loc	1 167 20 is_stmt 1
	movq	16(%rsp), %rsi
	callq	_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17h13935bc7e637f19cE
	jmp	.LBB0_5
.LBB0_3:
	.loc	1 0 20 is_stmt 0
	movq	8(%rsp), %rdi
	movq	(%rsp), %rax
.Ltmp3:
	.loc	1 170 9 is_stmt 1
	movq	%rax, 56(%rdi)
	.loc	1 171 9
	movq	16(%rsp), %rsi
	callq	_ZN84_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h30baf41324780ba0E
	jmp	.LBB0_5
.LBB0_4:
	.loc	1 170 9
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.1(%rip), %rdi
	callq	*_ZN4core9panicking11panic_const24panic_const_sub_overflow17hab14e776658d8858E@GOTPCREL(%rip)
.Ltmp4:
.LBB0_5:
	.loc	1 172 6 epilogue_begin
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp5:
.Lfunc_end0:
	.size	_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h65f4881e2030d610E, .Lfunc_end0-_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h65f4881e2030d610E
	.cfi_endproc

	.section	.text._ZN3std2rt10lang_start17h27e2b9e2c6bd9d09E,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17h27e2b9e2c6bd9d09E
	.globl	_ZN3std2rt10lang_start17h27e2b9e2c6bd9d09E
	.p2align	4
	.type	_ZN3std2rt10lang_start17h27e2b9e2c6bd9d09E,@function
_ZN3std2rt10lang_start17h27e2b9e2c6bd9d09E:
.Lfunc_begin1:
	.file	2 "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/std/src" "rt.rs"
	.loc	2 192 0
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movl	%ecx, %eax
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, 8(%rsp)
	movq	%rdx, 16(%rsp)
	movq	%rcx, 24(%rsp)
	movb	%al, 39(%rsp)
.Ltmp6:
	.loc	2 199 10 prologue_end
	movq	%rdi, (%rsp)
	.loc	2 198 5
	movq	%rsp, %rdi
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.2(%rip), %rsi
	movzbl	%al, %r8d
	callq	*_ZN3std2rt19lang_start_internal17h179a86731aa5c4aaE@GOTPCREL(%rip)
	.loc	2 204 2 epilogue_begin
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp7:
.Lfunc_end1:
	.size	_ZN3std2rt10lang_start17h27e2b9e2c6bd9d09E, .Lfunc_end1-_ZN3std2rt10lang_start17h27e2b9e2c6bd9d09E
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h975e66f67ae81916E","ax",@progbits
	.p2align	4
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h975e66f67ae81916E,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h975e66f67ae81916E:
.Lfunc_begin2:
	.loc	2 199 0
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, 8(%rsp)
.Ltmp8:
	.loc	2 199 70 prologue_end
	movq	(%rdi), %rdi
	.loc	2 199 18 is_stmt 0
	callq	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2afd2c7d0674a96fE
	callq	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h61953a0fd5719ceaE
	movb	%al, 23(%rsp)
.Ltmp9:
	.file	3 "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/std/src/sys/pal/unix/process" "process_common.rs"
	.loc	3 631 9 is_stmt 1
	movzbl	%al, %eax
.Ltmp10:
	.loc	2 199 93 epilogue_begin
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp11:
.Lfunc_end2:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h975e66f67ae81916E, .Lfunc_end2-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h975e66f67ae81916E
	.cfi_endproc
	.file	4 "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/std/src" "process.rs"

	.section	.text._ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2afd2c7d0674a96fE,"ax",@progbits
	.p2align	4
	.type	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2afd2c7d0674a96fE,@function
_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2afd2c7d0674a96fE:
.Lfunc_begin3:
	.file	5 "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/std/src/sys" "backtrace.rs"
	.loc	5 148 0
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, 8(%rsp)
.Ltmp12:
	.loc	5 152 18 prologue_end
	callq	_ZN4core3ops8function6FnOnce9call_once17hd703a48edda16cd5E
.Ltmp13:
	.file	6 "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src" "hint.rs"
	.loc	6 477 5
	#APP
	#NO_APP
.Ltmp14:
	.loc	5 158 2 epilogue_begin
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp15:
.Lfunc_end3:
	.size	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2afd2c7d0674a96fE, .Lfunc_end3-_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2afd2c7d0674a96fE
	.cfi_endproc

	.section	.text._ZN4core3mem11size_of_val17h6d9e552afa58affdE,"ax",@progbits
	.p2align	4
	.type	_ZN4core3mem11size_of_val17h6d9e552afa58affdE,@function
_ZN4core3mem11size_of_val17h6d9e552afa58affdE:
.Lfunc_begin4:
	.file	7 "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/mem" "mod.rs"
	.loc	7 332 0
	.cfi_startproc
	movq	%rdi, -24(%rsp)
	movq	%rsi, -16(%rsp)
.Ltmp16:
	.loc	7 334 14 prologue_end
	shlq	$2, %rsi
	movq	%rsi, -8(%rsp)
	movq	-8(%rsp), %rax
	.loc	7 335 2
	retq
.Ltmp17:
.Lfunc_end4:
	.size	_ZN4core3mem11size_of_val17h6d9e552afa58affdE, .Lfunc_end4-_ZN4core3mem11size_of_val17h6d9e552afa58affdE
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hb06e99ac8656f6d6E","ax",@progbits
	.p2align	4
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hb06e99ac8656f6d6E,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hb06e99ac8656f6d6E:
.Lfunc_begin5:
	.file	8 "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/ops" "function.rs"
	.loc	8 250 0
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, 16(%rsp)
.Ltmp18:
	.loc	8 250 5 prologue_end
	movq	(%rdi), %rdi
	callq	_ZN4core3ops8function6FnOnce9call_once17h1c445d99f54a92d5E
	.loc	8 250 5 epilogue_begin is_stmt 0
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp19:
.Lfunc_end5:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hb06e99ac8656f6d6E, .Lfunc_end5-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hb06e99ac8656f6d6E
	.cfi_endproc

	.section	.text._ZN4core3ops8function6FnOnce9call_once17h1c445d99f54a92d5E,"ax",@progbits
	.p2align	4
	.type	_ZN4core3ops8function6FnOnce9call_once17h1c445d99f54a92d5E,@function
_ZN4core3ops8function6FnOnce9call_once17h1c445d99f54a92d5E:
.Lfunc_begin6:
	.loc	8 250 0 is_stmt 1
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
.Ltmp20:
	leaq	8(%rsp), %rdi
.Ltmp23:
	.loc	8 250 5 prologue_end
	callq	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h975e66f67ae81916E
.Ltmp21:
	movl	%eax, 4(%rsp)
	jmp	.LBB6_3
.LBB6_1:
	.loc	8 250 5
	movq	24(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.LBB6_2:
.Ltmp22:
	.loc	8 0 5 is_stmt 0
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 24(%rsp)
	movl	%eax, 32(%rsp)
	jmp	.LBB6_1
.LBB6_3:
	movl	4(%rsp), %eax
	.loc	8 250 5 epilogue_begin
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp24:
.Lfunc_end6:
	.size	_ZN4core3ops8function6FnOnce9call_once17h1c445d99f54a92d5E, .Lfunc_end6-_ZN4core3ops8function6FnOnce9call_once17h1c445d99f54a92d5E
	.cfi_endproc
	.section	.gcc_except_table._ZN4core3ops8function6FnOnce9call_once17h1c445d99f54a92d5E,"a",@progbits
	.p2align	2, 0x0
GCC_except_table6:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp20-.Lfunc_begin6
	.uleb128 .Ltmp21-.Ltmp20
	.uleb128 .Ltmp22-.Lfunc_begin6
	.byte	0
	.uleb128 .Ltmp21-.Lfunc_begin6
	.uleb128 .Lfunc_end6-.Ltmp21
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	.text._ZN4core3ops8function6FnOnce9call_once17hd703a48edda16cd5E,"ax",@progbits
	.p2align	4
	.type	_ZN4core3ops8function6FnOnce9call_once17hd703a48edda16cd5E,@function
_ZN4core3ops8function6FnOnce9call_once17hd703a48edda16cd5E:
.Lfunc_begin7:
	.loc	8 250 0 is_stmt 1
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, 16(%rsp)
.Ltmp25:
	.loc	8 250 5 prologue_end
	callq	*%rdi
	.loc	8 250 5 epilogue_begin is_stmt 0
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp26:
.Lfunc_end7:
	.size	_ZN4core3ops8function6FnOnce9call_once17hd703a48edda16cd5E, .Lfunc_end7-_ZN4core3ops8function6FnOnce9call_once17hd703a48edda16cd5E
	.cfi_endproc

	.section	".text._ZN4core3ptr31drop_in_place$LT$dp_ex..Cat$GT$17h9aa1b5c63807ed14E","ax",@progbits
	.p2align	4
	.type	_ZN4core3ptr31drop_in_place$LT$dp_ex..Cat$GT$17h9aa1b5c63807ed14E,@function
_ZN4core3ptr31drop_in_place$LT$dp_ex..Cat$GT$17h9aa1b5c63807ed14E:
.Lfunc_begin8:
	.file	9 "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/ptr" "mod.rs"
	.loc	9 523 0 is_stmt 1
	.cfi_startproc
	movq	%rdi, -8(%rsp)
.Ltmp27:
	.loc	9 523 1 prologue_end
	retq
.Ltmp28:
.Lfunc_end8:
	.size	_ZN4core3ptr31drop_in_place$LT$dp_ex..Cat$GT$17h9aa1b5c63807ed14E, .Lfunc_end8-_ZN4core3ptr31drop_in_place$LT$dp_ex..Cat$GT$17h9aa1b5c63807ed14E
	.cfi_endproc

	.section	".text._ZN4core3ptr31drop_in_place$LT$dp_ex..Dog$GT$17haa225f289cc6a195E","ax",@progbits
	.p2align	4
	.type	_ZN4core3ptr31drop_in_place$LT$dp_ex..Dog$GT$17haa225f289cc6a195E,@function
_ZN4core3ptr31drop_in_place$LT$dp_ex..Dog$GT$17haa225f289cc6a195E:
.Lfunc_begin9:
	.loc	9 523 0
	.cfi_startproc
	movq	%rdi, -8(%rsp)
.Ltmp29:
	.loc	9 523 1 prologue_end
	retq
.Ltmp30:
.Lfunc_end9:
	.size	_ZN4core3ptr31drop_in_place$LT$dp_ex..Dog$GT$17haa225f289cc6a195E, .Lfunc_end9-_ZN4core3ptr31drop_in_place$LT$dp_ex..Dog$GT$17haa225f289cc6a195E
	.cfi_endproc

	.section	".text._ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17h4290c5a380cfe87dE","ax",@progbits
	.p2align	4
	.type	_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17h4290c5a380cfe87dE,@function
_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17h4290c5a380cfe87dE:
.Lfunc_begin10:
	.loc	9 523 0
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, (%rsp)
.Ltmp31:
	.loc	9 523 1 prologue_end
	callq	*_ZN4core3ptr171drop_in_place$LT$alloc..rc..Rc$LT$core..cell..UnsafeCell$LT$rand..rngs..reseeding..ReseedingRng$LT$rand_chacha..chacha..ChaCha12Core$C$rand_core..os..OsRng$GT$$GT$$GT$$GT$17h7cd476027919055aE@GOTPCREL(%rip)
	.loc	9 523 1 epilogue_begin is_stmt 0
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Ltmp32:
.Lfunc_end10:
	.size	_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17h4290c5a380cfe87dE, .Lfunc_end10-_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17h4290c5a380cfe87dE
	.cfi_endproc

	.section	".text._ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0c91bb19f0a043fdE","ax",@progbits
	.p2align	4
	.type	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0c91bb19f0a043fdE,@function
_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0c91bb19f0a043fdE:
.Lfunc_begin11:
	.loc	9 523 0 is_stmt 1
	.cfi_startproc
	movq	%rdi, -8(%rsp)
.Ltmp33:
	.loc	9 523 1 prologue_end
	retq
.Ltmp34:
.Lfunc_end11:
	.size	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0c91bb19f0a043fdE, .Lfunc_end11-_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0c91bb19f0a043fdE
	.cfi_endproc

	.section	.text._ZN4rand3rng3Rng6random17he47c0a49024989aaE,"ax",@progbits
	.p2align	4
	.type	_ZN4rand3rng3Rng6random17he47c0a49024989aaE,@function
_ZN4rand3rng3Rng6random17he47c0a49024989aaE:
.Lfunc_begin12:
	.file	10 "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src" "rng.rs"
	.loc	10 95 0
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, %rsi
	movq	%rsi, (%rsp)
.Ltmp35:
	.loc	10 99 9 prologue_end
	movl	$1, %edi
	callq	_ZN4rand5distr5other110_$LT$impl$u20$rand..distr..distribution..Distribution$LT$bool$GT$$u20$for$u20$rand..distr..StandardUniform$GT$6sample17he90ea3740add583dE
	.loc	10 100 6
	andb	$1, %al
	.loc	10 100 6 epilogue_begin is_stmt 0
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Ltmp36:
.Lfunc_end12:
	.size	_ZN4rand3rng3Rng6random17he47c0a49024989aaE, .Lfunc_end12-_ZN4rand3rng3Rng6random17he47c0a49024989aaE
	.cfi_endproc

	.section	".text._ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17h13935bc7e637f19cE","ax",@progbits
	.p2align	4
	.type	_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17h13935bc7e637f19cE,@function
_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17h13935bc7e637f19cE:
.Lfunc_begin13:
	.loc	1 216 0 is_stmt 1
	.cfi_startproc
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	movq	%rdi, 16(%rsp)
	movq	%rsi, 32(%rsp)
	movq	%rdi, 48(%rsp)
.Ltmp37:
	.loc	1 219 13 prologue_end
	movb	$0, 47(%rsp)
	.loc	1 219 37 is_stmt 0
	leaq	32(%rsp), %rdi
	callq	_ZN63_$LT$$RF$mut$u20$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17hcf4f85feb7206a28E
	movq	%rax, %rdi
	movq	%rdx, %rsi
	.loc	1 219 25
	callq	_ZN4core3mem11size_of_val17h6d9e552afa58affdE
	movq	16(%rsp), %rdi
	movq	%rax, 24(%rsp)
	movq	%rax, 56(%rsp)
.Ltmp38:
	.loc	1 221 25 is_stmt 1
	movb	$1, 47(%rsp)
	callq	*_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$6reseed17h8e26f388e23e8972E@GOTPCREL(%rip)
	movl	%eax, 40(%rsp)
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpl	$0, 40(%rsp)
	cmoveq	%rcx, %rax
	.loc	1 221 16 is_stmt 0
	testq	$1, %rax
	je	.LBB13_2
	.loc	1 221 20
	movb	$0, 47(%rsp)
	movl	40(%rsp), %eax
	movl	%eax, 68(%rsp)
.Ltmp39:
.LBB13_2:
	.loc	1 224 9 is_stmt 1
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpl	$0, 40(%rsp)
	cmoveq	%rcx, %rax
	testq	$1, %rax
	je	.LBB13_4
	testb	$1, 47(%rsp)
	jne	.LBB13_5
.LBB13_4:
	.loc	1 0 9 is_stmt 0
	movq	24(%rsp), %rcx
	movq	16(%rsp), %rax
	.loc	1 224 9
	movb	$0, 47(%rsp)
	.loc	1 226 35 is_stmt 1
	movq	48(%rax), %rax
	subq	%rcx, %rax
	movq	%rax, 8(%rsp)
	seto	%al
	jo	.LBB13_7
	jmp	.LBB13_6
.LBB13_5:
	.loc	1 224 9
	jmp	.LBB13_4
.LBB13_6:
	.loc	1 0 9 is_stmt 0
	movq	16(%rsp), %rdi
	movq	8(%rsp), %rax
	.loc	1 226 9 is_stmt 1
	movq	%rax, 56(%rdi)
	.loc	1 227 9
	movq	32(%rsp), %rsi
	callq	_ZN84_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h30baf41324780ba0E
.Ltmp40:
	.loc	1 228 6 epilogue_begin
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB13_7:
	.cfi_def_cfa_offset 80
.Ltmp41:
	.loc	1 226 35
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.3(%rip), %rdi
	callq	*_ZN4core9panicking11panic_const24panic_const_sub_overflow17hab14e776658d8858E@GOTPCREL(%rip)
.Ltmp42:
.Lfunc_end13:
	.size	_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17h13935bc7e637f19cE, .Lfunc_end13-_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17h13935bc7e637f19cE
	.cfi_endproc

	.section	".text._ZN4rand5distr5other110_$LT$impl$u20$rand..distr..distribution..Distribution$LT$bool$GT$$u20$for$u20$rand..distr..StandardUniform$GT$6sample17he90ea3740add583dE","ax",@progbits
	.p2align	4
	.type	_ZN4rand5distr5other110_$LT$impl$u20$rand..distr..distribution..Distribution$LT$bool$GT$$u20$for$u20$rand..distr..StandardUniform$GT$6sample17he90ea3740add583dE,@function
_ZN4rand5distr5other110_$LT$impl$u20$rand..distr..distribution..Distribution$LT$bool$GT$$u20$for$u20$rand..distr..StandardUniform$GT$6sample17he90ea3740add583dE:
.Lfunc_begin14:
	.file	11 "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/distr" "other.rs"
	.loc	11 198 0
	.cfi_startproc
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	movq	%rdi, 16(%rsp)
	movq	%rsi, 24(%rsp)
	movq	%rsi, 32(%rsp)
	movq	%rsi, 56(%rsp)
.Ltmp43:
	.file	12 "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/ptr" "non_null.rs"
	.loc	12 428 20 prologue_end
	movq	(%rsi), %rax
.Ltmp44:
	.file	13 "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/alloc/src" "rc.rs"
	.loc	13 2244 9
	addq	$16, %rax
	movq	%rax, 8(%rsp)
	movq	%rax, 48(%rsp)
.Ltmp45:
	.file	14 "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/rngs" "thread.rs"
	.loc	14 173 28
	cmpq	$0, %rax
	sete	%al
	xorb	$-1, %al
	testb	$1, %al
	jne	.LBB14_2
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.5(%rip), %rdi
	callq	*_ZN4core9panicking30panic_null_pointer_dereference17hf679854e69c25502E@GOTPCREL(%rip)
.LBB14_2:
	.loc	14 0 28 is_stmt 0
	movq	8(%rsp), %rdi
	.loc	14 173 28
	movq	%rdi, 40(%rsp)
	movq	%rdi, 64(%rsp)
.Ltmp46:
	.loc	1 114 9 is_stmt 1
	callq	_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h5ba689e9c842bbddE
.Ltmp47:
	.loc	11 203 9
	cmpl	$0, %eax
	setl	%al
	.loc	11 204 6
	andb	$1, %al
	.loc	11 204 6 epilogue_begin is_stmt 0
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp48:
.Lfunc_end14:
	.size	_ZN4rand5distr5other110_$LT$impl$u20$rand..distr..distribution..Distribution$LT$bool$GT$$u20$for$u20$rand..distr..StandardUniform$GT$6sample17he90ea3740add583dE, .Lfunc_end14-_ZN4rand5distr5other110_$LT$impl$u20$rand..distr..distribution..Distribution$LT$bool$GT$$u20$for$u20$rand..distr..StandardUniform$GT$6sample17he90ea3740add583dE
	.cfi_endproc

	.section	".text._ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h61953a0fd5719ceaE","ax",@progbits
	.p2align	4
	.type	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h61953a0fd5719ceaE,@function
_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h61953a0fd5719ceaE:
.Lfunc_begin15:
	.cfi_startproc
	.loc	4 2434 6 prologue_end is_stmt 1
	xorl	%eax, %eax
	retq
.Ltmp49:
.Lfunc_end15:
	.size	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h61953a0fd5719ceaE, .Lfunc_end15-_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h61953a0fd5719ceaE
	.cfi_endproc

	.section	".text._ZN63_$LT$$RF$mut$u20$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17hcf4f85feb7206a28E","ax",@progbits
	.p2align	4
	.type	_ZN63_$LT$$RF$mut$u20$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17hcf4f85feb7206a28E,@function
_ZN63_$LT$$RF$mut$u20$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17hcf4f85feb7206a28E:
.Lfunc_begin16:
	.file	15 "/rustc/ecade534c66478c51c5d3c1d3682dc4beb0ac972/library/core/src/convert" "mod.rs"
	.loc	15 715 0
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, (%rsp)
.Ltmp50:
	.loc	15 716 33 prologue_end
	movq	(%rdi), %rdi
	.loc	15 716 9 is_stmt 0
	movq	_ZN97_$LT$rand_chacha..chacha..Array64$LT$T$GT$$u20$as$u20$core..convert..AsRef$LT$$u5b$T$u5d$$GT$$GT$6as_ref17hc3771d368a6e604aE@GOTPCREL(%rip), %rax
	callq	*%rax
	.loc	15 717 6 epilogue_begin is_stmt 1
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Ltmp51:
.Lfunc_end16:
	.size	_ZN63_$LT$$RF$mut$u20$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17hcf4f85feb7206a28E, .Lfunc_end16-_ZN63_$LT$$RF$mut$u20$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17hcf4f85feb7206a28E
	.cfi_endproc

	.section	".text._ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h5ba689e9c842bbddE","ax",@progbits
	.p2align	4
	.type	_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h5ba689e9c842bbddE,@function
_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h5ba689e9c842bbddE:
.Lfunc_begin17:
	.file	16 "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_core-0.9.3/src" "block.rs"
	.loc	16 186 0
	.cfi_startproc
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	movq	%rdi, 40(%rsp)
	movq	%rdi, 56(%rsp)
.Ltmp52:
	.loc	16 187 12 prologue_end
	movq	320(%rdi), %rax
	movq	%rax, 48(%rsp)
	.loc	16 187 26 is_stmt 0
	callq	*_ZN97_$LT$rand_chacha..chacha..Array64$LT$T$GT$$u20$as$u20$core..convert..AsRef$LT$$u5b$T$u5d$$GT$$GT$6as_ref17hc3771d368a6e604aE@GOTPCREL(%rip)
	movq	48(%rsp), %rax
	.loc	16 187 12
	cmpq	%rdx, %rax
	jae	.LBB17_2
.LBB17_1:
	.loc	16 0 12
	movq	40(%rsp), %rdi
	.loc	16 191 21 is_stmt 1
	callq	*_ZN97_$LT$rand_chacha..chacha..Array64$LT$T$GT$$u20$as$u20$core..convert..AsRef$LT$$u5b$T$u5d$$GT$$GT$6as_ref17hc3771d368a6e604aE@GOTPCREL(%rip)
	movq	%rax, %rcx
	movq	40(%rsp), %rax
	movq	%rcx, 16(%rsp)
	movq	%rdx, 24(%rsp)
	.loc	16 191 43 is_stmt 0
	movq	320(%rax), %rax
	movq	%rax, 32(%rsp)
	.loc	16 191 21
	cmpq	%rdx, %rax
	jb	.LBB17_3
	jmp	.LBB17_4
.LBB17_2:
	.loc	16 0 21
	movq	40(%rsp), %rdi
	.loc	16 188 13 is_stmt 1
	xorl	%eax, %eax
	movl	%eax, %esi
	callq	_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h815b35b30d5e012dE
	jmp	.LBB17_1
.LBB17_3:
	.loc	16 0 13 is_stmt 0
	movq	40(%rsp), %rax
	movq	16(%rsp), %rcx
	movq	32(%rsp), %rdx
	.loc	16 191 21 is_stmt 1
	movl	(%rcx,%rdx,4), %ecx
	movl	%ecx, 4(%rsp)
	movl	%ecx, 68(%rsp)
.Ltmp53:
	.loc	16 192 9
	movq	320(%rax), %rax
	addq	$1, %rax
	movq	%rax, 8(%rsp)
	setb	%al
	jb	.LBB17_6
	jmp	.LBB17_5
.Ltmp54:
.LBB17_4:
	.loc	16 0 9 is_stmt 0
	movq	24(%rsp), %rsi
	movq	32(%rsp), %rdi
	.loc	16 191 21 is_stmt 1
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.7(%rip), %rdx
	callq	*_ZN4core9panicking18panic_bounds_check17h8ced4c0cdca08f31E@GOTPCREL(%rip)
.LBB17_5:
	.loc	16 0 21 is_stmt 0
	movl	4(%rsp), %eax
	movq	40(%rsp), %rcx
	movq	8(%rsp), %rdx
.Ltmp55:
	.loc	16 192 9 is_stmt 1
	movq	%rdx, 320(%rcx)
.Ltmp56:
	.loc	16 194 6 epilogue_begin
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB17_6:
	.cfi_def_cfa_offset 80
.Ltmp57:
	.loc	16 192 9
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.8(%rip), %rdi
	callq	*_ZN4core9panicking11panic_const24panic_const_add_overflow17hf9fdf5c752a7aa23E@GOTPCREL(%rip)
.Ltmp58:
.Lfunc_end17:
	.size	_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h5ba689e9c842bbddE, .Lfunc_end17-_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h5ba689e9c842bbddE
	.cfi_endproc

	.section	".text._ZN84_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h30baf41324780ba0E","ax",@progbits
	.p2align	4
	.type	_ZN84_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h30baf41324780ba0E,@function
_ZN84_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h30baf41324780ba0E:
.Lfunc_begin18:
	.file	17 "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_chacha-0.9.0/src" "chacha.rs"
	.loc	17 90 0
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rsi, %rdx
	movq	%rdi, (%rsp)
	movq	%rdx, 8(%rsp)
	movq	%rdi, 16(%rsp)
	movl	$6, 28(%rsp)
	movq	%rdx, 32(%rsp)
.Ltmp59:
	.file	18 "/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_chacha-0.9.0/src" "guts.rs"
	.loc	18 81 9 prologue_end
	movl	$6, %esi
	callq	*_ZN11rand_chacha4guts11refill_wide17hb2f7c5a521484136E@GOTPCREL(%rip)
.Ltmp60:
	.loc	17 92 14 epilogue_begin
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp61:
.Lfunc_end18:
	.size	_ZN84_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h30baf41324780ba0E, .Lfunc_end18-_ZN84_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h30baf41324780ba0E
	.cfi_endproc

	.section	".text._ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h815b35b30d5e012dE","ax",@progbits
	.p2align	4
	.type	_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h815b35b30d5e012dE,@function
_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h815b35b30d5e012dE:
.Lfunc_begin19:
	.loc	16 177 0
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
	movq	%rsi, 16(%rsp)
	movq	%rdi, 24(%rsp)
	movq	%rsi, 32(%rsp)
.Ltmp62:
	.loc	16 178 25 prologue_end
	callq	*_ZN97_$LT$rand_chacha..chacha..Array64$LT$T$GT$$u20$as$u20$core..convert..AsRef$LT$$u5b$T$u5d$$GT$$GT$6as_ref17hc3771d368a6e604aE@GOTPCREL(%rip)
	movq	16(%rsp), %rsi
	.loc	16 178 17 is_stmt 0
	cmpq	%rdx, %rsi
	jb	.LBB19_2
	.loc	16 178 9
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.9(%rip), %rdi
	movl	$53, %esi
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.10(%rip), %rdx
	callq	*_ZN4core9panicking5panic17h4e529a542afba437E@GOTPCREL(%rip)
.LBB19_2:
	.loc	16 0 9
	movq	8(%rsp), %rsi
	.loc	16 179 9 is_stmt 1
	movq	%rsi, %rdi
	addq	$256, %rdi
	callq	_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h65f4881e2030d610E
	movq	16(%rsp), %rcx
	movq	8(%rsp), %rax
	.loc	16 180 9
	movq	%rcx, 320(%rax)
	.loc	16 181 6 epilogue_begin
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp63:
.Lfunc_end19:
	.size	_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h815b35b30d5e012dE, .Lfunc_end19-_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h815b35b30d5e012dE
	.cfi_endproc

	.section	".text._ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17had468e7662b92aa1E","ax",@progbits
	.p2align	4
	.type	_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17had468e7662b92aa1E,@function
_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17had468e7662b92aa1E:
.Lfunc_begin20:
	.file	19 "/home/np/hack/verifopt/dp-ex" "src/main.rs"
	.loc	19 10 0
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, 48(%rsp)
.Ltmp64:
	.loc	19 11 9 prologue_end
	movq	%rsp, %rdi
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.12(%rip), %rsi
	callq	*_ZN4core3fmt9Arguments9new_const17h5a4b4947710cd61bE@GOTPCREL(%rip)
	movq	%rsp, %rdi
	callq	*_ZN3std2io5stdio6_print17he3d109100110923bE@GOTPCREL(%rip)
	.loc	19 12 6 epilogue_begin
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp65:
.Lfunc_end20:
	.size	_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17had468e7662b92aa1E, .Lfunc_end20-_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17had468e7662b92aa1E
	.cfi_endproc

	.section	".text._ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17h504ab8d43430beb1E","ax",@progbits
	.p2align	4
	.type	_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17h504ab8d43430beb1E,@function
_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17h504ab8d43430beb1E:
.Lfunc_begin21:
	.loc	19 18 0
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, 48(%rsp)
.Ltmp66:
	.loc	19 19 9 prologue_end
	movq	%rsp, %rdi
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.14(%rip), %rsi
	callq	*_ZN4core3fmt9Arguments9new_const17h5a4b4947710cd61bE@GOTPCREL(%rip)
	movq	%rsp, %rdi
	callq	*_ZN3std2io5stdio6_print17he3d109100110923bE@GOTPCREL(%rip)
	.loc	19 20 6 epilogue_begin
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp67:
.Lfunc_end21:
	.size	_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17h504ab8d43430beb1E, .Lfunc_end21-_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17h504ab8d43430beb1E
	.cfi_endproc

	.section	.text._ZN5dp_ex6dyn_dp17h2163eacfb0f4ef0dE,"ax",@progbits
	.p2align	4
	.type	_ZN5dp_ex6dyn_dp17h2163eacfb0f4ef0dE,@function
_ZN5dp_ex6dyn_dp17h2163eacfb0f4ef0dE:
.Lfunc_begin22:
	.loc	19 23 0
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception1
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
.Ltmp76:
	.loc	19 26 19 prologue_end
	movq	_ZN4rand4rngs6thread3rng17h445d2230ed7f88faE@GOTPCREL(%rip), %rax
	callq	*%rax
	movq	%rax, 32(%rsp)
.Ltmp68:
	leaq	32(%rsp), %rdi
.Ltmp77:
	.loc	19 27 8
	callq	_ZN4rand3rng3Rng6random17he47c0a49024989aaE
.Ltmp69:
	movb	%al, 15(%rsp)
	jmp	.LBB22_3
.Ltmp78:
.LBB22_1:
.Ltmp73:
	.loc	19 0 8 is_stmt 0
	leaq	32(%rsp), %rdi
	.loc	19 34 1 is_stmt 1
	callq	_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17h4290c5a380cfe87dE
.Ltmp74:
	jmp	.LBB22_9
.LBB22_2:
.Ltmp72:
	.loc	19 0 1 is_stmt 0
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 40(%rsp)
	movl	%eax, 48(%rsp)
	jmp	.LBB22_1
.LBB22_3:
	movb	15(%rsp), %al
.Ltmp79:
	.loc	19 27 8 is_stmt 1
	testb	$1, %al
	jne	.LBB22_5
	jmp	.LBB22_4
.LBB22_4:
	.loc	19 30 9
	movl	$1, %eax
	movq	%rax, 16(%rsp)
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.15(%rip), %rax
	movq	%rax, 24(%rsp)
	.loc	19 27 5
	jmp	.LBB22_6
.LBB22_5:
	.loc	19 28 9
	movl	$1, %eax
	movq	%rax, 16(%rsp)
	leaq	.Lanon.a6fa456c9412015864d7783aa51b16ac.16(%rip), %rax
	movq	%rax, 24(%rsp)
.LBB22_6:
	.loc	19 33 5
	movq	16(%rsp), %rdi
	movq	24(%rsp), %rax
	movq	24(%rax), %rax
.Ltmp70:
	callq	*%rax
.Ltmp71:
	jmp	.LBB22_7
.Ltmp80:
.LBB22_7:
	.loc	19 34 1
	leaq	32(%rsp), %rdi
	callq	_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17h4290c5a380cfe87dE
.Ltmp81:
	.loc	19 34 2 epilogue_begin is_stmt 0
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB22_8:
	.cfi_def_cfa_offset 64
.Ltmp75:
	.loc	19 23 1 is_stmt 1
	callq	*_ZN4core9panicking16panic_in_cleanup17h4898e2af8dc1f529E@GOTPCREL(%rip)
.LBB22_9:
	.loc	19 23 1
	movq	40(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.Ltmp82:
.Lfunc_end22:
	.size	_ZN5dp_ex6dyn_dp17h2163eacfb0f4ef0dE, .Lfunc_end22-_ZN5dp_ex6dyn_dp17h2163eacfb0f4ef0dE
	.cfi_endproc
	.section	.gcc_except_table._ZN5dp_ex6dyn_dp17h2163eacfb0f4ef0dE,"a",@progbits
	.p2align	2, 0x0
GCC_except_table22:
.Lexception1:
	.byte	255
	.byte	155
	.uleb128 .Lttbase0-.Lttbaseref0
.Lttbaseref0:
	.byte	1
	.uleb128 .Lcst_end1-.Lcst_begin1
.Lcst_begin1:
	.uleb128 .Lfunc_begin22-.Lfunc_begin22
	.uleb128 .Ltmp68-.Lfunc_begin22
	.byte	0
	.byte	0
	.uleb128 .Ltmp68-.Lfunc_begin22
	.uleb128 .Ltmp69-.Ltmp68
	.uleb128 .Ltmp72-.Lfunc_begin22
	.byte	0
	.uleb128 .Ltmp73-.Lfunc_begin22
	.uleb128 .Ltmp74-.Ltmp73
	.uleb128 .Ltmp75-.Lfunc_begin22
	.byte	1
	.uleb128 .Ltmp70-.Lfunc_begin22
	.uleb128 .Ltmp71-.Ltmp70
	.uleb128 .Ltmp72-.Lfunc_begin22
	.byte	0
	.uleb128 .Ltmp71-.Lfunc_begin22
	.uleb128 .Lfunc_end22-.Ltmp71
	.byte	0
	.byte	0
.Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	.text._ZN5dp_ex9static_dp17h12c823b24041d8eeE,"ax",@progbits
	.p2align	4
	.type	_ZN5dp_ex9static_dp17h12c823b24041d8eeE,@function
_ZN5dp_ex9static_dp17h12c823b24041d8eeE:
.Lfunc_begin23:
	.loc	19 60 0
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
.Ltmp83:
	.loc	19 61 15 prologue_end
	movl	$1, %eax
	movq	%rax, (%rsp)
.Ltmp84:
	.loc	19 62 5
	movl	$1, %edi
	callq	_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17h504ab8d43430beb1E
.Ltmp85:
	.loc	19 63 2 epilogue_begin
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Ltmp86:
.Lfunc_end23:
	.size	_ZN5dp_ex9static_dp17h12c823b24041d8eeE, .Lfunc_end23-_ZN5dp_ex9static_dp17h12c823b24041d8eeE
	.cfi_endproc

	.section	.text._ZN5dp_ex4main17hfaa1a74bb1a25d42E,"ax",@progbits
	.p2align	4
	.type	_ZN5dp_ex4main17hfaa1a74bb1a25d42E,@function
_ZN5dp_ex4main17hfaa1a74bb1a25d42E:
.Lfunc_begin24:
	.loc	19 65 0
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
.Ltmp87:
	.loc	19 80 5 prologue_end
	callq	_ZN5dp_ex6dyn_dp17h2163eacfb0f4ef0dE
	.loc	19 81 5
	callq	_ZN5dp_ex9static_dp17h12c823b24041d8eeE
	.loc	19 82 2 epilogue_begin
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Ltmp88:
.Lfunc_end24:
	.size	_ZN5dp_ex4main17hfaa1a74bb1a25d42E, .Lfunc_end24-_ZN5dp_ex4main17hfaa1a74bb1a25d42E
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4
	.type	main,@function
main:
.Lfunc_begin25:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rdx
	movq	__rustc_debug_gdb_scripts_section__@GOTPCREL(%rip), %rax
	movb	(%rax), %al
	movslq	%edi, %rsi
	leaq	_ZN5dp_ex4main17hfaa1a74bb1a25d42E(%rip), %rdi
	xorl	%ecx, %ecx
	callq	_ZN3std2rt10lang_start17h27e2b9e2c6bd9d09E
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end25:
	.size	main, .Lfunc_end25-main
	.cfi_endproc

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.0,@object
	.section	.rodata..Lanon.a6fa456c9412015864d7783aa51b16ac.0,"a",@progbits
.Lanon.a6fa456c9412015864d7783aa51b16ac.0:
	.ascii	"/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/rngs/reseeding.rs"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.0, 94

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.1,@object
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.1,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a6fa456c9412015864d7783aa51b16ac.1:
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.0
	.asciz	"^\000\000\000\000\000\000\000\252\000\000\000\t\000\000"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.1, 24

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.2,@object
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.2,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a6fa456c9412015864d7783aa51b16ac.2:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hb06e99ac8656f6d6E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h975e66f67ae81916E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h975e66f67ae81916E
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.2, 48

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.3,@object
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.3,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a6fa456c9412015864d7783aa51b16ac.3:
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.0
	.asciz	"^\000\000\000\000\000\000\000\342\000\000\000#\000\000"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.3, 24

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.4,@object
	.section	.rodata..Lanon.a6fa456c9412015864d7783aa51b16ac.4,"a",@progbits
.Lanon.a6fa456c9412015864d7783aa51b16ac.4:
	.ascii	"/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.2/src/rngs/thread.rs"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.4, 91

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.5,@object
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.5,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a6fa456c9412015864d7783aa51b16ac.5:
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.4
	.asciz	"[\000\000\000\000\000\000\000\255\000\000\000\034\000\000"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.5, 24

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.6,@object
	.section	.rodata..Lanon.a6fa456c9412015864d7783aa51b16ac.6,"a",@progbits
.Lanon.a6fa456c9412015864d7783aa51b16ac.6:
	.ascii	"/home/np/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rand_core-0.9.3/src/block.rs"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.6, 90

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.7,@object
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.7,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a6fa456c9412015864d7783aa51b16ac.7:
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.6
	.asciz	"Z\000\000\000\000\000\000\000\277\000\000\000\025\000\000"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.7, 24

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.8,@object
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.8,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a6fa456c9412015864d7783aa51b16ac.8:
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.6
	.asciz	"Z\000\000\000\000\000\000\000\300\000\000\000\t\000\000"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.8, 24

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.9,@object
	.section	.rodata..Lanon.a6fa456c9412015864d7783aa51b16ac.9,"a",@progbits
.Lanon.a6fa456c9412015864d7783aa51b16ac.9:
	.ascii	"assertion failed: index < self.results.as_ref().len()"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.9, 53

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.10,@object
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.10,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a6fa456c9412015864d7783aa51b16ac.10:
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.6
	.asciz	"Z\000\000\000\000\000\000\000\262\000\000\000\t\000\000"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.10, 24

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.11,@object
	.section	.rodata..Lanon.a6fa456c9412015864d7783aa51b16ac.11,"a",@progbits
.Lanon.a6fa456c9412015864d7783aa51b16ac.11:
	.ascii	"woof\n"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.11, 5

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.12,@object
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.12,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a6fa456c9412015864d7783aa51b16ac.12:
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.11
	.asciz	"\005\000\000\000\000\000\000"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.12, 16

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.13,@object
	.section	.rodata..Lanon.a6fa456c9412015864d7783aa51b16ac.13,"a",@progbits
.Lanon.a6fa456c9412015864d7783aa51b16ac.13:
	.ascii	"meow\n"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.13, 5

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.14,@object
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.14,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a6fa456c9412015864d7783aa51b16ac.14:
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.13
	.asciz	"\005\000\000\000\000\000\000"
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.14, 16

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.15,@object
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.15,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a6fa456c9412015864d7783aa51b16ac.15:
	.asciz	"\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\001\000\000\000\000\000\000"
	.quad	_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17had468e7662b92aa1E
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.15, 32

	.type	.Lanon.a6fa456c9412015864d7783aa51b16ac.16,@object
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.16,"aw",@progbits
	.p2align	3, 0x0
.Lanon.a6fa456c9412015864d7783aa51b16ac.16:
	.asciz	"\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\001\000\000\000\000\000\000"
	.quad	_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17h504ab8d43430beb1E
	.size	.Lanon.a6fa456c9412015864d7783aa51b16ac.16, 32

	.type	__rustc_debug_gdb_scripts_section__,@object
	.section	.debug_gdb_scripts,"aMS",@progbits,1,unique,1
	.weak	__rustc_debug_gdb_scripts_section__
__rustc_debug_gdb_scripts_section__:
	.asciz	"\001gdb_load_rust_pretty_printers.py"
	.size	__rustc_debug_gdb_scripts_section__, 34

	.section	.debug_abbrev,"",@progbits
	.byte	1
	.byte	17
	.byte	1
	.byte	37
	.byte	14
	.byte	19
	.byte	5
	.byte	3
	.byte	14
	.byte	16
	.byte	23
	.byte	27
	.byte	14
	.byte	17
	.byte	1
	.byte	85
	.byte	23
	.byte	0
	.byte	0
	.byte	2
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.byte	2
	.byte	24
	.byte	0
	.byte	0
	.byte	3
	.byte	19
	.byte	1
	.byte	29
	.byte	19
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	4
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	0
	.byte	0
	.byte	5
	.byte	15
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	51
	.byte	6
	.byte	0
	.byte	0
	.byte	6
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	62
	.byte	11
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	7
	.byte	57
	.byte	1
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	8
	.byte	19
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	9
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	10
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.ascii	"\210\001"
	.byte	15
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	11
	.byte	29
	.byte	1
	.byte	49
	.byte	19
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	88
	.byte	11
	.byte	89
	.byte	11
	.byte	87
	.byte	11
	.byte	0
	.byte	0
	.byte	12
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	49
	.byte	19
	.byte	0
	.byte	0
	.byte	13
	.byte	29
	.byte	0
	.byte	49
	.byte	19
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	88
	.byte	11
	.byte	89
	.byte	5
	.byte	87
	.byte	11
	.byte	0
	.byte	0
	.byte	14
	.byte	47
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	15
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	16
	.byte	19
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	50
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	17
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	50
	.byte	11
	.byte	0
	.byte	0
	.byte	18
	.byte	46
	.byte	1
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	60
	.byte	25
	.byte	0
	.byte	0
	.byte	19
	.byte	5
	.byte	0
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	20
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	0
	.byte	0
	.byte	21
	.byte	11
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	0
	.byte	0
	.byte	22
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	49
	.byte	19
	.byte	0
	.byte	0
	.byte	23
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	24
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	25
	.byte	21
	.byte	0
	.byte	0
	.byte	0
	.byte	26
	.byte	19
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	50
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	27
	.byte	11
	.byte	1
	.byte	85
	.byte	23
	.byte	0
	.byte	0
	.byte	28
	.byte	46
	.byte	0
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	106
	.byte	25
	.byte	0
	.byte	0
	.byte	29
	.byte	46
	.byte	1
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	60
	.byte	25
	.byte	0
	.byte	0
	.byte	30
	.byte	19
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	5
	.byte	50
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	31
	.byte	46
	.byte	1
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	32
	.byte	11
	.byte	0
	.byte	0
	.byte	32
	.byte	5
	.byte	0
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	33
	.byte	11
	.byte	1
	.byte	0
	.byte	0
	.byte	34
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.ascii	"\210\001"
	.byte	15
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	35
	.byte	29
	.byte	1
	.byte	49
	.byte	19
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	88
	.byte	11
	.byte	89
	.byte	5
	.byte	87
	.byte	11
	.byte	0
	.byte	0
	.byte	36
	.byte	46
	.byte	0
	.byte	71
	.byte	19
	.byte	32
	.byte	11
	.byte	0
	.byte	0
	.byte	37
	.byte	46
	.byte	1
	.byte	71
	.byte	19
	.byte	32
	.byte	11
	.byte	0
	.byte	0
	.byte	38
	.byte	5
	.byte	0
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	39
	.byte	46
	.byte	1
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	32
	.byte	11
	.byte	0
	.byte	0
	.byte	40
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.ascii	"\210\001"
	.byte	15
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	41
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	42
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	43
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	0
	.byte	0
	.byte	44
	.byte	23
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	45
	.byte	1
	.byte	1
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	46
	.byte	33
	.byte	0
	.byte	73
	.byte	19
	.byte	34
	.byte	13
	.byte	55
	.byte	11
	.byte	0
	.byte	0
	.byte	47
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	62
	.byte	11
	.byte	0
	.byte	0
	.byte	48
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	5
	.byte	50
	.byte	11
	.byte	0
	.byte	0
	.byte	49
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	71
	.byte	19
	.byte	0
	.byte	0
	.byte	50
	.byte	46
	.byte	1
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	32
	.byte	11
	.byte	0
	.byte	0
	.byte	51
	.byte	15
	.byte	0
	.byte	73
	.byte	19
	.byte	51
	.byte	6
	.byte	0
	.byte	0
	.byte	52
	.byte	19
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	0
	.section	.debug_info,"",@progbits
.Lcu_begin0:
	.long	.Ldebug_info_end0-.Ldebug_info_start0
.Ldebug_info_start0:
	.short	4
	.long	.debug_abbrev
	.byte	8
	.byte	1
	.long	.Linfo_string0
	.short	28
	.long	.Linfo_string1
	.long	.Lline_table_start0
	.long	.Linfo_string2
	.quad	0
	.long	.Ldebug_ranges4
	.byte	2
	.long	.Linfo_string3
	.long	61
	.byte	9
	.byte	3
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.2
	.byte	3
	.long	181
	.long	.Linfo_string19
	.byte	48
	.byte	8
	.byte	4
	.long	.Linfo_string4
	.long	139
	.byte	8
	.byte	0
	.byte	4
	.long	.Linfo_string7
	.long	159
	.byte	8
	.byte	8
	.byte	4
	.long	.Linfo_string9
	.long	159
	.byte	8
	.byte	16
	.byte	4
	.long	.Linfo_string10
	.long	139
	.byte	8
	.byte	24
	.byte	4
	.long	.Linfo_string11
	.long	139
	.byte	8
	.byte	32
	.byte	4
	.long	.Linfo_string12
	.long	139
	.byte	8
	.byte	40
	.byte	0
	.byte	5
	.long	152
	.long	.Linfo_string6
	.long	0
	.byte	6
	.long	.Linfo_string5
	.byte	7
	.byte	0
	.byte	6
	.long	.Linfo_string8
	.byte	7
	.byte	8
	.byte	7
	.long	.Linfo_string13
	.byte	7
	.long	.Linfo_string14
	.byte	7
	.long	.Linfo_string15
	.byte	8
	.long	.Linfo_string18
	.byte	8
	.byte	8
	.byte	4
	.long	.Linfo_string16
	.long	695
	.byte	8
	.byte	0
	.byte	0
	.byte	9
	.quad	.Lfunc_begin2
	.long	.Lfunc_end2-.Lfunc_begin2
	.byte	1
	.byte	87
	.long	.Linfo_string167
	.long	.Linfo_string168
	.byte	2
	.byte	199
	.long	1863
	.byte	10
	.byte	3
	.byte	145
	.byte	8
	.byte	6
	.long	.Linfo_string16
	.byte	8
	.byte	2
	.byte	193
	.long	695
	.byte	11
	.long	1889
	.quad	.Ltmp9
	.long	.Ltmp10-.Ltmp9
	.byte	2
	.byte	199
	.byte	85
	.byte	12
	.byte	2
	.byte	145
	.byte	23
	.long	1895
	.byte	13
	.long	1883
	.quad	.Ltmp9
	.long	.Ltmp10-.Ltmp9
	.byte	4
	.short	2063
	.byte	16
	.byte	0
	.byte	14
	.long	152
	.long	.Linfo_string48
	.byte	0
	.byte	0
	.byte	9
	.quad	.Lfunc_begin1
	.long	.Lfunc_end1-.Lfunc_begin1
	.byte	1
	.byte	87
	.long	.Linfo_string164
	.long	.Linfo_string165
	.byte	2
	.byte	192
	.long	4083
	.byte	15
	.byte	2
	.byte	145
	.byte	8
	.long	.Linfo_string16
	.byte	2
	.byte	193
	.long	695
	.byte	15
	.byte	2
	.byte	145
	.byte	16
	.long	.Linfo_string215
	.byte	2
	.byte	194
	.long	4083
	.byte	15
	.byte	2
	.byte	145
	.byte	24
	.long	.Linfo_string216
	.byte	2
	.byte	195
	.long	4136
	.byte	15
	.byte	2
	.byte	145
	.byte	39
	.long	.Linfo_string219
	.byte	2
	.byte	196
	.long	1856
	.byte	14
	.long	152
	.long	.Linfo_string48
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string31
	.byte	7
	.long	.Linfo_string32
	.byte	7
	.long	.Linfo_string33
	.byte	7
	.long	.Linfo_string34
	.byte	7
	.long	.Linfo_string35
	.byte	16
	.long	.Linfo_string38
	.byte	1
	.byte	1
	.byte	1
	.byte	17
	.long	.Linfo_string36
	.long	1856
	.byte	1
	.byte	0
	.byte	3
	.byte	18
	.long	.Linfo_string39
	.long	.Linfo_string40
	.byte	3
	.short	630
	.long	1863

	.byte	19
	.long	1870
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string52
	.byte	20
	.quad	.Lfunc_begin3
	.long	.Lfunc_end3-.Lfunc_begin3
	.byte	1
	.byte	87
	.long	.Linfo_string170
	.long	.Linfo_string171
	.byte	5
	.byte	148
	.byte	15
	.byte	2
	.byte	145
	.byte	8
	.long	.Linfo_string221
	.byte	5
	.byte	148
	.long	695
	.byte	21
	.quad	.Ltmp13
	.long	.Ltmp14-.Ltmp13
	.byte	10
	.byte	2
	.byte	145
	.byte	7
	.long	.Linfo_string220
	.byte	1
	.byte	5
	.byte	152
	.long	152
	.byte	11
	.long	1918
	.quad	.Ltmp13
	.long	.Ltmp14-.Ltmp13
	.byte	5
	.byte	155
	.byte	5
	.byte	22
	.byte	2
	.byte	145
	.byte	23
	.long	1940
	.byte	0
	.byte	0
	.byte	14
	.long	695
	.long	.Linfo_string169
	.byte	14
	.long	152
	.long	.Linfo_string48
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string34
	.byte	16
	.long	.Linfo_string38
	.byte	1
	.byte	1
	.byte	1
	.byte	17
	.long	.Linfo_string36
	.long	427
	.byte	1
	.byte	0
	.byte	3
	.byte	18
	.long	.Linfo_string43
	.long	.Linfo_string44
	.byte	4
	.short	2062
	.long	1863

	.byte	19
	.long	602
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string148
	.byte	23
	.quad	.Lfunc_begin15
	.long	.Lfunc_end15-.Lfunc_begin15
	.byte	1
	.byte	87
	.long	.Linfo_string194
	.long	.Linfo_string195
	.byte	4
	.short	2432
	.long	602
	.byte	24
	.byte	2
	.byte	145
	.byte	127
	.byte	4
	.short	2432
	.long	152
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	708
	.long	.Linfo_string17
	.long	0
	.byte	25
	.byte	2
	.long	.Linfo_string20
	.long	728
	.byte	9
	.byte	3
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.15
	.byte	3
	.long	789
	.long	.Linfo_string23
	.byte	32
	.byte	8
	.byte	4
	.long	.Linfo_string4
	.long	139
	.byte	8
	.byte	0
	.byte	4
	.long	.Linfo_string7
	.long	159
	.byte	8
	.byte	8
	.byte	4
	.long	.Linfo_string9
	.long	159
	.byte	8
	.byte	16
	.byte	4
	.long	.Linfo_string10
	.long	139
	.byte	8
	.byte	24
	.byte	0
	.byte	7
	.long	.Linfo_string21
	.byte	26
	.long	.Linfo_string22
	.byte	0
	.byte	3
	.byte	1
	.byte	26
	.long	.Linfo_string25
	.byte	0
	.byte	3
	.byte	1
	.byte	7
	.long	.Linfo_string161
	.byte	20
	.quad	.Lfunc_begin20
	.long	.Lfunc_end20-.Lfunc_begin20
	.byte	1
	.byte	87
	.long	.Linfo_string206
	.long	.Linfo_string207
	.byte	19
	.byte	10
	.byte	15
	.byte	2
	.byte	145
	.byte	48
	.long	.Linfo_string45
	.byte	19
	.byte	10
	.long	4286
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string142
	.byte	20
	.quad	.Lfunc_begin21
	.long	.Lfunc_end21-.Lfunc_begin21
	.byte	1
	.byte	87
	.long	.Linfo_string208
	.long	.Linfo_string207
	.byte	19
	.byte	18
	.byte	15
	.byte	2
	.byte	145
	.byte	48
	.long	.Linfo_string45
	.byte	19
	.byte	18
	.long	4299
	.byte	0
	.byte	0
	.byte	20
	.quad	.Lfunc_begin22
	.long	.Lfunc_end22-.Lfunc_begin22
	.byte	1
	.byte	87
	.long	.Linfo_string209
	.long	.Linfo_string210
	.byte	19
	.byte	23
	.byte	21
	.quad	.Ltmp76
	.long	.Ltmp81-.Ltmp76
	.byte	10
	.byte	2
	.byte	145
	.byte	16
	.long	.Linfo_string244
	.byte	8
	.byte	19
	.byte	24
	.long	4312
	.byte	27
	.long	.Ldebug_ranges3
	.byte	10
	.byte	2
	.byte	145
	.byte	32
	.long	.Linfo_string58
	.byte	8
	.byte	19
	.byte	26
	.long	1504
	.byte	0
	.byte	0
	.byte	0
	.byte	20
	.quad	.Lfunc_begin23
	.long	.Lfunc_end23-.Lfunc_begin23
	.byte	1
	.byte	87
	.long	.Linfo_string211
	.long	.Linfo_string212
	.byte	19
	.byte	60
	.byte	21
	.quad	.Ltmp84
	.long	.Ltmp85-.Ltmp84
	.byte	10
	.byte	2
	.byte	145
	.byte	0
	.long	.Linfo_string249
	.byte	8
	.byte	19
	.byte	61
	.long	4299
	.byte	0
	.byte	0
	.byte	28
	.quad	.Lfunc_begin24
	.long	.Lfunc_end24-.Lfunc_begin24
	.byte	1
	.byte	87
	.long	.Linfo_string213
	.long	.Linfo_string16
	.byte	19
	.byte	65

	.byte	0
	.byte	2
	.long	.Linfo_string24
	.long	1073
	.byte	9
	.byte	3
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.16
	.byte	3
	.long	797
	.long	.Linfo_string26
	.byte	32
	.byte	8
	.byte	4
	.long	.Linfo_string4
	.long	139
	.byte	8
	.byte	0
	.byte	4
	.long	.Linfo_string7
	.long	159
	.byte	8
	.byte	8
	.byte	4
	.long	.Linfo_string9
	.long	159
	.byte	8
	.byte	16
	.byte	4
	.long	.Linfo_string10
	.long	139
	.byte	8
	.byte	24
	.byte	0
	.byte	7
	.long	.Linfo_string27
	.byte	7
	.long	.Linfo_string28
	.byte	7
	.long	.Linfo_string29
	.byte	7
	.long	.Linfo_string30
	.byte	20
	.quad	.Lfunc_begin0
	.long	.Lfunc_end0-.Lfunc_begin0
	.byte	1
	.byte	87
	.long	.Linfo_string162
	.long	.Linfo_string163
	.byte	1
	.byte	162
	.byte	15
	.byte	2
	.byte	145
	.byte	24
	.long	.Linfo_string45
	.byte	1
	.byte	162
	.long	3409
	.byte	15
	.byte	2
	.byte	145
	.byte	16
	.long	.Linfo_string104
	.byte	1
	.byte	162
	.long	3422
	.byte	27
	.long	.Ldebug_ranges0
	.byte	10
	.byte	2
	.byte	145
	.byte	32
	.long	.Linfo_string214
	.byte	8
	.byte	1
	.byte	169
	.long	159
	.byte	0
	.byte	14
	.long	2805
	.long	.Linfo_string84
	.byte	14
	.long	3203
	.long	.Linfo_string88
	.byte	0
	.byte	0
	.byte	16
	.long	.Linfo_string93
	.byte	64
	.byte	3
	.byte	16
	.byte	14
	.long	2805
	.long	.Linfo_string84
	.byte	14
	.long	3203
	.long	.Linfo_string88
	.byte	17
	.long	.Linfo_string89
	.long	2805
	.byte	16
	.byte	0
	.byte	3
	.byte	17
	.long	.Linfo_string90
	.long	3203
	.byte	1
	.byte	64
	.byte	3
	.byte	17
	.long	.Linfo_string91
	.long	3186
	.byte	8
	.byte	48
	.byte	3
	.byte	17
	.long	.Linfo_string92
	.long	3186
	.byte	8
	.byte	56
	.byte	3
	.byte	29
	.long	.Linfo_string94
	.long	.Linfo_string95
	.byte	1
	.byte	216

	.byte	14
	.long	2805
	.long	.Linfo_string84
	.byte	14
	.long	3203
	.long	.Linfo_string88
	.byte	19
	.long	3409
	.byte	19
	.long	3422
	.byte	0
	.byte	0
	.byte	30
	.long	.Linfo_string107
	.short	336
	.byte	1
	.byte	16
	.byte	14
	.long	2805
	.long	.Linfo_string84
	.byte	14
	.long	3203
	.long	.Linfo_string88
	.byte	17
	.long	.Linfo_string36
	.long	3238
	.byte	16
	.byte	0
	.byte	3
	.byte	0
	.byte	7
	.long	.Linfo_string142
	.byte	31
	.long	.Linfo_string143
	.long	.Linfo_string144
	.byte	1
	.byte	113
	.long	3119
	.byte	1
	.byte	14
	.long	2805
	.long	.Linfo_string84
	.byte	14
	.long	3203
	.long	.Linfo_string88
	.byte	32
	.long	.Linfo_string45
	.byte	1
	.byte	113
	.long	3934
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string135
	.byte	7
	.long	.Linfo_string136
	.byte	31
	.long	.Linfo_string137
	.long	.Linfo_string138
	.byte	14
	.byte	170
	.long	3119
	.byte	1
	.byte	32
	.long	.Linfo_string45
	.byte	14
	.byte	170
	.long	3921
	.byte	33
	.byte	34
	.long	.Linfo_string58
	.byte	8
	.byte	14
	.byte	173
	.long	3934
	.byte	0
	.byte	0
	.byte	0
	.byte	16
	.long	.Linfo_string139
	.byte	8
	.byte	1
	.byte	8
	.byte	17
	.long	.Linfo_string58
	.long	3630
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string58
	.byte	7
	.long	.Linfo_string59
	.byte	9
	.quad	.Lfunc_begin12
	.long	.Lfunc_end12-.Lfunc_begin12
	.byte	1
	.byte	87
	.long	.Linfo_string190
	.long	.Linfo_string191
	.byte	10
	.byte	95
	.long	4090
	.byte	15
	.byte	2
	.byte	145
	.byte	0
	.long	.Linfo_string45
	.byte	10
	.byte	95
	.long	3921
	.byte	14
	.long	1504
	.long	.Linfo_string174
	.byte	14
	.long	4090
	.long	.Linfo_string48
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string145
	.byte	7
	.long	.Linfo_string146
	.byte	7
	.long	.Linfo_string147
	.byte	9
	.quad	.Lfunc_begin14
	.long	.Lfunc_end14-.Lfunc_begin14
	.byte	1
	.byte	87
	.long	.Linfo_string192
	.long	.Linfo_string193
	.byte	11
	.byte	198
	.long	4090
	.byte	15
	.byte	2
	.byte	145
	.byte	16
	.long	.Linfo_string45
	.byte	11
	.byte	198
	.long	4247
	.byte	15
	.byte	2
	.byte	145
	.byte	24
	.long	.Linfo_string58
	.byte	11
	.byte	198
	.long	3921
	.byte	11
	.long	1461
	.quad	.Ltmp43
	.long	.Ltmp47-.Ltmp43
	.byte	11
	.byte	203
	.byte	10
	.byte	12
	.byte	2
	.byte	145
	.byte	32
	.long	1477
	.byte	11
	.long	3738
	.quad	.Ltmp43
	.long	.Ltmp45-.Ltmp43
	.byte	14
	.byte	173
	.byte	34
	.byte	12
	.byte	2
	.byte	145
	.byte	56
	.long	3773
	.byte	35
	.long	3871
	.quad	.Ltmp43
	.long	.Ltmp44-.Ltmp43
	.byte	13
	.short	2244
	.byte	15
	.byte	12
	.byte	2
	.byte	145
	.byte	56
	.long	3895
	.byte	13
	.long	3842
	.quad	.Ltmp43
	.long	.Ltmp44-.Ltmp43
	.byte	13
	.short	358
	.byte	27
	.byte	0
	.byte	0
	.byte	21
	.quad	.Ltmp46
	.long	.Ltmp47-.Ltmp46
	.byte	22
	.byte	2
	.byte	145
	.byte	40
	.long	1489
	.byte	11
	.long	1403
	.quad	.Ltmp46
	.long	.Ltmp47-.Ltmp46
	.byte	14
	.byte	174
	.byte	9
	.byte	12
	.byte	3
	.byte	145
	.asciz	"\300"
	.long	1437
	.byte	0
	.byte	0
	.byte	0
	.byte	14
	.long	1504
	.long	.Linfo_string84
	.byte	0
	.byte	0
	.byte	0
	.byte	26
	.long	.Linfo_string237
	.byte	0
	.byte	1
	.byte	1
	.byte	0
	.byte	0
	.byte	6
	.long	.Linfo_string37
	.byte	7
	.byte	1
	.byte	6
	.long	.Linfo_string41
	.byte	5
	.byte	4
	.byte	5
	.long	427
	.long	.Linfo_string42
	.long	0
	.byte	36
	.long	447
	.byte	1
	.byte	37
	.long	622
	.byte	1
	.byte	38
	.long	.Linfo_string45
	.byte	4
	.short	2062
	.long	602
	.byte	0
	.byte	7
	.long	.Linfo_string46
	.byte	7
	.long	.Linfo_string47
	.byte	39
	.long	.Linfo_string49
	.long	.Linfo_string50
	.byte	6
	.short	476
	.byte	1
	.byte	14
	.long	152
	.long	.Linfo_string48
	.byte	40
	.long	.Linfo_string51
	.byte	1
	.byte	6
	.short	476
	.long	152
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string53
	.byte	23
	.quad	.Lfunc_begin4
	.long	.Lfunc_end4-.Lfunc_begin4
	.byte	1
	.byte	87
	.long	.Linfo_string172
	.long	.Linfo_string173
	.byte	7
	.short	332
	.long	159
	.byte	41
	.byte	2
	.byte	145
	.byte	104
	.long	.Linfo_string222
	.byte	7
	.short	332
	.long	4097
	.byte	14
	.long	3119
	.long	.Linfo_string48
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string54
	.byte	7
	.long	.Linfo_string55
	.byte	7
	.long	.Linfo_string56
	.byte	9
	.quad	.Lfunc_begin5
	.long	.Lfunc_end5-.Lfunc_begin5
	.byte	1
	.byte	87
	.long	.Linfo_string176
	.long	.Linfo_string177
	.byte	8
	.byte	250
	.long	1863
	.byte	42
	.byte	2
	.byte	145
	.byte	16
	.byte	8
	.byte	250
	.long	4162
	.byte	42
	.byte	2
	.byte	145
	.byte	15
	.byte	8
	.byte	250
	.long	152
	.byte	14
	.long	181
	.long	.Linfo_string174
	.byte	14
	.long	152
	.long	.Linfo_string175
	.byte	0
	.byte	9
	.quad	.Lfunc_begin6
	.long	.Lfunc_end6-.Lfunc_begin6
	.byte	1
	.byte	87
	.long	.Linfo_string178
	.long	.Linfo_string177
	.byte	8
	.byte	250
	.long	1863
	.byte	42
	.byte	2
	.byte	145
	.byte	8
	.byte	8
	.byte	250
	.long	181
	.byte	42
	.byte	2
	.byte	145
	.byte	23
	.byte	8
	.byte	250
	.long	152
	.byte	14
	.long	181
	.long	.Linfo_string174
	.byte	14
	.long	152
	.long	.Linfo_string175
	.byte	0
	.byte	20
	.quad	.Lfunc_begin7
	.long	.Lfunc_end7-.Lfunc_begin7
	.byte	1
	.byte	87
	.long	.Linfo_string179
	.long	.Linfo_string180
	.byte	8
	.byte	250
	.byte	42
	.byte	2
	.byte	145
	.byte	16
	.byte	8
	.byte	250
	.long	695
	.byte	42
	.byte	2
	.byte	145
	.byte	15
	.byte	8
	.byte	250
	.long	152
	.byte	14
	.long	695
	.long	.Linfo_string174
	.byte	14
	.long	152
	.long	.Linfo_string175
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string57
	.byte	43
	.quad	.Lfunc_begin8
	.long	.Lfunc_end8-.Lfunc_begin8
	.byte	1
	.byte	87
	.long	.Linfo_string181
	.long	.Linfo_string182
	.byte	9
	.short	523
	.byte	24
	.byte	2
	.byte	145
	.byte	120
	.byte	9
	.short	523
	.long	4175
	.byte	14
	.long	797
	.long	.Linfo_string48
	.byte	0
	.byte	43
	.quad	.Lfunc_begin9
	.long	.Lfunc_end9-.Lfunc_begin9
	.byte	1
	.byte	87
	.long	.Linfo_string183
	.long	.Linfo_string184
	.byte	9
	.short	523
	.byte	24
	.byte	2
	.byte	145
	.byte	120
	.byte	9
	.short	523
	.long	4188
	.byte	14
	.long	789
	.long	.Linfo_string48
	.byte	0
	.byte	43
	.quad	.Lfunc_begin10
	.long	.Lfunc_end10-.Lfunc_begin10
	.byte	1
	.byte	87
	.long	.Linfo_string185
	.long	.Linfo_string186
	.byte	9
	.short	523
	.byte	24
	.byte	2
	.byte	145
	.byte	0
	.byte	9
	.short	523
	.long	4201
	.byte	14
	.long	1504
	.long	.Linfo_string48
	.byte	0
	.byte	43
	.quad	.Lfunc_begin11
	.long	.Lfunc_end11-.Lfunc_begin11
	.byte	1
	.byte	87
	.long	.Linfo_string187
	.long	.Linfo_string188
	.byte	9
	.short	523
	.byte	24
	.byte	2
	.byte	145
	.byte	120
	.byte	9
	.short	523
	.long	4162
	.byte	14
	.long	181
	.long	.Linfo_string48
	.byte	0
	.byte	7
	.long	.Linfo_string99
	.byte	16
	.long	.Linfo_string117
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.long	3575
	.long	.Linfo_string48
	.byte	17
	.long	.Linfo_string115
	.long	3803
	.byte	8
	.byte	0
	.byte	3
	.byte	18
	.long	.Linfo_string118
	.long	.Linfo_string119
	.byte	12
	.short	424
	.long	3816

	.byte	14
	.long	3575
	.long	.Linfo_string48
	.byte	19
	.long	3829
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string75
	.byte	7
	.long	.Linfo_string76
	.byte	16
	.long	.Linfo_string78
	.byte	16
	.byte	1
	.byte	16
	.byte	17
	.long	.Linfo_string36
	.long	3173
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string102
	.byte	30
	.long	.Linfo_string109
	.short	336
	.byte	1
	.byte	16
	.byte	14
	.long	1358
	.long	.Linfo_string48
	.byte	17
	.long	.Linfo_string108
	.long	1358
	.byte	16
	.byte	0
	.byte	3
	.byte	0
	.byte	16
	.long	.Linfo_string112
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.long	159
	.long	.Linfo_string48
	.byte	17
	.long	.Linfo_string108
	.long	2594
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	16
	.long	.Linfo_string111
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.long	159
	.long	.Linfo_string48
	.byte	17
	.long	.Linfo_string108
	.long	159
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string125
	.byte	16
	.long	.Linfo_string126
	.byte	0
	.byte	1
	.byte	1
	.byte	14
	.long	3575
	.long	.Linfo_string48
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string149
	.byte	7
	.long	.Linfo_string142
	.byte	23
	.quad	.Lfunc_begin16
	.long	.Lfunc_end16-.Lfunc_begin16
	.byte	1
	.byte	87
	.long	.Linfo_string197
	.long	.Linfo_string198
	.byte	15
	.short	715
	.long	4097
	.byte	41
	.byte	2
	.byte	145
	.byte	0
	.long	.Linfo_string45
	.byte	15
	.short	715
	.long	4260
	.byte	14
	.long	2826
	.long	.Linfo_string48
	.byte	14
	.long	3119
	.long	.Linfo_string196
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string230
	.byte	7
	.long	.Linfo_string231
	.byte	16
	.long	.Linfo_string234
	.byte	4
	.byte	1
	.byte	4
	.byte	14
	.long	1863
	.long	.Linfo_string48
	.byte	17
	.long	.Linfo_string36
	.long	2771
	.byte	4
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string232
	.byte	16
	.long	.Linfo_string233
	.byte	4
	.byte	1
	.byte	4
	.byte	17
	.long	.Linfo_string36
	.long	1863
	.byte	4
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string60
	.byte	7
	.long	.Linfo_string61
	.byte	16
	.long	.Linfo_string83
	.byte	48
	.byte	1
	.byte	16
	.byte	17
	.long	.Linfo_string62
	.long	2968
	.byte	16
	.byte	0
	.byte	3
	.byte	0
	.byte	30
	.long	.Linfo_string97
	.short	256
	.byte	1
	.byte	4
	.byte	14
	.long	3119
	.long	.Linfo_string48
	.byte	17
	.long	.Linfo_string36
	.long	3435
	.byte	4
	.byte	0
	.byte	3
	.byte	0
	.byte	7
	.long	.Linfo_string157
	.byte	20
	.quad	.Lfunc_begin18
	.long	.Lfunc_end18-.Lfunc_begin18
	.byte	1
	.byte	87
	.long	.Linfo_string204
	.long	.Linfo_string205
	.byte	17
	.byte	90
	.byte	15
	.byte	2
	.byte	145
	.byte	0
	.long	.Linfo_string45
	.byte	17
	.byte	90
	.long	4273
	.byte	15
	.byte	2
	.byte	145
	.byte	8
	.long	.Linfo_string241
	.byte	17
	.byte	90
	.long	3422
	.byte	11
	.long	3973
	.quad	.Ltmp59
	.long	.Ltmp60-.Ltmp59
	.byte	17
	.byte	91
	.byte	17
	.byte	12
	.byte	2
	.byte	145
	.byte	16
	.long	3979
	.byte	12
	.byte	2
	.byte	145
	.byte	28
	.long	3990
	.byte	12
	.byte	2
	.byte	145
	.byte	32
	.long	4001
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string63
	.byte	16
	.long	.Linfo_string82
	.byte	48
	.byte	1
	.byte	16
	.byte	17
	.long	.Linfo_string64
	.long	3052
	.byte	16
	.byte	0
	.byte	2
	.byte	17
	.long	.Linfo_string80
	.long	3052
	.byte	16
	.byte	16
	.byte	2
	.byte	17
	.long	.Linfo_string81
	.long	3052
	.byte	16
	.byte	32
	.byte	2
	.byte	29
	.long	.Linfo_string151
	.long	.Linfo_string152
	.byte	18
	.byte	80

	.byte	19
	.long	3947
	.byte	19
	.long	3119
	.byte	19
	.long	3960
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string65
	.byte	7
	.long	.Linfo_string66
	.byte	44
	.long	.Linfo_string79
	.byte	16
	.byte	16
	.byte	4
	.long	.Linfo_string67
	.long	3106
	.byte	4
	.byte	0
	.byte	4
	.long	.Linfo_string70
	.long	3133
	.byte	8
	.byte	0
	.byte	4
	.long	.Linfo_string72
	.long	3153
	.byte	16
	.byte	0
	.byte	4
	.long	.Linfo_string74
	.long	2505
	.byte	16
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	45
	.long	3119
	.byte	46
	.long	3126
	.byte	0
	.byte	4
	.byte	0
	.byte	6
	.long	.Linfo_string68
	.byte	7
	.byte	4
	.byte	47
	.long	.Linfo_string69
	.byte	8
	.byte	7
	.byte	45
	.long	3146
	.byte	46
	.long	3126
	.byte	0
	.byte	2
	.byte	0
	.byte	6
	.long	.Linfo_string71
	.byte	7
	.byte	8
	.byte	45
	.long	3166
	.byte	46
	.long	3126
	.byte	0
	.byte	1
	.byte	0
	.byte	6
	.long	.Linfo_string73
	.byte	7
	.byte	16
	.byte	45
	.long	3186
	.byte	46
	.long	3126
	.byte	0
	.byte	2
	.byte	0
	.byte	6
	.long	.Linfo_string77
	.byte	5
	.byte	8
	.byte	7
	.long	.Linfo_string85
	.byte	7
	.long	.Linfo_string86
	.byte	26
	.long	.Linfo_string87
	.byte	0
	.byte	1
	.byte	1
	.byte	16
	.long	.Linfo_string236
	.byte	4
	.byte	1
	.byte	4
	.byte	17
	.long	.Linfo_string36
	.long	4224
	.byte	4
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string103
	.byte	30
	.long	.Linfo_string106
	.short	336
	.byte	1
	.byte	16
	.byte	14
	.long	1243
	.long	.Linfo_string84
	.byte	17
	.long	.Linfo_string104
	.long	2826
	.byte	4
	.byte	0
	.byte	3
	.byte	48
	.long	.Linfo_string105
	.long	159
	.byte	8
	.short	320
	.byte	3
	.byte	48
	.long	.Linfo_string46
	.long	1243
	.byte	16
	.short	256
	.byte	1
	.byte	29
	.long	.Linfo_string158
	.long	.Linfo_string159
	.byte	16
	.byte	177

	.byte	14
	.long	1243
	.long	.Linfo_string84
	.byte	19
	.long	4013
	.byte	19
	.long	159
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string150
	.byte	9
	.quad	.Lfunc_begin17
	.long	.Lfunc_end17-.Lfunc_begin17
	.byte	1
	.byte	87
	.long	.Linfo_string202
	.long	.Linfo_string203
	.byte	16
	.byte	186
	.long	3119
	.byte	15
	.byte	2
	.byte	145
	.byte	56
	.long	.Linfo_string45
	.byte	16
	.byte	186
	.long	4013
	.byte	27
	.long	.Ldebug_ranges2
	.byte	10
	.byte	3
	.byte	145
	.asciz	"\304"
	.long	.Linfo_string108
	.byte	4
	.byte	16
	.byte	191
	.long	3119
	.byte	0
	.byte	14
	.long	1243
	.long	.Linfo_string84
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	1243
	.long	.Linfo_string96
	.long	0
	.byte	5
	.long	2826
	.long	.Linfo_string98
	.long	0
	.byte	45
	.long	3119
	.byte	46
	.long	3126
	.byte	0
	.byte	64
	.byte	0
	.byte	49
	.quad	.Lfunc_begin13
	.long	.Lfunc_end13-.Lfunc_begin13
	.byte	1
	.byte	87
	.long	1317
	.byte	15
	.byte	2
	.byte	145
	.byte	48
	.long	.Linfo_string45
	.byte	1
	.byte	216
	.long	3409
	.byte	15
	.byte	2
	.byte	145
	.byte	32
	.long	.Linfo_string104
	.byte	1
	.byte	216
	.long	3422
	.byte	27
	.long	.Ldebug_ranges1
	.byte	10
	.byte	2
	.byte	145
	.byte	56
	.long	.Linfo_string214
	.byte	8
	.byte	1
	.byte	219
	.long	159
	.byte	21
	.quad	.Ltmp38
	.long	.Ltmp39-.Ltmp38
	.byte	10
	.byte	3
	.byte	145
	.asciz	"\304"
	.long	.Linfo_string227
	.byte	4
	.byte	1
	.byte	221
	.long	3211
	.byte	0
	.byte	0
	.byte	14
	.long	2805
	.long	.Linfo_string84
	.byte	14
	.long	3203
	.long	.Linfo_string88
	.byte	0
	.byte	7
	.long	.Linfo_string100
	.byte	7
	.long	.Linfo_string101
	.byte	30
	.long	.Linfo_string114
	.short	352
	.byte	3
	.byte	16
	.byte	14
	.long	2533
	.long	.Linfo_string48
	.byte	17
	.long	.Linfo_string110
	.long	2564
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.long	.Linfo_string113
	.long	2564
	.byte	8
	.byte	8
	.byte	3
	.byte	17
	.long	.Linfo_string108
	.long	2533
	.byte	16
	.byte	16
	.byte	3
	.byte	0
	.byte	16
	.long	.Linfo_string127
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.long	2533
	.long	.Linfo_string48
	.byte	14
	.long	3793
	.long	.Linfo_string123
	.byte	17
	.long	.Linfo_string57
	.long	2432
	.byte	8
	.byte	0
	.byte	3
	.byte	17
	.long	.Linfo_string124
	.long	2630
	.byte	1
	.byte	8
	.byte	3
	.byte	17
	.long	.Linfo_string100
	.long	3793
	.byte	1
	.byte	8
	.byte	3
	.byte	18
	.long	.Linfo_string128
	.long	.Linfo_string129
	.byte	13
	.short	355
	.long	3816

	.byte	14
	.long	2533
	.long	.Linfo_string48
	.byte	14
	.long	3793
	.long	.Linfo_string123
	.byte	19
	.long	3858
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string131
	.byte	50
	.long	.Linfo_string132
	.long	.Linfo_string133
	.byte	13
	.short	2243
	.long	3908
	.byte	1
	.byte	14
	.long	2533
	.long	.Linfo_string48
	.byte	14
	.long	3793
	.long	.Linfo_string123
	.byte	38
	.long	.Linfo_string45
	.byte	13
	.short	2243
	.long	3858
	.byte	0
	.byte	0
	.byte	0
	.byte	7
	.long	.Linfo_string100
	.byte	26
	.long	.Linfo_string122
	.byte	0
	.byte	1
	.byte	1
	.byte	0
	.byte	0
	.byte	5
	.long	3575
	.long	.Linfo_string116
	.long	0
	.byte	5
	.long	3575
	.long	.Linfo_string120
	.long	0
	.byte	5
	.long	2432
	.long	.Linfo_string121
	.long	0
	.byte	37
	.long	2461
	.byte	1
	.byte	14
	.long	3575
	.long	.Linfo_string48
	.byte	0
	.byte	5
	.long	3630
	.long	.Linfo_string130
	.long	0
	.byte	37
	.long	3692
	.byte	1
	.byte	14
	.long	2533
	.long	.Linfo_string48
	.byte	14
	.long	3793
	.long	.Linfo_string123
	.byte	38
	.long	.Linfo_string45
	.byte	13
	.short	355
	.long	3858
	.byte	0
	.byte	5
	.long	2533
	.long	.Linfo_string134
	.long	0
	.byte	5
	.long	1504
	.long	.Linfo_string140
	.long	0
	.byte	5
	.long	1358
	.long	.Linfo_string141
	.long	0
	.byte	5
	.long	2968
	.long	.Linfo_string153
	.long	0
	.byte	5
	.long	3435
	.long	.Linfo_string154
	.long	0
	.byte	37
	.long	3012
	.byte	1
	.byte	32
	.long	.Linfo_string45
	.byte	18
	.byte	80
	.long	3947
	.byte	32
	.long	.Linfo_string155
	.byte	18
	.byte	80
	.long	3119
	.byte	32
	.long	.Linfo_string156
	.byte	18
	.byte	80
	.long	3960
	.byte	0
	.byte	5
	.long	3238
	.long	.Linfo_string160
	.long	0
	.byte	49
	.quad	.Lfunc_begin19
	.long	.Lfunc_end19-.Lfunc_begin19
	.byte	1
	.byte	87
	.long	3294
	.byte	15
	.byte	2
	.byte	145
	.byte	24
	.long	.Linfo_string45
	.byte	16
	.byte	177
	.long	4013
	.byte	15
	.byte	2
	.byte	145
	.byte	32
	.long	.Linfo_string105
	.byte	16
	.byte	177
	.long	159
	.byte	14
	.long	1243
	.long	.Linfo_string84
	.byte	0
	.byte	6
	.long	.Linfo_string166
	.byte	5
	.byte	8
	.byte	6
	.long	.Linfo_string189
	.byte	2
	.byte	1
	.byte	8
	.long	.Linfo_string201
	.byte	16
	.byte	8
	.byte	4
	.long	.Linfo_string199
	.long	4127
	.byte	8
	.byte	0
	.byte	4
	.long	.Linfo_string200
	.long	159
	.byte	8
	.byte	8
	.byte	0
	.byte	51
	.long	3119
	.long	0
	.byte	5
	.long	4149
	.long	.Linfo_string218
	.long	0
	.byte	5
	.long	1856
	.long	.Linfo_string217
	.long	0
	.byte	5
	.long	181
	.long	.Linfo_string223
	.long	0
	.byte	5
	.long	797
	.long	.Linfo_string224
	.long	0
	.byte	5
	.long	789
	.long	.Linfo_string225
	.long	0
	.byte	5
	.long	1504
	.long	.Linfo_string226
	.long	0
	.byte	7
	.long	.Linfo_string228
	.byte	7
	.long	.Linfo_string229
	.byte	16
	.long	.Linfo_string235
	.byte	4
	.byte	1
	.byte	4
	.byte	17
	.long	.Linfo_string36
	.long	2735
	.byte	4
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	1846
	.long	.Linfo_string238
	.long	0
	.byte	5
	.long	3422
	.long	.Linfo_string239
	.long	0
	.byte	5
	.long	2805
	.long	.Linfo_string240
	.long	0
	.byte	5
	.long	789
	.long	.Linfo_string242
	.long	0
	.byte	5
	.long	797
	.long	.Linfo_string243
	.long	0
	.byte	8
	.long	.Linfo_string248
	.byte	16
	.byte	8
	.byte	4
	.long	.Linfo_string115
	.long	4342
	.byte	8
	.byte	0
	.byte	4
	.long	.Linfo_string246
	.long	4358
	.byte	8
	.byte	8
	.byte	0
	.byte	51
	.long	4351
	.long	0
	.byte	52
	.long	.Linfo_string245
	.byte	0
	.byte	1
	.byte	5
	.long	4371
	.long	.Linfo_string247
	.long	0
	.byte	45
	.long	159
	.byte	46
	.long	3126
	.byte	0
	.byte	4
	.byte	0
	.byte	0
.Ldebug_info_end0:
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.2,"aw",@progbits
.Lsec_end0:
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.15,"aw",@progbits
.Lsec_end1:
	.section	.data.rel.ro..Lanon.a6fa456c9412015864d7783aa51b16ac.16,"aw",@progbits
.Lsec_end2:
	.section	".text._ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h65f4881e2030d610E","ax",@progbits
.Lsec_end3:
	.section	.text._ZN3std2rt10lang_start17h27e2b9e2c6bd9d09E,"ax",@progbits
.Lsec_end4:
	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h975e66f67ae81916E","ax",@progbits
.Lsec_end5:
	.section	.text._ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2afd2c7d0674a96fE,"ax",@progbits
.Lsec_end6:
	.section	.text._ZN4core3mem11size_of_val17h6d9e552afa58affdE,"ax",@progbits
.Lsec_end7:
	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hb06e99ac8656f6d6E","ax",@progbits
.Lsec_end8:
	.section	.text._ZN4core3ops8function6FnOnce9call_once17h1c445d99f54a92d5E,"ax",@progbits
.Lsec_end9:
	.section	.text._ZN4core3ops8function6FnOnce9call_once17hd703a48edda16cd5E,"ax",@progbits
.Lsec_end10:
	.section	".text._ZN4core3ptr31drop_in_place$LT$dp_ex..Cat$GT$17h9aa1b5c63807ed14E","ax",@progbits
.Lsec_end11:
	.section	".text._ZN4core3ptr31drop_in_place$LT$dp_ex..Dog$GT$17haa225f289cc6a195E","ax",@progbits
.Lsec_end12:
	.section	".text._ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17h4290c5a380cfe87dE","ax",@progbits
.Lsec_end13:
	.section	".text._ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0c91bb19f0a043fdE","ax",@progbits
.Lsec_end14:
	.section	.text._ZN4rand3rng3Rng6random17he47c0a49024989aaE,"ax",@progbits
.Lsec_end15:
	.section	".text._ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17h13935bc7e637f19cE","ax",@progbits
.Lsec_end16:
	.section	".text._ZN4rand5distr5other110_$LT$impl$u20$rand..distr..distribution..Distribution$LT$bool$GT$$u20$for$u20$rand..distr..StandardUniform$GT$6sample17he90ea3740add583dE","ax",@progbits
.Lsec_end17:
	.section	".text._ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h61953a0fd5719ceaE","ax",@progbits
.Lsec_end18:
	.section	".text._ZN63_$LT$$RF$mut$u20$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17hcf4f85feb7206a28E","ax",@progbits
.Lsec_end19:
	.section	".text._ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h5ba689e9c842bbddE","ax",@progbits
.Lsec_end20:
	.section	".text._ZN84_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h30baf41324780ba0E","ax",@progbits
.Lsec_end21:
	.section	".text._ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h815b35b30d5e012dE","ax",@progbits
.Lsec_end22:
	.section	".text._ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17had468e7662b92aa1E","ax",@progbits
.Lsec_end23:
	.section	".text._ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17h504ab8d43430beb1E","ax",@progbits
.Lsec_end24:
	.section	.text._ZN5dp_ex6dyn_dp17h2163eacfb0f4ef0dE,"ax",@progbits
.Lsec_end25:
	.section	.text._ZN5dp_ex9static_dp17h12c823b24041d8eeE,"ax",@progbits
.Lsec_end26:
	.section	.text._ZN5dp_ex4main17hfaa1a74bb1a25d42E,"ax",@progbits
.Lsec_end27:
	.section	.debug_aranges,"",@progbits
	.long	476
	.short	2
	.long	.Lcu_begin0
	.byte	8
	.byte	0
	.zero	4,255
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.2
	.quad	.Lsec_end0-.Lanon.a6fa456c9412015864d7783aa51b16ac.2
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.15
	.quad	.Lsec_end1-.Lanon.a6fa456c9412015864d7783aa51b16ac.15
	.quad	.Lanon.a6fa456c9412015864d7783aa51b16ac.16
	.quad	.Lsec_end2-.Lanon.a6fa456c9412015864d7783aa51b16ac.16
	.quad	.Lfunc_begin0
	.quad	.Lsec_end3-.Lfunc_begin0
	.quad	.Lfunc_begin1
	.quad	.Lsec_end4-.Lfunc_begin1
	.quad	.Lfunc_begin2
	.quad	.Lsec_end5-.Lfunc_begin2
	.quad	.Lfunc_begin3
	.quad	.Lsec_end6-.Lfunc_begin3
	.quad	.Lfunc_begin4
	.quad	.Lsec_end7-.Lfunc_begin4
	.quad	.Lfunc_begin5
	.quad	.Lsec_end8-.Lfunc_begin5
	.quad	.Lfunc_begin6
	.quad	.Lsec_end9-.Lfunc_begin6
	.quad	.Lfunc_begin7
	.quad	.Lsec_end10-.Lfunc_begin7
	.quad	.Lfunc_begin8
	.quad	.Lsec_end11-.Lfunc_begin8
	.quad	.Lfunc_begin9
	.quad	.Lsec_end12-.Lfunc_begin9
	.quad	.Lfunc_begin10
	.quad	.Lsec_end13-.Lfunc_begin10
	.quad	.Lfunc_begin11
	.quad	.Lsec_end14-.Lfunc_begin11
	.quad	.Lfunc_begin12
	.quad	.Lsec_end15-.Lfunc_begin12
	.quad	.Lfunc_begin13
	.quad	.Lsec_end16-.Lfunc_begin13
	.quad	.Lfunc_begin14
	.quad	.Lsec_end17-.Lfunc_begin14
	.quad	.Lfunc_begin15
	.quad	.Lsec_end18-.Lfunc_begin15
	.quad	.Lfunc_begin16
	.quad	.Lsec_end19-.Lfunc_begin16
	.quad	.Lfunc_begin17
	.quad	.Lsec_end20-.Lfunc_begin17
	.quad	.Lfunc_begin18
	.quad	.Lsec_end21-.Lfunc_begin18
	.quad	.Lfunc_begin19
	.quad	.Lsec_end22-.Lfunc_begin19
	.quad	.Lfunc_begin20
	.quad	.Lsec_end23-.Lfunc_begin20
	.quad	.Lfunc_begin21
	.quad	.Lsec_end24-.Lfunc_begin21
	.quad	.Lfunc_begin22
	.quad	.Lsec_end25-.Lfunc_begin22
	.quad	.Lfunc_begin23
	.quad	.Lsec_end26-.Lfunc_begin23
	.quad	.Lfunc_begin24
	.quad	.Lsec_end27-.Lfunc_begin24
	.quad	0
	.quad	0
	.section	.debug_ranges,"",@progbits
.Ldebug_ranges0:
	.quad	.Ltmp1
	.quad	.Ltmp2
	.quad	.Ltmp3
	.quad	.Ltmp4
	.quad	0
	.quad	0
.Ldebug_ranges1:
	.quad	.Ltmp38
	.quad	.Ltmp40
	.quad	.Ltmp41
	.quad	.Ltmp42
	.quad	0
	.quad	0
.Ldebug_ranges2:
	.quad	.Ltmp53
	.quad	.Ltmp54
	.quad	.Ltmp55
	.quad	.Ltmp56
	.quad	.Ltmp57
	.quad	.Ltmp58
	.quad	0
	.quad	0
.Ldebug_ranges3:
	.quad	.Ltmp77
	.quad	.Ltmp78
	.quad	.Ltmp79
	.quad	.Ltmp80
	.quad	0
	.quad	0
.Ldebug_ranges4:
	.quad	.Lfunc_begin0
	.quad	.Lfunc_end0
	.quad	.Lfunc_begin1
	.quad	.Lfunc_end1
	.quad	.Lfunc_begin2
	.quad	.Lfunc_end2
	.quad	.Lfunc_begin3
	.quad	.Lfunc_end3
	.quad	.Lfunc_begin4
	.quad	.Lfunc_end4
	.quad	.Lfunc_begin5
	.quad	.Lfunc_end5
	.quad	.Lfunc_begin6
	.quad	.Lfunc_end6
	.quad	.Lfunc_begin7
	.quad	.Lfunc_end7
	.quad	.Lfunc_begin8
	.quad	.Lfunc_end8
	.quad	.Lfunc_begin9
	.quad	.Lfunc_end9
	.quad	.Lfunc_begin10
	.quad	.Lfunc_end10
	.quad	.Lfunc_begin11
	.quad	.Lfunc_end11
	.quad	.Lfunc_begin12
	.quad	.Lfunc_end12
	.quad	.Lfunc_begin13
	.quad	.Lfunc_end13
	.quad	.Lfunc_begin14
	.quad	.Lfunc_end14
	.quad	.Lfunc_begin15
	.quad	.Lfunc_end15
	.quad	.Lfunc_begin16
	.quad	.Lfunc_end16
	.quad	.Lfunc_begin17
	.quad	.Lfunc_end17
	.quad	.Lfunc_begin18
	.quad	.Lfunc_end18
	.quad	.Lfunc_begin19
	.quad	.Lfunc_end19
	.quad	.Lfunc_begin20
	.quad	.Lfunc_end20
	.quad	.Lfunc_begin21
	.quad	.Lfunc_end21
	.quad	.Lfunc_begin22
	.quad	.Lfunc_end22
	.quad	.Lfunc_begin23
	.quad	.Lfunc_end23
	.quad	.Lfunc_begin24
	.quad	.Lfunc_end24
	.quad	0
	.quad	0
	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"clang LLVM (rustc version 1.87.0-nightly (ecade534c 2025-03-14))"
.Linfo_string1:
	.asciz	"src/main.rs/@/47mem9q2d6p1mvtu0iveyp842"
.Linfo_string2:
	.asciz	"/home/np/hack/verifopt/dp-ex"
.Linfo_string3:
	.asciz	"<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable}"
.Linfo_string4:
	.asciz	"drop_in_place"
.Linfo_string5:
	.asciz	"()"
.Linfo_string6:
	.asciz	"*const ()"
.Linfo_string7:
	.asciz	"size"
.Linfo_string8:
	.asciz	"usize"
.Linfo_string9:
	.asciz	"align"
.Linfo_string10:
	.asciz	"__method3"
.Linfo_string11:
	.asciz	"__method4"
.Linfo_string12:
	.asciz	"__method5"
.Linfo_string13:
	.asciz	"std"
.Linfo_string14:
	.asciz	"rt"
.Linfo_string15:
	.asciz	"lang_start"
.Linfo_string16:
	.asciz	"main"
.Linfo_string17:
	.asciz	"fn()"
.Linfo_string18:
	.asciz	"{closure_env#0}<()>"
.Linfo_string19:
	.asciz	"<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable_type}"
.Linfo_string20:
	.asciz	"<dp_ex::Dog as dp_ex::Animal>::{vtable}"
.Linfo_string21:
	.asciz	"dp_ex"
.Linfo_string22:
	.asciz	"Dog"
.Linfo_string23:
	.asciz	"<dp_ex::Dog as dp_ex::Animal>::{vtable_type}"
.Linfo_string24:
	.asciz	"<dp_ex::Cat as dp_ex::Animal>::{vtable}"
.Linfo_string25:
	.asciz	"Cat"
.Linfo_string26:
	.asciz	"<dp_ex::Cat as dp_ex::Animal>::{vtable_type}"
.Linfo_string27:
	.asciz	"rand"
.Linfo_string28:
	.asciz	"rngs"
.Linfo_string29:
	.asciz	"reseeding"
.Linfo_string30:
	.asciz	"{impl#4}"
.Linfo_string31:
	.asciz	"sys"
.Linfo_string32:
	.asciz	"pal"
.Linfo_string33:
	.asciz	"unix"
.Linfo_string34:
	.asciz	"process"
.Linfo_string35:
	.asciz	"process_common"
.Linfo_string36:
	.asciz	"__0"
.Linfo_string37:
	.asciz	"u8"
.Linfo_string38:
	.asciz	"ExitCode"
.Linfo_string39:
	.asciz	"_ZN3std3sys3pal4unix7process14process_common8ExitCode6as_i3217hd6cb0c23f29c3e8aE"
.Linfo_string40:
	.asciz	"as_i32"
.Linfo_string41:
	.asciz	"i32"
.Linfo_string42:
	.asciz	"&std::sys::pal::unix::process::process_common::ExitCode"
.Linfo_string43:
	.asciz	"_ZN3std7process8ExitCode6to_i3217h92b92dfd82d5f0cfE"
.Linfo_string44:
	.asciz	"to_i32"
.Linfo_string45:
	.asciz	"self"
.Linfo_string46:
	.asciz	"core"
.Linfo_string47:
	.asciz	"hint"
.Linfo_string48:
	.asciz	"T"
.Linfo_string49:
	.asciz	"_ZN4core4hint9black_box17h7da4fda42db8f900E"
.Linfo_string50:
	.asciz	"black_box<()>"
.Linfo_string51:
	.asciz	"dummy"
.Linfo_string52:
	.asciz	"backtrace"
.Linfo_string53:
	.asciz	"mem"
.Linfo_string54:
	.asciz	"ops"
.Linfo_string55:
	.asciz	"function"
.Linfo_string56:
	.asciz	"FnOnce"
.Linfo_string57:
	.asciz	"ptr"
.Linfo_string58:
	.asciz	"rng"
.Linfo_string59:
	.asciz	"Rng"
.Linfo_string60:
	.asciz	"rand_chacha"
.Linfo_string61:
	.asciz	"chacha"
.Linfo_string62:
	.asciz	"state"
.Linfo_string63:
	.asciz	"guts"
.Linfo_string64:
	.asciz	"b"
.Linfo_string65:
	.asciz	"ppv_lite86"
.Linfo_string66:
	.asciz	"x86_64"
.Linfo_string67:
	.asciz	"u32x4"
.Linfo_string68:
	.asciz	"u32"
.Linfo_string69:
	.asciz	"__ARRAY_SIZE_TYPE__"
.Linfo_string70:
	.asciz	"u64x2"
.Linfo_string71:
	.asciz	"u64"
.Linfo_string72:
	.asciz	"u128x1"
.Linfo_string73:
	.asciz	"u128"
.Linfo_string74:
	.asciz	"sse2"
.Linfo_string75:
	.asciz	"core_arch"
.Linfo_string76:
	.asciz	"x86"
.Linfo_string77:
	.asciz	"i64"
.Linfo_string78:
	.asciz	"__m128i"
.Linfo_string79:
	.asciz	"vec128_storage"
.Linfo_string80:
	.asciz	"c"
.Linfo_string81:
	.asciz	"d"
.Linfo_string82:
	.asciz	"ChaCha"
.Linfo_string83:
	.asciz	"ChaCha12Core"
.Linfo_string84:
	.asciz	"R"
.Linfo_string85:
	.asciz	"rand_core"
.Linfo_string86:
	.asciz	"os"
.Linfo_string87:
	.asciz	"OsRng"
.Linfo_string88:
	.asciz	"Rsdr"
.Linfo_string89:
	.asciz	"inner"
.Linfo_string90:
	.asciz	"reseeder"
.Linfo_string91:
	.asciz	"threshold"
.Linfo_string92:
	.asciz	"bytes_until_reseed"
.Linfo_string93:
	.asciz	"ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>"
.Linfo_string94:
	.asciz	"_ZN4rand4rngs9reseeding29ReseedingCore$LT$R$C$Rsdr$GT$19reseed_and_generate17h13935bc7e637f19cE"
.Linfo_string95:
	.asciz	"reseed_and_generate<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>"
.Linfo_string96:
	.asciz	"&mut rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>"
.Linfo_string97:
	.asciz	"Array64<u32>"
.Linfo_string98:
	.asciz	"&mut rand_chacha::chacha::Array64<u32>"
.Linfo_string99:
	.asciz	"non_null"
.Linfo_string100:
	.asciz	"alloc"
.Linfo_string101:
	.asciz	"rc"
.Linfo_string102:
	.asciz	"cell"
.Linfo_string103:
	.asciz	"block"
.Linfo_string104:
	.asciz	"results"
.Linfo_string105:
	.asciz	"index"
.Linfo_string106:
	.asciz	"BlockRng<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>"
.Linfo_string107:
	.asciz	"ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>"
.Linfo_string108:
	.asciz	"value"
.Linfo_string109:
	.asciz	"UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>"
.Linfo_string110:
	.asciz	"strong"
.Linfo_string111:
	.asciz	"UnsafeCell<usize>"
.Linfo_string112:
	.asciz	"Cell<usize>"
.Linfo_string113:
	.asciz	"weak"
.Linfo_string114:
	.asciz	"RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>"
.Linfo_string115:
	.asciz	"pointer"
.Linfo_string116:
	.asciz	"*const alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>"
.Linfo_string117:
	.asciz	"NonNull<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>"
.Linfo_string118:
	.asciz	"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ref17h2f0470cdcae54db3E"
.Linfo_string119:
	.asciz	"as_ref<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>"
.Linfo_string120:
	.asciz	"&alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>"
.Linfo_string121:
	.asciz	"&core::ptr::non_null::NonNull<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>"
.Linfo_string122:
	.asciz	"Global"
.Linfo_string123:
	.asciz	"A"
.Linfo_string124:
	.asciz	"phantom"
.Linfo_string125:
	.asciz	"marker"
.Linfo_string126:
	.asciz	"PhantomData<alloc::rc::RcInner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>>>"
.Linfo_string127:
	.asciz	"Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>"
.Linfo_string128:
	.asciz	"_ZN5alloc2rc15Rc$LT$T$C$A$GT$5inner17h57fc78e9945e8c8dE"
.Linfo_string129:
	.asciz	"inner<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>"
.Linfo_string130:
	.asciz	"&alloc::rc::Rc<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>"
.Linfo_string131:
	.asciz	"{impl#25}"
.Linfo_string132:
	.asciz	"_ZN70_$LT$alloc..rc..Rc$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17hb6ac6f99dbef582dE"
.Linfo_string133:
	.asciz	"deref<core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>, alloc::alloc::Global>"
.Linfo_string134:
	.asciz	"&core::cell::UnsafeCell<rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>"
.Linfo_string135:
	.asciz	"thread"
.Linfo_string136:
	.asciz	"{impl#3}"
.Linfo_string137:
	.asciz	"_ZN68_$LT$rand..rngs..thread..ThreadRng$u20$as$u20$rand_core..RngCore$GT$8next_u3217h096f0839c6fdbe67E"
.Linfo_string138:
	.asciz	"next_u32"
.Linfo_string139:
	.asciz	"ThreadRng"
.Linfo_string140:
	.asciz	"&mut rand::rngs::thread::ThreadRng"
.Linfo_string141:
	.asciz	"&mut rand::rngs::reseeding::ReseedingRng<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>"
.Linfo_string142:
	.asciz	"{impl#1}"
.Linfo_string143:
	.asciz	"_ZN90_$LT$rand..rngs..reseeding..ReseedingRng$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h9aaf76541b3e6963E"
.Linfo_string144:
	.asciz	"next_u32<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>"
.Linfo_string145:
	.asciz	"distr"
.Linfo_string146:
	.asciz	"other"
.Linfo_string147:
	.asciz	"{impl#6}"
.Linfo_string148:
	.asciz	"{impl#57}"
.Linfo_string149:
	.asciz	"convert"
.Linfo_string150:
	.asciz	"{impl#2}"
.Linfo_string151:
	.asciz	"_ZN11rand_chacha4guts6ChaCha7refill417h7a24c7be9ffb049cE"
.Linfo_string152:
	.asciz	"refill4"
.Linfo_string153:
	.asciz	"&mut rand_chacha::guts::ChaCha"
.Linfo_string154:
	.asciz	"&mut [u32; 64]"
.Linfo_string155:
	.asciz	"drounds"
.Linfo_string156:
	.asciz	"out"
.Linfo_string157:
	.asciz	"{impl#23}"
.Linfo_string158:
	.asciz	"_ZN9rand_core5block17BlockRng$LT$R$GT$16generate_and_set17h815b35b30d5e012dE"
.Linfo_string159:
	.asciz	"generate_and_set<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>"
.Linfo_string160:
	.asciz	"&mut rand_core::block::BlockRng<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>"
.Linfo_string161:
	.asciz	"{impl#0}"
.Linfo_string162:
	.asciz	"_ZN103_$LT$rand..rngs..reseeding..ReseedingCore$LT$R$C$Rsdr$GT$$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h65f4881e2030d610E"
.Linfo_string163:
	.asciz	"generate<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>"
.Linfo_string164:
	.asciz	"_ZN3std2rt10lang_start17h27e2b9e2c6bd9d09E"
.Linfo_string165:
	.asciz	"lang_start<()>"
.Linfo_string166:
	.asciz	"isize"
.Linfo_string167:
	.asciz	"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h975e66f67ae81916E"
.Linfo_string168:
	.asciz	"{closure#0}<()>"
.Linfo_string169:
	.asciz	"F"
.Linfo_string170:
	.asciz	"_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h2afd2c7d0674a96fE"
.Linfo_string171:
	.asciz	"__rust_begin_short_backtrace<fn(), ()>"
.Linfo_string172:
	.asciz	"_ZN4core3mem11size_of_val17h6d9e552afa58affdE"
.Linfo_string173:
	.asciz	"size_of_val<[u32]>"
.Linfo_string174:
	.asciz	"Self"
.Linfo_string175:
	.asciz	"Args"
.Linfo_string176:
	.asciz	"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hb06e99ac8656f6d6E"
.Linfo_string177:
	.asciz	"call_once<std::rt::lang_start::{closure_env#0}<()>, ()>"
.Linfo_string178:
	.asciz	"_ZN4core3ops8function6FnOnce9call_once17h1c445d99f54a92d5E"
.Linfo_string179:
	.asciz	"_ZN4core3ops8function6FnOnce9call_once17hd703a48edda16cd5E"
.Linfo_string180:
	.asciz	"call_once<fn(), ()>"
.Linfo_string181:
	.asciz	"_ZN4core3ptr31drop_in_place$LT$dp_ex..Cat$GT$17h9aa1b5c63807ed14E"
.Linfo_string182:
	.asciz	"drop_in_place<dp_ex::Cat>"
.Linfo_string183:
	.asciz	"_ZN4core3ptr31drop_in_place$LT$dp_ex..Dog$GT$17haa225f289cc6a195E"
.Linfo_string184:
	.asciz	"drop_in_place<dp_ex::Dog>"
.Linfo_string185:
	.asciz	"_ZN4core3ptr50drop_in_place$LT$rand..rngs..thread..ThreadRng$GT$17h4290c5a380cfe87dE"
.Linfo_string186:
	.asciz	"drop_in_place<rand::rngs::thread::ThreadRng>"
.Linfo_string187:
	.asciz	"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h0c91bb19f0a043fdE"
.Linfo_string188:
	.asciz	"drop_in_place<std::rt::lang_start::{closure_env#0}<()>>"
.Linfo_string189:
	.asciz	"bool"
.Linfo_string190:
	.asciz	"_ZN4rand3rng3Rng6random17he47c0a49024989aaE"
.Linfo_string191:
	.asciz	"random<rand::rngs::thread::ThreadRng, bool>"
.Linfo_string192:
	.asciz	"_ZN4rand5distr5other110_$LT$impl$u20$rand..distr..distribution..Distribution$LT$bool$GT$$u20$for$u20$rand..distr..StandardUniform$GT$6sample17he90ea3740add583dE"
.Linfo_string193:
	.asciz	"sample<rand::rngs::thread::ThreadRng>"
.Linfo_string194:
	.asciz	"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h61953a0fd5719ceaE"
.Linfo_string195:
	.asciz	"report"
.Linfo_string196:
	.asciz	"U"
.Linfo_string197:
	.asciz	"_ZN63_$LT$$RF$mut$u20$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17hcf4f85feb7206a28E"
.Linfo_string198:
	.asciz	"as_ref<rand_chacha::chacha::Array64<u32>, [u32]>"
.Linfo_string199:
	.asciz	"data_ptr"
.Linfo_string200:
	.asciz	"length"
.Linfo_string201:
	.asciz	"&[u32]"
.Linfo_string202:
	.asciz	"_ZN74_$LT$rand_core..block..BlockRng$LT$R$GT$$u20$as$u20$rand_core..RngCore$GT$8next_u3217h5ba689e9c842bbddE"
.Linfo_string203:
	.asciz	"next_u32<rand::rngs::reseeding::ReseedingCore<rand_chacha::chacha::ChaCha12Core, rand_core::os::OsRng>>"
.Linfo_string204:
	.asciz	"_ZN84_$LT$rand_chacha..chacha..ChaCha12Core$u20$as$u20$rand_core..block..BlockRngCore$GT$8generate17h30baf41324780ba0E"
.Linfo_string205:
	.asciz	"generate"
.Linfo_string206:
	.asciz	"_ZN44_$LT$dp_ex..Dog$u20$as$u20$dp_ex..Animal$GT$5speak17had468e7662b92aa1E"
.Linfo_string207:
	.asciz	"speak"
.Linfo_string208:
	.asciz	"_ZN44_$LT$dp_ex..Cat$u20$as$u20$dp_ex..Animal$GT$5speak17h504ab8d43430beb1E"
.Linfo_string209:
	.asciz	"_ZN5dp_ex6dyn_dp17h2163eacfb0f4ef0dE"
.Linfo_string210:
	.asciz	"dyn_dp"
.Linfo_string211:
	.asciz	"_ZN5dp_ex9static_dp17h12c823b24041d8eeE"
.Linfo_string212:
	.asciz	"static_dp"
.Linfo_string213:
	.asciz	"_ZN5dp_ex4main17hfaa1a74bb1a25d42E"
.Linfo_string214:
	.asciz	"num_bytes"
.Linfo_string215:
	.asciz	"argc"
.Linfo_string216:
	.asciz	"argv"
.Linfo_string217:
	.asciz	"*const u8"
.Linfo_string218:
	.asciz	"*const *const u8"
.Linfo_string219:
	.asciz	"sigpipe"
.Linfo_string220:
	.asciz	"result"
.Linfo_string221:
	.asciz	"f"
.Linfo_string222:
	.asciz	"val"
.Linfo_string223:
	.asciz	"*mut std::rt::lang_start::{closure_env#0}<()>"
.Linfo_string224:
	.asciz	"*mut dp_ex::Cat"
.Linfo_string225:
	.asciz	"*mut dp_ex::Dog"
.Linfo_string226:
	.asciz	"*mut rand::rngs::thread::ThreadRng"
.Linfo_string227:
	.asciz	"e"
.Linfo_string228:
	.asciz	"getrandom"
.Linfo_string229:
	.asciz	"error"
.Linfo_string230:
	.asciz	"num"
.Linfo_string231:
	.asciz	"nonzero"
.Linfo_string232:
	.asciz	"niche_types"
.Linfo_string233:
	.asciz	"NonZeroI32Inner"
.Linfo_string234:
	.asciz	"NonZero<i32>"
.Linfo_string235:
	.asciz	"Error"
.Linfo_string236:
	.asciz	"OsError"
.Linfo_string237:
	.asciz	"StandardUniform"
.Linfo_string238:
	.asciz	"&rand::distr::StandardUniform"
.Linfo_string239:
	.asciz	"&&mut rand_chacha::chacha::Array64<u32>"
.Linfo_string240:
	.asciz	"&mut rand_chacha::chacha::ChaCha12Core"
.Linfo_string241:
	.asciz	"r"
.Linfo_string242:
	.asciz	"&dp_ex::Dog"
.Linfo_string243:
	.asciz	"&dp_ex::Cat"
.Linfo_string244:
	.asciz	"a"
.Linfo_string245:
	.asciz	"dyn dp_ex::Animal"
.Linfo_string246:
	.asciz	"vtable"
.Linfo_string247:
	.asciz	"&[usize; 4]"
.Linfo_string248:
	.asciz	"&dyn dp_ex::Animal"
.Linfo_string249:
	.asciz	"cat"
	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.ident	"rustc version 1.87.0-nightly (ecade534c 2025-03-14)"
	.section	".note.GNU-stack","",@progbits
	.section	.debug_line,"",@progbits
.Lline_table_start0:
