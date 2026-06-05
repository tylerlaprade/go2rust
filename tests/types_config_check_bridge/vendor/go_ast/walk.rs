use go2rust_stdlib_stubs::*;

use crate::{__go_type_name, format_any, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values};

use crate::r#mod::*;
use crate::commentmap::*;
use crate::filter::*;
use crate::import::*;
use crate::print::*;
use crate::resolve::*;
use crate::scope::*;

use std::any::Any;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

/// A Visitor's Visit method is invoked for each node encountered by [Walk].
/// If the result visitor w is not nil, [Walk] visits each of the children
/// of node with the visitor w, followed by a call of w.Visit(nil).
pub trait Visitor: std::fmt::Display + Any {
    fn __go_clone_box_visitor(&self) -> Box<dyn Visitor + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_visitor(&self, other: &(dyn Visitor + Send + Sync)) -> bool;
    fn visit(&mut self, node: Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Visitor + Send + Sync>>>>;
}

impl Clone for Box<dyn Visitor + Send + Sync> {
    fn clone(&self) -> Self {
        Visitor::__go_clone_box_visitor(self.as_ref())
    }
}

pub fn walk_list<N: crate::r#mod::Node + Clone + Send + Sync + 'static>(v: Arc<Mutex<Option<Box<dyn Visitor + Send + Sync>>>>, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<N>>>>>>>) {
    { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for node in __range_values.iter() {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new((*node.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Node + Send + Sync>))));
    } }
}

/// Walk traverses an AST in depth-first order: It starts by calling
/// v.Visit(node); node must not be nil. If the visitor w returned by
/// v.Visit(node) is not nil, Walk is invoked recursively with visitor
/// w for each of the non-nil children of node, followed by a call of
/// w.Visit(nil).
pub fn walk(mut v: Arc<Mutex<Option<Box<dyn Visitor + Send + Sync>>>>, node: Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>>) {
    let mut v: Arc<Mutex<Option<Box<dyn Visitor + Send + Sync>>>> = Arc::new(Mutex::new(v.lock().unwrap().as_ref().map(|__v| Visitor::__go_clone_box_visitor(__v.as_ref()))));
    {
        { let __iface_handle = { let __recv = v.clone(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).visit(node.clone()).clone(); __result }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *v.lock().unwrap() = __iface_value; };;
        if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
            return;;
        }
    }

        // walk children
        // (the order of the cases matches the order
        // of the corresponding node types in ast.go)
    {
    let _ts_subject = node.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Node + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CommentPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CommentPtr>()).unwrap().0.clone();
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CommentGroupPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CommentGroupPtr>()).unwrap().0.clone();
        walk_list::<crate::r#mod::Comment>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).list.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FieldPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FieldPtr>()).unwrap().0.clone();
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).doc.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CommentGroupPtr((*n.lock().unwrap().as_ref().unwrap()).doc.clone())) as Box<dyn Node + Send + Sync>))));
    };
        walk_list::<crate::r#mod::Ident>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).names.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).tag.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::BasicLitPtr((*n.lock().unwrap().as_ref().unwrap()).tag.clone())) as Box<dyn Node + Send + Sync>))));
    };
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).comment.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CommentGroupPtr((*n.lock().unwrap().as_ref().unwrap()).comment.clone())) as Box<dyn Node + Send + Sync>))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FieldListPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FieldListPtr>()).unwrap().0.clone();
        walk_list::<crate::r#mod::Field>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).list.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BadExprPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::IdentPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BasicLitPtr>()).is_some() {
        let n = _ts_subject.clone();
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::EllipsisPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::EllipsisPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).elt.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).elt.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FuncLitPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FuncLitPtr>()).unwrap().0.clone();
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::FuncTypePtr((*n.lock().unwrap().as_ref().unwrap()).r#type.clone())) as Box<dyn Node + Send + Sync>))));;
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::BlockStmtPtr((*n.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CompositeLitPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CompositeLitPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        walk_list::<Box<dyn Expr + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).elts.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ParenExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ParenExprPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::SelectorExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::SelectorExprPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::IdentPtr((*n.lock().unwrap().as_ref().unwrap()).sel.clone())) as Box<dyn Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::IndexExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::IndexExprPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::IndexListExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::IndexListExprPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        walk_list::<Box<dyn Expr + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).indices.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::SliceExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::SliceExprPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).low.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).low.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).high.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).high.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).max.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).max.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::TypeAssertExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::TypeAssertExprPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CallExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CallExprPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).fun.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        walk_list::<Box<dyn Expr + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).args.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::StarExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::StarExprPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::UnaryExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::UnaryExprPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BinaryExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BinaryExprPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).y.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::KeyValueExprPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::KeyValueExprPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).key.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ArrayTypePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ArrayTypePtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).len.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).len.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).elt.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::StructTypePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::StructTypePtr>()).unwrap().0.clone();
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::FieldListPtr((*n.lock().unwrap().as_ref().unwrap()).fields.clone())) as Box<dyn Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FuncTypePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FuncTypePtr>()).unwrap().0.clone();
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).type_params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::FieldListPtr((*n.lock().unwrap().as_ref().unwrap()).type_params.clone())) as Box<dyn Node + Send + Sync>))));
    };
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::FieldListPtr((*n.lock().unwrap().as_ref().unwrap()).params.clone())) as Box<dyn Node + Send + Sync>))));
    };
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).results.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::FieldListPtr((*n.lock().unwrap().as_ref().unwrap()).results.clone())) as Box<dyn Node + Send + Sync>))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::InterfaceTypePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::InterfaceTypePtr>()).unwrap().0.clone();
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::FieldListPtr((*n.lock().unwrap().as_ref().unwrap()).methods.clone())) as Box<dyn Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::MapTypePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::MapTypePtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).key.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ChanTypePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ChanTypePtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BadStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BadStmtPtr>()).unwrap().0.clone();
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::DeclStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::DeclStmtPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::EmptyStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::EmptyStmtPtr>()).unwrap().0.clone();
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::LabeledStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::LabeledStmtPtr>()).unwrap().0.clone();
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::IdentPtr((*n.lock().unwrap().as_ref().unwrap()).label.clone())) as Box<dyn Node + Send + Sync>))));;
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).stmt.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ExprStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ExprStmtPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::SendStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::SendStmtPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).chan.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::IncDecStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::IncDecStmtPtr>()).unwrap().0.clone();
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::AssignStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::AssignStmtPtr>()).unwrap().0.clone();
        walk_list::<Box<dyn Expr + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).lhs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
        walk_list::<Box<dyn Expr + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).rhs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::GoStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::GoStmtPtr>()).unwrap().0.clone();
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CallExprPtr((*n.lock().unwrap().as_ref().unwrap()).call.clone())) as Box<dyn Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::DeferStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::DeferStmtPtr>()).unwrap().0.clone();
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CallExprPtr((*n.lock().unwrap().as_ref().unwrap()).call.clone())) as Box<dyn Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ReturnStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ReturnStmtPtr>()).unwrap().0.clone();
        walk_list::<Box<dyn Expr + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).results.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BranchStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BranchStmtPtr>()).unwrap().0.clone();
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).label.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::IdentPtr((*n.lock().unwrap().as_ref().unwrap()).label.clone())) as Box<dyn Node + Send + Sync>))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BlockStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BlockStmtPtr>()).unwrap().0.clone();
        walk_list::<Box<dyn Stmt + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).list.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::IfStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::IfStmtPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).init.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).init.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).cond.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::BlockStmtPtr((*n.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn Node + Send + Sync>))));;
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).r#else.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).r#else.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CaseClausePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CaseClausePtr>()).unwrap().0.clone();
        walk_list::<Box<dyn Expr + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).list.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
        walk_list::<Box<dyn Stmt + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).body.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::SwitchStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::SwitchStmtPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).init.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).init.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).tag.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).tag.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::BlockStmtPtr((*n.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::TypeSwitchStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::TypeSwitchStmtPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).init.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).init.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).assign.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::BlockStmtPtr((*n.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CommClausePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::CommClausePtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).comm.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).comm.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        walk_list::<Box<dyn Stmt + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).body.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::SelectStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::SelectStmtPtr>()).unwrap().0.clone();
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::BlockStmtPtr((*n.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ForStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ForStmtPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).init.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).init.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).cond.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).cond.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).post.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).post.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::BlockStmtPtr((*n.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::RangeStmtPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::RangeStmtPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).key.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).key.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).value.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::BlockStmtPtr((*n.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn Node + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ImportSpecPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ImportSpecPtr>()).unwrap().0.clone();
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).doc.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CommentGroupPtr((*n.lock().unwrap().as_ref().unwrap()).doc.clone())) as Box<dyn Node + Send + Sync>))));
    };
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).name.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::IdentPtr((*n.lock().unwrap().as_ref().unwrap()).name.clone())) as Box<dyn Node + Send + Sync>))));
    };
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::BasicLitPtr((*n.lock().unwrap().as_ref().unwrap()).path.clone())) as Box<dyn Node + Send + Sync>))));;
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).comment.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CommentGroupPtr((*n.lock().unwrap().as_ref().unwrap()).comment.clone())) as Box<dyn Node + Send + Sync>))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ValueSpecPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ValueSpecPtr>()).unwrap().0.clone();
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).doc.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CommentGroupPtr((*n.lock().unwrap().as_ref().unwrap()).doc.clone())) as Box<dyn Node + Send + Sync>))));
    };
        walk_list::<crate::r#mod::Ident>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).names.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
        if { let __iface_handle = { let __field = (*n.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
    };
        walk_list::<Box<dyn Expr + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).values.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).comment.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CommentGroupPtr((*n.lock().unwrap().as_ref().unwrap()).comment.clone())) as Box<dyn Node + Send + Sync>))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::TypeSpecPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::TypeSpecPtr>()).unwrap().0.clone();
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).doc.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CommentGroupPtr((*n.lock().unwrap().as_ref().unwrap()).doc.clone())) as Box<dyn Node + Send + Sync>))));
    };
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::IdentPtr((*n.lock().unwrap().as_ref().unwrap()).name.clone())) as Box<dyn Node + Send + Sync>))));;
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).type_params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::FieldListPtr((*n.lock().unwrap().as_ref().unwrap()).type_params.clone())) as Box<dyn Node + Send + Sync>))));
    };
        walk(v.clone(), { let __inner: Box<dyn Node + Send + Sync> = (*(*n.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });;
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).comment.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CommentGroupPtr((*n.lock().unwrap().as_ref().unwrap()).comment.clone())) as Box<dyn Node + Send + Sync>))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BadDeclPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::BadDeclPtr>()).unwrap().0.clone();
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::GenDeclPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::GenDeclPtr>()).unwrap().0.clone();
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).doc.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CommentGroupPtr((*n.lock().unwrap().as_ref().unwrap()).doc.clone())) as Box<dyn Node + Send + Sync>))));
    };
        walk_list::<Box<dyn Spec + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).specs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FuncDeclPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FuncDeclPtr>()).unwrap().0.clone();
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).doc.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CommentGroupPtr((*n.lock().unwrap().as_ref().unwrap()).doc.clone())) as Box<dyn Node + Send + Sync>))));
    };
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).recv.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::FieldListPtr((*n.lock().unwrap().as_ref().unwrap()).recv.clone())) as Box<dyn Node + Send + Sync>))));
    };
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::IdentPtr((*n.lock().unwrap().as_ref().unwrap()).name.clone())) as Box<dyn Node + Send + Sync>))));;
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::FuncTypePtr((*n.lock().unwrap().as_ref().unwrap()).r#type.clone())) as Box<dyn Node + Send + Sync>))));;
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).body.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::BlockStmtPtr((*n.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn Node + Send + Sync>))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FilePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FilePtr>()).unwrap().0.clone();
        if { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).doc.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::CommentGroupPtr((*n.lock().unwrap().as_ref().unwrap()).doc.clone())) as Box<dyn Node + Send + Sync>))));
    };
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::IdentPtr((*n.lock().unwrap().as_ref().unwrap()).name.clone())) as Box<dyn Node + Send + Sync>))));;
        walk_list::<Box<dyn Decl + Send + Sync>>(v.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).decls.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::PackagePtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::PackagePtr>()).unwrap().0.clone();
        for (_, f) in { let __range_holder = (*n.lock().unwrap().as_ref().unwrap()).files.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        walk(v.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#mod::FilePtr(f.clone())) as Box<dyn Node + Send + Sync>))));
    };
    } else {
        let n = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("ast.Walk: unexpected node type {}", __go_type_name(n.lock().unwrap().as_ref().unwrap()))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }

        // Comments and fields
        // nothing to do
        // Expressions
        // nothing to do
        // Types
        // Statements
        // nothing to do
        // nothing to do
        // Declarations
        // nothing to do
        // Files and packages
        // don't walk n.Comments - they have been
        // visited already through the individual
        // nodes
    (*v.lock().unwrap().as_mut().unwrap()).visit(Arc::new(Mutex::new(None)));
}