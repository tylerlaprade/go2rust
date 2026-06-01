package main

import (
	"go/ast"
	"go/types"
	"strconv"
	"strings"
	"unicode/utf8"
)

func isPredeclaredTypeName(name string) bool {
	switch name {
	case "any", "bool", "byte", "complex64", "complex128", "float32", "float64",
		"int", "int8", "int16", "int32", "int64",
		"rune", "string",
		"uint", "uint8", "uint16", "uint32", "uint64", "uintptr":
		return true
	default:
		return false
	}
}

// WrapInArcMutex wraps an expression in Arc<Mutex<Option<...>>>
func WrapInArcMutex(out *strings.Builder, expr ast.Expr) {
	TrackImport("Arc")
	TrackImport("Mutex")
	out.WriteString("Arc::new(")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("::new(Some(")
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
		return "Arc<" + GetInnerWrapperType()
	}
	return "Rc<RefCell"
}

// GetInnerWrapperType returns just the inner wrapper (Mutex vs RefCell)
func GetInnerWrapperType() string {
	if NeedsConcurrentWrapper() {
		if currentContext != nil && currentContext.Imports != nil && currentContext.Imports.IsReservedName("Mutex") {
			return "StdMutex"
		}
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

func trackWrapperImports() {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
	}
}

// WriteWrapperPrefix writes the beginning of a wrapper (e.g., "Arc::new(Mutex::new(Some(")
func WriteWrapperPrefix(out *strings.Builder) {
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new(Some(")
		// DEBUG
		// fmt.Fprintf(os.Stderr, "DEBUG: Using Arc<Mutex<>> wrapper\n")
	} else {
		out.WriteString("Rc::new(RefCell::new(Some(")
		// DEBUG
		// fmt.Fprintf(os.Stderr, "DEBUG: Using Rc<RefCell<>> wrapper\n")
	}
}

// WriteWrapperSuffix writes the end of a wrapper (")))")
func WriteWrapperSuffix(out *strings.Builder) {
	out.WriteString(")))")
}

// WriteWrapperOptionPrefix writes a wrapper around an existing Option<T>.
func WriteWrapperOptionPrefix(out *strings.Builder) {
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new(")
	} else {
		out.WriteString("Rc::new(RefCell::new(")
	}
}

func WriteWrapperOptionSuffix(out *strings.Builder) {
	out.WriteString("))")
}

// WriteWrappedNone writes None with the appropriate wrapper (no Option, so we can't use the Prefix/Suffix functions)
func WriteWrappedNone(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString("Arc::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new(None))")
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

	resultStr := string(result)
	if isRustPathKeyword(resultStr) {
		return resultStr + "_"
	}
	if isRustKeyword(resultStr) {
		return "r#" + resultStr
	}

	return resultStr
}

// SanitizeRustModuleName converts a string to a valid Rust module name
func SanitizeRustModuleName(s string) string {
	var result []byte
	for i, r := range s {
		// Replace hyphens and other invalid characters with underscores
		if r == '-' || r == '.' || r == '/' || r == '\\' {
			result = append(result, '_')
		} else if i > 0 && isUpper(r) {
			// Insert underscore before uppercase letters (camelCase -> snake_case)
			result = append(result, '_')
			result = append(result, toLower(r))
		} else {
			result = append(result, toLower(r))
		}
	}

	resultStr := string(result)
	if resultStr == "std" {
		return "std_"
	}
	if isRustPathKeyword(resultStr) {
		return resultStr + "_"
	}
	if isRustKeyword(resultStr) {
		return "r#" + resultStr
	}

	// Ensure the name starts with a letter or underscore
	if len(resultStr) > 0 && !((resultStr[0] >= 'a' && resultStr[0] <= 'z') || resultStr[0] == '_') {
		resultStr = "_" + resultStr
	}

	return resultStr
}

// SanitizeRustModuleFileName converts a module name to the Rust source file
// stem that rustc expects for `mod <name>;`.
func SanitizeRustModuleFileName(moduleName string) string {
	return strings.TrimPrefix(moduleName, "r#")
}

func RustCrateNameForGoImportPath(goPath string) string {
	return SanitizeRustCrateName(goPath)
}

func SanitizeRustCrateName(name string) string {
	var result []rune
	for _, r := range name {
		switch {
		case r >= 'a' && r <= 'z':
			result = append(result, r)
		case r >= 'A' && r <= 'Z':
			result = append(result, rune(toLower(r)))
		case r >= '0' && r <= '9':
			result = append(result, r)
		case r == '_':
			result = append(result, r)
		default:
			result = append(result, '_')
		}
	}
	crate := string(result)
	if len(crate) > 0 && crate[0] >= '0' && crate[0] <= '9' {
		crate = "pkg_" + crate
	}
	if isRustPathKeyword(crate) || isRustKeyword(crate) {
		crate += "_"
	}
	return crate
}

func EscapeRustIdent(s string) string {
	if isRustPathKeyword(s) {
		return s + "_"
	}
	if isRustKeyword(s) {
		return "r#" + s
	}
	return s
}

func RustTypeNameForUse(s string) string {
	switch s {
	case "Box", "Option", "Result", "String", "Vec":
		return s + "_"
	default:
		return EscapeRustIdent(s)
	}
}

func RustLocalIdent(s string) string {
	if isPackageGlobalName(s) || isPackageTupleStructTypeName(s) {
		return EscapeRustIdent(s + "_local")
	}
	return EscapeRustIdent(s)
}

// isPackageTupleStructTypeName reports whether s names a package-level type that
// lowers to a Rust tuple struct (named scalars, slices, maps, channels,
// pointers, functions, arrays — anything but a plain struct/interface). A local
// or parameter sharing that name shadows the tuple struct, which Rust rejects in
// pattern position (E0530: "function parameters cannot shadow tuple structs"),
// e.g. go/types' `func (obj) setColor(color color)` where `color` is `type color
// uint32`. Renaming the binding to <name>_local — the same scheme used for
// package-global collisions — sidesteps it; type references keep the bare name.
func isPackageTupleStructTypeName(s string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return false
	}
	tn, ok := typeInfo.pkg.Scope().Lookup(s).(*types.TypeName)
	if !ok {
		return false
	}
	named, ok := tn.Type().(*types.Named)
	if !ok {
		return false
	}
	switch named.Underlying().(type) {
	case *types.Struct, *types.Interface:
		return false
	default:
		return true
	}
}

func isPackageGlobalName(s string) bool {
	if packageGlobalNames[s] {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return false
	}
	_, ok := typeInfo.pkg.Scope().Lookup(s).(*types.Var)
	return ok
}

func RustIdentForUse(ident *ast.Ident) string {
	if isPackageGlobalIdent(ident) {
		return rustPackageGlobalName(ident.Name)
	}
	return RustLocalIdent(ident.Name)
}

func RustFunctionName(name string) string {
	rustName := ToSnakeCase(name)
	if rustName == "_" {
		return "__blank"
	}
	return rustName
}

func RustStringLiteral(goLiteral string) string {
	unquoted, err := strconv.Unquote(goLiteral)
	if err != nil {
		return goLiteral
	}
	if !utf8.ValidString(unquoted) {
		var bytes strings.Builder
		bytes.WriteByte('[')
		for i, b := range []byte(unquoted) {
			if i > 0 {
				bytes.WriteString(", ")
			}
			bytes.WriteString("0x")
			bytes.WriteString(strconv.FormatInt(int64(b), 16))
			bytes.WriteString("u8")
		}
		bytes.WriteByte(']')
		return "String::from_utf8_lossy(&" + bytes.String() + ").into_owned()"
	}

	var out strings.Builder
	out.WriteByte('"')
	for _, r := range unquoted {
		switch r {
		case '\\':
			out.WriteString("\\\\")
		case '"':
			out.WriteString("\\\"")
		case '\n':
			out.WriteString("\\n")
		case '\r':
			out.WriteString("\\r")
		case '\t':
			out.WriteString("\\t")
		default:
			if r < 0x20 || r == 0x7f || r > 0x7e {
				out.WriteString("\\u{")
				out.WriteString(strconv.FormatInt(int64(r), 16))
				out.WriteByte('}')
			} else {
				out.WriteRune(r)
			}
		}
	}
	out.WriteByte('"')
	return out.String()
}

// RustCharLiteral converts a Go character (rune) literal to a Rust char
// literal, translating Go-only escapes (\a \b \f \v, octal, \uXXXX) that Rust
// rejects into Rust-safe forms. Printable ASCII chars are emitted verbatim so
// the common case (e.g. 'A', '\n') produces no snapshot churn. Mirrors
// RustStringLiteral's escaping for a single rune.
func RustCharLiteral(goLiteral string) string {
	// Only rune/char literals (single-quoted) need translation. Guarding on the
	// leading quote makes this a safe no-op for INT/FLOAT/STRING/raw literals,
	// so it can be applied at any site that emits a possibly-char BasicLit.
	if len(goLiteral) < 2 || goLiteral[0] != '\'' {
		return goLiteral
	}
	unquoted, err := strconv.Unquote(goLiteral)
	if err != nil {
		return goLiteral
	}
	runes := []rune(unquoted)
	if len(runes) != 1 {
		return goLiteral
	}
	r := runes[0]
	var out strings.Builder
	out.WriteByte('\'')
	switch r {
	case '\\':
		out.WriteString("\\\\")
	case '\'':
		out.WriteString("\\'")
	case '\n':
		out.WriteString("\\n")
	case '\r':
		out.WriteString("\\r")
	case '\t':
		out.WriteString("\\t")
	default:
		if r < 0x20 || r == 0x7f || r > 0x7e {
			out.WriteString("\\u{")
			out.WriteString(strconv.FormatInt(int64(r), 16))
			out.WriteByte('}')
		} else {
			out.WriteRune(r)
		}
	}
	out.WriteByte('\'')
	return out.String()
}

func isRustPathKeyword(s string) bool {
	switch s {
	case "crate", "self", "super":
		return true
	default:
		return false
	}
}

func isRustKeyword(s string) bool {
	switch s {
	case "as", "async", "await", "break", "const", "continue", "dyn",
		"else", "enum", "extern", "fn", "for", "if", "impl", "in", "let",
		"loop", "match", "mod", "move", "mut", "pub", "ref", "return",
		"static", "struct", "trait", "type", "unsafe", "use", "where",
		"while", "abstract", "become", "box", "do", "final", "macro", "override",
		"priv", "try", "typeof", "unsized", "virtual", "yield":
		return true
	default:
		return false
	}
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
