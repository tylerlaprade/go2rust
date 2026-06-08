use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
};

use crate::{
    map_select_swiss::{mapType},
    r#type::{ArrayType, ChanDir, FuncType, InterfaceType, Kind, Method, StructType, Type, UncommonType},
};

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
        let __go_clone_0_0 = { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.key.clone();
        let __go_clone_2_0 = self.elem.clone();
        let __go_clone_3_0 = self.group.clone();
        let __go_clone_4_0 = self.hasher.clone();
        let __go_clone_5_0 = { let __guard = self.group_size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.slot_size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.elem_off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r#type: __go_clone_0_0,
            key: __go_clone_1_0,
            elem: __go_clone_2_0,
            group: __go_clone_3_0,
            hasher: __go_clone_4_0,
            group_size: __go_clone_5_0,
            slot_size: __go_clone_6_0,
            elem_off: __go_clone_7_0,
            flags: __go_clone_8_0,
        }
    }
}


impl Default for SwissMapType {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Type::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        let __go_default_4_0 = Arc::new(Mutex::new(None));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            r#type: __go_default_0_0,
            key: __go_default_1_0,
            elem: __go_default_2_0,
            group: __go_default_3_0,
            hasher: __go_default_4_0,
            group_size: __go_default_5_0,
            slot_size: __go_default_6_0,
            elem_off: __go_default_7_0,
            flags: __go_default_8_0,
        }
    }
}

impl std::fmt::Display for SwissMapType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.key.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", { let __guard = self.elem.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", { let __guard = self.group.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_4 = format!("{}", "<func>");
        let __go_fmt_5 = format!("{}", (*self.group_size.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.slot_size.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.elem_off.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.flags.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8)
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

    pub fn array_type(&self) -> GoPtr<crate::r#type::ArrayType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.array_type()
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<crate::r#type::ChanDir>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chan_dir()
    }

    pub fn common(&self) -> Arc<Mutex<Option<crate::r#type::Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.common()
    }

    pub fn elem(&self) -> Arc<Mutex<Option<crate::r#type::Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.elem()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<crate::r#type::Method>>>> {
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

    pub fn func_type(&self) -> GoPtr<crate::r#type::FuncType> {
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

    pub fn interface_type(&self) -> GoPtr<crate::r#type::InterfaceType> {
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

    pub fn key(&self) -> Arc<Mutex<Option<crate::r#type::Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.key()
    }

    pub fn kind(&self) -> Arc<Mutex<Option<crate::r#type::Kind>>> {
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

    pub fn map_type(&self) -> GoPtr<SwissMapType> {
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

    pub fn struct_type(&self) -> GoPtr<crate::r#type::StructType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.struct_type()
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<crate::r#type::UncommonType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.uncommon()
    }
}

impl GoValueClone for SwissMapType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
