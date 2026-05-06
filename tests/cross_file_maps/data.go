package main

// Global map variable - transpiler needs to know this is a map
var Users = map[string]int{
	"alice": 1,
	"bob":   2,
	"carol": 3,
}

// Global slice variable - transpiler needs to know this is a slice
var Numbers = []int{10, 20, 30, 40, 50}

// Map of slices - complex type that requires proper type resolution
var Groups = map[string][]string{
	"admins": {"alice", "bob"},
	"users":  {"carol", "dave", "eve"},
}

// Slice of maps - another complex type
var Records = []map[string]interface{}{
	{"name": "Alice", "age": 30},
	{"name": "Bob", "age": 25},
}
