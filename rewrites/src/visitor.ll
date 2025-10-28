; ModuleID = 'visitor.bd645a118b4d35f2-cgu.0'
source_filename = "visitor.bd645a118b4d35f2-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"alloc::string::String" = type { %"alloc::vec::Vec<u8>" }
%"alloc::vec::Vec<u8>" = type { %"alloc::raw_vec::RawVec<u8>", i64 }
%"alloc::raw_vec::RawVec<u8>" = type { %"alloc::raw_vec::RawVecInner", %"core::marker::PhantomData<u8>" }
%"alloc::raw_vec::RawVecInner" = type { i64, ptr, %"alloc::alloc::Global" }
%"alloc::alloc::Global" = type {}
%"core::marker::PhantomData<u8>" = type {}
%"std::ffi::os_str::OsString" = type { %"std::sys::os_str::bytes::Buf" }
%"std::sys::os_str::bytes::Buf" = type { %"alloc::vec::Vec<u8>" }

@alloc_6f044937b383e8a24c927d5792e34522 = private unnamed_addr constant [77 x i8] c"/rustc/c8905eaa66e0c35a33626e974b9ce6955c739b5b/library/alloc/src/vec/mod.rs\00", align 1
@alloc_cfb6177d9d971911008c59a6c6f48657 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_6f044937b383e8a24c927d5792e34522, [16 x i8] c"L\00\00\00\00\00\00\00\BB\07\00\00\09\00\00\00" }>, align 8
@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he161afa8cbc4a796E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h482cc34c01180746E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h482cc34c01180746E" }>, align 8
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN48_$LT$visitor..Cat$u20$as$u20$visitor..Animal$GT$5speak17h33a1a39b85f49667E", ptr @"_ZN48_$LT$visitor..Cat$u20$as$u20$visitor..Animal$GT$5visit17h17a13a6e11bb2d16E" }>, align 8
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN48_$LT$visitor..Dog$u20$as$u20$visitor..Animal$GT$5speak17haa78590129a75b29E", ptr @"_ZN48_$LT$visitor..Dog$u20$as$u20$visitor..Animal$GT$5visit17h836b1a9842922031E" }>, align 8
@anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@alloc_8de87da3563342324e7c06669ee7121f = private unnamed_addr constant [76 x i8] c"/rustc/c8905eaa66e0c35a33626e974b9ce6955c739b5b/library/core/src/num/mod.rs\00", align 1
@alloc_6287d50ab95567f2f9e7d9dc52d679ca = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_8de87da3563342324e7c06669ee7121f, [16 x i8] c"K\00\00\00\00\00\00\00w\06\00\00\01\00\00\00" }>, align 8
@alloc_560a59ed819b9d9a5841f6e731c4c8e5 = private unnamed_addr constant [210 x i8] c"unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_daba3d88daebedb52f113d2be28fba64 = private unnamed_addr constant [81 x i8] c"/rustc/c8905eaa66e0c35a33626e974b9ce6955c739b5b/library/core/src/ptr/non_null.rs\00", align 1
@alloc_0f62f2ecf19a0c1d9ed0e08499a31586 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_daba3d88daebedb52f113d2be28fba64, [16 x i8] c"P\00\00\00\00\00\00\00\B9\03\00\00 \00\00\00" }>, align 8
@alloc_ec595fc0e82ef92fc59bd74f68296eae = private unnamed_addr constant [73 x i8] c"assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize", align 1
@alloc_de4e626d456b04760e72bc785ed7e52a = private unnamed_addr constant [201 x i8] c"unsafe precondition(s) violated: ptr::offset_from_unsigned requires `self >= origin`\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_78b6a49e36fe808e0ef950c2feb9fafc = private unnamed_addr constant [71 x i8] c"to_digit: invalid radix -- radix must be in the range 2 to 36 inclusive", align 1
@alloc_708eb9f2492b697e0d761b647be5d46c = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_78b6a49e36fe808e0ef950c2feb9fafc, [8 x i8] c"G\00\00\00\00\00\00\00" }>, align 8
@alloc_c14c99a629c04d85e9d8597db3e943d9 = private unnamed_addr constant [81 x i8] c"/rustc/c8905eaa66e0c35a33626e974b9ce6955c739b5b/library/core/src/char/methods.rs\00", align 1
@alloc_29e637a9f3d378308c05008cb51c611f = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c14c99a629c04d85e9d8597db3e943d9, [16 x i8] c"P\00\00\00\00\00\00\00\93\01\00\00\09\00\00\00" }>, align 8
@anon.d9cc4ab2fa0032d6aa5f7f68905737c8.1 = private unnamed_addr constant <{ [4 x i8], [4 x i8] }> <{ [4 x i8] zeroinitializer, [4 x i8] undef }>, align 4
@alloc_64e308ef4babfeb8b6220184de794a17 = private unnamed_addr constant [221 x i8] c"unsafe precondition(s) violated: hint::assert_unchecked must never be called when the condition is false\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_8d4cce0439563258f6e216a28ffc54c1 = private unnamed_addr constant [91 x i8] c"/rustc/c8905eaa66e0c35a33626e974b9ce6955c739b5b/library/core/src/iter/traits/exact_size.rs\00", align 1
@alloc_8f80298b638aa40322b9c81f1de6d623 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_8d4cce0439563258f6e216a28ffc54c1, [16 x i8] c"Z\00\00\00\00\00\00\00z\00\00\00\09\00\00\00" }>, align 8
@alloc_1be5ea12ba708d9a11b6e93a7d387a75 = private unnamed_addr constant [281 x i8] c"unsafe precondition(s) violated: Layout::from_size_align_unchecked requires that align is a power of 2 and the rounded-up allocation size does not exceed isize::MAX\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_92c0abf59bb1e44c2416a0aa21ae1fc9 = private unnamed_addr constant [78 x i8] c"/rustc/c8905eaa66e0c35a33626e974b9ce6955c739b5b/library/core/src/ub_checks.rs\00", align 1
@alloc_5475fa92f2f69e75e8dff3e87ac79e4f = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_92c0abf59bb1e44c2416a0aa21ae1fc9, [16 x i8] c"M\00\00\00\00\00\00\00\95\00\00\006\00\00\00" }>, align 8
@alloc_a28e8c8fd5088943a8b5d44af697ff83 = private unnamed_addr constant [279 x i8] c"unsafe precondition(s) violated: slice::from_raw_parts requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_fad0cd83b7d1858a846a172eb260e593 = private unnamed_addr constant [42 x i8] c"is_aligned_to: align is not a power-of-two", align 1
@alloc_e92e94d0ff530782b571cfd99ec66aef = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_fad0cd83b7d1858a846a172eb260e593, [8 x i8] c"*\00\00\00\00\00\00\00" }>, align 8
@alloc_49d458586a68fcc7bc8ac0106a32e0b1 = private unnamed_addr constant [82 x i8] c"/rustc/c8905eaa66e0c35a33626e974b9ce6955c739b5b/library/core/src/ptr/const_ptr.rs\00", align 1
@alloc_d35c6aec622884a82e66ab090a94939a = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_49d458586a68fcc7bc8ac0106a32e0b1, [16 x i8] c"Q\00\00\00\00\00\00\00^\05\00\00\0D\00\00\00" }>, align 8
@vtable.3 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN68_$LT$core..num..error..ParseIntError$u20$as$u20$core..fmt..Debug$GT$3fmt17hbd1afecc70c04c9fE" }>, align 8
@alloc_00ae4b301f7fab8ac9617c03fcbd7274 = private unnamed_addr constant [43 x i8] c"called `Result::unwrap()` on an `Err` value", align 1
@alloc_57d70e9d94c65ecfc15225d29a5ed72b = private unnamed_addr constant [198 x i8] c"unsafe precondition(s) violated: Vec::set_len requires that new_len <= capacity()\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_3d246f6ec978c24b4eba8fa520efcee0 = private unnamed_addr constant [75 x i8] c"/rustc/c8905eaa66e0c35a33626e974b9ce6955c739b5b/library/alloc/src/alloc.rs\00", align 1
@alloc_54356c3bdf01fcc9e71537cdcbc33887 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_3d246f6ec978c24b4eba8fa520efcee0, [16 x i8] c"J\00\00\00\00\00\00\00_\01\00\00\1B\00\00\00" }>, align 8
@alloc_bec1f9f021c110596a13fd563f15177c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_daba3d88daebedb52f113d2be28fba64, [16 x i8] c"P\00\00\00\00\00\00\00\89\05\00\00\12\00\00\00" }>, align 8
@alloc_0944926afe5312fa319f558031c55b23 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_daba3d88daebedb52f113d2be28fba64, [16 x i8] c"P\00\00\00\00\00\00\00\10\01\00\00\1B\00\00\00" }>, align 8
@alloc_e2199c9e7f384be1c4af111238e1c571 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_6f044937b383e8a24c927d5792e34522, [16 x i8] c"L\00\00\00\00\00\00\00e\06\00\00\12\00\00\00" }>, align 8
@alloc_6c0131d81e8ea7016aa80c8430b107a4 = private unnamed_addr constant [81 x i8] c"/rustc/c8905eaa66e0c35a33626e974b9ce6955c739b5b/library/alloc/src/raw_vec/mod.rs\00", align 1
@alloc_d09f624e675dc1990a785862f9024a3e = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_6c0131d81e8ea7016aa80c8430b107a4, [16 x i8] c"P\00\00\00\00\00\00\00\A7\01\00\00\15\00\00\00" }>, align 8
@vtable.4 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h0d0d792182b233cdE" }>, align 8
@alloc_f62df14955f7d78bca139b0a7668683d = private unnamed_addr constant [13 x i8] c"ParseIntError", align 1
@alloc_a5d866b1768ad3f826bccdb004a1a8ae = private unnamed_addr constant [4 x i8] c"kind", align 1
@alloc_16a2855a7d5ed82111f0529d346414d3 = private unnamed_addr constant [81 x i8] c"/rustc/c8905eaa66e0c35a33626e974b9ce6955c739b5b/library/core/src/alloc/layout.rs\00", align 1
@alloc_c4ba26a389b5ded934a968bc0482ce95 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_16a2855a7d5ed82111f0529d346414d3, [16 x i8] c"P\00\00\00\00\00\00\00\E0\00\00\00\12\00\00\00" }>, align 8
@vtable.5 = private unnamed_addr constant <{ [24 x i8], ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN67_$LT$visitor..SpeakBetterDogs$u20$as$u20$visitor..AnimalVisitor$GT$11receive_dog17h9724d8b1f3575287E", ptr @"_ZN67_$LT$visitor..SpeakBetterDogs$u20$as$u20$visitor..AnimalVisitor$GT$11receive_cat17h775be00c4adb485aE" }>, align 8
@alloc_c85489ac7d65f986716cfdb1dc3aed62 = private unnamed_addr constant [39 x i8] c"Pass in a number and see what happens!\0A", align 1
@alloc_da03007fb5fe1aab10d9ffd81ef29605 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_c85489ac7d65f986716cfdb1dc3aed62, [8 x i8] c"'\00\00\00\00\00\00\00" }>, align 8
@alloc_bb20a7c9ecab86b2b99c40df9e454434 = private unnamed_addr constant [11 x i8] c"visitor.rs\00", align 1
@alloc_818da8dffaa466cf3fa20cda7bc167b2 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_bb20a7c9ecab86b2b99c40df9e454434, [16 x i8] c"\0A\00\00\00\00\00\00\00\83\00\00\00$\00\00\00" }>, align 8
@alloc_da68476bc014f050336d4aef91e18025 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_bb20a7c9ecab86b2b99c40df9e454434, [16 x i8] c"\0A\00\00\00\00\00\00\00\83\00\00\000\00\00\00" }>, align 8

; <alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::size_hint
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17ha3687b90d10d927bE"(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %self) unnamed_addr #0 {
start:
  %exact = alloca [8 x i8], align 8
  br label %bb2

bb2:                                              ; preds = %start
  %_10 = getelementptr inbounds i8, ptr %self, i64 24
  %_8 = load ptr, ptr %_10, align 8
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %_11 = load ptr, ptr %0, align 8
; call core::ptr::non_null::NonNull<T>::offset_from_unsigned
  %1 = call i64 @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$20offset_from_unsigned17he7312f1b13fd2594E"(ptr %_8, ptr %_11)
  store i64 %1, ptr %exact, align 8
  br label %bb4

bb4:                                              ; preds = %bb2
  %_12 = load i64, ptr %exact, align 8
  %_13.1 = load i64, ptr %exact, align 8
  store i64 %_12, ptr %_0, align 8
  %2 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %2, i64 8
  store i64 %_13.1, ptr %3, align 8
  ret void

bb1:                                              ; No predecessors!
  unreachable
}

; <alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17hacbe05db63113b4dE"(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %iterator) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [8 x i8], align 8
  %1 = alloca [16 x i8], align 8
  %_32 = alloca [8 x i8], align 8
  %_20 = alloca [1 x i8], align 1
  %_19 = alloca [32 x i8], align 8
  %src = alloca [24 x i8], align 8
  %vector1 = alloca [24 x i8], align 8
  %_8 = alloca [24 x i8], align 8
  %element = alloca [24 x i8], align 8
  %_3 = alloca [24 x i8], align 8
  %vector = alloca [24 x i8], align 8
  store i8 1, ptr %_20, align 1
; invoke <std::env::Args as core::iter::traits::iterator::Iterator>::next
  invoke void @"_ZN73_$LT$std..env..Args$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h28703ed5d9e04e32E"(ptr sret([24 x i8]) align 8 %_3, ptr align 8 %iterator)
          to label %bb1 unwind label %cleanup

bb11:                                             ; preds = %bb9, %bb7, %cleanup
  %2 = load i8, ptr %_20, align 1
  %3 = trunc nuw i8 %2 to i1
  br i1 %3, label %bb10, label %bb8

cleanup:                                          ; preds = %start
  %4 = landingpad { ptr, i32 }
          cleanup
  %5 = extractvalue { ptr, i32 } %4, 0
  %6 = extractvalue { ptr, i32 } %4, 1
  store ptr %5, ptr %1, align 8
  %7 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %6, ptr %7, align 8
  br label %bb11

bb1:                                              ; preds = %start
  %8 = load i64, ptr %_3, align 8
  %9 = icmp eq i64 %8, -9223372036854775808
  %_5 = select i1 %9, i64 0, i64 1
  %10 = trunc nuw i64 %_5 to i1
  br i1 %10, label %bb3, label %bb12

bb3:                                              ; preds = %bb1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %element, ptr align 8 %_3, i64 24, i1 false)
; invoke <std::env::Args as core::iter::traits::iterator::Iterator>::size_hint
  invoke void @"_ZN73_$LT$std..env..Args$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17hca4464b764a5cf8eE"(ptr sret([24 x i8]) align 8 %_8, ptr align 8 %iterator)
          to label %bb4 unwind label %cleanup2

bb12:                                             ; preds = %bb1
  store i64 0, ptr %_0, align 8
  %11 = getelementptr inbounds i8, ptr %_0, i64 8
  store ptr getelementptr (i8, ptr null, i64 8), ptr %11, align 8
  %12 = getelementptr inbounds i8, ptr %_0, i64 16
  store i64 0, ptr %12, align 8
; call core::ptr::drop_in_place<std::env::Args>
  call void @"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd8d2745dbc46aebaE"(ptr align 8 %iterator)
  br label %bb6

bb6:                                              ; preds = %bb5, %bb12
  ret void

bb9:                                              ; preds = %cleanup2
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hf12fa1af090a7d2dE"(ptr align 8 %element) #18
          to label %bb11 unwind label %terminate

cleanup2:                                         ; preds = %bb14, %bb4, %bb3
  %13 = landingpad { ptr, i32 }
          cleanup
  %14 = extractvalue { ptr, i32 } %13, 0
  %15 = extractvalue { ptr, i32 } %13, 1
  store ptr %14, ptr %1, align 8
  %16 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %15, ptr %16, align 8
  br label %bb9

bb4:                                              ; preds = %bb3
  %lower = load i64, ptr %_8, align 8
  %17 = call i64 @llvm.uadd.sat.i64(i64 %lower, i64 1)
  store i64 %17, ptr %0, align 8
  %v2 = load i64, ptr %0, align 8
; invoke core::cmp::Ord::max
  %initial_capacity = invoke i64 @_ZN4core3cmp3Ord3max17hd057d6b10806e669E(i64 4, i64 %v2)
          to label %bb14 unwind label %cleanup2

bb14:                                             ; preds = %bb4
; invoke alloc::raw_vec::RawVecInner<A>::with_capacity_in
  %18 = invoke { i64, ptr } @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17ha96019c9826ab6b4E"(i64 %initial_capacity, i64 8, i64 24)
          to label %bb15 unwind label %cleanup2

bb15:                                             ; preds = %bb14
  %_28.0 = extractvalue { i64, ptr } %18, 0
  %_28.1 = extractvalue { i64, ptr } %18, 1
  store i64 %_28.0, ptr %vector1, align 8
  %19 = getelementptr inbounds i8, ptr %vector1, i64 8
  store ptr %_28.1, ptr %19, align 8
  %20 = getelementptr inbounds i8, ptr %vector1, i64 16
  store i64 0, ptr %20, align 8
  %21 = getelementptr inbounds i8, ptr %vector1, i64 8
  %_29 = load ptr, ptr %21, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %src, ptr align 8 %element, i64 24, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_29, ptr align 8 %src, i64 24, i1 false)
  br label %bb16

bb16:                                             ; preds = %bb15
  br label %bb21

bb21:                                             ; preds = %bb16
  %self = load i64, ptr %vector1, align 8
  store i64 %self, ptr %_32, align 8
  br label %bb19

bb20:                                             ; No predecessors!
  store i64 -1, ptr %_32, align 8
  br label %bb19

bb19:                                             ; preds = %bb21, %bb20
  %22 = load i64, ptr %_32, align 8
; call alloc::vec::Vec<T,A>::set_len::precondition_check
  call void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7set_len18precondition_check17h4038c2036fef2dbeE"(i64 1, i64 %22, ptr align 8 @alloc_cfb6177d9d971911008c59a6c6f48657) #19
  br label %bb18

bb18:                                             ; preds = %bb19
  %23 = getelementptr inbounds i8, ptr %vector1, i64 16
  store i64 1, ptr %23, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %vector, ptr align 8 %vector1, i64 24, i1 false)
  store i8 0, ptr %_20, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_19, ptr align 8 %iterator, i64 32, i1 false)
; invoke <alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend
  invoke void @"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17hffec8aa3fee714b6E"(ptr align 8 %vector, ptr align 8 %_19)
          to label %bb5 unwind label %cleanup3

bb7:                                              ; preds = %cleanup3
; invoke core::ptr::drop_in_place<alloc::vec::Vec<alloc::string::String>>
  invoke void @"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hdafa2e272e277cefE"(ptr align 8 %vector) #18
          to label %bb11 unwind label %terminate

cleanup3:                                         ; preds = %bb18
  %24 = landingpad { ptr, i32 }
          cleanup
  %25 = extractvalue { ptr, i32 } %24, 0
  %26 = extractvalue { ptr, i32 } %24, 1
  store ptr %25, ptr %1, align 8
  %27 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %26, ptr %27, align 8
  br label %bb7

bb5:                                              ; preds = %bb18
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %vector, i64 24, i1 false)
  br label %bb6

terminate:                                        ; preds = %bb10, %bb9, %bb7
  %28 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() #20
  unreachable

bb2:                                              ; No predecessors!
  unreachable

bb8:                                              ; preds = %bb10, %bb11
  %29 = load ptr, ptr %1, align 8
  %30 = getelementptr inbounds i8, ptr %1, i64 8
  %31 = load i32, ptr %30, align 8
  %32 = insertvalue { ptr, i32 } poison, ptr %29, 0
  %33 = insertvalue { ptr, i32 } %32, i32 %31, 1
  resume { ptr, i32 } %33

bb10:                                             ; preds = %bb11
; invoke core::ptr::drop_in_place<std::env::Args>
  invoke void @"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd8d2745dbc46aebaE"(ptr align 8 %iterator) #18
          to label %bb8 unwind label %terminate
}

; <<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN157_$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h844c3780aa5b83b6E"(ptr align 8 %self) unnamed_addr #1 {
start:
  %capacity = alloca [8 x i8], align 8
  %_4 = alloca [16 x i8], align 8
  %_7 = load ptr, ptr %self, align 8
  %slot = getelementptr inbounds i8, ptr %_7, i64 32
  %_8 = load ptr, ptr %self, align 8
  %ptr = load ptr, ptr %_8, align 8
  %_9 = load ptr, ptr %self, align 8
  %0 = getelementptr inbounds i8, ptr %_9, i64 16
  %capacity1 = load i64, ptr %0, align 8
  br label %bb4

bb4:                                              ; preds = %start
  store i64 %capacity1, ptr %capacity, align 8
  br label %bb2

bb2:                                              ; preds = %bb4
  %_12.0 = load i64, ptr %capacity, align 8
  store i64 %_12.0, ptr %_4, align 8
  %1 = getelementptr inbounds i8, ptr %_4, i64 8
  store ptr %ptr, ptr %1, align 8
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<std::ffi::os_str::OsString>>
  call void @"_ZN4core3ptr77drop_in_place$LT$alloc..raw_vec..RawVec$LT$std..ffi..os_str..OsString$GT$$GT$17h9b711af4b26a5dd5E"(ptr align 8 %_4)
  ret void

bb3:                                              ; No predecessors!
  unreachable
}

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17hbfdf2433702e39d1E(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #1 {
start:
  %_7 = alloca [8 x i8], align 8
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call i64 @_ZN3std2rt19lang_start_internal17h0af9494e5b7ea246E(ptr align 1 %_7, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe)
  ret i64 %_0
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h482cc34c01180746E"(ptr align 8 %_1) unnamed_addr #0 {
start:
  %_4 = load ptr, ptr %_1, align 8
; call std::sys::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17ha9110c82a40cb976E(ptr %_4)
; call <() as std::process::Termination>::report
  %self = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h42e0f9b21e8fc2b1E"()
  %_0 = zext i8 %self to i32
  ret i32 %_0
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17ha9110c82a40cb976E(ptr %f) unnamed_addr #2 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h1633c4a330b0e1a3E(ptr %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !4
  ret void
}

; <visitor::Cat as visitor::Animal>::speak
; Function Attrs: nonlazybind uwtable
define internal i64 @"_ZN48_$LT$visitor..Cat$u20$as$u20$visitor..Animal$GT$5speak17h33a1a39b85f49667E"(ptr align 1 %self) unnamed_addr #1 {
start:
  ret i64 11111
}

; <visitor::Cat as visitor::Animal>::visit
; Function Attrs: nonlazybind uwtable
define internal i64 @"_ZN48_$LT$visitor..Cat$u20$as$u20$visitor..Animal$GT$5visit17h17a13a6e11bb2d16E"(ptr align 1 %self, ptr align 1 %av.0, ptr align 8 %av.1) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds i8, ptr %av.1, i64 32
  %1 = load ptr, ptr %0, align 8, !invariant.load !5, !nonnull !5
  %_0 = call i64 %1(ptr align 1 %av.0, ptr align 1 %self, ptr align 8 @vtable.1)
  ret i64 %_0
}

; <visitor::Dog as visitor::Animal>::speak
; Function Attrs: nonlazybind uwtable
define internal i64 @"_ZN48_$LT$visitor..Dog$u20$as$u20$visitor..Animal$GT$5speak17haa78590129a75b29E"(ptr align 1 %self) unnamed_addr #1 {
start:
  ret i64 22222
}

; <visitor::Dog as visitor::Animal>::visit
; Function Attrs: nonlazybind uwtable
define internal i64 @"_ZN48_$LT$visitor..Dog$u20$as$u20$visitor..Animal$GT$5visit17h836b1a9842922031E"(ptr align 1 %self, ptr align 1 %av.0, ptr align 8 %av.1) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds i8, ptr %av.1, i64 24
  %1 = load ptr, ptr %0, align 8, !invariant.load !5, !nonnull !5
  %_0 = call i64 %1(ptr align 1 %av.0, ptr align 1 %self, ptr align 8 @vtable.2)
  ret i64 %_0
}

; core::intrinsics::cold_path
; Function Attrs: cold nounwind nonlazybind uwtable
define internal void @_ZN4core10intrinsics9cold_path17he85e9656edbd0508E() unnamed_addr #3 {
start:
  ret void
}

; core::cmp::Ord::max
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @_ZN4core3cmp3Ord3max17hd057d6b10806e669E(i64 %0, i64 %1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %2 = alloca [16 x i8], align 8
  %_6 = alloca [1 x i8], align 1
  %_0 = alloca [8 x i8], align 8
  %other = alloca [8 x i8], align 8
  %self = alloca [8 x i8], align 8
  store i64 %0, ptr %self, align 8
  store i64 %1, ptr %other, align 8
  store i8 1, ptr %_6, align 1
  %_3.i = load i64, ptr %other, align 8
  %_4.i = load i64, ptr %self, align 8
  %_0.i = icmp ult i64 %_3.i, %_4.i
  br label %bb1

bb5:                                              ; preds = %cleanup
  br label %bb9

cleanup:                                          ; No predecessors!
  %3 = landingpad { ptr, i32 }
          cleanup
  %4 = extractvalue { ptr, i32 } %3, 0
  %5 = extractvalue { ptr, i32 } %3, 1
  store ptr %4, ptr %2, align 8
  %6 = getelementptr inbounds i8, ptr %2, i64 8
  store i32 %5, ptr %6, align 8
  br label %bb5

bb1:                                              ; preds = %start
  br i1 %_0.i, label %bb2, label %bb3

bb3:                                              ; preds = %bb1
  %7 = load i64, ptr %other, align 8
  store i64 %7, ptr %_0, align 8
  %8 = load i8, ptr %_6, align 1
  %9 = trunc nuw i8 %8 to i1
  br i1 %9, label %bb7, label %bb4

bb2:                                              ; preds = %bb1
  store i8 0, ptr %_6, align 1
  %10 = load i64, ptr %self, align 8
  store i64 %10, ptr %_0, align 8
  br label %bb4

bb4:                                              ; preds = %bb2, %bb7, %bb3
  %11 = load i64, ptr %_0, align 8
  ret i64 %11

bb7:                                              ; preds = %bb3
  br label %bb4

bb9:                                              ; preds = %bb5
  %12 = load i8, ptr %_6, align 1
  %13 = trunc nuw i8 %12 to i1
  br i1 %13, label %bb8, label %bb6

bb6:                                              ; preds = %bb8, %bb9
  %14 = load ptr, ptr %2, align 8
  %15 = getelementptr inbounds i8, ptr %2, i64 8
  %16 = load i32, ptr %15, align 8
  %17 = insertvalue { ptr, i32 } poison, ptr %14, 0
  %18 = insertvalue { ptr, i32 } %17, i32 %16, 1
  resume { ptr, i32 } %18

bb8:                                              ; preds = %bb9
  br label %bb6
}

; core::fmt::rt::<impl core::fmt::Arguments>::new_const
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3eaa9ce0927cf188E"(ptr sret([48 x i8]) align 8 %_0, ptr align 8 %pieces) unnamed_addr #0 {
start:
  store ptr %pieces, ptr %_0, align 8
  %0 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 1, ptr %0, align 8
  %1 = load ptr, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  %3 = getelementptr inbounds i8, ptr %_0, i64 32
  store ptr %1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  store i64 %2, ptr %4, align 8
  %5 = getelementptr inbounds i8, ptr %_0, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %5, align 8
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  store i64 0, ptr %6, align 8
  ret void
}

; core::num::<impl usize>::from_ascii_radix
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3num23_$LT$impl$u20$usize$GT$16from_ascii_radix17h68649e2238f5d3a6E"(ptr sret([16 x i8]) align 8 %_0, ptr align 1 %0, i64 %1, i32 %radix) unnamed_addr #0 {
start:
  %is_positive = alloca [1 x i8], align 1
  %_59 = alloca [8 x i8], align 8
  %_58 = alloca [1 x i8], align 1
  %result = alloca [8 x i8], align 8
  %_43 = alloca [16 x i8], align 8
  %_37 = alloca [8 x i8], align 4
  %mul = alloca [16 x i8], align 8
  %_19 = alloca [8 x i8], align 4
  %digits = alloca [16 x i8], align 8
  %radix1 = alloca [4 x i8], align 4
  %_10 = alloca [1 x i8], align 1
  %rest = alloca [16 x i8], align 8
  %src = alloca [16 x i8], align 8
  store ptr %0, ptr %src, align 8
  %2 = getelementptr inbounds i8, ptr %src, i64 8
  store i64 %1, ptr %2, align 8
  %_3 = icmp ugt i32 2, %radix
  br i1 %_3, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %_4 = icmp ugt i32 %radix, 36
  br i1 %_4, label %bb2, label %bb3

bb2:                                              ; preds = %bb1, %start
; call core::num::from_ascii_radix_panic
  call void @_ZN4core3num22from_ascii_radix_panic17h97b498ae62be534eE(i32 %radix, ptr align 8 @alloc_6287d50ab95567f2f9e7d9dc52d679ca) #21
  unreachable

bb3:                                              ; preds = %bb1
  %3 = load ptr, ptr %src, align 8
  %4 = getelementptr inbounds i8, ptr %src, i64 8
  %_57 = load i64, ptr %4, align 8
  %5 = icmp eq i64 %_57, 0
  br i1 %5, label %bb4, label %bb5

bb4:                                              ; preds = %bb3
  %6 = getelementptr inbounds i8, ptr %_0, i64 1
  store i8 0, ptr %6, align 1
  store i8 1, ptr %_0, align 8
  br label %bb31

bb5:                                              ; preds = %bb3
  %7 = icmp eq i64 %_57, 1
  br i1 %7, label %bb7, label %bb6

bb31:                                             ; preds = %bb30, %bb28, %bb12, %bb4
  ret void

bb7:                                              ; preds = %bb5
  %8 = load ptr, ptr %src, align 8
  %9 = getelementptr inbounds i8, ptr %src, i64 8
  %10 = load i64, ptr %9, align 8
  %11 = getelementptr inbounds nuw i8, ptr %8, i64 0
  %12 = load i8, ptr %11, align 1
  switch i8 %12, label %bb6 [
    i8 43, label %bb12
    i8 45, label %bb12
  ]

bb6:                                              ; preds = %bb7, %bb5
  %_6 = icmp uge i64 %_57, 1
  br i1 %_6, label %bb9, label %bb8

bb12:                                             ; preds = %bb7, %bb7
  %13 = getelementptr inbounds i8, ptr %_0, i64 1
  store i8 1, ptr %13, align 1
  store i8 1, ptr %_0, align 8
  br label %bb31

bb8:                                              ; preds = %bb10, %bb9, %bb6
  store i8 1, ptr %is_positive, align 1
  store i64 0, ptr %result, align 8
  store i32 %radix, ptr %radix1, align 4
  %14 = load ptr, ptr %src, align 8
  %15 = getelementptr inbounds i8, ptr %src, i64 8
  %16 = load i64, ptr %15, align 8
  store ptr %14, ptr %digits, align 8
  %17 = getelementptr inbounds i8, ptr %digits, i64 8
  store i64 %16, ptr %17, align 8
  %18 = icmp ule i32 %radix, 16
  %19 = zext i1 %18 to i8
  store i8 %19, ptr %_58, align 1
  %20 = load i8, ptr %_58, align 1
  %21 = trunc nuw i8 %20 to i1
  br i1 %21, label %bb40, label %bb44

bb9:                                              ; preds = %bb6
  %22 = load ptr, ptr %src, align 8
  %23 = getelementptr inbounds i8, ptr %src, i64 8
  %24 = load i64, ptr %23, align 8
  %25 = getelementptr inbounds nuw i8, ptr %22, i64 0
  %26 = load i8, ptr %25, align 1
  switch i8 %26, label %bb8 [
    i8 43, label %bb11
    i8 45, label %bb10
  ]

bb11:                                             ; preds = %bb9
  %27 = load ptr, ptr %src, align 8
  %28 = getelementptr inbounds i8, ptr %src, i64 8
  %29 = load i64, ptr %28, align 8
  %rest.0 = getelementptr inbounds nuw i8, ptr %27, i64 1
  %rest.1 = sub i64 %29, 1
  store i8 1, ptr %is_positive, align 1
  store ptr %rest.0, ptr %src, align 8
  %30 = getelementptr inbounds i8, ptr %src, i64 8
  store i64 %rest.1, ptr %30, align 8
  store i64 0, ptr %result, align 8
  store i32 %radix, ptr %radix1, align 4
  %31 = load ptr, ptr %src, align 8
  %32 = getelementptr inbounds i8, ptr %src, i64 8
  %33 = load i64, ptr %32, align 8
  store ptr %31, ptr %digits, align 8
  %34 = getelementptr inbounds i8, ptr %digits, i64 8
  store i64 %33, ptr %34, align 8
  %35 = icmp ule i32 %radix, 16
  %36 = zext i1 %35 to i8
  store i8 %36, ptr %_58, align 1
  %37 = load i8, ptr %_58, align 1
  %38 = trunc nuw i8 %37 to i1
  br i1 %38, label %bb42, label %bb45

bb10:                                             ; preds = %bb9
  %39 = load ptr, ptr %src, align 8
  %40 = getelementptr inbounds i8, ptr %src, i64 8
  %41 = load i64, ptr %40, align 8
  %42 = getelementptr inbounds nuw i8, ptr %39, i64 1
  %43 = sub i64 %41, 1
  store ptr %42, ptr %rest, align 8
  %44 = getelementptr inbounds i8, ptr %rest, i64 8
  store i64 %43, ptr %44, align 8
  br label %bb8

bb45:                                             ; preds = %bb11
  br label %bb19

bb42:                                             ; preds = %bb11
  %45 = load ptr, ptr %digits, align 8
  %46 = getelementptr inbounds i8, ptr %digits, i64 8
  %47 = load i64, ptr %46, align 8
  store i64 %47, ptr %_59, align 8
  %48 = load i64, ptr %_59, align 8
  %49 = icmp ule i64 %48, 16
  %50 = zext i1 %49 to i8
  store i8 %50, ptr %_10, align 1
  %51 = load i8, ptr %_10, align 1
  %52 = trunc nuw i8 %51 to i1
  br i1 %52, label %bb43, label %bb47

bb19:                                             ; preds = %bb27, %bb46, %bb44, %bb47, %bb45
  %53 = load ptr, ptr %src, align 8
  %54 = getelementptr inbounds i8, ptr %src, i64 8
  %_30 = load i64, ptr %54, align 8
  %_31 = icmp uge i64 %_30, 1
  br i1 %_31, label %bb20, label %bb28

bb47:                                             ; preds = %bb42
  br label %bb19

bb43:                                             ; preds = %bb42
  br label %bb13

bb13:                                             ; preds = %bb18, %bb41, %bb43
  %55 = load ptr, ptr %src, align 8
  %56 = getelementptr inbounds i8, ptr %src, i64 8
  %_13 = load i64, ptr %56, align 8
  %_14 = icmp uge i64 %_13, 1
  br i1 %_14, label %bb14, label %bb28

bb44:                                             ; preds = %bb8
  br label %bb19

bb40:                                             ; preds = %bb8
  %57 = load ptr, ptr %digits, align 8
  %58 = getelementptr inbounds i8, ptr %digits, i64 8
  %59 = load i64, ptr %58, align 8
  store i64 %59, ptr %_59, align 8
  %60 = load i64, ptr %_59, align 8
  %61 = icmp ule i64 %60, 16
  %62 = zext i1 %61 to i8
  store i8 %62, ptr %_10, align 1
  %63 = load i8, ptr %_10, align 1
  %64 = trunc nuw i8 %63 to i1
  br i1 %64, label %bb41, label %bb46

bb46:                                             ; preds = %bb40
  br label %bb19

bb41:                                             ; preds = %bb40
  br label %bb13

bb28:                                             ; preds = %bb13, %bb19
  %65 = load i64, ptr %result, align 8
  %66 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %65, ptr %66, align 8
  store i8 0, ptr %_0, align 8
  br label %bb31

bb20:                                             ; preds = %bb19
  %67 = load ptr, ptr %src, align 8
  %68 = getelementptr inbounds i8, ptr %src, i64 8
  %69 = load i64, ptr %68, align 8
  %c = getelementptr inbounds nuw i8, ptr %67, i64 0
  %70 = load ptr, ptr %src, align 8
  %71 = getelementptr inbounds i8, ptr %src, i64 8
  %72 = load i64, ptr %71, align 8
  %rest.02 = getelementptr inbounds nuw i8, ptr %70, i64 1
  %rest.13 = sub i64 %72, 1
  %rhs = zext i32 %radix to i64
  %_63 = load i64, ptr %result, align 8
  %73 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %_63, i64 %rhs)
  %_62.0 = extractvalue { i64, i1 } %73, 0
  %_62.1 = extractvalue { i64, i1 } %73, 1
  br i1 %_62.1, label %bb33, label %bb35

bb35:                                             ; preds = %bb20
  %74 = getelementptr inbounds i8, ptr %mul, i64 8
  store i64 %_62.0, ptr %74, align 8
  store i64 1, ptr %mul, align 8
  br label %bb32

bb33:                                             ; preds = %bb20
  %75 = load i64, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %76 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  store i64 %75, ptr %mul, align 8
  %77 = getelementptr inbounds i8, ptr %mul, i64 8
  store i64 %76, ptr %77, align 8
  br label %bb32

bb32:                                             ; preds = %bb33, %bb35
  %_39 = load i8, ptr %c, align 1
  %_38 = zext i8 %_39 to i32
; call core::char::methods::<impl char>::to_digit
  %78 = call { i32, i32 } @"_ZN4core4char7methods22_$LT$impl$u20$char$GT$8to_digit17h4acd8210d27828f0E"(i32 %_38, i32 %radix)
  %79 = extractvalue { i32, i32 } %78, 0
  %80 = extractvalue { i32, i32 } %78, 1
  store i32 %79, ptr %_37, align 4
  %81 = getelementptr inbounds i8, ptr %_37, i64 4
  store i32 %80, ptr %81, align 4
  %82 = load i32, ptr %_37, align 4
  %83 = getelementptr inbounds i8, ptr %_37, i64 4
  %84 = load i32, ptr %83, align 4
  %_40 = zext i32 %82 to i64
  %85 = trunc nuw i64 %_40 to i1
  br i1 %85, label %bb23, label %bb22

bb23:                                             ; preds = %bb32
  %86 = getelementptr inbounds i8, ptr %_37, i64 4
  %value = load i32, ptr %86, align 4
  %x = zext i32 %value to i64
  %_42 = load i64, ptr %mul, align 8
  %87 = getelementptr inbounds i8, ptr %mul, i64 8
  %88 = load i64, ptr %87, align 8
  %89 = trunc nuw i64 %_42 to i1
  br i1 %89, label %bb25, label %bb24

bb22:                                             ; preds = %bb32
  %90 = getelementptr inbounds i8, ptr %_0, i64 1
  store i8 1, ptr %90, align 1
  store i8 1, ptr %_0, align 8
  br label %bb29

bb29:                                             ; preds = %bb26, %bb24, %bb22
  br label %bb30

bb25:                                             ; preds = %bb23
  %91 = getelementptr inbounds i8, ptr %mul, i64 8
  %92 = load i64, ptr %91, align 8
  store i64 %92, ptr %result, align 8
  %93 = load i64, ptr %result, align 8
  %_68.0 = add i64 %93, %x
  %_68.1 = icmp ult i64 %_68.0, %93
  br i1 %_68.1, label %bb37, label %bb39

bb24:                                             ; preds = %bb23
  %94 = getelementptr inbounds i8, ptr %_0, i64 1
  store i8 2, ptr %94, align 1
  store i8 1, ptr %_0, align 8
  br label %bb29

bb39:                                             ; preds = %bb25
  %95 = load i64, ptr %result, align 8
  %_69 = add nuw i64 %95, %x
  %96 = getelementptr inbounds i8, ptr %_43, i64 8
  store i64 %_69, ptr %96, align 8
  store i64 1, ptr %_43, align 8
  br label %bb36

bb37:                                             ; preds = %bb25
  %97 = load i64, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %98 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  store i64 %97, ptr %_43, align 8
  %99 = getelementptr inbounds i8, ptr %_43, i64 8
  store i64 %98, ptr %99, align 8
  br label %bb36

bb36:                                             ; preds = %bb37, %bb39
  %_44 = load i64, ptr %_43, align 8
  %100 = getelementptr inbounds i8, ptr %_43, i64 8
  %101 = load i64, ptr %100, align 8
  %102 = trunc nuw i64 %_44 to i1
  br i1 %102, label %bb27, label %bb26

bb27:                                             ; preds = %bb36
  %103 = getelementptr inbounds i8, ptr %_43, i64 8
  %104 = load i64, ptr %103, align 8
  store i64 %104, ptr %result, align 8
  store ptr %rest.02, ptr %src, align 8
  %105 = getelementptr inbounds i8, ptr %src, i64 8
  store i64 %rest.13, ptr %105, align 8
  br label %bb19

bb26:                                             ; preds = %bb36
  %106 = getelementptr inbounds i8, ptr %_0, i64 1
  store i8 2, ptr %106, align 1
  store i8 1, ptr %_0, align 8
  br label %bb29

bb30:                                             ; preds = %bb17, %bb29
  br label %bb31

bb14:                                             ; preds = %bb13
  %107 = load ptr, ptr %src, align 8
  %108 = getelementptr inbounds i8, ptr %src, i64 8
  %109 = load i64, ptr %108, align 8
  %c4 = getelementptr inbounds nuw i8, ptr %107, i64 0
  %110 = load ptr, ptr %src, align 8
  %111 = getelementptr inbounds i8, ptr %src, i64 8
  %112 = load i64, ptr %111, align 8
  %rest.05 = getelementptr inbounds nuw i8, ptr %110, i64 1
  %rest.16 = sub i64 %112, 1
  %_17 = load i64, ptr %result, align 8
  %_18 = zext i32 %radix to i64
  %113 = mul i64 %_17, %_18
  store i64 %113, ptr %result, align 8
  %_21 = load i8, ptr %c4, align 1
  %_20 = zext i8 %_21 to i32
; call core::char::methods::<impl char>::to_digit
  %114 = call { i32, i32 } @"_ZN4core4char7methods22_$LT$impl$u20$char$GT$8to_digit17h4acd8210d27828f0E"(i32 %_20, i32 %radix)
  %115 = extractvalue { i32, i32 } %114, 0
  %116 = extractvalue { i32, i32 } %114, 1
  store i32 %115, ptr %_19, align 4
  %117 = getelementptr inbounds i8, ptr %_19, i64 4
  store i32 %116, ptr %117, align 4
  %118 = load i32, ptr %_19, align 4
  %119 = getelementptr inbounds i8, ptr %_19, i64 4
  %120 = load i32, ptr %119, align 4
  %_22 = zext i32 %118 to i64
  %121 = trunc nuw i64 %_22 to i1
  br i1 %121, label %bb18, label %bb17

bb18:                                             ; preds = %bb14
  %122 = getelementptr inbounds i8, ptr %_19, i64 4
  %x7 = load i32, ptr %122, align 4
  %_24 = load i64, ptr %result, align 8
  %_25 = zext i32 %x7 to i64
  %123 = add i64 %_24, %_25
  store i64 %123, ptr %result, align 8
  store ptr %rest.05, ptr %src, align 8
  %124 = getelementptr inbounds i8, ptr %src, i64 8
  store i64 %rest.16, ptr %124, align 8
  br label %bb13

bb17:                                             ; preds = %bb14
  %125 = getelementptr inbounds i8, ptr %_0, i64 1
  store i8 1, ptr %125, align 1
  store i8 1, ptr %_0, align 8
  br label %bb30

bb16:                                             ; No predecessors!
  unreachable
}

; core::num::<impl core::str::traits::FromStr for usize>::from_str
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3num62_$LT$impl$u20$core..str..traits..FromStr$u20$for$u20$usize$GT$8from_str17ha37b1cc88275a127E"(ptr sret([16 x i8]) align 8 %_0, ptr align 1 %src.0, i64 %src.1) unnamed_addr #0 {
start:
; call core::num::<impl usize>::from_ascii_radix
  call void @"_ZN4core3num23_$LT$impl$u20$usize$GT$16from_ascii_radix17h68649e2238f5d3a6E"(ptr sret([16 x i8]) align 8 %_0, ptr align 1 %src.0, i64 %src.1, i32 10)
  ret void
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he161afa8cbc4a796E"(ptr %_1) unnamed_addr #0 {
start:
  %_2 = alloca [0 x i8], align 1
  %0 = load ptr, ptr %_1, align 8
; call core::ops::function::FnOnce::call_once
  %_0 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h4c68378479e7604bE(ptr %0)
  ret i32 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h1633c4a330b0e1a3E(ptr %_1) unnamed_addr #0 {
start:
  %_2 = alloca [0 x i8], align 1
  call void %_1()
  ret void
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h4c68378479e7604bE(ptr %0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %1 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [8 x i8], align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %_0 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h482cc34c01180746E"(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %2 = load ptr, ptr %1, align 8
  %3 = getelementptr inbounds i8, ptr %1, i64 8
  %4 = load i32, ptr %3, align 8
  %5 = insertvalue { ptr, i32 } poison, ptr %2, 0
  %6 = insertvalue { ptr, i32 } %5, i32 %4, 1
  resume { ptr, i32 } %6

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
  ret i32 %_0
}

; core::ptr::drop_in_place<<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<std::ffi::os_str::OsString,alloc::alloc::Global>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17h7b55f9c91b02259eE"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call <<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN157_$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h844c3780aa5b83b6E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<std::env::Args>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd8d2745dbc46aebaE"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call core::ptr::drop_in_place<std::env::ArgsOs>
  call void @"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17h82daf37aefcae33bE"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<std::env::ArgsOs>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17h82daf37aefcae33bE"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call core::ptr::drop_in_place<std::sys::args::common::Args>
  call void @"_ZN4core3ptr49drop_in_place$LT$std..sys..args..common..Args$GT$17hdde8f2c1718a2afaE"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::string::String>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hf12fa1af090a7d2dE"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h5e26b9bc9589f517E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<dyn visitor::Animal>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr44drop_in_place$LT$dyn$u20$visitor..Animal$GT$17hcf59a1f5839a6c1fE"(ptr align 1 %_1.0, ptr align 8 %_1.1) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds i8, ptr %_1.1, i64 0
  %1 = load ptr, ptr %0, align 8, !invariant.load !5
  %2 = icmp ne ptr %1, null
  br i1 %2, label %is_not_null, label %bb1

is_not_null:                                      ; preds = %start
  call void %1(ptr %_1.0)
  br label %bb1

bb1:                                              ; preds = %is_not_null, %start
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<u8>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h5e26b9bc9589f517E"(ptr align 8 %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbf837dd3fa685310E"(ptr align 8 %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  invoke void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h94bc0f21b29bd3adE"(ptr align 8 %_1) #18
          to label %bb1 unwind label %terminate

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  store ptr %2, ptr %0, align 8
  %4 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %3, ptr %4, align 8
  br label %bb3

bb4:                                              ; preds = %start
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h94bc0f21b29bd3adE"(ptr align 8 %_1)
  ret void

terminate:                                        ; preds = %bb3
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() #20
  unreachable

bb1:                                              ; preds = %bb3
  %6 = load ptr, ptr %0, align 8
  %7 = getelementptr inbounds i8, ptr %0, i64 8
  %8 = load i32, ptr %7, align 8
  %9 = insertvalue { ptr, i32 } poison, ptr %6, 0
  %10 = insertvalue { ptr, i32 } %9, i32 %8, 1
  resume { ptr, i32 } %10
}

; core::ptr::drop_in_place<std::ffi::os_str::OsString>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hda1747e650e0ef18E"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call core::ptr::drop_in_place<std::sys::os_str::bytes::Buf>
  call void @"_ZN4core3ptr49drop_in_place$LT$std..sys..os_str..bytes..Buf$GT$17h59f0a307e1104d0aE"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<std::sys::args::common::Args>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr49drop_in_place$LT$std..sys..args..common..Args$GT$17hdde8f2c1718a2afaE"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>>
  call void @"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17h1bd2a08a5fed32a6E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<std::sys::os_str::bytes::Buf>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr49drop_in_place$LT$std..sys..os_str..bytes..Buf$GT$17h59f0a307e1104d0aE"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h5e26b9bc9589f517E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<[alloc::string::String]>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17h4bd412caa8a8841aE"(ptr align 8 %_1.0, i64 %_1.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_3 = alloca [8 x i8], align 8
  store i64 0, ptr %_3, align 8
  br label %bb6

bb6:                                              ; preds = %bb5, %start
  %1 = load i64, ptr %_3, align 8
  %_7 = icmp eq i64 %1, %_1.1
  br i1 %_7, label %bb1, label %bb5

bb5:                                              ; preds = %bb6
  %2 = load i64, ptr %_3, align 8
  %_6 = getelementptr inbounds nuw %"alloc::string::String", ptr %_1.0, i64 %2
  %3 = load i64, ptr %_3, align 8
  %4 = add i64 %3, 1
  store i64 %4, ptr %_3, align 8
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hf12fa1af090a7d2dE"(ptr align 8 %_6)
          to label %bb6 unwind label %cleanup

bb1:                                              ; preds = %bb6
  ret void

bb4:                                              ; preds = %bb3, %cleanup
  %5 = load i64, ptr %_3, align 8
  %_5 = icmp eq i64 %5, %_1.1
  br i1 %_5, label %bb2, label %bb3

cleanup:                                          ; preds = %bb5
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = extractvalue { ptr, i32 } %6, 0
  %8 = extractvalue { ptr, i32 } %6, 1
  store ptr %7, ptr %0, align 8
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %8, ptr %9, align 8
  br label %bb4

bb3:                                              ; preds = %bb4
  %10 = load i64, ptr %_3, align 8
  %_4 = getelementptr inbounds nuw %"alloc::string::String", ptr %_1.0, i64 %10
  %11 = load i64, ptr %_3, align 8
  %12 = add i64 %11, 1
  store i64 %12, ptr %_3, align 8
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hf12fa1af090a7d2dE"(ptr align 8 %_4) #18
          to label %bb4 unwind label %terminate

bb2:                                              ; preds = %bb4
  %13 = load ptr, ptr %0, align 8
  %14 = getelementptr inbounds i8, ptr %0, i64 8
  %15 = load i32, ptr %14, align 8
  %16 = insertvalue { ptr, i32 } poison, ptr %13, 0
  %17 = insertvalue { ptr, i32 } %16, i32 %15, 1
  resume { ptr, i32 } %17

terminate:                                        ; preds = %bb3
  %18 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() #20
  unreachable
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h94bc0f21b29bd3adE"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h52f54b56938d812bE"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<[std::ffi::os_str::OsString]>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17h527ae053523823f1E"(ptr align 8 %_1.0, i64 %_1.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_3 = alloca [8 x i8], align 8
  store i64 0, ptr %_3, align 8
  br label %bb6

bb6:                                              ; preds = %bb5, %start
  %1 = load i64, ptr %_3, align 8
  %_7 = icmp eq i64 %1, %_1.1
  br i1 %_7, label %bb1, label %bb5

bb5:                                              ; preds = %bb6
  %2 = load i64, ptr %_3, align 8
  %_6 = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %_1.0, i64 %2
  %3 = load i64, ptr %_3, align 8
  %4 = add i64 %3, 1
  store i64 %4, ptr %_3, align 8
; invoke core::ptr::drop_in_place<std::ffi::os_str::OsString>
  invoke void @"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hda1747e650e0ef18E"(ptr align 8 %_6)
          to label %bb6 unwind label %cleanup

bb1:                                              ; preds = %bb6
  ret void

bb4:                                              ; preds = %bb3, %cleanup
  %5 = load i64, ptr %_3, align 8
  %_5 = icmp eq i64 %5, %_1.1
  br i1 %_5, label %bb2, label %bb3

cleanup:                                          ; preds = %bb5
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = extractvalue { ptr, i32 } %6, 0
  %8 = extractvalue { ptr, i32 } %6, 1
  store ptr %7, ptr %0, align 8
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %8, ptr %9, align 8
  br label %bb4

bb3:                                              ; preds = %bb4
  %10 = load i64, ptr %_3, align 8
  %_4 = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %_1.0, i64 %10
  %11 = load i64, ptr %_3, align 8
  %12 = add i64 %11, 1
  store i64 %12, ptr %_3, align 8
; invoke core::ptr::drop_in_place<std::ffi::os_str::OsString>
  invoke void @"_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17hda1747e650e0ef18E"(ptr align 8 %_4) #18
          to label %bb4 unwind label %terminate

bb2:                                              ; preds = %bb4
  %13 = load ptr, ptr %0, align 8
  %14 = getelementptr inbounds i8, ptr %0, i64 8
  %15 = load i32, ptr %14, align 8
  %16 = insertvalue { ptr, i32 } poison, ptr %13, 0
  %17 = insertvalue { ptr, i32 } %16, i32 %15, 1
  resume { ptr, i32 } %17

terminate:                                        ; preds = %bb3
  %18 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() #20
  unreachable
}

; core::ptr::drop_in_place<alloc::vec::Vec<alloc::string::String>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hdafa2e272e277cefE"(ptr align 8 %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h564b856a31d78babE"(ptr align 8 %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<alloc::string::String>>
  invoke void @"_ZN4core3ptr72drop_in_place$LT$alloc..raw_vec..RawVec$LT$alloc..string..String$GT$$GT$17hea1b3b06024600e6E"(ptr align 8 %_1) #18
          to label %bb1 unwind label %terminate

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  store ptr %2, ptr %0, align 8
  %4 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %3, ptr %4, align 8
  br label %bb3

bb4:                                              ; preds = %start
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<alloc::string::String>>
  call void @"_ZN4core3ptr72drop_in_place$LT$alloc..raw_vec..RawVec$LT$alloc..string..String$GT$$GT$17hea1b3b06024600e6E"(ptr align 8 %_1)
  ret void

terminate:                                        ; preds = %bb3
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() #20
  unreachable

bb1:                                              ; preds = %bb3
  %6 = load ptr, ptr %0, align 8
  %7 = getelementptr inbounds i8, ptr %0, i64 8
  %8 = load i32, ptr %7, align 8
  %9 = insertvalue { ptr, i32 } poison, ptr %6, 0
  %10 = insertvalue { ptr, i32 } %9, i32 %8, 1
  resume { ptr, i32 } %10
}

; core::ptr::drop_in_place<alloc::boxed::Box<dyn visitor::Animal>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr69drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$visitor..Animal$GT$$GT$17hd6ede007c5530f41E"(ptr align 8 %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_6.0 = load ptr, ptr %_1, align 8
  %1 = getelementptr inbounds i8, ptr %_1, i64 8
  %_6.1 = load ptr, ptr %1, align 8
  %2 = getelementptr inbounds i8, ptr %_6.1, i64 0
  %3 = load ptr, ptr %2, align 8, !invariant.load !5
  %4 = icmp ne ptr %3, null
  br i1 %4, label %is_not_null, label %bb3

is_not_null:                                      ; preds = %start
  invoke void %3(ptr %_6.0)
          to label %bb3 unwind label %cleanup

bb3:                                              ; preds = %is_not_null, %start
; call <alloc::boxed::Box<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9d12390e067757c5E"(ptr align 8 %_1)
  ret void

bb4:                                              ; preds = %cleanup
; invoke <alloc::boxed::Box<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9d12390e067757c5E"(ptr align 8 %_1) #18
          to label %bb1 unwind label %terminate

cleanup:                                          ; preds = %is_not_null
  %5 = landingpad { ptr, i32 }
          cleanup
  %6 = extractvalue { ptr, i32 } %5, 0
  %7 = extractvalue { ptr, i32 } %5, 1
  store ptr %6, ptr %0, align 8
  %8 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %7, ptr %8, align 8
  br label %bb4

terminate:                                        ; preds = %bb4
  %9 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() #20
  unreachable

bb1:                                              ; preds = %bb4
  %10 = load ptr, ptr %0, align 8
  %11 = getelementptr inbounds i8, ptr %0, i64 8
  %12 = load i32, ptr %11, align 8
  %13 = insertvalue { ptr, i32 } poison, ptr %10, 0
  %14 = insertvalue { ptr, i32 } %13, i32 %12, 1
  resume { ptr, i32 } %14
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<alloc::string::String>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr72drop_in_place$LT$alloc..raw_vec..RawVec$LT$alloc..string..String$GT$$GT$17hea1b3b06024600e6E"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h2f2c5eb1e1cec426E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<std::ffi::os_str::OsString>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr77drop_in_place$LT$alloc..raw_vec..RawVec$LT$std..ffi..os_str..OsString$GT$$GT$17h9b711af4b26a5dd5E"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h514f7f901ceb5ef5E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<std::ffi::os_str::OsString>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17h1bd2a08a5fed32a6E"(ptr align 8 %_1) unnamed_addr #1 {
start:
; call <alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8c8d0c6a3a56fbc3E"(ptr align 8 %_1)
  ret void
}

; core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hdcc24cf9de78f312E"(ptr %ptr, ptr align 8 %0) unnamed_addr #4 {
start:
  %_5 = alloca [16 x i8], align 8
  %_3 = alloca [48 x i8], align 8
  %_6 = ptrtoint ptr %ptr to i64
  %1 = icmp eq i64 %_6, 0
  br i1 %1, label %bb1, label %bb2

bb1:                                              ; preds = %start
  %2 = getelementptr inbounds nuw { ptr, i64 }, ptr %_5, i64 0
  store ptr @alloc_560a59ed819b9d9a5841f6e731c4c8e5, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %2, i64 8
  store i64 210, ptr %3, align 8
  store ptr %_5, ptr %_3, align 8
  %4 = getelementptr inbounds i8, ptr %_3, i64 8
  store i64 1, ptr %4, align 8
  %5 = load ptr, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %6 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  %7 = getelementptr inbounds i8, ptr %_3, i64 32
  store ptr %5, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %7, i64 8
  store i64 %6, ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %_3, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %9, i64 8
  store i64 0, ptr %10, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h269f239d23f67cf5E(ptr align 8 %_3, i1 zeroext false, ptr align 8 %0) #22
  unreachable

bb2:                                              ; preds = %start
  ret void
}

; core::ptr::non_null::NonNull<T>::offset_from_unsigned
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$20offset_from_unsigned17he7312f1b13fd2594E"(ptr %self, ptr %subtracted) unnamed_addr #0 {
start:
  %0 = alloca [8 x i8], align 8
  br label %bb2

bb2:                                              ; preds = %start
; call core::ptr::const_ptr::<impl *const T>::offset_from_unsigned::precondition_check
  call void @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$20offset_from_unsigned18precondition_check17ha890b8a0bcf3941fE"(ptr %self, ptr %subtracted, ptr align 8 @alloc_0f62f2ecf19a0c1d9ed0e08499a31586) #19
  br label %bb4

bb4:                                              ; preds = %bb2
  br label %bb5

bb5:                                              ; preds = %bb4
  br label %bb6

bb6:                                              ; preds = %bb5
  %1 = ptrtoint ptr %self to i64
  %2 = ptrtoint ptr %subtracted to i64
  %3 = sub nuw i64 %1, %2
  %4 = udiv exact i64 %3, 24
  store i64 %4, ptr %0, align 8
  %_0 = load i64, ptr %0, align 8
  ret i64 %_0

bb7:                                              ; No predecessors!
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h5afda12f2d28188fE(ptr align 1 @alloc_ec595fc0e82ef92fc59bd74f68296eae, i64 73, ptr align 8 @alloc_0f62f2ecf19a0c1d9ed0e08499a31586) #21
  unreachable
}

; core::ptr::const_ptr::<impl *const T>::offset_from_unsigned::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$20offset_from_unsigned18precondition_check17ha890b8a0bcf3941fE"(ptr %this, ptr %origin, ptr align 8 %0) unnamed_addr #4 {
start:
  %_7 = alloca [16 x i8], align 8
  %_5 = alloca [48 x i8], align 8
  %_3 = icmp uge ptr %this, %origin
  br i1 %_3, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %1 = getelementptr inbounds nuw { ptr, i64 }, ptr %_7, i64 0
  store ptr @alloc_de4e626d456b04760e72bc785ed7e52a, ptr %1, align 8
  %2 = getelementptr inbounds i8, ptr %1, i64 8
  store i64 201, ptr %2, align 8
  store ptr %_7, ptr %_5, align 8
  %3 = getelementptr inbounds i8, ptr %_5, i64 8
  store i64 1, ptr %3, align 8
  %4 = load ptr, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %5 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  %6 = getelementptr inbounds i8, ptr %_5, i64 32
  store ptr %4, ptr %6, align 8
  %7 = getelementptr inbounds i8, ptr %6, i64 8
  store i64 %5, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %_5, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 0, ptr %9, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h269f239d23f67cf5E(ptr align 8 %_5, i1 zeroext false, ptr align 8 %0) #22
  unreachable

bb1:                                              ; preds = %start
  ret void
}

; core::str::<impl str>::parse
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3str21_$LT$impl$u20$str$GT$5parse17h5a5a3b1975ff4c07E"(ptr sret([16 x i8]) align 8 %_0, ptr align 1 %self.0, i64 %self.1) unnamed_addr #0 {
start:
; call core::num::<impl core::str::traits::FromStr for usize>::from_str
  call void @"_ZN4core3num62_$LT$impl$u20$core..str..traits..FromStr$u20$for$u20$usize$GT$8from_str17ha37b1cc88275a127E"(ptr sret([16 x i8]) align 8 %_0, ptr align 1 %self.0, i64 %self.1)
  ret void
}

; core::char::methods::<impl char>::to_digit
; Function Attrs: inlinehint nonlazybind uwtable
define internal { i32, i32 } @"_ZN4core4char7methods22_$LT$impl$u20$char$GT$8to_digit17h4acd8210d27828f0E"(i32 %self, i32 %radix) unnamed_addr #0 {
start:
  %value = alloca [4 x i8], align 4
  %_6 = alloca [48 x i8], align 8
  %_0 = alloca [8 x i8], align 4
  %_3 = icmp uge i32 %radix, 2
  br i1 %_3, label %bb1, label %bb3

bb3:                                              ; preds = %bb1, %start
  store ptr @alloc_708eb9f2492b697e0d761b647be5d46c, ptr %_6, align 8
  %0 = getelementptr inbounds i8, ptr %_6, i64 8
  store i64 1, ptr %0, align 8
  %1 = load ptr, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  %3 = getelementptr inbounds i8, ptr %_6, i64 32
  store ptr %1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  store i64 %2, ptr %4, align 8
  %5 = getelementptr inbounds i8, ptr %_6, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %5, align 8
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  store i64 0, ptr %6, align 8
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h0d47791d04b10c1eE(ptr align 8 %_6, ptr align 8 @alloc_29e637a9f3d378308c05008cb51c611f) #21
  unreachable

bb1:                                              ; preds = %start
  %_4 = icmp ule i32 %radix, 36
  br i1 %_4, label %bb2, label %bb3

bb2:                                              ; preds = %bb1
  %_8 = icmp ugt i32 %self, 57
  br i1 %_8, label %bb4, label %bb6

bb6:                                              ; preds = %bb4, %bb2
  %7 = sub i32 %self, 48
  store i32 %7, ptr %value, align 4
  br label %bb7

bb4:                                              ; preds = %bb2
  %_9 = icmp ugt i32 %radix, 10
  br i1 %_9, label %bb5, label %bb6

bb5:                                              ; preds = %bb4
  %_11 = sub i32 %self, 65
  %_10 = and i32 %_11, -33
  %8 = add i32 %_10, 10
  store i32 %8, ptr %value, align 4
  br label %bb7

bb7:                                              ; preds = %bb5, %bb6
  %_15 = load i32, ptr %value, align 4
  %_14 = icmp ult i32 %_15, %radix
  br i1 %_14, label %bb8, label %bb9

bb9:                                              ; preds = %bb7
  %9 = load i32, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.1, align 4
  %10 = load i32, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.1, i64 4), align 4
  store i32 %9, ptr %_0, align 4
  %11 = getelementptr inbounds i8, ptr %_0, i64 4
  store i32 %10, ptr %11, align 4
  br label %bb10

bb8:                                              ; preds = %bb7
  %12 = load i32, ptr %value, align 4
  %13 = getelementptr inbounds i8, ptr %_0, i64 4
  store i32 %12, ptr %13, align 4
  store i32 1, ptr %_0, align 4
  br label %bb10

bb10:                                             ; preds = %bb8, %bb9
  %14 = load i32, ptr %_0, align 4
  %15 = getelementptr inbounds i8, ptr %_0, i64 4
  %16 = load i32, ptr %15, align 4
  %17 = insertvalue { i32, i32 } poison, i32 %14, 0
  %18 = insertvalue { i32, i32 } %17, i32 %16, 1
  ret { i32, i32 } %18
}

; core::hint::assert_unchecked::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @_ZN4core4hint16assert_unchecked18precondition_check17haedab59b76512c9dE(i1 zeroext %cond, ptr align 8 %0) unnamed_addr #4 {
start:
  %_5 = alloca [16 x i8], align 8
  %_3 = alloca [48 x i8], align 8
  br i1 %cond, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %1 = getelementptr inbounds nuw { ptr, i64 }, ptr %_5, i64 0
  store ptr @alloc_64e308ef4babfeb8b6220184de794a17, ptr %1, align 8
  %2 = getelementptr inbounds i8, ptr %1, i64 8
  store i64 221, ptr %2, align 8
  store ptr %_5, ptr %_3, align 8
  %3 = getelementptr inbounds i8, ptr %_3, i64 8
  store i64 1, ptr %3, align 8
  %4 = load ptr, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %5 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  %6 = getelementptr inbounds i8, ptr %_3, i64 32
  store ptr %4, ptr %6, align 8
  %7 = getelementptr inbounds i8, ptr %6, i64 8
  store i64 %5, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %_3, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 0, ptr %9, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h269f239d23f67cf5E(ptr align 8 %_3, i1 zeroext false, ptr align 8 %0) #22
  unreachable

bb2:                                              ; preds = %start
  ret void
}

; core::iter::traits::exact_size::ExactSizeIterator::len
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @_ZN4core4iter6traits10exact_size17ExactSizeIterator3len17h1d06ac11ea56894eE(ptr align 8 %self) unnamed_addr #0 {
start:
  %_9 = alloca [48 x i8], align 8
  %_6 = alloca [16 x i8], align 8
  %_3 = alloca [24 x i8], align 8
  %upper = alloca [16 x i8], align 8
; call <alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::size_hint
  call void @"_ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17ha3687b90d10d927bE"(ptr sret([24 x i8]) align 8 %_3, ptr align 8 %self)
  %lower = load i64, ptr %_3, align 8
  %0 = getelementptr inbounds i8, ptr %_3, i64 8
  %1 = load i64, ptr %0, align 8
  %2 = getelementptr inbounds i8, ptr %0, i64 8
  %3 = load i64, ptr %2, align 8
  store i64 %1, ptr %upper, align 8
  %4 = getelementptr inbounds i8, ptr %upper, i64 8
  store i64 %3, ptr %4, align 8
  %5 = getelementptr inbounds i8, ptr %_6, i64 8
  store i64 %lower, ptr %5, align 8
  store i64 1, ptr %_6, align 8
  %_10 = load i64, ptr %upper, align 8
  %6 = getelementptr inbounds i8, ptr %upper, i64 8
  %7 = load i64, ptr %6, align 8
  %8 = trunc nuw i64 %_10 to i1
  br i1 %8, label %bb6, label %bb5

bb6:                                              ; preds = %start
  %l = getelementptr inbounds i8, ptr %upper, i64 8
  %r = getelementptr inbounds i8, ptr %_6, i64 8
  %9 = getelementptr inbounds i8, ptr %upper, i64 8
  %_13 = load i64, ptr %9, align 8
  %_7 = icmp eq i64 %_13, %lower
  br i1 %_7, label %bb2, label %bb3

bb5:                                              ; preds = %start
  br label %bb3

bb3:                                              ; preds = %bb6, %bb5
  store ptr null, ptr %_9, align 8
; call core::panicking::assert_failed
  call void @_ZN4core9panicking13assert_failed17h61b46efbfcc8f986E(i8 0, ptr align 8 %upper, ptr align 8 %_6, ptr align 8 %_9, ptr align 8 @alloc_8f80298b638aa40322b9c81f1de6d623) #21
  unreachable

bb2:                                              ; preds = %bb6
  ret i64 %lower

bb4:                                              ; No predecessors!
  unreachable
}

; core::iter::traits::iterator::Iterator::collect
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core4iter6traits8iterator8Iterator7collect17h1241d4dc4bf0b811E(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_6 = alloca [32 x i8], align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_6, ptr align 8 %self, i64 32, i1 false)
; invoke <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
  invoke void @"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h537f4e5d452b9dddE"(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %_6)
          to label %bb1 unwind label %cleanup

bb4:                                              ; preds = %cleanup
  br label %bb2

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  store ptr %2, ptr %0, align 8
  %4 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %3, ptr %4, align 8
  br label %bb4

bb1:                                              ; preds = %start
  ret void

bb2:                                              ; preds = %bb3, %bb4
  %5 = load ptr, ptr %0, align 8
  %6 = getelementptr inbounds i8, ptr %0, i64 8
  %7 = load i32, ptr %6, align 8
  %8 = insertvalue { ptr, i32 } poison, ptr %5, 0
  %9 = insertvalue { ptr, i32 } %8, i32 %7, 1
  resume { ptr, i32 } %9

bb3:                                              ; No predecessors!
; invoke core::ptr::drop_in_place<std::env::Args>
  invoke void @"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd8d2745dbc46aebaE"(ptr align 8 %self) #18
          to label %bb2 unwind label %terminate

terminate:                                        ; preds = %bb3
  %10 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() #20
  unreachable
}

; core::alloc::layout::Layout::from_size_align_unchecked::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17h119c3a7b70072a80E(i64 %size, i64 %align, ptr align 8 %0) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %_7 = alloca [16 x i8], align 8
  %_5 = alloca [48 x i8], align 8
; invoke core::alloc::layout::Layout::is_size_align_valid
  %_3 = invoke zeroext i1 @_ZN4core5alloc6layout6Layout19is_size_align_valid17h3abcb2ab3c4240feE(i64 %size, i64 %align)
          to label %bb1 unwind label %terminate

terminate:                                        ; preds = %start
  %1 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17h309187e32d89ba06E() #20
  unreachable

bb1:                                              ; preds = %start
  br i1 %_3, label %bb2, label %bb3

bb3:                                              ; preds = %bb1
  %2 = getelementptr inbounds nuw { ptr, i64 }, ptr %_7, i64 0
  store ptr @alloc_1be5ea12ba708d9a11b6e93a7d387a75, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %2, i64 8
  store i64 281, ptr %3, align 8
  store ptr %_7, ptr %_5, align 8
  %4 = getelementptr inbounds i8, ptr %_5, i64 8
  store i64 1, ptr %4, align 8
  %5 = load ptr, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %6 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  %7 = getelementptr inbounds i8, ptr %_5, i64 32
  store ptr %5, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %7, i64 8
  store i64 %6, ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %_5, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %9, i64 8
  store i64 0, ptr %10, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h269f239d23f67cf5E(ptr align 8 %_5, i1 zeroext false, ptr align 8 %0) #22
  unreachable

bb2:                                              ; preds = %bb1
  ret void
}

; core::slice::raw::from_raw_parts::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @_ZN4core5slice3raw14from_raw_parts18precondition_check17hb9f39a9d479f1cf1E(ptr %data, i64 %size, i64 %align, i64 %len, ptr align 8 %0) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %1 = alloca [4 x i8], align 4
  %max_len = alloca [8 x i8], align 8
  %_14 = alloca [48 x i8], align 8
  %_11 = alloca [16 x i8], align 8
  %_9 = alloca [48 x i8], align 8
  %2 = call i64 @llvm.ctpop.i64(i64 %align)
  %3 = trunc i64 %2 to i32
  store i32 %3, ptr %1, align 4
  %_18 = load i32, ptr %1, align 4
  %4 = icmp eq i32 %_18, 1
  br i1 %4, label %bb8, label %bb9

bb8:                                              ; preds = %start
  %_16 = ptrtoint ptr %data to i64
  %_17 = sub i64 %align, 1
  %_15 = and i64 %_16, %_17
  %5 = icmp eq i64 %_15, 0
  br i1 %5, label %bb6, label %bb7

bb9:                                              ; preds = %start
  store ptr @alloc_e92e94d0ff530782b571cfd99ec66aef, ptr %_14, align 8
  %6 = getelementptr inbounds i8, ptr %_14, i64 8
  store i64 1, ptr %6, align 8
  %7 = load ptr, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %8 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  %9 = getelementptr inbounds i8, ptr %_14, i64 32
  store ptr %7, ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %9, i64 8
  store i64 %8, ptr %10, align 8
  %11 = getelementptr inbounds i8, ptr %_14, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %11, align 8
  %12 = getelementptr inbounds i8, ptr %11, i64 8
  store i64 0, ptr %12, align 8
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17h0d47791d04b10c1eE(ptr align 8 %_14, ptr align 8 @alloc_d35c6aec622884a82e66ab090a94939a) #21
          to label %unreachable unwind label %terminate

bb6:                                              ; preds = %bb8
  %_12 = icmp eq i64 %_16, 0
  %_5 = xor i1 %_12, true
  br i1 %_5, label %bb1, label %bb4

bb7:                                              ; preds = %bb8
  br label %bb4

bb4:                                              ; preds = %bb7, %bb6
  br label %bb5

bb1:                                              ; preds = %bb6
  %_22 = icmp eq i64 %size, 0
  %13 = icmp eq i64 %size, 0
  br i1 %13, label %bb11, label %bb12

bb11:                                             ; preds = %bb1
  store i64 -1, ptr %max_len, align 8
  br label %bb14

bb12:                                             ; preds = %bb1
  br i1 %_22, label %panic, label %bb13

bb14:                                             ; preds = %bb13, %bb11
  %14 = load i64, ptr %max_len, align 8
  %_7 = icmp ule i64 %len, %14
  br i1 %_7, label %bb2, label %bb3

bb13:                                             ; preds = %bb12
  %15 = udiv i64 9223372036854775807, %size
  store i64 %15, ptr %max_len, align 8
  br label %bb14

panic:                                            ; preds = %bb12
; invoke core::panicking::panic_const::panic_const_div_by_zero
  invoke void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h5f35521000e5b02bE(ptr align 8 @alloc_5475fa92f2f69e75e8dff3e87ac79e4f) #21
          to label %unreachable unwind label %terminate

terminate:                                        ; preds = %bb9, %panic
  %16 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17h309187e32d89ba06E() #20
  unreachable

unreachable:                                      ; preds = %bb9, %panic
  unreachable

bb3:                                              ; preds = %bb14
  br label %bb5

bb2:                                              ; preds = %bb14
  ret void

bb5:                                              ; preds = %bb4, %bb3
  %17 = getelementptr inbounds nuw { ptr, i64 }, ptr %_11, i64 0
  store ptr @alloc_a28e8c8fd5088943a8b5d44af697ff83, ptr %17, align 8
  %18 = getelementptr inbounds i8, ptr %17, i64 8
  store i64 279, ptr %18, align 8
  store ptr %_11, ptr %_9, align 8
  %19 = getelementptr inbounds i8, ptr %_9, i64 8
  store i64 1, ptr %19, align 8
  %20 = load ptr, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %21 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  %22 = getelementptr inbounds i8, ptr %_9, i64 32
  store ptr %20, ptr %22, align 8
  %23 = getelementptr inbounds i8, ptr %22, i64 8
  store i64 %21, ptr %23, align 8
  %24 = getelementptr inbounds i8, ptr %_9, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %24, align 8
  %25 = getelementptr inbounds i8, ptr %24, i64 8
  store i64 0, ptr %25, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h269f239d23f67cf5E(ptr align 8 %_9, i1 zeroext false, ptr align 8 %0) #22
  unreachable
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h42e0f9b21e8fc2b1E"() unnamed_addr #0 {
start:
  ret i8 0
}

; alloc::vec::Vec<T,A>::extend_desugared
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17h202857389a05496cE"(ptr align 8 %self, ptr align 8 %iterator) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [8 x i8], align 8
  %1 = alloca [16 x i8], align 8
  %_27 = alloca [8 x i8], align 8
  %src = alloca [24 x i8], align 8
  %_11 = alloca [24 x i8], align 8
  %_9 = alloca [8 x i8], align 8
  %element = alloca [24 x i8], align 8
  %_3 = alloca [24 x i8], align 8
  br label %bb1

bb1:                                              ; preds = %bb20, %start
; invoke <std::env::Args as core::iter::traits::iterator::Iterator>::next
  invoke void @"_ZN73_$LT$std..env..Args$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h28703ed5d9e04e32E"(ptr sret([24 x i8]) align 8 %_3, ptr align 8 %iterator)
          to label %bb2 unwind label %cleanup

bb11:                                             ; preds = %bb13, %cleanup
; invoke core::ptr::drop_in_place<std::env::Args>
  invoke void @"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd8d2745dbc46aebaE"(ptr align 8 %iterator) #18
          to label %bb12 unwind label %terminate

cleanup:                                          ; preds = %bb1
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = extractvalue { ptr, i32 } %2, 0
  %4 = extractvalue { ptr, i32 } %2, 1
  store ptr %3, ptr %1, align 8
  %5 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %4, ptr %5, align 8
  br label %bb11

bb2:                                              ; preds = %bb1
  %6 = load i64, ptr %_3, align 8
  %7 = icmp eq i64 %6, -9223372036854775808
  %_5 = select i1 %7, i64 0, i64 1
  %8 = trunc nuw i64 %_5 to i1
  br i1 %8, label %bb3, label %bb9

bb3:                                              ; preds = %bb2
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %element, ptr align 8 %_3, i64 24, i1 false)
  %9 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %9, align 8
  %_19 = icmp ule i64 %len, 384307168202282325
  br label %bb16

bb9:                                              ; preds = %bb2
; call core::ptr::drop_in_place<std::env::Args>
  call void @"_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17hd8d2745dbc46aebaE"(ptr align 8 %iterator)
  ret void

bb16:                                             ; preds = %bb3
  %self1 = load i64, ptr %self, align 8
  store i64 %self1, ptr %_9, align 8
  br label %bb14

bb15:                                             ; No predecessors!
  store i64 -1, ptr %_9, align 8
  br label %bb14

bb14:                                             ; preds = %bb16, %bb15
  %10 = load i64, ptr %_9, align 8
  %_8 = icmp eq i64 %len, %10
  br i1 %_8, label %bb4, label %bb7

bb7:                                              ; preds = %bb14
  br label %bb8

bb4:                                              ; preds = %bb14
; invoke <std::env::Args as core::iter::traits::iterator::Iterator>::size_hint
  invoke void @"_ZN73_$LT$std..env..Args$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17hca4464b764a5cf8eE"(ptr sret([24 x i8]) align 8 %_11, ptr align 8 %iterator)
          to label %bb5 unwind label %cleanup2

bb8:                                              ; preds = %bb6, %bb7
  %11 = getelementptr inbounds i8, ptr %self, i64 8
  %_24 = load ptr, ptr %11, align 8
  %dst = getelementptr inbounds nuw %"alloc::string::String", ptr %_24, i64 %len
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %src, ptr align 8 %element, i64 24, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %dst, ptr align 8 %src, i64 24, i1 false)
  %new_len = add i64 %len, 1
  br label %bb18

bb13:                                             ; preds = %cleanup2
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hf12fa1af090a7d2dE"(ptr align 8 %element) #18
          to label %bb11 unwind label %terminate

cleanup2:                                         ; preds = %bb5, %bb4
  %12 = landingpad { ptr, i32 }
          cleanup
  %13 = extractvalue { ptr, i32 } %12, 0
  %14 = extractvalue { ptr, i32 } %12, 1
  store ptr %13, ptr %1, align 8
  %15 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %14, ptr %15, align 8
  br label %bb13

bb5:                                              ; preds = %bb4
  %lower = load i64, ptr %_11, align 8
  %16 = call i64 @llvm.uadd.sat.i64(i64 %lower, i64 1)
  store i64 %16, ptr %0, align 8
  %_14 = load i64, ptr %0, align 8
; invoke alloc::vec::Vec<T,A>::reserve
  invoke void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17hda58e18fa806afdbE"(ptr align 8 %self, i64 %_14)
          to label %bb6 unwind label %cleanup2

bb6:                                              ; preds = %bb5
  br label %bb8

bb18:                                             ; preds = %bb8
  br label %bb23

bb23:                                             ; preds = %bb18
  %self3 = load i64, ptr %self, align 8
  store i64 %self3, ptr %_27, align 8
  br label %bb21

bb22:                                             ; No predecessors!
  store i64 -1, ptr %_27, align 8
  br label %bb21

bb21:                                             ; preds = %bb23, %bb22
  %17 = load i64, ptr %_27, align 8
; call alloc::vec::Vec<T,A>::set_len::precondition_check
  call void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7set_len18precondition_check17h4038c2036fef2dbeE"(i64 %new_len, i64 %17, ptr align 8 @alloc_cfb6177d9d971911008c59a6c6f48657) #19
  br label %bb20

bb20:                                             ; preds = %bb21
  %18 = getelementptr inbounds i8, ptr %self, i64 16
  store i64 %new_len, ptr %18, align 8
  br label %bb1

terminate:                                        ; preds = %bb11, %bb13
  %19 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() #20
  unreachable

bb24:                                             ; No predecessors!
  unreachable

bb12:                                             ; preds = %bb11
  %20 = load ptr, ptr %1, align 8
  %21 = getelementptr inbounds i8, ptr %1, i64 8
  %22 = load i32, ptr %21, align 8
  %23 = insertvalue { ptr, i32 } poison, ptr %20, 0
  %24 = insertvalue { ptr, i32 } %23, i32 %22, 1
  resume { ptr, i32 } %24
}

; alloc::vec::Vec<T,A>::len
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3len17he7e60de7c339d2d1E"(ptr align 8 %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 16
  %_0 = load i64, ptr %0, align 8
  %_2 = icmp ule i64 %_0, 384307168202282325
  ret i64 %_0
}

; alloc::vec::Vec<T,A>::reserve
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17hda58e18fa806afdbE"(ptr align 8 %self, i64 %additional) unnamed_addr #1 {
start:
  %self1 = alloca [8 x i8], align 8
  %elem_layout = alloca [16 x i8], align 8
  %0 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %0, align 8
  store i64 8, ptr %elem_layout, align 8
  %1 = getelementptr inbounds i8, ptr %elem_layout, i64 8
  store i64 24, ptr %1, align 8
  br label %bb6

bb6:                                              ; preds = %start
  %self2 = load i64, ptr %self, align 8
  store i64 %self2, ptr %self1, align 8
  br label %bb4

bb5:                                              ; No predecessors!
  store i64 -1, ptr %self1, align 8
  br label %bb4

bb4:                                              ; preds = %bb6, %bb5
  %2 = load i64, ptr %self1, align 8
  %_10 = sub i64 %2, %len
  %_7 = icmp ugt i64 %additional, %_10
  br i1 %_7, label %bb1, label %bb2

bb2:                                              ; preds = %bb4
  br label %bb3

bb1:                                              ; preds = %bb4
; call alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle
  call void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h2531361a4c3d470dE"(ptr align 8 %self, i64 %len, i64 %additional, i64 8, i64 24)
  br label %bb3

bb3:                                              ; preds = %bb1, %bb2
  ret void
}

; alloc::vec::Vec<T,A>::set_len::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7set_len18precondition_check17h4038c2036fef2dbeE"(i64 %new_len, i64 %capacity, ptr align 8 %0) unnamed_addr #4 {
start:
  %_7 = alloca [16 x i8], align 8
  %_5 = alloca [48 x i8], align 8
  %_3 = icmp ule i64 %new_len, %capacity
  br i1 %_3, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %1 = getelementptr inbounds nuw { ptr, i64 }, ptr %_7, i64 0
  store ptr @alloc_57d70e9d94c65ecfc15225d29a5ed72b, ptr %1, align 8
  %2 = getelementptr inbounds i8, ptr %1, i64 8
  store i64 198, ptr %2, align 8
  store ptr %_7, ptr %_5, align 8
  %3 = getelementptr inbounds i8, ptr %_5, i64 8
  store i64 1, ptr %3, align 8
  %4 = load ptr, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %5 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  %6 = getelementptr inbounds i8, ptr %_5, i64 32
  store ptr %4, ptr %6, align 8
  %7 = getelementptr inbounds i8, ptr %6, i64 8
  store i64 %5, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %_5, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 0, ptr %9, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h269f239d23f67cf5E(ptr align 8 %_5, i1 zeroext false, ptr align 8 %0) #22
  unreachable

bb1:                                              ; preds = %start
  ret void
}

; alloc::alloc::exchange_malloc
; Function Attrs: inlinehint nonlazybind uwtable
define internal ptr @_ZN5alloc5alloc15exchange_malloc17h3562346063b497feE(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %_4 = alloca [16 x i8], align 8
  br label %bb4

bb4:                                              ; preds = %start
; call core::alloc::layout::Layout::from_size_align_unchecked::precondition_check
  call void @_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17h119c3a7b70072a80E(i64 %size, i64 %align, ptr align 8 @alloc_54356c3bdf01fcc9e71537cdcbc33887) #19
  br label %bb5

bb5:                                              ; preds = %bb4
; call alloc::alloc::Global::alloc_impl
  %0 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h918950dfdd85ffb7E(ptr align 1 inttoptr (i64 1 to ptr), i64 %align, i64 %size, i1 zeroext false)
  %1 = extractvalue { ptr, i64 } %0, 0
  %2 = extractvalue { ptr, i64 } %0, 1
  store ptr %1, ptr %_4, align 8
  %3 = getelementptr inbounds i8, ptr %_4, i64 8
  store i64 %2, ptr %3, align 8
  %4 = load ptr, ptr %_4, align 8
  %5 = getelementptr inbounds i8, ptr %_4, i64 8
  %6 = load i64, ptr %5, align 8
  %7 = ptrtoint ptr %4 to i64
  %8 = icmp eq i64 %7, 0
  %_5 = select i1 %8, i64 1, i64 0
  %9 = trunc nuw i64 %_5 to i1
  br i1 %9, label %bb2, label %bb3

bb2:                                              ; preds = %bb5
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17h0dd18b38b699425cE(i64 %align, i64 %size) #21
  unreachable

bb3:                                              ; preds = %bb5
  %ptr.0 = load ptr, ptr %_4, align 8
  %10 = getelementptr inbounds i8, ptr %_4, i64 8
  %ptr.1 = load i64, ptr %10, align 8
  ret ptr %ptr.0

bb1:                                              ; No predecessors!
  unreachable
}

; alloc::alloc::Global::alloc_impl
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h918950dfdd85ffb7E(ptr align 1 %self, i64 %0, i64 %1, i1 zeroext %zeroed) unnamed_addr #0 {
start:
  %self4 = alloca [8 x i8], align 8
  %self3 = alloca [8 x i8], align 8
  %_12 = alloca [8 x i8], align 8
  %layout2 = alloca [16 x i8], align 8
  %layout1 = alloca [16 x i8], align 8
  %raw_ptr = alloca [8 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  store i64 %0, ptr %layout, align 8
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  %size = load i64, ptr %3, align 8
  %4 = icmp eq i64 %size, 0
  br i1 %4, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %_18 = load i64, ptr %layout, align 8
  %_19 = getelementptr i8, ptr null, i64 %_18
  %data = getelementptr i8, ptr null, i64 %_18
  br label %bb7

bb1:                                              ; preds = %start
  br i1 %zeroed, label %bb3, label %bb4

bb7:                                              ; preds = %bb2
  %_23 = getelementptr i8, ptr null, i64 %_18
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hdcc24cf9de78f312E"(ptr %_23, ptr align 8 @alloc_bec1f9f021c110596a13fd563f15177c) #19
  br label %bb9

bb9:                                              ; preds = %bb7
  store ptr %data, ptr %_0, align 8
  %5 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 0, ptr %5, align 8
  br label %bb6

bb6:                                              ; preds = %bb21, %bb14, %bb9
  %6 = load ptr, ptr %_0, align 8
  %7 = getelementptr inbounds i8, ptr %_0, i64 8
  %8 = load i64, ptr %7, align 8
  %9 = insertvalue { ptr, i64 } poison, ptr %6, 0
  %10 = insertvalue { ptr, i64 } %9, i64 %8, 1
  ret { ptr, i64 } %10

bb4:                                              ; preds = %bb1
  %11 = load i64, ptr %layout, align 8
  %12 = getelementptr inbounds i8, ptr %layout, i64 8
  %13 = load i64, ptr %12, align 8
  store i64 %11, ptr %layout2, align 8
  %14 = getelementptr inbounds i8, ptr %layout2, i64 8
  store i64 %13, ptr %14, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsiQDbhR5a9UD_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #19
  %_35 = load i64, ptr %layout, align 8
; call __rustc::__rust_alloc
  %15 = call ptr @_RNvCsiQDbhR5a9UD_7___rustc12___rust_alloc(i64 %size, i64 %_35) #19
  store ptr %15, ptr %raw_ptr, align 8
  br label %bb5

bb3:                                              ; preds = %bb1
  %16 = load i64, ptr %layout, align 8
  %17 = getelementptr inbounds i8, ptr %layout, i64 8
  %18 = load i64, ptr %17, align 8
  store i64 %16, ptr %layout1, align 8
  %19 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %18, ptr %19, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsiQDbhR5a9UD_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #19
  %_30 = load i64, ptr %layout, align 8
; call __rustc::__rust_alloc_zeroed
  %20 = call ptr @_RNvCsiQDbhR5a9UD_7___rustc19___rust_alloc_zeroed(i64 %size, i64 %_30) #19
  store ptr %20, ptr %raw_ptr, align 8
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  %_37 = load ptr, ptr %raw_ptr, align 8
  %21 = load ptr, ptr %raw_ptr, align 8
  %_38 = ptrtoint ptr %21 to i64
  %22 = icmp eq i64 %_38, 0
  br i1 %22, label %bb14, label %bb15

bb14:                                             ; preds = %bb5
  store ptr null, ptr %self4, align 8
  store ptr null, ptr %self3, align 8
  %23 = load ptr, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, align 8
  %24 = load i64, ptr getelementptr inbounds (i8, ptr @anon.d9cc4ab2fa0032d6aa5f7f68905737c8.0, i64 8), align 8
  store ptr %23, ptr %_0, align 8
  %25 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %24, ptr %25, align 8
  br label %bb6

bb15:                                             ; preds = %bb5
  br label %bb16

bb16:                                             ; preds = %bb15
  %_40 = load ptr, ptr %raw_ptr, align 8
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hdcc24cf9de78f312E"(ptr %_40, ptr align 8 @alloc_0944926afe5312fa319f558031c55b23) #19
  br label %bb18

bb18:                                             ; preds = %bb16
  store ptr %_37, ptr %self4, align 8
  %v = load ptr, ptr %self4, align 8
  store ptr %v, ptr %self3, align 8
  %v5 = load ptr, ptr %self3, align 8
  store ptr %v5, ptr %_12, align 8
  %ptr = load ptr, ptr %_12, align 8
  br label %bb19

bb19:                                             ; preds = %bb18
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hdcc24cf9de78f312E"(ptr %ptr, ptr align 8 @alloc_bec1f9f021c110596a13fd563f15177c) #19
  br label %bb21

bb21:                                             ; preds = %bb19
  store ptr %ptr, ptr %_0, align 8
  %26 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %size, ptr %26, align 8
  br label %bb6
}

; alloc::string::String::as_str
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @_ZN5alloc6string6String6as_str17h6b8505dbd1cf610bE(ptr align 8 %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %_6 = load ptr, ptr %0, align 8
  %1 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %1, align 8
  br label %bb1

bb1:                                              ; preds = %start
; call core::slice::raw::from_raw_parts::precondition_check
  call void @_ZN4core5slice3raw14from_raw_parts18precondition_check17hb9f39a9d479f1cf1E(ptr %_6, i64 1, i64 1, i64 %len, ptr align 8 @alloc_e2199c9e7f384be1c4af111238e1c571) #19
  br label %bb3

bb3:                                              ; preds = %bb1
  %2 = insertvalue { ptr, i64 } poison, ptr %_6, 0
  %3 = insertvalue { ptr, i64 } %2, i64 %len, 1
  ret { ptr, i64 } %3
}

; alloc::raw_vec::RawVecInner<A>::with_capacity_in
; Function Attrs: inlinehint nonlazybind uwtable
define internal { i64, ptr } @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17ha96019c9826ab6b4E"(i64 %capacity, i64 %elem_layout.0, i64 %elem_layout.1) unnamed_addr #0 {
start:
  %self = alloca [8 x i8], align 8
  %elem_layout = alloca [16 x i8], align 8
  %this = alloca [16 x i8], align 8
  %_4 = alloca [24 x i8], align 8
; call alloc::raw_vec::RawVecInner<A>::try_allocate_in
  call void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h3a43f382272280f9E"(ptr sret([24 x i8]) align 8 %_4, i64 %capacity, i1 zeroext false, i64 %elem_layout.0, i64 %elem_layout.1)
  %_5 = load i64, ptr %_4, align 8
  %0 = trunc nuw i64 %_5 to i1
  br i1 %0, label %bb3, label %bb4

bb3:                                              ; preds = %start
  %1 = getelementptr inbounds i8, ptr %_4, i64 8
  %err.0 = load i64, ptr %1, align 8
  %2 = getelementptr inbounds i8, ptr %1, i64 8
  %err.1 = load i64, ptr %2, align 8
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17h8bb563fcfd5b89c4E(i64 %err.0, i64 %err.1) #21
  unreachable

bb4:                                              ; preds = %start
  %3 = getelementptr inbounds i8, ptr %_4, i64 8
  %4 = load i64, ptr %3, align 8
  %5 = getelementptr inbounds i8, ptr %3, i64 8
  %6 = load ptr, ptr %5, align 8
  store i64 %4, ptr %this, align 8
  %7 = getelementptr inbounds i8, ptr %this, i64 8
  store ptr %6, ptr %7, align 8
  store i64 %elem_layout.0, ptr %elem_layout, align 8
  %8 = getelementptr inbounds i8, ptr %elem_layout, i64 8
  store i64 %elem_layout.1, ptr %8, align 8
  %9 = icmp eq i64 %elem_layout.1, 0
  br i1 %9, label %bb6, label %bb7

bb6:                                              ; preds = %bb4
  store i64 -1, ptr %self, align 8
  br label %bb5

bb7:                                              ; preds = %bb4
  %self1 = load i64, ptr %this, align 8
  store i64 %self1, ptr %self, align 8
  br label %bb5

bb5:                                              ; preds = %bb7, %bb6
  %10 = load i64, ptr %self, align 8
  %_13 = sub i64 %10, 0
  %_8 = icmp ugt i64 %capacity, %_13
  %cond = xor i1 %_8, true
  br label %bb8

bb8:                                              ; preds = %bb5
; call core::hint::assert_unchecked::precondition_check
  call void @_ZN4core4hint16assert_unchecked18precondition_check17haedab59b76512c9dE(i1 zeroext %cond, ptr align 8 @alloc_d09f624e675dc1990a785862f9024a3e) #19
  br label %bb9

bb9:                                              ; preds = %bb8
  %_0.0 = load i64, ptr %this, align 8
  %11 = getelementptr inbounds i8, ptr %this, i64 8
  %_0.1 = load ptr, ptr %11, align 8
  %12 = insertvalue { i64, ptr } poison, i64 %_0.0, 0
  %13 = insertvalue { i64, ptr } %12, ptr %_0.1, 1
  ret { i64, ptr } %13

bb2:                                              ; No predecessors!
  unreachable
}

; <I as core::iter::traits::collect::IntoIterator>::into_iter
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h31d60d93fd9fd0c8E"(ptr sret([32 x i8]) align 8 %_0, ptr align 8 %self) unnamed_addr #0 {
start:
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %self, i64 32, i1 false)
  ret void
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h9a22138e3eccb850E"(ptr align 1 %self, ptr %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %layout1 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  store i64 %0, ptr %layout, align 8
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  %_4 = load i64, ptr %3, align 8
  %4 = icmp eq i64 %_4, 0
  br i1 %4, label %bb2, label %bb1

bb2:                                              ; preds = %bb1, %start
  ret void

bb1:                                              ; preds = %start
  %5 = load i64, ptr %layout, align 8
  %6 = getelementptr inbounds i8, ptr %layout, i64 8
  %7 = load i64, ptr %6, align 8
  store i64 %5, ptr %layout1, align 8
  %8 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %7, ptr %8, align 8
  %_11 = load i64, ptr %layout, align 8
; call __rustc::__rust_dealloc
  call void @_RNvCsiQDbhR5a9UD_7___rustc14___rust_dealloc(ptr %ptr, i64 %_4, i64 %_11) #19
  br label %bb2
}

; <alloc::string::String as core::ops::deref::Deref>::deref
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @"_ZN65_$LT$alloc..string..String$u20$as$u20$core..ops..deref..Deref$GT$5deref17h773407254690b17eE"(ptr align 8 %self) unnamed_addr #0 {
start:
; call alloc::string::String::as_str
  %0 = call { ptr, i64 } @_ZN5alloc6string6String6as_str17h6b8505dbd1cf610bE(ptr align 8 %self)
  %_0.0 = extractvalue { ptr, i64 } %0, 0
  %_0.1 = extractvalue { ptr, i64 } %0, 1
  %1 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %_0.1, 1
  ret { ptr, i64 } %2
}

; <visitor::SpeakBetterDogs as visitor::AnimalVisitor>::receive_cat
; Function Attrs: nonlazybind uwtable
define internal i64 @"_ZN67_$LT$visitor..SpeakBetterDogs$u20$as$u20$visitor..AnimalVisitor$GT$11receive_cat17h775be00c4adb485aE"(ptr align 1 %self, ptr align 1 %a.0, ptr align 8 %a.1) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds i8, ptr %a.1, i64 24
  %1 = load ptr, ptr %0, align 8, !invariant.load !5, !nonnull !5
  %_0 = call i64 %1(ptr align 1 %a.0)
  ret i64 %_0
}

; <visitor::SpeakBetterDogs as visitor::AnimalVisitor>::receive_dog
; Function Attrs: nonlazybind uwtable
define internal i64 @"_ZN67_$LT$visitor..SpeakBetterDogs$u20$as$u20$visitor..AnimalVisitor$GT$11receive_dog17h9724d8b1f3575287E"(ptr align 1 %self, ptr align 1 %_a.0, ptr align 8 %_a.1) unnamed_addr #1 {
start:
  ret i64 44444
}

; <core::num::error::ParseIntError as core::fmt::Debug>::fmt
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @"_ZN68_$LT$core..num..error..ParseIntError$u20$as$u20$core..fmt..Debug$GT$3fmt17hbd1afecc70c04c9fE"(ptr align 1 %self, ptr align 8 %f) unnamed_addr #0 {
start:
  %_5 = alloca [8 x i8], align 8
  store ptr %self, ptr %_5, align 8
; call core::fmt::Formatter::debug_struct_field1_finish
  %_0 = call zeroext i1 @_ZN4core3fmt9Formatter26debug_struct_field1_finish17h93d2a3c38589b30aE(ptr align 8 %f, ptr align 1 @alloc_f62df14955f7d78bca139b0a7668683d, i64 13, ptr align 1 @alloc_a5d866b1768ad3f826bccdb004a1a8ae, i64 4, ptr align 1 %_5, ptr align 8 @vtable.4)
  ret i1 %_0
}

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h564b856a31d78babE"(ptr align 8 %self) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %_5 = load ptr, ptr %0, align 8
  %1 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %1, align 8
; call core::ptr::drop_in_place<[alloc::string::String]>
  call void @"_ZN4core3ptr52drop_in_place$LT$$u5b$alloc..string..String$u5d$$GT$17h4bd412caa8a8841aE"(ptr align 8 %_5, i64 %len)
  ret void
}

; <alloc::boxed::Box<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9d12390e067757c5E"(ptr align 8 %self) unnamed_addr #0 {
start:
  %0 = alloca [8 x i8], align 8
  %1 = alloca [8 x i8], align 8
  %layout = alloca [16 x i8], align 8
  %ptr.0 = load ptr, ptr %self, align 8
  %2 = getelementptr inbounds i8, ptr %self, i64 8
  %ptr.1 = load ptr, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %ptr.1, i64 8
  %4 = load i64, ptr %3, align 8, !invariant.load !5
  %5 = getelementptr inbounds i8, ptr %ptr.1, i64 16
  %6 = load i64, ptr %5, align 8, !invariant.load !5
  store i64 %4, ptr %1, align 8
  %size = load i64, ptr %1, align 8
  %7 = getelementptr inbounds i8, ptr %ptr.1, i64 8
  %8 = load i64, ptr %7, align 8, !invariant.load !5
  %9 = getelementptr inbounds i8, ptr %ptr.1, i64 16
  %10 = load i64, ptr %9, align 8, !invariant.load !5
  store i64 %10, ptr %0, align 8
  %align = load i64, ptr %0, align 8
  br label %bb6

bb6:                                              ; preds = %start
; call core::alloc::layout::Layout::from_size_align_unchecked::precondition_check
  call void @_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17h119c3a7b70072a80E(i64 %size, i64 %align, ptr align 8 @alloc_c4ba26a389b5ded934a968bc0482ce95) #19
  br label %bb7

bb7:                                              ; preds = %bb6
  %11 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %size, ptr %11, align 8
  store i64 %align, ptr %layout, align 8
  %12 = icmp eq i64 %size, 0
  br i1 %12, label %bb3, label %bb1

bb3:                                              ; preds = %bb1, %bb7
  ret void

bb1:                                              ; preds = %bb7
  %_7 = getelementptr inbounds i8, ptr %self, i64 16
  %13 = load i64, ptr %layout, align 8
  %14 = getelementptr inbounds i8, ptr %layout, i64 8
  %15 = load i64, ptr %14, align 8
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h9a22138e3eccb850E"(ptr align 1 %_7, ptr %ptr.0, i64 %13, i64 %15)
  br label %bb3
}

; <std::env::Args as core::iter::traits::iterator::Iterator>::size_hint
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN73_$LT$std..env..Args$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17hca4464b764a5cf8eE"(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %self) unnamed_addr #0 {
start:
; call <alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::size_hint
  call void @"_ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17ha3687b90d10d927bE"(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %self)
  ret void
}

; <usize as core::slice::index::SliceIndex<[T]>>::index
; Function Attrs: inlinehint nonlazybind uwtable
define internal align 8 ptr @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17hcd43b733a08d739cE"(i64 %self, ptr align 8 %slice.0, i64 %slice.1, ptr align 8 %0) unnamed_addr #0 {
start:
  %_4 = icmp ult i64 %self, %slice.1
  br i1 %_4, label %bb1, label %panic

bb1:                                              ; preds = %start
  %_0 = getelementptr inbounds nuw %"alloc::string::String", ptr %slice.0, i64 %self
  ret ptr %_0

panic:                                            ; preds = %start
; call core::panicking::panic_bounds_check
  call void @_ZN4core9panicking18panic_bounds_check17h66bd68becca9aec3E(i64 %self, i64 %slice.1, ptr align 8 %0) #21
  unreachable
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h2f2c5eb1e1cec426E"(ptr align 8 %self) unnamed_addr #1 {
start:
; call alloc::raw_vec::RawVecInner<A>::deallocate
  call void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$10deallocate17h08c62d48ae68ea00E"(ptr align 8 %self, i64 8, i64 24)
  ret void
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h514f7f901ceb5ef5E"(ptr align 8 %self) unnamed_addr #1 {
start:
; call alloc::raw_vec::RawVecInner<A>::deallocate
  call void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$10deallocate17h08c62d48ae68ea00E"(ptr align 8 %self, i64 8, i64 24)
  ret void
}

; visitor::get_animal
; Function Attrs: nonlazybind uwtable
define internal { ptr, ptr } @_ZN7visitor10get_animal17h3f4745417c09507aE(i64 %num) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %1 = alloca [16 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %2 = icmp eq i64 %num, 0
  br i1 %2, label %bb1, label %bb3

bb1:                                              ; preds = %start
; invoke alloc::alloc::exchange_malloc
  %_4.i = invoke ptr @_ZN5alloc5alloc15exchange_malloc17h3562346063b497feE(i64 0, i64 1)
          to label %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h964f2179d9677cc5E.exit" unwind label %cleanup.i

cleanup.i:                                        ; preds = %bb1
  %3 = landingpad { ptr, i32 }
          cleanup
  %4 = extractvalue { ptr, i32 } %3, 0
  %5 = extractvalue { ptr, i32 } %3, 1
  store ptr %4, ptr %1, align 8
  %6 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %5, ptr %6, align 8
  %7 = load ptr, ptr %1, align 8
  %8 = getelementptr inbounds i8, ptr %1, i64 8
  %9 = load i32, ptr %8, align 8
  %10 = insertvalue { ptr, i32 } poison, ptr %7, 0
  %11 = insertvalue { ptr, i32 } %10, i32 %9, 1
  resume { ptr, i32 } %11

"_ZN5alloc5boxed12Box$LT$T$GT$3new17h964f2179d9677cc5E.exit": ; preds = %bb1
  store ptr %_4.i, ptr %_0, align 8
  %12 = getelementptr inbounds i8, ptr %_0, i64 8
  store ptr @vtable.1, ptr %12, align 8
  br label %bb5

bb3:                                              ; preds = %start
; invoke alloc::alloc::exchange_malloc
  %_4.i1 = invoke ptr @_ZN5alloc5alloc15exchange_malloc17h3562346063b497feE(i64 0, i64 1)
          to label %"_ZN5alloc5boxed12Box$LT$T$GT$3new17hf79260cc7aee2bdcE.exit" unwind label %cleanup.i2

cleanup.i2:                                       ; preds = %bb3
  %13 = landingpad { ptr, i32 }
          cleanup
  %14 = extractvalue { ptr, i32 } %13, 0
  %15 = extractvalue { ptr, i32 } %13, 1
  store ptr %14, ptr %0, align 8
  %16 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %15, ptr %16, align 8
  %17 = load ptr, ptr %0, align 8
  %18 = getelementptr inbounds i8, ptr %0, i64 8
  %19 = load i32, ptr %18, align 8
  %20 = insertvalue { ptr, i32 } poison, ptr %17, 0
  %21 = insertvalue { ptr, i32 } %20, i32 %19, 1
  resume { ptr, i32 } %21

"_ZN5alloc5boxed12Box$LT$T$GT$3new17hf79260cc7aee2bdcE.exit": ; preds = %bb3
  store ptr %_4.i1, ptr %_0, align 8
  %22 = getelementptr inbounds i8, ptr %_0, i64 8
  store ptr @vtable.2, ptr %22, align 8
  br label %bb5

bb5:                                              ; preds = %"_ZN5alloc5boxed12Box$LT$T$GT$3new17hf79260cc7aee2bdcE.exit", %"_ZN5alloc5boxed12Box$LT$T$GT$3new17h964f2179d9677cc5E.exit"
  %23 = load ptr, ptr %_0, align 8
  %24 = getelementptr inbounds i8, ptr %_0, i64 8
  %25 = load ptr, ptr %24, align 8
  %26 = insertvalue { ptr, ptr } poison, ptr %23, 0
  %27 = insertvalue { ptr, ptr } %26, ptr %25, 1
  ret { ptr, ptr } %27
}

; visitor::run_not_rw
; Function Attrs: nonlazybind uwtable
define internal i64 @_ZN7visitor10run_not_rw17h307de46600722450E(ptr align 1 %0, ptr align 8 %1, ptr align 1 %dc) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %2 = alloca [16 x i8], align 8
  %animal = alloca [16 x i8], align 8
  store ptr %0, ptr %animal, align 8
  %3 = getelementptr inbounds i8, ptr %animal, i64 8
  store ptr %1, ptr %3, align 8
  %_5.0 = load ptr, ptr %animal, align 8
  %4 = getelementptr inbounds i8, ptr %animal, i64 8
  %_5.1 = load ptr, ptr %4, align 8
  %5 = getelementptr inbounds i8, ptr %_5.1, i64 32
  %6 = load ptr, ptr %5, align 8, !invariant.load !5, !nonnull !5
  %_0 = invoke i64 %6(ptr align 1 %_5.0, ptr align 1 %dc, ptr align 8 @vtable.5)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::boxed::Box<dyn visitor::Animal>>
  invoke void @"_ZN4core3ptr69drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$visitor..Animal$GT$$GT$17hd6ede007c5530f41E"(ptr align 8 %animal) #18
          to label %bb4 unwind label %terminate

cleanup:                                          ; preds = %start
  %7 = landingpad { ptr, i32 }
          cleanup
  %8 = extractvalue { ptr, i32 } %7, 0
  %9 = extractvalue { ptr, i32 } %7, 1
  store ptr %8, ptr %2, align 8
  %10 = getelementptr inbounds i8, ptr %2, i64 8
  store i32 %9, ptr %10, align 8
  br label %bb3

bb1:                                              ; preds = %start
; call core::ptr::drop_in_place<alloc::boxed::Box<dyn visitor::Animal>>
  call void @"_ZN4core3ptr69drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$visitor..Animal$GT$$GT$17hd6ede007c5530f41E"(ptr align 8 %animal)
  ret i64 %_0

terminate:                                        ; preds = %bb3
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() #20
  unreachable

bb4:                                              ; preds = %bb3
  %12 = load ptr, ptr %2, align 8
  %13 = getelementptr inbounds i8, ptr %2, i64 8
  %14 = load i32, ptr %13, align 8
  %15 = insertvalue { ptr, i32 } poison, ptr %12, 0
  %16 = insertvalue { ptr, i32 } %15, i32 %14, 1
  resume { ptr, i32 } %16
}

; visitor::main
; Function Attrs: nonlazybind uwtable
define hidden void @_ZN7visitor4main17h129d384e47d754f8E() unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %e.i = alloca [1 x i8], align 1
  %1 = alloca [16 x i8], align 8
  %_10 = alloca [16 x i8], align 8
  %_6 = alloca [48 x i8], align 8
  %_2 = alloca [32 x i8], align 8
  %args = alloca [24 x i8], align 8
; call std::env::args
  call void @_ZN3std3env4args17h0e9e5574ec5b0f4eE(ptr sret([32 x i8]) align 8 %_2)
; call core::iter::traits::iterator::Iterator::collect
  call void @_ZN4core4iter6traits8iterator8Iterator7collect17h1241d4dc4bf0b811E(ptr sret([24 x i8]) align 8 %args, ptr align 8 %_2)
; invoke alloc::vec::Vec<T,A>::len
  %_3 = invoke i64 @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3len17he7e60de7c339d2d1E"(ptr align 8 %args)
          to label %bb3 unwind label %cleanup

bb14:                                             ; preds = %cleanup.body
; invoke core::ptr::drop_in_place<alloc::vec::Vec<alloc::string::String>>
  invoke void @"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hdafa2e272e277cefE"(ptr align 8 %args) #18
          to label %bb15 unwind label %terminate

cleanup:                                          ; preds = %bb11, %bb10, %bb8, %bb7, %bb4, %bb6, %bb5, %start
  %2 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body

cleanup.body:                                     ; preds = %cleanup.i, %cleanup
  %eh.lpad-body = phi { ptr, i32 } [ %2, %cleanup ], [ %20, %cleanup.i ]
  %3 = extractvalue { ptr, i32 } %eh.lpad-body, 0
  %4 = extractvalue { ptr, i32 } %eh.lpad-body, 1
  store ptr %3, ptr %1, align 8
  %5 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %4, ptr %5, align 8
  br label %bb14

bb3:                                              ; preds = %start
  %6 = icmp eq i64 %_3, 1
  br i1 %6, label %bb5, label %bb4

bb5:                                              ; preds = %bb3
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_const
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3eaa9ce0927cf188E"(ptr sret([48 x i8]) align 8 %_6, ptr align 8 @alloc_da03007fb5fe1aab10d9ffd81ef29605)
          to label %bb6 unwind label %cleanup

bb4:                                              ; preds = %bb3
; invoke <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
  %_12 = invoke align 8 ptr @"_ZN81_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..index..Index$LT$I$GT$$GT$5index17h1a257cc57aba1d49E"(ptr align 8 %args, i64 1, ptr align 8 @alloc_818da8dffaa466cf3fa20cda7bc167b2)
          to label %bb7 unwind label %cleanup

bb6:                                              ; preds = %bb5
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17hdbec2ce0c9bd28ecE(ptr align 8 %_6)
          to label %bb16 unwind label %cleanup

bb16:                                             ; preds = %bb6
  br label %bb12

bb12:                                             ; preds = %bb17, %bb16
; call core::ptr::drop_in_place<alloc::vec::Vec<alloc::string::String>>
  call void @"_ZN4core3ptr65drop_in_place$LT$alloc..vec..Vec$LT$alloc..string..String$GT$$GT$17hdafa2e272e277cefE"(ptr align 8 %args)
  ret void

bb7:                                              ; preds = %bb4
; invoke <alloc::string::String as core::ops::deref::Deref>::deref
  %7 = invoke { ptr, i64 } @"_ZN65_$LT$alloc..string..String$u20$as$u20$core..ops..deref..Deref$GT$5deref17h773407254690b17eE"(ptr align 8 %_12)
          to label %bb8 unwind label %cleanup

bb8:                                              ; preds = %bb7
  %_11.0 = extractvalue { ptr, i64 } %7, 0
  %_11.1 = extractvalue { ptr, i64 } %7, 1
; invoke core::str::<impl str>::parse
  invoke void @"_ZN4core3str21_$LT$impl$u20$str$GT$5parse17h5a5a3b1975ff4c07E"(ptr sret([16 x i8]) align 8 %_10, ptr align 1 %_11.0, i64 %_11.1)
          to label %bb9 unwind label %cleanup

bb9:                                              ; preds = %bb8
  %8 = load i8, ptr %_10, align 8
  %9 = trunc nuw i8 %8 to i1
  %_2.i = zext i1 %9 to i64
  br i1 %9, label %bb2.i, label %"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h27cb0c4e9651f440E.exit"

bb2.i:                                            ; preds = %bb9
  %10 = getelementptr inbounds i8, ptr %_10, i64 1
  %11 = load i8, ptr %10, align 1
  store i8 %11, ptr %e.i, align 1
; invoke core::result::unwrap_failed
  invoke void @_ZN4core6result13unwrap_failed17h379356178f9993a4E(ptr align 1 @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 43, ptr align 1 %e.i, ptr align 8 @vtable.3, ptr align 8 @alloc_da68476bc014f050336d4aef91e18025) #21
          to label %unreachable.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %bb2.i
  %12 = landingpad { ptr, i32 }
          cleanup
  %13 = extractvalue { ptr, i32 } %12, 0
  %14 = extractvalue { ptr, i32 } %12, 1
  store ptr %13, ptr %0, align 8
  %15 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %14, ptr %15, align 8
  %16 = load ptr, ptr %0, align 8
  %17 = getelementptr inbounds i8, ptr %0, i64 8
  %18 = load i32, ptr %17, align 8
  %19 = insertvalue { ptr, i32 } poison, ptr %16, 0
  %20 = insertvalue { ptr, i32 } %19, i32 %18, 1
  br label %cleanup.body

unreachable.i:                                    ; preds = %bb2.i
  unreachable

"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h27cb0c4e9651f440E.exit": ; preds = %bb9
  %21 = getelementptr inbounds i8, ptr %_10, i64 8
  %t.i = load i64, ptr %21, align 8
  br label %bb10

bb10:                                             ; preds = %"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h27cb0c4e9651f440E.exit"
; invoke visitor::get_animal
  %22 = invoke { ptr, ptr } @_ZN7visitor10get_animal17h3f4745417c09507aE(i64 %t.i)
          to label %bb11 unwind label %cleanup

bb11:                                             ; preds = %bb10
  %a.0 = extractvalue { ptr, ptr } %22, 0
  %a.1 = extractvalue { ptr, ptr } %22, 1
; invoke visitor::run_not_rw
  %_15 = invoke i64 @_ZN7visitor10run_not_rw17h307de46600722450E(ptr align 1 %a.0, ptr align 8 %a.1, ptr align 1 inttoptr (i64 1 to ptr))
          to label %bb17 unwind label %cleanup

bb17:                                             ; preds = %bb11
  br label %bb12

terminate:                                        ; preds = %bb14
  %23 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() #20
  unreachable

bb15:                                             ; preds = %bb14
  %24 = load ptr, ptr %1, align 8
  %25 = getelementptr inbounds i8, ptr %1, i64 8
  %26 = load i32, ptr %25, align 8
  %27 = insertvalue { ptr, i32 } poison, ptr %24, 0
  %28 = insertvalue { ptr, i32 } %27, i32 %26, 1
  resume { ptr, i32 } %28
}

; <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
; Function Attrs: inlinehint nonlazybind uwtable
define internal align 8 ptr @"_ZN81_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..index..Index$LT$I$GT$$GT$5index17h1a257cc57aba1d49E"(ptr align 8 %self, i64 %index, ptr align 8 %0) unnamed_addr #0 {
start:
  %1 = getelementptr inbounds i8, ptr %self, i64 8
  %_6 = load ptr, ptr %1, align 8
  %2 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %2, align 8
  br label %bb1

bb1:                                              ; preds = %start
; call core::slice::raw::from_raw_parts::precondition_check
  call void @_ZN4core5slice3raw14from_raw_parts18precondition_check17hb9f39a9d479f1cf1E(ptr %_6, i64 24, i64 8, i64 %len, ptr align 8 @alloc_e2199c9e7f384be1c4af111238e1c571) #19
  br label %bb3

bb3:                                              ; preds = %bb1
; call <usize as core::slice::index::SliceIndex<[T]>>::index
  %_0 = call align 8 ptr @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17hcd43b733a08d739cE"(i64 %index, ptr align 8 %_6, i64 %len, ptr align 8 %0)
  ret ptr %_0
}

; <alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8c8d0c6a3a56fbc3E"(ptr align 8 %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %self1 = alloca [8 x i8], align 8
  %guard = alloca [8 x i8], align 8
  store ptr %self, ptr %guard, align 8
  %_6 = load ptr, ptr %guard, align 8
  store ptr %_6, ptr %self1, align 8
  %1 = getelementptr inbounds i8, ptr %_6, i64 8
  %self2 = load ptr, ptr %1, align 8
; invoke core::iter::traits::exact_size::ExactSizeIterator::len
  %len = invoke i64 @_ZN4core4iter6traits10exact_size17ExactSizeIterator3len17h1d06ac11ea56894eE(ptr align 8 %_6)
          to label %bb5 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<std::ffi::os_str::OsString,alloc::alloc::Global>>
  invoke void @"_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17h7b55f9c91b02259eE"(ptr align 8 %guard) #18
          to label %bb4 unwind label %terminate

cleanup:                                          ; preds = %bb5, %start
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = extractvalue { ptr, i32 } %2, 0
  %4 = extractvalue { ptr, i32 } %2, 1
  store ptr %3, ptr %0, align 8
  %5 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %4, ptr %5, align 8
  br label %bb3

bb5:                                              ; preds = %start
; invoke core::ptr::drop_in_place<[std::ffi::os_str::OsString]>
  invoke void @"_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17h527ae053523823f1E"(ptr align 8 %self2, i64 %len)
          to label %bb1 unwind label %cleanup

bb1:                                              ; preds = %bb5
; call core::ptr::drop_in_place<<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<std::ffi::os_str::OsString,alloc::alloc::Global>>
  call void @"_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17h7b55f9c91b02259eE"(ptr align 8 %guard)
  ret void

terminate:                                        ; preds = %bb3
  %6 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() #20
  unreachable

bb4:                                              ; preds = %bb3
  %7 = load ptr, ptr %0, align 8
  %8 = getelementptr inbounds i8, ptr %0, i64 8
  %9 = load i32, ptr %8, align 8
  %10 = insertvalue { ptr, i32 } poison, ptr %7, 0
  %11 = insertvalue { ptr, i32 } %10, i32 %9, 1
  resume { ptr, i32 } %11
}

; <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h537f4e5d452b9dddE"(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %iter) unnamed_addr #0 {
start:
  %_2 = alloca [32 x i8], align 8
; call <I as core::iter::traits::collect::IntoIterator>::into_iter
  call void @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h31d60d93fd9fd0c8E"(ptr sret([32 x i8]) align 8 %_2, ptr align 8 %iter)
; call <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  call void @"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hf90764434b48ab9bE"(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %_2)
  ret void
}

; <alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17hffec8aa3fee714b6E"(ptr align 8 %self, ptr align 8 %iter) unnamed_addr #1 {
start:
; call alloc::vec::Vec<T,A>::extend_desugared
  call void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17h202857389a05496cE"(ptr align 8 %self, ptr align 8 %iter)
  ret void
}

; <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hf90764434b48ab9bE"(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %iterator) unnamed_addr #1 {
start:
; call <alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
  call void @"_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17hacbe05db63113b4dE"(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %iterator)
  ret void
}

; Function Attrs: nounwind nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #5

; <std::env::Args as core::iter::traits::iterator::Iterator>::next
; Function Attrs: nonlazybind uwtable
declare void @"_ZN73_$LT$std..env..Args$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h28703ed5d9e04e32E"(ptr sret([24 x i8]) align 8, ptr align 8) unnamed_addr #1

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #6

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.uadd.sat.i64(i64, i64) #7

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind nonlazybind optsize uwtable
declare void @_ZN4core9panicking16panic_in_cleanup17h12fbc16d19b4b24cE() unnamed_addr #8

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h0af9494e5b7ea246E(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #7

; core::num::from_ascii_radix_panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core3num22from_ascii_radix_panic17h97b498ae62be534eE(i32, ptr align 8) unnamed_addr #9

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
declare void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbf837dd3fa685310E"(ptr align 8) unnamed_addr #1

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
declare void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h52f54b56938d812bE"(ptr align 8) unnamed_addr #1

; core::panicking::panic_nounwind_fmt
; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable
declare void @_ZN4core9panicking18panic_nounwind_fmt17h269f239d23f67cf5E(ptr align 8, i1 zeroext, ptr align 8) unnamed_addr #10

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h5afda12f2d28188fE(ptr align 1, i64, ptr align 8) unnamed_addr #9

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking9panic_fmt17h0d47791d04b10c1eE(ptr align 8, ptr align 8) unnamed_addr #9

; core::panicking::assert_failed
; Function Attrs: cold minsize noinline noreturn nonlazybind optsize uwtable
declare void @_ZN4core9panicking13assert_failed17h61b46efbfcc8f986E(i8, ptr align 8, ptr align 8, ptr align 8, ptr align 8) unnamed_addr #11

; core::alloc::layout::Layout::is_size_align_valid
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core5alloc6layout6Layout19is_size_align_valid17h3abcb2ab3c4240feE(i64, i64) unnamed_addr #1

; core::panicking::panic_cannot_unwind
; Function Attrs: cold minsize noinline noreturn nounwind nonlazybind optsize uwtable
declare void @_ZN4core9panicking19panic_cannot_unwind17h309187e32d89ba06E() unnamed_addr #8

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctpop.i64(i64) #7

; core::panicking::panic_const::panic_const_div_by_zero
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h5f35521000e5b02bE(ptr align 8) unnamed_addr #9

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core6result13unwrap_failed17h379356178f9993a4E(ptr align 1, i64, ptr align 1, ptr align 8, ptr align 8) unnamed_addr #9

; alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle
; Function Attrs: cold nonlazybind uwtable
declare void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17h2531361a4c3d470dE"(ptr align 8, i64, i64, i64, i64) unnamed_addr #12

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn nonlazybind optsize uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17h0dd18b38b699425cE(i64, i64) unnamed_addr #13

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind nonlazybind uwtable
declare void @_RNvCsiQDbhR5a9UD_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #5

; __rustc::__rust_alloc
; Function Attrs: nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias ptr @_RNvCsiQDbhR5a9UD_7___rustc12___rust_alloc(i64, i64 allocalign) unnamed_addr #14

; __rustc::__rust_alloc_zeroed
; Function Attrs: nounwind nonlazybind allockind("alloc,zeroed,aligned") allocsize(0) uwtable
declare noalias ptr @_RNvCsiQDbhR5a9UD_7___rustc19___rust_alloc_zeroed(i64, i64 allocalign) unnamed_addr #15

; alloc::raw_vec::RawVecInner<A>::try_allocate_in
; Function Attrs: nonlazybind uwtable
declare void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h3a43f382272280f9E"(ptr sret([24 x i8]) align 8, i64, i1 zeroext, i64, i64) unnamed_addr #1

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn nonlazybind optsize uwtable
declare void @_ZN5alloc7raw_vec12handle_error17h8bb563fcfd5b89c4E(i64, i64) unnamed_addr #13

; __rustc::__rust_dealloc
; Function Attrs: nounwind nonlazybind allockind("free") uwtable
declare void @_RNvCsiQDbhR5a9UD_7___rustc14___rust_dealloc(ptr allocptr, i64, i64) unnamed_addr #16

; <&T as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h0d0d792182b233cdE"(ptr align 8, ptr align 8) unnamed_addr #1

; core::fmt::Formatter::debug_struct_field1_finish
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter26debug_struct_field1_finish17h93d2a3c38589b30aE(ptr align 8, ptr align 1, i64, ptr align 1, i64, ptr align 1, ptr align 8) unnamed_addr #1

; core::panicking::panic_bounds_check
; Function Attrs: cold minsize noinline noreturn nonlazybind optsize uwtable
declare void @_ZN4core9panicking18panic_bounds_check17h66bd68becca9aec3E(i64, i64, ptr align 8) unnamed_addr #11

; alloc::raw_vec::RawVecInner<A>::deallocate
; Function Attrs: nonlazybind uwtable
declare void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$10deallocate17h08c62d48ae68ea00E"(ptr align 8, i64, i64) unnamed_addr #1

; std::env::args
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std3env4args17h0e9e5574ec5b0f4eE(ptr sret([32 x i8]) align 8) unnamed_addr #1

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17hdbec2ce0c9bd28ecE(ptr align 8) unnamed_addr #1

; Function Attrs: nonlazybind
define i32 @main(i32 %0, ptr %1) unnamed_addr #17 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17hbfdf2433702e39d1E(ptr @_ZN7visitor4main17h129d384e47d754f8E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { cold nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #4 = { inlinehint nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #5 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #6 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #7 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #8 = { cold minsize noinline noreturn nounwind nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #9 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #10 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #11 = { cold minsize noinline noreturn nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #12 = { cold nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #13 = { cold minsize noreturn nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #14 = { nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsiQDbhR5a9UD_7___rustc19___rust_alloc_zeroed" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #15 = { nounwind nonlazybind allockind("alloc,zeroed,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #16 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #17 = { nonlazybind "target-cpu"="x86-64" }
attributes #18 = { cold }
attributes #19 = { nounwind }
attributes #20 = { cold noreturn nounwind }
attributes #21 = { noreturn }
attributes #22 = { noreturn nounwind }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{!"rustc version 1.92.0-nightly (c8905eaa6 2025-09-28)"}
!4 = !{i64 15024925181305076}
!5 = !{}
