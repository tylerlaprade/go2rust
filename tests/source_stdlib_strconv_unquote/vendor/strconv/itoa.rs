use go2rust_stdlib_stubs::*;

use crate::atob::*;
use crate::atoc::*;
use crate::atof::*;
use crate::atoi::*;
use crate::bytealg::*;
use crate::ctoa::*;
use crate::decimal::*;
use crate::eisel_lemire::*;
use crate::ftoa::*;
use crate::ftoaryu::*;
use crate::isprint::*;
use crate::quote::*;

pub(crate) const FAST_SMALLS: bool = true;


pub(crate) const N_SMALLS: i32 = 100;


pub(crate) const SMALLS_STRING: &'static str = "00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899";


pub(crate) const HOST32BIT: bool = (!(((0 as u64) as u64)) >> (32 as u64)) as u64 == 0 as u64;


pub(crate) const DIGITS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyz";
