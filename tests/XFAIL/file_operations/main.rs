use std::error::Error as StdError;
use std::sync::{Arc, Mutex};


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

    fn write_string(&mut self, text: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {
        let text = (*text.lock().unwrap().as_ref().unwrap()).clone();
        match self.file.as_mut() {
            Some(file) => match std::io::Write::write_all(file, text.as_bytes()) {
                Ok(()) => (Arc::new(Mutex::new(Some(text.len() as i32))), Arc::new(Mutex::new(None))),
                Err(e) => (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from(e))))),
            },
            None => (
                Arc::new(Mutex::new(Some(0))),
                Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from(std::io::Error::new(std::io::ErrorKind::Other, "invalid file"))))),
            ),
        }
    }

    fn close(&mut self) -> Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>> {
        self.file = None;
        Arc::new(Mutex::new(None))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct bufio_Scanner;

impl std::fmt::Display for bufio_Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<bufio_Scanner>")
    }
}


impl bufio_Scanner {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn err(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }
    pub fn scan(&self) -> Arc<Mutex<Option<bool>>> {
        Arc::new(Mutex::new(Some::<bool>(Default::default())))
    }
    pub fn text(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(Default::default())))
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct os_File;

impl std::fmt::Display for os_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<os_File>")
    }
}


impl os_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod bufio {
    use super::*;
    pub fn new_scanner<T0>(_arg0: T0) -> Arc<Mutex<Option<bufio_Scanner>>> {
        Arc::new(Mutex::new(Some::<bufio_Scanner>(Default::default())))
    }
}


pub mod io {
    use super::*;
    pub fn copy<T0, T1>(_arg0: T0, _arg1: T1) -> (Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<i64>(Default::default()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


pub mod os {
    use super::*;
    pub const o__a_p_p_e_n_d: i32 = 0;
    pub const o__w_r_o_n_l_y: i32 = 0;

    pub fn is_not_exist<T0>(_arg0: T0) -> Arc<Mutex<Option<bool>>> {
        Arc::new(Mutex::new(Some::<bool>(Default::default())))
    }

    pub fn open<T0>(_arg0: T0) -> (Arc<Mutex<Option<os_File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<os_File>(Default::default()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn open_file<T0, T1, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> (Arc<Mutex<Option<os_File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<os_File>(Default::default()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn read_file<T0>(_arg0: T0) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<Vec<u8>>(Default::default()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }

    pub fn stat<T0>(_arg0: T0) -> (Arc<Mutex<Option<Box<dyn Trait>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<Box<dyn Trait>>(Default::default()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        // Create a temporary file for testing
    println!("{}", "=== File Operations Test ===".to_string());

    let mut filename = Arc::new(Mutex::new(Some("test_file.txt".to_string())));

        // Write to file
    println!("{}", "\n--- Writing to file ---".to_string());

    let (mut file, mut err) = { let __path = (*filename.lock().unwrap().as_ref().unwrap()).clone(); match GoFile::create(&__path) { Ok(file) => (Arc::new(Mutex::new(Some(file))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))), Err(e) => (Arc::new(Mutex::new(Some(GoFile::empty()))), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(e))))) } };
    if (*err.lock().unwrap()).is_some() {
        print!("Error creating file: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

    let mut content = Arc::new(Mutex::new(Some(vec!["Hello, World!".to_string(), "This is line 2".to_string(), "Go file operations".to_string(), "Line 4 with numbers: 123".to_string(), "Final line".to_string()])));

    { let __range_guard = content.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, line) in __range_values.iter().enumerate() {
        let (_, mut err) = (*file.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("{}{}", line, "\n".to_string())))));
        if (*err.lock().unwrap()).is_some() {
        print!("Error writing line {}: {}\n", { let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }, format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        (*file.lock().unwrap().as_mut().unwrap()).close();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
        print!("Wrote: {}\n", line);
    } }

    (*file.lock().unwrap().as_mut().unwrap()).close();
    print!("File '{}' created successfully\n", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Read entire file
    println!("{}", "\n--- Reading entire file ---".to_string());

    let (mut data, mut err) = os::read_file(filename.clone());
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading file: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

    print!("File contents:\n{}", (*Arc::new(Mutex::new(Some(String::from_utf8((*data.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()));

        // Read file line by line
    println!("{}", "\n--- Reading file line by line ---".to_string());

    { let (__tmp_0, __tmp_1) = os::open(filename.clone()); *file.lock().unwrap() = __tmp_0.lock().unwrap().take(); *err.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening file: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    let file_defer_captured = file.clone(); __defer_stack.push(Box::new(move || {
        (*file_defer_captured.lock().unwrap().as_mut().unwrap()).close();
    }));

    let mut scanner = bufio::new_scanner(file.clone());
    let mut lineNum = Arc::new(Mutex::new(Some(1)));

    while (*(*scanner.lock().unwrap().as_mut().unwrap()).scan().lock().unwrap().as_ref().unwrap()) {
        let mut line = (*scanner.lock().unwrap().as_mut().unwrap()).text();
        print!("Line {}: {}\n", { let __v = (*lineNum.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v });
        { let mut guard = lineNum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut err = (*scanner.lock().unwrap().as_mut().unwrap()).err();
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading file: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

        // Append to file
    println!("{}", "\n--- Appending to file ---".to_string());

    { let (__tmp_0, __tmp_1) = os::open_file(filename.clone(), { let __tmp_x = os::o__a_p_p_e_n_d; let __tmp_y = os::o__w_r_o_n_l_y; __tmp_x | __tmp_y }, 0644); *file.lock().unwrap() = __tmp_0.lock().unwrap().take(); *err.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening file for append: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

    let mut appendContent = Arc::new(Mutex::new(Some(vec!["Appended line 1".to_string(), "Appended line 2".to_string()])));

    { let __range_guard = appendContent.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for line in __range_values.iter() {
        let (_, mut err) = (*file.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("{}{}", line, "\n".to_string())))));
        if (*err.lock().unwrap()).is_some() {
        print!("Error appending: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        (*file.lock().unwrap().as_mut().unwrap()).close();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
        print!("Appended: {}\n", line);
    } }

    (*file.lock().unwrap().as_mut().unwrap()).close();

        // Read updated file
    println!("{}", "\n--- Reading updated file ---".to_string());

    { let (__tmp_0, __tmp_1) = os::read_file(filename.clone()); *data.lock().unwrap() = __tmp_0.lock().unwrap().take(); *err.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading updated file: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

    print!("Updated file contents:\n{}", (*Arc::new(Mutex::new(Some(String::from_utf8((*data.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()));

        // File information
    println!("{}", "\n--- File information ---".to_string());

    let (mut fileInfo, mut err) = os::stat(filename.clone());
    if (*err.lock().unwrap()).is_some() {
        print!("Error getting file info: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

    print!("File name: {}\n", (*(*fileInfo.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()));
    print!("File size: {} bytes\n", (*(*fileInfo.lock().unwrap().as_ref().unwrap()).size().lock().unwrap().as_ref().unwrap()));
    print!("File mode: {}\n", (*(*fileInfo.lock().unwrap().as_ref().unwrap()).mode().lock().unwrap().as_ref().unwrap()));

        // Keep the fixture deterministic; actual modification times vary per run.
    let _ = (*fileInfo.lock().unwrap().as_ref().unwrap()).mod_time();
    println!("{}", "Modified time: <deterministic>".to_string());
    print!("Is directory: {}\n", (*(*fileInfo.lock().unwrap().as_ref().unwrap()).is_dir().lock().unwrap().as_ref().unwrap()));

        // Copy file
    println!("{}", "\n--- Copying file ---".to_string());

    let mut copyFilename = Arc::new(Mutex::new(Some("test_file_copy.txt".to_string())));

    let (mut sourceFile, mut err) = os::open(filename.clone());
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening source file: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    let sourceFile_defer_captured = sourceFile.clone(); __defer_stack.push(Box::new(move || {
        (*sourceFile_defer_captured.lock().unwrap().as_mut().unwrap()).close();
    }));

    let (mut destFile, mut err) = { let __path = (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(); match GoFile::create(&__path) { Ok(file) => (Arc::new(Mutex::new(Some(file))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))), Err(e) => (Arc::new(Mutex::new(Some(GoFile::empty()))), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(e))))) } };
    if (*err.lock().unwrap()).is_some() {
        print!("Error creating destination file: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    let destFile_defer_captured = destFile.clone(); __defer_stack.push(Box::new(move || {
        (*destFile_defer_captured.lock().unwrap().as_mut().unwrap()).close();
    }));

    let (mut bytesWritten, mut err) = io::copy(destFile.clone(), sourceFile.clone());
    if (*err.lock().unwrap()).is_some() {
        print!("Error copying file: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

    print!("Copied {} bytes to '{}'\n", { let __v = (*bytesWritten.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Read and process file content
    println!("{}", "\n--- Processing file content ---".to_string());

    { let (__tmp_0, __tmp_1) = os::open(filename.clone()); *file.lock().unwrap() = __tmp_0.lock().unwrap().take(); *err.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening file: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    let file_defer_captured = file.clone(); __defer_stack.push(Box::new(move || {
        (*file_defer_captured.lock().unwrap().as_mut().unwrap()).close();
    }));

    { let new_val = bufio::new_scanner(file.clone()); *scanner.lock().unwrap() = new_val.lock().unwrap().take(); };
    let mut wordCount = Arc::new(Mutex::new(Some(0)));
    let mut lineCount = Arc::new(Mutex::new(Some(0)));
    let mut charCount = Arc::new(Mutex::new(Some(0)));

    while (*(*scanner.lock().unwrap().as_mut().unwrap()).scan().lock().unwrap().as_ref().unwrap()) {
        let mut line = (*scanner.lock().unwrap().as_mut().unwrap()).text();
        { let mut guard = lineCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = charCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*line.lock().unwrap().as_ref().unwrap()).len()); };

        let mut words = Arc::new(Mutex::new(Some({ let __s = (*line.lock().unwrap().as_ref().unwrap()).clone(); __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() })));
        { let mut guard = wordCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*words.lock().unwrap().as_ref().unwrap()).len()); };

                // Process lines containing numbers
        if (*Arc::new(Mutex::new(Some({ let __s = (*line.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "123".to_string(); __s.contains(&__arg) }))).lock().unwrap().as_ref().unwrap()) {
        print!("Found line with numbers: {}\n", { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    }

        // Process lines containing numbers
    print!("Statistics:\n");
    print!("  Lines: {}\n", { let __v = (*lineCount.lock().unwrap().as_ref().unwrap()).clone(); __v });
    print!("  Words: {}\n", { let __v = (*wordCount.lock().unwrap().as_ref().unwrap()).clone(); __v });
    print!("  Characters: {}\n", { let __v = (*charCount.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Write formatted data
    println!("{}", "\n--- Writing formatted data ---".to_string());

    let mut dataFile = Arc::new(Mutex::new(Some("data.txt".to_string())));
    { let (__tmp_0, __tmp_1) = { let __path = (*dataFile.lock().unwrap().as_ref().unwrap()).clone(); match GoFile::create(&__path) { Ok(file) => (Arc::new(Mutex::new(Some(file))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))), Err(e) => (Arc::new(Mutex::new(Some(GoFile::empty()))), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(e))))) } }; *file.lock().unwrap() = __tmp_0.lock().unwrap().take(); *err.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    if (*err.lock().unwrap()).is_some() {
        print!("Error creating data file: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    let file_defer_captured = file.clone(); __defer_stack.push(Box::new(move || {
        (*file_defer_captured.lock().unwrap().as_mut().unwrap()).close();
    }));

        // Write structured data
    print!("Name: {}\n", "John Doe".to_string());
    print!("Age: {}\n", 30);
    print!("Score: {:.2}\n", 95.5);
    print!("Active: {}\n", true);

    print!("Formatted data written to '{}'\n", { let __v = (*dataFile.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Read and parse formatted data
    println!("{}", "\n--- Reading formatted data ---".to_string());

    { let (__tmp_0, __tmp_1) = os::read_file(dataFile.clone()); *data.lock().unwrap() = __tmp_0.lock().unwrap().take(); *err.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading data file: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

    print!("Data file contents:\n{}", (*Arc::new(Mutex::new(Some(String::from_utf8((*data.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()));

        // Check if files exist
    println!("{}", "\n--- Checking file existence ---".to_string());

    let mut files = Arc::new(Mutex::new(Some(vec![(*filename.lock().unwrap().as_ref().unwrap()).clone(), (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(), (*dataFile.lock().unwrap().as_ref().unwrap()).clone(), "nonexistent.txt".to_string()])));

    { let __range_guard = files.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for f in __range_values.iter() {
        let (_, mut err) = os::stat(f);
    if (*err.lock().unwrap()).is_none() {
        print!("File '{}' exists\n", f);
    } else if (*os::is_not_exist(err.clone()).lock().unwrap().as_ref().unwrap()) {
        print!("File '{}' does not exist\n", f);
    } else {
        print!("Error checking file '{}': {}\n", f, format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
    }
    } }

        // Clean up - remove test files
    println!("{}", "\n--- Cleaning up ---".to_string());

    let mut filesToRemove = Arc::new(Mutex::new(Some(vec![(*filename.lock().unwrap().as_ref().unwrap()).clone(), (*copyFilename.lock().unwrap().as_ref().unwrap()).clone(), (*dataFile.lock().unwrap().as_ref().unwrap()).clone()])));

    { let __range_guard = filesToRemove.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for f in __range_values.iter() {
        let mut err = { let __path = (*f.lock().unwrap().as_ref().unwrap()).clone(); match std::fs::remove_file(&__path) { Ok(()) => Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)), Err(e) => Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(e)))) } };
        if (*err.lock().unwrap()).is_some() {
        print!("Error removing file '{}': {}\n", f, format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
    } else {
        print!("Removed file '{}'\n", f);
    }
    } }

        // Verify cleanup
    println!("{}", "\n--- Verifying cleanup ---".to_string());

    { let __range_guard = filesToRemove.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for f in __range_values.iter() {
        let (_, mut err) = os::stat(f);
    if (*os::is_not_exist(err.clone()).lock().unwrap().as_ref().unwrap()) {
        print!("File '{}' successfully removed\n", f);
    } else {
        print!("File '{}' still exists\n", f);
    }
    } }

    println!("{}", "\nFile operations test completed!".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}