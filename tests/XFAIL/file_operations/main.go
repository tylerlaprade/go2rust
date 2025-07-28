package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

func main() {
	// Create a temporary file for testing
	fmt.Println("=== File Operations Test ===")

	filename := "test_file.txt"

	// Write to file
	fmt.Println("\n--- Writing to file ---")

	file, err := os.Create(filename)
	if err != nil {
		fmt.Printf("Error creating file: %v\n", err)
		return
	}

	content := []string{
		"Hello, World!",
		"This is line 2",
		"Go file operations",
		"Line 4 with numbers: 123",
		"Final line",
	}

	for i, line := range content {
		_, err := file.WriteString(line + "\n")
		if err != nil {
			fmt.Printf("Error writing line %d: %v\n", i+1, err)
			file.Close()
			return
		}
		fmt.Printf("Wrote: %s\n", line)
	}

	file.Close()
	fmt.Printf("File '%s' created successfully\n", filename)

	// Read entire file
	fmt.Println("\n--- Reading entire file ---")

	data, err := os.ReadFile(filename)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}

	fmt.Printf("File contents:\n%s", string(data))

	// Read file line by line
	fmt.Println("\n--- Reading file line by line ---")

	file, err = os.Open(filename)
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	lineNum := 1

	for scanner.Scan() {
		line := scanner.Text()
		fmt.Printf("Line %d: %s\n", lineNum, line)
		lineNum++
	}

	if err := scanner.Err(); err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}

	// Append to file
	fmt.Println("\n--- Appending to file ---")

	file, err = os.OpenFile(filename, os.O_APPEND|os.O_WRONLY, 0644)
	if err != nil {
		fmt.Printf("Error opening file for append: %v\n", err)
		return
	}

	appendContent := []string{
		"Appended line 1",
		"Appended line 2",
	}

	for _, line := range appendContent {
		_, err := file.WriteString(line + "\n")
		if err != nil {
			fmt.Printf("Error appending: %v\n", err)
			file.Close()
			return
		}
		fmt.Printf("Appended: %s\n", line)
	}

	file.Close()

	// Read updated file
	fmt.Println("\n--- Reading updated file ---")

	data, err = os.ReadFile(filename)
	if err != nil {
		fmt.Printf("Error reading updated file: %v\n", err)
		return
	}

	fmt.Printf("Updated file contents:\n%s", string(data))

	// File information
	fmt.Println("\n--- File information ---")

	fileInfo, err := os.Stat(filename)
	if err != nil {
		fmt.Printf("Error getting file info: %v\n", err)
		return
	}

	fmt.Printf("File name: %s\n", fileInfo.Name())
	fmt.Printf("File size: %d bytes\n", fileInfo.Size())
	fmt.Printf("File mode: %v\n", fileInfo.Mode())
	fmt.Printf("Modified time: %v\n", fileInfo.ModTime())
	fmt.Printf("Is directory: %t\n", fileInfo.IsDir())

	// Copy file
	fmt.Println("\n--- Copying file ---")

	copyFilename := "test_file_copy.txt"

	sourceFile, err := os.Open(filename)
	if err != nil {
		fmt.Printf("Error opening source file: %v\n", err)
		return
	}
	defer sourceFile.Close()

	destFile, err := os.Create(copyFilename)
	if err != nil {
		fmt.Printf("Error creating destination file: %v\n", err)
		return
	}
	defer destFile.Close()

	bytesWritten, err := io.Copy(destFile, sourceFile)
	if err != nil {
		fmt.Printf("Error copying file: %v\n", err)
		return
	}

	fmt.Printf("Copied %d bytes to '%s'\n", bytesWritten, copyFilename)

	// Read and process file content
	fmt.Println("\n--- Processing file content ---")

	file, err = os.Open(filename)
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		return
	}
	defer file.Close()

	scanner = bufio.NewScanner(file)
	wordCount := 0
	lineCount := 0
	charCount := 0

	for scanner.Scan() {
		line := scanner.Text()
		lineCount++
		charCount += len(line)

		words := strings.Fields(line)
		wordCount += len(words)

		// Process lines containing numbers
		if strings.Contains(line, "123") {
			fmt.Printf("Found line with numbers: %s\n", line)
		}
	}

	fmt.Printf("Statistics:\n")
	fmt.Printf("  Lines: %d\n", lineCount)
	fmt.Printf("  Words: %d\n", wordCount)
	fmt.Printf("  Characters: %d\n", charCount)

	// Write formatted data
	fmt.Println("\n--- Writing formatted data ---")

	dataFile := "data.txt"
	file, err = os.Create(dataFile)
	if err != nil {
		fmt.Printf("Error creating data file: %v\n", err)
		return
	}
	defer file.Close()

	// Write structured data
	fmt.Fprintf(file, "Name: %s\n", "John Doe")
	fmt.Fprintf(file, "Age: %d\n", 30)
	fmt.Fprintf(file, "Score: %.2f\n", 95.5)
	fmt.Fprintf(file, "Active: %t\n", true)

	fmt.Printf("Formatted data written to '%s'\n", dataFile)

	// Read and parse formatted data
	fmt.Println("\n--- Reading formatted data ---")

	data, err = os.ReadFile(dataFile)
	if err != nil {
		fmt.Printf("Error reading data file: %v\n", err)
		return
	}

	fmt.Printf("Data file contents:\n%s", string(data))

	// Check if files exist
	fmt.Println("\n--- Checking file existence ---")

	files := []string{filename, copyFilename, dataFile, "nonexistent.txt"}

	for _, f := range files {
		if _, err := os.Stat(f); err == nil {
			fmt.Printf("File '%s' exists\n", f)
		} else if os.IsNotExist(err) {
			fmt.Printf("File '%s' does not exist\n", f)
		} else {
			fmt.Printf("Error checking file '%s': %v\n", f, err)
		}
	}

	// Clean up - remove test files
	fmt.Println("\n--- Cleaning up ---")

	filesToRemove := []string{filename, copyFilename, dataFile}

	for _, f := range filesToRemove {
		err := os.Remove(f)
		if err != nil {
			fmt.Printf("Error removing file '%s': %v\n", f, err)
		} else {
			fmt.Printf("Removed file '%s'\n", f)
		}
	}

	// Verify cleanup
	fmt.Println("\n--- Verifying cleanup ---")

	for _, f := range filesToRemove {
		if _, err := os.Stat(f); os.IsNotExist(err) {
			fmt.Printf("File '%s' successfully removed\n", f)
		} else {
			fmt.Printf("File '%s' still exists\n", f)
		}
	}

	fmt.Println("\nFile operations test completed!")
}
