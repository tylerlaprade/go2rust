use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

pub trait GoValueClone {
    fn go_value_clone(&self) -> Self;
}

macro_rules! impl_go_value_clone_copy {
    ($($t:ty),* $(,)?) => {
        $(impl GoValueClone for $t {
            fn go_value_clone(&self) -> Self { *self }
        })*
    };
}

impl_go_value_clone_copy!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, &'static str);

impl GoValueClone for String {
    fn go_value_clone(&self) -> Self { self.clone() }
}

impl GoValueClone for Box<dyn Any + Send + Sync> {
    fn go_value_clone(&self) -> Self { go_any_clone(self.as_ref()) }
}

pub trait GoComparable {
    fn go_eq(&self, other: &Self) -> bool;
    fn go_hash(&self, seed: usize) -> usize;
}

fn go_hash_value<T: std::hash::Hash>(value: &T, seed: usize) -> usize {
    let mut __hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&seed, &mut __hasher);
    std::hash::Hash::hash(value, &mut __hasher);
    std::hash::Hasher::finish(&__hasher) as usize
}

macro_rules! impl_go_comparable_hash {
    ($($t:ty),* $(,)?) => {
        $(impl GoComparable for $t {
            fn go_eq(&self, other: &Self) -> bool { self == other }
            fn go_hash(&self, seed: usize) -> usize { go_hash_value(self, seed) }
        })*
    };
}

impl_go_comparable_hash!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, String, &'static str);

impl GoComparable for f32 {
    fn go_eq(&self, other: &Self) -> bool { self == other }
    fn go_hash(&self, seed: usize) -> usize { go_hash_value(&self.to_bits(), seed) }
}

impl GoComparable for f64 {
    fn go_eq(&self, other: &Self) -> bool { self == other }
    fn go_hash(&self, seed: usize) -> usize { go_hash_value(&self.to_bits(), seed) }
}

fn go_any_comparable_eq(left: &(dyn Any + Send + Sync), right: &(dyn Any + Send + Sync)) -> bool {
    if left.type_id() != right.type_id() {
        return false;
    }
    if let Some(v) = left.downcast_ref::<i32>() { return right.downcast_ref::<i32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i64>() { return right.downcast_ref::<i64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i8>() { return right.downcast_ref::<i8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i16>() { return right.downcast_ref::<i16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u32>() { return right.downcast_ref::<u32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u64>() { return right.downcast_ref::<u64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u8>() { return right.downcast_ref::<u8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u16>() { return right.downcast_ref::<u16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<usize>() { return right.downcast_ref::<usize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<isize>() { return right.downcast_ref::<isize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f64>() { return right.downcast_ref::<f64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f32>() { return right.downcast_ref::<f32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<String>() { return right.downcast_ref::<String>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<&str>() { return right.downcast_ref::<&str>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<bool>() { return right.downcast_ref::<bool>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<char>() { return right.downcast_ref::<char>().map_or(false, |r| v == r); }
    panic!("interface comparison with uncomparable dynamic type")
}

fn go_any_comparable_hash(value: &(dyn Any + Send + Sync), seed: usize) -> usize {
    if let Some(v) = value.downcast_ref::<i32>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i64>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i8>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i16>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u32>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u64>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u8>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u16>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<usize>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<isize>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<f64>() { return go_hash_value(&(value.type_id(), v.to_bits()), seed); }
    if let Some(v) = value.downcast_ref::<f32>() { return go_hash_value(&(value.type_id(), v.to_bits()), seed); }
    if let Some(v) = value.downcast_ref::<String>() { return go_hash_value(&(value.type_id(), v), seed); }
    if let Some(v) = value.downcast_ref::<&str>() { return go_hash_value(&(value.type_id(), v), seed); }
    if let Some(v) = value.downcast_ref::<bool>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<char>() { return go_hash_value(&(value.type_id(), *v), seed); }
    panic!("interface hash with uncomparable dynamic type")
}

impl GoComparable for Box<dyn Any + Send + Sync> {
    fn go_eq(&self, other: &Self) -> bool { go_any_comparable_eq(self.as_ref(), other.as_ref()) }
    fn go_hash(&self, seed: usize) -> usize { go_any_comparable_hash(self.as_ref(), seed) }
}

#[derive(Clone, Debug, Default)]
pub struct GoRWMutex;

impl GoRWMutex {
    pub fn new() -> Self {
        Self
    }

    pub fn lock(&self) {}
    pub fn unlock(&self) {}
    pub fn r_lock(&self) {}
    pub fn r_unlock(&self) {}
}

impl std::fmt::Display for GoRWMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RWMutex")
    }
}


pub struct GoPtrKey<T>(pub Arc<Mutex<Option<T>>>);

impl<T> Clone for GoPtrKey<T> {
    fn clone(&self) -> Self { GoPtrKey(self.0.clone()) }
}

impl<T> GoPtrKey<T> {
    pub fn new(value: Arc<Mutex<Option<T>>>) -> Self { GoPtrKey(value) }
    pub fn value(&self) -> Arc<Mutex<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Arc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for GoPtrKey<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for GoPtrKey<T> {}
impl<T> PartialOrd for GoPtrKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for GoPtrKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for GoPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for GoPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

pub use serde_json;

pub trait GoJsonInputArg {
    fn into_go_json_bytes(self) -> Vec<u8>;
}

pub trait GoJsonDecode: Sized {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String>;
}

pub trait GoJsonDecodeTarget {
    fn assign_go_json(self, value: &serde_json::Value) -> Result<(), String>;
}

fn go_json_no_error() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    Arc::new(Mutex::new(None))
}

fn go_json_error(message: String) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(message))))
}

pub fn go_json_expected(value: &serde_json::Value, want: &str) -> String {
    format!("expected {}, got {}", want, value)
}

impl GoJsonInputArg for Vec<u8> {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self
    }
}

impl GoJsonInputArg for String {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl<'a> GoJsonInputArg for &'a str {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl<T> GoJsonInputArg for Arc<Mutex<Option<T>>>
where
    T: GoJsonInputArg + Clone,
{
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.lock().unwrap().as_ref().cloned().map(|value| value.into_go_json_bytes()).unwrap_or_default()
    }
}

impl GoJsonDecode for String {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_str().map(|value| value.to_string()).ok_or_else(|| go_json_expected(value, "string"))
    }
}

impl GoJsonDecode for bool {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_bool().ok_or_else(|| go_json_expected(value, "bool"))
    }
}

impl GoJsonDecode for i32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().map(|value| value as i32).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for i16 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().map(|value| value as i16).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for i8 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().map(|value| value as i8).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for i64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u8 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as u8).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u16 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as u16).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as u32).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for usize {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as usize).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for f64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_f64().ok_or_else(|| go_json_expected(value, "number"))
    }
}

impl GoJsonDecode for f32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_f64().map(|value| value as f32).ok_or_else(|| go_json_expected(value, "number"))
    }
}

impl<T> GoJsonDecode for Vec<T>
where
    T: GoJsonDecode,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let array = value.as_array().ok_or_else(|| go_json_expected(value, "array"))?;
        array.iter().map(T::go_json_decode).collect()
    }
}

impl<T, const N: usize> GoJsonDecode for [T; N]
where
    T: GoJsonDecode + Default,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let array = value.as_array().ok_or_else(|| go_json_expected(value, "array"))?;
        if array.len() != N {
            return Err(format!("expected array of length {}, got {}", N, array.len()));
        }
        let mut out = std::array::from_fn(|_| T::default());
        for (index, item) in array.iter().enumerate() {
            out[index] = T::go_json_decode(item)?;
        }
        Ok(out)
    }
}

impl<V> GoJsonDecode for BTreeMap<String, V>
where
    V: GoJsonDecode,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = BTreeMap::new();
        for (key, value) in object {
            out.insert(key.clone(), V::go_json_decode(value)?);
        }
        Ok(out)
    }
}

impl<T> GoJsonDecode for Arc<Mutex<Option<T>>>
where
    T: GoJsonDecode,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        if value.is_null() {
            Ok(Arc::new(Mutex::new(None)))
        } else {
            Ok(Arc::new(Mutex::new(Some(T::go_json_decode(value)?))))
        }
    }
}

impl<T> GoJsonDecodeTarget for Arc<Mutex<Option<T>>>
where
    T: GoJsonDecode,
{
    fn assign_go_json(self, value: &serde_json::Value) -> Result<(), String> {
        if value.is_null() {
            *self.lock().unwrap() = None;
        } else {
            *self.lock().unwrap() = Some(T::go_json_decode(value)?);
        }
        Ok(())
    }
}

impl GoJsonInputArg for bytes_Buffer {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.__go_bytes()
    }
}

pub trait GoTypesBridgeStringArg {
    fn into_go_types_bridge_string(self) -> String;
}

impl GoTypesBridgeStringArg for String {
    fn into_go_types_bridge_string(self) -> String { self }
}

impl<'a> GoTypesBridgeStringArg for &'a str {
    fn into_go_types_bridge_string(self) -> String { self.to_string() }
}

impl<'a> GoTypesBridgeStringArg for &'a String {
    fn into_go_types_bridge_string(self) -> String { self.clone() }
}

impl GoTypesBridgeStringArg for Arc<Mutex<Option<String>>> {
    fn into_go_types_bridge_string(self) -> String {
        self.lock().unwrap().as_ref().cloned().unwrap_or_default()
    }
}

pub trait GoTypesBridgeInfoArg {
    fn apply_go_types_bridge_facts(self, type_facts: &[serde_json::Value], exprs_by_pos: &BTreeMap<i32, Vec<ast_Expr>>);
}

impl GoTypesBridgeInfoArg for () {
    fn apply_go_types_bridge_facts(self, _type_facts: &[serde_json::Value], _exprs_by_pos: &BTreeMap<i32, Vec<ast_Expr>>) {}
}

impl GoTypesBridgeInfoArg for Arc<Mutex<Option<types_Info>>> {
    fn apply_go_types_bridge_facts(self, type_facts: &[serde_json::Value], exprs_by_pos: &BTreeMap<i32, Vec<ast_Expr>>) {
        let mut info_guard = self.lock().unwrap();
        if let Some(info_value) = info_guard.as_mut() {
            let mut types_guard = info_value.types.lock().unwrap();
            if let Some(types_map) = types_guard.as_mut() {
                for fact in type_facts {
                    if fact.get("kind").and_then(|v| v.as_str()) != Some("basic") {
                        continue;
                    }
                    let pos = fact.get("pos").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
                    let Some(exprs) = exprs_by_pos.get(&pos) else { continue; };
                    let name = fact.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
                    let kind = fact.get("basicKind").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
                    let info_bits = fact.get("basicInfo").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
                    for expr in exprs {
                        types_map.insert(expr.clone(), Arc::new(Mutex::new(Some::<types_TypeAndValue>(types_TypeAndValue { r#type: Arc::new(Mutex::new(Some::<types_Type>(__go_types_basic_type(name.clone(), kind, info_bits)))), value: Default::default() }))));
                    }
                }
            }
        }
    }
}

const __GO_TYPES_BRIDGE_HELPER: &str = r#"
package main

import (
	"encoding/json"
	"fmt"
	"go/ast"
	"go/importer"
	"go/parser"
	"go/token"
	"go/types"
	"os"
	"sort"
)

type request struct {
	Path  string `json:"path"`
	Files []file `json:"files"`
}

type file struct {
	Filename string `json:"filename"`
	Source   string `json:"source"`
}

type response struct {
	Package packageFact `json:"package"`
	Errors  []string    `json:"errors"`
	Types   []typeFact  `json:"types"`
}

type packageFact struct {
	Path string `json:"path"`
	Name string `json:"name"`
}

type typeFact struct {
	Pos       int    `json:"pos"`
	Kind      string `json:"kind"`
	Name      string `json:"name"`
	BasicKind int    `json:"basicKind"`
	BasicInfo int    `json:"basicInfo"`
}

func main() {
	var req request
	if err := json.NewDecoder(os.Stdin).Decode(&req); err != nil {
		_ = json.NewEncoder(os.Stdout).Encode(response{Errors: []string{err.Error()}})
		return
	}

	fset := token.NewFileSet()
	files := make([]*ast.File, 0, len(req.Files))
	for _, input := range req.Files {
		file, err := parser.ParseFile(fset, input.Filename, input.Source, parser.ParseComments|parser.SkipObjectResolution)
		if err != nil {
			_ = json.NewEncoder(os.Stdout).Encode(response{Errors: []string{err.Error()}})
			return
		}
		files = append(files, file)
	}

	info := &types.Info{
		Types: make(map[ast.Expr]types.TypeAndValue),
		Defs:  make(map[*ast.Ident]types.Object),
		Uses:  make(map[*ast.Ident]types.Object),
	}
	var errs []string
	config := &types.Config{
		Importer: importer.Default(),
		Error: func(err error) {
			errs = append(errs, err.Error())
		},
	}
	pkg, err := config.Check(req.Path, fset, files, info)
	if err != nil {
		msg := err.Error()
		if len(errs) == 0 || errs[len(errs)-1] != msg {
			errs = append(errs, msg)
		}
	}

	resp := response{Errors: errs}
	if pkg != nil {
		resp.Package = packageFact{Path: pkg.Path(), Name: pkg.Name()}
	}
	for expr, tv := range info.Types {
		if tv.Type == nil || expr == nil {
			continue
		}
		if basic, ok := types.Unalias(tv.Type).Underlying().(*types.Basic); ok {
			resp.Types = append(resp.Types, typeFact{
				Pos:       int(expr.Pos()),
				Kind:      "basic",
				Name:      basic.Name(),
				BasicKind: int(basic.Kind()),
				BasicInfo: int(basic.Info()),
			})
		}
	}
	sort.Slice(resp.Types, func(i, j int) bool {
		if resp.Types[i].Pos != resp.Types[j].Pos {
			return resp.Types[i].Pos < resp.Types[j].Pos
		}
		return resp.Types[i].Name < resp.Types[j].Name
	})
	if err := json.NewEncoder(os.Stdout).Encode(resp); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
"#;

fn __go_types_bridge_error(message: String) -> Box<dyn StdError + Send + Sync> {
    Box::new(std::io::Error::new(std::io::ErrorKind::Other, message)) as Box<dyn StdError + Send + Sync>
}

fn __go_types_basic_type(name: String, kind: i32, info: i32) -> types_Type {
    types_Type::__go_from(types_Basic {
        __go_kind: types_BasicKind(kind),
        __go_info: types_BasicInfo(info),
        __go_name: name,
    })
}

fn __go_types_config_check<T0: GoTypesBridgeStringArg, T3: GoTypesBridgeInfoArg>(
    path_arg: T0,
    files: Arc<Mutex<Option<Vec<Arc<Mutex<Option<ast_File>>>>>>>,
    info: T3,
) -> Result<types_Package, Box<dyn StdError + Send + Sync>> {
    let path = path_arg.into_go_types_bridge_string();
    let file_values = files.lock().unwrap().as_ref().cloned().unwrap_or_default();
    let mut request_files = Vec::<serde_json::Value>::new();
    let mut exprs_by_pos = BTreeMap::<i32, Vec<ast_Expr>>::new();
    for file_handle in file_values {
        let file_guard = file_handle.lock().unwrap();
        let Some(file) = file_guard.as_ref() else { continue; };
        let filename = file.__go_filename.lock().unwrap().as_ref().cloned().unwrap_or_default();
        let source = file.__go_source.lock().unwrap().as_ref().cloned().unwrap_or_default();
        if source.is_empty() {
            continue;
        }
        __go_types_collect_file_exprs(file, &mut exprs_by_pos);
        request_files.push(serde_json::json!({
            "filename": filename,
            "source": source,
        }));
    }
    if request_files.is_empty() {
        return Err(__go_types_bridge_error("go/types bridge requires parser.ParseFile source metadata".to_string()));
    }

    let request = serde_json::json!({
        "path": path,
        "files": request_files,
    });
    let output = __go_types_run_bridge_helper(&request.to_string())?;
    let response: serde_json::Value = serde_json::from_slice(&output)
        .map_err(|err| __go_types_bridge_error(format!("failed to decode go/types bridge response: {}", err)))?;
    if let Some(errors) = response.get("errors").and_then(|v| v.as_array()) {
        if !errors.is_empty() {
            let message = errors.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("; ");
            return Err(__go_types_bridge_error(message));
        }
    }

    if let Some(type_facts) = response.get("types").and_then(|v| v.as_array()) {
        info.apply_go_types_bridge_facts(type_facts, &exprs_by_pos);
    }

    Ok(types_Package::default())
}

fn __go_types_run_bridge_helper(request_json: &str) -> Result<Vec<u8>, Box<dyn StdError + Send + Sync>> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let unique = format!(
        "go2rust-types-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default()
    );
    let dir = std::env::temp_dir().join(unique);
    std::fs::create_dir_all(&dir)
        .map_err(|err| __go_types_bridge_error(format!("failed to create go/types bridge dir: {}", err)))?;
    let helper_path = dir.join("main.go");
    std::fs::write(&helper_path, __GO_TYPES_BRIDGE_HELPER)
        .map_err(|err| __go_types_bridge_error(format!("failed to write go/types bridge helper: {}", err)))?;
    let mut child = Command::new("go")
        .arg("run")
        .arg(&helper_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| __go_types_bridge_error(format!("failed to launch go/types bridge helper: {}", err)))?;
    {
        let stdin = child.stdin.as_mut().ok_or_else(|| __go_types_bridge_error("failed to open go/types bridge stdin".to_string()))?;
        stdin.write_all(request_json.as_bytes())
            .map_err(|err| __go_types_bridge_error(format!("failed to write go/types bridge request: {}", err)))?;
    }
    let output = child.wait_with_output()
        .map_err(|err| __go_types_bridge_error(format!("failed to wait for go/types bridge helper: {}", err)))?;
    let _ = std::fs::remove_dir_all(&dir);
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(__go_types_bridge_error(format!("go/types bridge helper failed: {}", stderr)));
    }
    Ok(output.stdout)
}

fn __go_types_record_expr(exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>, expr: &ast_Expr) {
    if expr.__go_pos != 0 {
        exprs_by_pos.entry(expr.__go_pos).or_default().push(expr.clone());
    }
}

fn __go_types_collect_file_exprs(file: &ast_File, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    let decls = file.decls.lock().unwrap().as_ref().cloned().unwrap_or_default();
    for decl in decls {
        __go_types_collect_decl_exprs(&decl, exprs_by_pos);
    }
}

fn __go_types_collect_decl_exprs(decl: &ast_Decl, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(value) = decl.downcast_ref::<ast_GenDecl>() {
        let specs = value.specs.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for spec in specs {
            __go_types_collect_spec_exprs(&spec, exprs_by_pos);
        }
    } else if let Some(value) = decl.downcast_ref::<ast_FuncDecl>() {
        __go_types_collect_opt_field_list(&value.recv, exprs_by_pos);
        __go_types_collect_func_type(&value.r#type, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
    }
}

fn __go_types_collect_spec_exprs(spec: &ast_Spec, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(value) = spec.downcast_ref::<ast_ValueSpec>() {
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
        let values = value.values.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for expr in values {
            __go_types_collect_expr(&expr, exprs_by_pos);
        }
    } else if let Some(value) = spec.downcast_ref::<ast_TypeSpec>() {
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
    }
}

fn __go_types_collect_opt_expr(value: &Arc<Mutex<Option<ast_Expr>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(expr) = value.lock().unwrap().as_ref().cloned() {
        __go_types_collect_expr(&expr, exprs_by_pos);
    }
}

fn __go_types_collect_expr(expr: &ast_Expr, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    __go_types_record_expr(exprs_by_pos, expr);
    if let Some(value) = expr.downcast_ref::<ast_ArrayType>() {
        __go_types_collect_opt_expr(&value.len, exprs_by_pos);
        __go_types_collect_opt_expr(&value.elt, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_BinaryExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.y, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_CallExpr>() {
        __go_types_collect_call_expr(value, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_CompositeLit>() {
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
        let elts = value.elts.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for elt in elts {
            __go_types_collect_expr(&elt, exprs_by_pos);
        }
    } else if let Some(value) = expr.downcast_ref::<ast_IndexExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.index, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_IndexListExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        let indices = value.indices.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for index in indices {
            __go_types_collect_expr(&index, exprs_by_pos);
        }
    } else if let Some(value) = expr.downcast_ref::<ast_KeyValueExpr>() {
        __go_types_collect_opt_expr(&value.key, exprs_by_pos);
        __go_types_collect_opt_expr(&value.value, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_MapType>() {
        __go_types_collect_opt_expr(&value.key, exprs_by_pos);
        __go_types_collect_opt_expr(&value.value, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_ParenExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_SelectorExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_SliceExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.low, exprs_by_pos);
        __go_types_collect_opt_expr(&value.high, exprs_by_pos);
        __go_types_collect_opt_expr(&value.max, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_StarExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_TypeAssertExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_UnaryExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    }
}

fn __go_types_collect_call_expr(value: &ast_CallExpr, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    __go_types_collect_opt_expr(&value.fun, exprs_by_pos);
    let args = value.args.lock().unwrap().as_ref().cloned().unwrap_or_default();
    for arg in args {
        __go_types_collect_expr(&arg, exprs_by_pos);
    }
}

fn __go_types_collect_opt_stmt(value: &Arc<Mutex<Option<ast_Stmt>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(stmt) = value.lock().unwrap().as_ref().cloned() {
        __go_types_collect_stmt_exprs(&stmt, exprs_by_pos);
    }
}

fn __go_types_collect_stmt_exprs(stmt: &ast_Stmt, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(value) = stmt.downcast_ref::<ast_AssignStmt>() {
        let lhs = value.lhs.lock().unwrap().as_ref().cloned().unwrap_or_default();
        let rhs = value.rhs.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for expr in lhs.into_iter().chain(rhs.into_iter()) {
            __go_types_collect_expr(&expr, exprs_by_pos);
        }
    } else if let Some(value) = stmt.downcast_ref::<ast_DeclStmt>() {
        __go_types_collect_opt_decl(&value.decl, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_ExprStmt>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_ReturnStmt>() {
        let results = value.results.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for expr in results {
            __go_types_collect_expr(&expr, exprs_by_pos);
        }
    } else if let Some(value) = stmt.downcast_ref::<ast_IfStmt>() {
        __go_types_collect_opt_stmt(&value.init, exprs_by_pos);
        __go_types_collect_opt_expr(&value.cond, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
        __go_types_collect_opt_stmt(&value.r#else, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_ForStmt>() {
        __go_types_collect_opt_stmt(&value.init, exprs_by_pos);
        __go_types_collect_opt_expr(&value.cond, exprs_by_pos);
        __go_types_collect_opt_stmt(&value.post, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_RangeStmt>() {
        __go_types_collect_opt_expr(&value.key, exprs_by_pos);
        __go_types_collect_opt_expr(&value.value, exprs_by_pos);
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
    }
}

fn __go_types_collect_opt_decl(value: &Arc<Mutex<Option<ast_Decl>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(decl) = value.lock().unwrap().as_ref().cloned() {
        __go_types_collect_decl_exprs(&decl, exprs_by_pos);
    }
}

fn __go_types_collect_opt_block(value: &Arc<Mutex<Option<ast_BlockStmt>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(block) = value.lock().unwrap().as_ref() {
        let list = block.list.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for stmt in list {
            __go_types_collect_stmt_exprs(&stmt, exprs_by_pos);
        }
    }
}

fn __go_types_collect_func_type(value: &Arc<Mutex<Option<ast_FuncType>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(func_type) = value.lock().unwrap().as_ref() {
        __go_types_collect_opt_field_list(&func_type.params, exprs_by_pos);
        __go_types_collect_opt_field_list(&func_type.results, exprs_by_pos);
    }
}

fn __go_types_collect_opt_field_list(value: &Arc<Mutex<Option<ast_FieldList>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(field_list) = value.lock().unwrap().as_ref() {
        let fields = field_list.list.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for field in fields {
            let field_guard = field.lock().unwrap();
            if let Some(field_value) = field_guard.as_ref() {
                __go_types_collect_opt_expr(&field_value.r#type, exprs_by_pos);
                __go_types_collect_opt_basic_lit(&field_value.tag, exprs_by_pos);
            }
        }
    }
}

fn __go_types_collect_opt_basic_lit(value: &Arc<Mutex<Option<ast_BasicLit>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(lit) = value.lock().unwrap().as_ref() {
        let lit_pos = lit.pos.lock().unwrap().as_ref().map(|pos| pos.0).unwrap_or_default();
        if lit_pos != 0 {
            exprs_by_pos.entry(lit_pos).or_default().push(ast_Expr::__go_from_with_pos(lit.clone(), lit_pos));
        }
    }
}



#[derive(Debug, Clone)]
pub struct bytes_Buffer {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
}

impl Default for bytes_Buffer {
    fn default() -> Self {
        Self { __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())) }
    }
}

impl std::fmt::Display for bytes_Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__go_string())
    }
}

impl bytes_Buffer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn __go_from_string(value: String) -> Self {
        Self { __go_data: std::sync::Arc::new(std::sync::Mutex::new(value.into_bytes())) }
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        self.__go_data.lock().unwrap().extend_from_slice(data);
    }

    pub fn __go_bytes(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    pub fn __go_string(&self) -> String {
        String::from_utf8_lossy(&self.__go_data.lock().unwrap()).into_owned()
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(self.__go_string())))
    }

    pub fn bytes(&self) -> Arc<Mutex<Option<Vec<u8>>>> {
        Arc::new(Mutex::new(Some::<Vec<u8>>(self.__go_bytes())))
    }

    pub fn len(&self) -> i32 {
        self.__go_data.lock().unwrap().len() as i32
    }

    pub fn reset(&self) {
        self.__go_data.lock().unwrap().clear();
    }

    pub fn available(&self) -> i32 {
        self.len()
    }

    pub fn available_buffer(&self) -> Arc<Mutex<Option<Vec<u8>>>> {
        Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::new())))
    }

    pub fn cap(&self) -> i32 {
        self.len()
    }

    pub fn grow<T0>(&self, _arg0: T0) {
    }

    pub fn next<T0>(&self, _arg0: T0) -> Arc<Mutex<Option<Vec<u8>>>> {
        Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::new())))
    }

    pub fn read<T0>(&self, _arg0: T0) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (0 as i32, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_byte(&self) -> (u8, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (0 as u8, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_bytes<T0>(&self, _arg0: T0) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::new()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_from<T0>(&self, _arg0: T0) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (0 as i64, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_rune(&self) -> (i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (0 as i32, 0 as i32, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_string<T0>(&self, _arg0: T0) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<String>(String::new()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn truncate<T0>(&self, _arg0: T0) {
        self.reset();
    }

    pub fn unread_byte(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn unread_rune(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<&str>() {
            (*v).to_string()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<String>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            String::new()
        };
        let bytes = value.into_bytes();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn write_byte<T0: 'static>(&self, arg0: T0) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<u8>() {
            *v
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<i32>() {
            *v as u8
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<u8>>>>() {
            v.lock().unwrap().as_ref().copied().unwrap_or_default()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<i32>>>>() {
            v.lock().unwrap().as_ref().copied().unwrap_or_default() as u8
        } else {
            0
        };
        self.__go_write_bytes(&[value]);
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn write_rune<T0: 'static>(&self, arg0: T0) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<char>() {
            *v
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<i32>() {
            char::from_u32(*v as u32).unwrap_or('\0')
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<i32>>>>() {
            char::from_u32(v.lock().unwrap().as_ref().copied().unwrap_or_default() as u32).unwrap_or('\0')
        } else {
            '\0'
        };
        let mut encoded = [0u8; 4];
        let bytes = value.encode_utf8(&mut encoded).as_bytes().to_vec();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn write_to<T0>(&self, _arg0: T0) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (self.__go_data.lock().unwrap().len() as i64, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct constant_Value;

impl std::fmt::Display for constant_Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<constant_Value>")
    }
}


impl constant_Value {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct constraint_Expr {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl constraint_Expr {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for constraint_Expr {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for constraint_Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<constraint_Expr>")
    }
}

impl std::fmt::Display for constraint_Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<constraint_Expr>")
    }
}

impl PartialEq for constraint_Expr {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for constraint_Expr {}

impl PartialOrd for constraint_Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for constraint_Expr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct fs_FileInfo {
    pub name: String,
    pub is_dir: bool,
    pub size: i64,
}

impl std::fmt::Display for fs_FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fs_FileInfo>")
    }
}


impl fs_FileInfo {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(self.name.clone())))
    }
    pub fn size(&self) -> i64 {
        self.size
    }
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
}


#[derive(Clone)]
pub struct io_Reader {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl io_Reader {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for io_Reader {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for io_Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Reader>")
    }
}

impl std::fmt::Display for io_Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Reader>")
    }
}

impl PartialEq for io_Reader {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for io_Reader {}

impl PartialOrd for io_Reader {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for io_Reader {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone)]
pub struct os_File {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
    pub __go_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub __go_wait_for_close: bool,
}

impl Default for os_File {
    fn default() -> Self {
        Self {
            __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            __go_wait_for_close: false,
        }
    }
}

impl std::fmt::Display for os_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<os_File>")
    }
}

impl PartialEq for os_File {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__go_data, &other.__go_data)
    }
}

impl Eq for os_File {}

impl os_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        self.__go_data.lock().unwrap().extend_from_slice(data);
    }

    pub fn __go_read_all(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    pub fn __go_read_all_for_copy(&self) -> Vec<u8> {
        while self.__go_wait_for_close && !self.__go_closed.load(std::sync::atomic::Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        self.__go_read_all()
    }

    pub fn close(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.__go_closed.store(true, std::sync::atomic::Ordering::SeqCst);
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (Arc::new(Mutex::new(Some::<i32>(n))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<&str>() {
            (*v).to_string()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<String>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            String::new()
        };
        let bytes = value.into_bytes();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (Arc::new(Mutex::new(Some::<i32>(n))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read<T0>(&self, _arg0: T0) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<i32>(0))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_at<T0: 'static, T1: 'static>(&self, arg0: T0, arg1: T1) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let offset = if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<i64>() {
            *v
        } else if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<i64>>>>() {
            v.lock().unwrap().as_ref().copied().unwrap_or_default()
        } else {
            0
        };
        let data = self.__go_read_all();
        let mut n = 0i32;
        if offset >= 0 {
            let start = offset as usize;
            if start < data.len() {
                if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
                    let mut guard = v.lock().unwrap();
                    if let Some(target) = guard.as_mut() {
                        let count = std::cmp::min(target.len(), data.len() - start);
                        target[..count].copy_from_slice(&data[start..start + count]);
                        n = count as i32;
                    }
                }
            }
        }
        (Arc::new(Mutex::new(Some::<i32>(n))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct reflect_Value;

impl std::fmt::Display for reflect_Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_Value>")
    }
}


impl reflect_Value {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Basic {
    pub __go_kind: types_BasicKind,
    pub __go_info: types_BasicInfo,
    pub __go_name: String,
}

impl std::fmt::Display for types_Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__go_name)
    }
}


impl types_Basic {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_BasicInfo(pub i32);

impl PartialEq<i32> for types_BasicInfo {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<types_BasicInfo> for i32 {
    fn eq(&self, other: &types_BasicInfo) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for types_BasicInfo {
    type Output = types_BasicInfo;
    fn bitand(self, other: Self) -> types_BasicInfo {
        types_BasicInfo(self.0 & other.0)
    }
}

impl std::ops::BitOr for types_BasicInfo {
    type Output = types_BasicInfo;
    fn bitor(self, other: Self) -> types_BasicInfo {
        types_BasicInfo(self.0 | other.0)
    }
}

impl std::fmt::Display for types_BasicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_BasicInfo>")
    }
}


impl types_BasicInfo {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_BasicKind(pub i32);

impl PartialEq<i32> for types_BasicKind {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<types_BasicKind> for i32 {
    fn eq(&self, other: &types_BasicKind) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for types_BasicKind {
    type Output = types_BasicKind;
    fn bitand(self, other: Self) -> types_BasicKind {
        types_BasicKind(self.0 & other.0)
    }
}

impl std::ops::BitOr for types_BasicKind {
    type Output = types_BasicKind;
    fn bitor(self, other: Self) -> types_BasicKind {
        types_BasicKind(self.0 | other.0)
    }
}

impl std::fmt::Display for types_BasicKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_BasicKind>")
    }
}


impl types_BasicKind {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Config;

impl std::fmt::Display for types_Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Config>")
    }
}


impl types_Config {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn check<T0: GoTypesBridgeStringArg, T1, T3: GoTypesBridgeInfoArg>(&self, _arg0: T0, _arg1: T1, _arg2: Arc<Mutex<Option<Vec<Arc<Mutex<Option<ast_File>>>>>>>, _arg3: T3) -> (Arc<Mutex<Option<types_Package>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        match __go_types_config_check(_arg0, _arg2, _arg3) {
            Ok(pkg) => (Arc::new(Mutex::new(Some::<types_Package>(pkg))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))),
            Err(err) => (Arc::new(Mutex::new(None::<types_Package>)), Arc::new(Mutex::new(Some::<Box<dyn StdError + Send + Sync>>(err)))),
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct types_Info {
    pub types: Arc<Mutex<Option<BTreeMap<ast_Expr, Arc<Mutex<Option<types_TypeAndValue>>>>>>>,
}

impl std::fmt::Display for types_Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Info>")
    }
}


impl types_Info {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Package;

impl std::fmt::Display for types_Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Package>")
    }
}


impl types_Package {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        panic!("types_Package.name bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Clone)]
pub struct types_Type {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl types_Type {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for types_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl PartialEq for types_Type {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for types_Type {}

impl PartialOrd for types_Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for types_Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Default)]
pub struct types_TypeAndValue {
    pub r#type: Arc<Mutex<Option<types_Type>>>,
    pub value: Arc<Mutex<Option<constant_Value>>>,
}

impl std::fmt::Display for types_TypeAndValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_TypeAndValue>")
    }
}


impl types_TypeAndValue {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod bits {
    use super::*;
    pub fn mul<T0, T1>(_arg0: T0, _arg1: T1) -> (u64, u64) {
        panic!("mul bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod bytes {
    use super::*;
    pub fn has_prefix<T0, T1>(_arg0: T0, _arg1: T1) -> bool {
        panic!("has_prefix bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn last_index_byte<T0, T1>(_arg0: T0, _arg1: T1) -> i32 {
        panic!("last_index_byte bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod constraint {
    use super::*;
    pub fn go_version<T0>(_arg0: T0) -> Arc<Mutex<Option<String>>> {
        panic!("go_version bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn parse<T0>(_arg0: T0) -> (Arc<Mutex<Option<constraint_Expr>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("parse bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod fs {
    use super::*;
    pub fn SkipAll() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }

    pub fn SkipDir() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }
}


pub mod io {
    use super::*;
    pub fn read_all<T0>(_arg0: T0) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("read_all bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod os {
    use super::*;
    use std::path::Path;

    pub trait GoStringArg {
        fn into_go_string(self) -> String;
    }

    impl GoStringArg for String {
        fn into_go_string(self) -> String {
            self
        }
    }

    impl<'a> GoStringArg for &'a str {
        fn into_go_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStringArg for &'a String {
        fn into_go_string(self) -> String {
            self.clone()
        }
    }

    impl GoStringArg for Arc<Mutex<Option<String>>> {
        fn into_go_string(self) -> String {
            self.lock().unwrap().as_ref().cloned().unwrap_or_default()
        }
    }

    type GoError = Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>;

    fn no_error() -> GoError {
        Arc::new(Mutex::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        Arc::new(Mutex::new(Some(Box::new(err))))
    }

    pub const PATH_LIST_SEPARATOR: i32 = 58;
    pub const PATH_SEPARATOR: i32 = 47;

    pub fn lstat<T0: GoStringArg>(_arg0: T0) -> (Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::symlink_metadata(&path) {
            Ok(metadata) => {
                let name = Path::new(&path).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_else(|| path.clone());
                (Arc::new(Mutex::new(Some::<fs_FileInfo>(fs_FileInfo { name, is_dir: metadata.is_dir(), size: metadata.len() as i64 }))), no_error())
            }
            Err(err) => (Arc::new(Mutex::new(Some::<fs_FileInfo>(fs_FileInfo::default()))), io_error(err)),
        }
    }

    pub fn read_file<T0: GoStringArg>(_arg0: T0) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::read(&path) {
            Ok(data) => (Arc::new(Mutex::new(Some::<Vec<u8>>(data))), no_error()),
            Err(err) => (Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::new()))), io_error(err)),
        }
    }
}


pub mod strconv {
    use super::*;

    pub trait GoStrconvStringArg {
        fn into_go_strconv_string(self) -> String;
    }

    impl GoStrconvStringArg for String {
        fn into_go_strconv_string(self) -> String {
            self
        }
    }

    impl<'a> GoStrconvStringArg for &'a str {
        fn into_go_strconv_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStrconvStringArg for &'a String {
        fn into_go_strconv_string(self) -> String {
            self.clone()
        }
    }

    impl GoStrconvStringArg for Arc<Mutex<Option<String>>> {
        fn into_go_strconv_string(self) -> String {
            self.lock().unwrap().as_ref().cloned().unwrap_or_default()
        }
    }

    fn strconv_error(message: String) -> Box<dyn std::error::Error + Send + Sync> {
        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, message))
    }

    fn strconv_hex_digit(ch: char) -> Option<u32> {
        ch.to_digit(16)
    }

    fn strconv_read_hex<I: Iterator<Item = char>>(chars: &mut I, count: usize) -> Result<char, Box<dyn std::error::Error + Send + Sync>> {
        let mut value = 0u32;
        for _ in 0..count {
            let ch = chars.next().ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
            let digit = strconv_hex_digit(ch).ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
            value = (value << 4) | digit;
        }
        char::from_u32(value).ok_or_else(|| strconv_error("invalid quoted string".to_string()))
    }

    fn strconv_unquote_text(input: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut chars = input.chars();
        let quote = chars.next().ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
        if quote != '"' && quote != '\'' && quote != char::from(96) {
            return Err(strconv_error("invalid quoted string".to_string()));
        }
        let inner = input.strip_prefix(quote).and_then(|s| s.strip_suffix(quote)).ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
        if quote == char::from(96) {
            return Ok(inner.to_string());
        }
        let mut out = String::new();
        let mut chars = inner.chars();
        while let Some(ch) = chars.next() {
            if ch != '\\' {
                out.push(ch);
                continue;
            }
            let esc = chars.next().ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
            match esc {
                'a' => out.push('\x07'),
                'b' => out.push('\x08'),
                'f' => out.push('\x0c'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'v' => out.push('\x0b'),
                '\\' => out.push('\\'),
                '"' => out.push('"'),
                '\'' => out.push('\''),
                'x' => out.push(strconv_read_hex(&mut chars, 2)?),
                'u' => out.push(strconv_read_hex(&mut chars, 4)?),
                'U' => out.push(strconv_read_hex(&mut chars, 8)?),
                '0'..='7' => {
                    let mut value = esc.to_digit(8).unwrap();
                    for _ in 0..2 {
                        let Some(next) = chars.clone().next() else { break };
                        let Some(digit) = next.to_digit(8) else { break };
                        chars.next();
                        value = (value << 3) | digit;
                    }
                    out.push(char::from_u32(value).ok_or_else(|| strconv_error("invalid quoted string".to_string()))?);
                }
                _ => return Err(strconv_error("invalid quoted string".to_string())),
            }
        }
        Ok(out)
    }

    pub fn parse_uint<T0, T1, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> (u64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        panic!("parse_uint bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod unicode {
    use super::*;
    pub const MAX_RUNE: i32 = 1114111;

    pub fn is_digit<T0>(_arg0: T0) -> bool {
        panic!("is_digit bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn is_letter<T0>(_arg0: T0) -> bool {
        panic!("is_letter bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn is_upper<T0>(_arg0: T0) -> bool {
        panic!("is_upper bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod utf8 {
    use super::*;
    pub const RUNE_ERROR: i32 = 65533;
    pub const RUNE_SELF: i32 = 128;

    pub fn append_rune<T0, T1>(_arg0: T0, _arg1: T1) -> Arc<Mutex<Option<Vec<u8>>>> {
        panic!("append_rune bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn decode_rune<T0>(_arg0: T0) -> (i32, i32) {
        panic!("decode_rune bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn decode_rune_in_string<T0>(_arg0: T0) -> (i32, i32) {
        panic!("decode_rune_in_string bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }

    pub fn rune_count_in_string<T0>(_arg0: T0) -> i32 {
        panic!("rune_count_in_string bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}
