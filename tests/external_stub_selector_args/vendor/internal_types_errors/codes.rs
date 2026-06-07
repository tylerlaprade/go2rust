use go2rust_stdlib_stubs::*;

use crate::code_string::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const INVALID_SYNTAX_TREE: i32 = -1;


pub const TEST: i32 = 1;
pub const BLANK_PKG_NAME: i32 = 2;
pub const MISMATCHED_PKG_NAME: i32 = 3;
pub const INVALID_PKG_USE: i32 = 4;
pub const BAD_IMPORT_PATH: i32 = 5;
pub const BROKEN_IMPORT: i32 = 6;
pub const IMPORT_C_RENAMED: i32 = 7;
pub const UNUSED_IMPORT: i32 = 8;
pub const INVALID_INIT_CYCLE: i32 = 9;
pub const DUPLICATE_DECL: i32 = 10;
pub const INVALID_DECL_CYCLE: i32 = 11;
pub const INVALID_TYPE_CYCLE: i32 = 12;
pub const INVALID_CONST_INIT: i32 = 13;
pub const INVALID_CONST_VAL: i32 = 14;
pub const INVALID_CONST_TYPE: i32 = 15;
pub const UNTYPED_NIL_USE: i32 = 16;
pub const WRONG_ASSIGN_COUNT: i32 = 17;
pub const UNASSIGNABLE_OPERAND: i32 = 18;
pub const NO_NEW_VAR: i32 = 19;
pub const MULTI_VAL_ASSIGN_OP: i32 = 20;
pub const INVALID_IFACE_ASSIGN: i32 = 21;
pub const INVALID_CHAN_ASSIGN: i32 = 22;
pub const INCOMPATIBLE_ASSIGN: i32 = 23;
pub const UNADDRESSABLE_FIELD_ASSIGN: i32 = 24;
pub const NOT_A_TYPE: i32 = 25;
pub const INVALID_ARRAY_LEN: i32 = 26;
pub const BLANK_IFACE_METHOD: i32 = 27;
pub const INCOMPARABLE_MAP_KEY: i32 = 28;
pub const INVALID_PTR_EMBED: i32 = 30;
pub const BAD_RECV: i32 = 31;
pub const INVALID_RECV: i32 = 32;
pub const DUPLICATE_FIELD_AND_METHOD: i32 = 33;
pub const DUPLICATE_METHOD: i32 = 34;
pub const INVALID_BLANK: i32 = 35;
pub const INVALID_IOTA: i32 = 36;
pub const MISSING_INIT_BODY: i32 = 37;
pub const INVALID_INIT_SIG: i32 = 38;
pub const INVALID_INIT_DECL: i32 = 39;
pub const INVALID_MAIN_DECL: i32 = 40;
pub const TOO_MANY_VALUES: i32 = 41;
pub const NOT_AN_EXPR: i32 = 42;
pub const TRUNCATED_FLOAT: i32 = 43;
pub const NUMERIC_OVERFLOW: i32 = 44;
pub const UNDEFINED_OP: i32 = 45;
pub const MISMATCHED_TYPES: i32 = 46;
pub const DIV_BY_ZERO: i32 = 47;
pub const NON_NUMERIC_INC_DEC: i32 = 48;
pub const UNADDRESSABLE_OPERAND: i32 = 49;
pub const INVALID_INDIRECTION: i32 = 50;
pub const NON_INDEXABLE_OPERAND: i32 = 51;
pub const INVALID_INDEX: i32 = 52;
pub const SWAPPED_SLICE_INDICES: i32 = 53;
pub const NON_SLICEABLE_OPERAND: i32 = 54;
pub const INVALID_SLICE_EXPR: i32 = 55;
pub const INVALID_SHIFT_COUNT: i32 = 56;
pub const INVALID_SHIFT_OPERAND: i32 = 57;
pub const INVALID_RECEIVE: i32 = 58;
pub const INVALID_SEND: i32 = 59;
pub const DUPLICATE_LIT_KEY: i32 = 60;
pub const MISSING_LIT_KEY: i32 = 61;
pub const INVALID_LIT_INDEX: i32 = 62;
pub const OVERSIZE_ARRAY_LIT: i32 = 63;
pub const MIXED_STRUCT_LIT: i32 = 64;
pub const INVALID_STRUCT_LIT: i32 = 65;
pub const MISSING_LIT_FIELD: i32 = 66;
pub const DUPLICATE_LIT_FIELD: i32 = 67;
pub const UNEXPORTED_LIT_FIELD: i32 = 68;
pub const INVALID_LIT_FIELD: i32 = 69;
pub const UNTYPED_LIT: i32 = 70;
pub const INVALID_LIT: i32 = 71;
pub const AMBIGUOUS_SELECTOR: i32 = 72;
pub const UNDECLARED_IMPORTED_NAME: i32 = 73;
pub const UNEXPORTED_NAME: i32 = 74;
pub const UNDECLARED_NAME: i32 = 75;
pub const MISSING_FIELD_OR_METHOD: i32 = 76;
pub const BAD_DOT_DOT_DOT_SYNTAX: i32 = 77;
pub const NON_VARIADIC_DOT_DOT_DOT: i32 = 78;
pub const MISPLACED_DOT_DOT_DOT: i32 = 79;
pub const INVALID_DOT_DOT_DOT: i32 = 81;
pub const UNCALLED_BUILTIN: i32 = 82;
pub const INVALID_APPEND: i32 = 83;
pub const INVALID_CAP: i32 = 84;
pub const INVALID_CLOSE: i32 = 85;
pub const INVALID_COPY: i32 = 86;
pub const INVALID_COMPLEX: i32 = 87;
pub const INVALID_DELETE: i32 = 88;
pub const INVALID_IMAG: i32 = 89;
pub const INVALID_LEN: i32 = 90;
pub const SWAPPED_MAKE_ARGS: i32 = 91;
pub const INVALID_MAKE: i32 = 92;
pub const INVALID_REAL: i32 = 93;
pub const INVALID_ASSERT: i32 = 94;
pub const IMPOSSIBLE_ASSERT: i32 = 95;
pub const INVALID_CONVERSION: i32 = 96;
pub const INVALID_UNTYPED_CONVERSION: i32 = 97;
pub const BAD_OFFSETOF_SYNTAX: i32 = 98;
pub const INVALID_OFFSETOF: i32 = 99;
pub const UNUSED_EXPR: i32 = 100;
pub const UNUSED_VAR: i32 = 101;
pub const MISSING_RETURN: i32 = 102;
pub const WRONG_RESULT_COUNT: i32 = 103;
pub const OUT_OF_SCOPE_RESULT: i32 = 104;
pub const INVALID_COND: i32 = 105;
pub const INVALID_POST_DECL: i32 = 106;
pub const INVALID_ITER_VAR: i32 = 108;
pub const INVALID_RANGE_EXPR: i32 = 109;
pub const MISPLACED_BREAK: i32 = 110;
pub const MISPLACED_CONTINUE: i32 = 111;
pub const MISPLACED_FALLTHROUGH: i32 = 112;
pub const DUPLICATE_CASE: i32 = 113;
pub const DUPLICATE_DEFAULT: i32 = 114;
pub const BAD_TYPE_KEYWORD: i32 = 115;
pub const INVALID_TYPE_SWITCH: i32 = 116;
pub const INVALID_EXPR_SWITCH: i32 = 117;
pub const INVALID_SELECT_CASE: i32 = 118;
pub const UNDECLARED_LABEL: i32 = 119;
pub const DUPLICATE_LABEL: i32 = 120;
pub const MISPLACED_LABEL: i32 = 121;
pub const UNUSED_LABEL: i32 = 122;
pub const JUMP_OVER_DECL: i32 = 123;
pub const JUMP_INTO_BLOCK: i32 = 124;
pub const INVALID_METHOD_EXPR: i32 = 125;
pub const WRONG_ARG_COUNT: i32 = 126;
pub const INVALID_CALL: i32 = 127;
pub const UNUSED_RESULTS: i32 = 128;
pub const INVALID_DEFER: i32 = 129;
pub const INVALID_GO: i32 = 130;
pub const BAD_DECL: i32 = 131;
pub const REPEATED_DECL: i32 = 132;
pub const INVALID_UNSAFE_ADD: i32 = 133;
pub const INVALID_UNSAFE_SLICE: i32 = 134;
pub const UNSUPPORTED_FEATURE: i32 = 135;
pub const NOT_A_GENERIC_TYPE: i32 = 136;
pub const WRONG_TYPE_ARG_COUNT: i32 = 137;
pub const CANNOT_INFER_TYPE_ARGS: i32 = 138;
pub const INVALID_TYPE_ARG: i32 = 139;
pub const INVALID_INSTANCE_CYCLE: i32 = 140;
pub const INVALID_UNION: i32 = 141;
pub const MISPLACED_CONSTRAINT_IFACE: i32 = 142;
pub const INVALID_METHOD_TYPE_PARAMS: i32 = 143;
pub const MISPLACED_TYPE_PARAM: i32 = 144;
pub const INVALID_UNSAFE_SLICE_DATA: i32 = 145;
pub const INVALID_UNSAFE_STRING: i32 = 146;
pub const INVALID_CLEAR: i32 = 148;
pub const TYPE_TOO_LARGE: i32 = 149;
pub const INVALID_MIN_MAX_OPERAND: i32 = 150;
pub const TOO_NEW: i32 = 151;


#[derive(Debug, Clone, Default)]
pub struct Code(pub Arc<Mutex<Option<i32>>>);

impl Display for Code {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Code {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for Code {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Code {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for Code {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Code> for i32 {
    fn eq(&self, other: &Code) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Code> for i32 {
    fn partial_cmp(&self, other: &Code) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Code {
    type Output = Code;
    fn add(self, other: Self) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Code {
    type Output = Code;
    fn add(self, other: i32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Code> for i32 {
    type Output = Code;
    fn add(self, other: Code) -> Code {
        Code(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Code {
    type Output = Code;
    fn sub(self, other: Self) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Code {
    type Output = Code;
    fn sub(self, other: i32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Code> for i32 {
    type Output = Code;
    fn sub(self, other: Code) -> Code {
        Code(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Code {
    type Output = Code;
    fn mul(self, other: Self) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Code {
    type Output = Code;
    fn mul(self, other: i32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Code> for i32 {
    type Output = Code;
    fn mul(self, other: Code) -> Code {
        Code(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Code {
    type Output = Code;
    fn div(self, other: Self) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Code {
    type Output = Code;
    fn div(self, other: i32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Code> for i32 {
    type Output = Code;
    fn div(self, other: Code) -> Code {
        Code(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Code {
    type Output = Code;
    fn neg(self) -> Code {
        Code(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Code {
    type Output = Code;
    fn rem(self, other: Self) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Code {
    type Output = Code;
    fn rem(self, other: i32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Code> for i32 {
    type Output = Code;
    fn rem(self, other: Code) -> Code {
        Code(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Code {
    type Output = Code;
    fn bitand(self, other: Self) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Code {
    type Output = Code;
    fn bitand(self, other: i32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Code> for i32 {
    type Output = Code;
    fn bitand(self, other: Code) -> Code {
        Code(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Code {
    type Output = Code;
    fn bitor(self, other: Self) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Code {
    type Output = Code;
    fn bitor(self, other: i32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Code> for i32 {
    type Output = Code;
    fn bitor(self, other: Code) -> Code {
        Code(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Code {
    type Output = Code;
    fn bitxor(self, other: Self) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Code {
    type Output = Code;
    fn bitxor(self, other: i32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Code> for i32 {
    type Output = Code;
    fn bitxor(self, other: Code) -> Code {
        Code(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Code {
    type Output = Code;
    fn not(self) -> Code {
        Code(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Code {
    type Output = Code;
    fn shl(self, other: Code) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Code {
    type Output = Code;
    fn shl(self, other: i32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Code {
    type Output = Code;
    fn shl(self, other: i8) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Code {
    type Output = Code;
    fn shl(self, other: i16) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Code {
    type Output = Code;
    fn shl(self, other: i64) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Code {
    type Output = Code;
    fn shl(self, other: u32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Code {
    type Output = Code;
    fn shl(self, other: u8) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Code {
    type Output = Code;
    fn shl(self, other: u16) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Code {
    type Output = Code;
    fn shl(self, other: u64) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Code {
    type Output = Code;
    fn shl(self, other: usize) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Code {
    type Output = Code;
    fn shr(self, other: Code) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Code {
    type Output = Code;
    fn shr(self, other: i32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Code {
    type Output = Code;
    fn shr(self, other: i8) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Code {
    type Output = Code;
    fn shr(self, other: i16) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Code {
    type Output = Code;
    fn shr(self, other: i64) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Code {
    type Output = Code;
    fn shr(self, other: u32) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Code {
    type Output = Code;
    fn shr(self, other: u8) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Code {
    type Output = Code;
    fn shr(self, other: u16) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Code {
    type Output = Code;
    fn shr(self, other: u64) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Code {
    type Output = Code;
    fn shr(self, other: usize) -> Code {
        Code(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Code {}

impl Ord for Code {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}
