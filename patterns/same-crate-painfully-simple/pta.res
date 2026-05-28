FuncId(0) - "same_crate_painfully_simple::main"
	FuncId(0)::local_6 (1) ==> { FuncId(0)::local_3 }
	FuncId(0)::local_13 (1) ==> { FuncId(0)::local_15.cast#0 }
	FuncId(0)::local_9 (2) ==> { FuncId(0)::local_2 FuncId(0)::local_3 }
	FuncId(0)::local_17 (1) ==> { FuncId(0)::local_8 }
	FuncId(0)::local_14 (1) ==> { FuncId(0)::local_15 }
	FuncId(0)::local_15.index.1 (1) ==> { FuncId(8).cast#1 }
	FuncId(0)::local_4 (2) ==> { FuncId(0)::local_3 FuncId(0)::local_2 }
	FuncId(0)::local_18 (1) ==> { StrRefArr }
	FuncId(0)::local_5 (1) ==> { FuncId(0)::local_2 }
	FuncId(0)::local_11.0 (1) ==> { StrRefArr.cast#3 }
	FuncId(0)::local_15.cast#0.index.0 (1) ==> { FuncId(0)::local_8.cast#2 }
	FuncId(0)::local_16.0 (1) ==> { FuncId(0)::local_8.cast#2 }
	FuncId(0)::local_15.index.0 (1) ==> { FuncId(0)::local_8.cast#2 }
	FuncId(0)::local_12 (1) ==> { StrRefArr.cast#3 }
	FuncId(0)::local_16.1 (1) ==> { FuncId(8).cast#1 }
	FuncId(0)::local_11.2 (1) ==> { FuncId(0)::local_15.cast#0 }
	FuncId(0)::local_15.cast#0.index.1 (1) ==> { FuncId(8).cast#1 }
FuncId(1) - "same_crate_painfully_simple::main::promoted[0]"
	FuncId(1)::ret (1) ==> { StrRefArr }
FuncId(2) - "same_crate_painfully_simple::noop"
	FuncId(2)::local_3 (1) ==> { StrRefArr.cast#3 }
	FuncId(2)::local_2.0 (1) ==> { StrRefArr.cast#3 }
	FuncId(2)::local_4 (1) ==> { StrRefArr }
FuncId(3) - "core::fmt::rt::{impl#1}::new_debug<f32>"
	FuncId(3)::param_1 (1) ==> { FuncId(0)::local_8 }
	FuncId(3)::ret.0 (1) ==> { FuncId(0)::local_8.cast#2 }
	FuncId(3)::local_2 (1) ==> { FuncId(8) }
	FuncId(3)::local_4 (1) ==> { FuncId(0)::local_8.cast#2 }
	FuncId(3)::local_3 (1) ==> { FuncId(8).cast#1 }
	FuncId(3)::ret.1 (1) ==> { FuncId(8).cast#1 }
FuncId(4) - "core::fmt::{impl#2}::new_v1<>"
	FuncId(4)::param_2 (1) ==> { FuncId(0)::local_15.cast#0 }
	FuncId(4)::ret.0 (1) ==> { StrRefArr.cast#3 }
	FuncId(4)::param_1 (1) ==> { StrRefArr.cast#3 }
	FuncId(4)::ret.2 (1) ==> { FuncId(0)::local_15.cast#0 }
FuncId(6) - "same_crate_painfully_simple::noop::promoted[0]"
	FuncId(6)::ret (1) ==> { StrRefArr }
FuncId(7) - "core::fmt::{impl#2}::new_const<1>"
	FuncId(7)::param_1 (1) ==> { StrRefArr.cast#3 }
	FuncId(7)::ret.0 (1) ==> { StrRefArr.cast#3 }
FuncId(9) - "core::fmt::{impl#2}::new_v1::promoted[0]"
	FuncId(9)::ret (1) ==> { StrRefArr }
FuncId(11) - "core::fmt::{impl#2}::new_const::promoted[0]"
	FuncId(11)::ret (1) ==> { StrRefArr }
FuncId(12) - "core::fmt::{impl#2}::new_const::promoted[1]"
	FuncId(12)::ret (1) ==> { FuncId(12)::local_1 }
FuncId(13) - "same_crate_painfully_simple::{impl#0}::area"
	FuncId(13)::param_1 (1) ==> { FuncId(0)::local_2 }
FuncId(14) - "same_crate_painfully_simple::{impl#1}::area"
	FuncId(14)::param_1 (1) ==> { FuncId(0)::local_3 }
