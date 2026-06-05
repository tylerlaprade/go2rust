use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_register_embedded_owner, go_strconv_format_float, go_strconv_format_int};

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

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// An objset is a set of objects identified by their unique id.
/// The zero value for objset is a ready-to-use empty objset.
#[derive(Clone, Default)]
pub struct objset(pub Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>>);

impl Display for objset {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


impl objset {
    /// insert attempts to insert an object obj into objset s.
    /// If s already contains an alternative object alt with
    /// the same name, insert leaves s unchanged and returns alt.
    /// Otherwise it inserts obj and returns nil.
    pub fn insert(&mut self, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> {
        let mut id = (*obj.lock().unwrap().as_ref().unwrap()).id();
        {
        let mut alt = { let __map = { let __map_holder = self.0.clone().clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*id.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if (*alt.lock().unwrap()).is_some() {
            return alt.clone();;
        }
    }
        if { let __map_holder = self.0.clone(); let __map_guard = __map_holder.lock().unwrap(); (*__map_guard).is_none() } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>::new()))); *self = objset(new_val); };
    }
        { let __map_key = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = obj.clone(); (*self.0.clone().lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        return Arc::new(Mutex::new(None));
    }
}