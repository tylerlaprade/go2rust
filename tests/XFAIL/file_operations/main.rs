#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};


thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send + Sync>>> = RefCell::new(None);
}

fn go_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Arc::new(Mutex::new(slot.borrow_mut().take())))
}

fn go_store_panic_payload(payload: Box<dyn Any + Send>) {
    let payload = match payload.downcast::<Box<dyn Any + Send + Sync>>() {
        Ok(boxed) => {
            let mut payload = *boxed;
            loop {
                match payload.downcast::<Box<dyn Any + Send + Sync>>() {
                    Ok(boxed) => {
                        payload = *boxed;
                    }
                    Err(payload) => {
                        __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(payload));
                        return;
                    }
                }
            }
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<String>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(*value)) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(value.to_string())) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i32>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i64>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let _payload = match payload.downcast::<bool>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(_payload) => _payload,
    };
    panic!("recover: unsupported Rust panic payload; emit panic_any with a Go any payload instead")
}

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    __text
}

thread_local! {
    static __GO_PANIC_HOOK_SILENCE: std::cell::Cell<usize> = std::cell::Cell::new(0);
}

fn go_silence_panic_hook() -> usize {
    static __GO_PANIC_HOOK: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_HOOK.call_once(|| {
        let __previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |__info| {
            if __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.get()) == 0 {
                __previous(__info);
            }
        }));
    });
    __GO_PANIC_HOOK_SILENCE.with(|__depth| {
        let __previous = __depth.get();
        __depth.set(__previous + 1);
        __previous
    })
}

fn go_restore_panic_hook(__previous: usize) {
    __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.set(__previous));
}

fn go_resume_unrecovered_panic() {
    if let Some(payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        std::panic::panic_any(payload);
    }
}

struct GoFile {
    file: Option<std::fs::File>,
}

impl GoFile {
    fn create(path: &str) -> Result<Self, std::io::Error> {
        std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
            .map(|file| GoFile { file: Some(file) })
    }

    fn empty() -> Self {
        GoFile { file: None }
    }

    fn write_string(&mut self, text: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>) {
        let text = (*text.lock().unwrap().as_ref().unwrap()).clone();
        match self.file.as_mut() {
            Some(file) => match std::io::Write::write_all(file, text.as_bytes()) {
                Ok(()) => (Arc::new(Mutex::new(Some(text.len() as i64))), Arc::new(Mutex::new(None))),
                Err(e) => (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(std::sync::Arc::<dyn std::error::Error + Send + Sync>::from(Box::<dyn std::error::Error + Send + Sync>::from(e)))))),
            },
            None => (
                Arc::new(Mutex::new(Some(0))),
                Arc::new(Mutex::new(Some(std::sync::Arc::<dyn std::error::Error + Send + Sync>::from(Box::<dyn std::error::Error + Send + Sync>::from(std::io::Error::new(std::io::ErrorKind::Other, "invalid file")))))),
            ),
        }
    }

    fn close(&mut self) -> Arc<Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>> {
        self.file = None;
        Arc::new(Mutex::new(None))
    }
}

fn main() {
    ::bufio::__go_init_all();
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        println!("{}", format!("{}", "=== File Operations Test ===".to_string()));
        let mut filename = Arc::new(Mutex::new(Some("test_file.txt".to_string())));
        println!("{}", format!("{}", "\n--- Writing to file ---".to_string()));
        let (mut file, mut err) = { let __path = (*filename.lock().unwrap().as_ref().unwrap()).clone(); match GoFile::create(&__path) { Ok(file) => (Arc::new(Mutex::new(Some(file))), Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (Arc::new(Mutex::new(Some(GoFile::empty()))), Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error creating file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let mut content = Arc::new(Mutex::new(Some(vec!["Hello, World!".to_string(), "This is line 2".to_string(), "Go file operations".to_string(), "Line 4 with numbers: 123".to_string(), "Final line".to_string()])));
        { let __range_holder = content.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, line) in __range_values.iter().enumerate() {
        let (_, mut err) = { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.write_string(Arc::new(Mutex::new(Some(format!("{}{}", line, "\n".to_string()))))); __result };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error writing line {}: {}\n", format!("{}", { let __tmp_x = i as i64; let __tmp_y = 1; __tmp_x + __tmp_y }), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        print!("Wrote: {}\n", line);
    } }
        { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
        print!("File '{}' created successfully\n", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v });
        println!("{}", format!("{}", "\n--- Reading entire file ---".to_string()));
        let (mut data, __tmp_1) = os::read_file({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        let mut data = { let __slice_header = GoSliceHeader::go_slice_header(&data); Arc::new(Mutex::new(__slice_header)) };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error reading file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        print!("File contents:\n{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*data.lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap()));
        println!("{}", format!("{}", "\n--- Reading file line by line ---".to_string()));
        { let (__tmp_0, __tmp_1) = os::open({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); file = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error opening file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let file_defer_captured = file.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = file_defer_captured.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    }));
        let mut scanner = bufio::new_scanner(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = file.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn io::r#mod::Reader + Send + Sync>))));
        let mut lineNum = Arc::new(Mutex::new(Some(1i64)));
        while { let __recv = scanner.clone(); let __recv_ptr: *mut bufio::scan::Scanner = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio::scan::Scanner }; let __result = unsafe { &mut *__recv_ptr }.scan(); __result } {
        let mut line = { let __recv = scanner.clone(); let __recv_ptr: *const bufio::scan::Scanner = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const bufio::scan::Scanner }; let __result = unsafe { &*__recv_ptr }.text(); __result };
        print!("Line {}: {}\n", format!("{}", { let __v = (*lineNum.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        { let mut guard = lineNum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        {
        let mut err = { let __recv = scanner.clone(); let __recv_ptr: *const bufio::scan::Scanner = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const bufio::scan::Scanner }; let __result = unsafe { &*__recv_ptr }.err(); __result };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            print!("Error reading file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });;
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    };
        }
    }
        println!("{}", format!("{}", "\n--- Appending to file ---".to_string()));
        { let (__tmp_0, __tmp_1) = os::open_file({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, 1025, fs_FileMode(0644 as u32)); file = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error opening file for append: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let mut appendContent = Arc::new(Mutex::new(Some(vec!["Appended line 1".to_string(), "Appended line 2".to_string()])));
        { let __range_holder = appendContent.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for line in __range_values.iter() {
        let (_, mut err) = { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.write_string(Arc::new(Mutex::new(Some(format!("{}{}", line, "\n".to_string()))))); __result };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error appending: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        print!("Appended: {}\n", line);
    } }
        { let __recv = file.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
        println!("{}", format!("{}", "\n--- Reading updated file ---".to_string()));
        { let (__tmp_0, __tmp_1) = os::read_file({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *data.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error reading updated file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        print!("Updated file contents:\n{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*data.lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap()));
        println!("{}", format!("{}", "\n--- File information ---".to_string()));
        let (mut fileInfo, __tmp_1) = os::stat({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error getting file info: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        print!("File name: {}\n", (*(*fileInfo.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()));
        print!("File size: {} bytes\n", (*fileInfo.lock().unwrap().as_ref().unwrap()).size());
        print!("File mode: {}\n", (*(*fileInfo.lock().unwrap().as_ref().unwrap()).mode().lock().unwrap().as_ref().unwrap()));
        let _ = (*fileInfo.lock().unwrap().as_ref().unwrap()).mod_time();
        println!("{}", format!("{}", "Modified time: <deterministic>".to_string()));
        print!("Is directory: {}\n", (*fileInfo.lock().unwrap().as_ref().unwrap()).is_dir());
        println!("{}", format!("{}", "\n--- Copying file ---".to_string()));
        let mut copyFilename = Arc::new(Mutex::new(Some("test_file_copy.txt".to_string())));
        let (mut sourceFile, __tmp_1) = os::open({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error opening source file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let sourceFile_defer_captured = sourceFile.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = sourceFile_defer_captured.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    }));
        let (mut destFile, __tmp_1) = { let __path = (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(); match GoFile::create(&__path) { Ok(file) => (Arc::new(Mutex::new(Some(file))), Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (Arc::new(Mutex::new(Some(GoFile::empty()))), Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error creating destination file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let destFile_defer_captured = destFile.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = destFile_defer_captured.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    }));
        let (mut bytesWritten, __tmp_1) = io::copy(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = destFile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn io::r#mod::Writer + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = sourceFile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn io::r#mod::Reader + Send + Sync>)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error copying file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        print!("Copied {} bytes to '{}'\n", format!("{}", bytesWritten), format!("{}", { let __v = (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        println!("{}", format!("{}", "\n--- Processing file content ---".to_string()));
        { let (__tmp_0, __tmp_1) = os::open({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); file = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error opening file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let file_defer_captured = file.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = file_defer_captured.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    }));
        { let new_val = bufio::new_scanner(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = file.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn io::r#mod::Reader + Send + Sync>)))).clone(); scanner = new_val; };
        let mut wordCount = Arc::new(Mutex::new(Some(0i64)));
        let mut lineCount = Arc::new(Mutex::new(Some(0i64)));
        let mut charCount = Arc::new(Mutex::new(Some(0i64)));
        while { let __recv = scanner.clone(); let __recv_ptr: *mut bufio::scan::Scanner = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio::scan::Scanner }; let __result = unsafe { &mut *__recv_ptr }.scan(); __result } {
        let mut line = { let __recv = scanner.clone(); let __recv_ptr: *const bufio::scan::Scanner = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const bufio::scan::Scanner }; let __result = unsafe { &*__recv_ptr }.text(); __result };
        { let mut guard = lineCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = ({ let __len = (*line.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64; let mut guard = charCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        let mut words = strings::fields(Arc::new(Mutex::new(Some({ let __arg_holder = line.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut words = { let __slice_header = GoSliceHeader::go_slice_header(&words); Arc::new(Mutex::new(__slice_header)) };
        { let __rhs = ({ let __len = (*words.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64; let mut guard = wordCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if strings::contains(Arc::new(Mutex::new(Some({ let __arg_holder = line.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("123".to_string())))) {
        print!("Found line with numbers: {}\n", { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    }
        print!("Statistics:\n");
        print!("  Lines: {}\n", { let __v = (*lineCount.lock().unwrap().as_ref().unwrap()).clone(); __v });
        print!("  Words: {}\n", { let __v = (*wordCount.lock().unwrap().as_ref().unwrap()).clone(); __v });
        print!("  Characters: {}\n", { let __v = (*charCount.lock().unwrap().as_ref().unwrap()).clone(); __v });
        println!("{}", format!("{}", "\n--- Writing formatted data ---".to_string()));
        let mut dataFile = Arc::new(Mutex::new(Some("data.txt".to_string())));
        { let (__tmp_0, __tmp_1) = { let __path = (*dataFile.lock().unwrap().as_ref().unwrap()).clone(); match GoFile::create(&__path) { Ok(file) => (Arc::new(Mutex::new(Some(file))), Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>))), Err(e) => (Arc::new(Mutex::new(Some(GoFile::empty()))), Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e)))))) } }; file = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error creating data file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        let file_defer_captured = file.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = file_defer_captured.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    }));
        { let __s = format!("Name: {}\n", "John Doe".to_string()); (*file.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let __s = format!("Age: {}\n", 30); (*file.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let __s = format!("Score: {:.2}\n", 95.5); (*file.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let __s = format!("Active: {}\n", true); (*file.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        print!("Formatted data written to '{}'\n", { let __v = (*dataFile.lock().unwrap().as_ref().unwrap()).clone(); __v });
        println!("{}", format!("{}", "\n--- Reading formatted data ---".to_string()));
        { let (__tmp_0, __tmp_1) = os::read_file({ let __arg_holder = dataFile.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *data.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error reading data file: {}\n", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        print!("Data file contents:\n{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*data.lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap()));
        println!("{}", format!("{}", "\n--- Checking file existence ---".to_string()));
        let mut files = Arc::new(Mutex::new(Some(vec![({ let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v }), ({ let __v = (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(); __v }), ({ let __v = (*dataFile.lock().unwrap().as_ref().unwrap()).clone(); __v }), "nonexistent.txt".to_string()])));
        { let __range_holder = files.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for f in __range_values.iter() {
        {
        let (_, mut err) = os::stat((*f).clone());;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            print!("File '{}' exists\n", f);;
        } else if os::is_not_exist({ let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }) {
        print!("File '{}' does not exist\n", f);
    } else {
        print!("Error checking file '{}': {}\n", format!("{}", f), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }
    }
    } }
        println!("{}", format!("{}", "\n--- Cleaning up ---".to_string()));
        let mut filesToRemove = Arc::new(Mutex::new(Some(vec![({ let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v }), ({ let __v = (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(); __v }), ({ let __v = (*dataFile.lock().unwrap().as_ref().unwrap()).clone(); __v })])));
        { let __range_holder = filesToRemove.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for f in __range_values.iter() {
        let mut err = { let __path = (*f).clone(); match std::fs::remove_file(&__path) { Ok(()) => Arc::new(Mutex::new(None::<Arc<dyn StdError + Send + Sync>>)), Err(e) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(e))))) } };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("Error removing file '{}': {}\n", format!("{}", f), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    } else {
        print!("Removed file '{}'\n", f);
    }
    } }
        println!("{}", format!("{}", "\n--- Verifying cleanup ---".to_string()));
        { let __range_holder = filesToRemove.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for f in __range_values.iter() {
        {
        let (_, mut err) = os::stat((*f).clone());;
        if os::is_not_exist({ let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }) {
            print!("File '{}' successfully removed\n", f);;
        } else {
            print!("File '{}' still exists\n", f);;
        }
    }
    } }
        println!("{}", format!("{}", "\nFile operations test completed!".to_string()));

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            go_restore_panic_hook(__go_previous_panic_hook);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                    go_store_panic_payload(__go_defer_panic);
                }
            }
            go_restore_panic_hook(__go_previous_panic_hook);
            go_resume_unrecovered_panic();
            ()
        }
    }
}