package main

// Kept in an earlier package file than nestedError: package-wide error
// equality analysis must still give this field type Rust equality support.
type nestedCode struct {
	code string
}
