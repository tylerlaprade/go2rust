fn main() {
    println!("{}", "=== File Operations Test ===".to_string());
    let mut filename = "test_file.txt".to_string();
    println!("{}", "\n--- Writing to file ---".to_string());
    let (mut file, mut err) = os.create(filename);
    if err.is_some() {
        print!("Error creating file: {}\n", err);
        return;
    }
    let mut content = vec!["Hello, World!".to_string(), "This is line 2".to_string(), "Go file operations".to_string(), "Line 4 with numbers: 123".to_string(), "Final line".to_string()];
    for (i, line) in content.iter().enumerate() {
        let (_, mut err) = file.write_string(line + "\n".to_string());
        if err.is_some() {
        print!("Error writing line {}: {}\n", i + 1, err);
        file.close();
        return;
    }
        print!("Wrote: {}\n", line);
    }
    file.close();
    print!("File '{}' created successfully\n", filename);
    println!("{}", "\n--- Reading entire file ---".to_string());
    let (mut data, mut err) = os.read_file(filename);
    if err.is_some() {
        print!("Error reading file: {}\n", err);
        return;
    }
    print!("File contents:\n{}", string(data));
    println!("{}", "\n--- Reading file line by line ---".to_string());
    (file, err) = os.open(filename);
    if err.is_some() {
        print!("Error opening file: {}\n", err);
        return;
    }
    
    let mut scanner = bufio.new_scanner(file);
    let mut lineNum = 1;
    while scanner.scan() {
        let mut line = scanner.text();
        print!("Line {}: {}\n", lineNum, line);
        lineNum += 1;
    }
    let mut err = scanner.err();
    if err.is_some() {
        print!("Error reading file: {}\n", err);
        return;
    }
    println!("{}", "\n--- Appending to file ---".to_string());
    (file, err) = os.open_file(filename, os.o__a_p_p_e_n_d | os.o__w_r_o_n_l_y, 0644);
    if err.is_some() {
        print!("Error opening file for append: {}\n", err);
        return;
    }
    let mut appendContent = vec!["Appended line 1".to_string(), "Appended line 2".to_string()];
    for (_, line) in appendContent.iter().enumerate() {
        let (_, mut err) = file.write_string(line + "\n".to_string());
        if err.is_some() {
        print!("Error appending: {}\n", err);
        file.close();
        return;
    }
        print!("Appended: {}\n", line);
    }
    file.close();
    println!("{}", "\n--- Reading updated file ---".to_string());
    (data, err) = os.read_file(filename);
    if err.is_some() {
        print!("Error reading updated file: {}\n", err);
        return;
    }
    print!("Updated file contents:\n{}", string(data));
    println!("{}", "\n--- File information ---".to_string());
    let (mut fileInfo, mut err) = os.stat(filename);
    if err.is_some() {
        print!("Error getting file info: {}\n", err);
        return;
    }
    print!("File name: {}\n", fileInfo.name());
    print!("File size: {} bytes\n", fileInfo.size());
    print!("File mode: {}\n", fileInfo.mode());
    print!("Modified time: {}\n", fileInfo.mod_time());
    print!("Is directory: {}\n", fileInfo.is_dir());
    println!("{}", "\n--- Copying file ---".to_string());
    let mut copyFilename = "test_file_copy.txt".to_string();
    let (mut sourceFile, mut err) = os.open(filename);
    if err.is_some() {
        print!("Error opening source file: {}\n", err);
        return;
    }
    
    let (mut destFile, mut err) = os.create(copyFilename);
    if err.is_some() {
        print!("Error creating destination file: {}\n", err);
        return;
    }
    
    let (mut bytesWritten, mut err) = io.copy(destFile, sourceFile);
    if err.is_some() {
        print!("Error copying file: {}\n", err);
        return;
    }
    print!("Copied {} bytes to '{}'\n", bytesWritten, copyFilename);
    println!("{}", "\n--- Processing file content ---".to_string());
    (file, err) = os.open(filename);
    if err.is_some() {
        print!("Error opening file: {}\n", err);
        return;
    }
    
    scanner = bufio.new_scanner(file);
    let mut wordCount = 0;
    let mut lineCount = 0;
    let mut charCount = 0;
    while scanner.scan() {
        let mut line = scanner.text();
        lineCount += 1;
        charCount += line.len();
        let mut words = strings.fields(line);
        wordCount += words.len();
        if strings.contains(line, "123".to_string()) {
        print!("Found line with numbers: {}\n", line);
    }
    }
    print!("Statistics:\n");
    print!("  Lines: {}\n", lineCount);
    print!("  Words: {}\n", wordCount);
    print!("  Characters: {}\n", charCount);
    println!("{}", "\n--- Writing formatted data ---".to_string());
    let mut dataFile = "data.txt".to_string();
    (file, err) = os.create(dataFile);
    if err.is_some() {
        print!("Error creating data file: {}\n", err);
        return;
    }
    
    fmt.fprintf(file, "Name: %s\n".to_string(), "John Doe".to_string());
    fmt.fprintf(file, "Age: %d\n".to_string(), 30);
    fmt.fprintf(file, "Score: %.2f\n".to_string(), 95.5);
    fmt.fprintf(file, "Active: %t\n".to_string(), true);
    print!("Formatted data written to '{}'\n", dataFile);
    println!("{}", "\n--- Reading formatted data ---".to_string());
    (data, err) = os.read_file(dataFile);
    if err.is_some() {
        print!("Error reading data file: {}\n", err);
        return;
    }
    print!("Data file contents:\n{}", string(data));
    println!("{}", "\n--- Checking file existence ---".to_string());
    let mut files = vec![filename, copyFilename, dataFile, "nonexistent.txt".to_string()];
    for (_, f) in files.iter().enumerate() {
        let (_, mut err) = os.stat(f);
    if err.is_none() {
        print!("File '{}' exists\n", f);
    } else if os.is_not_exist(err) {
        print!("File '{}' does not exist\n", f);
    } else {
        print!("Error checking file '{}': {}\n", f, err);
    }
    }
    println!("{}", "\n--- Cleaning up ---".to_string());
    let mut filesToRemove = vec![filename, copyFilename, dataFile];
    for (_, f) in filesToRemove.iter().enumerate() {
        let mut err = os.remove(f);
        if err.is_some() {
        print!("Error removing file '{}': {}\n", f, err);
    } else {
        print!("Removed file '{}'\n", f);
    }
    }
    println!("{}", "\n--- Verifying cleanup ---".to_string());
    for (_, f) in filesToRemove.iter().enumerate() {
        let (_, mut err) = os.stat(f);
    if os.is_not_exist(err) {
        print!("File '{}' successfully removed\n", f);
    } else {
        print!("File '{}' still exists\n", f);
    }
    }
    println!("{}", "\nFile operations test completed!".to_string());
}