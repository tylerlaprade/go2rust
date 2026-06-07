use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::alias::*;
use crate::api::*;
use crate::api_predicates::*;
use crate::array::*;
use crate::assignments::*;
use crate::badlinkname::*;
use crate::basic::*;
use crate::builtins::*;
use crate::call::*;
use crate::chan::*;
use crate::check::*;
use crate::r#const::*;
use crate::context::*;
use crate::conversions::*;
use crate::decl::*;
use crate::errors::*;
use crate::errsupport::*;
use crate::eval::*;
use crate::expr::*;
use crate::exprstring::*;
use crate::format::*;
use crate::gccgosizes::*;
use crate::gcsizes::*;
use crate::index::*;
use crate::infer::*;
use crate::initorder::*;
use crate::instantiate::*;
use crate::interface::*;
use crate::iter::*;
use crate::labels::*;
use crate::literals::*;
use crate::lookup::*;
use crate::map::*;
use crate::methodset::*;
use crate::mono::*;
use crate::named::*;
use crate::object::*;
use crate::objset::*;
use crate::operand::*;
use crate::package::*;
use crate::pointer::*;
use crate::predicates::*;
use crate::recording::*;
use crate::resolver::*;
use crate::r#return::*;
use crate::scope::*;
use crate::scope2::*;
use crate::signature::*;
use crate::sizes::*;
use crate::slice::*;
use crate::stmt::*;
use crate::r#struct::*;
use crate::subst::*;
use crate::termlist::*;
use crate::tuple::*;
use crate::r#type::*;
use crate::typelists::*;
use crate::typeparam::*;
use crate::typeset::*;
use crate::typestring::*;
use crate::typeterm::*;
use crate::typexpr::*;
use crate::under::*;
use crate::unify::*;
use crate::union::*;
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const FIELD_VAL: i32 = 0;
pub const METHOD_VAL: i32 = 1;
pub const METHOD_EXPR: i32 = 2;


/// SelectionKind describes the kind of a selector expression x.f
/// (excluding qualified identifiers).
///
/// If x is a struct or *struct, a selector expression x.f may denote a
/// sequence of selection operations x.a.b.c.f. The SelectionKind
/// describes the kind of the final (explicit) operation; all the
/// previous (implicit) operations are always field selections.
/// Each element of Indices specifies an implicit field (a, b, c)
/// by its index in the struct type of the field selection operand.
///
/// For a FieldVal operation, the final selection refers to the field
/// specified by Selection.Obj.
///
/// For a MethodVal operation, the final selection refers to a method.
/// If the "pointerness" of the method's declared receiver does not
/// match that of the effective receiver after implicit field
/// selection, then an & or * operation is implicitly applied to the
/// receiver variable or value.
/// So, x.f denotes (&x.a.b.c).f when f requires a pointer receiver but
/// x.a.b.c is a non-pointer variable; and it denotes (*x.a.b.c).f when
/// f requires a non-pointer receiver but x.a.b.c is a pointer value.
///
/// All pointer indirections, whether due to implicit or explicit field
/// selections or * operations inserted for "pointerness", panic if
/// applied to a nil pointer, so a method call x.f() may panic even
/// before the function call.
///
/// By contrast, a MethodExpr operation T.f is essentially equivalent
/// to a function literal of the form:
///
///	func(x T, args) (results) { return x.f(args) }
///
/// Consequently, any implicit field selections and * operations
/// inserted for "pointerness" are not evaluated until the function is
/// called, so a T.f or (*T).f expression never panics.
#[derive(Debug, Clone, Default)]
pub struct SelectionKind(pub Arc<Mutex<Option<i32>>>);

impl Display for SelectionKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for SelectionKind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for SelectionKind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for SelectionKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for SelectionKind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<SelectionKind> for i32 {
    fn eq(&self, other: &SelectionKind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<SelectionKind> for i32 {
    fn partial_cmp(&self, other: &SelectionKind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for SelectionKind {
    type Output = SelectionKind;
    fn add(self, other: Self) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for SelectionKind {
    type Output = SelectionKind;
    fn add(self, other: i32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<SelectionKind> for i32 {
    type Output = SelectionKind;
    fn add(self, other: SelectionKind) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for SelectionKind {
    type Output = SelectionKind;
    fn sub(self, other: Self) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for SelectionKind {
    type Output = SelectionKind;
    fn sub(self, other: i32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<SelectionKind> for i32 {
    type Output = SelectionKind;
    fn sub(self, other: SelectionKind) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for SelectionKind {
    type Output = SelectionKind;
    fn mul(self, other: Self) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for SelectionKind {
    type Output = SelectionKind;
    fn mul(self, other: i32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<SelectionKind> for i32 {
    type Output = SelectionKind;
    fn mul(self, other: SelectionKind) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for SelectionKind {
    type Output = SelectionKind;
    fn div(self, other: Self) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for SelectionKind {
    type Output = SelectionKind;
    fn div(self, other: i32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<SelectionKind> for i32 {
    type Output = SelectionKind;
    fn div(self, other: SelectionKind) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for SelectionKind {
    type Output = SelectionKind;
    fn neg(self) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for SelectionKind {
    type Output = SelectionKind;
    fn rem(self, other: Self) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for SelectionKind {
    type Output = SelectionKind;
    fn rem(self, other: i32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<SelectionKind> for i32 {
    type Output = SelectionKind;
    fn rem(self, other: SelectionKind) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for SelectionKind {
    type Output = SelectionKind;
    fn bitand(self, other: Self) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for SelectionKind {
    type Output = SelectionKind;
    fn bitand(self, other: i32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<SelectionKind> for i32 {
    type Output = SelectionKind;
    fn bitand(self, other: SelectionKind) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for SelectionKind {
    type Output = SelectionKind;
    fn bitor(self, other: Self) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for SelectionKind {
    type Output = SelectionKind;
    fn bitor(self, other: i32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<SelectionKind> for i32 {
    type Output = SelectionKind;
    fn bitor(self, other: SelectionKind) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for SelectionKind {
    type Output = SelectionKind;
    fn bitxor(self, other: Self) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for SelectionKind {
    type Output = SelectionKind;
    fn bitxor(self, other: i32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<SelectionKind> for i32 {
    type Output = SelectionKind;
    fn bitxor(self, other: SelectionKind) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for SelectionKind {
    type Output = SelectionKind;
    fn not(self) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for SelectionKind {
    type Output = SelectionKind;
    fn shl(self, other: SelectionKind) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for SelectionKind {
    type Output = SelectionKind;
    fn shl(self, other: i32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for SelectionKind {
    type Output = SelectionKind;
    fn shl(self, other: i8) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for SelectionKind {
    type Output = SelectionKind;
    fn shl(self, other: i16) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for SelectionKind {
    type Output = SelectionKind;
    fn shl(self, other: i64) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for SelectionKind {
    type Output = SelectionKind;
    fn shl(self, other: u32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for SelectionKind {
    type Output = SelectionKind;
    fn shl(self, other: u8) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for SelectionKind {
    type Output = SelectionKind;
    fn shl(self, other: u16) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for SelectionKind {
    type Output = SelectionKind;
    fn shl(self, other: u64) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for SelectionKind {
    type Output = SelectionKind;
    fn shl(self, other: usize) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for SelectionKind {
    type Output = SelectionKind;
    fn shr(self, other: SelectionKind) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for SelectionKind {
    type Output = SelectionKind;
    fn shr(self, other: i32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for SelectionKind {
    type Output = SelectionKind;
    fn shr(self, other: i8) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for SelectionKind {
    type Output = SelectionKind;
    fn shr(self, other: i16) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for SelectionKind {
    type Output = SelectionKind;
    fn shr(self, other: i64) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for SelectionKind {
    type Output = SelectionKind;
    fn shr(self, other: u32) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for SelectionKind {
    type Output = SelectionKind;
    fn shr(self, other: u8) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for SelectionKind {
    type Output = SelectionKind;
    fn shr(self, other: u16) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for SelectionKind {
    type Output = SelectionKind;
    fn shr(self, other: u64) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for SelectionKind {
    type Output = SelectionKind;
    fn shr(self, other: usize) -> SelectionKind {
        SelectionKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for SelectionKind {}

impl Ord for SelectionKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A Selection describes a selector expression x.f.
/// For the declarations:
///
///	type T struct{ x int; E }
///	type E struct{}
///	func (e E) m() {}
///	var p *T
///
/// the following relations exist:
///
///	Selector    Kind          Recv    Obj    Type       Index     Indirect
///
///	p.x         FieldVal      T       x      int        {0}       true
///	p.m         MethodVal     *T      m      func()     {1, 0}    true
///	T.m         MethodExpr    T       m      func(T)    {1, 0}    false
#[derive(Clone)]
pub struct Selection {
    pub kind: Arc<Mutex<Option<SelectionKind>>>,
    pub recv: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
    pub obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>,
    pub index: Arc<Mutex<Option<Vec<i32>>>>,
    pub indirect: Arc<Mutex<Option<bool>>>,
}

impl Selection {
    pub fn __go_value_clone(&self) -> Self {
        Self { kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recv: self.recv.clone(), obj: self.obj.clone(), index: self.index.clone(), indirect: { let __guard = self.indirect.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Selection {
    fn default() -> Self {
        Self { kind: Arc::new(Mutex::new(Some(SelectionKind(Arc::new(Mutex::new(Some(0))))))), recv: Arc::new(Mutex::new(None)), obj: Arc::new(Mutex::new(None)), index: Arc::new(Mutex::new(None)), indirect: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for Selection {
    fn go_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
    fn go_hash(&self, seed: usize) -> usize {
        let mut __hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut __hasher);
        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);
        std::hash::Hasher::finish(&__hasher) as usize
    }
}

impl GoJsonDecode for Selection {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub obj: Arc<Mutex<Option<Func>>>,
    pub ptr: Arc<Mutex<Option<bool>>>,
    pub recv: Arc<Mutex<Option<go_ast::r#mod::Ident>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { obj: self.obj.clone(), ptr: { let __guard = self.ptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recv: self.recv.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { obj: Arc::new(Mutex::new(None)), ptr: Arc::new(Mutex::new(Some(false))), recv: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { let __guard = self.obj.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.ptr.lock().unwrap().as_ref().unwrap()), { let __guard = self.recv.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Selection {
    /// Kind returns the selection kind.
    pub fn kind(&self) -> Arc<Mutex<Option<SelectionKind>>> {
        return self.kind.clone();
    }

    /// Recv returns the type of x in x.f.
    pub fn recv(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        return { let __field = self.recv.clone(); __field };
    }

    /// Obj returns the object denoted by x.f; a *Var for
    /// a field selection, and a *Func in all other cases.
    pub fn obj(&self) -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> {
        return { let __field = self.obj.clone(); __field };
    }

    /// Type returns the type of x.f, which may be different from the type of f.
    /// See Selection for more information.
    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        { let _switch_val = { let __selector_holder = self.kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (SelectionKind(Arc::new(Mutex::new(Some(METHOD_VAL as i32))))) {
                        // The type of x.f is a method with its receiver type set
                        // to the type of x.
            let mut sig = Arc::new(Mutex::new(Some({ let __v = (*({
        let val = (*({
        let val = self.obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).clone(); __v })));
            let mut recv = Arc::new(Mutex::new(Some({ let __v = (*(*sig.lock().unwrap().as_ref().unwrap()).recv.lock().unwrap().as_ref().unwrap()).clone(); __v })));
            { let __iface_handle = { let __field = self.recv.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*recv.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
            { let new_val = recv.clone().clone(); (*sig.lock().unwrap().as_mut().unwrap()).recv = new_val; };
            return Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone().clone())) as Box<dyn Type + Send + Sync>)));
        } else if _switch_val == (SelectionKind(Arc::new(Mutex::new(Some(METHOD_EXPR as i32))))) {
                        // The type of x.f is a function (without receiver)
                        // and an additional first argument with the same type as x.
                        // TODO(gri) Similar code is already in call.go - factor!
                        // TODO(gri) Compute this eagerly to avoid allocations.
            let mut sig = Arc::new(Mutex::new(Some({ let __v = (*({
        let val = (*({
        let val = self.obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).clone(); __v })));
            let mut arg0 = Arc::new(Mutex::new(Some({ let __v = (*(*sig.lock().unwrap().as_ref().unwrap()).recv.lock().unwrap().as_ref().unwrap()).clone(); __v })));
            *(*sig.lock().unwrap().as_ref().unwrap()).recv.lock().unwrap() = None;
            { let __iface_handle = { let __field = self.recv.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*arg0.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
            let mut params: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>> = Arc::new(Mutex::new(None));
            if { let __nil_target = (*sig.lock().unwrap().as_ref().unwrap()).params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*(*sig.lock().unwrap().as_ref().unwrap()).params.lock().unwrap().as_ref().unwrap()).vars.clone(); params = new_val; };
    }
            { let new_val = new_tuple({ let __append_target = Arc::new(Mutex::new(Some(vec![arg0.clone()]))).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = params.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }).clone(); (*sig.lock().unwrap().as_mut().unwrap()).params = new_val; };
            return Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone().clone())) as Box<dyn Type + Send + Sync>)));
        }
    }
                // The type of x.f is a method with its receiver type set
                // to the type of x.
                // The type of x.f is a function (without receiver)
                // and an additional first argument with the same type as x.
                // TODO(gri) Similar code is already in call.go - factor!
                // TODO(gri) Compute this eagerly to avoid allocations.
                // In all other cases, the type of x.f is the type of x.
        (*self.obj.lock().unwrap().as_ref().unwrap()).r#type().clone()
    }

    /// Index describes the path from x to f in x.f.
    /// The last index entry is the field or method index of the type declaring f;
    /// either:
    ///
    ///  1. the list of declared methods of a named type; or
    ///  2. the list of methods of an interface type; or
    ///  3. the list of fields of a struct type.
    ///
    /// The earlier index entries are the indices of the embedded fields implicitly
    /// traversed to get from (the type of) x to f, starting at embedding depth 0.
    pub fn index(&self) -> Arc<Mutex<Option<Vec<i32>>>> {
        return self.index.clone();
    }

    /// Indirect reports whether any pointer indirection was required to get from
    /// x to f in x.f.
    ///
    /// Beware: Indirect spuriously returns true (Go issue #8353) for a
    /// MethodVal selection in which the receiver argument and parameter
    /// both have type *T so there is no indirection.
    /// Unfortunately, a fix is too risky.
    pub fn indirect(&self) -> bool {
        return (*self.indirect.lock().unwrap().as_ref().unwrap());
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        selection_string(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(None)))
    }
}

impl SelectionKind {
}

impl cmp::r#mod::Ordered for SelectionKind {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SelectionKind>() {
            self == __other
        } else {
            false
        }
    }
}

/// SelectionString returns the string form of s.
/// The Qualifier controls the printing of
/// package-level objects, and may be nil.
///
/// Examples:
///
///	"field (T) f int"
///	"method (T) f(X) Y"
///	"method expr (T) f(X) Y"
pub fn selection_string(s: Arc<Mutex<Option<Selection>>>, qf: crate::typestring::Qualifier) -> Arc<Mutex<Option<String>>> {
    let mut k: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    { let _switch_val = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (SelectionKind(Arc::new(Mutex::new(Some(FIELD_VAL as i32))))) {
            { let new_val = "field ".to_string(); *k.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (SelectionKind(Arc::new(Mutex::new(Some(METHOD_VAL as i32))))) {
            { let new_val = "method ".to_string(); *k.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (SelectionKind(Arc::new(Mutex::new(Some(METHOD_EXPR as i32))))) {
            { let new_val = "method expr ".to_string(); *k.lock().unwrap() = Some(new_val); };
        } else {
            std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
        }
    }
    let mut buf: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    (*buf.lock().unwrap().as_mut().unwrap()).write_byte(Arc::new(Mutex::new(Some(('(' as i32) as u8))));
    write_type(buf.clone(), { let __recv = s.clone(); let __recv_ptr: *const Selection = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Selection }; let __result = unsafe { &*__recv_ptr }.recv(); __result }.clone(), qf.clone());
    { let __s = format!(") {}", (*(*(*s.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())); (*buf.clone().lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(__s.into_bytes())))) };
    {
        let mut T = { let __recv = s.clone(); let __recv_ptr: *const Selection = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Selection }; let __result = unsafe { &*__recv_ptr }.r#type(); __result };;
        if { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = SelectionKind(Arc::new(Mutex::new(Some(FIELD_VAL as i32)))); __tmp_x == __tmp_y } {
            (*buf.lock().unwrap().as_mut().unwrap()).write_byte(Arc::new(Mutex::new(Some((' ' as i32) as u8))));;
            write_type(buf.clone(), T.clone(), qf.clone());;
        } else {
            write_signature(buf.clone(), ({
        let val = T.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }), qf.clone());;
        }
    }
    return (*buf.lock().unwrap().as_ref().unwrap()).string();
}

impl GoValueClone for Selection {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
