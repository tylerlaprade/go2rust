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
use crate::scope2::*;
use crate::selection::*;
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
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Scope maintains a set of objects and links to its containing
/// (parent) and contained (children) scopes. Objects may be inserted
/// and looked up by name. The zero value for Scope is a ready-to-use
/// empty scope.
#[derive(Clone)]
pub struct Scope {
    pub parent: Arc<Mutex<Option<Scope>>>,
    pub children: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Scope>>>>>>>,
    pub number: Arc<Mutex<Option<i32>>>,
    pub elems: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>>,
    pub pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub end: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub comment: Arc<Mutex<Option<String>>>,
    pub is_func: Arc<Mutex<Option<bool>>>,
}

impl Scope {
    pub fn __go_value_clone(&self) -> Self {
        Self { parent: self.parent.clone(), children: self.children.clone(), number: { let __guard = self.number.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, elems: self.elems.clone(), pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, end: { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, comment: { let __guard = self.comment.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_func: { let __guard = self.is_func.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Scope {
    fn default() -> Self {
        Self { parent: Arc::new(Mutex::new(None)), children: Arc::new(Mutex::new(None)), number: Arc::new(Mutex::new(Some(0))), elems: Arc::new(Mutex::new(None)), pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), end: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), comment: Arc::new(Mutex::new(Some(String::new()))), is_func: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for Scope {
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

impl GoJsonDecode for Scope {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A lazyObject represents an imported Object that has not been fully
/// resolved yet by its importer.
#[derive(Clone)]
pub struct lazyObject {
    pub parent: Arc<Mutex<Option<Scope>>>,
    pub resolve: Arc<Mutex<Option<Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> + Send + Sync>>>>,
    pub obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>,
    pub once: GoOnce,
}

impl lazyObject {
    pub fn __go_value_clone(&self) -> Self {
        Self { parent: self.parent.clone(), resolve: self.resolve.clone(), obj: self.obj.clone(), once: self.once.clone() }
    }
}


impl Default for lazyObject {
    fn default() -> Self {
        Self { parent: Arc::new(Mutex::new(None)), resolve: Arc::new(Mutex::new(None)), obj: Arc::new(Mutex::new(None)), once: GoOnce::new() }
    }
}

impl std::fmt::Display for lazyObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for lazyObject {
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


impl Scope {
    /// Parent returns the scope's containing (parent) scope.
    pub fn parent(&self) -> Arc<Mutex<Option<Scope>>> {
        self.parent.clone()
    }

    /// Len returns the number of scope elements.
    pub fn len(&self) -> i32 {
        ({ let __len_target = { let __field = self.elems.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    /// Names returns the scope's element names in sorted order.
    pub fn names(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let mut names = Arc::new(Mutex::new(Some(vec!["".to_string(); (({ let __len_target = { let __field = self.elems.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize])));
        let mut i = Arc::new(Mutex::new(Some(0)));
        for (name, _) in { let __range_holder = self.elems.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        (*names.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = name;
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        slices::sort::<Vec<String>, String>(names.clone());
        return names.clone();
    }

    /// NumChildren returns the number of scopes nested in s.
    pub fn num_children(&self) -> i32 {
        ({ let __len_target = { let __field = self.children.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    /// Child returns the i'th child scope for 0 <= i < NumChildren().
    pub fn child(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Scope>>> {
        { let __seq = { let __seq_holder = self.children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
    }

    /// Lookup returns the object in scope s with the given name if such an
    /// object exists; otherwise the result is nil.
    pub fn lookup(&self, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> {
        let mut obj = resolve(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __map = { let __map_holder = self.elems.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone());
                // Hijack Lookup for "any": with gotypesalias=1, we want the Universe to
                // return an Alias for "any", and with gotypesalias=0 we want to return
                // the legacy representation of aliases.
                //
                // This is rather tricky, but works out after auditing of the usage of
                // s.elems. The only external API to access scope elements is Lookup.
                //
                // TODO: remove this once gotypesalias=0 is no longer supported.
        if { let __left_holder = obj.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Object + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::object::TypeNamePtr({ let __arg_holder = universeAnyAlias.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __right_opt: Option<&(dyn Object + Send + Sync)> = Some(&__right_wrapper as &(dyn Object + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_object(__right), _ => false }; __eq } && !alias_any() {
        return Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr({ let __arg_holder = universeAnyNoAlias.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })) as Box<dyn Object + Send + Sync>)));
    }
        return obj.clone();
    }

    /// Insert attempts to insert an object obj into scope s.
    /// If s already contains an alternative object alt with
    /// the same name, Insert leaves s unchanged and returns alt.
    /// Otherwise it inserts obj, sets the object's parent scope
    /// if not already set, and returns nil.
    pub fn insert(&mut self, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> {
        let mut name = (*obj.lock().unwrap().as_ref().unwrap()).name();
        {
        let mut alt = self.lookup(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*alt.lock().unwrap()).is_some(); __nil_result } {
            return alt.clone();;
        }
    }
        self.insert_1(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), obj.clone());
                // TODO(gri) Can we always set the parent to s (or is there
                // a need to keep the original parent or some race condition)?
                // If we can, than we may not need environment.lookupScope
                // which is only there so that we get the correct scope for
                // marking "used" dot-imported packages.
        if { let __nil_result = (*(*obj.lock().unwrap().as_ref().unwrap()).parent().lock().unwrap()).is_none(); __nil_result } {
        (*obj.lock().unwrap().as_mut().unwrap()).set_parent(Arc::new(Mutex::new(Some(self.clone()))));
    }
        return Arc::new(Mutex::new(None));
    }

    /// InsertLazy is like Insert, but allows deferring construction of the
    /// inserted object until it's accessed with Lookup. The Object
    /// returned by resolve must have the same name as given to InsertLazy.
    /// If s already contains an alternative object with the same name,
    /// InsertLazy leaves s unchanged and returns false. Otherwise it
    /// records the binding and returns true. The object's parent scope
    /// will be set to s after resolve is called.
    pub fn __insert_lazy(&mut self, name: Arc<Mutex<Option<String>>>, resolve: Arc<Mutex<Option<Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> + Send + Sync>>>>) -> bool {
        if { let __nil_result = (*{ let __map = { let __map_holder = self.elems.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).is_some(); __nil_result } {
        return false;
    }
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg1 = Arc::new(Mutex::new(Some(Box::new(lazyObjectPtr(Arc::new(Mutex::new(Some(lazyObject { parent: Arc::new(Mutex::new(Some(self.clone()))), resolve: resolve.clone(), ..Default::default() }))).clone())) as Box<dyn Object + Send + Sync>))); self.insert_1(__method_arg0, __method_arg1) };
        true
    }

    pub fn insert_1(&mut self, name: Arc<Mutex<Option<String>>>, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) {
        if { let __nil_target = self.elems.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>::new()))); self.elems = new_val; };
    }
        { let __map_key = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = obj.clone(); (*self.elems.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    /// WriteTo writes a string representation of the scope to w,
    /// with the scope elements sorted by name.
    /// The level of indentation is controlled by n >= 0, with
    /// n == 0 for no indentation.
    /// If recurse is set, it also writes nested (children) scopes.
    pub fn write_to(&self, w: Arc<Mutex<Option<io_Writer>>>, n: Arc<Mutex<Option<i32>>>, recurse: Arc<Mutex<Option<bool>>>) {
        const ind: &'static str = ".  ";

        let mut indn = Arc::new(Mutex::new(Some({ let __s = ind; let __count = (*n.lock().unwrap().as_ref().unwrap()); __s.repeat(__count as usize) })));
        { let __s = format!("{}{} scope {:p} {{\n", { let __v = (*indn.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*self.comment.lock().unwrap().as_ref().unwrap()), self); let __n = __s.len() as i32; (*w.lock().unwrap().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (__n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))) };
        let mut indn1 = Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*indn.lock().unwrap().as_ref().unwrap()).clone(); __v }, ind))));
        { let __range_holder = self.names().clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for name in __range_values.iter() {
        { let __s = format!("{}{}\n", { let __v = (*indn1.lock().unwrap().as_ref().unwrap()).clone(); __v }, format!("{}", (*(self.lookup(Arc::new(Mutex::new(Some((*name).clone()))))).lock().unwrap().as_ref().unwrap()))); let __n = __s.len() as i32; (*w.lock().unwrap().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (__n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))) };
    } }
        if { let __v = (*recurse.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __range_holder = self.children.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        { let __recv = s.clone(); let __recv_ptr: *const Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Scope }; let __result = unsafe { &*__recv_ptr }.write_to(w.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = recurse.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    } }
    }
        { let __s = format!("{}}}\n", { let __v = (*indn.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __n = __s.len() as i32; (*w.lock().unwrap().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (__n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))) };
    }

    /// String returns a string representation of the scope, for debugging.
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut buf: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.write_to(Arc::new(Mutex::new(Some(io_Writer::__go_from(buf.clone())))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false))));
        return Arc::new(Mutex::new(Some({ let __builder = buf.clone(); let __guard = __builder.lock().unwrap(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value })));
    }
}

impl positioner for Scope {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Scope::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Scope>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ScopePtr(pub Arc<Mutex<Option<Scope>>>);

impl std::fmt::Display for ScopePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl positioner for ScopePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Scope::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ScopePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl lazyObject {
    /// stub implementations so *lazyObject implements Object and we can
    /// store them directly into Scope.elems.
    pub fn parent(&self) -> Arc<Mutex<Option<Scope>>> {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn exported(&self) -> bool {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn id(&self) -> Arc<Mutex<Option<String>>> {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn order(&self) -> u32 {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn color(&self) -> Arc<Mutex<Option<crate::object::color>>> {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn set_type(&self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn set_order(&self, __arg0: Arc<Mutex<Option<u32>>>) {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn set_color(&self, color_local: Arc<Mutex<Option<color>>>) {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn set_parent(&self, __arg0: Arc<Mutex<Option<Scope>>>) {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn same_id(&self, __arg0: Arc<Mutex<Option<Package>>>, __arg1: Arc<Mutex<Option<String>>>, __arg2: Arc<Mutex<Option<bool>>>) -> bool {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn set_scope_pos(&self, __arg0: Arc<Mutex<Option<go_token::position::Pos>>>) {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }
}

impl Object for lazyObject {
    fn exported(&self) -> bool {
        lazyObject::exported(self)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        lazyObject::id(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        lazyObject::name(self)
    }
    fn parent(&self) -> Arc<Mutex<Option<Scope>>> {
        lazyObject::parent(self)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        lazyObject::pkg(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        lazyObject::pos(self)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        lazyObject::string(self)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        lazyObject::r#type(self)
    }
    fn color(&self) -> Arc<Mutex<Option<crate::object::color>>> {
        lazyObject::color(self)
    }
    fn order(&self) -> u32 {
        lazyObject::order(self)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        lazyObject::same_id(self, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        lazyObject::scope_pos(self)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<crate::object::color>>>) {
        lazyObject::set_color(self, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        lazyObject::set_order(self, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<Scope>>>) {
        lazyObject::set_parent(self, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        lazyObject::set_scope_pos(self, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        lazyObject::set_type(self, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<lazyObject>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct lazyObjectPtr(pub Arc<Mutex<Option<lazyObject>>>);

impl std::fmt::Display for lazyObjectPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Object for lazyObjectPtr {
    fn exported(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::exported(__recv)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::id(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::name(__recv)
    }
    fn parent(&self) -> Arc<Mutex<Option<Scope>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::parent(__recv)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::pkg(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::pos(__recv)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::string(__recv)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::r#type(__recv)
    }
    fn color(&self) -> Arc<Mutex<Option<crate::object::color>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::color(__recv)
    }
    fn order(&self) -> u32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::order(__recv)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::same_id(__recv, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::scope_pos(__recv)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<crate::object::color>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        lazyObject::set_color(__recv, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        lazyObject::set_order(__recv, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<Scope>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        lazyObject::set_parent(__recv, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        lazyObject::set_scope_pos(__recv, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        lazyObject::set_type(__recv, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<lazyObjectPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl positioner for lazyObject {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        lazyObject::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<lazyObject>() {
            false
        } else {
            false
        }
    }
}

impl positioner for lazyObjectPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        lazyObject::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<lazyObjectPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// NewScope returns a new, empty scope contained in the given parent
/// scope, if any. The comment is for debugging only.
pub fn new_scope(parent: Arc<Mutex<Option<Scope>>>, pos: Arc<Mutex<Option<go_token::position::Pos>>>, end: Arc<Mutex<Option<go_token::position::Pos>>>, comment: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Scope>>> {
    let mut s = Arc::new(Mutex::new(Some(Scope { parent: parent.clone(), children: Arc::new(Mutex::new(None)), number: Arc::new(Mutex::new(Some(0))), elems: Arc::new(Mutex::new(None)), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), end: Arc::new(Mutex::new(Some({ let __arg_holder = end.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), comment: Arc::new(Mutex::new(Some({ let __arg_holder = comment.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), is_func: Arc::new(Mutex::new(Some(false))), ..Default::default() })));

        // don't add children to Universe scope!
    if { let __nil_result = (*parent.lock().unwrap()).is_some(); __nil_result } && { let __left = parent.clone(); let __right = (*Universe.lock().unwrap().as_ref().unwrap()).clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        { let new_val = { let __append_target = (*parent.lock().unwrap().as_ref().unwrap()).children.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(s.clone()); __append_target.clone() }; (*parent.lock().unwrap().as_mut().unwrap()).children = new_val; };
        { let new_val = ({ let __len_target = { let __field = (*parent.lock().unwrap().as_ref().unwrap()).children.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32; *(*s.lock().unwrap().as_ref().unwrap()).number.lock().unwrap() = Some(new_val); };
    }
    return s.clone();
}

/// resolve returns the Object represented by obj, resolving lazy
/// objects as appropriate.
pub fn resolve(name: Arc<Mutex<Option<String>>>, mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> {
    let mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(obj.lock().unwrap().as_ref().map(|__v| Object::__go_clone_box_object(__v.as_ref()))));
    {
        let (mut lazy, mut ok) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<lazyObjectPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<lazyObject>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<lazyObject>)), false)
        }
    });;
        if ok {
            { let __once = (*lazy.lock().unwrap().as_ref().unwrap()).once.clone(); __once.r#do(|| {
        let mut obj = { let __f_holder = (*lazy.lock().unwrap().as_ref().unwrap()).resolve.clone(); let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        {
        let (_, mut ok) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<lazyObjectPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<lazyObject>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<lazyObject>)), false)
        }
    });;
        if ok {
            std::panic::panic_any(Box::new("recursive lazy object".to_string()) as Box<dyn Any + Send + Sync>);;
        }
    }
        if { let __tmp_x = (*(*obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("lazy object has unexpected name".to_string()) as Box<dyn Any + Send + Sync>);
    }
        if { let __nil_result = (*(*obj.lock().unwrap().as_ref().unwrap()).parent().lock().unwrap()).is_none(); __nil_result } {
        (*obj.lock().unwrap().as_mut().unwrap()).set_parent({ let __field = (*lazy.lock().unwrap().as_ref().unwrap()).parent.clone(); __field });
    }
        { let __iface_handle = obj.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*lazy.lock().unwrap().as_mut().unwrap()).obj.lock().unwrap() = __iface_value; };
    }); };;
            { let __iface_handle = { let __field = (*lazy.lock().unwrap().as_ref().unwrap()).obj.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *obj.lock().unwrap() = __iface_value; };;
        }
    }
    return obj.clone();
}

impl GoValueClone for Scope {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for lazyObject {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
