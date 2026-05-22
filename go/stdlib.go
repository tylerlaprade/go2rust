package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"reflect"
	"slices"
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
		if handler, exists := builtinMappings[ident.Name]; exists && isBuiltinCallTarget(ident) {
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
		if x, ok := current.(*ast.SelectorExpr); ok {
			parts = append([]string{x.Sel.Name}, parts...)
			current = x.X
			continue
		}
		if x, ok := current.(*ast.Ident); ok {
			return resolveStdlibPackageName(x.Name) + "." + strings.Join(parts, "."), true
		}
		return "", false
	}
}

var stdlibMappings map[string]StdlibHandler
var builtinMappings map[string]StdlibHandler

func init() {
	stdlibMappings = map[string]StdlibHandler{
		"fmt.Println":              transpileFmtPrintln,
		"fmt.Printf":               transpileFmtPrintf,
		"fmt.Print":                transpileFmtPrint,
		"fmt.Sprintf":              transpileFmtSprintf,
		"fmt.Fprintln":             transpileFmtFprintln,
		"fmt.Fprintf":              transpileFmtFprintf,
		"fmt.Errorf":               transpileFmtErrorf,
		"strings.ToLower":          transpileStringsToLower,
		"strings.ToUpper":          transpileStringsToUpper,
		"strings.TrimSpace":        transpileStringsTrimSpace,
		"strings.Title":            transpileStringsTitle,
		"strings.Contains":         transpileStringsContains,
		"strings.Index":            transpileStringsIndex,
		"strings.LastIndex":        transpileStringsLastIndex,
		"strings.Count":            transpileStringsCount,
		"strings.Compare":          transpileStringsCompare,
		"strings.Cut":              transpileStringsCut,
		"strings.HasSuffix":        transpileStringsHasSuffix,
		"strings.HasPrefix":        transpileStringsHasPrefix,
		"strings.TrimSuffix":       transpileStringsTrimSuffix,
		"strings.TrimPrefix":       transpileStringsTrimPrefix,
		"strings.IndexAny":         transpileStringsIndexAny,
		"strings.Split":            transpileStringsSplit,
		"strings.Join":             transpileStringsJoin,
		"strings.Fields":           transpileStringsFields,
		"strings.Replace":          transpileStringsReplace,
		"strings.ReplaceAll":       transpileStringsReplaceAll,
		"strings.Repeat":           transpileStringsRepeat,
		"strings.EqualFold":        transpileStringsEqualFold,
		"strings.TrimLeft":         transpileStringsTrimLeft,
		"strings.TrimRight":        transpileStringsTrimRight,
		"strings.Trim":             transpileStringsTrim,
		"strconv.Itoa":             transpileStrconvItoa,
		"strconv.Atoi":             transpileStrconvAtoi,
		"strconv.FormatFloat":      transpileStrconvFormatFloat,
		"strconv.FormatInt":        transpileStrconvFormatInt,
		"strconv.Quote":            transpileStrconvQuote,
		"errors.New":               transpileErrorsNew,
		"sort.Strings":             transpileSortStrings,
		"sort.Ints":                transpileSortInts,
		"slices.Sort":              transpileSlicesSort,
		"slices.SortFunc":          transpileSlicesSortFunc,
		"slices.Contains":          transpileSlicesContains,
		"slices.Clone":             transpileSlicesClone,
		"slices.Clip":              transpileSlicesClip,
		"time.Sleep":               transpileTimeSleep,
		"time.Now":                 transpileTimeNow,
		"time.Unix":                transpileTimeUnix,
		"time.After":               transpileTimeAfter,
		"time.NewTicker":           transpileTimeNewTicker,
		"time.NewTimer":            transpileTimeNewTimer,
		"time.Tick":                transpileTimeTick,
		"context.Background":       transpileContextBackground,
		"context.WithTimeout":      transpileContextWithTimeout,
		"context.WithCancel":       transpileContextWithCancel,
		"context.WithCancelCause":  transpileContextWithCancelCause,
		"os.Create":                transpileOsCreate,
		"os.Remove":                transpileOsRemove,
		"reflect.TypeOf":           transpileReflectTypeOf,
		"sync/atomic.AddInt64":     transpileAtomicAddInt64,
		"sync/atomic.LoadInt64":    transpileAtomicLoadInt64,
		"sync/atomic.LoadPointer":  transpileAtomicLoadPointer,
		"sync/atomic.StorePointer": transpileAtomicStorePointer,
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
		"min":     transpileMin,
		"max":     transpileMax,
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
			transpilePrintArgString(out, arg)
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
			transpilePrintArgString(out, arg)
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
			transpilePrintArgString(out, arg)
		}
	}

	out.WriteString(")")
}

func transpilePrintArgString(out *strings.Builder, arg ast.Expr) {
	out.WriteString("format!(\"{}\", ")
	transpilePrintArg(out, arg)
	out.WriteString(")")
}

func writeFormatAnyPrintArg(out *strings.Builder, arg ast.Expr) {
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
		out.WriteString("(")
		TranspileExpression(out, arg)
		out.WriteString(")")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap().as_ref()")
	}
	out.WriteString(")")
}

// Helper function to unwrap arguments for print statements
func transpilePrintArg(out *strings.Builder, arg ast.Expr) {
	if isEmptyInterfaceValueExpr(arg) {
		writeFormatAnyPrintArg(out, arg)
		return
	}

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
				writeFormatAnyPrintArg(out, arg)
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
					} else if _, ok := arg.(*ast.SelectorExpr); ok {
						out.WriteString("(*")
						TranspileExpressionContext(out, arg, LValue)
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
		} else if typeInfo.IsSlice(arg) || typeInfo.IsArray(arg) {
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
			elemType = typeInfo.GetArrayOrSliceElemType(arg)
			if isPointerType(elemType) {
				if typeHasGoStringMethod(elemType) {
					NeedFormatSliceWrappedStringer()
					writeFormatSliceCall(out, arg, "format_slice_wrapped_stringer", "format_slice_wrapped_stringer_values")
				} else {
					NeedFormatSliceWrappedValues()
					writeFormatSliceCall(out, arg, "format_slice_wrapped", "format_slice_wrapped_values")
				}
				return
			}
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
			} else if formatSliceArgumentIsBareValue(arg) {
				out.WriteString("format_slice_values(&")
				TranspileExpression(out, arg)
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
			if writeCurrentReceiverValueClone(out, ident) {
				return
			}
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
		if _, ok := arg.(*ast.BasicLit); ok {
			TranspileExpression(out, arg)
			return
		}
		if writeTrackedRangeSlicePrintArg(out, arg) {
			return
		}
		if writeNoTypeInfoPrintArg(out, arg) {
			return
		}
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
				typeInfo := GetTypeInfo()
				isEmptyInterface := false
				if typeInfo != nil {
					if typ := typeInfo.GetType(ident); typ != nil {
						if intf, ok := typ.Underlying().(*types.Interface); ok && intf.NumMethods() == 0 {
							isEmptyInterface = true
						}
					}
				}
				if isEmptyInterface {
					out.WriteString("go_type_name(&**")
					out.WriteString(ident.Name)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())")
				} else {
					out.WriteString("go_type_name(")
					out.WriteString(ident.Name)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())")
				}
			}
		} else {
			out.WriteString("go_type_name(&")
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
	writeFmtMacroCall(out, "print!", call, 0, TranspileExpression)
}

func writeFmtMacroCall(out *strings.Builder, macro string, call *ast.CallExpr, formatArgIndex int, dynamicFormatArg func(*strings.Builder, ast.Expr)) {
	out.WriteString(macro)
	out.WriteString("(")

	var skipIndices []int
	var charIndices []int
	var typeNameIndices []int
	var unicodeIndices []int
	hexFormats := make(map[int]string)
	if len(call.Args) > formatArgIndex {
		if lit, ok := call.Args[formatArgIndex].(*ast.BasicLit); ok && lit.Kind == token.STRING {
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
			dynamicFormatArg(out, call.Args[formatArgIndex])
			out.WriteString(")")
			return
		}

		// Rest of the arguments, skipping those no longer needed
		for i := formatArgIndex + 1; i < len(call.Args); i++ {
			// Check if this argument index should be skipped (0-based in skipIndices)
			shouldSkip := false
			for _, skipIdx := range skipIndices {
				if skipIdx == i-formatArgIndex-1 {
					shouldSkip = true
					break
				}
			}
			if !shouldSkip {
				out.WriteString(", ")
				transpileFormatArg(out, call.Args[i], i-formatArgIndex-1, charIndices, typeNameIndices, unicodeIndices, hexFormats)
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
				transpilePrintArgString(out, arg)
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
				transpilePrintArgString(out, arg)
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
	if fmtFprintfTargetIsStringsBuilder(call.Args[0]) {
		if isStringsBuilderReceiverBare(call.Args[0]) {
			writeStringsBuilderRawReceiver(out, call.Args[0])
		} else {
			out.WriteString("(*")
			writeStringsBuilderReceiverHandle(out, call.Args[0])
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap())")
		}
		out.WriteString(".push_str(&")
		writeFmtMacroCall(out, "format!", call, 1, writeOwnedStringStdlibArg)
		out.WriteString(")")
		return
	}
	// Check if writing to stderr
	macro := "print!"
	if isOsStderr(call.Args[0]) {
		macro = "eprint!"
	}
	writeFmtMacroCall(out, macro, call, 1, TranspileExpression)
}

func fmtFprintfTargetIsStringsBuilder(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	return typeInfo != nil && isStringsBuilderReceiverType(typeInfo.GetType(expr))
}

func formatSliceArgumentIsBareValue(arg ast.Expr) bool {
	if isExpressionResultBare(arg) {
		return true
	}
	if _, ok := arg.(*ast.SelectorExpr); ok {
		return !selectorRValueReturnsWrappedHandle(arg)
	}
	return false
}

func writeTrackedRangeSlicePrintArg(out *strings.Builder, arg ast.Expr) bool {
	ident, ok := arg.(*ast.Ident)
	if !ok {
		return false
	}
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar || !strings.HasPrefix(varType, "&Vec<") {
		return false
	}
	NeedFormatSlice()
	TrackImport("Display")
	out.WriteString("format_slice_values(")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(")")
	return true
}

func writeNoTypeInfoPrintArg(out *strings.Builder, arg ast.Expr) bool {
	if call, ok := arg.(*ast.CallExpr); ok && callReturnsSliceBySyntax(call) {
		NeedFormatSlice()
		TrackImport("Display")
		out.WriteString("format_slice(&")
		TranspileExpression(out, call)
		out.WriteString(")")
		return true
	}
	if sel, ok := arg.(*ast.SelectorExpr); ok {
		if writeNoTypeInfoSelectorStdlibInterfacePrintArg(out, sel) {
			return true
		}
		if writeNoTypeInfoSelectorCollectionPrintArg(out, sel) {
			return true
		}
	}
	ident, ok := arg.(*ast.Ident)
	if !ok {
		return false
	}
	name := RustIdentForUse(ident)
	if kind, ok := localCollectionKind(ident); ok {
		switch kind {
		case "slice":
			NeedFormatSlice()
			TrackImport("Display")
			out.WriteString("format_slice(&")
			out.WriteString(name)
			out.WriteString(")")
			return true
		case "map":
			NeedFormatMap()
			TrackImport("Display")
			TrackImport("Ord")
			out.WriteString("format_map(&")
			out.WriteString(name)
			out.WriteString(")")
			return true
		}
	}
	if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
		if varType == "ref_value" || strings.HasPrefix(varType, "&") {
			return writeOwnedRangeValue(out, ident)
		}
		if isWrappedRangeVarType(varType) {
			out.WriteString("(*")
			out.WriteString(name)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			return true
		}
		out.WriteString(name)
		return true
	}
	if ident.Name == "_" || ident.Name == "nil" || ident.Name == "true" || ident.Name == "false" || isConstIdent(ident) || isLocalConstantIdent(ident) || isVarBare(ident.Name) {
		return false
	}
	out.WriteString("(*")
	out.WriteString(name)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
	return true
}

func writeNoTypeInfoSelectorStdlibInterfacePrintArg(out *strings.Builder, sel *ast.SelectorExpr) bool {
	fieldType, ok := syntaxSelectorFieldType(sel)
	if !ok {
		return false
	}
	if _, ok := externalStdlibInterfaceTypeExpr(fieldType); !ok {
		return false
	}
	out.WriteString("format!(\"{}\", (*")
	TranspileExpressionContext(out, sel, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()))")
	return true
}

func writeNoTypeInfoSelectorCollectionPrintArg(out *strings.Builder, sel *ast.SelectorExpr) bool {
	fieldType, ok := syntaxSelectorFieldType(sel)
	if !ok {
		return false
	}
	arrayType, ok := fieldType.(*ast.ArrayType)
	if !ok {
		return false
	}
	NeedFormatSlice()
	TrackImport("Display")
	helper := "format_slice"
	if _, ok := arrayType.Elt.(*ast.StarExpr); ok {
		NeedFormatSliceWrappedValues()
		helper = "format_slice_wrapped"
	}
	out.WriteString(helper)
	out.WriteString("(&")
	TranspileExpressionContext(out, sel, LValue)
	out.WriteString(")")
	return true
}

func syntaxSelectorFieldType(sel *ast.SelectorExpr) (ast.Expr, bool) {
	structType, ok := syntaxStructTypeNameForSelectorBase(sel.X)
	if !ok {
		if ident, identOK := sel.X.(*ast.Ident); identOK && currentReceiver != "" && ident.Name == currentReceiver {
			structType = currentReceiverType
			ok = true
		}
	}
	if !ok {
		return nil, false
	}
	fieldInfo := resolveFieldAccess(structType, sel.Sel.Name)
	if !fieldInfo.Found {
		return nil, false
	}
	fieldOwner := structType
	if fieldInfo.IsPromoted && len(fieldInfo.EmbeddedPath) > 0 {
		fieldOwner = fieldInfo.EmbeddedPath[len(fieldInfo.EmbeddedPath)-1]
	}
	structDef := structDefs[fieldOwner]
	if structDef == nil {
		return nil, false
	}
	fieldType, ok := structDef.FieldTypes[sel.Sel.Name]
	return fieldType, ok
}

func callReturnsSliceBySyntax(call *ast.CallExpr) bool {
	resultType := callSingleReturnTypeExpr(call)
	arrayType, ok := resultType.(*ast.ArrayType)
	return ok && arrayType.Len == nil
}

func writeFormatSliceCall(out *strings.Builder, arg ast.Expr, wrappedHelper string, valuesHelper string) {
	if ident, ok := arg.(*ast.Ident); ok {
		if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			out.WriteString(valuesHelper)
			out.WriteString("(")
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(")")
			return
		}
		out.WriteString(wrappedHelper)
		out.WriteString("(&")
		out.WriteString(RustIdentForUse(ident))
		out.WriteString(")")
		return
	}
	if formatSliceArgumentIsBareValue(arg) {
		out.WriteString(valuesHelper)
		out.WriteString("(&")
		TranspileExpression(out, arg)
		out.WriteString(")")
		return
	}
	out.WriteString(wrappedHelper)
	out.WriteString("(&")
	TranspileExpression(out, arg)
	out.WriteString(")")
}

func transpileFmtSprintf(out *strings.Builder, call *ast.CallExpr) {
	// fmt.Sprintf returns a string, which needs to be wrapped
	WriteWrapperPrefix(out)
	out.WriteString("format!")
	out.WriteString("(")

	var skipIndices []int
	var charIndices []int
	var typeNameIndices []int
	var unicodeIndices []int
	hexFormats := make(map[int]string)
	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust and get skip indices
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
				transpileFormatArg(out, call.Args[i], i-1, charIndices, typeNameIndices, unicodeIndices, hexFormats)
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

	var skipIndices []int
	var charIndices []int
	var typeNameIndices []int
	var unicodeIndices []int
	hexFormats := make(map[int]string)
	if len(call.Args) > 0 {
		// First arg is the format string
		literalFormat := false
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust
			format, skips, chars, typeNames, unicodes, hexes := convertFormatStringWithSkips(lit.Value)
			skipIndices = skips
			charIndices = chars
			typeNameIndices = typeNames
			unicodeIndices = unicodes
			hexFormats = hexes
			out.WriteString(format)
			literalFormat = true
		} else {
			out.WriteString("\"{}\", ")
			writeOwnedStringStdlibArg(out, call.Args[0])
		}

		// Rest of the arguments
		if literalFormat {
			for i := 1; i < len(call.Args); i++ {
				shouldSkip := false
				for _, skipIdx := range skipIndices {
					if skipIdx == i-1 {
						shouldSkip = true
						break
					}
				}
				if shouldSkip {
					continue
				}
				out.WriteString(", ")
				transpileFormatArg(out, call.Args[i], i-1, charIndices, typeNameIndices, unicodeIndices, hexFormats)
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
			writeOwnedStringStdlibArg(out, call.Args[0])
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

func transpileAtomicLoadPointer(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 1 {
		out.WriteString("/* ERROR: atomic.LoadPointer requires pointer */ unimplemented!()")
		return
	}

	trackWrapperImports()
	out.WriteString("{ let __target = ")
	writeAtomicTarget(out, call.Args[0])
	if NeedsConcurrentWrapper() {
		out.WriteString("; let __guard = __target.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }")
	} else {
		out.WriteString("; let __guard = __target.borrow(); Rc::new(RefCell::new((*__guard).clone())) }")
	}
}

func transpileAtomicStorePointer(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		out.WriteString("/* ERROR: atomic.StorePointer requires pointer and value */ unimplemented!()")
		return
	}

	out.WriteString("{ let __target = ")
	writeAtomicTarget(out, call.Args[0])
	out.WriteString("; let __stored = ")
	writeAtomicStorePointerValue(out, call.Args[1])
	if NeedsConcurrentWrapper() {
		out.WriteString("; *__target.lock().unwrap() = __stored; }")
	} else {
		out.WriteString("; *__target.borrow_mut() = __stored; }")
	}
}

func writeAtomicStorePointerValue(out *strings.Builder, value ast.Expr) {
	if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("None")
		return
	}

	out.WriteString("{ let __value = ")
	TranspileExpressionContext(out, value, LValue)
	if NeedsConcurrentWrapper() {
		out.WriteString("; let __guard = __value.lock().unwrap(); (*__guard).clone() }")
	} else {
		out.WriteString("; let __guard = __value.borrow(); (*__guard).clone() }")
	}
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

	typ := typeInfo.GetType(call.Args[0])
	if typ == nil {
		out.WriteString("/* ERROR: Type information required for reflect.TypeOf */ unimplemented!()")
		return
	}

	st := typeInfo.GetStructType(call.Args[0])
	NeedReflect()
	WriteWrapperPrefix(out)
	out.WriteString("GoReflectType { name: ")
	writeReflectString(out, reflectTypeName(typ))
	out.WriteString(", fields: ")
	WriteWrapperPrefix(out)
	out.WriteString("vec![")
	if st != nil {
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
	}
	out.WriteString("]")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
	WriteWrapperSuffix(out)
}

func reflectTypeName(typ types.Type) string {
	if typ == nil {
		return ""
	}
	return types.TypeString(typ, func(pkg *types.Package) string {
		if pkg == nil {
			return ""
		}
		return pkg.Name()
	})
}

func writeOwnedStringStdlibArg(out *strings.Builder, arg ast.Expr) {
	if lit, ok := arg.(*ast.BasicLit); ok && lit.Kind == token.STRING {
		TranspileExpression(out, lit)
		return
	}
	if isConstantExpression(arg) {
		TranspileExpression(out, arg)
		return
	}
	if writeRangeStringValue(out, arg) {
		return
	}
	if _, ok := arg.(*ast.BinaryExpr); ok {
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsString(arg) {
			TranspileExpression(out, arg)
			return
		}
	}
	if _, ok := arg.(*ast.IndexExpr); ok {
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsString(arg) {
			TranspileExpression(out, arg)
			return
		}
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
	out.WriteString(").as_ref().cloned().unwrap_or_default()")
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
		out.WriteString("{ let __s = ")
		writeOwnedStringStdlibArg(out, call.Args[0])
		out.WriteString("; __s.to_uppercase() }")
		WriteWrapperSuffix(out)
	}
}

func transpileStringsToLower(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		WriteWrapperPrefix(out)
		out.WriteString("{ let __s = ")
		writeOwnedStringStdlibArg(out, call.Args[0])
		out.WriteString("; __s.to_lowercase() }")
		WriteWrapperSuffix(out)
	}
}

func transpileStringsTrimSpace(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		WriteWrapperPrefix(out)
		out.WriteString("{ let __s = ")
		writeOwnedStringStdlibArg(out, call.Args[0])
		out.WriteString("; __s.trim().to_string() }")
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

func transpileStringsTrimSuffix(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __suffix = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; __s.strip_suffix(&__suffix).unwrap_or(&__s).to_string() }")
	WriteWrapperSuffix(out)
}

func transpileStringsTrimPrefix(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __s = ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString("; let __prefix = ")
	writeOwnedStringStdlibArg(out, call.Args[1])
	out.WriteString("; __s.strip_prefix(&__prefix).unwrap_or(&__s).to_string() }")
	WriteWrapperSuffix(out)
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
	writeNumericConversionValue(out, call.Args[1])
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
		transpileNilSafeSort(out, call.Args[0])
	}
}

func transpileSortInts(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		transpileNilSafeSort(out, call.Args[0])
	}
}

func transpileSlicesSort(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		transpileNilSafeSort(out, call.Args[0])
	}
}

func transpileNilSafeSort(out *strings.Builder, arg ast.Expr) {
	out.WriteString("{ let mut __sort_guard = ")
	TranspileExpressionContext(out, arg, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString("; if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } }")
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

	out.WriteString("{ let mut __cmp_guard = __cmp_holder")
	WriteBorrowMethod(out, true)
	out.WriteString("; let __cmp_fn = __cmp_guard.as_mut().unwrap(); (*__cmp_fn)(")
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

	out.WriteString("{ ")
	if usesFunctionValue {
		out.WriteString("let __cmp_holder = ")
		TranspileExpression(out, call.Args[1])
		out.WriteString("; ")
	}
	out.WriteString("let mut __sort_guard = ")
	TranspileExpressionContext(out, call.Args[0], LValue)
	WriteBorrowMethod(out, true)
	out.WriteString("; if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort_by(|__a, __b| { let __cmp = ")
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
	out.WriteString(".as_ref().unwrap()).cmp(&0); __ord }); } }")
}

func transpileSlicesContains(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if _, ok := localInterfaceSliceElemName(typeInfo.GetType(call.Args[0])); ok {
			WriteWrapperPrefix(out)
			out.WriteString("{ let __slice_holder = ")
			TranspileExpressionContext(out, call.Args[0], LValue)
			out.WriteString(".clone(); let __slice_guard = __slice_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __slice = __slice_guard.as_ref().unwrap(); ")
			if isBareLocalInterfaceValue(call.Args[1]) {
				out.WriteString("let __value = ")
				TranspileExpression(out, call.Args[1])
				out.WriteString("; __slice.iter().any(|__item| __item.__go_eq(__value)) }")
			} else {
				out.WriteString("let __value_holder = ")
				TranspileExpressionContext(out, call.Args[1], LValue)
				out.WriteString(".clone(); let __value_guard = __value_holder")
				WriteBorrowMethod(out, false)
				out.WriteString("; let __value = __value_guard.as_ref().unwrap().as_ref(); __slice.iter().any(|__item| __item.__go_eq(__value)) }")
			}
			WriteWrapperSuffix(out)
			return
		}
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

func transpileSlicesClone(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 1 {
		out.WriteString("/* ERROR: slices.Clone expects 1 argument */")
		return
	}
	WriteWrapperPrefix(out)
	writeUnwrappedSliceClone(out, call.Args[0])
	WriteWrapperSuffix(out)
}

func transpileSlicesClip(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 1 {
		out.WriteString("/* ERROR: slices.Clip expects 1 argument */")
		return
	}
	WriteWrapperPrefix(out)
	writeUnwrappedSliceClone(out, call.Args[0])
	WriteWrapperSuffix(out)
}

func transpileStrconvItoa(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		WriteWrapperPrefix(out)
		writeNumericConversionValue(out, call.Args[0])
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

func transpileStrconvQuote(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 1 {
		out.WriteString("/* ERROR: strconv.Quote requires string */ unimplemented!()")
		return
	}

	WriteWrapperPrefix(out)
	out.WriteString("format!(\"{:?}\", ")
	writeOwnedStringStdlibArg(out, call.Args[0])
	out.WriteString(")")
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
	jsonName  string
	rustName  string
	typ       types.Type
	kind      jsonMarshalFieldKind
	basicKind types.BasicKind
	omitEmpty bool
	named     bool
}

type jsonMarshalFieldKind int

const (
	jsonMarshalBasicField jsonMarshalFieldKind = iota
	jsonMarshalStringMapField
	jsonMarshalStringSliceField
	jsonMarshalByteSliceMapField
)

func jsonFieldName(goName, tag string) (string, bool, bool) {
	jsonTag := reflect.StructTag(tag).Get("json")
	if jsonTag == "-" {
		return "", false, false
	}
	omitEmpty := false
	if idx := strings.Index(jsonTag, ","); idx >= 0 {
		options := strings.Split(jsonTag[idx+1:], ",")
		jsonTag = jsonTag[:idx]
		for _, option := range options {
			if option == "omitempty" {
				omitEmpty = true
			}
		}
	}
	if jsonTag == "" {
		return goName, true, omitEmpty
	}
	return jsonTag, true, omitEmpty
}

func isJsonStringMapType(typ types.Type) bool {
	m, ok := types.Unalias(typ).Underlying().(*types.Map)
	if !ok {
		return false
	}
	key, ok := types.Unalias(m.Key()).Underlying().(*types.Basic)
	if !ok || key.Kind() != types.String {
		return false
	}
	elem, ok := types.Unalias(m.Elem()).Underlying().(*types.Basic)
	return ok && elem.Kind() == types.String
}

func isJsonStringSliceType(typ types.Type) bool {
	slice, ok := types.Unalias(typ).Underlying().(*types.Slice)
	if !ok {
		return false
	}
	elem, ok := types.Unalias(slice.Elem()).Underlying().(*types.Basic)
	return ok && elem.Kind() == types.String
}

func isJsonByteSliceType(typ types.Type) bool {
	slice, ok := types.Unalias(typ).Underlying().(*types.Slice)
	if !ok {
		return false
	}
	elem, ok := types.Unalias(slice.Elem()).Underlying().(*types.Basic)
	return ok && elem.Kind() == types.Uint8
}

func isJsonStringByteSliceMapType(typ types.Type) bool {
	m, ok := types.Unalias(typ).Underlying().(*types.Map)
	if !ok {
		return false
	}
	key, ok := types.Unalias(m.Key()).Underlying().(*types.Basic)
	return ok && key.Kind() == types.String && isJsonByteSliceType(m.Elem())
}

func jsonMarshalStructFields(st *types.Struct) ([]jsonMarshalField, bool) {
	fields := []jsonMarshalField{}
	for i := 0; i < st.NumFields(); i++ {
		field := st.Field(i)
		if !field.Exported() {
			continue
		}
		jsonName, include, omitEmpty := jsonFieldName(field.Name(), st.Tag(i))
		if !include {
			continue
		}
		_, named := types.Unalias(field.Type()).(*types.Named)
		basic, ok := types.Unalias(field.Type()).Underlying().(*types.Basic)
		if ok {
			switch basic.Kind() {
			case types.Bool,
				types.Int, types.Int8, types.Int16, types.Int32, types.Int64,
				types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64, types.Uintptr,
				types.Float32, types.Float64,
				types.String:
				fields = append(fields, jsonMarshalField{
					jsonName:  jsonName,
					rustName:  ToSnakeCase(field.Name()),
					typ:       field.Type(),
					kind:      jsonMarshalBasicField,
					basicKind: basic.Kind(),
					omitEmpty: omitEmpty,
					named:     named,
				})
				continue
			default:
				return nil, false
			}
		}
		if isJsonStringMapType(field.Type()) {
			fields = append(fields, jsonMarshalField{
				jsonName:  jsonName,
				rustName:  ToSnakeCase(field.Name()),
				typ:       field.Type(),
				kind:      jsonMarshalStringMapField,
				omitEmpty: omitEmpty,
			})
			continue
		}
		if isJsonStringSliceType(field.Type()) {
			fields = append(fields, jsonMarshalField{
				jsonName:  jsonName,
				rustName:  ToSnakeCase(field.Name()),
				typ:       field.Type(),
				kind:      jsonMarshalStringSliceField,
				omitEmpty: omitEmpty,
			})
			continue
		}
		if isJsonStringByteSliceMapType(field.Type()) {
			fields = append(fields, jsonMarshalField{
				jsonName:  jsonName,
				rustName:  ToSnakeCase(field.Name()),
				typ:       field.Type(),
				kind:      jsonMarshalByteSliceMapField,
				omitEmpty: omitEmpty,
			})
			continue
		}
		return nil, false
	}
	return fields, true
}

func jsonMarshalBasicKindFromSyntax(expr ast.Expr) (types.BasicKind, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return types.Invalid, false
	}
	switch ident.Name {
	case "bool":
		return types.Bool, true
	case "int":
		return types.Int, true
	case "int8":
		return types.Int8, true
	case "int16":
		return types.Int16, true
	case "int32", "rune":
		return types.Int32, true
	case "int64":
		return types.Int64, true
	case "uint":
		return types.Uint, true
	case "uint8", "byte":
		return types.Uint8, true
	case "uint16":
		return types.Uint16, true
	case "uint32":
		return types.Uint32, true
	case "uint64":
		return types.Uint64, true
	case "uintptr":
		return types.Uintptr, true
	case "float32":
		return types.Float32, true
	case "float64":
		return types.Float64, true
	case "string":
		return types.String, true
	default:
		return types.Invalid, false
	}
}

func isJsonStringMapTypeExpr(expr ast.Expr) bool {
	mapType, ok := expr.(*ast.MapType)
	if !ok {
		return false
	}
	key, keyOK := mapType.Key.(*ast.Ident)
	value, valueOK := mapType.Value.(*ast.Ident)
	return keyOK && valueOK && key.Name == "string" && value.Name == "string"
}

func isJsonStringSliceTypeExpr(expr ast.Expr) bool {
	sliceType, ok := expr.(*ast.ArrayType)
	if !ok || sliceType.Len != nil {
		return false
	}
	elem, ok := sliceType.Elt.(*ast.Ident)
	return ok && elem.Name == "string"
}

func isJsonByteSliceTypeExpr(expr ast.Expr) bool {
	sliceType, ok := expr.(*ast.ArrayType)
	if !ok || sliceType.Len != nil {
		return false
	}
	elem, ok := sliceType.Elt.(*ast.Ident)
	return ok && (elem.Name == "byte" || elem.Name == "uint8")
}

func isJsonStringByteSliceMapTypeExpr(expr ast.Expr) bool {
	mapType, ok := expr.(*ast.MapType)
	if !ok {
		return false
	}
	key, ok := mapType.Key.(*ast.Ident)
	return ok && key.Name == "string" && isJsonByteSliceTypeExpr(mapType.Value)
}

func jsonStructTagFromSyntax(field *ast.Field) string {
	if field == nil || field.Tag == nil {
		return ""
	}
	tag, err := strconv.Unquote(field.Tag.Value)
	if err != nil {
		return ""
	}
	return tag
}

func isExportedGoName(name string) bool {
	for _, r := range name {
		return isUpper(r)
	}
	return false
}

func jsonMarshalStructFieldsFromSyntax(st *ast.StructType) ([]jsonMarshalField, bool) {
	if st == nil || st.Fields == nil {
		return nil, false
	}
	fields := []jsonMarshalField{}
	for _, field := range st.Fields.List {
		if len(field.Names) == 0 {
			return nil, false
		}
		for _, name := range field.Names {
			if name == nil || !isExportedGoName(name.Name) {
				continue
			}
			jsonName, include, omitEmpty := jsonFieldName(name.Name, jsonStructTagFromSyntax(field))
			if !include {
				continue
			}
			if basicKind, ok := jsonMarshalBasicKindFromSyntax(field.Type); ok {
				fields = append(fields, jsonMarshalField{
					jsonName:  jsonName,
					rustName:  ToSnakeCase(name.Name),
					kind:      jsonMarshalBasicField,
					basicKind: basicKind,
					omitEmpty: omitEmpty,
				})
				continue
			}
			if isJsonStringMapTypeExpr(field.Type) {
				fields = append(fields, jsonMarshalField{
					jsonName:  jsonName,
					rustName:  ToSnakeCase(name.Name),
					kind:      jsonMarshalStringMapField,
					omitEmpty: omitEmpty,
				})
				continue
			}
			if isJsonStringSliceTypeExpr(field.Type) {
				fields = append(fields, jsonMarshalField{
					jsonName:  jsonName,
					rustName:  ToSnakeCase(name.Name),
					kind:      jsonMarshalStringSliceField,
					omitEmpty: omitEmpty,
				})
				continue
			}
			if isJsonStringByteSliceMapTypeExpr(field.Type) {
				fields = append(fields, jsonMarshalField{
					jsonName:  jsonName,
					rustName:  ToSnakeCase(name.Name),
					kind:      jsonMarshalByteSliceMapField,
					omitEmpty: omitEmpty,
				})
				continue
			}
			return nil, false
		}
	}
	return fields, true
}

func jsonMarshalStructTypeFromSyntaxType(expr ast.Expr) (*ast.StructType, bool) {
	switch t := expr.(type) {
	case *ast.Ident:
		if def, ok := structDefs[t.Name]; ok && def != nil && def.ASTType != nil {
			return def.ASTType, true
		}
	case *ast.StructType:
		return t, true
	}
	return nil, false
}

func jsonMarshalStructTypeFromSyntaxArg(arg ast.Expr) (*ast.StructType, bool) {
	switch expr := arg.(type) {
	case *ast.CompositeLit:
		return jsonMarshalStructTypeFromSyntaxType(expr.Type)
	case *ast.UnaryExpr:
		if expr.Op == token.AND {
			if lit, ok := expr.X.(*ast.CompositeLit); ok {
				return jsonMarshalStructTypeFromSyntaxType(lit.Type)
			}
		}
	case *ast.Ident:
		info := lookupVarInfo(expr.Name)
		if info == nil || info.RustType == "" {
			return nil, false
		}
		rustType := strings.TrimPrefix(info.RustType, "&")
		names := make([]string, 0, len(structDefs))
		for name := range structDefs {
			names = append(names, name)
		}
		slices.Sort(names)
		for _, name := range names {
			if rustType == RustTypeNameForUse(name) {
				if def := structDefs[name]; def != nil && def.ASTType != nil {
					return def.ASTType, true
				}
			}
		}
	}
	return nil, false
}

func escapeRustFormatLiteral(s string) string {
	s = strings.ReplaceAll(s, "{", "{{")
	return strings.ReplaceAll(s, "}", "}}")
}

func writeJsonMarshalValueBinding(out *strings.Builder, arg ast.Expr) {
	if _, ok := arg.(*ast.CompositeLit); ok {
		TranspileExpression(out, arg)
		return
	}
	if isExpressionResultBare(arg) {
		TranspileExpression(out, arg)
		return
	}
	out.WriteString("(*")
	TranspileExpressionContext(out, arg, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
}

func writeJsonFieldBorrow(out *strings.Builder, field jsonMarshalField) {
	out.WriteString("__json_value.")
	out.WriteString(field.rustName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref()")
}

func writeJsonFieldValueRef(out *strings.Builder, field jsonMarshalField) {
	writeJsonFieldBorrow(out, field)
	out.WriteString(".unwrap()")
	if field.named {
		out.WriteString(".0")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
	}
}

func writeJsonNamedClosureValueRef(out *strings.Builder) {
	out.WriteString("__v.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
}

func writeJsonBasicEmptyCondition(out *strings.Builder, field jsonMarshalField) {
	out.WriteString("!(")
	writeJsonFieldBorrow(out, field)
	out.WriteString(".map(|__v| ")
	switch field.basicKind {
	case types.String:
		if field.named {
			writeJsonNamedClosureValueRef(out)
			out.WriteString(".is_empty()")
		} else {
			out.WriteString("__v.is_empty()")
		}
	case types.Bool:
		out.WriteString("!*")
		if field.named {
			writeJsonNamedClosureValueRef(out)
		} else {
			out.WriteString("__v")
		}
	case types.Float32, types.Float64:
		out.WriteString("*")
		if field.named {
			writeJsonNamedClosureValueRef(out)
		} else {
			out.WriteString("__v")
		}
		out.WriteString(" == 0.0")
	default:
		out.WriteString("*")
		if field.named {
			writeJsonNamedClosureValueRef(out)
		} else {
			out.WriteString("__v")
		}
		out.WriteString(" == 0")
	}
	out.WriteString(").unwrap_or(true))")
}

func writeJsonBasicFieldPush(out *strings.Builder, field jsonMarshalField) {
	if field.omitEmpty {
		out.WriteString("if ")
		writeJsonBasicEmptyCondition(out, field)
		out.WriteString(" { ")
	}
	out.WriteString("__json_fields.push(format!(")
	if field.basicKind == types.String {
		out.WriteString(strconv.Quote(strconv.Quote(field.jsonName) + ":\"{}\""))
		out.WriteString(", go_json_escape(")
		writeJsonFieldValueRef(out, field)
		out.WriteString(")")
	} else {
		out.WriteString(strconv.Quote(strconv.Quote(field.jsonName) + ":{}"))
		out.WriteString(", *")
		writeJsonFieldValueRef(out, field)
	}
	out.WriteString("));")
	if field.omitEmpty {
		out.WriteString(" }")
	}
}

func writeJsonStringMapFieldPush(out *strings.Builder, field jsonMarshalField) {
	out.WriteString("{ let __map_guard = __json_value.")
	out.WriteString(field.rustName)
	WriteBorrowMethod(out, false)
	out.WriteString("; ")
	if field.omitEmpty {
		out.WriteString("if let Some(__map) = __map_guard.as_ref() { if !__map.is_empty() { ")
	} else {
		out.WriteString("if let Some(__map) = __map_guard.as_ref() { ")
	}
	out.WriteString("let __map_entries = __map.iter().map(|(__k, __v)| { let __v_guard = __v")
	WriteBorrowMethod(out, false)
	out.WriteString("; format!(\"\\\"{}\\\":\\\"{}\\\"\", go_json_escape(__k), go_json_escape(__v_guard.as_ref().unwrap())) }).collect::<Vec<_>>().join(\",\"); ")
	out.WriteString("__json_fields.push(format!(")
	out.WriteString(strconv.Quote(strconv.Quote(field.jsonName) + ":{{{}}}"))
	out.WriteString(", __map_entries));")
	if field.omitEmpty {
		out.WriteString(" } }")
	} else {
		out.WriteString(" } else { __json_fields.push(")
		out.WriteString(strconv.Quote(strconv.Quote(field.jsonName) + ":null"))
		out.WriteString(".to_string()); }")
	}
	out.WriteString(" }")
}

func writeJsonStringSliceFieldPush(out *strings.Builder, field jsonMarshalField) {
	out.WriteString("{ let __slice_guard = __json_value.")
	out.WriteString(field.rustName)
	WriteBorrowMethod(out, false)
	out.WriteString("; ")
	if field.omitEmpty {
		out.WriteString("if let Some(__slice) = __slice_guard.as_ref() { if !__slice.is_empty() { ")
	} else {
		out.WriteString("if let Some(__slice) = __slice_guard.as_ref() { ")
	}
	out.WriteString("let __slice_entries = __slice.iter().map(|__v| format!(\"\\\"{}\\\"\", go_json_escape(__v))).collect::<Vec<_>>().join(\",\"); ")
	out.WriteString("__json_fields.push(format!(")
	out.WriteString(strconv.Quote(strconv.Quote(field.jsonName) + ":[{}]"))
	out.WriteString(", __slice_entries));")
	if field.omitEmpty {
		out.WriteString(" } }")
	} else {
		out.WriteString(" } else { __json_fields.push(")
		out.WriteString(strconv.Quote(strconv.Quote(field.jsonName) + ":null"))
		out.WriteString(".to_string()); }")
	}
	out.WriteString(" }")
}

func writeJsonByteSliceMapFieldPush(out *strings.Builder, field jsonMarshalField) {
	out.WriteString("{ let __map_guard = __json_value.")
	out.WriteString(field.rustName)
	WriteBorrowMethod(out, false)
	out.WriteString("; ")
	if field.omitEmpty {
		out.WriteString("if let Some(__map) = __map_guard.as_ref() { if !__map.is_empty() { ")
	} else {
		out.WriteString("if let Some(__map) = __map_guard.as_ref() { ")
	}
	out.WriteString("let __map_entries = __map.iter().map(|(__k, __v)| { let __v_guard = __v")
	WriteBorrowMethod(out, false)
	out.WriteString("; if let Some(__bytes) = __v_guard.as_ref() { format!(\"\\\"{}\\\":\\\"{}\\\"\", go_json_escape(__k), go_base64_encode(__bytes)) } else { format!(\"\\\"{}\\\":null\", go_json_escape(__k)) } }).collect::<Vec<_>>().join(\",\"); ")
	out.WriteString("__json_fields.push(format!(")
	out.WriteString(strconv.Quote(strconv.Quote(field.jsonName) + ":{{{}}}"))
	out.WriteString(", __map_entries));")
	if field.omitEmpty {
		out.WriteString(" } }")
	} else {
		out.WriteString(" } else { __json_fields.push(")
		out.WriteString(strconv.Quote(strconv.Quote(field.jsonName) + ":null"))
		out.WriteString(".to_string()); }")
	}
	out.WriteString(" }")
}

func transpileJsonMarshal(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) == 0 {
		out.WriteString("/* ERROR: json.Marshal requires a value */ unimplemented!()")
		return
	}

	var fields []jsonMarshalField
	typeInfoStructUnsupported := false
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if st := typeInfo.GetStructType(call.Args[0]); st != nil {
			var ok bool
			fields, ok = jsonMarshalStructFields(st)
			if !ok {
				fields = nil
				typeInfoStructUnsupported = true
			}
		}
	}
	if fields == nil {
		if st, ok := jsonMarshalStructTypeFromSyntaxArg(call.Args[0]); ok {
			var fieldsOK bool
			fields, fieldsOK = jsonMarshalStructFieldsFromSyntax(st)
			if !fieldsOK {
				out.WriteString("/* ERROR: json.Marshal currently supports exported basic, []string, map[string]string, and map[string][]byte struct fields */ unimplemented!()")
				return
			}
		}
	}
	if fields == nil {
		if typeInfoStructUnsupported {
			out.WriteString("/* ERROR: json.Marshal currently supports exported basic, []string, map[string]string, and map[string][]byte struct fields */ unimplemented!()")
			return
		}
		out.WriteString("/* ERROR: json.Marshal currently supports struct values with exported basic fields */ unimplemented!()")
		return
	}

	for _, field := range fields {
		if field.kind == jsonMarshalStringMapField || field.kind == jsonMarshalStringSliceField || field.kind == jsonMarshalByteSliceMapField || field.basicKind == types.String {
			NeedJsonEscape()
		}
		if field.kind == jsonMarshalByteSliceMapField {
			NeedBase64()
		}
	}

	out.WriteString("{ let __json_value = ")
	writeJsonMarshalValueBinding(out, call.Args[0])
	out.WriteString("; let mut __json_fields: Vec<String> = Vec::new(); ")
	for _, field := range fields {
		switch field.kind {
		case jsonMarshalBasicField:
			writeJsonBasicFieldPush(out, field)
		case jsonMarshalStringMapField:
			writeJsonStringMapFieldPush(out, field)
		case jsonMarshalStringSliceField:
			writeJsonStringSliceFieldPush(out, field)
		case jsonMarshalByteSliceMapField:
			writeJsonByteSliceMapFieldPush(out, field)
		}
		out.WriteString(" ")
	}
	out.WriteString("let __json = format!(\"{{{}}}\", __json_fields.join(\",\")); (")
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

func transpileMin(out *strings.Builder, call *ast.CallExpr) {
	transpileBuiltinMinMax(out, call, "min")
}

func transpileMax(out *strings.Builder, call *ast.CallExpr) {
	transpileBuiltinMinMax(out, call, "max")
}

func transpileBuiltinMinMax(out *strings.Builder, call *ast.CallExpr, op string) {
	if len(call.Args) == 0 {
		out.WriteString("/* ERROR: ")
		out.WriteString(op)
		out.WriteString(" requires at least one argument */ unimplemented!()")
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for ")
		out.WriteString(op)
		out.WriteString(" */ unimplemented!()")
		return
	}
	resultType := typeInfo.GetType(call)
	if resultType == nil {
		out.WriteString("/* ERROR: Type information required for ")
		out.WriteString(op)
		out.WriteString(" */ unimplemented!()")
		return
	}
	basic, ok := types.Unalias(resultType).Underlying().(*types.Basic)
	if !ok {
		out.WriteString("/* ERROR: ")
		out.WriteString(op)
		out.WriteString(" requires an ordered basic type */ unimplemented!()")
		return
	}

	args := make([]string, 0, len(call.Args))
	for _, arg := range call.Args {
		var argOut strings.Builder
		writeBuiltinMinMaxArg(&argOut, arg, basic)
		args = append(args, argOut.String())
	}
	expr := args[0]
	for _, arg := range args[1:] {
		if basic.Kind() == types.Float32 || basic.Kind() == types.Float64 {
			expr = "(" + expr + ")." + op + "(" + arg + ")"
		} else {
			expr = "std::cmp::" + op + "(" + expr + ", " + arg + ")"
		}
	}
	out.WriteString(expr)
}

func writeBuiltinMinMaxArg(out *strings.Builder, arg ast.Expr, basic *types.Basic) {
	if basic.Kind() == types.String || basic.Kind() == types.UntypedString {
		writeStringSequenceValue(out, arg)
		return
	}
	out.WriteString("(")
	writeUnwrappedForFormat(out, arg)
	out.WriteString(" as ")
	out.WriteString(goTypesTypeToRust(basic))
	out.WriteString(")")
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
		if rustType, ok := unsafeTypeSizeRustTypeFromSyntax(call.Args[0]); ok {
			out.WriteString("std::mem::")
			out.WriteString(rustFunc)
			out.WriteString("::<")
			out.WriteString(rustType)
			out.WriteString(">()")
			WriteWrapperSuffix(out)
			return
		}
		out.WriteString("/* ERROR: Type information required for unsafe.")
		out.WriteString(goFunc)
		out.WriteString(" */ unimplemented!()")
		WriteWrapperSuffix(out)
		return
	}

	argType := typeInfo.GetType(call.Args[0])
	if argType == nil {
		if rustType, ok := unsafeTypeSizeRustTypeFromSyntax(call.Args[0]); ok {
			out.WriteString("std::mem::")
			out.WriteString(rustFunc)
			out.WriteString("::<")
			out.WriteString(rustType)
			out.WriteString(">()")
			WriteWrapperSuffix(out)
			return
		}
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

func unsafeTypeSizeRustTypeFromSyntax(expr ast.Expr) (string, bool) {
	switch e := expr.(type) {
	case *ast.Ident:
		if info := lookupVarInfo(e.Name); info != nil && info.RustType != "" {
			return unwrapStoredRustType(info.RustType), true
		}
	case *ast.BasicLit:
		switch e.Kind {
		case token.INT:
			return "i32", true
		case token.FLOAT:
			return "f64", true
		case token.CHAR:
			return "i32", true
		case token.STRING:
			return "String", true
		}
	case *ast.UnaryExpr:
		if e.Op == token.AND {
			return "usize", true
		}
	}
	return "", false
}

func unwrapStoredRustType(rustType string) string {
	rustType = strings.TrimPrefix(rustType, "&")
	for {
		switch {
		case strings.HasPrefix(rustType, "Rc<RefCell<Option<") && strings.HasSuffix(rustType, ">>>"):
			rustType = strings.TrimSuffix(strings.TrimPrefix(rustType, "Rc<RefCell<Option<"), ">>>")
		case strings.HasPrefix(rustType, "Arc<Mutex<Option<") && strings.HasSuffix(rustType, ">>>"):
			rustType = strings.TrimSuffix(strings.TrimPrefix(rustType, "Arc<Mutex<Option<"), ">>>")
		default:
			return rustType
		}
	}
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

func appendExpandsStringIntoByteSlice(call *ast.CallExpr) bool {
	if call == nil || !call.Ellipsis.IsValid() || len(call.Args) != 2 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return typeInfo.IsByteSliceOrArray(call.Args[0]) && typeInfo.IsString(call.Args[1])
}

func isBuiltinAppendCall(call *ast.CallExpr) bool {
	if call == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	ident, ok := call.Fun.(*ast.Ident)
	if !ok {
		return false
	}
	obj, ok := typeInfo.info.Uses[ident]
	if !ok {
		return false
	}
	builtin, ok := obj.(*types.Builtin)
	return ok && builtin.Name() == "append"
}

func appendCallReturnsBareIndexedSlice(call *ast.CallExpr) bool {
	if !isBuiltinAppendCall(call) || len(call.Args) == 0 {
		return false
	}
	indexExpr, ok := call.Args[0].(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.IsMap(indexExpr.X) {
		return false
	}
	targetType := typeInfo.GetType(indexExpr)
	if targetType == nil {
		return false
	}
	if _, ok := types.Unalias(targetType).Underlying().(*types.Slice); !ok {
		return false
	}
	containerType := typeInfo.GetType(indexExpr.X)
	if containerType == nil {
		return false
	}
	switch types.Unalias(containerType).Underlying().(type) {
	case *types.Array, *types.Slice:
		return true
	default:
		return false
	}
}

func transpileAppend(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 2 {
		if transpileNamedSliceAppend(out, call) {
			return
		}
		writeAppendElement := func(expr ast.Expr) {
			var elemType types.Type
			if typeInfo := GetTypeInfo(); typeInfo != nil {
				elemType = typeInfo.GetSliceElemType(call.Args[0])
				if isGoErrorType(elemType) && writeGoErrorHandleValue(out, expr) {
					return
				}
				if writeNilStdlibInterfaceBareValue(out, expr, elemType) {
					return
				}
				if writeStdlibInterfaceBareConversion(out, expr, elemType) {
					return
				}
				if callExpr, ok := expr.(*ast.CallExpr); ok && typeInfo.ReturnsWrappedValue(callExpr) && !callReturnsBareChannelValue(callExpr) {
					if compositeLiteralElementKeepsHandle(elemType) {
						TranspileExpression(out, expr)
						return
					}
					out.WriteString("(*")
					TranspileExpression(out, expr)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).clone()")
					return
				}
				if elemType != nil {
					if isFunctionSignatureType(elemType) && writeFunctionValueHandle(out, expr) {
						return
					}
					if writeBareFixedArrayCompositeLiteral(out, expr, elemType) {
						return
					}
					if writeConstExpressionForExpectedGoType(out, expr, elemType) {
						return
					}
					if basic, ok := types.Unalias(elemType).Underlying().(*types.Basic); ok && basic.Kind() == types.String && writeRangeStringValue(out, expr) {
						return
					}
					if writeWrappedRangeValueForExpectedType(out, expr, elemType) {
						return
					}
					if _, isNamed := types.Unalias(elemType).(*types.Named); !isNamed {
						if _, ok := types.Unalias(elemType).Underlying().(*types.Slice); ok {
							writeUnwrappedSliceClone(out, expr)
							return
						}
					}
					if _, ok := localNamedInterfaceTypeNameFromTypes(elemType); ok && isBareLocalInterfaceValue(expr) {
						writeLocalInterfaceBareClone(out, expr)
						return
					}
					if _, ok := types.Unalias(elemType).Underlying().(*types.Pointer); ok && typeInfo.IsPointer(expr) {
						TranspileExpressionContext(out, expr, LValue)
						out.WriteString(".clone()")
						return
					}
				}
			}
			if elemRustType, ok := trackedRangeElemRustType(call.Args[0]); ok && writeWrappedRangeValueForRustElemType(out, expr, elemRustType) {
				return
			}
			if ident, ok := expr.(*ast.Ident); ok {
				if writeOwnedRangeValue(out, ident) {
					return
				}
			}
			if !writeOwnedExpressionValue(out, expr) {
				TranspileExpression(out, expr)
			}
		}
		writeAppendTarget := func(expr ast.Expr) {
			if ident, ok := expr.(*ast.Ident); ok {
				if writeCurrentReceiverStorage(out, ident) {
					return
				}
				varName := RustIdentForUse(ident)
				if currentCaptureRenames != nil {
					if renamed, exists := currentCaptureRenames[ident.Name]; exists {
						varName = RustLocalIdent(renamed)
					}
				}
				out.WriteString(varName)
				return
			}
			switch expr.(type) {
			case *ast.SelectorExpr, *ast.IndexExpr:
				TranspileExpressionContext(out, expr, LValue)
			default:
				TranspileExpression(out, expr)
			}
		}
		writeAppendExpansionSource := func(expr ast.Expr) {
			if appendExpandsStringIntoByteSlice(call) {
				writeOwnedStringStdlibArg(out, expr)
				out.WriteString(".as_bytes().iter().cloned()")
			} else {
				writeSliceCloneOrEmpty(out, expr)
				out.WriteString(".iter().cloned()")
			}
		}
		writeIndexedSliceAppend := func(indexExpr *ast.IndexExpr) bool {
			if !appendCallReturnsBareIndexedSlice(call) {
				return false
			}
			writeMutableTarget := func() {
				out.WriteString("(*")
				TranspileExpressionContext(out, indexExpr.X, LValue)
				WriteBorrowMethod(out, true)
				out.WriteString(".as_mut().unwrap())[")
				writeExpressionAsUsize(out, indexExpr.Index)
				out.WriteString("]")
			}
			writeReadTarget := func() {
				out.WriteString("(*")
				TranspileExpressionContext(out, indexExpr.X, LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())[")
				writeExpressionAsUsize(out, indexExpr.Index)
				out.WriteString("]")
			}
			out.WriteString("{ ")
			writeMutableTarget()
			if call.Ellipsis.IsValid() {
				out.WriteString(".extend(")
				writeAppendExpansionSource(call.Args[1])
				out.WriteString(")")
			} else if len(call.Args) == 2 {
				out.WriteString(".push(")
				writeAppendElement(call.Args[1])
				out.WriteString(")")
			} else {
				out.WriteString(".extend(vec![")
				for i := 1; i < len(call.Args); i++ {
					if i > 1 {
						out.WriteString(", ")
					}
					writeAppendElement(call.Args[i])
				}
				out.WriteString("])")
			}
			out.WriteString("; ")
			writeReadTarget()
			out.WriteString(".clone() }")
			return true
		}
		writeMapIndexSliceAppend := func(indexExpr *ast.IndexExpr) bool {
			typeInfo := GetTypeInfo()
			if typeInfo == nil || !typeInfo.IsMap(indexExpr.X) {
				return false
			}
			keyType, valueType := typeInfo.GetMapTypes(indexExpr.X)
			if valueType == nil {
				return false
			}
			if _, ok := types.Unalias(valueType).Underlying().(*types.Slice); !ok {
				return false
			}
			writeMapHandle := func() {
				if ident, ok := indexExpr.X.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpressionContext(out, indexExpr.X, LValue)
				}
			}
			out.WriteString("{ let __slice = { let __map_holder = ")
			writeMapHandle()
			out.WriteString(".clone(); let __map_guard = __map_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; __map_guard.as_ref().unwrap().get(")
			writeMapLookupKeyWithType(out, indexExpr.Index, keyType)
			out.WriteString(").cloned().unwrap_or_else(|| ")
			WriteWrappedNone(out)
			out.WriteString(") }; (*__slice")
			WriteBorrowMethod(out, true)
			out.WriteString(").get_or_insert_with(Vec::new)")
			if call.Ellipsis.IsValid() {
				out.WriteString(".extend(")
				writeAppendExpansionSource(call.Args[1])
				out.WriteString(")")
			} else if len(call.Args) == 2 {
				out.WriteString(".push(")
				writeAppendElement(call.Args[1])
				out.WriteString(")")
			} else {
				out.WriteString(".extend(vec![")
				for i := 1; i < len(call.Args); i++ {
					if i > 1 {
						out.WriteString(", ")
					}
					writeAppendElement(call.Args[i])
				}
				out.WriteString("])")
			}
			out.WriteString("; __slice.clone() }")
			return true
		}

		// append() in Go returns the slice, but our slices are wrapped
		// We need to create the vector on first append so nil slices stay nil
		// until they are actually appended to, then return the wrapped slice.
		if indexExpr, ok := call.Args[0].(*ast.IndexExpr); ok && writeMapIndexSliceAppend(indexExpr) {
			return
		}
		if indexExpr, ok := call.Args[0].(*ast.IndexExpr); ok && writeIndexedSliceAppend(indexExpr) {
			return
		}
		writeAppendTargetMutationPrefix := func() {
			out.WriteString("{ let __append_target = ")
			writeAppendTarget(call.Args[0])
			out.WriteString(".clone(); (*__append_target")
			WriteBorrowMethod(out, true)
			out.WriteString(").get_or_insert_with(Vec::new)")
		}
		writeAppendTargetMutationSuffix := func() {
			out.WriteString("; __append_target.clone() }")
		}
		if call.Ellipsis.IsValid() {
			// Slice expansion: append(dst, src...) → extend from src.
			// Go also permits append([]byte, string...), which expands bytes.
			writeAppendTargetMutationPrefix()
			out.WriteString(".extend(")
			writeAppendExpansionSource(call.Args[1])
			out.WriteString(")")
			writeAppendTargetMutationSuffix()
		} else if len(call.Args) == 2 {
			// Single element append
			writeAppendTargetMutationPrefix()
			out.WriteString(".push(")
			writeAppendElement(call.Args[1])
			out.WriteString(")")
			writeAppendTargetMutationSuffix()
		} else {
			// Multiple elements, use extend
			writeAppendTargetMutationPrefix()
			out.WriteString(".extend(vec![")
			for i := 1; i < len(call.Args); i++ {
				if i > 1 {
					out.WriteString(", ")
				}
				writeAppendElement(call.Args[i])
			}
			out.WriteString("])")
			writeAppendTargetMutationSuffix()
		}
	}
}

func transpileNamedSliceAppend(out *strings.Builder, call *ast.CallExpr) bool {
	named, sliceType, ok := namedSliceTypeForExpr(call.Args[0])
	if !ok {
		return false
	}
	elemIsPointer := false
	if _, ok := sliceType.Elem().(*types.Pointer); ok {
		elemIsPointer = true
	}
	writeElement := func(expr ast.Expr) {
		if elemIsPointer {
			if ident, ok := expr.(*ast.Ident); ok {
				if isWrappedValueIdent(ident) {
					out.WriteString(RustIdentForUse(ident))
					out.WriteString(".clone()")
					return
				}
				if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
					out.WriteString(EscapeRustIdent(ident.Name))
					out.WriteString(".clone()")
					return
				}
			}
			TranspileExpression(out, expr)
			return
		}
		if writeNilStdlibInterfaceBareValue(out, expr, sliceType.Elem()) {
			return
		}
		if !writeOwnedExpressionValue(out, expr) {
			TranspileExpression(out, expr)
		}
	}
	out.WriteString("{ let __base = ")
	writeNamedSliceInnerHandleClone(out, call.Args[0])
	out.WriteString("; let __base_guard = __base")
	WriteBorrowMethod(out, false)
	out.WriteString("; let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); ")
	if call.Ellipsis.IsValid() {
		if isNamedSliceExpression(call.Args[1]) {
			out.WriteString("let __src = ")
			writeNamedSliceInnerHandleClone(out, call.Args[1])
			out.WriteString("; let __src_guard = __src")
			WriteBorrowMethod(out, false)
			out.WriteString("; if let Some(__src_values) = __src_guard.as_ref() { __values.extend(__src_values.iter().cloned()); }; ")
		} else {
			out.WriteString("__values.extend(")
			TranspileExpression(out, call.Args[1])
			out.WriteString(".iter().cloned()); ")
		}
	} else if len(call.Args) == 2 {
		out.WriteString("__values.push(")
		writeElement(call.Args[1])
		out.WriteString("); ")
	} else {
		out.WriteString("__values.extend(vec![")
		for i := 1; i < len(call.Args); i++ {
			if i > 1 {
				out.WriteString(", ")
			}
			writeElement(call.Args[i])
		}
		out.WriteString("]); ")
	}
	WriteWrapperPrefix(out)
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	out.WriteString("__values")
	WriteWrapperSuffix(out)
	out.WriteString(")")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
	return true
}

func transpileLen(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			out.WriteString(RustStringLiteral(lit.Value))
			out.WriteString(".len()")
			return
		}

		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.IsChannel(call.Args[0]) {
			writeChannelExpression(out, call.Args[0])
			out.WriteString(".len()")
			return
		}

		if writeNamedSliceLen(out, call.Args[0]) {
			return
		}

		// len() returns the length of arrays, slices, maps, strings, or channels
		if isExpressionResultBare(call.Args[0]) {
			// Bare value (range var, index result, etc.) - access directly
			TranspileExpressionContext(out, call.Args[0], LValue)
			out.WriteString(".len()")
		} else if typeInfo != nil && (typeInfo.IsSlice(call.Args[0]) || typeInfo.IsMap(call.Args[0])) {
			out.WriteString("(*")
			TranspileExpressionContext(out, call.Args[0], LValue)
			WriteBorrowMethod(out, false)
			out.WriteString(").as_ref().map(|__v| __v.len()).unwrap_or(0)")
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
				writeChannelCapacityAsUsize(out, call.Args[1])
				out.WriteString(")")
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
			keyType := goMapKeyTypeToRustBase(mapType.Key)
			valueType := GoTypeToRust(mapType.Value)
			if typeInfo := GetTypeInfo(); typeInfo != nil {
				if typ := typeInfo.GetType(call.Args[0]); typ != nil {
					if checkedMap, ok := typ.Underlying().(*types.Map); ok {
						keyType = goTypesMapKeyToRust(checkedMap.Key())
						valueType = goTypesMapValueToRust(checkedMap.Elem())
					}
				}
			}
			out.WriteString("BTreeMap::<")
			out.WriteString(keyType)
			out.WriteString(", ")
			out.WriteString(valueType)
			out.WriteString(">::new()")
			out.WriteString(")))")
		} else if arrayType, ok := call.Args[0].(*ast.ArrayType); ok && arrayType.Len == nil {
			// Slice type - check element type
			elementType := zeroValueForGoType(arrayType.Elt)
			if typeInfo := GetTypeInfo(); typeInfo != nil {
				if typ := typeInfo.GetType(call.Args[0]); typ != nil {
					if sliceType, ok := types.Unalias(typ).Underlying().(*types.Slice); ok {
						elementType = zeroValueForTypesType(sliceType.Elem())
					}
				}
			}
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

func writeChannelCapacityAsUsize(out *strings.Builder, expr ast.Expr) {
	if lit, ok := expr.(*ast.BasicLit); ok && lit.Kind == token.INT {
		out.WriteString(lit.Value)
		out.WriteString(" as usize")
		return
	}
	writeExpressionAsUsize(out, expr)
}

func transpileCap(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.IsChannel(call.Args[0]) {
			writeChannelExpression(out, call.Args[0])
			out.WriteString(".capacity()")
			return
		}

		if writeNamedSliceCap(out, call.Args[0]) {
			return
		}

		member := "capacity()"
		if typeInfo != nil && typeInfo.IsArray(call.Args[0]) {
			member = "len()"
		}

		if isExpressionResultBare(call.Args[0]) {
			TranspileExpressionContext(out, call.Args[0], LValue)
			out.WriteString(".")
			out.WriteString(member)
		} else if typeInfo != nil && typeInfo.IsSlice(call.Args[0]) {
			out.WriteString("(*")
			TranspileExpressionContext(out, call.Args[0], LValue)
			WriteBorrowMethod(out, false)
			out.WriteString(").as_ref().map(|__v| __v.capacity()).unwrap_or(0)")
		} else {
			out.WriteString("(*")
			TranspileExpressionContext(out, call.Args[0], LValue)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).")
			out.WriteString(member)
		}
	}
}

func transpileDelete(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 2 {
		var keyType types.Type
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			keyType, _ = typeInfo.GetMapTypes(call.Args[0])
		}
		out.WriteString("{ let __map_handle = ")
		if ident, ok := call.Args[0].(*ast.Ident); ok {
			out.WriteString(ident.Name)
		} else {
			TranspileExpressionContext(out, call.Args[0], LValue)
		}
		out.WriteString(".clone(); let mut __map_guard = __map_handle")
		WriteBorrowMethod(out, true)
		out.WriteString("; __map_guard.as_mut().unwrap().remove(")
		writeMapLookupKeyWithType(out, call.Args[1], keyType)
		out.WriteString("); }")
	}
}

func transpileCopy(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 2 {
		// Go: copy(dst, src) copies min(len(dst), len(src)) elements, returns count
		typeInfo := GetTypeInfo()
		srcIsString := typeInfo != nil && typeInfo.IsString(call.Args[1])

		if dstSlice, ok := call.Args[0].(*ast.SliceExpr); ok {
			out.WriteString("{ let _dst_start = ")
			writeCopySliceLow(out, dstSlice)
			out.WriteString("; let _dst_len = ")
			writeCopySliceLen(out, dstSlice)
			out.WriteString("; let _src = ")
			writeCopySourceValue(out, call.Args[1], srcIsString)
			out.WriteString("; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*")
			TranspileExpressionContext(out, dstSlice.X, LValue)
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } ")
			WriteWrapperPrefix(out)
			out.WriteString("_n as i32")
			WriteWrapperSuffix(out)
			out.WriteString(" }")
			return
		}

		out.WriteString("{ let _src = ")
		writeCopySourceValue(out, call.Args[1], srcIsString)
		out.WriteString("; let _n = std::cmp::min((")
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

func writeCopySourceValue(out *strings.Builder, expr ast.Expr, isString bool) {
	if isString {
		writeStringSequenceValue(out, expr)
		out.WriteString(".as_bytes().to_vec()")
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.ReturnsWrappedValue(expr) && !isExpressionResultBare(expr) {
		out.WriteString("(*")
		TranspileExpression(out, expr)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return
	}
	out.WriteString("(")
	TranspileExpression(out, expr)
	out.WriteString(").clone()")
}

func writeCopySliceLow(out *strings.Builder, slice *ast.SliceExpr) {
	if slice.Low != nil {
		writeExpressionAsUsize(out, slice.Low)
	} else {
		out.WriteString("0")
	}
}

func writeCopySliceLen(out *strings.Builder, slice *ast.SliceExpr) {
	if slice.High != nil {
		out.WriteString("(")
		writeExpressionAsUsize(out, slice.High)
		out.WriteString(") - _dst_start")
	} else {
		out.WriteString("(*")
		TranspileExpressionContext(out, slice.X, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).len() - _dst_start")
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
	TrackImport("BTreeMap")
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
func generateSliceFormatter(out *strings.Builder, includeWrappedValues bool, includeWrappedStringer bool) {
	TrackImport("Display")
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
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

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}
`)
		if includeWrappedValues {
			out.WriteString(`

fn format_slice_wrapped_values<T>(slice: &[Arc<Mutex<Option<T>>>]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| {
        let inner = v.lock().unwrap();
        match inner.as_ref() {
            Some(value) => format!("&{}", value),
            None => "<nil>".to_string(),
        }
    }).collect();
    format!("[{}]", formatted.join(" "))
}
`)
		}
		if includeWrappedStringer {
			out.WriteString(`

fn format_slice_wrapped_stringer<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        format_slice_wrapped_stringer_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}

fn format_slice_wrapped_stringer_values<T>(slice: &[Arc<Mutex<Option<T>>>]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| {
        let inner = v.lock().unwrap();
        match inner.as_ref() {
            Some(value) => value.to_string(),
            None => "<nil>".to_string(),
        }
    }).collect();
    format!("[{}]", formatted.join(" "))
}
`)
		}
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString(`fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
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

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}
`)
		if includeWrappedValues {
			out.WriteString(`

fn format_slice_wrapped_values<T>(slice: &[Rc<RefCell<Option<T>>>]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| {
        let inner = v.borrow();
        match inner.as_ref() {
            Some(value) => format!("&{}", value),
            None => "<nil>".to_string(),
        }
    }).collect();
    format!("[{}]", formatted.join(" "))
}
`)
		}
		if includeWrappedStringer {
			out.WriteString(`

fn format_slice_wrapped_stringer<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        format_slice_wrapped_stringer_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}

fn format_slice_wrapped_stringer_values<T>(slice: &[Rc<RefCell<Option<T>>>]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| {
        let inner = v.borrow();
        match inner.as_ref() {
            Some(value) => value.to_string(),
            None => "<nil>".to_string(),
        }
    }).collect();
    format!("[{}]", formatted.join(" "))
}
`)
		}
	}
}

func generateNestedSliceFormatter(out *strings.Builder) {
	TrackImport("Display")
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`fn format_nested_slice<T, C, Inner>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s
            .as_ref()
            .iter()
            .map(|inner| format_slice_values(inner.as_ref()))
            .collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString(`fn format_nested_slice<T, C, Inner>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s
            .as_ref()
            .iter()
            .map(|inner| format_slice_values(inner.as_ref()))
            .collect();
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
							writeOwnedStringStdlibArg(out, callExpr.Args[0])
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
	anyType := rustAnyTraitObject()
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString("Arc::new(Mutex::new(None::<")
		out.WriteString(anyType)
		out.WriteString(">))")
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString("Rc::new(RefCell::new(None::<")
		out.WriteString(anyType)
		out.WriteString(">))")
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
			if multiplier, unit, ok := durationBinaryParts(binOp); ok {
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
		if multiplier, unit, ok := durationBinaryParts(binOp); ok {
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

func writeTimeDurationBinaryExpression(out *strings.Builder, expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.MUL {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isTimeDurationType(typeInfo.GetType(expr)) {
		return false
	}
	if _, _, ok := durationBinaryParts(expr); !ok {
		return false
	}
	transpileDurationArg(out, expr)
	return true
}

func writeTimeDurationValue(out *strings.Builder, value ast.Expr) {
	if binOp, ok := value.(*ast.BinaryExpr); ok && binOp.Op == token.MUL {
		if _, _, ok := durationBinaryParts(binOp); ok {
			transpileDurationArg(out, value)
			return
		}
	}
	if hasStdlibSelectorMapping(value) {
		TranspileExpression(out, value)
		return
	}
	if isConstantExpression(value) {
		out.WriteString("std::time::Duration::from_nanos(")
		writeConstExpressionCastValue(out, value)
		out.WriteString(" as u64)")
		return
	}
	TranspileExpression(out, value)
}

func isTimeDurationType(typ types.Type) bool {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	return named.Obj().Pkg().Path() == "time" && named.Obj().Name() == "Duration"
}

func durationBinaryParts(expr *ast.BinaryExpr) (ast.Expr, string, bool) {
	if expr == nil || expr.Op != token.MUL {
		return nil, "", false
	}
	if unit, ok := timeDurationUnitName(expr.Y); ok {
		return expr.X, unit, true
	}
	if unit, ok := timeDurationUnitName(expr.X); ok {
		return expr.Y, unit, true
	}
	return nil, "", false
}

func timeDurationUnitName(expr ast.Expr) (string, bool) {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return "", false
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok || resolveStdlibPackageName(ident.Name) != "time" {
		return "", false
	}
	switch sel.Sel.Name {
	case "Hour", "Minute", "Second", "Millisecond", "Microsecond", "Nanosecond":
		return sel.Sel.Name, true
	default:
		return "", false
	}
}
