use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped, go_lookup_embedded_owner, go_register_embedded_owner};

use crate::r#mod::*;
use crate::abi_arm64::*;
use crate::compiletype::*;
use crate::escape::*;
use crate::funcpc::*;
use crate::iface::*;
use crate::map_noswiss::*;
use crate::map_select_swiss::*;
use crate::rangefuncconsts::*;
use crate::runtime::*;
use crate::stack::*;
use crate::switch::*;
use crate::symtab::*;
use crate::r#type::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const SWISS_MAP_GROUP_SLOTS_BITS: i32 = 3;
pub const SWISS_MAP_GROUP_SLOTS: i32 = 1 << SWISS_MAP_GROUP_SLOTS_BITS;
pub const SWISS_MAP_MAX_KEY_BYTES: i32 = 128;
pub const SWISS_MAP_MAX_ELEM_BYTES: i32 = 128;
pub(crate) const CTRL_EMPTY: i32 = 0b10000000;
pub(crate) const BITSET_L_S_B: i64 = 0x0101010101010101;
pub const SWISS_MAP_CTRL_EMPTY: u64 = BITSET_L_S_B as u64 * (CTRL_EMPTY as u64);


pub const SWISS_MAP_NEED_KEY_UPDATE: i32 = 1 << 0;
pub const SWISS_MAP_HASH_MIGHT_PANIC: i32 = 1 << 1;
pub const SWISS_MAP_INDIRECT_KEY: i32 = 1 << 2;
pub const SWISS_MAP_INDIRECT_ELEM: i32 = 1 << 3;


#[derive(Clone)]
pub struct SwissMapType {
    pub r#type: Arc<Mutex<Option<Type>>>,
    pub key: Arc<Mutex<Option<Type>>>,
    pub elem: Arc<Mutex<Option<Type>>>,
    pub group: Arc<Mutex<Option<Type>>>,
    pub hasher: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> usize + Send + Sync>>>>,
    pub group_size: Arc<Mutex<Option<usize>>>,
    pub slot_size: Arc<Mutex<Option<usize>>>,
    pub elem_off: Arc<Mutex<Option<usize>>>,
    pub flags: Arc<Mutex<Option<u32>>>,
}

impl SwissMapType {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, key: self.key.clone(), elem: self.elem.clone(), group: self.group.clone(), hasher: self.hasher.clone(), group_size: { let __guard = self.group_size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, slot_size: { let __guard = self.slot_size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, elem_off: { let __guard = self.elem_off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flags: { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for SwissMapType {
    fn default() -> Self {
        Self { r#type: Arc::new(Mutex::new(Some(Type::default()))), key: Arc::new(Mutex::new(None)), elem: Arc::new(Mutex::new(None)), group: Arc::new(Mutex::new(None)), hasher: Arc::new(Mutex::new(None)), group_size: Arc::new(Mutex::new(Some(0))), slot_size: Arc::new(Mutex::new(Some(0))), elem_off: Arc::new(Mutex::new(Some(0))), flags: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for SwissMapType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {}}}", (*self.r#type.lock().unwrap().as_ref().unwrap()), { let __guard = self.key.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.elem.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.group.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, "<func>", (*self.group_size.lock().unwrap().as_ref().unwrap()), (*self.slot_size.lock().unwrap().as_ref().unwrap()), (*self.elem_off.lock().unwrap().as_ref().unwrap()), (*self.flags.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for SwissMapType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("GroupSize") {
            out.group_size = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("SlotSize") {
            out.slot_size = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ElemOff") {
            out.elem_off = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Flags") {
            out.flags = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl SwissMapType {
    pub fn need_key_update(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = (*self.flags.lock().unwrap().as_ref().unwrap()); let __tmp_y = SWISS_MAP_NEED_KEY_UPDATE as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
    }

    pub fn hash_might_panic(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = (*self.flags.lock().unwrap().as_ref().unwrap()); let __tmp_y = SWISS_MAP_HASH_MIGHT_PANIC as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
    }

    pub fn indirect_key(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = (*self.flags.lock().unwrap().as_ref().unwrap()); let __tmp_y = SWISS_MAP_INDIRECT_KEY as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
    }

    pub fn indirect_elem(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = (*self.flags.lock().unwrap().as_ref().unwrap()); let __tmp_y = SWISS_MAP_INDIRECT_ELEM as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
    }

    pub fn align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.align()
    }

    pub fn array_type(&self) -> Arc<Mutex<Option<ArrayType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.array_type()
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chan_dir()
    }

    pub fn common(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.common()
    }

    pub fn elem(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.elem()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported_methods()
    }

    pub fn field_align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.field_align()
    }

    pub fn func_type(&self) -> Arc<Mutex<Option<FuncType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.func_type()
    }

    pub fn gc_slice(&self, begin: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.gc_slice(begin, end)
    }

    pub fn has_name(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.has_name()
    }

    pub fn iface_indir(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.iface_indir()
    }

    pub fn interface_type(&self) -> Arc<Mutex<Option<InterfaceType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.interface_type()
    }

    pub fn is_direct_iface(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_direct_iface()
    }

    pub fn key(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.key()
    }

    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.kind()
    }

    pub fn len(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn map_type(&self) -> Arc<Mutex<Option<SwissMapType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.map_type()
    }

    pub fn num_method(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.num_method()
    }

    pub fn pointers(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pointers()
    }

    pub fn size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.size()
    }

    pub fn struct_type(&self) -> Arc<Mutex<Option<StructType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.struct_type()
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<UncommonType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.uncommon()
    }
}