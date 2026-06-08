use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::badlinkname_unix::*;
use crate::bpf_bsd::*;
use crate::dirent::*;
use crate::env_unix::*;
use crate::exec_libc2::*;
use crate::exec_unix::*;
use crate::flock_bsd::*;
use crate::forkpipe::*;
use crate::linkname_bsd::*;
use crate::linkname_darwin::*;
use crate::linkname_libc::*;
use crate::linkname_unix::*;
use crate::net::*;
use crate::rlimit::*;
use crate::rlimit_darwin::*;
use crate::route_bsd::*;
use crate::sockcmsg_unix::*;
use crate::sockcmsg_unix_other::*;
use crate::r#mod::*;
use crate::syscall_bsd::*;
use crate::syscall_darwin::*;
use crate::syscall_darwin_arm64::*;
use crate::syscall_unix::*;
use crate::time_nofake::*;
use crate::timestruct::*;
use crate::zerrors_darwin_arm64::*;
use crate::zsyscall_darwin_arm64::*;
use crate::zsysnum_darwin_arm64::*;
use crate::ztypes_darwin_arm64::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// InterfaceMulticastAddrMessage represents a routing message
/// containing network interface address entries.
///
/// Deprecated: Use golang.org/x/net/route instead.
#[derive(Debug, Clone)]
pub struct InterfaceMulticastAddrMessage {
    pub header: Arc<Mutex<Option<IfmaMsghdr2>>>,
    pub data: Arc<Mutex<Option<Vec<u8>>>>,
}

impl InterfaceMulticastAddrMessage {
    pub fn __go_value_clone(&self) -> Self {
        Self { header: { let __guard = self.header.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: self.data.clone() }
    }
}


impl Default for InterfaceMulticastAddrMessage {
    fn default() -> Self {
        Self { header: Arc::new(Mutex::new(Some(IfmaMsghdr2::default()))), data: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for InterfaceMulticastAddrMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.header.lock().unwrap().as_ref().unwrap()), format_slice(&self.data))
    }
}

impl GoJsonDecode for InterfaceMulticastAddrMessage {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Data") {
            out.data = <Arc<Mutex<Option<Vec<u8>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl crate::route_bsd::anyMessage {
    pub fn to_routing_message(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn RoutingMessage + Send + Sync>>>> {
        { let _switch_val = { let __v = self.r#type.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (R_T_M__A_D_D as u8) || _switch_val == (R_T_M__D_E_L_E_T_E as u8) || _switch_val == (R_T_M__C_H_A_N_G_E as u8) || _switch_val == (R_T_M__G_E_T as u8) || _switch_val == (R_T_M__L_O_S_I_N_G as u8) || _switch_val == (R_T_M__R_E_D_I_R_E_C_T as u8) || _switch_val == (R_T_M__M_I_S_S as u8) || _switch_val == (R_T_M__L_O_C_K as u8) || _switch_val == (R_T_M__R_E_S_O_L_V_E as u8) {
            let mut p: GoPtr<crate::route_bsd::RouteMessage> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            return Arc::new(Mutex::new(Some(Box::new(crate::route_bsd::RouteMessagePtr(Arc::new(Mutex::new(Some(RouteMessage { header: Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.header.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), data: Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (SIZEOF_RT_MSGHDR) as usize; let __high = (*self.msglen.clone().lock().unwrap().as_ref().unwrap()) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), ..Default::default() }))).clone())) as Box<dyn RoutingMessage + Send + Sync>)));
        } else if _switch_val == (R_T_M__I_F_I_N_F_O as u8) {
            let mut p: GoPtr<crate::route_bsd::InterfaceMessage> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            return Arc::new(Mutex::new(Some(Box::new(crate::route_bsd::InterfaceMessagePtr(Arc::new(Mutex::new(Some(InterfaceMessage { header: Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.header.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), data: Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (SIZEOF_IF_MSGHDR) as usize; let __high = (*self.msglen.clone().lock().unwrap().as_ref().unwrap()) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), ..Default::default() }))).clone())) as Box<dyn RoutingMessage + Send + Sync>)));
        } else if _switch_val == (R_T_M__N_E_W_A_D_D_R as u8) || _switch_val == (R_T_M__D_E_L_A_D_D_R as u8) {
            let mut p: GoPtr<crate::route_bsd::InterfaceAddrMessage> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            return Arc::new(Mutex::new(Some(Box::new(crate::route_bsd::InterfaceAddrMessagePtr(Arc::new(Mutex::new(Some(InterfaceAddrMessage { header: Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.header.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), data: Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (SIZEOF_IFA_MSGHDR) as usize; let __high = (*self.msglen.clone().lock().unwrap().as_ref().unwrap()) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), ..Default::default() }))).clone())) as Box<dyn RoutingMessage + Send + Sync>)));
        } else if _switch_val == (R_T_M__N_E_W_M_A_D_D_R2 as u8) || _switch_val == (R_T_M__D_E_L_M_A_D_D_R as u8) {
            let mut p: GoPtr<InterfaceMulticastAddrMessage> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            return Arc::new(Mutex::new(Some(Box::new(InterfaceMulticastAddrMessagePtr(Arc::new(Mutex::new(Some(InterfaceMulticastAddrMessage { header: Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.header.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), data: Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (SIZEOF_IFMA_MSGHDR2) as usize; let __high = (*self.msglen.clone().lock().unwrap().as_ref().unwrap()) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), ..Default::default() }))).clone())) as Box<dyn RoutingMessage + Send + Sync>)));
        }
    }
        return Arc::new(Mutex::new(None));
    }
}

impl InterfaceMulticastAddrMessage {
    pub fn sockaddr(&self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut sas: Arc<Mutex<Option<[Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>; 8]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let mut b = Arc::new(Mutex::new(Some({ let __seq_holder = self.data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
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
        } else {
            let (mut sa, mut l, mut err) = parse_link_layer_addr(b.clone());
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
            (*sas.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = Arc::new(Mutex::new(Some(Box::new(crate::syscall_darwin::SockaddrDatalinkPtr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>)));
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (l) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return (Arc::new(Mutex::new(Some({ let __seq_holder = sas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(None)));
    }
}

impl RoutingMessage for InterfaceMulticastAddrMessage {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        InterfaceMulticastAddrMessage::sockaddr(self)
    }
    fn __go_clone_box_routing_message(&self) -> Box<dyn RoutingMessage + Send + Sync> {
        Box::new(self.clone()) as Box<dyn RoutingMessage + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_routing_message(&self, other: &(dyn RoutingMessage + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfaceMulticastAddrMessage>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct InterfaceMulticastAddrMessagePtr(pub Arc<Mutex<Option<InterfaceMulticastAddrMessage>>>);

impl std::fmt::Display for InterfaceMulticastAddrMessagePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl RoutingMessage for InterfaceMulticastAddrMessagePtr {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        InterfaceMulticastAddrMessage::sockaddr(__recv)
    }
    fn __go_clone_box_routing_message(&self) -> Box<dyn RoutingMessage + Send + Sync> {
        Box::new(self.clone()) as Box<dyn RoutingMessage + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_routing_message(&self, other: &(dyn RoutingMessage + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<InterfaceMulticastAddrMessagePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl GoValueClone for InterfaceMulticastAddrMessage {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
