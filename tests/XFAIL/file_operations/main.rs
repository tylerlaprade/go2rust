use std::sync::{Arc, Mutex};

fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        // Create a temporary file for testing
    println!("{}", "=== File Operations Test ===".to_string());

    let mut filename = Arc::new(Mutex::new(Some("test_file.txt".to_string())));

        // Write to file
    println!("{}", "\n--- Writing to file ---".to_string());

    let (mut file, mut err) = os.create(Arc::new(Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error creating file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

    let mut content = Arc::new(Mutex::new(Some(vec!["Hello, World!".to_string(), "This is line 2".to_string(), "Go file operations".to_string(), "Line 4 with numbers: 123".to_string(), "Final line".to_string()])));

    for (i, line) in (*content.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        let (_, mut err) = (*file.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("{}{}", line, "\n".to_string())))));
        if (*err.lock().unwrap()).is_some() {
        print!("Error writing line {}: {}\n", i + 1, (*err.lock().unwrap().as_mut().unwrap()));
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
    }

    (*file.lock().unwrap().as_mut().unwrap()).close();
    print!("File '{}' created successfully\n", (*filename.lock().unwrap().as_mut().unwrap()));

        // Read entire file
    println!("{}", "\n--- Reading entire file ---".to_string());

    let (mut data, mut err) = os.read_file(Arc::new(Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
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

    (file, err) = os.open(Arc::new(Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    __defer_stack.push(Box::new(move || {
        (*file.lock().unwrap().as_mut().unwrap()).close();
    }));

    let mut scanner = bufio.new_scanner(Arc::new(Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap())))));
    let mut lineNum = Arc::new(Mutex::new(Some(1)));

    while (*scanner.lock().unwrap().as_mut().unwrap()).scan() {
        let mut line = (*scanner.lock().unwrap().as_mut().unwrap()).text();
        print!("Line {}: {}\n", (*lineNum.lock().unwrap().as_mut().unwrap()), (*line.lock().unwrap().as_mut().unwrap()));
        { let mut guard = lineNum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut err = (*scanner.lock().unwrap().as_mut().unwrap()).err();
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
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

    (file, err) = os.open_file(Arc::new(Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some((*(*os.lock().unwrap().as_mut().unwrap())::o__a_p_p_e_n_d.lock().unwrap().as_ref().unwrap()) | (*(*os.lock().unwrap().as_mut().unwrap())::o__w_r_o_n_l_y.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some(0644))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening file for append: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

    let mut appendContent = Arc::new(Mutex::new(Some(vec!["Appended line 1".to_string(), "Appended line 2".to_string()])));

    for line in &(*appendContent.lock().unwrap().as_mut().unwrap()) {
        let (_, mut err) = (*file.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("{}{}", line, "\n".to_string())))));
        if (*err.lock().unwrap()).is_some() {
        print!("Error appending: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
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
    }

    (*file.lock().unwrap().as_mut().unwrap()).close();

        // Read updated file
    println!("{}", "\n--- Reading updated file ---".to_string());

    (data, err) = os.read_file(Arc::new(Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading updated file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
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

    let (mut fileInfo, mut err) = os.stat(Arc::new(Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error getting file info: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

    print!("File name: {}\n", (*fileInfo.name().lock().unwrap().as_ref().unwrap()));
    print!("File size: {} bytes\n", (*fileInfo.size().lock().unwrap().as_ref().unwrap()));
    print!("File mode: {}\n", (*fileInfo.mode().lock().unwrap().as_ref().unwrap()));
    print!("Modified time: {}\n", (*fileInfo.mod_time().lock().unwrap().as_ref().unwrap()));
    print!("Is directory: {}\n", (*fileInfo.is_dir().lock().unwrap().as_ref().unwrap()));

        // Copy file
    println!("{}", "\n--- Copying file ---".to_string());

    let mut copyFilename = Arc::new(Mutex::new(Some("test_file_copy.txt".to_string())));

    let (mut sourceFile, mut err) = os.open(Arc::new(Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening source file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    __defer_stack.push(Box::new(move || {
        (*sourceFile.lock().unwrap().as_mut().unwrap()).close();
    }));

    let (mut destFile, mut err) = os.create(Arc::new(Mutex::new(Some((*copyFilename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error creating destination file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    __defer_stack.push(Box::new(move || {
        (*destFile.lock().unwrap().as_mut().unwrap()).close();
    }));

    let (mut bytesWritten, mut err) = io.copy(Arc::new(Mutex::new(Some((*destFile.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some((*sourceFile.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error copying file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }

    print!("Copied {} bytes to '{}'\n", (*bytesWritten.lock().unwrap().as_mut().unwrap()), (*copyFilename.lock().unwrap().as_mut().unwrap()));

        // Read and process file content
    println!("{}", "\n--- Processing file content ---".to_string());

    (file, err) = os.open(Arc::new(Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    __defer_stack.push(Box::new(move || {
        (*file.lock().unwrap().as_mut().unwrap()).close();
    }));

    { let new_val = bufio.new_scanner(Arc::new(Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap()))))); *scanner.lock().unwrap() = Some(new_val); };
    let mut wordCount = Arc::new(Mutex::new(Some(0)));
    let mut lineCount = Arc::new(Mutex::new(Some(0)));
    let mut charCount = Arc::new(Mutex::new(Some(0)));

    while (*scanner.lock().unwrap().as_mut().unwrap()).scan() {
        let mut line = (*scanner.lock().unwrap().as_mut().unwrap()).text();
        { let mut guard = lineCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = charCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*line.lock().unwrap().as_ref().unwrap()).len()); };

        let mut words = strings.fields(Arc::new(Mutex::new(Some((*line.lock().unwrap().as_mut().unwrap())))));
        { let mut guard = wordCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*words.lock().unwrap().as_ref().unwrap()).len()); };

                // Process lines containing numbers
        if strings.contains(Arc::new(Mutex::new(Some((*line.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("123".to_string())))) {
        print!("Found line with numbers: {}\n", (*line.lock().unwrap().as_mut().unwrap()));
    }
    }

        // Process lines containing numbers
    print!("Statistics:\n");
    print!("  Lines: {}\n", (*lineCount.lock().unwrap().as_mut().unwrap()));
    print!("  Words: {}\n", (*wordCount.lock().unwrap().as_mut().unwrap()));
    print!("  Characters: {}\n", (*charCount.lock().unwrap().as_mut().unwrap()));

        // Write formatted data
    println!("{}", "\n--- Writing formatted data ---".to_string());

    let mut dataFile = Arc::new(Mutex::new(Some("data.txt".to_string())));
    (file, err) = os.create(Arc::new(Mutex::new(Some((*dataFile.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error creating data file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    __defer_stack.push(Box::new(move || {
        (*file.lock().unwrap().as_mut().unwrap()).close();
    }));

        // Write structured data
    fmt.fprintf(Arc::new(Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("Name: %s\n".to_string()))), Arc::new(Mutex::new(Some("John Doe".to_string()))));
    fmt.fprintf(Arc::new(Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("Age: %d\n".to_string()))), Arc::new(Mutex::new(Some(30))));
    fmt.fprintf(Arc::new(Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("Score: %.2f\n".to_string()))), Arc::new(Mutex::new(Some(95.5))));
    fmt.fprintf(Arc::new(Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("Active: %t\n".to_string()))), Arc::new(Mutex::new(Some(true))));

    print!("Formatted data written to '{}'\n", (*dataFile.lock().unwrap().as_mut().unwrap()));

        // Read and parse formatted data
    println!("{}", "\n--- Reading formatted data ---".to_string());

    (data, err) = os.read_file(Arc::new(Mutex::new(Some((*dataFile.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading data file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
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

    let mut files = Arc::new(Mutex::new(Some(vec![(*filename.lock().unwrap().as_mut().unwrap()), (*copyFilename.lock().unwrap().as_mut().unwrap()), (*dataFile.lock().unwrap().as_mut().unwrap()), "nonexistent.txt".to_string()])));

    for f in &(*files.lock().unwrap().as_mut().unwrap()) {
        let (_, mut err) = os.stat(Arc::new(Mutex::new(Some(f))));
    if (*err.lock().unwrap()).is_none() {
        print!("File '{}' exists\n", f);
    } else if os.is_not_exist(Arc::new(Mutex::new(Some((*err.lock().unwrap().as_mut().unwrap()))))) {
        print!("File '{}' does not exist\n", f);
    } else {
        print!("Error checking file '{}': {}\n", f, (*err.lock().unwrap().as_mut().unwrap()));
    }
    }

        // Clean up - remove test files
    println!("{}", "\n--- Cleaning up ---".to_string());

    let mut filesToRemove = Arc::new(Mutex::new(Some(vec![(*filename.lock().unwrap().as_mut().unwrap()), (*copyFilename.lock().unwrap().as_mut().unwrap()), (*dataFile.lock().unwrap().as_mut().unwrap())])));

    for f in &(*filesToRemove.lock().unwrap().as_mut().unwrap()) {
        let mut err = os.remove(Arc::new(Mutex::new(Some(f))));
        if (*err.lock().unwrap()).is_some() {
        print!("Error removing file '{}': {}\n", f, (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        print!("Removed file '{}'\n", f);
    }
    }

        // Verify cleanup
    println!("{}", "\n--- Verifying cleanup ---".to_string());

    for f in &(*filesToRemove.lock().unwrap().as_mut().unwrap()) {
        let (_, mut err) = os.stat(Arc::new(Mutex::new(Some(f))));
    if os.is_not_exist(Arc::new(Mutex::new(Some((*err.lock().unwrap().as_mut().unwrap()))))) {
        print!("File '{}' successfully removed\n", f);
    } else {
        print!("File '{}' still exists\n", f);
    }
    }

    println!("{}", "\nFile operations test completed!".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}