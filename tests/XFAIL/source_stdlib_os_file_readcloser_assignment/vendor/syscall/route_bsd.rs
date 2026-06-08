use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{syscall_bsd::{any_to_sockaddr}, syscall_darwin::{SockaddrDatalink}, syscall_unix::{DARWIN64_BIT, NETBSD32_BIT, Sockaddr, SockaddrInet4, SockaddrInet6}, zerrors_darwin_arm64::{A_F__I_N_E_T, A_F__I_N_E_T6, A_F__L_I_N_K, A_F__U_N_S_P_E_C, E_I_N_V_A_L, R_T_A_X__I_F_P, R_T_A_X__M_A_X, R_T_A__I_F_P}, ztypes_darwin_arm64::{IfMsghdr, IfaMsghdr, RawSockaddr, RawSockaddrAny, RawSockaddrDatalink, RawSockaddrInet4, RawSockaddrInet6, RtMsghdr, SIZEOF_PTR, SIZEOF_SOCKADDR_INET4, SIZEOF_SOCKADDR_INET6}};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const OFFSETOF_INET4: i32 = (std::mem::offset_of!(crate::ztypes_darwin_arm64::RawSockaddrInet4, addr) as i32);
pub(crate) const OFFSETOF_INET6: i32 = (std::mem::offset_of!(crate::ztypes_darwin_arm64::RawSockaddrInet6, addr) as i32);


pub(crate) const ANY_MESSAGE_LEN: i32 = (std::mem::size_of::<anyMessage>() as i32);


/// RoutingMessage represents a routing message.
///
/// Deprecated: Use golang.org/x/net/route instead.
pub trait RoutingMessage: std::fmt::Display + Any {
    fn __go_clone_box_routing_message(&self) -> Box<dyn RoutingMessage + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_routing_message(&self, other: &(dyn RoutingMessage + Send + Sync)) -> bool;
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn RoutingMessage + Send + Sync> {
    fn clone(&self) -> Self {
        RoutingMessage::__go_clone_box_routing_message(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct anyMessage {
    pub msglen: Arc<Mutex<Option<u16>>>,
    pub version: Arc<Mutex<Option<u8>>>,
    pub r#type: Arc<Mutex<Option<u8>>>,
}

impl anyMessage {
    pub fn __go_value_clone(&self) -> Self {
        Self { msglen: { let __guard = self.msglen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, version: { let __guard = self.version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for anyMessage {
    fn default() -> Self {
        Self { msglen: Arc::new(Mutex::new(Some(0))), version: Arc::new(Mutex::new(Some(0))), r#type: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for anyMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.msglen.lock().unwrap().as_ref().unwrap()), (*self.version.lock().unwrap().as_ref().unwrap()), (*self.r#type.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for anyMessage {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Msglen") {
            out.msglen = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Version") {
            out.version = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Type") {
            out.r#type = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// RouteMessage represents a routing message containing routing
/// entries.
///
/// Deprecated: Use golang.org/x/net/route instead.
#[derive(Debug, Clone)]
pub struct RouteMessage {
    pub header: Arc<Mutex<Option<RtMsghdr>>>,
    pub data: Arc<Mutex<Option<Vec<u8>>>>,
}

impl RouteMessage {
    pub fn __go_value_clone(&self) -> Self {
        Self { header: { let __guard = self.header.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: self.data.clone() }
    }
}


impl Default for RouteMessage {
    fn default() -> Self {
        Self { header: Arc::new(Mutex::new(Some(RtMsghdr::default()))), data: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for RouteMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.header.lock().unwrap().as_ref().unwrap()), format_slice(&self.data))
    }
}

impl GoJsonDecode for RouteMessage {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Data") {
            out.data = <Arc<Mutex<Option<Vec<u8>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// InterfaceMessage represents a routing message containing
/// network interface entries.
///
/// Deprecated: Use golang.org/x/net/route instead.
#[derive(Debug, Clone)]
pub struct InterfaceMessage {
    pub header: Arc<Mutex<Option<IfMsghdr>>>,
    pub data: Arc<Mutex<Option<Vec<u8>>>>,
}

impl InterfaceMessage {
    pub fn __go_value_clone(&self) -> Self {
        Self { header: { let __guard = self.header.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: self.data.clone() }
    }
}


impl Default for InterfaceMessage {
    fn default() -> Self {
        Self { header: Arc::new(Mutex::new(Some(IfMsghdr::default()))), data: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for InterfaceMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.header.lock().unwrap().as_ref().unwrap()), format_slice(&self.data))
    }
}

impl GoJsonDecode for InterfaceMessage {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Data") {
            out.data = <Arc<Mutex<Option<Vec<u8>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// InterfaceAddrMessage represents a routing message containing
/// network interface address entries.
///
/// Deprecated: Use golang.org/x/net/route instead.
#[derive(Debug, Clone)]
pub struct InterfaceAddrMessage {
    pub header: Arc<Mutex<Option<IfaMsghdr>>>,
    pub data: Arc<Mutex<Option<Vec<u8>>>>,
}

impl InterfaceAddrMessage {
    pub fn __go_value_clone(&self) -> Self {
        Self { header: { let __guard = self.header.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: self.data.clone() }
    }
}


impl Default for InterfaceAddrMessage {
    fn default() -> Self {
        Self { header: Arc::new(Mutex::new(Some(IfaMsghdr::default()))), data: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for InterfaceAddrMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.header.lock().unwrap().as_ref().unwrap()), format_slice(&self.data))
    }
}

impl GoJsonDecode for InterfaceAddrMessage {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Data") {
            out.data = <Arc<Mutex<Option<Vec<u8>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub(crate) static freebsdConfArch: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static minRoutingSockaddrLen: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *freebsdConfArch.lock().unwrap() = Some(String::new());
    *minRoutingSockaddrLen.lock().unwrap() = Some(0);
    *minRoutingSockaddrLen.lock().unwrap() = Some(rsa_align_of(Arc::new(Mutex::new(Some(0)))));
}


pub(crate) fn __go_zero_globals() {
    *freebsdConfArch.lock().unwrap() = Some(String::new());
    *minRoutingSockaddrLen.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_1() {
    *minRoutingSockaddrLen.lock().unwrap() = Some(rsa_align_of(Arc::new(Mutex::new(Some(0)))));
}


impl RouteMessage {
    pub fn sockaddr(&self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut sas: Arc<Mutex<Option<[Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>; 8]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let mut b = Arc::new(Mutex::new(Some({ let __seq_holder = self.data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        let mut family = Arc::new(Mutex::new(Some(A_F__U_N_S_P_E_C as u8)));
        let mut i = Arc::new(Mutex::new(Some(0 as u64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = R_T_A_X__M_A_X as u64; __tmp_x < __tmp_y } && { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*minRoutingSockaddrLen.lock().unwrap().as_ref().unwrap()) as i32); __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __tmp_x = (*(*self.header.lock().unwrap().as_ref().unwrap()).addrs.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (1 as i32); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        let mut rsa: GoPtr<crate::ztypes_darwin_arm64::RawSockaddr> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let _switch_val = { let __v = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.family.clone()); __ptr_value }.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (A_F__L_I_N_K as u8) {
            let (mut sa, mut err) = parse_sockaddr_link(b.clone());
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
            (*sas.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = Arc::new(Mutex::new(Some(Box::new(crate::syscall_darwin::SockaddrDatalinkPtr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>)));
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (rsa_align_of(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))))) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
        } else if _switch_val == (A_F__I_N_E_T as u8) || _switch_val == (A_F__I_N_E_T6 as u8) {
            let (mut sa, mut err) = parse_sockaddr_inet(b.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.family.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
            (*sas.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = sa.clone();
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (rsa_align_of(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))))) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
            { let new_val = { let __selector_holder = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.family.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *family.lock().unwrap() = Some(new_val); };
        } else {
            let (mut sa, mut err) = parse_network_layer_addr(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = family.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
            (*sas.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = sa.clone();
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (rsa_align_of(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as i32))))) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return (Arc::new(Mutex::new(Some({ let __seq_holder = sas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(None)));
    }
}

impl RoutingMessage for RouteMessage {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        RouteMessage::sockaddr(self)
    }
    fn __go_clone_box_routing_message(&self) -> Box<dyn RoutingMessage + Send + Sync> {
        Box::new(self.clone()) as Box<dyn RoutingMessage + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_routing_message(&self, other: &(dyn RoutingMessage + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RouteMessage>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct RouteMessagePtr(pub Arc<Mutex<Option<RouteMessage>>>);

impl std::fmt::Display for RouteMessagePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl RoutingMessage for RouteMessagePtr {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        RouteMessage::sockaddr(__recv)
    }
    fn __go_clone_box_routing_message(&self) -> Box<dyn RoutingMessage + Send + Sync> {
        Box::new(self.clone()) as Box<dyn RoutingMessage + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_routing_message(&self, other: &(dyn RoutingMessage + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RouteMessagePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl InterfaceMessage {
    pub fn sockaddr(&self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut sas: Arc<Mutex<Option<[Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>; 8]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        if { let __tmp_x = { let __tmp_x = (*(*self.header.lock().unwrap().as_ref().unwrap()).addrs.lock().unwrap().as_ref().unwrap()); let __tmp_y = R_T_A__I_F_P as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
        let (mut sa, mut err) = parse_sockaddr_link(Arc::new(Mutex::new(Some({ let __seq_holder = self.data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
        (*sas.lock().unwrap().as_mut().unwrap())[(R_T_A_X__I_F_P) as usize] = Arc::new(Mutex::new(Some(Box::new(crate::syscall_darwin::SockaddrDatalinkPtr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>)));
        return (Arc::new(Mutex::new(Some({ let __seq_holder = sas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(None)));
    }
}

impl RoutingMessage for InterfaceMessage {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        InterfaceMessage::sockaddr(self)
    }
    fn __go_clone_box_routing_message(&self) -> Box<dyn RoutingMessage + Send + Sync> {
        Box::new(self.clone()) as Box<dyn RoutingMessage + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_routing_message(&self, other: &(dyn RoutingMessage + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfaceMessage>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct InterfaceMessagePtr(pub Arc<Mutex<Option<InterfaceMessage>>>);

impl std::fmt::Display for InterfaceMessagePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl RoutingMessage for InterfaceMessagePtr {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        InterfaceMessage::sockaddr(__recv)
    }
    fn __go_clone_box_routing_message(&self) -> Box<dyn RoutingMessage + Send + Sync> {
        Box::new(self.clone()) as Box<dyn RoutingMessage + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_routing_message(&self, other: &(dyn RoutingMessage + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfaceMessagePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl InterfaceAddrMessage {
    pub fn sockaddr(&self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut sas: Arc<Mutex<Option<[Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>; 8]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let mut b = Arc::new(Mutex::new(Some({ let __seq_holder = self.data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        let mut family = Arc::new(Mutex::new(Some(A_F__U_N_S_P_E_C as u8)));
        let mut i = Arc::new(Mutex::new(Some(0 as u64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = R_T_A_X__M_A_X as u64; __tmp_x < __tmp_y } && { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*minRoutingSockaddrLen.lock().unwrap().as_ref().unwrap()) as i32); __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __tmp_x = (*(*self.header.lock().unwrap().as_ref().unwrap()).addrs.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (1 as i32); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        let mut rsa: GoPtr<crate::ztypes_darwin_arm64::RawSockaddr> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let _switch_val = { let __v = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.family.clone()); __ptr_value }.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (A_F__L_I_N_K as u8) {
            let (mut sa, mut err) = parse_sockaddr_link(b.clone());
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
            (*sas.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = Arc::new(Mutex::new(Some(Box::new(crate::syscall_darwin::SockaddrDatalinkPtr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>)));
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (rsa_align_of(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))))) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
        } else if _switch_val == (A_F__I_N_E_T as u8) || _switch_val == (A_F__I_N_E_T6 as u8) {
            let (mut sa, mut err) = parse_sockaddr_inet(b.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.family.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
            (*sas.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = sa.clone();
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (rsa_align_of(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))))) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
            { let new_val = { let __selector_holder = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.family.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *family.lock().unwrap() = Some(new_val); };
        } else {
            let (mut sa, mut err) = parse_network_layer_addr(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = family.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
            (*sas.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = sa.clone();
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (rsa_align_of(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as i32))))) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return (Arc::new(Mutex::new(Some({ let __seq_holder = sas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(None)));
    }
}

impl RoutingMessage for InterfaceAddrMessage {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        InterfaceAddrMessage::sockaddr(self)
    }
    fn __go_clone_box_routing_message(&self) -> Box<dyn RoutingMessage + Send + Sync> {
        Box::new(self.clone()) as Box<dyn RoutingMessage + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_routing_message(&self, other: &(dyn RoutingMessage + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfaceAddrMessage>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct InterfaceAddrMessagePtr(pub Arc<Mutex<Option<InterfaceAddrMessage>>>);

impl std::fmt::Display for InterfaceAddrMessagePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl RoutingMessage for InterfaceAddrMessagePtr {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        InterfaceAddrMessage::sockaddr(__recv)
    }
    fn __go_clone_box_routing_message(&self) -> Box<dyn RoutingMessage + Send + Sync> {
        Box::new(self.clone()) as Box<dyn RoutingMessage + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_routing_message(&self, other: &(dyn RoutingMessage + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfaceAddrMessagePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// Round the length of a raw sockaddr up to align it properly.
pub fn rsa_align_of(salen: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut salign = Arc::new(Mutex::new(Some(SIZEOF_PTR)));
    if DARWIN64_BIT {
                // Darwin kernels require 32-bit aligned access to
                // routing facilities.
        { let new_val = 4; *salign.lock().unwrap() = Some(new_val); };
    } else if NETBSD32_BIT {
        { let new_val = 8; *salign.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = "darwin".to_string(); let __tmp_y = "freebsd".to_string(); __tmp_x == __tmp_y } {
        if { let __tmp_x = (*freebsdConfArch.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "amd64".to_string(); __tmp_x == __tmp_y } {
        { let new_val = 8; *salign.lock().unwrap() = Some(new_val); };
    }
    }
        // Darwin kernels require 32-bit aligned access to
        // routing facilities.
        // NetBSD 6 and beyond kernels require 64-bit aligned
        // access to routing facilities.
        // In the case of kern.supported_archs="amd64 i386",
        // we need to know the underlying kernel's
        // architecture because the alignment for routing
        // facilities are set at the build time of the kernel.
    if { let __tmp_x = { let __v = (*salen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return { let __v = (*salign.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    return { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*salen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*salign.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x - __tmp_y }); let __tmp_y = !({ let __tmp_x = { let __v = (*salign.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); __tmp_x & __tmp_y };
}

/// parseSockaddrLink parses b as a datalink socket address.
pub fn parse_sockaddr_link(b: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<crate::syscall_darwin::SockaddrDatalink>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 8; __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
    let (mut sa, _, mut err) = parse_link_layer_addr(Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (4) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
    let mut rsa: GoPtr<crate::ztypes_darwin_arm64::RawSockaddrDatalink> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    { let new_val = { let __selector_holder = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };
    { let new_val = { let __selector_holder = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.family.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).family.lock().unwrap() = Some(new_val); };
    { let new_val = { let __selector_holder = { let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.index.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
    return (sa.clone(), Arc::new(Mutex::new(None)));
}

/// parseLinkLayerAddr parses b as a datalink socket address in
/// conventional BSD kernel form.
pub fn parse_link_layer_addr(mut b: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<crate::syscall_darwin::SockaddrDatalink>>>, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // The encoding looks like the following:
        // +----------------------------+
        // | Type             (1 octet) |
        // +----------------------------+
        // | Name length      (1 octet) |
        // +----------------------------+
        // | Address length   (1 octet) |
        // +----------------------------+
        // | Selector length  (1 octet) |
        // +----------------------------+
        // | Data            (variable) |
        // +----------------------------+
    type linkLayerAddr = AnonymousStruct1;
    let mut lla: GoPtr<linkLayerAddr> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let mut l = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __tmp_x = 4; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = lla.with_mut(|__ptr_value| __ptr_value.nlen.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = lla.with_mut(|__ptr_value| __ptr_value.alen.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = lla.with_mut(|__ptr_value| __ptr_value.slen.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
    if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(None)), 0, Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (4) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
    let mut sa = Arc::new(Mutex::new(Some(SockaddrDatalink { r#type: Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = lla.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), nlen: Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = lla.with_mut(|__ptr_value| __ptr_value.nlen.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), alen: Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = lla.with_mut(|__ptr_value| __ptr_value.alen.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), slen: Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = lla.with_mut(|__ptr_value| __ptr_value.slen.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), len: Default::default(), family: Default::default(), index: Default::default(), data: Default::default(), raw: Arc::new(Mutex::new(Some(RawSockaddrDatalink::default()))) })));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = 12; let __tmp_y = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x - __tmp_y }; __tmp_x < __tmp_y } {
        (*(*sa.lock().unwrap().as_ref().unwrap()).data.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as i8))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return (sa.clone(), rsa_align_of(Arc::new(Mutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), Arc::new(Mutex::new(None)));
}

/// parseSockaddrInet parses b as an internet socket address.
pub fn parse_sockaddr_inet(b: Arc<Mutex<Option<Vec<u8>>>>, family: Arc<Mutex<Option<u8>>>) -> (Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    { let _switch_val = { let __v = (*family.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (A_F__I_N_E_T as u8) {
            if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 16; __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
            let mut rsa: GoPtr<crate::ztypes_darwin_arm64::RawSockaddrAny> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            return any_to_sockaddr(rsa.clone());
        } else if _switch_val == (A_F__I_N_E_T6 as u8) {
            if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 28; __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
            let mut rsa: GoPtr<crate::ztypes_darwin_arm64::RawSockaddrAny> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            return any_to_sockaddr(rsa.clone());
        } else {
            return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
        }
    }
}

/// parseNetworkLayerAddr parses b as an internet socket address in
/// conventional BSD kernel form.
pub fn parse_network_layer_addr(b: Arc<Mutex<Option<Vec<u8>>>>, family: Arc<Mutex<Option<u8>>>) -> (Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // The encoding looks similar to the NLRI encoding.
        // +----------------------------+
        // | Length           (1 octet) |
        // +----------------------------+
        // | Address prefix  (variable) |
        // +----------------------------+
        //
        // The differences between the kernel form and the NLRI
        // encoding are:
        //
        // - The length field of the kernel form indicates the prefix
        //   length in bytes, not in bits
        //
        // - In the kernel form, zero value of the length field
        //   doesn't mean 0.0.0.0/0 or ::/0
        //
        // - The kernel form appends leading bytes to the prefix field
        //   to make the <length, prefix> tuple to be conformed with
        //   the routing message boundary
    let mut l = Arc::new(Mutex::new(Some(rsa_align_of(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as i32)))) as i32)));
    if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }

        // Don't reorder case expressions.
        // The case expressions for IPv6 must come first.
    if { let __tmp_x = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = SIZEOF_SOCKADDR_INET6 as u8; __tmp_x == __tmp_y } {
            let mut sa = Arc::new(Mutex::new(Some(SockaddrInet6 { port: Arc::new(Mutex::new(Some(0))), zone_id: Arc::new(Mutex::new(Some(0))), addr: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), raw: Arc::new(Mutex::new(Some(Default::default()))) })));
            { let _dst_start = 0; let _dst_len = (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (OFFSETOF_INET6) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
            return (Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::SockaddrInet6Ptr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>))), Arc::new(Mutex::new(None)));
        } else if { let __tmp_x = { let __v = (*family.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = A_F__I_N_E_T6 as u8; __tmp_x == __tmp_y } {
            let mut sa = Arc::new(Mutex::new(Some(SockaddrInet6 { port: Arc::new(Mutex::new(Some(0))), zone_id: Arc::new(Mutex::new(Some(0))), addr: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), raw: Arc::new(Mutex::new(Some(Default::default()))) })));
            if { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; let __tmp_y = 8; __tmp_x < __tmp_y } {
        { let _dst_start = 0; let _dst_len = (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = ({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    } else {
        { let _dst_start = 0; let _dst_len = (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x - __tmp_y }) as usize; let __high = ({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    }
            return (Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::SockaddrInet6Ptr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>))), Arc::new(Mutex::new(None)));
        } else if { let __tmp_x = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = SIZEOF_SOCKADDR_INET4 as u8; __tmp_x == __tmp_y } {
            let mut sa = Arc::new(Mutex::new(Some(SockaddrInet4 { port: Arc::new(Mutex::new(Some(0))), addr: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), raw: Arc::new(Mutex::new(Some(Default::default()))) })));
            { let _dst_start = 0; let _dst_len = (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (OFFSETOF_INET4) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
            return (Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::SockaddrInet4Ptr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>))), Arc::new(Mutex::new(None)));
        } else {
            let mut sa = Arc::new(Mutex::new(Some(SockaddrInet4 { port: Arc::new(Mutex::new(Some(0))), addr: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), raw: Arc::new(Mutex::new(Some(Default::default()))) })));
            if { let __tmp_x = { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; let __tmp_y = 4; __tmp_x < __tmp_y } {
        { let _dst_start = 0; let _dst_len = (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = ({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    } else {
        { let _dst_start = 0; let _dst_len = (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x - __tmp_y }) as usize; let __high = ({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    }
            return (Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::SockaddrInet4Ptr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>))), Arc::new(Mutex::new(None)));
        }
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub r#type: Arc<Mutex<Option<u8>>>,
    pub nlen: Arc<Mutex<Option<u8>>>,
    pub alen: Arc<Mutex<Option<u8>>>,
    pub slen: Arc<Mutex<Option<u8>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nlen: { let __guard = self.nlen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alen: { let __guard = self.alen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, slen: { let __guard = self.slen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { r#type: Arc::new(Mutex::new(Some(0))), nlen: Arc::new(Mutex::new(Some(0))), alen: Arc::new(Mutex::new(Some(0))), slen: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.nlen.lock().unwrap().as_ref().unwrap()), (*self.alen.lock().unwrap().as_ref().unwrap()), (*self.slen.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Type") {
            out.r#type = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Nlen") {
            out.nlen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Alen") {
            out.alen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Slen") {
            out.slen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for anyMessage {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RouteMessage {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for InterfaceMessage {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for InterfaceAddrMessage {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
