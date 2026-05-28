use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct fs_DirEntry {
    pub name: String,
    pub is_dir: bool,
}

impl std::fmt::Display for fs_DirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fs_DirEntry>")
    }
}


impl fs_DirEntry {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(self.name.clone())))
    }
    pub fn is_dir(&self) -> bool {
        self.is_dir
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


pub mod filepath {
    use super::*;
    use std::path::{Path, PathBuf};

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

    pub trait GoPathJoinArgs {
        fn into_path_parts(self) -> Vec<String>;
    }

    impl<T0: GoStringArg> GoPathJoinArgs for (T0,) {
        fn into_path_parts(self) -> Vec<String> {
            vec![self.0.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg> GoPathJoinArgs for (T0, T1) {
        fn into_path_parts(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg, T2: GoStringArg> GoPathJoinArgs for (T0, T1, T2) {
        fn into_path_parts(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string(), self.2.into_go_string()]
        }
    }

    type GoError = Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>;

    fn no_error() -> GoError {
        Arc::new(Mutex::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        Arc::new(Mutex::new(Some(Box::new(err))))
    }

    fn normalize_path(path: PathBuf) -> String {
        path.components().collect::<PathBuf>().to_string_lossy().into_owned()
    }

    pub fn base<T0: GoStringArg>(_arg0: T0) -> Arc<Mutex<Option<String>>> {
        let path = _arg0.into_go_string();
        Arc::new(Mutex::new(Some::<String>(Path::new(&path).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or(path))))
    }

    pub fn join<T0: GoPathJoinArgs>(_arg0: T0) -> Arc<Mutex<Option<String>>> {
        let mut path = PathBuf::new();
        for part in _arg0.into_path_parts() {
            if !part.is_empty() {
                path.push(part);
            }
        }
        Arc::new(Mutex::new(Some::<String>(path.to_string_lossy().into_owned())))
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

    pub fn read_dir<T0: GoStringArg>(_arg0: T0) -> (Arc<Mutex<Option<Vec<fs_DirEntry>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let path = _arg0.into_go_string();
        let entries = match std::fs::read_dir(&path) {
            Ok(entries) => entries,
            Err(err) => return (Arc::new(Mutex::new(Some::<Vec<fs_DirEntry>>(Vec::new()))), io_error(err)),
        };
        let mut result = Vec::new();
        for entry in entries {
            match entry {
                Ok(entry) => {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    let is_dir = entry.file_type().map(|file_type| file_type.is_dir()).unwrap_or(false);
                    result.push(fs_DirEntry { name, is_dir });
                }
                Err(err) => return (Arc::new(Mutex::new(Some::<Vec<fs_DirEntry>>(Vec::new()))), io_error(err)),
            }
        }
        result.sort_by(|left, right| left.name.cmp(&right.name));
        (Arc::new(Mutex::new(Some::<Vec<fs_DirEntry>>(result))), no_error())
    }

    pub fn stat<T0: GoStringArg>(_arg0: T0) -> (Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::metadata(&path) {
            Ok(metadata) => {
                let name = Path::new(&path).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_else(|| path.clone());
                (Arc::new(Mutex::new(Some::<fs_FileInfo>(fs_FileInfo { name, is_dir: metadata.is_dir(), size: metadata.len() as i64 }))), no_error())
            }
            Err(err) => (Arc::new(Mutex::new(Some::<fs_FileInfo>(fs_FileInfo::default()))), io_error(err)),
        }
    }
}


fn main() {
    let (mut entries, mut err) = os::read_dir("data".to_string());
    if (*err.lock().unwrap()).is_some() {
        println!("{}", format!("{}", "read error".to_string()));
        return;
    }

    { let __range_holder = entries.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for entry in __range_values.iter() {
        let mut joined = filepath::join(("data".to_string(), entry.name()));
        println!("{} {} {} {}", format!("{}", "entry".to_string()), format!("{}", { let __v = (*joined.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*filepath::base(joined.clone()).lock().unwrap().as_ref().unwrap())), format!("{}", entry.is_dir()));
    } }

    let (mut info, mut err) = os::stat("data/nested".to_string());
    if (*err.lock().unwrap()).is_some() {
        println!("{}", format!("{}", "stat error".to_string()));
        return;
    }
    println!("{} {}", format!("{}", "nested".to_string()), format!("{}", (*info.lock().unwrap().as_ref().unwrap()).is_dir()));
}