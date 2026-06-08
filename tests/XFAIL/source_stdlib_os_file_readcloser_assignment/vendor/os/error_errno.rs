use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use std::sync::{Arc, Mutex};

pub(crate) const ERR_E_N_O_S_Y_S: usize = syscall::E_N_O_S_Y_S;
pub(crate) const ERR_E_R_A_N_G_E: usize = syscall::E_R_A_N_G_E;
pub(crate) const ERR_E_N_O_M_E_M: usize = syscall::E_N_O_M_E_M;


pub type syscallErrorType = Arc<Mutex<Option<syscall::syscall_unix::Errno>>>;
