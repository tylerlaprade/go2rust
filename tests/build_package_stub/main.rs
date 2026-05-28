use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct build_Context {
    pub g_o_r_o_o_t: Arc<Mutex<Option<String>>>,
}

impl std::fmt::Display for build_Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<build_Context>")
    }
}


impl build_Context {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn import<T0: build::GoStringArg, T1, T2>(&self, _arg0: T0, _arg1: T1, _arg2: T2) -> (Arc<Mutex<Option<build_Package>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        build::go_build_import_path(_arg0.into_go_string())
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct build_ImportMode(pub u32);

impl PartialEq<u32> for build_ImportMode {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<build_ImportMode> for u32 {
    fn eq(&self, other: &build_ImportMode) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for build_ImportMode {
    type Output = build_ImportMode;
    fn bitand(self, other: Self) -> build_ImportMode {
        build_ImportMode(self.0 & other.0)
    }
}

impl std::ops::BitOr for build_ImportMode {
    type Output = build_ImportMode;
    fn bitor(self, other: Self) -> build_ImportMode {
        build_ImportMode(self.0 | other.0)
    }
}

impl std::fmt::Display for build_ImportMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<build_ImportMode>")
    }
}


impl build_ImportMode {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct build_Package {
    pub dir: Arc<Mutex<Option<String>>>,
    pub goroot: Arc<Mutex<Option<bool>>>,
    pub import_path: Arc<Mutex<Option<String>>>,
    pub pkg_obj: Arc<Mutex<Option<String>>>,
}

impl std::fmt::Display for build_Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<build_Package>")
    }
}


impl build_Package {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod build {
    use super::*;
    use std::path::PathBuf;

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

    fn go_build_no_error() -> GoError {
        Arc::new(Mutex::new(None::<Box<dyn std::error::Error + Send + Sync>>))
    }

    fn go_build_error(message: String) -> GoError {
        Arc::new(Mutex::new(Some::<Box<dyn std::error::Error + Send + Sync>>(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, message)))))
    }

    fn go_build_string(value: String) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(value)))
    }

    fn go_build_bool(value: bool) -> Arc<Mutex<Option<bool>>> {
        Arc::new(Mutex::new(Some::<bool>(value)))
    }

    fn go_build_goroot() -> String {
        static GOROOT: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        GOROOT.get_or_init(|| {
            if let Ok(value) = std::env::var("GOROOT") {
                if !value.is_empty() {
                    return value;
                }
            }
            std::process::Command::new("go")
                .args(["env", "GOROOT"])
                .output()
                .ok()
                .and_then(|output| if output.status.success() { String::from_utf8(output.stdout).ok() } else { None })
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
                .unwrap_or_default()
        }).clone()
    }

    fn go_build_package(import_path: String, dir: String, goroot: bool) -> build_Package {
        build_Package {
            dir: go_build_string(dir),
            goroot: go_build_bool(goroot),
            import_path: go_build_string(import_path),
            pkg_obj: go_build_string(String::new()),
            ..Default::default()
        }
    }

    fn go_build_dir_for_import(goroot: &str, import_path: &str) -> PathBuf {
        let mut dir = PathBuf::from(goroot);
        dir.push("src");
        for part in import_path.split('/') {
            if !part.is_empty() {
                dir.push(part);
            }
        }
        dir
    }

    pub(crate) fn go_build_import_path(import_path: String) -> (Arc<Mutex<Option<build_Package>>>, GoError) {
        if import_path.is_empty() || go_build_is_local_import_str(&import_path) {
            return (
                Arc::new(Mutex::new(Some::<build_Package>(go_build_package(import_path.clone(), String::new(), false)))),
                go_build_error(format!("cannot import {}", import_path)),
            );
        }
        let goroot = go_build_goroot();
        if !goroot.is_empty() {
            let dir = go_build_dir_for_import(&goroot, &import_path);
            if dir.is_dir() {
                return (
                    Arc::new(Mutex::new(Some::<build_Package>(go_build_package(import_path.clone(), dir.to_string_lossy().into_owned(), true)))),
                    go_build_no_error(),
                );
            }
        }
        (
            Arc::new(Mutex::new(Some::<build_Package>(go_build_package(import_path.clone(), String::new(), false)))),
            go_build_error(format!("cannot find package {}", import_path)),
        )
    }

    fn go_build_is_local_import_str(path: &str) -> bool {
        path == "." || path == ".." || path.starts_with("./") || path.starts_with("../")
    }

    pub const FIND_ONLY: build_ImportMode = build_ImportMode(0);

    pub fn Default() -> Arc<Mutex<Option<build_Context>>> {
        Arc::new(Mutex::new(Some::<build_Context>(build_Context { g_o_r_o_o_t: go_build_string(go_build_goroot()), ..Default::default() })))
    }


    pub fn is_local_import<T0: GoStringArg>(_arg0: T0) -> bool {
        go_build_is_local_import_str(&_arg0.into_go_string())
    }
}


fn main() {
    let (mut pkg, mut err) = (*build::Default().lock().unwrap().as_mut().unwrap()).import("fmt".to_string(), "".to_string(), { let __go_arg = build::FIND_ONLY; __go_arg });
    println!("{} {} {} {}", format!("{}", (*err.lock().unwrap()).is_none()), format!("{}", (*{ let __field = (*pkg.lock().unwrap().as_ref().unwrap()).goroot.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*pkg.lock().unwrap().as_ref().unwrap()).import_path.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", { let __tmp_x = { let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = ""; __tmp_x != __tmp_y }));
    println!("{}", format!("{}", { let __tmp_x = { let __selector_holder = (*build::Default().lock().unwrap().as_ref().unwrap()).g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = ""; __tmp_x != __tmp_y }));
    println!("{} {}", format!("{}", build::is_local_import("./pkg".to_string())), format!("{}", build::is_local_import("fmt".to_string())));
}