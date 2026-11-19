use go2rust_stdlib_stubs::*;

pub(crate) const FAST_SMALLS: bool = true;


pub(crate) const N_SMALLS: i32 = 100;


pub(crate) const SMALLS_STRING: &'static str = concat!(
    "00010203040506070809101112131415",
    "16171819202122232425262728293031",
    "32333435363738394041424344454647",
    "48495051525354555657585960616263",
    "64656667686970717273747576777879",
    "80818283848586878889909192939495",
    "96979899",
);


pub(crate) const HOST32BIT: bool = (!(((0 as u64) as u64)) >> (32 as u64)) as u64 == 0 as u64;


pub(crate) const DIGITS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyz";
