package main

import (
	"fmt"
	"strings"
)

func main() {
	// Basic string operations
	fmt.Println("=== Basic string operations ===")

	str := "Hello, World!"
	fmt.Printf("Original string: %s\n", str)
	fmt.Printf("Length: %d\n", len(str))

	// String indexing and slicing
	fmt.Printf("First character: %c\n", str[0])
	fmt.Printf("Last character: %c\n", str[len(str)-1])
	fmt.Printf("Substring [0:5]: %s\n", str[0:5])
	fmt.Printf("Substring [7:]: %s\n", str[7:])

	// String concatenation
	fmt.Println("\n=== String concatenation ===")
	first := "Hello"
	second := "World"
	combined := first + ", " + second + "!"
	fmt.Printf("Concatenated: %s\n", combined)

	// Using strings package
	fmt.Println("\n=== strings package functions ===")

	text := "  Go is awesome for systems programming  "
	fmt.Printf("Original: '%s'\n", text)

	// Trimming
	trimmed := strings.TrimSpace(text)
	fmt.Printf("Trimmed: '%s'\n", trimmed)

	// Case conversion
	fmt.Printf("Upper: %s\n", strings.ToUpper(trimmed))
	fmt.Printf("Lower: %s\n", strings.ToLower(trimmed))
	fmt.Printf("Title: %s\n", strings.Title(trimmed))

	// String searching
	fmt.Println("\n=== String searching ===")
	searchText := "The quick brown fox jumps over the lazy dog"
	fmt.Printf("Text: %s\n", searchText)

	fmt.Printf("Contains 'fox': %t\n", strings.Contains(searchText, "fox"))
	fmt.Printf("Contains 'cat': %t\n", strings.Contains(searchText, "cat"))

	fmt.Printf("Index of 'fox': %d\n", strings.Index(searchText, "fox"))
	fmt.Printf("Index of 'cat': %d\n", strings.Index(searchText, "cat"))

	fmt.Printf("Last index of 'the': %d\n", strings.LastIndex(searchText, "the"))

	fmt.Printf("Count of 'the': %d\n", strings.Count(searchText, "the"))

	// String prefixes and suffixes
	fmt.Println("\n=== Prefixes and suffixes ===")
	filename := "document.txt"
	fmt.Printf("Filename: %s\n", filename)
	fmt.Printf("Has .txt suffix: %t\n", strings.HasSuffix(filename, ".txt"))
	fmt.Printf("Has .pdf suffix: %t\n", strings.HasSuffix(filename, ".pdf"))
	fmt.Printf("Has 'doc' prefix: %t\n", strings.HasPrefix(filename, "doc"))

	// String splitting and joining
	fmt.Println("\n=== Splitting and joining ===")
	csv := "apple,banana,cherry,date"
	fmt.Printf("CSV: %s\n", csv)

	fruits := strings.Split(csv, ",")
	fmt.Printf("Split result: %v\n", fruits)

	rejoined := strings.Join(fruits, " | ")
	fmt.Printf("Rejoined: %s\n", rejoined)

	// Fields (split on whitespace)
	sentence := "The quick brown fox"
	words := strings.Fields(sentence)
	fmt.Printf("Words: %v\n", words)

	// String replacement
	fmt.Println("\n=== String replacement ===")
	original := "I like cats and cats like me"
	fmt.Printf("Original: %s\n", original)

	replaced := strings.Replace(original, "cats", "dogs", 1)
	fmt.Printf("Replace first 'cats': %s\n", replaced)

	replacedAll := strings.ReplaceAll(original, "cats", "dogs")
	fmt.Printf("Replace all 'cats': %s\n", replacedAll)

	// String repetition
	fmt.Println("\n=== String repetition ===")
	pattern := "Go! "
	repeated := strings.Repeat(pattern, 3)
	fmt.Printf("Repeated: %s\n", repeated)

	// String comparison
	fmt.Println("\n=== String comparison ===")
	str1 := "apple"
	str2 := "banana"
	str3 := "apple"

	fmt.Printf("'%s' == '%s': %t\n", str1, str2, str1 == str2)
	fmt.Printf("'%s' == '%s': %t\n", str1, str3, str1 == str3)
	fmt.Printf("'%s' < '%s': %t\n", str1, str2, str1 < str2)

	// Case-insensitive comparison
	fmt.Printf("EqualFold('Apple', 'APPLE'): %t\n", strings.EqualFold("Apple", "APPLE"))

	// String building with strings.Builder
	fmt.Println("\n=== String building ===")
	var builder strings.Builder

	builder.WriteString("Building ")
	builder.WriteString("a ")
	builder.WriteString("string ")
	builder.WriteString("efficiently")

	built := builder.String()
	fmt.Printf("Built string: %s\n", built)
	fmt.Printf("Builder length: %d\n", builder.Len())

	// Rune iteration (Unicode support)
	fmt.Println("\n=== Unicode and runes ===")
	unicode := "Hello, 世界! 🌍"
	fmt.Printf("Unicode string: %s\n", unicode)
	fmt.Printf("Byte length: %d\n", len(unicode))

	runeCount := 0
	for _, r := range unicode {
		runeCount++
		fmt.Printf("Rune: %c (U+%04X)\n", r, r)
	}
	fmt.Printf("Rune count: %d\n", runeCount)

	// String trimming variations
	fmt.Println("\n=== String trimming variations ===")
	messy := "!!!Hello World!!!"
	fmt.Printf("Original: %s\n", messy)
	fmt.Printf("TrimLeft '!': %s\n", strings.TrimLeft(messy, "!"))
	fmt.Printf("TrimRight '!': %s\n", strings.TrimRight(messy, "!"))
	fmt.Printf("Trim '!': %s\n", strings.Trim(messy, "!"))

	// String formatting with different verbs
	fmt.Println("\n=== String formatting ===")
	name := "Alice"
	age := 30
	height := 5.6

	fmt.Printf("Name: %s, Age: %d, Height: %.1f\n", name, age, height)
	fmt.Printf("Quoted string: %q\n", name)
	fmt.Printf("String with width: '%10s'\n", name)
	fmt.Printf("Left-aligned: '%-10s'\n", name)
}
