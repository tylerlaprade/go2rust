fn main() {
    println!("{}", "=== File Operations Test ===".to_string());
    let mut filename = std::sync::Arc::new(std::sync::Mutex::new(Some("test_file.txt".to_string())));
    println!("{}", "\n--- Writing to file ---".to_string());
    let (mut file, mut err) = (*os.lock().unwrap().as_mut().unwrap()).create(std::sync::Arc::new(std::sync::Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error creating file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    let mut content = std::sync::Arc::new(std::sync::Mutex::new(Some(vec!["Hello, World!".to_string(), "This is line 2".to_string(), "Go file operations".to_string(), "Line 4 with numbers: 123".to_string(), "Final line".to_string()])));
    for (i, line) in (*content.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        let (_, mut err) = (*file.lock().unwrap().as_mut().unwrap()).write_string(std::sync::Arc::new(std::sync::Mutex::new(Some(format!("{}{}", line, "\n".to_string())))));
        if (*err.lock().unwrap()).is_some() {
        print!("Error writing line {}: {}\n", i + 1, (*err.lock().unwrap().as_mut().unwrap()));
        (*file.lock().unwrap().as_mut().unwrap()).close();
        return;
    }
        print!("Wrote: {}\n", line);
    }
    (*file.lock().unwrap().as_mut().unwrap()).close();
    print!("File '{}' created successfully\n", (*filename.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n--- Reading entire file ---".to_string());
    let (mut data, mut err) = (*os.lock().unwrap().as_mut().unwrap()).read_file(std::sync::Arc::new(std::sync::Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    print!("File contents:\n{}", string(data.clone()));
    println!("{}", "\n--- Reading file line by line ---".to_string());
    (file, err) = (*os.lock().unwrap().as_mut().unwrap()).open(std::sync::Arc::new(std::sync::Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    // defer (*file.lock().unwrap().as_mut().unwrap()).close() // TODO: defer not yet supported
    let mut scanner = (*bufio.lock().unwrap().as_mut().unwrap()).new_scanner(std::sync::Arc::new(std::sync::Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap())))));
    let mut lineNum = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*scanner.lock().unwrap().as_mut().unwrap()).scan() {
        let mut line = (*scanner.lock().unwrap().as_mut().unwrap()).text();
        print!("Line {}: {}\n", (*lineNum.lock().unwrap().as_mut().unwrap()), (*line.lock().unwrap().as_mut().unwrap()));
        { let mut guard = lineNum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut err = (*scanner.lock().unwrap().as_mut().unwrap()).err();
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    println!("{}", "\n--- Appending to file ---".to_string());
    (file, err) = (*os.lock().unwrap().as_mut().unwrap()).open_file(std::sync::Arc::new(std::sync::Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*os.lock().unwrap().as_mut().unwrap()).o__a_p_p_e_n_d | (*os.lock().unwrap().as_mut().unwrap()).o__w_r_o_n_l_y))), std::sync::Arc::new(std::sync::Mutex::new(Some(0644))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening file for append: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    let mut appendContent = std::sync::Arc::new(std::sync::Mutex::new(Some(vec!["Appended line 1".to_string(), "Appended line 2".to_string()])));
    for line in &(*appendContent.lock().unwrap().as_mut().unwrap()) {
        let (_, mut err) = (*file.lock().unwrap().as_mut().unwrap()).write_string(std::sync::Arc::new(std::sync::Mutex::new(Some(format!("{}{}", line, "\n".to_string())))));
        if (*err.lock().unwrap()).is_some() {
        print!("Error appending: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        (*file.lock().unwrap().as_mut().unwrap()).close();
        return;
    }
        print!("Appended: {}\n", line);
    }
    (*file.lock().unwrap().as_mut().unwrap()).close();
    println!("{}", "\n--- Reading updated file ---".to_string());
    (data, err) = (*os.lock().unwrap().as_mut().unwrap()).read_file(std::sync::Arc::new(std::sync::Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading updated file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    print!("Updated file contents:\n{}", string(data.clone()));
    println!("{}", "\n--- File information ---".to_string());
    let (mut fileInfo, mut err) = (*os.lock().unwrap().as_mut().unwrap()).stat(std::sync::Arc::new(std::sync::Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error getting file info: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    print!("File name: {}\n", (*fileInfo.lock().unwrap().as_mut().unwrap()).name());
    print!("File size: {} bytes\n", (*fileInfo.lock().unwrap().as_mut().unwrap()).size());
    print!("File mode: {}\n", (*fileInfo.lock().unwrap().as_mut().unwrap()).mode());
    print!("Modified time: {}\n", (*fileInfo.lock().unwrap().as_mut().unwrap()).mod_time());
    print!("Is directory: {}\n", (*fileInfo.lock().unwrap().as_mut().unwrap()).is_dir());
    println!("{}", "\n--- Copying file ---".to_string());
    let mut copyFilename = std::sync::Arc::new(std::sync::Mutex::new(Some("test_file_copy.txt".to_string())));
    let (mut sourceFile, mut err) = (*os.lock().unwrap().as_mut().unwrap()).open(std::sync::Arc::new(std::sync::Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening source file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    // defer (*sourceFile.lock().unwrap().as_mut().unwrap()).close() // TODO: defer not yet supported
    let (mut destFile, mut err) = (*os.lock().unwrap().as_mut().unwrap()).create(std::sync::Arc::new(std::sync::Mutex::new(Some((*copyFilename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error creating destination file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    // defer (*destFile.lock().unwrap().as_mut().unwrap()).close() // TODO: defer not yet supported
    let (mut bytesWritten, mut err) = (*io.lock().unwrap().as_mut().unwrap()).copy(std::sync::Arc::new(std::sync::Mutex::new(Some((*destFile.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*sourceFile.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error copying file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    print!("Copied {} bytes to '{}'\n", (*bytesWritten.lock().unwrap().as_mut().unwrap()), (*copyFilename.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n--- Processing file content ---".to_string());
    (file, err) = (*os.lock().unwrap().as_mut().unwrap()).open(std::sync::Arc::new(std::sync::Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error opening file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    // defer (*file.lock().unwrap().as_mut().unwrap()).close() // TODO: defer not yet supported
    { let new_val = (*bufio.lock().unwrap().as_mut().unwrap()).new_scanner(std::sync::Arc::new(std::sync::Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap()))))); *scanner.lock().unwrap() = Some(new_val); };
    let mut wordCount = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    let mut lineCount = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    let mut charCount = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*scanner.lock().unwrap().as_mut().unwrap()).scan() {
        let mut line = (*scanner.lock().unwrap().as_mut().unwrap()).text();
        { let mut guard = lineCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = charCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*line.lock().unwrap().as_mut().unwrap()).len()); };
        let mut words = (*strings.lock().unwrap().as_mut().unwrap()).fields(std::sync::Arc::new(std::sync::Mutex::new(Some((*line.lock().unwrap().as_mut().unwrap())))));
        { let mut guard = wordCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*words.lock().unwrap().as_mut().unwrap()).len()); };
        if (*strings.lock().unwrap().as_mut().unwrap()).contains(std::sync::Arc::new(std::sync::Mutex::new(Some((*line.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some("123".to_string())))) {
        print!("Found line with numbers: {}\n", (*line.lock().unwrap().as_mut().unwrap()));
    }
    }
    print!("Statistics:\n");
    print!("  Lines: {}\n", (*lineCount.lock().unwrap().as_mut().unwrap()));
    print!("  Words: {}\n", (*wordCount.lock().unwrap().as_mut().unwrap()));
    print!("  Characters: {}\n", (*charCount.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n--- Writing formatted data ---".to_string());
    let mut dataFile = std::sync::Arc::new(std::sync::Mutex::new(Some("data.txt".to_string())));
    (file, err) = (*os.lock().unwrap().as_mut().unwrap()).create(std::sync::Arc::new(std::sync::Mutex::new(Some((*dataFile.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error creating data file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    // defer (*file.lock().unwrap().as_mut().unwrap()).close() // TODO: defer not yet supported
    (*fmt.lock().unwrap().as_mut().unwrap()).fprintf(std::sync::Arc::new(std::sync::Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some("Name: %s\n".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("John Doe".to_string()))));
    (*fmt.lock().unwrap().as_mut().unwrap()).fprintf(std::sync::Arc::new(std::sync::Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some("Age: %d\n".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(30))));
    (*fmt.lock().unwrap().as_mut().unwrap()).fprintf(std::sync::Arc::new(std::sync::Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some("Score: %.2f\n".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(95.5))));
    (*fmt.lock().unwrap().as_mut().unwrap()).fprintf(std::sync::Arc::new(std::sync::Mutex::new(Some((*file.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some("Active: %t\n".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(true))));
    print!("Formatted data written to '{}'\n", (*dataFile.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n--- Reading formatted data ---".to_string());
    (data, err) = (*os.lock().unwrap().as_mut().unwrap()).read_file(std::sync::Arc::new(std::sync::Mutex::new(Some((*dataFile.lock().unwrap().as_mut().unwrap())))));
    if (*err.lock().unwrap()).is_some() {
        print!("Error reading data file: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    print!("Data file contents:\n{}", string(data.clone()));
    println!("{}", "\n--- Checking file existence ---".to_string());
    let mut files = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![(*filename.lock().unwrap().as_mut().unwrap()), (*copyFilename.lock().unwrap().as_mut().unwrap()), (*dataFile.lock().unwrap().as_mut().unwrap()), "nonexistent.txt".to_string()])));
    for f in &(*files.lock().unwrap().as_mut().unwrap()) {
        let (_, mut err) = (*os.lock().unwrap().as_mut().unwrap()).stat(std::sync::Arc::new(std::sync::Mutex::new(Some(f))));
    if (*err.lock().unwrap()).is_none() {
        print!("File '{}' exists\n", f);
    } else if (*os.lock().unwrap().as_mut().unwrap()).is_not_exist(std::sync::Arc::new(std::sync::Mutex::new(Some((*err.lock().unwrap().as_mut().unwrap()))))) {
        print!("File '{}' does not exist\n", f);
    } else {
        print!("Error checking file '{}': {}\n", f, (*err.lock().unwrap().as_mut().unwrap()));
    }
    }
    println!("{}", "\n--- Cleaning up ---".to_string());
    let mut filesToRemove = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![(*filename.lock().unwrap().as_mut().unwrap()), (*copyFilename.lock().unwrap().as_mut().unwrap()), (*dataFile.lock().unwrap().as_mut().unwrap())])));
    for f in &(*filesToRemove.lock().unwrap().as_mut().unwrap()) {
        let mut err = (*os.lock().unwrap().as_mut().unwrap()).remove(std::sync::Arc::new(std::sync::Mutex::new(Some(f))));
        if (*err.lock().unwrap()).is_some() {
        print!("Error removing file '{}': {}\n", f, (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        print!("Removed file '{}'\n", f);
    }
    }
    println!("{}", "\n--- Verifying cleanup ---".to_string());
    for f in &(*filesToRemove.lock().unwrap().as_mut().unwrap()) {
        let (_, mut err) = (*os.lock().unwrap().as_mut().unwrap()).stat(std::sync::Arc::new(std::sync::Mutex::new(Some(f))));
    if (*os.lock().unwrap().as_mut().unwrap()).is_not_exist(std::sync::Arc::new(std::sync::Mutex::new(Some((*err.lock().unwrap().as_mut().unwrap()))))) {
        print!("File '{}' successfully removed\n", f);
    } else {
        print!("File '{}' still exists\n", f);
    }
    }
    println!("{}", "\nFile operations test completed!".to_string());
}