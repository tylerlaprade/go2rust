package main

import (
	"go/ast"
	"strings"
)

// WrapInArcMutex wraps an expression in Arc<Mutex<Option<...>>>
func WrapInArcMutex(out *strings.Builder, expr ast.Expr) {
	TrackImport("Arc")
	TrackImport("Mutex")
	out.WriteString("Arc::new(Mutex::new(Some(")
	TranspileExpression(out, expr)
	WriteWrapperSuffix(out)
}

// UnwrapArcMutex generates code to access the value inside Arc<Mutex<Option<...>>>
func UnwrapArcMutex(out *strings.Builder, expr ast.Expr) {
	out.WriteString("(*")
	TranspileExpression(out, expr)
	out.WriteString(".lock().unwrap().as_mut().unwrap())")
}

// WrapInRcRefCell wraps an expression in Rc<RefCell<Option<...>>>
func WrapInRcRefCell(out *strings.Builder, expr ast.Expr) {
	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString("Rc::new(RefCell::new(Some(")
	TranspileExpression(out, expr)
	WriteWrapperSuffix(out)
}

// UnwrapRcRefCell generates code to access the value inside Rc<RefCell<Option<...>>>
func UnwrapRcRefCell(out *strings.Builder, expr ast.Expr) {
	out.WriteString("(*")
	TranspileExpression(out, expr)
	out.WriteString(".borrow_mut())")
}

// WrapValue wraps an expression in the appropriate wrapper type based on concurrency needs
func WrapValue(out *strings.Builder, expr ast.Expr) {
	if NeedsConcurrentWrapper() {
		WrapInArcMutex(out, expr)
	} else {
		WrapInRcRefCell(out, expr)
	}
}

// UnwrapValue generates code to access the value inside the appropriate wrapper type
func UnwrapValue(out *strings.Builder, expr ast.Expr) {
	if NeedsConcurrentWrapper() {
		UnwrapArcMutex(out, expr)
	} else {
		UnwrapRcRefCell(out, expr)
	}
}

// GetWrapperType returns the wrapper type string based on concurrency needs
func GetWrapperType() string {
	if NeedsConcurrentWrapper() {
		return "Arc<Mutex"
	}
	return "Rc<RefCell"
}

// GetInnerWrapperType returns just the inner wrapper (Mutex vs RefCell)
func GetInnerWrapperType() string {
	if NeedsConcurrentWrapper() {
		return "Mutex"
	}
	return "RefCell"
}

// GetOuterWrapperType returns just the outer wrapper (Arc vs Rc)
func GetOuterWrapperType() string {
	if NeedsConcurrentWrapper() {
		return "Arc"
	}
	return "Rc"
}

// WriteWrapperPrefix writes the beginning of a wrapper (e.g., "Arc::new(Mutex::new(Some(")
func WriteWrapperPrefix(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString("Arc::new(Mutex::new(Some(")
		// DEBUG
		// fmt.Fprintf(os.Stderr, "DEBUG: Using Arc<Mutex<>> wrapper\n")
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString("Rc::new(RefCell::new(Some(")
		// DEBUG
		// fmt.Fprintf(os.Stderr, "DEBUG: Using Rc<RefCell<>> wrapper\n")
	}
}

// WriteWrapperSuffix writes the end of a wrapper (")))")
func WriteWrapperSuffix(out *strings.Builder) {
	out.WriteString(")))")
}

// WriteWrappedNone writes None with the appropriate wrapper (no Option, so we can't use the Prefix/Suffix functions)
func WriteWrappedNone(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString("Arc::new(Mutex::new(None))")
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString("Rc::new(RefCell::new(None))")
	}
}

// WriteBorrowMethod writes the appropriate borrow method (lock().unwrap() vs borrow/borrow_mut)
func WriteBorrowMethod(out *strings.Builder, mutable bool) {
	if NeedsConcurrentWrapper() {
		out.WriteString(".lock().unwrap()")
	} else if mutable {
		out.WriteString(".borrow_mut()")
	} else {
		out.WriteString(".borrow()")
	}
}

func ToSnakeCase(s string) string {
	var result []byte
	for i, r := range s {
		if i > 0 && isUpper(r) {
			result = append(result, '_')
		}
		result = append(result, toLower(r))
	}

	// Escape Rust keywords
	resultStr := string(result)
	switch resultStr {
	case "type", "match", "move", "ref", "impl", "trait", "mod", "pub", "use", "where", "async", "await", "dyn":
		return "r#" + resultStr
	}

	return resultStr
}

func isUpper(r rune) bool {
	return r >= 'A' && r <= 'Z'
}

func toLower(r rune) byte {
	if r >= 'A' && r <= 'Z' {
		return byte(r + ('a' - 'A'))
	}
	return byte(r)
}
