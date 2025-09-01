package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

type StdlibHandler func(*strings.Builder, *ast.CallExpr)

func GetStdlibHandler(call *ast.CallExpr) StdlibHandler {
	// Handle selector expressions like fmt.Println
	if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		if ident, ok := sel.X.(*ast.Ident); ok {
			key := ident.Name + "." + sel.Sel.Name
			if handler, exists := stdlibMappings[key]; exists {
				return handler
			}
		}
	}

	// Handle builtin functions like println, append, len
	if ident, ok := call.Fun.(*ast.Ident); ok {
		if handler, exists := builtinMappings[ident.Name]; exists {
			return handler
		}
	}

	return nil
}

var stdlibMappings map[string]StdlibHandler
var builtinMappings map[string]StdlibHandler

func init() {
	stdlibMappings = map[string]StdlibHandler{
		"fmt.Println":       transpileFmtPrintln,
		"fmt.Printf":        transpileFmtPrintf,
		"fmt.Sprintf":       transpileFmtSprintf,
		"fmt.Errorf":        transpileFmtErrorf,
		"strings.ToLower":   transpileStringsToLower,
		"strings.ToUpper":   transpileStringsToUpper,
		"strings.TrimSpace": transpileStringsTrimSpace,
		"strconv.Itoa":      transpileStrconvItoa,
		"strconv.Atoi":      transpileStrconvAtoi,
		"errors.New":        transpileErrorsNew,
		"sort.Strings":      transpileSortStrings,
		"sort.Ints":         transpileSortInts,
		"slices.Sort":       transpileSlicesSort,
		"time.Sleep":        transpileTimeSleep,
	}

	builtinMappings = map[string]StdlibHandler{
		"println": transpileBuiltinPrintln,
		"append":  transpileAppend,
		"len":     transpileLen,
		"make":    transpileMake,
		"cap":     transpileCap,
		"delete":  transpileDelete,
		"new":     transpileNew,
		"complex": transpileComplex,
		"real":    transpileReal,
		"imag":    transpileImag,
		"panic":   transpilePanic,
		"recover": transpileRecover,
	}
}

func transpileFmtPrintln(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("println!")
	out.WriteString("(")

	if len(call.Args) > 0 {
		out.WriteString("\"")
		for i := range call.Args {
			if i > 0 {
				out.WriteString(" ")
			}
			out.WriteString("{}")
		}
		out.WriteString("\"")

		for _, arg := range call.Args {
			out.WriteString(", ")
			transpilePrintArg(out, arg)
		}
	}

	out.WriteString(")")
}

func transpileBuiltinPrintln(out *strings.Builder, call *ast.CallExpr) {
	// Go's builtin `println` write to stderr, not stdout
	out.WriteString("eprintln!")
	out.WriteString("(")

	if len(call.Args) > 0 {
		out.WriteString("\"")
		for i := range call.Args {
			if i > 0 {
				out.WriteString(" ")
			}
			out.WriteString("{}")
		}
		out.WriteString("\"")

		for _, arg := range call.Args {
			out.WriteString(", ")
			transpilePrintArg(out, arg)
		}
	}

	out.WriteString(")")
}

// Helper function to unwrap arguments for print statements
func transpilePrintArg(out *strings.Builder, arg ast.Expr) {
	// Type-based printing using TypeInfo
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.GetType(arg) != nil {
		argType := typeInfo.GetType(arg)

		// Check if it's any kind of interface
		if intf, ok := argType.Underlying().(*types.Interface); ok {
			// Special case for error type - use Display not Debug
			if named, ok := argType.(*types.Named); ok {
				if named.Obj().Name() == "error" && named.Obj().Pkg() == nil {
					// It's the builtin error type - use Display formatting
					if ident, ok := arg.(*ast.Ident); ok {
						out.WriteString("format!(\"{}\", (*")
						out.WriteString(ident.Name)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()))")
					} else {
						// Complex expression
						out.WriteString("format!(\"{}\", (*(")
						TranspileExpression(out, arg)
						out.WriteString(")")
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()))")
					}
					return
				}
			}

			if intf.NumMethods() == 0 {
				// It's interface{} - use format_any helper
				NeedFormatAny()
				out.WriteString("format_any(")
				if ident, ok := arg.(*ast.Ident); ok {
					out.WriteString(ident.Name)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap().as_ref()")
				} else {
					// Complex expression
					out.WriteString("(")
					TranspileExpression(out, arg)
					out.WriteString(")")
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap().as_ref()")
				}
				out.WriteString(")")
			} else {
				// It's a named interface - use Display formatting
				// This will use the Display impl of the concrete type
				out.WriteString("format!(\"{}\", ")
				if ident, ok := arg.(*ast.Ident); ok {
					out.WriteString("(*")
					out.WriteString(ident.Name)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())")
				} else {
					// Complex expression
					out.WriteString("(*(")
					TranspileExpression(out, arg)
					out.WriteString(")")
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())")
				}
				out.WriteString(")")
			}
			return
		}

		if typeInfo.IsMap(arg) {
			NeedFormatMap()
			TrackImport("Display")
			TrackImport("Ord")
			out.WriteString("format_map(&")
			if ident, ok := arg.(*ast.Ident); ok {
				// For identifiers, just use the name directly (it's already wrapped)
				out.WriteString(ident.Name)
			} else {
				TranspileExpression(out, arg)
			}
			out.WriteString(")")
			return
		} else if typeInfo.IsSlice(arg) {
			// Check if it's a slice of interface{}
			elemType := typeInfo.GetSliceElemType(arg)
			if elemType != nil {
				if intf, ok := elemType.Underlying().(*types.Interface); ok && intf.NumMethods() == 0 {
					// It's []interface{} - use format_any_slice
					NeedFormatAnySlice()
					TrackImport("Any")
					out.WriteString("format_any_slice(&")
					if ident, ok := arg.(*ast.Ident); ok {
						out.WriteString(ident.Name)
					} else {
						TranspileExpression(out, arg)
					}
					out.WriteString(")")
					return
				}
			}
			// Regular slice - use format_slice
			NeedFormatSlice()
			TrackImport("Display")
			out.WriteString("format_slice(&")
			if ident, ok := arg.(*ast.Ident); ok {
				// For identifiers, just use the name directly (it's already wrapped)
				out.WriteString(ident.Name)
			} else {
				TranspileExpression(out, arg)
			}
			out.WriteString(")")
			return
		}
		// Type is known but not a map or slice - fall through
	} else {
		// Type info not available - add error comment
		out.WriteString("/* ERROR: Type information not available for print argument */ ")
	}

	// Check if this is a field access on self (already wrapped)
	if sel, ok := arg.(*ast.SelectorExpr); ok {
		if ident, ok := sel.X.(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver {
			// self.field - need to unwrap for display
			out.WriteString("(*self.")
			out.WriteString(ToSnakeCase(sel.Sel.Name))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			return
		}
	}
	// Check if this is a function call that returns a wrapped value
	if callExpr, ok := arg.(*ast.CallExpr); ok {
		// Check if it's a method call or user function call
		needsUnwrap := false

		// Check for method call
		if _, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
			needsUnwrap = true
		} else if _, ok := callExpr.Fun.(*ast.Ident); ok && GetStdlibHandler(callExpr) == nil {
			// User function call (not stdlib)
			needsUnwrap = true
		}

		if needsUnwrap {
			// Method call or user function call - unwrap the result
			out.WriteString("(*")
			TranspileExpression(out, arg)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			return
		}
	}

	// For selector expressions (field access), TranspileExpression already handles unwrapping
	// in RValue context, so we don't need to add extra unwrapping here
	if _, ok := arg.(*ast.SelectorExpr); ok {
		TranspileExpression(out, arg)
		return
	}

	// For other cases, just use regular expression transpilation
	TranspileExpression(out, arg)
}

// convertFormatStringWithSkips converts Go format verbs to Rust format strings
// Returns the converted format string and indices of arguments to skip (for %T)
func convertFormatStringWithSkips(goFormat string) (string, []int) {
	var skipIndices []int
	format := goFormat

	// First, escape any literal curly braces that aren't part of Go format verbs
	// We need to do this before converting format verbs
	// Use placeholders to avoid double-escaping
	format = strings.ReplaceAll(format, "{", "__OPEN_BRACE__")
	format = strings.ReplaceAll(format, "}", "__CLOSE_BRACE__")

	// Track which arguments correspond to which verbs
	argIndex := 0
	result := strings.Builder{}
	i := 0
	for i < len(format) {
		if i < len(format)-1 && format[i] == '%' {
			if format[i+1] == '%' {
				// Literal percent
				result.WriteString("%")
				i += 2
			} else if format[i+1] == 'T' {
				// %T - skip this argument
				skipIndices = append(skipIndices, argIndex)
				result.WriteString("<type>")
				argIndex++
				i += 2
			} else if format[i+1] == '.' {
				// Handle precision format like %.10f, %.2d, etc.
				j := i + 2
				// Find the digits
				for j < len(format) && format[j] >= '0' && format[j] <= '9' {
					j++
				}
				if j < len(format) && (format[j] == 'f' || format[j] == 'd' || format[j] == 's') {
					precision := format[i+2 : j]
					result.WriteString("{:.")
					result.WriteString(precision)
					result.WriteString("}")
					argIndex++
					i = j + 1
				} else {
					// Unknown format, keep as-is
					result.WriteByte(format[i])
					i++
				}
			} else {
				// Handle single-char format verbs
				switch format[i+1] {
				case 'd', 's', 'v', 'f', 't', 'c':
					result.WriteString("{}")
					argIndex++
				case 'b':
					result.WriteString("{:b}")
					argIndex++
				case 'U':
					result.WriteString("{:?}")
					argIndex++
				default:
					// Unknown verb, keep as-is
					result.WriteByte(format[i])
					result.WriteByte(format[i+1])
				}
				i += 2
			}
		} else {
			result.WriteByte(format[i])
			i++
		}
	}

	// Now escape the literal braces that were in the original string
	finalFormat := result.String()
	finalFormat = strings.ReplaceAll(finalFormat, "__OPEN_BRACE__", "{{")
	finalFormat = strings.ReplaceAll(finalFormat, "__CLOSE_BRACE__", "}}")

	return finalFormat, skipIndices
}

// convertFormatString converts Go format strings to Rust format strings
func convertFormatString(goFormat string) string {
	converted, _ := convertFormatStringWithSkips(goFormat)
	return converted
}

func transpileFmtPrintf(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("print!")
	out.WriteString("(")

	var skipIndices []int
	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust and get skip indices
			format, skips := convertFormatStringWithSkips(lit.Value)
			skipIndices = skips
			out.WriteString(format)
		} else {
			TranspileExpression(out, call.Args[0])
		}

		// Rest of the arguments, skipping those for %T
		for i := 1; i < len(call.Args); i++ {
			// Check if this argument index should be skipped (0-based in skipIndices)
			shouldSkip := false
			for _, skipIdx := range skipIndices {
				if skipIdx == i-1 { // i-1 because format string is arg 0
					shouldSkip = true
					break
				}
			}
			if !shouldSkip {
				out.WriteString(", ")
				transpilePrintArg(out, call.Args[i])
			}
		}
	}

	out.WriteString(")")
}

func transpileFmtSprintf(out *strings.Builder, call *ast.CallExpr) {
	// fmt.Sprintf returns a string, which needs to be wrapped
	WriteWrapperPrefix(out)
	out.WriteString("format!")
	out.WriteString("(")

	var skipIndices []int
	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust and get skip indices
			format, skips := convertFormatStringWithSkips(lit.Value)
			skipIndices = skips
			out.WriteString(format)
		} else {
			TranspileExpression(out, call.Args[0])
		}

		// Rest of the arguments, skipping those for %T
		for i := 1; i < len(call.Args); i++ {
			// Check if this argument index should be skipped (0-based in skipIndices)
			shouldSkip := false
			for _, skipIdx := range skipIndices {
				if skipIdx == i-1 { // i-1 because format string is arg 0
					shouldSkip = true
					break
				}
			}
			if !shouldSkip {
				out.WriteString(", ")
				transpilePrintArg(out, call.Args[i])
			}
		}
	}

	out.WriteString("))))")
}

func transpileFmtErrorf(out *strings.Builder, call *ast.CallExpr) {
	WriteWrapperPrefix(out)
	out.WriteString("Box::new(format!")
	out.WriteString("(")

	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust
			format := convertFormatString(lit.Value)
			out.WriteString(format)
		} else {
			TranspileExpression(out, call.Args[0])
		}

		// Rest of the arguments
		for i := 1; i < len(call.Args); i++ {
			out.WriteString(", ")
			TranspileExpression(out, call.Args[i])
		}
	}

	out.WriteString(")) as Box<dyn Error + Send + Sync>)))")
}

func transpileErrorsNew(out *strings.Builder, call *ast.CallExpr) {
	WriteWrapperPrefix(out)
	out.WriteString("Box::<dyn std::error::Error + Send + Sync>::from(")

	if len(call.Args) > 0 {
		// The argument is the error message
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// String literal - already has quotes
			out.WriteString(lit.Value)
			out.WriteString(".to_string()")
		} else {
			// Expression - might already be a string
			TranspileExpression(out, call.Args[0])
		}
	}

	out.WriteString("))))")
}

func transpileStringsToUpper(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".to_uppercase()")
	}
}

func transpileStringsToLower(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".to_lowercase()")
	}
}

func transpileStringsTrimSpace(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".trim()")
	}
}

func transpileSortStrings(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		// sort.Strings sorts a slice of strings in-place
		// We need to get mutable access to the vector inside the Arc<Mutex<Option<Vec<String>>>>
		out.WriteString("(*")
		if ident, ok := call.Args[0].(*ast.Ident); ok {
			out.WriteString(ident.Name)
		} else {
			TranspileExpression(out, call.Args[0])
		}
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).sort()")
	}
}

func transpileSortInts(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		// sort.Ints sorts a slice of integers in-place
		// We need to get mutable access to the vector inside the Arc<Mutex<Option<Vec<i32>>>>
		out.WriteString("(*")
		if ident, ok := call.Args[0].(*ast.Ident); ok {
			out.WriteString(ident.Name)
		} else {
			TranspileExpression(out, call.Args[0])
		}
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).sort()")
	}
}

func transpileSlicesSort(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		// slices.Sort is a generic sort function that works with any ordered type
		// We need to get mutable access to the vector inside the Arc<Mutex<Option<Vec<T>>>>
		out.WriteString("(*")
		if ident, ok := call.Args[0].(*ast.Ident); ok {
			out.WriteString(ident.Name)
		} else {
			TranspileExpression(out, call.Args[0])
		}
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).sort()")
	}
}

func transpileStrconvItoa(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".to_string()")
	}
}

func transpileStrconvAtoi(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		out.WriteString("match ")
		TranspileExpression(out, call.Args[0])
		out.WriteString(".parse::<i32>() { ")
		out.WriteString("Ok(n) => (")
		WriteWrapperPrefix(out)
		out.WriteString("n))), ")
		WriteWrappedNone(out)
		out.WriteString("), ")
		TrackImport("Error")
		out.WriteString("Err(e) => (")
		WriteWrapperPrefix(out)
		out.WriteString("0))), ")
		WriteWrapperPrefix(out)
		out.WriteString("Box::new(e) as Box<dyn Error + Send + Sync>)))) }")
	}
}

func transpileAppend(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 2 {
		// append() in Go returns the slice, but our slices are wrapped
		// We need to push to the inner vector and return the wrapped slice
		if len(call.Args) == 2 {
			// Single element append
			out.WriteString("{(*")
			if ident, ok := call.Args[0].(*ast.Ident); ok {
				out.WriteString(ident.Name)
			} else {
				TranspileExpression(out, call.Args[0])
			}
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap()).push(")
			TranspileExpression(out, call.Args[1])
			out.WriteString("); ")
			// Return the wrapped slice itself
			if ident, ok := call.Args[0].(*ast.Ident); ok {
				out.WriteString(ident.Name)
			} else {
				TranspileExpression(out, call.Args[0])
			}
			out.WriteString(".clone()}")
		} else {
			// Multiple elements, use extend
			out.WriteString("{(*")
			if ident, ok := call.Args[0].(*ast.Ident); ok {
				out.WriteString(ident.Name)
			} else {
				TranspileExpression(out, call.Args[0])
			}
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap()).extend(vec![")
			for i := 1; i < len(call.Args); i++ {
				if i > 1 {
					out.WriteString(", ")
				}
				TranspileExpression(out, call.Args[i])
			}
			out.WriteString("]); ")
			// Return the wrapped slice itself
			if ident, ok := call.Args[0].(*ast.Ident); ok {
				out.WriteString(ident.Name)
			} else {
				TranspileExpression(out, call.Args[0])
			}
			out.WriteString(".clone()}")
		}
	}
}

func transpileLen(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		// len() returns the length of arrays, slices, maps, strings, or channels
		// The argument is wrapped, so we need to unwrap it first
		// Keep as usize - Rust's natural size type for collections
		out.WriteString("(*")
		// Use LValue context so identifiers don't unwrap themselves
		TranspileExpressionContext(out, call.Args[0], LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).len()")
	}
}

func transpileMake(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 1 {
		// Check if it's a map type
		if mapType, ok := call.Args[0].(*ast.MapType); ok {
			WriteWrapperPrefix(out)
			TrackImport("HashMap")
			out.WriteString("HashMap::<")
			out.WriteString(goTypeToRustBase(mapType.Key))
			out.WriteString(", ")
			out.WriteString(GoTypeToRust(mapType.Value))
			out.WriteString(">::new()")
			out.WriteString(")))")
		} else if arrayType, ok := call.Args[0].(*ast.ArrayType); ok && arrayType.Len == nil {
			// Slice type - check element type
			elementType := "0" // default
			if ident, ok := arrayType.Elt.(*ast.Ident); ok {
				switch ident.Name {
				case "string":
					elementType = `"".to_string()`
				case "int", "int32", "int64":
					elementType = "0"
				case "float32", "float64":
					elementType = "0.0"
				case "bool":
					elementType = "false"
				}
			}

			WriteWrapperPrefix(out)
			if len(call.Args) >= 2 {
				// Check if size is 0
				if lit, ok := call.Args[1].(*ast.BasicLit); ok && lit.Value == "0" {
					// Empty vector with capacity
					out.WriteString("Vec::with_capacity(")
					if len(call.Args) >= 3 {
						TranspileExpression(out, call.Args[2])
					} else {
						out.WriteString("0")
					}
					out.WriteString(")")
				} else {
					// Vector with initial size
					out.WriteString("vec![")
					out.WriteString(elementType)
					out.WriteString("; ")
					TranspileExpression(out, call.Args[1])
					out.WriteString("]")
				}
			} else {
				out.WriteString("Vec::new()")
			}
			out.WriteString(")))")
		}
	}
}

func transpileCap(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		TranspileExpression(out, call.Args[0])
		out.WriteString(".capacity()")
	}
}

func transpileDelete(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 2 {
		out.WriteString("(*")
		// For delete, we need the raw identifier, not the unwrapped value
		if ident, ok := call.Args[0].(*ast.Ident); ok {
			out.WriteString(ident.Name)
		} else {
			// For complex expressions, we'd need to handle differently
			// For now, just use the expression as-is
			TranspileExpression(out, call.Args[0])
		}
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).remove(&")
		TranspileExpression(out, call.Args[1])
		out.WriteString(")")
	}
}

func transpileNew(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		WriteWrapperPrefix(out)
		out.WriteString(GoTypeToRust(call.Args[0]))
		out.WriteString("::default())))")
	}
}

// transpileComplex handles the complex() builtin function
func transpileComplex(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 2 {
		return
	}

	// TODO: Determine the type - complex64 or complex128
	// For now, default to complex128 (f64)
	WriteWrapperPrefix(out)
	out.WriteString("num::Complex::new(")
	out.WriteString("*")
	TranspileExpression(out, call.Args[0])
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap(), ")
	out.WriteString("*")
	TranspileExpression(out, call.Args[1])
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()))))")
}

// transpileReal handles the real() builtin function
func transpileReal(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 1 {
		return
	}

	WriteWrapperPrefix(out)
	out.WriteString("(*")
	TranspileExpression(out, call.Args[0])
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).re)))")
}

// transpileImag handles the imag() builtin function
func transpileImag(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 1 {
		return
	}

	WriteWrapperPrefix(out)
	out.WriteString("(*")
	TranspileExpression(out, call.Args[0])
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).im)))")
}

// Helper function to format maps like Go does
func generateMapFormatter(out *strings.Builder) {
	TrackImport("Display")
	TrackImport("Ord")

	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<HashMap<K, Arc<Mutex<Option<V>>>>>>>) -> String 
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());
        
        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();
        
        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString(`fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<HashMap<K, Rc<RefCell<Option<V>>>>>>>) -> String 
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());
        
        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.borrow();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();
        
        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
`)
	}
}

// Helper function to format slices like Go does
func generateSliceFormatter(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString(`fn format_slice<T>(slice: &Rc<RefCell<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}
`)
	}
}

// transpilePanic handles the panic() builtin function
func transpilePanic(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("panic!(")
	if len(call.Args) > 0 {
		// Check if the argument is a string literal
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// String literal - use it directly
			out.WriteString(lit.Value)
		} else if callExpr, ok := call.Args[0].(*ast.CallExpr); ok {
			// Check if it's fmt.Errorf - handle specially
			if sel, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
				if pkg, ok := sel.X.(*ast.Ident); ok && pkg.Name == "fmt" && sel.Sel.Name == "Errorf" {
					// panic(fmt.Errorf(...)) - extract the format string directly
					if len(callExpr.Args) > 0 {
						if lit, ok := callExpr.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
							// Convert format string
							format := convertFormatString(lit.Value)
							out.WriteString(format)
							// Add the rest of the arguments
							for i := 1; i < len(callExpr.Args); i++ {
								out.WriteString(", ")
								transpilePrintArg(out, callExpr.Args[i])
							}
						} else {
							// Non-literal format string
							out.WriteString("\"{}\", ")
							TranspileExpression(out, call.Args[0])
						}
					}
				} else {
					// Other function call - format it
					out.WriteString("\"{:?}\", ")
					TranspileExpression(out, call.Args[0])
				}
			} else {
				// Other call expression - format it
				out.WriteString("\"{:?}\", ")
				TranspileExpression(out, call.Args[0])
			}
		} else {
			// Other expression - format it
			out.WriteString("\"{:?}\", ")
			TranspileExpression(out, call.Args[0])
		}
	} else {
		out.WriteString("\"explicit panic\"")
	}
	out.WriteString(")")
}

// transpileRecover handles the recover() builtin function
func transpileRecover(out *strings.Builder, call *ast.CallExpr) {
	// In Rust, we can use std::panic::catch_unwind for similar functionality
	// For now, we'll generate a placeholder that returns None
	// A proper implementation would need to track defer context and use catch_unwind
	// This is a simplified version that always returns None
	// Don't use WriteWrapperPrefix as it adds Some() which we don't want for None
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString("Arc::new(Mutex::new(None::<String>))")
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString("Rc::new(RefCell::new(None::<String>))")
	}
}

// transpileTimeSleep handles the time.Sleep function
func transpileTimeSleep(out *strings.Builder, call *ast.CallExpr) {
	// Track that we need time and thread imports
	TrackImport("thread")
	TrackImport("time::Duration")

	if len(call.Args) > 0 {
		// time.Sleep takes a Duration in nanoseconds in Go
		// We need to convert to milliseconds for Rust's Duration::from_millis
		// Handle different cases of duration arguments

		// Check if it's a simple multiplication like 500 * time.Millisecond
		if binOp, ok := call.Args[0].(*ast.BinaryExpr); ok && binOp.Op == token.MUL {
			// Check if one side is time.Millisecond, time.Second, etc.
			var multiplier ast.Expr
			var unit string

			if sel, ok := binOp.Y.(*ast.SelectorExpr); ok {
				if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "time" {
					unit = sel.Sel.Name
					multiplier = binOp.X
				}
			} else if sel, ok := binOp.X.(*ast.SelectorExpr); ok {
				if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "time" {
					unit = sel.Sel.Name
					multiplier = binOp.Y
				}
			}

			if unit != "" {
				out.WriteString("std::thread::sleep(std::time::Duration::")
				switch unit {
				case "Millisecond":
					out.WriteString("from_millis(")
					TranspileExpression(out, multiplier)
				case "Second":
					out.WriteString("from_secs(")
					TranspileExpression(out, multiplier)
				case "Microsecond":
					out.WriteString("from_micros(")
					TranspileExpression(out, multiplier)
				case "Nanosecond":
					out.WriteString("from_nanos(")
					TranspileExpression(out, multiplier)
				case "Minute":
					out.WriteString("from_secs(60 * ")
					TranspileExpression(out, multiplier)
				case "Hour":
					out.WriteString("from_secs(3600 * ")
					TranspileExpression(out, multiplier)
				default:
					// Unknown unit, default to milliseconds
					out.WriteString("from_millis(")
					TranspileExpression(out, multiplier)
				}

				if unit == "Minute" || unit == "Hour" {
					out.WriteString(")")
				}
				out.WriteString("))")
				return
			}
		}

		// Fallback: assume it's a duration in nanoseconds
		out.WriteString("std::thread::sleep(std::time::Duration::from_nanos(")
		TranspileExpression(out, call.Args[0])
		out.WriteString(" as u64))")
	}
}
