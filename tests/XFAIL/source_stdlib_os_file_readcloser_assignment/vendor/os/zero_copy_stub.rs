use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
};

use crate::{types::{File}};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

impl crate::types::File {
    pub fn write_to_1(&self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut written: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut handled: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        (0, false, Arc::new(Mutex::new(None)))
    }

    pub fn read_from_1(&self, r: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut handled: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        (0, false, Arc::new(Mutex::new(None)))
    }
}