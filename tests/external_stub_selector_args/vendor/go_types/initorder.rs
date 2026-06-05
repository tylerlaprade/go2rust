use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

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

use internal_types_errors::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A dependency is an object that may be a dependency in an initialization
/// expression. Only constants, variables, and functions can be dependencies.
/// Constants are here because constant expression cycles are reported during
/// initialization order computation.
pub trait dependency: Object + std::fmt::Display + Any {
    fn __go_clone_box_dependency(&self) -> Box<dyn dependency + Send + Sync>;
    fn __go_eq_dependency(&self, other: &(dyn dependency + Send + Sync)) -> bool;
    fn is_dependency(&self);
}

impl Clone for Box<dyn dependency + Send + Sync> {
    fn clone(&self) -> Self {
        dependency::__go_clone_box_dependency(self.as_ref())
    }
}

#[derive(Clone)]
pub struct GodependencyInterfaceKey(pub Arc<Mutex<Option<Box<dyn dependency + Send + Sync>>>>);

impl GodependencyInterfaceKey {
    pub fn new(value: Arc<Mutex<Option<Box<dyn dependency + Send + Sync>>>>) -> Self { GodependencyInterfaceKey(value) }
    pub fn value(&self) -> Arc<Mutex<Option<Box<dyn dependency + Send + Sync>>>> { self.0.clone() }
    fn addr(&self) -> usize { Arc::as_ptr(&self.0) as usize }
    fn identity(&self) -> (u64, String) {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() {
            None => (0, String::new()),
            Some(__v) => {
                let mut __hasher = std::collections::hash_map::DefaultHasher::new();
                std::hash::Hash::hash(&__v.as_ref().__go_as_any().type_id(), &mut __hasher);
                (std::hash::Hasher::finish(&__hasher), format!("{}", __v))
            }
        }
    }
}
impl PartialEq for GodependencyInterfaceKey {
    fn eq(&self, other: &Self) -> bool {
        let __left_guard = self.0.lock().unwrap();
        let __right_guard = other.0.lock().unwrap();
        match (__left_guard.as_ref(), __right_guard.as_ref()) {
            (None, None) => true,
            (Some(__left), Some(__right)) => __left.as_ref().__go_eq_dependency(__right.as_ref()),
            _ => false,
        }
    }
}
impl Eq for GodependencyInterfaceKey {}
impl PartialOrd for GodependencyInterfaceKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for GodependencyInterfaceKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other { return std::cmp::Ordering::Equal; }
        match self.identity().cmp(&other.identity()) {
            std::cmp::Ordering::Equal => self.addr().cmp(&other.addr()),
            ordering => ordering,
        }
    }
}
impl std::fmt::Debug for GodependencyInterfaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.identity().1) }
}
impl std::fmt::Display for GodependencyInterfaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.identity().1) }
}

impl Object for Box<dyn dependency + Send + Sync> {
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        (**self).__go_eq_object(other)
    }
    fn exported(&self) -> bool {
        (**self).exported()
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        (**self).id()
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        (**self).name()
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        (**self).parent()
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        (**self).pkg()
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        (**self).string()
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        (**self).r#type()
    }
    fn color(&self) -> Arc<Mutex<Option<crate::object::color>>> {
        (**self).color()
    }
    fn order(&self) -> u32 {
        (**self).order()
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        (**self).same_id(pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).scope_pos()
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<crate::object::color>>>) {
        (**self).set_color(color_local)
    }
    fn set_order(&mut self, _arg0: Arc<Mutex<Option<u32>>>) {
        (**self).set_order(_arg0)
    }
    fn set_parent(&mut self, _arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        (**self).set_parent(_arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        (**self).set_scope_pos(pos)
    }
    fn set_type(&mut self, _arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        (**self).set_type(_arg0)
    }
}

impl positioner for Box<dyn dependency + Send + Sync> {
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        let _ = other;
        panic!("interface equality for structurally adapted dependency as positioner")
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
}

/// A graphNode represents a node in the object dependency graph.
/// Each node p in n.pred represents an edge p->n, and each node
/// s in n.succ represents an edge n->s; with a->b indicating that
/// a depends on b.
#[derive(Clone)]
pub struct graphNode {
    pub obj: Arc<Mutex<Option<Box<dyn dependency + Send + Sync>>>>,
    pub pred: Arc<Mutex<Option<nodeSet>>>,
    pub succ: Arc<Mutex<Option<nodeSet>>>,
    pub index: Arc<Mutex<Option<i32>>>,
    pub ndeps: Arc<Mutex<Option<i32>>>,
}

impl graphNode {
    pub fn __go_value_clone(&self) -> Self {
        Self { obj: self.obj.clone(), pred: self.pred.clone(), succ: self.succ.clone(), index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ndeps: { let __guard = self.ndeps.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for graphNode {
    fn default() -> Self {
        Self { obj: Arc::new(Mutex::new(None)), pred: Arc::new(Mutex::new(None)), succ: Arc::new(Mutex::new(None)), index: Arc::new(Mutex::new(Some(0))), ndeps: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for graphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.obj.lock().unwrap().as_ref().unwrap()), (*self.pred.lock().unwrap().as_ref().unwrap()), (*self.succ.lock().unwrap().as_ref().unwrap()), (*self.index.lock().unwrap().as_ref().unwrap()), (*self.ndeps.lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for graphNode {
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

impl GoJsonDecode for graphNode {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct nodeSet(pub Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<graphNode>, Arc<Mutex<Option<bool>>>>>>>);

impl Display for nodeSet {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


/// nodeQueue implements the container/heap interface;
/// a nodeQueue may be used as a priority queue.
#[derive(Clone, Default)]
pub struct nodeQueue(pub Arc<Mutex<Option<Vec<Arc<Mutex<Option<graphNode>>>>>>>);

impl Display for nodeQueue {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice_wrapped(&self.0))
    }
}


impl crate::check::Checker {
    /// initOrder computes the Info.InitOrder for package variables.
    pub fn init_order(&mut self) {
                // An InitOrder may already have been computed if a package is
                // built from several calls to (*Checker).Files. Clear it.
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.info.lock().unwrap().as_ref().unwrap()).init_order.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..(0) as usize].to_vec() }))); (*self.info.lock().unwrap().as_mut().unwrap()).init_order = new_val; };
                // Compute the object dependency graph and initialize
                // a priority queue with the list of graph nodes.
        let mut pq = Arc::new(Mutex::new(Some(nodeQueue(dependency_graph({ let __field = self.obj_map.clone(); __field })))));
        heap::init(pq.clone());
        const debug: bool = false;

        if debug {
        print!("Computing initialization order for {}\n\n", format!("&{}", (*{ let __field = self.pkg.clone(); __field }.lock().unwrap().as_ref().unwrap())));
        println!("{}", format!("{}", "Object dependency graph:".to_string()));
        for (__range_key, d) in { let __range_holder = self.obj_map.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let mut obj = __range_key.value();
                // only print objects that may appear in the dependency graph
        {
        let (mut obj, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::ConstPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn dependency + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn dependency + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::VarPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn dependency + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn dependency + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn dependency + Send + Sync>>)), false)
        }
    });;
        if (*obj.lock().unwrap()).is_some() {
            if { let __tmp_x = (({ let __len_target = { let __field = (*d.lock().unwrap().as_ref().unwrap()).deps.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        print!("\t{} depends on\n", (*(*obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()));
        for (__range_key, _) in { let __range_holder = (*d.lock().unwrap().as_ref().unwrap()).deps.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let dep = __range_key.value();
        print!("\t\t{}\n", (*(*dep.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()));
    }
    } else {
        print!("\t{} has no dependencies\n", (*(*obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()));
    };
        }
    }
    }
                // only print objects that may appear in the dependency graph
        println!();
        println!("{}", format!("{}", "Transposed object dependency graph (functions eliminated):".to_string()));
        { let __range_holder = { let __named_slice = (*pq.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter() {
        print!("\t{} depends on {} nodes\n", (*(*(*n.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()), (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).ndeps.clone(); __field }.lock().unwrap().as_ref().unwrap()));
        for (__range_key, _) in { let __range_holder = { let __named_map_holder = (*n.lock().unwrap().as_ref().unwrap()).pred.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let p = __range_key.value();
        print!("\t\t{} is dependent\n", (*(*(*p.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()));
    }
    } }
        println!();
        println!("{}", format!("{}", "Processing nodes:".to_string()));
    }
                // only print objects that may appear in the dependency graph
                // Determine initialization order by removing the highest priority node
                // (the one with the fewest dependencies) and its edges from the graph,
                // repeatedly, until there are no nodes left.
                // In a valid Go program, those nodes always have zero dependencies (after
                // removing all incoming dependencies), otherwise there are initialization
                // cycles.
        let mut emitted = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::resolver::declInfo>, Arc<Mutex<Option<bool>>>>::new())));
        while { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*pq.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // get the next node
        let mut n = ({
        let val = heap::pop(pq.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<graphNode>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();

        if debug {
        print!("\t{} (src pos {}) depends on {} nodes now\n", (*(*(*n.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()), (*(*n.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).order(), (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).ndeps.clone(); __field }.lock().unwrap().as_ref().unwrap()));
    }

                // if n still depends on other nodes, we have a cycle
        if { let __tmp_x = (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).ndeps.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut cycle = find_path({ let __field = self.obj_map.clone(); __field }, { let __inner: Box<dyn Object + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) }, { let __inner: Box<dyn Object + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) }, Arc::new(Mutex::new(Some(BTreeMap::<GoObjectInterfaceKey, Arc<Mutex<Option<bool>>>>::new()))));
                // If n.obj is not part of the cycle (e.g., n.obj->b->c->d->c),
                // cycle will be nil. Don't report anything in that case since
                // the cycle is reported when the algorithm gets to an object
                // in the cycle.
                // Furthermore, once an object in the cycle is encountered,
                // the cycle will be broken (dependency count will be reduced
                // below), and so the remaining nodes in the cycle don't trigger
                // another error (unless they are part of multiple cycles).
        if (*cycle.lock().unwrap()).is_some() {
        self.report_cycle(cycle.clone());
    }
    }

                // If n.obj is not part of the cycle (e.g., n.obj->b->c->d->c),
                // cycle will be nil. Don't report anything in that case since
                // the cycle is reported when the algorithm gets to an object
                // in the cycle.
                // Furthermore, once an object in the cycle is encountered,
                // the cycle will be broken (dependency count will be reduced
                // below), and so the remaining nodes in the cycle don't trigger
                // another error (unless they are part of multiple cycles).
                // Ok to continue, but the variable initialization order
                // will be incorrect at this point since it assumes no
                // cycle errors.
                // reduce dependency count of all dependent nodes
                // and update priority queue
        for (__range_key, _) in { let __range_holder = { let __named_map_holder = (*n.lock().unwrap().as_ref().unwrap()).pred.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let p = __range_key.value();
        { let __target = (*p.lock().unwrap().as_ref().unwrap()).ndeps.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        heap::fix(pq.clone(), { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).index.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned });
    }

                // record the init order for variables with initializers only
        let (mut v, _) = ({
        let val = (*n.lock().unwrap().as_ref().unwrap()).obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn dependency + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::VarPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Var>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Var>)), false)
        }
    });
        let mut info = { let __map = { let __map_holder = self.obj_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new(Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(v.clone())) as Box<dyn Object + Send + Sync>)))))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        if (*v.lock().unwrap()).is_none() || !{ let __recv = info.clone(); let __recv_ptr: *const crate::resolver::declInfo = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::resolver::declInfo }; let __result = unsafe { &*__recv_ptr }.has_initializer(); __result } {
        continue
    }

                // n:1 variable declarations such as: a, b = f()
                // introduce a node for each lhs variable (here: a, b);
                // but they all have the same initializer - emit only
                // one, for the first variable seen
        if { let __map = { let __map_holder = emitted.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(info.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        continue
    }
                // initializer already emitted, if any
        { let __map_key = GoLocalPtrKey::new(info.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*emitted.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };

        let mut infoLhs = (*info.lock().unwrap().as_ref().unwrap()).lhs.clone();
        if (*infoLhs.lock().unwrap()).is_none() {
        { let new_val = Arc::new(Mutex::new(Some(vec![v.clone()]))); infoLhs = new_val; };
    }
        let mut init = Arc::new(Mutex::new(Some(Initializer { lhs: infoLhs.clone(), rhs: { let __field = (*info.lock().unwrap().as_ref().unwrap()).init.clone(); __field }, ..Default::default() })));
        { let new_val = { let __append_target = (*self.info.lock().unwrap().as_ref().unwrap()).init_order.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(init.clone()); __append_target.clone() }; (*self.info.lock().unwrap().as_mut().unwrap()).init_order = new_val; };
    }
                // get the next node
                // if n still depends on other nodes, we have a cycle
                // If n.obj is not part of the cycle (e.g., n.obj->b->c->d->c),
                // cycle will be nil. Don't report anything in that case since
                // the cycle is reported when the algorithm gets to an object
                // in the cycle.
                // Furthermore, once an object in the cycle is encountered,
                // the cycle will be broken (dependency count will be reduced
                // below), and so the remaining nodes in the cycle don't trigger
                // another error (unless they are part of multiple cycles).
                // Ok to continue, but the variable initialization order
                // will be incorrect at this point since it assumes no
                // cycle errors.
                // reduce dependency count of all dependent nodes
                // and update priority queue
                // record the init order for variables with initializers only
                // n:1 variable declarations such as: a, b = f()
                // introduce a node for each lhs variable (here: a, b);
                // but they all have the same initializer - emit only
                // one, for the first variable seen
                // initializer already emitted, if any
                // possibly nil (see declInfo.lhs field comment)
        if debug {
        println!();
        println!("{}", format!("{}", "Initialization order:".to_string()));
        { let __range_holder = (*self.info.lock().unwrap().as_ref().unwrap()).init_order.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for init in __range_values.iter() {
        print!("\t{}\n", format!("&{}", (*init.lock().unwrap().as_ref().unwrap())));
    } }
        println!();
    }
    }

    /// reportCycle reports an error for the given cycle.
    pub fn report_cycle(&self, cycle: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>>) {
        let mut obj = { let __seq = { let __seq_holder = cycle.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
                // report a more concise error for self references
        if { let __tmp_x = ((*cycle.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_INIT_CYCLE as i32))))))), Arc::new(Mutex::new(Some("initialization cycle: %s refers to itself".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*obj.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
        return;
    }
        let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_INIT_CYCLE as i32))))))));
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("initialization cycle for %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*obj.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
                // "cycle[i] refers to cycle[j]" for (i,j) = (0,n-1), (n-1,n-2), ..., (1,0) for len(cycle) = n.
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = ((*cycle.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        let mut next = { let __seq = { let __seq_holder = cycle.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("%s refers to %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*obj.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = (*next.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __iface_handle = next.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *obj.lock().unwrap() = (*__iface_guard).clone(); };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
    }
}

impl graphNode {
    /// cost returns the cost of removing this node, which involves copying each
    /// predecessor to each successor (and vice-versa).
    pub fn cost(&self) -> i32 {
        return { let __tmp_x = ({ let __named_map_holder = self.pred.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) } as i32); let __tmp_y = ({ let __named_map_holder = self.succ.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) } as i32); __tmp_x * __tmp_y };
    }
}

impl nodeSet {
    pub fn add(&mut self, p: Arc<Mutex<Option<graphNode>>>) {
        if { let __map_holder = self.0.clone(); let __map_guard = __map_holder.lock().unwrap(); (*__map_guard).is_none() } {
        { let new_val = Arc::new(Mutex::new(Some(nodeSet(Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<graphNode>, Arc<Mutex<Option<bool>>>>::new()))))))); *self = new_val.lock().unwrap().take().unwrap_or_default(); };
    }
        { let __map_key = GoLocalPtrKey::new(p.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*self.0.clone().lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
}

impl nodeQueue {
    pub fn len(&self) -> i32 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32;
    }

    pub fn swap(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) {
        let (mut x, mut y) = ({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone(), { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone());
        { let __tmp_0 = y.clone(); let __tmp_1 = x.clone(); (*self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_0; (*self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_1; };
        { let __tmp_0 = (*j.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = (*i.lock().unwrap().as_ref().unwrap()).clone(); *(*x.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(__tmp_0); *(*y.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(__tmp_1); };
    }

    pub fn less(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) -> bool {
        let (mut x, mut y) = ({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone(), { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone());
                // Prioritize all constants before non-constants. See go.dev/issue/66575/.
        let (_, mut xConst) = ({
        let val = (*x.lock().unwrap().as_ref().unwrap()).obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn dependency + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::ConstPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Const>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Const>)), false)
        }
    });
        let (_, mut yConst) = ({
        let val = (*y.lock().unwrap().as_ref().unwrap()).obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn dependency + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::ConstPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Const>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Const>)), false)
        }
    });
        if { let __tmp_x = xConst; let __tmp_y = yConst; __tmp_x != __tmp_y } {
        return xConst;
    }
                // nodes are prioritized by number of incoming dependencies (1st key)
                // and source order (2nd key)
        return { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).ndeps.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).ndeps.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).ndeps.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).ndeps.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && { let __tmp_x = (*(*x.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).order(); let __tmp_y = (*(*y.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).order(); __tmp_x < __tmp_y };
    }

    pub fn push(&self, x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn pop(&mut self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.clone();
        { let new_val = -1; *(*x.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
        { let new_val = nodeQueue(Arc::new(Mutex::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __seq[..__high].to_vec() })))); *self = new_val; };
        return Arc::new(Mutex::new(Some(Box::new(x.clone()) as Box<dyn Any + Send + Sync>)));
    }
}

/// findPath returns the (reversed) list of objects []Object{to, ... from}
/// such that there is a path of object dependencies from 'from' to 'to'.
/// If there is no such path, the result is nil.
pub fn find_path(objMap: Arc<Mutex<Option<BTreeMap<GoObjectInterfaceKey, Arc<Mutex<Option<declInfo>>>>>>>, from: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, to: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, seen: Arc<Mutex<Option<BTreeMap<GoObjectInterfaceKey, Arc<Mutex<Option<bool>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>> {
    if { let __map = { let __map_holder = seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new(from.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        return Arc::new(Mutex::new(None));
    }
    { let __map_key = GoObjectInterfaceKey::new(from.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };

        // sort deps for deterministic result
    let mut deps: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
    for (__range_key, _) in { let __range_holder = (*{ let __map = { let __map_holder = objMap.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new(from.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).deps.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let d = __range_key.value();
        { let new_val = { let __append_target = deps.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(d.clone()); __append_target.clone() }; deps = new_val; };
    }
    let deps_closure_clone = deps.clone(); { let __sort_target = deps_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some(Box::new(move |i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>| -> bool {
        return { let __tmp_x = { let __recv = { let __seq = { let __seq_holder = deps_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).order(); __result }; let __tmp_y = { let __recv = { let __seq = { let __seq_holder = deps_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).order(); __result }; __tmp_x < __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i32))), Arc::new(Mutex::new(Some((__sort_j - 1) as i32)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };

    { let __range_holder = deps.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for d in __range_values.iter() {
        if { let __left_holder = d.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Object + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = to.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Object + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_object(__right), _ => false }; __eq } {
        return Arc::new(Mutex::new(Some(vec![d.clone()])));
    }
        {
        let mut P = find_path(objMap.clone(), d.clone(), to.clone(), seen.clone());;
        if (*P.lock().unwrap()).is_some() {
            return { let __append_target = P.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*d).clone()); __append_target.clone() };;
        }
    }
    } }

    return Arc::new(Mutex::new(None));
}

/// dependencyGraph computes the object dependency graph from the given objMap,
/// with any function nodes removed. The resulting graph contains only constants
/// and variables.
pub fn dependency_graph(objMap: Arc<Mutex<Option<BTreeMap<GoObjectInterfaceKey, Arc<Mutex<Option<declInfo>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<graphNode>>>>>>> {
        // M is the dependency (Object) -> graphNode mapping
    let mut M = Arc::new(Mutex::new(Some(BTreeMap::<GodependencyInterfaceKey, Arc<Mutex<Option<graphNode>>>>::new())));
    for (__range_key, _) in { let __range_holder = objMap.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let mut obj = __range_key.value();
                // only consider nodes that may be an initialization dependency
        {
        let (mut obj, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::ConstPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn dependency + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn dependency + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::VarPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn dependency + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn dependency + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn dependency + Send + Sync>>)), false)
        }
    });;
        if (*obj.lock().unwrap()).is_some() {
            { let __map_key = GodependencyInterfaceKey::new(obj.clone()); let __map_value = Arc::new(Mutex::new(Some(graphNode { obj: obj.clone(), ..Default::default() }))); (*M.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }

        // only consider nodes that may be an initialization dependency
        // compute edges for graph M
        // (We need to include all nodes, even isolated ones, because they still need
        // to be scheduled for initialization in correct order relative to other nodes.)
    for (__range_key, n) in { let __range_holder = M.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let obj = __range_key.value();
                // for each dependency obj -> d (= deps[i]), create graph edges n->s and s->n
        for (__range_key, _) in { let __range_holder = (*{ let __map = { let __map_holder = objMap.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new({ let __inner: Box<dyn Object + Send + Sync> = (*obj.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) }))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).deps.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let mut d = __range_key.value();
                // only consider nodes that may be an initialization dependency
        {
        let (mut d, _) = ({
        let val = d.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::ConstPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn dependency + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn dependency + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::VarPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn dependency + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn dependency + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn dependency + Send + Sync>>)), false)
        }
    });;
        if (*d.lock().unwrap()).is_some() {
            let mut d = { let __map = { let __map_holder = M.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GodependencyInterfaceKey::new(d.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
            (*(*n.lock().unwrap().as_ref().unwrap()).succ.lock().unwrap().as_mut().unwrap()).add(d.clone());;
            (*(*d.lock().unwrap().as_ref().unwrap()).pred.lock().unwrap().as_mut().unwrap()).add(n.clone());;
        }
    }
    }
    }

        // for each dependency obj -> d (= deps[i]), create graph edges n->s and s->n
        // only consider nodes that may be an initialization dependency
    let mut G: Arc<Mutex<Option<Vec<Arc<Mutex<Option<graphNode>>>>>>> = Arc::new(Mutex::new(None));let mut funcG: Arc<Mutex<Option<Vec<Arc<Mutex<Option<graphNode>>>>>>> = Arc::new(Mutex::new(None));
    for (_, n) in { let __range_holder = M.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        {
        let (_, mut ok) = ({
        let val = (*n.lock().unwrap().as_ref().unwrap()).obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn dependency + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
        }
    });;
        if ok {
            { let new_val = { let __append_target = funcG.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(n.clone()); __append_target.clone() }; funcG = new_val; };;
        } else {
            { let new_val = { let __append_target = G.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(n.clone()); __append_target.clone() }; G = new_val; };;
        }
    }
    }

        // remove function nodes and collect remaining graph nodes in G
        // (Mutually recursive functions may introduce cycles among themselves
        // which are permitted. Yet such cycles may incorrectly inflate the dependency
        // count for variables which in turn may not get scheduled for initialization
        // in correct order.)
        //
        // Note that because we recursively copy predecessors and successors
        // throughout the function graph, the cost of removing a function at
        // position X is proportional to cost * (len(funcG)-X). Therefore, we should
        // remove high-cost functions last.
    slices::sort_func::<Vec<Arc<Mutex<Option<graphNode>>>>, graphNode>(funcG.clone(), Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<graphNode>>>, b: Arc<Mutex<Option<graphNode>>>| -> i32 {
        cmp::compare::<i32>({ let __recv = a.clone(); let __recv_ptr: *const graphNode = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const graphNode }; let __result = unsafe { &*__recv_ptr }.cost(); __result }, { let __recv = b.clone(); let __recv_ptr: *const graphNode = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const graphNode }; let __result = unsafe { &*__recv_ptr }.cost(); __result })
    }) as Box<dyn FnMut(Arc<Mutex<Option<graphNode>>>, Arc<Mutex<Option<graphNode>>>) -> i32 + Send + Sync>))));
    { let __range_holder = funcG.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter() {
                // connect each predecessor p of n with each successor s
                // and drop the function node (don't collect it in G)
        for (__range_key, _) in { let __range_holder = { let __named_map_holder = (*n.lock().unwrap().as_ref().unwrap()).pred.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let p = __range_key.value();
                // ignore self-cycles
        if { let __left = p.clone(); let __right = n.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
                // Each successor s of n becomes a successor of p, and
                // each predecessor p of n becomes a predecessor of s.
        for (__range_key, _) in { let __range_holder = { let __named_map_holder = (*n.lock().unwrap().as_ref().unwrap()).succ.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let s = __range_key.value();
                // ignore self-cycles
        if { let __left = s.clone(); let __right = n.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        (*(*p.lock().unwrap().as_ref().unwrap()).succ.lock().unwrap().as_mut().unwrap()).add(s.clone());
        (*(*s.lock().unwrap().as_ref().unwrap()).pred.lock().unwrap().as_mut().unwrap()).add(p.clone());
    }
    }
                // ignore self-cycles
        { let __map_handle = { let __named_map = (*(*p.lock().unwrap().as_ref().unwrap()).succ.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }; let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&GoLocalPtrKey::new(n.clone())); };
    }
    }
                // ignore self-cycles
                // Each successor s of n becomes a successor of p, and
                // each predecessor p of n becomes a predecessor of s.
                // ignore self-cycles
                // remove edge to n
        for (__range_key, _) in { let __range_holder = { let __named_map_holder = (*n.lock().unwrap().as_ref().unwrap()).succ.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let s = __range_key.value();
        { let __map_handle = { let __named_map = (*(*s.lock().unwrap().as_ref().unwrap()).pred.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }; let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&GoLocalPtrKey::new(n.clone())); };
    }
    } }

        // connect each predecessor p of n with each successor s
        // and drop the function node (don't collect it in G)
        // ignore self-cycles
        // Each successor s of n becomes a successor of p, and
        // each predecessor p of n becomes a predecessor of s.
        // ignore self-cycles
        // remove edge to n
        // remove edge to n
        // fill in index and ndeps fields
    { let __range_holder = G.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, n) in __range_values.iter().enumerate() {
        { let new_val = i as i32; *(*n.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
        { let new_val = { let __named_map_holder = (*n.lock().unwrap().as_ref().unwrap()).succ.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) } as i32; *(*n.lock().unwrap().as_ref().unwrap()).ndeps.lock().unwrap() = Some(new_val); };
    } }

    return G.clone();
}

impl GoValueClone for graphNode {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
