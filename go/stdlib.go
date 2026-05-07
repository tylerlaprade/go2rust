package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"reflect"
	"strconv"
	"strings"
)

type StdlibHandler func(*strings.Builder, *ast.CallExpr)

func GetStdlibHandler(call *ast.CallExpr) StdlibHandler {
	// Handle selector expressions like fmt.Println
	if key, ok := stdlibCallKey(call.Fun); ok {
		if handler, exists := stdlibMappings[key]; exists {
			return handler
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

func stdlibCallKey(expr ast.Expr) (string, bool) {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return "", false
	}

	parts := []string{sel.Sel.Name}
	current := sel.X
	for {
		switch x := current.(type) {
		case *ast.SelectorExpr:
			parts = append([]string{x.Sel.Name}, parts...)
			current = x.X
		case *ast.Ident:
			return resolveStdlibPackageName(x.Name) + "." + strings.Join(parts, "."), true
		default:
			return "", false
		}
	}
}

var stdlibMappings map[string]StdlibHandler
var builtinMappings map[string]StdlibHandler

func init() {
	stdlibMappings = map[string]StdlibHandler{
		"fmt.Println":             transpileFmtPrintln,
		"fmt.Printf":              transpileFmtPrintf,
		"fmt.Print":               transpileFmtPrint,
		"fmt.Sprintf":             transpileFmtSprintf,
		"fmt.Fprintln":            transpileFmtFprintln,
		"fmt.Fprintf":             transpileFmtFprintf,
		"fmt.Errorf":              transpileFmtErrorf,
		"strings.ToLower":         transpileStringsToLower,
		"strings.ToUpper":         transpileStringsToUpper,
		"strings.TrimSpace":       transpileStringsTrimSpace,
		"strings.Title":           transpileStringsTitle,
		"strings.Contains":        transpileStringsContains,
		"strings.Index":           transpileStringsIndex,
		"strings.LastIndex":       transpileStringsLastIndex,
		"strings.Count":           transpileStringsCount,
		"strings.Compare":         transpileStringsCompare,
		"strings.Cut":             transpileStringsCut,
		"strings.HasSuffix":       transpileStringsHasSuffix,
		"strings.HasPrefix":       transpileStringsHasPrefix,
		"strings.IndexAny":        transpileStringsIndexAny,
		"strings.Split":           transpileStringsSplit,
		"strings.Join":            transpileStringsJoin,
		"strings.Fields":          transpileStringsFields,
		"strings.Replace":         transpileStringsReplace,
		"strings.ReplaceAll":      transpileStringsReplaceAll,
		"strings.Repeat":          transpileStringsRepeat,
		"strings.EqualFold":       transpileStringsEqualFold,
		"strings.TrimLeft":        transpileStringsTrimLeft,
		"strings.TrimRight":       transpileStringsTrimRight,
		"strings.Trim":            transpileStringsTrim,
		"strconv.Itoa":            transpileStrconvItoa,
		"strconv.Atoi":            transpileStrconvAtoi,
		"strconv.FormatFloat":     transpileStrconvFormatFloat,
		"strconv.FormatInt":       transpileStrconvFormatInt,
		"errors.New":              transpileErrorsNew,
		"sort.Strings":            transpileSortStrings,
		"sort.Ints":               transpileSortInts,
		"slices.Sort":             transpileSlicesSort,
		"slices.SortFunc":         transpileSlicesSortFunc,
		"slices.Contains":         transpileSlicesContains,
		"time.Sleep":              transpileTimeSleep,
		"time.Now":                transpileTimeNow,
		"time.Unix":               transpileTimeUnix,
		"time.After":              transpileTimeAfter,
		"time.NewTicker":          transpileTimeNewTicker,
		"time.NewTimer":           transpileTimeNewTimer,
		"time.Tick":               transpileTimeTick,
		"context.Background":      transpileContextBackground,
		"context.WithTimeout":     transpileContextWithTimeout,
		"context.WithCancel":      transpileContextWithCancel,
		"context.WithCancelCause": transpileContextWithCancelCause,
		"flag.String":             transpileFlagString,
		"flag.Parse":              transpileFlagParse,
		"os.Create":               transpileOsCreate,
		"os.Remove":               transpileOsRemove,
		"reflect.TypeOf":          transpileReflectTypeOf,
		"sync/atomic.AddInt64":    transpileAtomicAddInt64,
		"sync/atomic.LoadInt64":   transpileAtomicLoadInt64,
		"encoding/base64.StdEncoding.EncodeToString": transpileBase64EncodeToString,
		"encoding/base64.StdEncoding.DecodeString":   transpileBase64DecodeString,
		"crypto/sha256.Sum256":                       transpileSha256Sum256,
		"encoding/json.Marshal":                      transpileJsonMarshal,
		"math.Sqrt":                                  transpileMathSqrt,
		"math.Pow":                                   transpileMathPow,
		"math.Max":                                   transpileMathMax,
		"math.Min":                                   transpileMathMin,
		"unsafe.Sizeof":                              transpileUnsafeSizeof,
		"unsafe.Alignof":                             transpileUnsafeAlignof,
		"unsafe.Offsetof":                            transpileUnsafeOffsetof,
		"math/rand.Seed":                             transpileRandSeed,
		"math/rand.Intn":                             transpileRandIntn,
		"math/rand.Float64":                          transpileRandFloat64,
		"net/url.Parse":                              transpileUrlParse,
		"regexp.MustCompile":                         transpileRegexpMustCompile,
	}

	builtinMappings = map[string]StdlibHandler{
		"println": transpileBuiltinPrintln,
		"append":  transpileAppend,
		"len":     transpileLen,
		"make":    transpileMake,
		"cap":     transpileCap,
		"copy":    transpileCopy,
		"delete":  transpileDelete,
		"new":     transpileNew,
		"complex": transpileComplex,
		"real":    transpileReal,
		"imag":    transpileImag,
		"panic":   transpilePanic,
		"recover": transpileRecover,
		"close":   transpileClose,
	}
}

func transpileFmtPrint(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("print!")
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
						out.WriteString(RustIdentForUse(ident))
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
					out.WriteString(RustIdentForUse(ident))
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap().as_ref()")
				} else if _, ok := arg.(*ast.SelectorExpr); ok {
					TranspileExpressionContext(out, arg, LValue)
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
				// Check if it's a bare interface param (&dyn Trait) - use directly
				if ident, ok := arg.(*ast.Ident); ok && isVarBare(ident.Name) {
					// Interface parameter is already &dyn Trait with Display supertrait
					out.WriteString(RustIdentForUse(ident))
				} else {
					// This will use the Display impl of the concrete type
					out.WriteString("format!(\"{}\", ")
					if ident, ok := arg.(*ast.Ident); ok {
						out.WriteString("(*")
						out.WriteString(RustIdentForUse(ident))
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
				out.WriteString(RustIdentForUse(ident))
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
						out.WriteString(RustIdentForUse(ident))
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
			if ident, ok := arg.(*ast.Ident); ok {
				if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
					out.WriteString("format_slice_values(")
					out.WriteString(RustIdentForUse(ident))
					out.WriteString(")")
					return
				}
				out.WriteString("format_slice(&")
				out.WriteString(RustIdentForUse(ident))
				out.WriteString(")")
			} else {
				out.WriteString("format_slice(&")
				TranspileExpression(out, arg)
				out.WriteString(")")
			}
			return
		}
		// Check if it's a pointer to a struct - Go prints "&{...}" for these
		if ptr, ok := argType.(*types.Pointer); ok {
			if _, ok := ptr.Elem().Underlying().(*types.Struct); ok {
				out.WriteString("format!(\"&{}\", (*")
				if ident, ok := arg.(*ast.Ident); ok {
					out.WriteString(RustIdentForUse(ident))
				} else {
					TranspileExpression(out, arg)
				}
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()))")
				return
			}
		}
		if ident, ok := arg.(*ast.Ident); ok && !isVarBare(ident.Name) {
			if obj := typeInfo.GetObject(ident); obj != nil {
				if _, isConst := obj.(*types.Const); isConst {
					TranspileExpression(out, arg)
					return
				}
			}
			if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
				if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
					if _, ok := argType.Underlying().(*types.Basic); ok {
						varName := RustIdentForUse(ident)
						if currentCaptureRenames != nil {
							if renamed, exists := currentCaptureRenames[ident.Name]; exists {
								varName = RustLocalIdent(renamed)
							}
						}
						out.WriteString("{ let __v = (*")
						out.WriteString(varName)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).clone(); __v }")
						return
					}
				}
			}
		}
		// Type is known but not a map, slice, or pointer-to-struct - fall through
	} else {
		// Type info not available - add error comment
		out.WriteString("/* ERROR: Type information not available for print argument */ ")
	}

	// Check if this is a field access on self (already wrapped)
	if sel, ok := arg.(*ast.SelectorExpr); ok {
		if ident, ok := sel.X.(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver {
			// self.field - need to unwrap for display, resolving promoted fields
			fieldInfo := resolveFieldAccess(currentReceiverType, sel.Sel.Name)
			if fieldInfo.IsPromoted && len(fieldInfo.EmbeddedPath) > 0 {
				// Promoted field - traverse through embedded structs
				out.WriteString("(*")
				out.WriteString("self.")
				out.WriteString(ToSnakeCase(fieldInfo.EmbeddedPath[0]))
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()")
				for i := 1; i < len(fieldInfo.EmbeddedPath); i++ {
					out.WriteString(".")
					out.WriteString(ToSnakeCase(fieldInfo.EmbeddedPath[i]))
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()")
				}
				out.WriteString(".")
				out.WriteString(fieldInfo.FieldName)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())")
			} else {
				out.WriteString("(*self.")
				out.WriteString(ToSnakeCase(sel.Sel.Name))
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())")
			}
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

	// Slice expressions (e.g. s[:]) emit wrapped values - unwrap for print
	if _, ok := arg.(*ast.SliceExpr); ok {
		out.WriteString("(*")
		TranspileExpression(out, arg)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
		return
	}

	// For other cases, just use regular expression transpilation
	TranspileExpression(out, arg)
}

// convertFormatStringWithSkips converts Go format verbs to Rust format strings
// Returns: (format_string, skipIndices, charIndices, typeNameIndices, unicodeIndices, hexFormats)
func convertFormatStringWithSkips(goFormat string) (string, []int, []int, []int, []int, map[int]string) {
	var skipIndices []int
	var typeNameIndices []int
	var unicodeIndices []int
	hexFormats := make(map[int]string)
	format := goFormat
	quoted := false
	if unquoted, err := strconv.Unquote(goFormat); err == nil {
		format = unquoted
		quoted = true
	}

	// First, escape any literal curly braces that aren't part of Go format verbs
	// We need to do this before converting format verbs
	// Use placeholders to avoid double-escaping
	format = strings.ReplaceAll(format, "{", "__OPEN_BRACE__")
	format = strings.ReplaceAll(format, "}", "__CLOSE_BRACE__")

	// Track which arguments correspond to which verbs
	argIndex := 0
	var charIndices []int
	result := strings.Builder{}
	i := 0
	for i < len(format) {
		if i < len(format)-1 && format[i] == '%' {
			if format[i+1] == '%' {
				// Literal percent
				result.WriteString("%")
				i += 2
			} else if format[i+1] == 'T' {
				// %T - print Go type name of the argument
				typeNameIndices = append(typeNameIndices, argIndex)
				result.WriteString("{}")
				argIndex++
				i += 2
			} else if format[i+1] == '+' && i+2 < len(format) {
				switch format[i+2] {
				case 'v':
					result.WriteString("{:?}")
					argIndex++
					i += 3
				default:
					// Unknown flagged format, keep as-is
					result.WriteByte(format[i])
					result.WriteByte(format[i+1])
					i += 2
				}
			} else if format[i+1] == '#' && i+2 < len(format) {
				switch format[i+2] {
				case 'x', 'X':
					result.WriteString("{}")
					hexFormats[argIndex] = "#" + string(format[i+2])
					argIndex++
					i += 3
				case 'v':
					result.WriteString("{:?}")
					argIndex++
					i += 3
				default:
					// Unknown flagged format, keep as-is
					result.WriteByte(format[i])
					result.WriteByte(format[i+1])
					i += 2
				}
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
				j := i + 1
				leftAlign := false
				zeroPad := false
				if j < len(format) && format[j] == '-' {
					leftAlign = true
					j++
				}
				if j < len(format) && format[j] == '0' {
					zeroPad = true
					j++
				}
				widthStart := j
				for j < len(format) && format[j] >= '0' && format[j] <= '9' {
					j++
				}
				if j < len(format) && j > widthStart {
					width := format[widthStart:j]
					switch format[j] {
					case 's':
						if leftAlign {
							result.WriteString("{:<")
						} else {
							result.WriteString("{:>")
						}
						result.WriteString(width)
						result.WriteString("}")
						argIndex++
						i = j + 1
						continue
					case 'x', 'X':
						result.WriteString("{}")
						spec := ""
						if zeroPad {
							spec += "0"
						}
						spec += width
						spec += string(format[j])
						hexFormats[argIndex] = spec
						argIndex++
						i = j + 1
						continue
					}
				}

				// Handle single-char format verbs
				switch format[i+1] {
				case 'd', 's', 'v', 't', 'w':
					result.WriteString("{}")
					argIndex++
				case 'q':
					result.WriteString("{:?}")
					argIndex++
				case 'f':
					// Go's %f defaults to 6 decimal places
					result.WriteString("{:.6}")
					argIndex++
				case 'x':
					result.WriteString("{}")
					hexFormats[argIndex] = "x"
					argIndex++
				case 'X':
					result.WriteString("{}")
					hexFormats[argIndex] = "X"
					argIndex++
				case 'c':
					result.WriteString("{}")
					charIndices = append(charIndices, argIndex)
					argIndex++
				case 'b':
					result.WriteString("{:b}")
					argIndex++
				case 'U':
					result.WriteString("U+{:04X}")
					unicodeIndices = append(unicodeIndices, argIndex)
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
	if quoted {
		finalFormat = strconv.Quote(finalFormat)
	}

	return finalFormat, skipIndices, charIndices, typeNameIndices, unicodeIndices, hexFormats
}

// convertFormatString converts Go format strings to Rust format strings
func convertFormatString(goFormat string) string {
	converted, _, _, _, _, _ := convertFormatStringWithSkips(goFormat)
	return converted
}

func formatIndexMatches(indices []int, argIndex int) bool {
	for _, idx := range indices {
		if idx == argIndex {
			return true
		}
	}
	return false
}

func transpilePrintHexArg(out *strings.Builder, arg ast.Expr, formatSpec string) {
	upper := strings.Contains(formatSpec, "X")
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.IsByteSliceOrArray(arg) {
		NeedHexFormat()
		out.WriteString("go_format_hex_bytes(")
		writeBorrowedWrappedStdlibArg(out, arg)
		if upper {
			out.WriteString(", true)")
		} else {
			out.WriteString(", false)")
		}
		return
	}

	if typeInfo != nil && typeInfo.IsString(arg) {
		NeedHexFormat()
		out.WriteString("go_format_hex_bytes(")
		if lit, ok := arg.(*ast.BasicLit); ok && lit.Kind == token.STRING {
			TranspileExpression(out, lit)
			out.WriteString(".as_bytes()")
		} else {
			out.WriteString("(*")
			TranspileExpressionContext(out, arg, LValue)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).as_bytes()")
		}
		if upper {
			out.WriteString(", true)")
		} else {
			out.WriteString(", false)")
		}
		return
	}

	out.WriteString("format!(\"{:")
	out.WriteString(formatSpec)
	out.WriteString("}\", ")
	if ident, ok := arg.(*ast.Ident); ok {
		if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			transpilePrintArg(out, arg)
			out.WriteString(" as u32")
			out.WriteString(")")
			return
		}
	}
	transpilePrintArg(out, arg)
	out.WriteString(")")
}

func transpileFormatArg(out *strings.Builder, arg ast.Expr, argIndex int, charIndices []int, typeNameIndices []int, unicodeIndices []int, hexFormats map[int]string) {
	isTypeNameArg := false
	for _, tnIdx := range typeNameIndices {
		if tnIdx == argIndex {
			isTypeNameArg = true
			break
		}
	}
	isCharArg := false
	for _, charIdx := range charIndices {
		if charIdx == argIndex {
			isCharArg = true
			break
		}
	}
	isUnicodeArg := false
	for _, uIdx := range unicodeIndices {
		if uIdx == argIndex {
			isUnicodeArg = true
			break
		}
	}
	if isTypeNameArg {
		NeedGoTypeName()
		if ident, ok := arg.(*ast.Ident); ok {
			if isVarBare(ident.Name) {
				out.WriteString("go_type_name(")
				out.WriteString(ident.Name)
				out.WriteString(")")
			} else {
				out.WriteString("go_type_name(&**")
				out.WriteString(ident.Name)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())")
			}
		} else {
			out.WriteString("go_type_name(&*")
			transpilePrintArg(out, arg)
			out.WriteString(")")
		}
	} else if isUnicodeArg {
		transpilePrintArg(out, arg)
		out.WriteString(" as u32")
	} else if hexSpec, isHexArg := hexFormats[argIndex]; isHexArg {
		transpilePrintHexArg(out, arg, hexSpec)
	} else if isCharArg {
		isAlreadyChar := false
		if ident, ok := arg.(*ast.Ident); ok {
			if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
				isAlreadyChar = true
			}
		}
		if isAlreadyChar {
			transpilePrintArg(out, arg)
		} else {
			out.WriteString("(")
			transpilePrintArg(out, arg)
			out.WriteString(") as u8 as char")
		}
	} else {
		transpilePrintArg(out, arg)
	}
}

func transpileFmtPrintf(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("print!")
	out.WriteString("(")

	var skipIndices []int
	var charIndices []int
	var typeNameIndices []int
	var unicodeIndices []int
	hexFormats := make(map[int]string)
	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust and get skip/char/typeName indices
			format, skips, chars, typeNames, unicodes, hexes := convertFormatStringWithSkips(lit.Value)
			skipIndices = skips
			charIndices = chars
			typeNameIndices = typeNames
			unicodeIndices = unicodes
			hexFormats = hexes
			out.WriteString(format)
		} else {
			out.WriteString("\"{}\"")
			out.WriteString(", ")
			TranspileExpression(out, call.Args[0])
			out.WriteString(")")
			return
		}

		// Rest of the arguments, skipping those no longer needed
		for i := 1; i < len(call.Args); i++ {
			// Check if this argument index should be skipped (0-based in skipIndices)
			shouldSkip := false
			for _, skipIdx := range skipIndices {
				if skipIdx == i-1 {
					shouldSkip = true
					break
				}
			}
			if !shouldSkip {
				out.WriteString(", ")
				transpileFormatArg(out, call.Args[i], i-1, charIndices, typeNameIndices, unicodeIndices, hexFormats)
			}
		}
	}

	out.WriteString(")")
}

// isOsStderr checks if an expression is os.Stderr
func isOsStderr(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	ident, ok := sel.X.(*ast.Ident)
	return ok && ident.Name == "os" && sel.Sel.Name == "Stderr"
}

func transpileFmtFprintln(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 1 {
		out.WriteString("/* ERROR: fmt.Fprintln requires at least 1 argument */")
		return
	}
	// Check if writing to stderr
	if isOsStderr(call.Args[0]) {
		out.WriteString("eprintln!")
		out.WriteString("(")
		// Remaining args (skip the writer)
		remaining := call.Args[1:]
		if len(remaining) > 0 {
			out.WriteString("\"")
			for i := range remaining {
				if i > 0 {
					out.WriteString(" ")
				}
				out.WriteString("{}")
			}
			out.WriteString("\"")
			for _, arg := range remaining {
				out.WriteString(", ")
				transpilePrintArg(out, arg)
			}
		}
		out.WriteString(")")
	} else {
		// Default to stdout for os.Stdout or other writers
		out.WriteString("println!")
		out.WriteString("(")
		remaining := call.Args[1:]
		if len(remaining) > 0 {
			out.WriteString("\"")
			for i := range remaining {
				if i > 0 {
					out.WriteString(" ")
				}
				out.WriteString("{}")
			}
			out.WriteString("\"")
			for _, arg := range remaining {
				out.WriteString(", ")
				transpilePrintArg(out, arg)
			}
		}
		out.WriteString(")")
	}
}

func transpileFmtFprintf(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		out.WriteString("/* ERROR: fmt.Fprintf requires at least 2 arguments */")
		return
	}
	// Check if writing to stderr
	macro := "print!"
	if isOsStderr(call.Args[0]) {
		macro = "eprint!"
	}
	out.WriteString(macro)
	out.WriteString("(")

	// Second arg is the format string, remaining are values
	var skipIndices []int
	var charIndices []int
	var typeNameIndices []int
	var unicodeIndices []int
	hexFormats := make(map[int]string)
	if lit, ok := call.Args[1].(*ast.BasicLit); ok && lit.Kind == token.STRING {
		format, skips, chars, typeNames, unicodes, hexes := convertFormatStringWithSkips(lit.Value)
		skipIndices = skips
		charIndices = chars
		typeNameIndices = typeNames
		unicodeIndices = unicodes
		hexFormats = hexes
		out.WriteString(format)
	} else {
		out.WriteString("\"{}\"")
		out.WriteString(", ")
		TranspileExpression(out, call.Args[1])
		out.WriteString(")")
		return
	}

	for i := 2; i < len(call.Args); i++ {
		shouldSkip := false
		for _, skipIdx := range skipIndices {
			if skipIdx == i-2 {
				shouldSkip = true
				break
			}
		}
		if !shouldSkip {
			out.WriteString(", ")
			transpileFormatArg(out, call.Args[i], i-2, charIndices, typeNameIndices, unicodeIndices, hexFormats)
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
	var typeNameIndices []int
	hexFormats := make(map[int]string)
	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust and get skip indices
			format, skips, _, typeNames, _, hexes := convertFormatStringWithSkips(lit.Value)
			skipIndices = skips
			typeNameIndices = typeNames
			hexFormats = hexes
			out.WriteString(format)
		} else {
			out.WriteString("\"{}\"")
			out.WriteString(", ")
			writeOwnedStringStdlibArg(out, call.Args[0])
			out.WriteString(")")
			WriteWrapperSuffix(out)
			return
		}

		// Rest of the arguments, skipping those no longer needed
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
				isTypeNameArg := false
				for _, tnIdx := range typeNameIndices {
					if tnIdx == i-1 {
						isTypeNameArg = true
						break
					}
				}
				if isTypeNameArg {
					NeedGoTypeName()
					out.WriteString("go_type_name(&*")
					transpilePrintArg(out, call.Args[i])
					out.WriteString(")")
				} else if hexSpec, isHexArg := hexFormats[i-1]; isHexArg {
					transpilePrintHexArg(out, call.Args[i], hexSpec)
				} else {
					transpilePrintArg(out, call.Args[i])
				}
			}
		}
	}

	out.WriteString("))))")
}

func transpileFmtErrorf(out *strings.Builder, call *ast.CallExpr) {
	TrackImport("Error")
	WriteWrapperPrefix(out)
	if NeedsConcurrentWrapper() {
		out.WriteString("Box::<dyn StdError + Send + Sync>::from(format!")
	} else {
		out.WriteString("Box::<dyn StdError>::from(format!")
	}
	out.WriteString("(")

	if len(call.Args) > 0 {
		// First arg is the format string
		literalFormat := false
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust
			format := convertFormatString(lit.Value)
			out.WriteString(format)
			literalFormat = true
		} else {
			out.WriteString("\"{}\", ")
			writeOwnedStringStdlibArg(out, call.Args[0])
		}

		// Rest of the arguments
		if literalFormat {
			for i := 1; i < len(call.Args); i++ {
				out.WriteString(", ")
				TranspileExpression(out, call.Args[i])
			}
		}
	}

	out.WriteString(")))))")
}

func transpileErrorsNew(out *strings.Builder, call *ast.CallExpr) {
	WriteWrapperPrefix(out)
	if NeedsConcurrentWrapper() {
		out.WriteString("Box::<dyn std::error::Error + Send + Sync>::from(")
	} else {
		out.WriteString("Box::<dyn std::error::Error>::from(")
	}

	if len(call.Args) > 0 {
		// The argument is the error message
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// String literal - already has quotes
			out.WriteString(RustStringLiteral(lit.Value))
			out.WriteString(".to_string()")
		} else {
			// Expression - might already be a string
			TranspileExpression(out, call.Args[0])
		}
	}

	out.WriteString("))))")
}

func transpileFlagString(out *strings.Builder, call *ast.CallExpr) {
	WriteWrapperPrefix(out)
	if len(call.Args) > 1 {
		writeOwnedStringStdlibArg(out, call.Args[1])
	} else {
		out.WriteString("String::new()")
	}
	WriteWrapperSuffix(out)
}

func transpileFlagParse(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("()")
}

func transpileOsCreate(out *strings.Builder, call *ast.CallExpr) {
	NeedOsFile()
	TrackImport("Error")
	out.WriteString("{ let __path = ")
	if len(call.Args) > 0 {
		writeOwnedStringStdlibArg(out, call.Args[0])
	} else {
		out.WriteString("String::new()")
	}
	out.WriteString("; match GoFile::create(&__path) { Ok(file) => (")
	WriteWrapperPrefix(out)
	out.WriteString("file")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	writeTypedErrorNone(out)
	out.WriteString("), Err(e) => (")
	WriteWrapperPrefix(out)
	out.WriteString("GoFile::empty()")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	WriteWrapperPrefix(out)
	if NeedsConcurrentWrapper() {
		out.WriteString("Box::<dyn StdError + Send + Sync>::from(e)")
	} else {
		out.WriteString("Box::<dyn StdError>::from(e)")
	}
	WriteWrapperSuffix(out)
	out.WriteString(") } }")
}

func transpileOsRemove(out *strings.Builder, call *ast.CallExpr) {
	TrackImport("Error")
	out.WriteString("{ let __path = ")
	if len(call.Args) > 0 {
		writeOwnedStringStdlibArg(out, call.Args[0])
	} else {
		out.WriteString("String::new()")
	}
	out.WriteString("; match std::fs::remove_file(&__path) { Ok(()) => ")
	writeTypedErrorNone(out)
	out.WriteString(", Err(e) => ")
	WriteWrapperPrefix(out)
	if NeedsConcurrentWrapper() {
		out.WriteString("Box::<dyn StdError + Send + Sync>::from(e)")
	} else {
		out.WriteString("Box::<dyn StdError>::from(e)")
	}
	WriteWrapperSuffix(out)
	out.WriteString(" } }")
}

func writeAtomicTarget(out *strings.Builder, expr ast.Expr) {
	TranspileExpression(out, expr)
}

func transpileAtomicAddInt64(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		out.WriteString("/* ERROR: atomic.AddInt64 requires pointer and delta */ unimplemented!()")
		return
	}

	WriteWrapperPrefix(out)
	out.WriteString("{ let __target = ")
	writeAtomicTarget(out, call.Args[0])
	out.WriteString("; let __delta = ")
	TranspileExpression(out, call.Args[1])
	out.WriteString(" as i64; ")
	if NeedsConcurrentWrapper() {
		out.WriteString("let mut __guard = __target.lock().unwrap();")
	} else {
		out.WriteString("let mut __guard = __target.borrow_mut();")
	}
	out.WriteString(" let __value = __guard.as_mut().unwrap(); *__value += __delta; *__value }")
	WriteWrapperSuffix(out)
}

func transpileAtomicLoadInt64(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 1 {
		out.WriteString("/* ERROR: atomic.LoadInt64 requires pointer */ unimplemented!()")
		return
	}

	WriteWrapperPrefix(out)
	out.WriteString("{ let __target = ")
	writeAtomicTarget(out, call.Args[0])
	if NeedsConcurrentWrapper() {
		out.WriteString("; let __guard = __target.lock().unwrap();")
	} else {
		out.WriteString("; let __guard = __target.borrow();")
	}
	out.WriteString(" *__guard.as_ref().unwrap() }")
	WriteWrapperSuffix(out)
}

func writeReflectString(out *strings.Builder, value string) {
	WriteWrapperPrefix(out)
	out.WriteString(strconv.Quote(value))
	out.WriteString(".to_string()")
	WriteWrapperSuffix(out)
}

func transpileReflectTypeOf(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) == 0 {
		out.WriteString("/* ERROR: reflect.TypeOf requires a value */ unimplemented!()")
		return
	}

	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for reflect.TypeOf */ unimplemented!()")
		return
	}

	st := typeInfo.GetStructType(call.Args[0])
	if st == nil {
		out.WriteString("/* ERROR: reflect.TypeOf currently supports struct values */ unimplemented!()")
		return
	}

	NeedReflect()
	WriteWrapperPrefix(out)
	out.WriteString("GoReflectType { fields: ")
	WriteWrapperPrefix(out)
	out.WriteString("vec![")
	for i := 0; i < st.NumFields(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString("GoReflectField { name: ")
		writeReflectString(out, st.Field(i).Name())
		out.WriteString(", tag: ")
		WriteWrapperPrefix(out)
		out.WriteString("GoReflectStructTag { raw: ")
		writeReflectString(out, st.Tag(i))
		out.WriteString(" }")
		WriteWrapperSuffix(out)
		out.WriteString(" }")
	}
	out.WriteString("]")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
	WriteWrapperSuffix(out)
}

func writeOwnedStringStdlibArg(out *strings.Builder, arg ast.Expr) {
	if lit, ok := arg.(*ast.BasicLit); ok && lit.Kind == token.STRING {
		TranspileExpression(out, lit)
		return
	}

	out.WriteString("(*")
	TranspileExpressionContext(out, arg, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
}

func writeOwnedStringSliceStdlibArg(out *strings.Builder, arg ast.Expr) {
	out.WriteString("(*")
	TranspileExpressionContext(out, arg, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
}

func writeStringBinaryResult(out *strings.Builder, call *ast.CallExpr, method string) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __arg = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; __s.")
	out.WriteString(method)
	out.WriteString("(&__arg) }")
	WriteWrapperSuffix(out)
}

func transpileStringsToUpper(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		WriteWrapperPrefix(out)
		TranspileExpression(out, call.Args[0])
		out.WriteString(".to_uppercase()")
		WriteWrapperSuffix(out)
	}
}

func transpileStringsToLower(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		WriteWrapperPrefix(out)
		TranspileExpression(out, call.Args[0])
		out.WriteString(".to_lowercase()")
		WriteWrapperSuffix(out)
	}
}

func transpileStringsTrimSpace(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		WriteWrapperPrefix(out)
		TranspileExpression(out, call.Args[0])
		out.WriteString(".trim()")
		out.WriteString(".to_string()")
		WriteWrapperSuffix(out)
	}
}

func transpileStringsTitle(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) == 0 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let mut __out = String::new(); let mut __new_word = true; for __ch in __s.chars() { if __ch.is_alphanumeric() { if __new_word { for __upper in __ch.to_uppercase() { __out.push(__upper); } } else { __out.push(__ch); } __new_word = false; } else { __out.push(__ch); __new_word = true; } } __out }")
	WriteWrapperSuffix(out)
}

func transpileStringsContains(out *strings.Builder, call *ast.CallExpr) {
	writeStringBinaryResult(out, call, "contains")
}

func transpileStringsIndex(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __substr = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; __s.find(&__substr).map(|__i| __i as i32).unwrap_or(-1) }")
	WriteWrapperSuffix(out)
}

func transpileStringsLastIndex(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __substr = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; __s.rfind(&__substr).map(|__i| __i as i32).unwrap_or(-1) }")
	WriteWrapperSuffix(out)
}

func transpileStringsCount(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __substr = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; if __substr.is_empty() { __s.chars().count() as i32 + 1 } else { __s.matches(&__substr).count() as i32 } }")
	WriteWrapperSuffix(out)
}

func transpileStringsCompare(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __a = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __b = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; match __a.cmp(&__b) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 } }")
	WriteWrapperSuffix(out)
}

func transpileStringsCut(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __sep = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; if let Some(__idx) = __s.find(&__sep) { let __before = __s[..__idx].to_string(); let __after = __s[__idx + __sep.len()..].to_string(); (")
	WriteWrapperPrefix(out)
	out.WriteString("__before")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	WriteWrapperPrefix(out)
	out.WriteString("__after")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	WriteWrapperPrefix(out)
	out.WriteString("true")
	WriteWrapperSuffix(out)
	out.WriteString(") } else { (")
	WriteWrapperPrefix(out)
	out.WriteString("__s")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	WriteWrapperPrefix(out)
	out.WriteString("String::new()")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	WriteWrapperPrefix(out)
	out.WriteString("false")
	WriteWrapperSuffix(out)
	out.WriteString(") } }")
}

func transpileStringsHasSuffix(out *strings.Builder, call *ast.CallExpr) {
	writeStringBinaryResult(out, call, "ends_with")
}

func transpileStringsHasPrefix(out *strings.Builder, call *ast.CallExpr) {
	writeStringBinaryResult(out, call, "starts_with")
}

func transpileStringsIndexAny(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __chars = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; __s.char_indices().find(|(_, __ch)| __chars.contains(*__ch)).map(|(__i, _)| __i as i32).unwrap_or(-1) }")
	WriteWrapperSuffix(out)
}

func transpileStringsSplit(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __sep = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() }")
	WriteWrapperSuffix(out)
}

func transpileStringsJoin(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __parts = ")
	writeOwnedStringSliceStdlibArg(out, call.Args[0])
	out.WriteString("; let __sep = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; __parts.join(&__sep) }")
	WriteWrapperSuffix(out)
}

func transpileStringsFields(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) == 0 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() }")
	WriteWrapperSuffix(out)
}

func transpileStringsReplace(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 4 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __old = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; let __new = ")
	writeOwnedStringStdlibArg(out, call.Args[2])
	out.WriteString("; let __n = ")
	TranspileExpression(out, call.Args[3])
	out.WriteString("; if __n < 0 { __s.replace(&__old, &__new) } else { __s.replacen(&__old, &__new, __n as usize) } }")
	WriteWrapperSuffix(out)
}

func transpileStringsReplaceAll(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 3 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __old = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; let __new = ")
	writeOwnedStringStdlibArg(out, call.Args[2])
	out.WriteString("; __s.replace(&__old, &__new) }")
	WriteWrapperSuffix(out)
}

func transpileStringsRepeat(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __count = ")
	TranspileExpression(out, call.Args[1])
	out.WriteString("; __s.repeat(__count as usize) }")
	WriteWrapperSuffix(out)
}

func transpileStringsEqualFold(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __a = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __b = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; __a.to_lowercase() == __b.to_lowercase() }")
	WriteWrapperSuffix(out)
}

func transpileStringsTrimLeft(out *strings.Builder, call *ast.CallExpr) {
	transpileStringsTrimCutset(out, call, "trim_start_matches")
}

func transpileStringsTrimRight(out *strings.Builder, call *ast.CallExpr) {
	transpileStringsTrimCutset(out, call, "trim_end_matches")
}

func transpileStringsTrim(out *strings.Builder, call *ast.CallExpr) {
	transpileStringsTrimCutset(out, call, "trim_matches")
}

func transpileStringsTrimCutset(out *strings.Builder, call *ast.CallExpr, method string) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __cutset = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; __s.")
	out.WriteString(method)
	out.WriteString("(|__ch| __cutset.contains(__ch)).to_string() }")
	WriteWrapperSuffix(out)
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

func writeSortFuncWrappedElement(out *strings.Builder, name string) {
	WriteWrapperPrefix(out)
	out.WriteString(name)
	out.WriteString(".clone()")
	WriteWrapperSuffix(out)
}

func writeSortFuncComparatorCall(out *strings.Builder, cmp ast.Expr) {
	if writeDirectFunctionReference(out, cmp) {
		out.WriteString("(")
		writeSortFuncWrappedElement(out, "__a")
		out.WriteString(", ")
		writeSortFuncWrappedElement(out, "__b")
		out.WriteString(")")
		return
	}

	out.WriteString("{ let __cmp_guard = __cmp_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __cmp_fn = __cmp_guard.as_ref().unwrap(); (*__cmp_fn)(")
	writeSortFuncWrappedElement(out, "__a")
	out.WriteString(", ")
	writeSortFuncWrappedElement(out, "__b")
	out.WriteString(") }")
}

func writeDirectFunctionReference(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}

	switch e := expr.(type) {
	case *ast.Ident:
		if _, ok := typeInfo.GetObject(e).(*types.Func); !ok {
			return false
		}
		out.WriteString(rustFunctionNameForUse(e.Name))
		return true
	case *ast.SelectorExpr:
		if _, ok := typeInfo.GetObject(e.Sel).(*types.Func); !ok {
			return false
		}
		TranspileExpression(out, e)
		return true
	default:
		return false
	}
}

func transpileSlicesSortFunc(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}

	usesFunctionValue := false
	var direct strings.Builder
	if !writeDirectFunctionReference(&direct, call.Args[1]) {
		usesFunctionValue = true
	}

	if usesFunctionValue {
		out.WriteString("{ let __cmp_holder = ")
		TranspileExpression(out, call.Args[1])
		out.WriteString("; ")
	}
	out.WriteString("(*")
	TranspileExpressionContext(out, call.Args[0], LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(".as_mut().unwrap()).sort_by(|__a, __b| { let __cmp = ")
	if usesFunctionValue {
		writeSortFuncComparatorCall(out, call.Args[1])
	} else {
		out.WriteString(direct.String())
		out.WriteString("(")
		writeSortFuncWrappedElement(out, "__a")
		out.WriteString(", ")
		writeSortFuncWrappedElement(out, "__b")
		out.WriteString(")")
	}
	out.WriteString("; let __ord = (*__cmp")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).cmp(&0); __ord })")
	if usesFunctionValue {
		out.WriteString(" }")
	}
}

func transpileSlicesContains(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __slice_holder = ")
	TranspileExpressionContext(out, call.Args[0], LValue)
	out.WriteString(".clone(); let __slice_guard = __slice_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __slice = __slice_guard.as_ref().unwrap(); let __value = ")
	writeMaybeUnwrappedExpression(out, call.Args[1])
	out.WriteString("; __slice.contains(&__value) }")
	WriteWrapperSuffix(out)
}

func transpileStrconvItoa(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		WriteWrapperPrefix(out)
		TranspileExpression(out, call.Args[0])
		out.WriteString(".to_string()")
		WriteWrapperSuffix(out)
	}
}

func transpileStrconvAtoi(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		// Capture input string for Go-compatible error message
		out.WriteString("{ let __atoi_input = ")
		TranspileExpression(out, call.Args[0])
		out.WriteString(".clone(); match __atoi_input.parse::<i32>() { ")
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
		if NeedsConcurrentWrapper() {
			out.WriteString("Box::<dyn StdError + Send + Sync>::from(format!(\"strconv.Atoi: parsing \\\"{}\\\": invalid syntax\", __atoi_input)))))) } }")
		} else {
			out.WriteString("Box::<dyn StdError>::from(format!(\"strconv.Atoi: parsing \\\"{}\\\": invalid syntax\", __atoi_input)))))) } }")
		}
	}
}

func transpileStrconvFormatFloat(out *strings.Builder, call *ast.CallExpr) {
	NeedStrconvFormat()
	WriteWrapperPrefix(out)
	out.WriteString("go_strconv_format_float(")
	if len(call.Args) > 0 {
		writeUnwrappedForFormat(out, call.Args[0])
	} else {
		out.WriteString("0.0")
	}
	out.WriteString(" as f64, ")
	if len(call.Args) > 1 {
		out.WriteString("char::from_u32((")
		writeUnwrappedForFormat(out, call.Args[1])
		out.WriteString(") as u32).unwrap_or('f')")
	} else {
		out.WriteString("'f'")
	}
	out.WriteString(", ")
	if len(call.Args) > 2 {
		writeUnwrappedForFormat(out, call.Args[2])
	} else {
		out.WriteString("-1")
	}
	out.WriteString(" as i32)")
	WriteWrapperSuffix(out)
}

func transpileStrconvFormatInt(out *strings.Builder, call *ast.CallExpr) {
	NeedStrconvFormat()
	WriteWrapperPrefix(out)
	out.WriteString("go_strconv_format_int(")
	if len(call.Args) > 0 {
		writeUnwrappedForFormat(out, call.Args[0])
	} else {
		out.WriteString("0")
	}
	out.WriteString(" as i64, ")
	if len(call.Args) > 1 {
		writeUnwrappedForFormat(out, call.Args[1])
	} else {
		out.WriteString("10")
	}
	out.WriteString(" as i32)")
	WriteWrapperSuffix(out)
}

func writeBorrowedWrappedStdlibArg(out *strings.Builder, arg ast.Expr) {
	if lit, ok := arg.(*ast.BasicLit); ok && lit.Kind == token.STRING {
		out.WriteString("&")
		TranspileExpression(out, arg)
		return
	}

	out.WriteString("&*")
	if ident, ok := arg.(*ast.Ident); ok {
		out.WriteString(ident.Name)
	} else {
		out.WriteString("(")
		TranspileExpression(out, arg)
		out.WriteString(")")
	}
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
}

func transpileBase64EncodeToString(out *strings.Builder, call *ast.CallExpr) {
	NeedBase64()
	if len(call.Args) == 0 {
		WriteWrapperPrefix(out)
		out.WriteString("String::new()")
		WriteWrapperSuffix(out)
		return
	}

	WriteWrapperPrefix(out)
	out.WriteString("go_base64_encode(")
	writeBorrowedWrappedStdlibArg(out, call.Args[0])
	out.WriteString(")")
	WriteWrapperSuffix(out)
}

func transpileBase64DecodeString(out *strings.Builder, call *ast.CallExpr) {
	NeedBase64()
	TrackImport("Error")
	out.WriteString("{ match go_base64_decode(")
	if len(call.Args) > 0 {
		writeBorrowedWrappedStdlibArg(out, call.Args[0])
	} else {
		out.WriteString("&String::new()")
	}
	out.WriteString(") { Ok(v) => (")
	WriteWrapperPrefix(out)
	out.WriteString("v")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	WriteWrappedNone(out)
	out.WriteString("), Err(e) => (")
	WriteWrapperPrefix(out)
	out.WriteString("Vec::<u8>::new()")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	WriteWrapperPrefix(out)
	if NeedsConcurrentWrapper() {
		out.WriteString("Box::<dyn StdError + Send + Sync>::from(e)")
	} else {
		out.WriteString("Box::<dyn StdError>::from(e)")
	}
	WriteWrapperSuffix(out)
	out.WriteString(") } }")
}

func transpileSha256Sum256(out *strings.Builder, call *ast.CallExpr) {
	NeedSha256()
	WriteWrapperPrefix(out)
	out.WriteString("go_sha256_sum256(")
	if len(call.Args) > 0 {
		writeBorrowedWrappedStdlibArg(out, call.Args[0])
	} else {
		out.WriteString("&[]")
	}
	out.WriteString(")")
	WriteWrapperSuffix(out)
}

type jsonMarshalField struct {
	jsonName string
	rustName string
	kind     types.BasicKind
}

func jsonFieldName(goName, tag string) (string, bool) {
	jsonTag := reflect.StructTag(tag).Get("json")
	if jsonTag == "-" {
		return "", false
	}
	if idx := strings.Index(jsonTag, ","); idx >= 0 {
		jsonTag = jsonTag[:idx]
	}
	if jsonTag == "" {
		return goName, true
	}
	return jsonTag, true
}

func jsonMarshalStructFields(st *types.Struct) ([]jsonMarshalField, bool) {
	fields := []jsonMarshalField{}
	for i := 0; i < st.NumFields(); i++ {
		field := st.Field(i)
		if !field.Exported() {
			continue
		}
		jsonName, include := jsonFieldName(field.Name(), st.Tag(i))
		if !include {
			continue
		}
		basic, ok := field.Type().Underlying().(*types.Basic)
		if !ok {
			return nil, false
		}
		switch basic.Kind() {
		case types.Bool,
			types.Int, types.Int8, types.Int16, types.Int32, types.Int64,
			types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64, types.Uintptr,
			types.Float32, types.Float64,
			types.String:
			fields = append(fields, jsonMarshalField{
				jsonName: jsonName,
				rustName: ToSnakeCase(field.Name()),
				kind:     basic.Kind(),
			})
		default:
			return nil, false
		}
	}
	return fields, true
}

func escapeRustFormatLiteral(s string) string {
	s = strings.ReplaceAll(s, "{", "{{")
	return strings.ReplaceAll(s, "}", "}}")
}

func transpileJsonMarshal(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) == 0 {
		out.WriteString("/* ERROR: json.Marshal requires a value */ unimplemented!()")
		return
	}

	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for json.Marshal */ unimplemented!()")
		return
	}

	st := typeInfo.GetStructType(call.Args[0])
	if st == nil {
		out.WriteString("/* ERROR: json.Marshal currently supports struct values with exported basic fields */ unimplemented!()")
		return
	}

	fields, ok := jsonMarshalStructFields(st)
	if !ok {
		out.WriteString("/* ERROR: json.Marshal currently supports exported bool, numeric, and string struct fields */ unimplemented!()")
		return
	}

	format := strings.Builder{}
	format.WriteString("{{")
	for i, field := range fields {
		if i > 0 {
			format.WriteString(",")
		}
		format.WriteString(escapeRustFormatLiteral(strconv.Quote(field.jsonName)))
		format.WriteString(":")
		if field.kind == types.String {
			format.WriteString("\"{}\"")
			NeedJsonEscape()
		} else {
			format.WriteString("{}")
		}
	}
	format.WriteString("}}")

	out.WriteString("{ let __json_input = ")
	if ident, ok := call.Args[0].(*ast.Ident); ok {
		out.WriteString(ident.Name)
		out.WriteString(".clone()")
	} else {
		TranspileExpression(out, call.Args[0])
	}
	out.WriteString("; let __json_guard = __json_input")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __json_value = __json_guard.as_ref().unwrap(); let __json = format!(")
	out.WriteString(strconv.Quote(format.String()))
	for _, field := range fields {
		out.WriteString(", ")
		if field.kind == types.String {
			out.WriteString("go_json_escape(&*__json_value.")
			out.WriteString(field.rustName)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		} else {
			out.WriteString("(*__json_value.")
			out.WriteString(field.rustName)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		}
	}
	out.WriteString("); (")
	WriteWrapperPrefix(out)
	out.WriteString("__json.into_bytes()")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	writeTypedErrorNone(out)
	out.WriteString(") }")
}

func transpileMathSqrt(out *strings.Builder, call *ast.CallExpr) {
	transpileMathUnary(out, call, "sqrt")
}

func transpileMathPow(out *strings.Builder, call *ast.CallExpr) {
	transpileMathBinary(out, call, "powf")
}

func transpileMathMax(out *strings.Builder, call *ast.CallExpr) {
	transpileMathBinary(out, call, "max")
}

func transpileMathMin(out *strings.Builder, call *ast.CallExpr) {
	transpileMathBinary(out, call, "min")
}

func transpileMathUnary(out *strings.Builder, call *ast.CallExpr, method string) {
	if len(call.Args) == 0 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("(")
	writeUnwrappedForFormat(out, call.Args[0])
	out.WriteString(" as f64).")
	out.WriteString(method)
	out.WriteString("()")
	WriteWrapperSuffix(out)
}

func transpileMathBinary(out *strings.Builder, call *ast.CallExpr, method string) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("(")
	writeUnwrappedForFormat(out, call.Args[0])
	out.WriteString(" as f64).")
	out.WriteString(method)
	out.WriteString("(")
	writeUnwrappedForFormat(out, call.Args[1])
	out.WriteString(" as f64)")
	WriteWrapperSuffix(out)
}

func transpileUnsafeSizeof(out *strings.Builder, call *ast.CallExpr) {
	transpileUnsafeTypeSizeCall(out, call, "Sizeof", "size_of")
}

func transpileUnsafeAlignof(out *strings.Builder, call *ast.CallExpr) {
	transpileUnsafeTypeSizeCall(out, call, "Alignof", "align_of")
}

func transpileUnsafeTypeSizeCall(out *strings.Builder, call *ast.CallExpr, goFunc string, rustFunc string) {
	WriteWrapperPrefix(out)
	if len(call.Args) == 0 {
		out.WriteString("/* ERROR: unsafe.")
		out.WriteString(goFunc)
		out.WriteString(" requires an argument */ unimplemented!()")
		WriteWrapperSuffix(out)
		return
	}

	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for unsafe.")
		out.WriteString(goFunc)
		out.WriteString(" */ unimplemented!()")
		WriteWrapperSuffix(out)
		return
	}

	argType := typeInfo.GetType(call.Args[0])
	if argType == nil {
		out.WriteString("/* ERROR: Type information unavailable for unsafe.")
		out.WriteString(goFunc)
		out.WriteString(" */ unimplemented!()")
		WriteWrapperSuffix(out)
		return
	}

	out.WriteString("std::mem::")
	out.WriteString(rustFunc)
	out.WriteString("::<")
	out.WriteString(goTypesTypeToRust(argType))
	out.WriteString(">()")
	WriteWrapperSuffix(out)
}

func transpileUnsafeOffsetof(out *strings.Builder, call *ast.CallExpr) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString("Arc::new(Mutex::new(Some::<usize>(unimplemented!(\"unsafe.Offsetof requires struct layout support\"))))")
		return
	}
	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString("Rc::new(RefCell::new(Some::<usize>(unimplemented!(\"unsafe.Offsetof requires struct layout support\"))))")
}

func transpileRandSeed(out *strings.Builder, call *ast.CallExpr) {
	NeedGoRand()
	out.WriteString("go_rand_seed(")
	if len(call.Args) > 0 {
		writeUnwrappedForFormat(out, call.Args[0])
	} else {
		out.WriteString("1")
	}
	out.WriteString(" as i64)")
}

func transpileRandIntn(out *strings.Builder, call *ast.CallExpr) {
	NeedGoRand()
	WriteWrapperPrefix(out)
	out.WriteString("go_rand_intn(")
	if len(call.Args) > 0 {
		writeUnwrappedForFormat(out, call.Args[0])
	} else {
		out.WriteString("0")
	}
	out.WriteString(" as i32)")
	WriteWrapperSuffix(out)
}

func transpileRandFloat64(out *strings.Builder, call *ast.CallExpr) {
	NeedGoRand()
	WriteWrapperPrefix(out)
	out.WriteString("go_rand_float64()")
	WriteWrapperSuffix(out)
}

func writeTypedErrorNone(out *strings.Builder) {
	TrackImport("Error")
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString("Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))")
		return
	}
	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString("Rc::new(RefCell::new(None::<Box<dyn StdError>>))")
}

func transpileUrlParse(out *strings.Builder, call *ast.CallExpr) {
	NeedUrl()
	out.WriteString("{ let __url_input = ")
	if len(call.Args) > 0 {
		writeOwnedStringStdlibArg(out, call.Args[0])
	} else {
		out.WriteString("String::new()")
	}
	out.WriteString("; (")
	WriteWrapperPrefix(out)
	out.WriteString("go_url_parse(&__url_input)")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	writeTypedErrorNone(out)
	out.WriteString(") }")
}

func transpileRegexpMustCompile(out *strings.Builder, call *ast.CallExpr) {
	NeedRegexp()
	WriteWrapperPrefix(out)
	out.WriteString("GoRegexp { pattern: ")
	WriteWrapperPrefix(out)
	if len(call.Args) > 0 {
		writeOwnedStringStdlibArg(out, call.Args[0])
	} else {
		out.WriteString("String::new()")
	}
	WriteWrapperSuffix(out)
	out.WriteString(" }")
	WriteWrapperSuffix(out)
}

func transpileAppend(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 2 {
		writeAppendTarget := func(expr ast.Expr) {
			if ident, ok := expr.(*ast.Ident); ok {
				out.WriteString(ident.Name)
				return
			}
			switch expr.(type) {
			case *ast.SelectorExpr, *ast.IndexExpr:
				TranspileExpressionContext(out, expr, LValue)
			default:
				TranspileExpression(out, expr)
			}
		}

		// append() in Go returns the slice, but our slices are wrapped
		// We need to create the vector on first append so nil slices stay nil
		// until they are actually appended to, then return the wrapped slice.
		if call.Ellipsis.IsValid() {
			// Slice expansion: append(dst, src...) → extend from src
			out.WriteString("{(*")
			writeAppendTarget(call.Args[0])
			WriteBorrowMethod(out, true)
			out.WriteString(").get_or_insert_with(Vec::new).extend(")
			TranspileExpression(out, call.Args[1])
			out.WriteString(".iter().cloned()); ")
			// Return the wrapped slice itself
			writeAppendTarget(call.Args[0])
			out.WriteString(".clone()}")
		} else if len(call.Args) == 2 {
			// Single element append
			out.WriteString("{(*")
			writeAppendTarget(call.Args[0])
			WriteBorrowMethod(out, true)
			out.WriteString(").get_or_insert_with(Vec::new).push(")
			if !writeOwnedExpressionValue(out, call.Args[1]) {
				TranspileExpression(out, call.Args[1])
			}
			out.WriteString("); ")
			// Return the wrapped slice itself
			writeAppendTarget(call.Args[0])
			out.WriteString(".clone()}")
		} else {
			// Multiple elements, use extend
			out.WriteString("{(*")
			writeAppendTarget(call.Args[0])
			WriteBorrowMethod(out, true)
			out.WriteString(").get_or_insert_with(Vec::new).extend(vec![")
			for i := 1; i < len(call.Args); i++ {
				if i > 1 {
					out.WriteString(", ")
				}
				if !writeOwnedExpressionValue(out, call.Args[i]) {
					TranspileExpression(out, call.Args[i])
				}
			}
			out.WriteString("]); ")
			// Return the wrapped slice itself
			writeAppendTarget(call.Args[0])
			out.WriteString(".clone()}")
		}
	}
}

func transpileLen(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.IsChannel(call.Args[0]) {
			writeChannelExpression(out, call.Args[0])
			out.WriteString(".len()")
			return
		}

		// len() returns the length of arrays, slices, maps, strings, or channels
		if isExpressionResultBare(call.Args[0]) {
			// Bare value (range var, index result, etc.) - access directly
			TranspileExpressionContext(out, call.Args[0], LValue)
			out.WriteString(".len()")
		} else {
			// The argument is wrapped, so we need to unwrap it first
			// Keep as usize - Rust's natural size type for collections
			out.WriteString("(*")
			// Use LValue context so identifiers don't unwrap themselves
			TranspileExpressionContext(out, call.Args[0], LValue)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).len()")
		}
	}
}

func transpileMake(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 1 {
		// Check if it's a channel type
		if chanType, ok := call.Args[0].(*ast.ChanType); ok {
			NeedGoChannel()
			elemType := goTypeToRustBase(chanType.Value)
			if len(call.Args) > 1 {
				// Buffered: make(chan T, n)
				out.WriteString("GoChannel::<")
				out.WriteString(elemType)
				out.WriteString(">::new_buffered(")
				TranspileExpression(out, call.Args[1])
				out.WriteString(" as usize)")
			} else {
				// Unbuffered: make(chan T)
				out.WriteString("GoChannel::<")
				out.WriteString(elemType)
				out.WriteString(">::new()")
			}
			return
		}
		// Check if it's a map type
		if mapType, ok := call.Args[0].(*ast.MapType); ok {
			WriteWrapperPrefix(out)
			TrackImport("BTreeMap")
			out.WriteString("BTreeMap::<")
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
						writeExpressionAsUsize(out, call.Args[2])
					} else {
						out.WriteString("0")
					}
					out.WriteString(")")
				} else if len(call.Args) >= 3 {
					// Vector with initial size AND capacity: make([]T, len, cap)
					out.WriteString("{ let mut v = Vec::with_capacity(")
					writeExpressionAsUsize(out, call.Args[2])
					out.WriteString("); v.resize(")
					writeExpressionAsUsize(out, call.Args[1])
					out.WriteString(", ")
					out.WriteString(elementType)
					out.WriteString("); v }")
				} else {
					// Vector with initial size only
					out.WriteString("vec![")
					out.WriteString(elementType)
					out.WriteString("; ")
					writeExpressionAsUsize(out, call.Args[1])
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
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.IsChannel(call.Args[0]) {
			writeChannelExpression(out, call.Args[0])
			out.WriteString(".capacity()")
			return
		}

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

func transpileCopy(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 2 {
		// Go: copy(dst, src) copies min(len(dst), len(src)) elements, returns count
		// Generate inline block that works with already-unwrapped Vec values
		typeInfo := GetTypeInfo()
		srcIsString := typeInfo != nil && typeInfo.IsString(call.Args[1])
		out.WriteString("{ let _src = (")
		TranspileExpression(out, call.Args[1])
		if srcIsString {
			out.WriteString(").as_bytes().to_vec(); let _n = std::cmp::min((")
		} else {
			out.WriteString(").clone(); let _n = std::cmp::min((")
		}
		TranspileExpression(out, call.Args[0])
		out.WriteString(").len(), _src.len()); for _i in 0.._n { ")
		// Destination needs mutable borrow for assignment
		if ident, ok := call.Args[0].(*ast.Ident); ok {
			out.WriteString("(*")
			out.WriteString(ident.Name)
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap())")
		} else {
			out.WriteString("(*")
			TranspileExpression(out, call.Args[0])
			out.WriteString(")")
		}
		out.WriteString("[_i] = _src[_i].clone(); } ")
		WriteWrapperPrefix(out)
		out.WriteString("_n as i32")
		WriteWrapperSuffix(out)
		out.WriteString(" }")
	}
}

func transpileNew(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		WriteWrapperPrefix(out)
		out.WriteString(goTypeToRustBase(call.Args[0]))
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
		out.WriteString(`fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String 
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
		out.WriteString(`fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String 
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

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
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

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
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
			out.WriteString(RustStringLiteral(lit.Value))
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

func transpileClose(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		writeChannelExpression(out, call.Args[0])
		out.WriteString(".close()")
	}
}

// stdlibSelectorMappings maps non-call stdlib selectors (constants, etc.) to Rust expressions.
// Used by TranspileExpression for SelectorExpr when the selector is on a stdlib package.
var stdlibSelectorMappings = map[string]string{
	"math.E":           "std::f64::consts::E",
	"math.Pi":          "std::f64::consts::PI",
	"os.Args":          "__go_os_args.clone()",
	"time.Hour":        "std::time::Duration::from_secs(3600)",
	"time.Minute":      "std::time::Duration::from_secs(60)",
	"time.Second":      "std::time::Duration::from_secs(1)",
	"time.Millisecond": "std::time::Duration::from_millis(1)",
	"time.Microsecond": "std::time::Duration::from_micros(1)",
	"time.Nanosecond":  "std::time::Duration::from_nanos(1)",
}

// GetStdlibSelectorMapping returns the Rust expression for a stdlib selector constant,
// or empty string if no mapping exists.
func GetStdlibSelectorMapping(pkgName, selName string) string {
	key := pkgName + "." + selName
	if rustExpr, ok := stdlibSelectorMappings[key]; ok {
		return rustExpr
	}
	return ""
}

func transpileTimeNow(out *strings.Builder, call *ast.CallExpr) {
	NeedGoTime()
	WriteWrapperPrefix(out)
	out.WriteString("GoTime::now()")
	out.WriteString(")))")
}

func transpileTimeUnix(out *strings.Builder, call *ast.CallExpr) {
	NeedGoTime()
	WriteWrapperPrefix(out)
	out.WriteString("GoTime::from_unix(")
	if len(call.Args) > 0 {
		writeUnwrappedForFormat(out, call.Args[0])
	} else {
		out.WriteString("0")
	}
	out.WriteString(" as i64, ")
	if len(call.Args) > 1 {
		writeUnwrappedForFormat(out, call.Args[1])
	} else {
		out.WriteString("0")
	}
	out.WriteString(" as i64)")
	out.WriteString(")))")
}

func transpileTimeAfter(out *strings.Builder, call *ast.CallExpr) {
	NeedGoAfter()
	TrackImport("time::Duration")
	// time.After returns a channel that fires after a duration
	NeedGoChannel()
	out.WriteString("go_channel_after(")
	if len(call.Args) > 0 {
		transpileDurationArg(out, call.Args[0])
	}
	out.WriteString(")")
}

func transpileTimeNewTicker(out *strings.Builder, call *ast.CallExpr) {
	NeedGoTicker()
	TrackImport("time::Duration")
	WriteWrapperPrefix(out)
	out.WriteString("go_new_ticker(")
	if len(call.Args) > 0 {
		transpileDurationArg(out, call.Args[0])
	}
	out.WriteString(")")
	out.WriteString(")))")
}

func transpileTimeNewTimer(out *strings.Builder, call *ast.CallExpr) {
	NeedGoTimer()
	TrackImport("time::Duration")
	WriteWrapperPrefix(out)
	out.WriteString("go_new_timer(")
	if len(call.Args) > 0 {
		transpileDurationArg(out, call.Args[0])
	}
	out.WriteString(")")
	out.WriteString(")))")
}

func transpileTimeTick(out *strings.Builder, call *ast.CallExpr) {
	NeedGoTick()
	TrackImport("time::Duration")
	out.WriteString("go_tick(")
	if len(call.Args) > 0 {
		transpileDurationArg(out, call.Args[0])
	}
	out.WriteString(")")
}

func transpileContextBackground(out *strings.Builder, call *ast.CallExpr) {
	NeedGoContext()
	WriteWrapperPrefix(out)
	out.WriteString("GoContext::background()")
	out.WriteString(")))")
}

func transpileContextWithTimeout(out *strings.Builder, call *ast.CallExpr) {
	NeedGoContext()
	// context.WithTimeout(ctx, duration) returns (ctx, cancel)
	out.WriteString("GoContext::with_timeout(")
	if len(call.Args) > 0 {
		writeContextParentArg(out, call.Args[0])
	}
	out.WriteString(", ")
	if len(call.Args) > 1 {
		transpileDurationArg(out, call.Args[1])
	}
	out.WriteString(")")
}

func transpileContextWithCancel(out *strings.Builder, call *ast.CallExpr) {
	NeedGoContext()
	out.WriteString("GoContext::with_cancel(")
	if len(call.Args) > 0 {
		writeContextParentArg(out, call.Args[0])
	}
	out.WriteString(")")
}

func transpileContextWithCancelCause(out *strings.Builder, call *ast.CallExpr) {
	NeedGoContext()
	out.WriteString("GoContext::with_cancel_cause(")
	if len(call.Args) > 0 {
		writeContextParentArg(out, call.Args[0])
	}
	out.WriteString(")")
}

func writeContextParentArg(out *strings.Builder, arg ast.Expr) {
	TranspileExpressionContext(out, arg, LValue)
	out.WriteString(".clone()")
}

// transpileDurationArg handles a Go duration argument, recognizing patterns like N * time.Unit
func transpileDurationArg(out *strings.Builder, arg ast.Expr) {
	if binOp, ok := arg.(*ast.BinaryExpr); ok && binOp.Op == token.MUL {
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
			out.WriteString("std::time::Duration::")
			switch unit {
			case "Hour":
				out.WriteString("from_secs(3600 * ")
				TranspileExpression(out, multiplier)
				out.WriteString(")")
			case "Minute":
				out.WriteString("from_secs(60 * ")
				TranspileExpression(out, multiplier)
				out.WriteString(")")
			case "Second":
				out.WriteString("from_secs(")
				TranspileExpression(out, multiplier)
				out.WriteString(")")
			case "Millisecond":
				out.WriteString("from_millis(")
				TranspileExpression(out, multiplier)
				out.WriteString(")")
			case "Microsecond":
				out.WriteString("from_micros(")
				TranspileExpression(out, multiplier)
				out.WriteString(")")
			case "Nanosecond":
				out.WriteString("from_nanos(")
				TranspileExpression(out, multiplier)
				out.WriteString(")")
			default:
				out.WriteString("from_millis(")
				TranspileExpression(out, multiplier)
				out.WriteString(")")
			}
			return
		}
	}
	// Fallback: treat as raw expression
	TranspileExpression(out, arg)
}
