fn main() {
    println!("{}", "=== File Operations Test ===".to_string());
    let mut filename = "test_file.txt".to_string();
    println!("{}", "\n--- Writing to file ---".to_string());
    let mut file, let mut err = os.create(filename);
    
    let mut content = vec!["Hello, World!".to_string(), "This is line 2".to_string(), "Go file operations".to_string(), "Line 4 with numbers: 123".to_string(), "Final line".to_string()];
    for (i, line) in content.iter().enumerate() {
        let mut _, let mut err = file.write_string(line + "\n".to_string());
        
        print!("Wrote: {}\n", line);
    }
    file.close();
    print!("File '{}' created successfully\n", filename);
    println!("{}", "\n--- Reading entire file ---".to_string());
    let mut data, let mut err = os.read_file(filename);
    
    print!("File contents:\n{}", string(data));
    println!("{}", "\n--- Reading file line by line ---".to_string());
    file, err = os.open(filename);
    
    
    let mut scanner = bufio.new_scanner(file);
    let mut lineNum = 1;
    while scanner.scan() {
        let mut line = scanner.text();
        print!("Line {}: {}\n", lineNum, line);
        lineNum += 1;
    }
    
    println!("{}", "\n--- Appending to file ---".to_string());
    file, err = os.open_file(filename, os.o__a_p_p_e_n_d | os.o__w_r_o_n_l_y, 0644);
    
    let mut appendContent = vec!["Appended line 1".to_string(), "Appended line 2".to_string()];
    for (_, line) in appendContent.iter().enumerate() {
        let mut _, let mut err = file.write_string(line + "\n".to_string());
        
        print!("Appended: {}\n", line);
    }
    file.close();
    println!("{}", "\n--- Reading updated file ---".to_string());
    data, err = os.read_file(filename);
    
    print!("Updated file contents:\n{}", string(data));
    println!("{}", "\n--- File information ---".to_string());
    let mut fileInfo, let mut err = os.stat(filename);
    
    print!("File name: {}\n", fileInfo.name());
    print!("File size: {} bytes\n", fileInfo.size());
    print!("File mode: {}\n", fileInfo.mode());
    print!("Modified time: {}\n", fileInfo.mod_time());
    print!("Is directory: %t\n", fileInfo.is_dir());
    println!("{}", "\n--- Copying file ---".to_string());
    let mut copyFilename = "test_file_copy.txt".to_string();
    let mut sourceFile, let mut err = os.open(filename);
    
    
    let mut destFile, let mut err = os.create(copyFilename);
    
    
    let mut bytesWritten, let mut err = io.copy(destFile, sourceFile);
    
    print!("Copied {} bytes to '{}'\n", bytesWritten, copyFilename);
    println!("{}", "\n--- Processing file content ---".to_string());
    file, err = os.open(filename);
    
    
    scanner = bufio.new_scanner(file);
    let mut wordCount = 0;
    let mut lineCount = 0;
    let mut charCount = 0;
    while scanner.scan() {
        let mut line = scanner.text();
        lineCount += 1;
        charCount.push_str(&line.len());
        let mut words = strings.fields(line);
        wordCount.push_str(&words.len());
        
    }
    print!("Statistics:\n");
    print!("  Lines: {}\n", lineCount);
    print!("  Words: {}\n", wordCount);
    print!("  Characters: {}\n", charCount);
    println!("{}", "\n--- Writing formatted data ---".to_string());
    let mut dataFile = "data.txt".to_string();
    file, err = os.create(dataFile);
    
    
    fmt.fprintf(file, "Name: %s\n".to_string(), "John Doe".to_string());
    fmt.fprintf(file, "Age: %d\n".to_string(), 30);
    fmt.fprintf(file, "Score: %.2f\n".to_string(), 95.5);
    fmt.fprintf(file, "Active: %t\n".to_string(), true);
    print!("Formatted data written to '{}'\n", dataFile);
    println!("{}", "\n--- Reading formatted data ---".to_string());
    data, err = os.read_file(dataFile);
    
    print!("Data file contents:\n{}", string(data));
    println!("{}", "\n--- Checking file existence ---".to_string());
    let mut files = vec![filename, copyFilename, dataFile, "nonexistent.txt".to_string()];
    for (_, f) in files.iter().enumerate() {
        
    }
    println!("{}", "\n--- Cleaning up ---".to_string());
    let mut filesToRemove = vec![filename, copyFilename, dataFile];
    for (_, f) in filesToRemove.iter().enumerate() {
        let mut err = os.remove(f);
        
    }
    println!("{}", "\n--- Verifying cleanup ---".to_string());
    for (_, f) in filesToRemove.iter().enumerate() {
        
    }
    println!("{}", "\nFile operations test completed!".to_string());
}