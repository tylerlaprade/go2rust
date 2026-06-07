use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload};

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
use std::sync::{Arc, Mutex};

pub(crate) static gccgoArchSizes: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<String, Arc<Mutex<Option<crate::sizes::StdSizes>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *gccgoArchSizes.lock().unwrap() = Some(BTreeMap::new());
    {
        fn __go_init_gccgoArchSizes_map_chunk_0(__go_map: &mut BTreeMap<String, Arc<Mutex<Option<crate::sizes::StdSizes>>>>) {
            __go_map.insert("386".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(4 as i64))), ..Default::default() }))).clone());
            __go_map.insert("alpha".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("amd64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("amd64p32".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("arm".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("armbe".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("arm64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("arm64be".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("ia64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("loong64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("m68k".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(2 as i64))), ..Default::default() }))).clone());
            __go_map.insert("mips".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("mipsle".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("mips64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("mips64le".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("mips64p32".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        }
        fn __go_init_gccgoArchSizes_map_chunk_1(__go_map: &mut BTreeMap<String, Arc<Mutex<Option<crate::sizes::StdSizes>>>>) {
            __go_map.insert("mips64p32le".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("nios2".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("ppc".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("ppc64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("ppc64le".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("riscv".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("riscv64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("s390".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("s390x".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("sh".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("shbe".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("sparc".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("sparc64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("wasm".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        }
        let mut __go_map = BTreeMap::<String, Arc<Mutex<Option<crate::sizes::StdSizes>>>>::new();
        __go_init_gccgoArchSizes_map_chunk_0(&mut __go_map);
        __go_init_gccgoArchSizes_map_chunk_1(&mut __go_map);
        *gccgoArchSizes.lock().unwrap() = Some(__go_map);
    }
}


pub(crate) fn __go_zero_globals() {
    *gccgoArchSizes.lock().unwrap() = Some(BTreeMap::new());
}


pub(crate) fn __go_init_order_5() {
    {
        fn __go_init_gccgoArchSizes_map_chunk_0(__go_map: &mut BTreeMap<String, Arc<Mutex<Option<crate::sizes::StdSizes>>>>) {
            __go_map.insert("386".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(4 as i64))), ..Default::default() }))).clone());
            __go_map.insert("alpha".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("amd64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("amd64p32".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("arm".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("armbe".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("arm64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("arm64be".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("ia64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("loong64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("m68k".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(2 as i64))), ..Default::default() }))).clone());
            __go_map.insert("mips".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("mipsle".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("mips64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("mips64le".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("mips64p32".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        }
        fn __go_init_gccgoArchSizes_map_chunk_1(__go_map: &mut BTreeMap<String, Arc<Mutex<Option<crate::sizes::StdSizes>>>>) {
            __go_map.insert("mips64p32le".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("nios2".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("ppc".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("ppc64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("ppc64le".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("riscv".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("riscv64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("s390".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("s390x".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("sh".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("shbe".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("sparc".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(4 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("sparc64".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
            __go_map.insert("wasm".to_string(), Arc::new(Mutex::new(Some(crate::sizes::StdSizes { word_size: Arc::new(Mutex::new(Some(8 as i64))), max_align: Arc::new(Mutex::new(Some(8 as i64))), ..Default::default() }))).clone());
        }
        let mut __go_map = BTreeMap::<String, Arc<Mutex<Option<crate::sizes::StdSizes>>>>::new();
        __go_init_gccgoArchSizes_map_chunk_0(&mut __go_map);
        __go_init_gccgoArchSizes_map_chunk_1(&mut __go_map);
        *gccgoArchSizes.lock().unwrap() = Some(__go_map);
    }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
