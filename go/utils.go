package main

import (
	"go/ast"
	"go/types"
	"strconv"
	"strings"
	"unicode/utf8"
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
		out.WriteString("Arc::new(Mutex::new(Some(")
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
	crate := strings.ReplaceAll(goPath, "/", "_")
	crate = strings.ReplaceAll(crate, ".", "_")
	crate = strings.ReplaceAll(crate, "-", "_")
	if len(crate) > 0 && crate[0] >= '0' && crate[0] <= '9' {
		crate = "pkg_" + crate
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
	if isPackageGlobalName(s) {
		return EscapeRustIdent(s + "_local")
	}
	return EscapeRustIdent(s)
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
		return EscapeRustIdent(ident.Name)
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
