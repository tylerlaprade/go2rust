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
		if handler, exists := stdlibMappings[key]; exists &&
			(!stdlibCallUsesSourceMappedPackage(call.Fun) || sourceMappedStdlibCallUsesHandler(key)) {
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

func sourceMappedStdlibCallUsesHandler(key string) bool {
	switch key {
	case "sort.Slice", "sort.SliceStable":
		return true
	default:
		return false
	}
}

func stdlibCallUsesSourceMappedPackage(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	current := sel.X
	for {
		if x, ok := current.(*ast.SelectorExpr); ok {
			current = x.X
			continue
		}
		ident, ok := current.(*ast.Ident)
		if !ok {
			return false
		}
		return isSourceMappedPackagePath(resolveStdlibPackageName(ident.Name))
	}
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
		"strconv.ParseBool":        transpileStrconvParseBool,
		"strconv.FormatFloat":      transpileStrconvFormatFloat,
		"strconv.FormatInt":        transpileStrconvFormatInt,
		"strconv.Quote":            transpileStrconvQuote,
		"errors.New":               transpileErrorsNew,
		"sort.Strings":             transpileSortStrings,
		"sort.Ints":                transpileSortInts,
		"sort.Sort":                transpileSortInterfaceSort,
		"sort.Stable":              transpileSortInterfaceSort,
		"sort.Search":              transpileSortSearch,
		"sort.Find":                transpileSortFind,
		"sort.Slice":               transpileSortSlice,
		"sort.SliceStable":         transpileSortSlice,
		"slices.Sort":              transpileSlicesSort,
		"slices.SortFunc":          transpileSlicesSortFunc,
		"slices.Contains":          transpileSlicesContains,
		"slices.Delete":            transpileSlicesDelete,
		"slices.Clone":             transpileSlicesClone,
		"slices.Clip":              transpileSlicesClip,
		"time.Sleep":               transpileTimeSleep,
		"time.Now":                 transpileTimeNow,
		"time.Unix":                transpileTimeUnix,
		"time.After":               transpileTimeAfter,
		"time.NewTicker":           transpileTimeNewTicker,
		"time.NewTimer":            transpileTimeNewTimer,
		"time.Tick":                transpileTimeTick,
		"time.AfterFunc":           transpileTimeAfterFunc,
		"context.Background":       transpileContextBackground,
		"context.WithTimeout":      transpileContextWithTimeout,
		"context.WithCancel":       transpileContextWithCancel,
		"context.WithCancelCause":  transpileContextWithCancelCause,
		"os.Create":                transpileOsCreate,
		"os.Remove":                transpileOsRemove,
		"reflect.TypeOf":           transpileReflectTypeOf,
		"reflect.ValueOf":          transpileReflectValueOf,
		"sync/atomic.AddInt64":     transpileAtomicAddInt64,
		"sync/atomic.LoadInt64":    transpileAtomicLoadInt64,
		"sync/atomic.LoadUint32":   transpileAtomicLoadUint32,
		"sync/atomic.StoreUint32":  transpileAtomicStoreUint32,
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
		"unsafe.Add":                                 transpileUnsafeAdd,
		"unsafe.Slice":                               transpileUnsafeSlice,
		"unsafe.String":                              transpileUnsafeString,
		"unsafe.SliceData":                           transpileUnsafeSliceData,
		"unsafe.StringData":                          transpileUnsafeStringData,
		"math/rand.Seed":                             transpileRandSeed,
		"math/rand.Intn":                             transpileRandIntn,
		"math/rand.Float64":                          transpileRandFloat64,
		"net/url.Parse":                              transpileUrlParse,
		"regexp.MustCompile":                         transpileRegexpMustCompile,
	}

	builtinMappings = map[string]StdlibHandler{
		"print":   transpileBuiltinPrint,
		"println": transpileBuiltinPrintln,
		"append":  transpileAppend,
		"len":     transpileLen,
		"make":    transpileMake,
		"cap":     transpileCap,
		"copy":    transpileCopy,
		"delete":  transpileDelete,
		"clear":   transpileClear,
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
	if writeFmtPrintlnVariadicAny(out, call) {
		return
	}

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

func writeFmtPrintlnVariadicAny(out *strings.Builder, call *ast.CallExpr) bool {
	if call == nil || !call.Ellipsis.IsValid() || len(call.Args) != 1 {
		return false
	}
	if !isEmptyInterfaceSliceArgument(call.Args[0]) {
		return false
	}
	NeedFormatAnySlice()
	TrackImport("Any")
	out.WriteString("println!(\"{}\", format_any_variadic(&")
	if ident, ok := call.Args[0].(*ast.Ident); ok {
		out.WriteString(RustIdentForUse(ident))
	} else {
		TranspileExpression(out, call.Args[0])
	}
	out.WriteString("))")
	return true
}

func isEmptyInterfaceSliceArgument(arg ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	elemType := typeInfo.GetSliceElemType(arg)
	if elemType == nil {
		return false
	}
	intf, ok := types.Unalias(elemType).Underlying().(*types.Interface)
	return ok && intf.NumMethods() == 0
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

func transpileBuiltinPrint(out *strings.Builder, call *ast.CallExpr) {
	// Go's builtin `print` writes to stderr without adding separators or a newline.
	out.WriteString("eprint!")
	out.WriteString("(")

	out.WriteString("\"")
	for range call.Args {
		out.WriteString("{}")
	}
	out.WriteString("\"")

	for _, arg := range call.Args {
		out.WriteString(", ")
		transpilePrintArgString(out, arg)
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
	if writeBareAnyPayloadRef(out, arg) {
		out.WriteString(")")
		return
	}
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

func writeBareAnyPayloadRef(out *strings.Builder, arg ast.Expr) bool {
	ident, ok := arg.(*ast.Ident)
	if !ok {
		return false
	}
	rustType := ""
	if varType, ok := rangeLoopVars[ident.Name]; ok {
		rustType = strings.TrimPrefix(varType, "&")
	} else if info := lookupVarInfo(ident.Name); info != nil && info.WrapLevel == WrapNone {
		rustType = strings.TrimPrefix(info.RustType, "&")
	}
	if !strings.Contains(rustType, "Box<dyn Any") {
		return false
	}
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(".as_ref()")
	return true
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
		if writeGoErrorFormatArg(out, arg, argType) {
			return
		}
		if call, ok := arg.(*ast.CallExpr); ok && typeInfo.IsTypeConversion(call) && !typeConversionEmitsWrappedValue(call) {
			TranspileExpression(out, arg)
			return
		}

		// Check if it's any kind of interface
		if intf, ok := argType.Underlying().(*types.Interface); ok {
			// Special case for error type - use Display not Debug
			if named, ok := argType.(*types.Named); ok {
				if named.Obj().Name() == "error" && named.Obj().Pkg() == nil {
					// It's the builtin error type - use Display formatting
					if ident, ok := arg.(*ast.Ident); ok {
						out.WriteString("format!(\"{}\", (*")
						out.WriteString(rustIdentForUseWithCapture(ident))
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
			if _, isLocalNamedInterface := transpiledNamedInterfaceTypeNameFromTypes(elemType); isLocalNamedInterface {
				// Local named interface slice elements are wrapped as
				// Rc<RefCell<Option<Box<dyn Trait>>>>; use the wrapped stringer
				// helper since named interfaces always carry Display via their
				// trait bound.
				NeedFormatSliceWrappedStringer()
				writeFormatSliceCall(out, arg, "format_slice_wrapped_stringer", "format_slice_wrapped_stringer_values")
				return
			}
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
		if writePointerAddressPrintArg(out, arg, argType) {
			return
		}
		// Check if it's a pointer to a struct - Go prints "&{...}" for these
		if ptr, ok := argType.(*types.Pointer); ok {
			if _, ok := ptr.Elem().Underlying().(*types.Struct); ok {
				if ident, ok := arg.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
					// Avoid recursing into our own Display impl by formatting
					// the receiver as a pointer address.
					out.WriteString("format!(\"{:p}\", self)")
					return
				}
				out.WriteString("format!(\"&{}\", (*")
				if ident, ok := arg.(*ast.Ident); ok {
					out.WriteString(RustIdentForUse(ident))
				} else if sel, ok := arg.(*ast.SelectorExpr); ok {
					writeSelectorHandleClone(out, sel)
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
		if ident, ok := sel.X.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
			// self.field - need to unwrap for display, resolving promoted fields
			receiverName := currentReceiverRustName()
			fieldInfo := resolveFieldAccess(currentReceiverType, sel.Sel.Name)
			if fieldInfo.IsPromoted && len(fieldInfo.EmbeddedPath) > 0 {
				// Promoted field - traverse through embedded structs
				out.WriteString("(*")
				out.WriteString(receiverName)
				out.WriteString(".")
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
				out.WriteString("(*")
				out.WriteString(receiverName)
				out.WriteString(".")
				out.WriteString(ToSnakeCase(sel.Sel.Name))
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())")
			}
			return
		}
	}
	// Check if this is a function call that returns a wrapped value
	if callExpr, ok := arg.(*ast.CallExpr); ok {
		if callReturnsBareScalar(callExpr) {
			TranspileExpression(out, arg)
			return
		}
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

func writePointerAddressPrintArg(out *strings.Builder, arg ast.Expr, argType types.Type) bool {
	if !isPointerToPointerType(argType) {
		return false
	}
	unary, ok := unwrapParens(arg).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return false
	}
	ident, ok := unwrapParens(unary.X).(*ast.Ident)
	if !ok || ident.Name == "_" || ident.Name == "nil" {
		return false
	}
	trackWrapperImports()
	out.WriteString("format!(\"0x{:x}\", ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::as_ptr(&")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(") as usize)")
	return true
}

func isPointerToPointerType(typ types.Type) bool {
	ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	_, ok = types.Unalias(ptr.Elem()).Underlying().(*types.Pointer)
	return ok
}

func writeGoErrorFormatArg(out *strings.Builder, arg ast.Expr, argType types.Type) bool {
	if !isGoErrorType(argType) {
		return false
	}
	out.WriteString("format!(\"{}\", (*")
	if ident, ok := arg.(*ast.Ident); ok {
		out.WriteString(rustIdentForUseWithCapture(ident))
	} else {
		TranspileExpressionContext(out, arg, LValue)
	}
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()))")
	return true
}

// convertFormatStringWithSkips converts Go format verbs to Rust format strings
// Returns: (format_string, skipIndices, charIndices, typeNameIndices, unicodeIndices, pointerIndices, hexFormats)
func convertFormatStringWithSkips(goFormat string) (string, []int, []int, []int, []int, []int, map[int]string) {
	var skipIndices []int
	var typeNameIndices []int
	var unicodeIndices []int
	var pointerIndices []int
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
				if format[i+2] == '.' {
					j := i + 3
					for j < len(format) && format[j] >= '0' && format[j] <= '9' {
						j++
					}
					if j < len(format) && (format[j] == 'f' || format[j] == 'g') {
						precision := format[i+3 : j]
						result.WriteString("{:+.")
						result.WriteString(precision)
						result.WriteString("}")
						argIndex++
						i = j + 1
						continue
					}
				}
				switch format[i+2] {
				case 'd', 'f', 'g':
					result.WriteString("{:+}")
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
				case 'U':
					// %#U adds the character literal in single quotes after the
					// code point; approximate with the same U+XXXX form for now.
					result.WriteString("U+{:04X}")
					unicodeIndices = append(unicodeIndices, argIndex)
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
				if j < len(format) && (format[j] == 'f' || format[j] == 'g' || format[j] == 'd' || format[j] == 's') {
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
					case 'd', 'f':
						result.WriteString("{:")
						if leftAlign {
							result.WriteString("<")
						}
						if zeroPad {
							result.WriteString("0")
						}
						result.WriteString(width)
						if format[j] == 'f' {
							result.WriteString(".6")
						}
						result.WriteString("}")
						argIndex++
						i = j + 1
						continue
					}
				}

				// Handle single-char format verbs
				switch format[i+1] {
				case 'd', 'g', 's', 'v', 't', 'w':
					result.WriteString("{}")
					argIndex++
				case 'p':
					result.WriteString("{:p}")
					pointerIndices = append(pointerIndices, argIndex)
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

	return finalFormat, skipIndices, charIndices, typeNameIndices, unicodeIndices, pointerIndices, hexFormats
}

// convertFormatString converts Go format strings to Rust format strings
func convertFormatString(goFormat string) string {
	converted, _, _, _, _, _, _ := convertFormatStringWithSkips(goFormat)
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

func transpileFormatPointerArg(out *strings.Builder, arg ast.Expr) {
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("std::ptr::null::<()>()")
		return
	}

	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("unimplemented!(\"type info required for %p format argument\")")
		return
	}
	argType := typeInfo.GetType(arg)
	if argType == nil {
		out.WriteString("unimplemented!(\"type info required for %p format argument\")")
		return
	}

	if ident, ok := arg.(*ast.Ident); ok {
		if isCurrentReceiverIdent(ident) {
			out.WriteString("self")
			return
		}
	}
	if isBareLocalInterfaceValue(arg) {
		TranspileExpression(out, arg)
		return
	}

	switch types.Unalias(argType).Underlying().(type) {
	case *types.Pointer, *types.Interface, *types.Slice, *types.Map, *types.Chan, *types.Signature:
		if NeedsConcurrentWrapper() {
			TrackImport("Arc")
			out.WriteString("Arc::as_ptr(&")
		} else {
			TrackImport("Rc")
			out.WriteString("Rc::as_ptr(&")
		}
		TranspileExpressionContext(out, arg, LValue)
		out.WriteString(")")
	default:
		out.WriteString("unimplemented!(\"unsupported %p format argument type\")")
	}
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
	if typeInfo != nil && isNamedIntegerType(typeInfo.GetType(arg)) {
		var value strings.Builder
		if writeNamedIntegerPrimitiveExpression(&value, arg) {
			out.WriteString("format!(\"{:")
			out.WriteString(formatSpec)
			out.WriteString("}\", ")
			out.WriteString(value.String())
			out.WriteString(")")
			return
		}
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

func writeTypeNameFormatArg(out *strings.Builder, arg ast.Expr) {
	NeedGoTypeName()
	out.WriteString(goTypeNameHelperRustName)
	out.WriteString("(")
	if writeBareAnyPayloadRef(out, arg) {
		out.WriteString(")")
		return
	}
	if ident, ok := arg.(*ast.Ident); ok {
		if isVarBare(ident.Name) {
			out.WriteString(ident.Name)
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
				out.WriteString("&**")
				out.WriteString(ident.Name)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()")
			} else {
				out.WriteString(ident.Name)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()")
			}
		}
	} else {
		out.WriteString("&")
		transpilePrintArg(out, arg)
	}
	out.WriteString(")")
}

func transpileFormatArg(out *strings.Builder, arg ast.Expr, argIndex int, charIndices []int, typeNameIndices []int, unicodeIndices []int, pointerIndices []int, hexFormats map[int]string) {
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
	isPointerArg := false
	for _, ptrIdx := range pointerIndices {
		if ptrIdx == argIndex {
			isPointerArg = true
			break
		}
	}

	if isTypeNameArg {
		writeTypeNameFormatArg(out, arg)
	} else if isUnicodeArg {
		transpilePrintArg(out, arg)
		out.WriteString(" as u32")
	} else if isPointerArg {
		transpileFormatPointerArg(out, arg)
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
	var pointerIndices []int
	hexFormats := make(map[int]string)
	if len(call.Args) > formatArgIndex {
		if lit, ok := call.Args[formatArgIndex].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust and get skip/char/typeName indices
			format, skips, chars, typeNames, unicodes, pointers, hexes := convertFormatStringWithSkips(lit.Value)
			skipIndices = skips
			charIndices = chars
			typeNameIndices = typeNames
			unicodeIndices = unicodes
			pointerIndices = pointers
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
				transpileFormatArg(out, call.Args[i], i-formatArgIndex-1, charIndices, typeNameIndices, unicodeIndices, pointerIndices, hexFormats)
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
	if fmtFprintfTargetIsStringsBuilder(call.Args[0]) {
		writeStringsBuilderFormattedWrite(out, call.Args[0], func() {
			writeFmtFprintlnFormatCall(out, call)
		})
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

func writeFmtFprintlnFormatCall(out *strings.Builder, call *ast.CallExpr) {
	out.WriteString("format!(")
	remaining := call.Args[1:]
	if len(remaining) == 0 {
		out.WriteString("\"\\n\"")
	} else {
		out.WriteString("\"")
		for i := range remaining {
			if i > 0 {
				out.WriteString(" ")
			}
			out.WriteString("{}")
		}
		out.WriteString("\\n\"")
		for _, arg := range remaining {
			out.WriteString(", ")
			transpilePrintArgString(out, arg)
		}
	}
	out.WriteString(")")
}

func transpileFmtFprintf(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		out.WriteString("/* ERROR: fmt.Fprintf requires at least 2 arguments */")
		return
	}
	if fmtFprintfTargetIsStringsBuilder(call.Args[0]) {
		writeStringsBuilderFormattedWrite(out, call.Args[0], func() {
			writeFmtMacroCall(out, "format!", call, 1, writeOwnedStringStdlibArg)
		})
		return
	}
	if fmtFprintfTargetIsByteWriter(call.Args[0]) {
		writeFprintfByteWriterTuple(out, call)
		return
	}
	if fmtFprintfTargetHasUserWriteMethod(call.Args[0]) {
		writeFprintfUserWriteTuple(out, call)
		return
	}
	// Check if writing to stderr
	macro := "print!"
	if isOsStderr(call.Args[0]) {
		macro = "eprint!"
	}
	writeFmtMacroCall(out, macro, call, 1, TranspileExpression)
}

func writeStringsBuilderFormattedWrite(out *strings.Builder, target ast.Expr, writeFormatted func()) {
	if isStringsBuilderReceiverBare(target) {
		writeStringsBuilderRawReceiver(out, target)
	} else {
		out.WriteString("(*")
		writeStringsBuilderReceiverHandle(out, target)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap())")
	}
	if fmtFprintfTargetIsSourceMappedStringsBuilder(target) {
		out.WriteString(".write_string(")
		WriteWrapperPrefix(out)
		writeFormatted()
		WriteWrapperSuffix(out)
		out.WriteString(")")
		return
	}
	out.WriteString(".push_str(&")
	writeFormatted()
	out.WriteString(")")
}

func fmtFprintfTargetIsStringsBuilder(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	return typeInfo != nil && isStringsBuilderReceiverType(typeInfo.GetType(expr))
}

func fmtFprintfTargetIsSourceMappedStringsBuilder(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	return typeInfo != nil && isSourceMappedStringsBuilderReceiverType(typeInfo.GetType(expr))
}

func fmtFprintfTargetIsByteWriter(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	return typeInfo != nil && isByteWriterReceiverType(typeInfo.GetType(expr)) &&
		!isSourceMappedBytesBufferReceiverType(typeInfo.GetType(expr))
}

// fmtFprintfTargetHasUserWriteMethod reports whether the Fprintf target is a
// type that satisfies io.Writer via its own Write([]byte) (int, error) method.
// The stdlib path (bytes.Buffer, io.Writer) is handled by
// fmtFprintfTargetIsByteWriter and takes priority. os.Stdout/os.Stderr are
// excluded so they continue to route through print!/eprint! at the call site.
func fmtFprintfTargetHasUserWriteMethod(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil || (isByteWriterReceiverType(typ) && !isSourceMappedBytesBufferReceiverType(typ)) {
		return false
	}
	if isOsStdoutOrStderr(expr) {
		return false
	}
	return hasByteSliceWriteMethod(typ)
}

// isOsStdoutOrStderr reports whether expr is the selector `os.Stdout` or
// `os.Stderr`. Both are *os.File and would otherwise match the user-defined
// writer path; the existing print!/eprint! lowering must take priority.
func isOsStdoutOrStderr(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok || ident.Name != "os" {
		return false
	}
	return sel.Sel.Name == "Stdout" || sel.Sel.Name == "Stderr"
}

// writeFprintfByteWriterTuple emits a block expression that formats the args,
// writes them to the target's __go_write_bytes, and returns the Go
// (int, error) shape so `n, err := fmt.Fprintf(...)` destructures correctly.
func writeFprintfByteWriterTuple(out *strings.Builder, call *ast.CallExpr) {
	TrackImport("Error")
	errorInner := externalStubErrorInnerType()
	out.WriteString("{ let __s = ")
	writeFmtMacroCall(out, "format!", call, 1, writeOwnedStringStdlibArg)
	out.WriteString("; let __n = __s.len() as i32; (*")
	TranspileExpressionContext(out, call.Args[0], LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (__n, ")
	out.WriteString(wrappedExternalStubNoneExpr(errorInner))
	out.WriteString(") }")
}

// writeFprintfUserWriteTuple emits a block expression that formats the args
// and calls the target's user-defined `write` method with the formatted bytes.
// The user's write returns the Go (int, error) shape directly, so the block
// evaluates to a value that destructures as `n, err := fmt.Fprintf(...)`.
// The user's Write has a pointer receiver and mutates state, so it is emitted
// in Rust with &mut self — the wrapped path must use borrow_mut/as_mut.
func writeFprintfUserWriteTuple(out *strings.Builder, call *ast.CallExpr) {
	TrackImport("Error")
	out.WriteString("{ let __s = ")
	writeFmtMacroCall(out, "format!", call, 1, writeOwnedStringStdlibArg)
	out.WriteString("; ")
	if fmtFprintfTargetIsBareReceiver(call.Args[0]) || isExpressionResultBare(call.Args[0]) {
		out.WriteString("(*")
		TranspileExpressionContext(out, call.Args[0], LValue)
		out.WriteString(")")
	} else {
		out.WriteString("(*")
		TranspileExpressionContext(out, call.Args[0], LValue)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap())")
	}
	out.WriteString(".write(")
	out.WriteString(wrappedExternalStubExpr("Vec<u8>", "__s.into_bytes()"))
	out.WriteString(") }")
}

// fmtFprintfTargetIsBareReceiver reports whether expr is the current method's
// receiver identifier. The Rust receiver renames to `self` which is a bare
// reference, not a wrapped handle, so `.borrow().as_ref().unwrap()` must be
// skipped.
func fmtFprintfTargetIsBareReceiver(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	return isCurrentReceiverIdent(ident)
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
		if ident, identOK := sel.X.(*ast.Ident); identOK && isCurrentReceiverIdent(ident) {
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
	var pointerIndices []int
	hexFormats := make(map[int]string)
	if len(call.Args) > 0 {
		// First arg is the format string
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust and get skip indices
			format, skips, chars, typeNames, unicodes, pointers, hexes := convertFormatStringWithSkips(lit.Value)
			skipIndices = skips
			charIndices = chars
			typeNameIndices = typeNames
			unicodeIndices = unicodes
			pointerIndices = pointers
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
				transpileFormatArg(out, call.Args[i], i-1, charIndices, typeNameIndices, unicodeIndices, pointerIndices, hexFormats)
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
	var pointerIndices []int
	hexFormats := make(map[int]string)
	if len(call.Args) > 0 {
		// First arg is the format string
		literalFormat := false
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// Convert Go format verbs to Rust
			format, skips, chars, typeNames, unicodes, pointers, hexes := convertFormatStringWithSkips(lit.Value)
			skipIndices = skips
			charIndices = chars
			typeNameIndices = typeNames
			unicodeIndices = unicodes
			pointerIndices = pointers
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
				transpileFormatArg(out, call.Args[i], i-1, charIndices, typeNameIndices, unicodeIndices, pointerIndices, hexFormats)
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

func transpileAtomicLoadUint32(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 1 {
		out.WriteString("/* ERROR: atomic.LoadUint32 requires pointer */ unimplemented!()")
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

func transpileAtomicStoreUint32(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		out.WriteString("/* ERROR: atomic.StoreUint32 requires pointer and value */ unimplemented!()")
		return
	}

	out.WriteString("{ let __target = ")
	writeAtomicTarget(out, call.Args[0])
	out.WriteString("; let __stored = ")
	writeNumericConversionValue(out, call.Args[1])
	out.WriteString(" as u32; ")
	if NeedsConcurrentWrapper() {
		out.WriteString("let mut __guard = __target.lock().unwrap();")
	} else {
		out.WriteString("let mut __guard = __target.borrow_mut();")
	}
	out.WriteString(" *__guard.as_mut().unwrap() = __stored; }")
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
		out.WriteString("; let __guard = __target.lock().unwrap(); Arc::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new((*__guard).clone())) }")
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
	writeGoReflectTypeLiteral(out, typ, st)
	WriteWrapperSuffix(out)
}

func transpileReflectValueOf(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 1 {
		out.WriteString("/* ERROR: reflect.ValueOf requires one value */ unimplemented!()")
		return
	}

	valueType, st, ok := reflectValueOfStructPointerType(call.Args[0])
	if !ok {
		NeedReflect()
		WriteWrapperPrefix(out)
		writeUnsupportedGoReflectValueLiteral(out, "reflect.ValueOf requires statically known pointer-to-struct type")
		WriteWrapperSuffix(out)
		return
	}

	NeedReflect()
	out.WriteString("{ let __reflect_target = ")
	if !writeReflectValueTargetHandle(out, call.Args[0]) {
		out.WriteString("unimplemented!(\"reflect.ValueOf requires pointer-compatible struct value\")")
	}
	out.WriteString("; ")
	WriteWrapperPrefix(out)
	writeGoReflectValueLiteral(out, valueType, st, "__reflect_target")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
}

func reflectValueOfStructPointerType(arg ast.Expr) (types.Type, *types.Struct, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil, false
	}
	typ := typeInfo.GetType(arg)
	if typ == nil {
		return nil, nil, false
	}
	ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	if !ok {
		return nil, nil, false
	}
	valueType := types.Unalias(ptr.Elem())
	st, ok := valueType.Underlying().(*types.Struct)
	if !ok {
		return nil, nil, false
	}
	return valueType, st, true
}

func writeReflectValueTargetHandle(out *strings.Builder, arg ast.Expr) bool {
	if writeAlreadyWrappedCallArgument(out, arg) {
		return true
	}
	TranspileExpression(out, arg)
	return true
}

func writeGoReflectTypeLiteral(out *strings.Builder, typ types.Type, st *types.Struct) {
	out.WriteString("GoReflectType { name: ")
	writeReflectString(out, reflectTypeName(typ))
	out.WriteString(", kind: ")
	WriteWrapperPrefix(out)
	writeGoReflectKindValue(out, typ)
	WriteWrapperSuffix(out)
	out.WriteString(", elem: ")
	writeGoReflectElemValue(out, typ)
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
}

func writeGoReflectKindValue(out *strings.Builder, typ types.Type) {
	out.WriteString("reflect_Kind(")
	out.WriteString(strconv.Itoa(int(goReflectKindForType(typ))))
	out.WriteString(")")
}

func writeGoReflectElemValue(out *strings.Builder, typ types.Type) {
	elem, ok := goReflectElemType(typ)
	if !ok {
		WriteWrapperOptionPrefix(out)
		out.WriteString("None::<Box<GoReflectType>>")
		WriteWrapperOptionSuffix(out)
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("Box::new(")
	writeGoReflectTypeLiteral(out, elem, structTypeForReflectType(elem))
	out.WriteString(")")
	WriteWrapperSuffix(out)
}

func goReflectKindForType(typ types.Type) reflect.Kind {
	if typ == nil {
		return reflect.Invalid
	}
	switch t := types.Unalias(typ).Underlying().(type) {
	case *types.Basic:
		switch t.Kind() {
		case types.Bool:
			return reflect.Bool
		case types.Int:
			return reflect.Int
		case types.Int8:
			return reflect.Int8
		case types.Int16:
			return reflect.Int16
		case types.Int32:
			return reflect.Int32
		case types.Int64:
			return reflect.Int64
		case types.Uint:
			return reflect.Uint
		case types.Uint8:
			return reflect.Uint8
		case types.Uint16:
			return reflect.Uint16
		case types.Uint32:
			return reflect.Uint32
		case types.Uint64:
			return reflect.Uint64
		case types.Uintptr:
			return reflect.Uintptr
		case types.Float32:
			return reflect.Float32
		case types.Float64:
			return reflect.Float64
		case types.Complex64:
			return reflect.Complex64
		case types.Complex128:
			return reflect.Complex128
		case types.String:
			return reflect.String
		case types.UnsafePointer:
			return reflect.UnsafePointer
		case types.UntypedBool:
			return reflect.Bool
		case types.UntypedInt:
			return reflect.Int
		case types.UntypedFloat:
			return reflect.Float64
		case types.UntypedComplex:
			return reflect.Complex128
		case types.UntypedString:
			return reflect.String
		default:
			return reflect.Invalid
		}
	case *types.Array:
		return reflect.Array
	case *types.Chan:
		return reflect.Chan
	case *types.Signature:
		return reflect.Func
	case *types.Interface:
		return reflect.Interface
	case *types.Map:
		return reflect.Map
	case *types.Pointer:
		return reflect.Pointer
	case *types.Slice:
		return reflect.Slice
	case *types.Struct:
		return reflect.Struct
	default:
		return reflect.Invalid
	}
}

func goReflectElemType(typ types.Type) (types.Type, bool) {
	if typ == nil {
		return nil, false
	}
	switch t := types.Unalias(typ).Underlying().(type) {
	case *types.Array:
		return t.Elem(), true
	case *types.Chan:
		return t.Elem(), true
	case *types.Map:
		return t.Elem(), true
	case *types.Pointer:
		return t.Elem(), true
	case *types.Slice:
		return t.Elem(), true
	default:
		return nil, false
	}
}

func structTypeForReflectType(typ types.Type) *types.Struct {
	if typ == nil {
		return nil
	}
	st, _ := types.Unalias(typ).Underlying().(*types.Struct)
	return st
}

func writeGoReflectValueLiteral(out *strings.Builder, typ types.Type, st *types.Struct, targetName string) {
	out.WriteString("GoReflectValue { typ: ")
	WriteWrapperPrefix(out)
	writeGoReflectTypeLiteral(out, typ, st)
	WriteWrapperSuffix(out)
	out.WriteString(", fields: ")
	WriteWrapperPrefix(out)
	out.WriteString("vec![")
	if st != nil {
		for i := 0; i < st.NumFields(); i++ {
			if i > 0 {
				out.WriteString(", ")
			}
			writeGoReflectFieldValueLiteral(out, st.Field(i), targetName)
		}
	}
	out.WriteString("]")
	WriteWrapperSuffix(out)
	out.WriteString(", bool_getter: ")
	WriteWrappedNone(out)
	out.WriteString(", bool_setter: ")
	WriteWrappedNone(out)
	out.WriteString(", unsupported: None")
	out.WriteString(" }")
}

func writeUnsupportedGoReflectValueLiteral(out *strings.Builder, message string) {
	out.WriteString("GoReflectValue { typ: ")
	WriteWrapperPrefix(out)
	writeGoReflectTypeLiteral(out, nil, nil)
	WriteWrapperSuffix(out)
	out.WriteString(", fields: ")
	WriteWrapperPrefix(out)
	out.WriteString("vec![]")
	WriteWrapperSuffix(out)
	out.WriteString(", bool_getter: ")
	WriteWrappedNone(out)
	out.WriteString(", bool_setter: ")
	WriteWrappedNone(out)
	out.WriteString(", unsupported: Some(")
	out.WriteString(strconv.Quote(message))
	out.WriteString(")")
	out.WriteString(" }")
}

func writeGoReflectFieldValueLiteral(out *strings.Builder, field *types.Var, targetName string) {
	fieldType := types.Unalias(field.Type())
	fieldStruct, _ := fieldType.Underlying().(*types.Struct)
	out.WriteString("GoReflectValue { typ: ")
	WriteWrapperPrefix(out)
	writeGoReflectTypeLiteral(out, fieldType, fieldStruct)
	WriteWrapperSuffix(out)
	out.WriteString(", fields: ")
	WriteWrapperPrefix(out)
	out.WriteString("vec![]")
	WriteWrapperSuffix(out)
	out.WriteString(", bool_getter: ")
	if reflectFieldIsBool(field) {
		writeGoReflectBoolGetter(out, targetName, ToSnakeCase(field.Name()))
	} else {
		WriteWrappedNone(out)
	}
	out.WriteString(", bool_setter: ")
	if reflectFieldIsBool(field) && field.Exported() {
		writeGoReflectBoolSetter(out, targetName, ToSnakeCase(field.Name()))
	} else {
		WriteWrappedNone(out)
	}
	out.WriteString(", unsupported: None")
	out.WriteString(" }")
}

func reflectFieldIsBool(field *types.Var) bool {
	basic, ok := types.Unalias(field.Type()).Underlying().(*types.Basic)
	return ok && basic.Kind() == types.Bool
}

func writeGoReflectBoolGetter(out *strings.Builder, targetName string, rustField string) {
	WriteWrapperPrefix(out)
	out.WriteString("{ let __field_target = ")
	out.WriteString(targetName)
	out.WriteString(".clone(); Box::new(move || -> bool { let __target_guard = __field_target")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __target_value = __target_guard.as_ref().expect(\"reflect.Value.Bool requires a struct value\"); let __field_value = { let __field_guard = __target_value.")
	out.WriteString(rustField)
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__field_guard.as_ref().unwrap()).clone() }; __field_value }) as GoReflectBoolGetter }")
	WriteWrapperSuffix(out)
}

func writeGoReflectBoolSetter(out *strings.Builder, targetName string, rustField string) {
	WriteWrapperPrefix(out)
	out.WriteString("{ let __field_target = ")
	out.WriteString(targetName)
	out.WriteString(".clone(); Box::new(move |__value: ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("<")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("<Option<bool>>>| { let __new_value = (*__value")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone(); let mut __target_guard = __field_target")
	WriteBorrowMethod(out, true)
	out.WriteString("; let __target_value = __target_guard.as_mut().expect(\"reflect.Value.SetBool requires a settable struct value\"); *__target_value.")
	out.WriteString(rustField)
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(__new_value); }) as GoReflectBoolSetter }")
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
		// A constant string-typed type conversion (e.g. string(rune)) lowers to
		// a wrapped value via TranspileExpression, but this helper must yield a
		// raw owned String so the result is usable as a Rust str Pattern (&str).
		// Unwrap such conversions; other constants (literals, const idents,
		// const arithmetic) already lower to a raw string.
		if call, ok := arg.(*ast.CallExpr); ok {
			if ti := GetTypeInfo(); ti != nil && ti.IsTypeConversion(call) && ti.IsString(arg) && typeConversionEmitsWrappedValue(call) {
				out.WriteString("(*")
				TranspileExpression(out, arg)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
				return
			}
		}
		TranspileExpression(out, arg)
		return
	}
	if writeRangeStringValue(out, arg) {
		return
	}
	if paren, ok := arg.(*ast.ParenExpr); ok {
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsString(arg) {
			switch paren.X.(type) {
			case *ast.BinaryExpr, *ast.IndexExpr:
				TranspileExpression(out, arg)
				return
			}
			if isExpressionResultBare(paren.X) {
				TranspileExpression(out, arg)
				return
			}
		}
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
	// The Go signature returns `(string, string, bool)` — the bool slot is a
	// predeclared Copy scalar, so emit it bare to match the widened shape.
	out.WriteString(", true) } else { (")
	WriteWrapperPrefix(out)
	out.WriteString("__s")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	WriteWrapperPrefix(out)
	out.WriteString("String::new()")
	WriteWrapperSuffix(out)
	out.WriteString(", false) } }")
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

func transpileSortSearch(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	out.WriteString("{ let mut __sort_i: i32 = 0; let mut __sort_j: i32 = (")
	TranspileExpression(out, call.Args[0])
	out.WriteString(") as i32; let __sort_pred = ")
	if !writeFunctionValueHandle(out, call.Args[1]) {
		TranspileExpression(out, call.Args[1])
	}
	out.WriteString("; while __sort_i < __sort_j { let __sort_h = (((__sort_i as u32 + __sort_j as u32) >> 1) as i32); let __sort_ok = { let mut __pred_guard = __sort_pred")
	WriteBorrowMethod(out, true)
	out.WriteString("; let __pred = __pred_guard.as_mut().expect(\"sort.Search predicate is nil\"); __pred(")
	writeSortIndexArg(out, "__sort_h")
	out.WriteString(") }; if !__sort_ok { __sort_i = __sort_h + 1; } else { __sort_j = __sort_h; } } __sort_i }")
}

func transpileSortFind(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	out.WriteString("{ let __sort_n: i32 = (")
	TranspileExpression(out, call.Args[0])
	out.WriteString(") as i32; let mut __sort_i: i32 = 0; let mut __sort_j: i32 = __sort_n; let __sort_cmp = ")
	if !writeFunctionValueHandle(out, call.Args[1]) {
		TranspileExpression(out, call.Args[1])
	}
	out.WriteString("; while __sort_i < __sort_j { let __sort_h = (((__sort_i as u32 + __sort_j as u32) >> 1) as i32); let __sort_order = { let mut __cmp_guard = __sort_cmp")
	WriteBorrowMethod(out, true)
	out.WriteString("; let __cmp = __cmp_guard.as_mut().expect(\"sort.Find comparator is nil\"); __cmp(")
	writeSortIndexArg(out, "__sort_h")
	out.WriteString(") }; if __sort_order > 0 { __sort_i = __sort_h + 1; } else { __sort_j = __sort_h; } } let __sort_found = __sort_i < __sort_n && { let mut __cmp_guard = __sort_cmp")
	WriteBorrowMethod(out, true)
	out.WriteString("; let __cmp = __cmp_guard.as_mut().expect(\"sort.Find comparator is nil\"); __cmp(")
	writeSortIndexArg(out, "__sort_i")
	out.WriteString(") == 0 }; (__sort_i, __sort_found) }")
}

func transpileSortInterfaceSort(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 1 {
		return
	}
	arg := call.Args[0]
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.GetType(arg) == nil {
		out.WriteString("/* ERROR: Type information required for sort.Sort */ unimplemented!(\"type info required for sort.Sort\")")
		return
	}
	out.WriteString("{ let mut __sort_data = ")
	if !writeSortInterfaceReceiverValue(out, arg, typeInfo.GetType(arg)) {
		TranspileExpressionContext(out, arg, LValue)
	}
	out.WriteString("; let __sort_len = __sort_data.len(); for __sort_i in 1..(__sort_len as usize) { let mut __sort_j = __sort_i as i32; while __sort_j > 0 { if !__sort_data.less(")
	writeSortInterfaceMethodArgs(out, []string{"__sort_j", "__sort_j - 1"})
	out.WriteString(") { break; } __sort_data.swap(")
	writeSortInterfaceMethodArgs(out, []string{"__sort_j", "__sort_j - 1"})
	out.WriteString("); __sort_j -= 1; } } }")
}

func transpileSortSlice(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	out.WriteString("{ let __sort_target = ")
	TranspileExpressionContext(out, call.Args[0], LValue)
	out.WriteString(".clone(); let __sort_less = ")
	if !writeFunctionValueHandle(out, call.Args[1]) {
		TranspileExpression(out, call.Args[1])
	}
	out.WriteString("; let __sort_len = { let __sort_guard = __sort_target")
	WriteBorrowMethod(out, false)
	out.WriteString("; __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; ")
	out.WriteString("for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less")
	WriteBorrowMethod(out, true)
	out.WriteString("; let __less = __less_guard.as_mut().expect(\"sort.Slice less function is nil\"); __less(")
	writeSortIndexArg(out, "__sort_j as i32")
	out.WriteString(", ")
	writeSortIndexArg(out, "(__sort_j - 1) as i32")
	out.WriteString(") }; if !__should_swap { break; } { let mut __sort_guard = __sort_target")
	WriteBorrowMethod(out, true)
	out.WriteString("; if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } }")
}

func writeSortInterfaceReceiverValue(out *strings.Builder, arg ast.Expr, typ types.Type) bool {
	named, ok := sortInterfaceNamedReceiverType(typ)
	if !ok {
		return false
	}
	rustType := goTypesTypeToRust(named)
	if sortInterfaceArgEmitsBareNamedValue(arg, named) {
		TranspileExpression(out, arg)
		return true
	}
	if sortInterfaceArgExposesNamedInnerHandle(arg, named) {
		out.WriteString(rustType)
		out.WriteString("(")
		TranspileExpressionContext(out, arg, LValue)
		out.WriteString(".clone())")
		return true
	}
	out.WriteString("(*")
	TranspileExpressionContext(out, arg, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
	return true
}

func sortInterfaceNamedReceiverType(typ types.Type) (*types.Named, bool) {
	if typ == nil {
		return nil, false
	}
	unaliased := types.Unalias(typ)
	if ptr, ok := unaliased.(*types.Pointer); ok {
		if named, ok := types.Unalias(ptr.Elem()).(*types.Named); ok {
			return named, true
		}
		return nil, false
	}
	if named, ok := unaliased.(*types.Named); ok {
		return named, true
	}
	return nil, false
}

func sortInterfaceArgEmitsBareNamedValue(arg ast.Expr, named *types.Named) bool {
	expr := unwrapParens(arg)
	if named == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil || !types.Identical(types.Unalias(typ), types.Unalias(named)) {
		return false
	}
	if call, ok := expr.(*ast.CallExpr); ok && typeInfo.IsTypeConversion(call) && !typeConversionEmitsWrappedValue(call) {
		return true
	}
	if _, ok := expr.(*ast.CompositeLit); !ok {
		return false
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Struct, *types.Array, *types.Slice:
		return true
	default:
		return false
	}
}

func sortInterfaceArgExposesNamedInnerHandle(arg ast.Expr, named *types.Named) bool {
	ident, ok := unwrapParens(arg).(*ast.Ident)
	if !ok || !isCurrentReceiverIdent(ident) || named == nil || named.Obj() == nil {
		return false
	}
	if currentReceiverType != named.Obj().Name() {
		return false
	}
	_, isTypeDef := LookupTypeDefinition(currentReceiverType)
	return isTypeDef
}

func writeSortInterfaceMethodArgs(out *strings.Builder, args []string) {
	for i, arg := range args {
		if i > 0 {
			out.WriteString(", ")
		}
		writeSortIndexArg(out, arg)
	}
}

func writeSortIndexArg(out *strings.Builder, value string) {
	WriteWrapperPrefix(out)
	out.WriteString(value)
	WriteWrapperSuffix(out)
}

func transpileNilSafeSort(out *strings.Builder, arg ast.Expr) {
	out.WriteString("{ let __sort_target = ")
	TranspileExpressionContext(out, arg, LValue)
	out.WriteString(".clone(); let mut __sort_guard = __sort_target")
	WriteBorrowMethod(out, true)
	out.WriteString("; if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } }")
}

func writeSortFuncWrappedElement(out *strings.Builder, name string, elemIsInterface, elemHasInherentWrapper bool) {
	if elemIsInterface {
		if elemHasInherentWrapper {
			// Local named interface slices wrap each element as
			// Rc<RefCell<Option<Box<dyn Trait>>>>; the closure now takes the
			// wrapped handle directly, so clone it through the iter ref.
			out.WriteString(name)
			out.WriteString(".clone()")
			return
		}
		out.WriteString(name)
		out.WriteString(".as_ref()")
		return
	}
	if elemHasInherentWrapper {
		// Element is already in wrapper shape (pointer/slice/map/chan/fn);
		// cloning the iter ref yields the closure's parameter type directly.
		out.WriteString(name)
		out.WriteString(".clone()")
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString(name)
	out.WriteString(".clone()")
	WriteWrapperSuffix(out)
}

func writeSortFuncComparatorCall(out *strings.Builder, cmp ast.Expr, elemIsInterface, elemHasInherentWrapper bool) {
	if writeDirectFunctionReference(out, cmp) {
		out.WriteString("(")
		writeSortFuncWrappedElement(out, "__a", elemIsInterface, elemHasInherentWrapper)
		out.WriteString(", ")
		writeSortFuncWrappedElement(out, "__b", elemIsInterface, elemHasInherentWrapper)
		out.WriteString(")")
		return
	}

	out.WriteString("{ let mut __cmp_guard = __cmp_holder")
	WriteBorrowMethod(out, true)
	out.WriteString("; let __cmp_fn = __cmp_guard.as_mut().unwrap(); (*__cmp_fn)(")
	writeSortFuncWrappedElement(out, "__a", elemIsInterface, elemHasInherentWrapper)
	out.WriteString(", ")
	writeSortFuncWrappedElement(out, "__b", elemIsInterface, elemHasInherentWrapper)
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

	elemIsInterface := false
	elemHasInherentWrapper := false
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if _, ok := localInterfaceSliceElemName(typeInfo.GetType(call.Args[0])); ok {
			elemIsInterface = true
		}
		if elemType := typeInfo.GetSliceElemType(call.Args[0]); elemType != nil {
			elemHasInherentWrapper = TypeHasInherentWrapper(elemType)
		}
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
		writeSortFuncComparatorCall(out, call.Args[1], elemIsInterface, elemHasInherentWrapper)
	} else {
		out.WriteString(direct.String())
		out.WriteString("(")
		writeSortFuncWrappedElement(out, "__a", elemIsInterface, elemHasInherentWrapper)
		out.WriteString(", ")
		writeSortFuncWrappedElement(out, "__b", elemIsInterface, elemHasInherentWrapper)
		out.WriteString(")")
	}
	// Comparator returns a bare Go int (i32) under the widened scalar return
	// rule, so the result is already a primitive and `.cmp(&0)` applies to it
	// directly without a wrap/unwrap dance.
	out.WriteString("; let __ord = __cmp.cmp(&0); __ord }); } }")
}

func transpileSlicesContains(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) < 2 {
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if writeSlicesContainsPointerIdentity(out, call, typeInfo) {
			return
		}
		if elemName, ok := localInterfaceSliceElemName(typeInfo.GetType(call.Args[0])); ok {
			elemSnake := traitMethodSuffix(elemName)
			WriteWrapperPrefix(out)
			out.WriteString("{ let __slice_holder = ")
			TranspileExpressionContext(out, call.Args[0], LValue)
			out.WriteString(".clone(); let __slice_guard = __slice_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __slice = __slice_guard.as_ref().unwrap(); ")
			if ident, ok := call.Args[1].(*ast.Ident); ok && ident.Name == "nil" {
				out.WriteString("__slice.iter().any(|__item| { let __item_guard = __item")
				WriteBorrowMethod(out, false)
				out.WriteString("; __item_guard.as_ref().is_none() }) }")
			} else if isBareLocalInterfaceValue(call.Args[1]) {
				out.WriteString("let __value = ")
				TranspileExpression(out, call.Args[1])
				out.WriteString("; __slice.iter().any(|__item| { let __item_guard = __item")
				WriteBorrowMethod(out, false)
				out.WriteString("; if let Some(__left) = __item_guard.as_ref() { __left.as_ref().__go_eq_")
				out.WriteString(elemSnake)
				out.WriteString("(__value) } else { false } }) }")
			} else {
				out.WriteString("let __value_holder = ")
				TranspileExpressionContext(out, call.Args[1], LValue)
				out.WriteString(".clone(); let __value_guard = __value_holder")
				WriteBorrowMethod(out, false)
				out.WriteString("; let __value_option = __value_guard.as_ref().map(|__value| __value.clone()); drop(__value_guard); __slice.iter().any(|__item| { let __item_guard = __item")
				WriteBorrowMethod(out, false)
				out.WriteString("; match (__item_guard.as_ref(), __value_option.as_ref()) { (Some(__left), Some(__right)) => __left.as_ref().__go_eq_")
				out.WriteString(elemSnake)
				out.WriteString("(__right.as_ref()), (None, None) => true, _ => false } }) }")
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

func writeSlicesContainsPointerIdentity(out *strings.Builder, call *ast.CallExpr, typeInfo *TypeInfo) bool {
	if len(call.Args) < 2 || typeInfo == nil {
		return false
	}
	sliceType := typeInfo.GetType(call.Args[0])
	valueType := typeInfo.GetType(call.Args[1])
	if sliceType == nil || valueType == nil {
		return false
	}
	slice, ok := types.Unalias(sliceType).Underlying().(*types.Slice)
	if !ok {
		return false
	}
	if _, ok := types.Unalias(slice.Elem()).(*types.Pointer); !ok {
		return false
	}
	if _, ok := types.Unalias(valueType).(*types.Pointer); !ok || !types.AssignableTo(valueType, slice.Elem()) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __slice_holder = ")
	TranspileExpressionContext(out, call.Args[0], LValue)
	out.WriteString(".clone(); let __slice_guard = __slice_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __slice = __slice_guard.as_ref().unwrap(); let __value = ")
	writePointerHandleExpression(out, call.Args[1])
	out.WriteString("; __slice.iter().any(|__item| { let __both_nil = (*__item")
	WriteBorrowMethod(out, false)
	out.WriteString(").is_none() && (*__value")
	WriteBorrowMethod(out, false)
	out.WriteString(").is_none(); __both_nil || ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::ptr_eq(__item, &__value) }) }")
	WriteWrapperSuffix(out)
	return true
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

func transpileSlicesDelete(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 3 {
		out.WriteString("/* ERROR: slices.Delete expects 3 arguments */")
		return
	}
	out.WriteString("{ let __slice_holder = ")
	TranspileExpressionContext(out, call.Args[0], LValue)
	out.WriteString(".clone(); let mut __slice_guard = __slice_holder")
	WriteBorrowMethod(out, true)
	out.WriteString("; let __slice = __slice_guard.get_or_insert_with(Vec::new); let __i = ")
	writeExpressionAsUsize(out, call.Args[1])
	out.WriteString("; let __j = ")
	writeExpressionAsUsize(out, call.Args[2])
	out.WriteString("; if __i <= __j && __j <= __slice.len() { __slice.drain(__i..__j); } __slice_holder.clone() }")
}

func transpileStrconvItoa(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		WriteWrapperPrefix(out)
		out.WriteString("(")
		writeNumericConversionValue(out, call.Args[0])
		out.WriteString(").to_string()")
		WriteWrapperSuffix(out)
	}
}

func transpileStrconvAtoi(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		// strconv.Atoi returns (int, error). The int slot is a predeclared
		// Copy scalar, so it lowers to a bare i32 at the boundary; the error
		// slot keeps the wrapped representation.
		out.WriteString("{ let __atoi_input = ")
		writeStringSequenceValue(out, call.Args[0])
		out.WriteString("; match __atoi_input.parse::<i32>() { ")
		out.WriteString("Ok(n) => (n, ")
		WriteWrappedNone(out)
		out.WriteString("), ")
		TrackImport("Error")
		out.WriteString("Err(_) => (0 as i32, ")
		WriteWrapperPrefix(out)
		if NeedsConcurrentWrapper() {
			out.WriteString("Box::<dyn StdError + Send + Sync>::from(format!(\"strconv.Atoi: parsing \\\"{}\\\": invalid syntax\", __atoi_input)))))) } }")
		} else {
			out.WriteString("Box::<dyn StdError>::from(format!(\"strconv.Atoi: parsing \\\"{}\\\": invalid syntax\", __atoi_input)))))) } }")
		}
	}
}

func transpileStrconvParseBool(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) > 0 {
		out.WriteString("{ let __parse_bool_input = ")
		writeStringSequenceValue(out, call.Args[0])
		out.WriteString("; match __parse_bool_input.as_str() { ")
		out.WriteString("\"1\" | \"t\" | \"T\" | \"TRUE\" | \"true\" | \"True\" => (true, ")
		WriteWrappedNone(out)
		out.WriteString("), ")
		out.WriteString("\"0\" | \"f\" | \"F\" | \"FALSE\" | \"false\" | \"False\" => (false, ")
		WriteWrappedNone(out)
		out.WriteString("), ")
		TrackImport("Error")
		out.WriteString("_ => (false, ")
		WriteWrapperPrefix(out)
		if NeedsConcurrentWrapper() {
			out.WriteString("Box::<dyn StdError + Send + Sync>::from(format!(\"strconv.ParseBool: parsing \\\"{}\\\": invalid syntax\", __parse_bool_input)))))) } }")
		} else {
			out.WriteString("Box::<dyn StdError>::from(format!(\"strconv.ParseBool: parsing \\\"{}\\\": invalid syntax\", __parse_bool_input)))))) } }")
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
	basicName string
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

func jsonMarshalBasicKindName(kind types.BasicKind) string {
	switch kind {
	case types.Bool:
		return "bool"
	case types.Int:
		return "int"
	case types.Int8:
		return "int8"
	case types.Int16:
		return "int16"
	case types.Int32:
		return "int32"
	case types.Int64:
		return "int64"
	case types.Uint:
		return "uint"
	case types.Uint8:
		return "uint8"
	case types.Uint16:
		return "uint16"
	case types.Uint32:
		return "uint32"
	case types.Uint64:
		return "uint64"
	case types.Uintptr:
		return "uintptr"
	case types.Float32:
		return "float32"
	case types.Float64:
		return "float64"
	case types.String:
		return "string"
	default:
		return ""
	}
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
					basicName: jsonMarshalBasicKindName(basic.Kind()),
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

func jsonStructTagFromField(field *ast.Field) string {
	if field == nil || field.Tag == nil {
		return ""
	}
	tag, err := strconv.Unquote(field.Tag.Value)
	if err != nil {
		return ""
	}
	return tag
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
	switch field.basicName {
	case "string":
		if field.named {
			writeJsonNamedClosureValueRef(out)
			out.WriteString(".is_empty()")
		} else {
			out.WriteString("__v.is_empty()")
		}
	case "bool":
		out.WriteString("!*")
		if field.named {
			writeJsonNamedClosureValueRef(out)
		} else {
			out.WriteString("__v")
		}
	case "float32", "float64":
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
	if field.basicName == "string" {
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

	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("unimplemented!(\"type info required for json.Marshal\")")
		return
	}
	st := typeInfo.GetStructType(call.Args[0])
	if st == nil {
		out.WriteString("unimplemented!(\"type info required for json.Marshal\")")
		return
	}
	fields, ok := jsonMarshalStructFields(st)
	if !ok {
		out.WriteString("/* ERROR: json.Marshal currently supports exported basic, []string, map[string]string, and map[string][]byte struct fields */ unimplemented!()")
		return
	}

	for _, field := range fields {
		if field.kind == jsonMarshalStringMapField || field.kind == jsonMarshalStringSliceField || field.kind == jsonMarshalByteSliceMapField || field.basicName == "string" {
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
		out.WriteString("unimplemented!(\"type info required for unsafe.")
		out.WriteString(goFunc)
		out.WriteString("\")")
		WriteWrapperSuffix(out)
		return
	}

	argType := typeInfo.GetType(call.Args[0])
	if argType == nil {
		out.WriteString("unimplemented!(\"type info required for unsafe.")
		out.WriteString(goFunc)
		out.WriteString("\")")
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

func transpileUnsafeAdd(out *strings.Builder, call *ast.CallExpr) {
	transpileUnsupportedUnsafeIntrinsic(out, call, "Add")
}

func transpileUnsafeSlice(out *strings.Builder, call *ast.CallExpr) {
	transpileUnsupportedUnsafeIntrinsic(out, call, "Slice")
}

func transpileUnsafeString(out *strings.Builder, call *ast.CallExpr) {
	if writeUnsafeStringFromByteSliceAddress(out, call) {
		return
	}
	transpileUnsupportedUnsafeIntrinsic(out, call, "String")
}

func transpileUnsafeSliceData(out *strings.Builder, call *ast.CallExpr) {
	transpileUnsupportedUnsafeIntrinsic(out, call, "SliceData")
}

func transpileUnsafeStringData(out *strings.Builder, call *ast.CallExpr) {
	transpileUnsupportedUnsafeIntrinsic(out, call, "StringData")
}

func transpileUnsupportedUnsafeIntrinsic(out *strings.Builder, call *ast.CallExpr, goFunc string) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		WriteWrapperPrefix(out)
		out.WriteString("unimplemented!(\"type info required for unsafe.")
		out.WriteString(goFunc)
		out.WriteString("\")")
		WriteWrapperSuffix(out)
		return
	}
	resultType := typeInfo.GetType(call)
	if resultType == nil {
		WriteWrapperPrefix(out)
		out.WriteString("unimplemented!(\"type info required for unsafe.")
		out.WriteString(goFunc)
		out.WriteString("\")")
		WriteWrapperSuffix(out)
		return
	}
	out.WriteString("{ let __go_unsafe_result: ")
	out.WriteString(goTypesReturnTypeToRust(resultType))
	out.WriteString(" = unimplemented!(\"unsafe.")
	out.WriteString(goFunc)
	out.WriteString(" requires unsafe intrinsic support\"); __go_unsafe_result }")
}

func writeUnsafeStringFromByteSliceAddress(out *strings.Builder, call *ast.CallExpr) bool {
	if len(call.Args) != 2 {
		return false
	}
	indexExpr, ok := addressOfIndexExpr(call.Args[0])
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		WriteWrapperPrefix(out)
		out.WriteString(`unimplemented!("type info required for unsafe.String byte-slice address")`)
		WriteWrapperSuffix(out)
		return true
	}
	seqType := typeInfo.GetType(indexExpr.X)
	if seqType == nil {
		WriteWrapperPrefix(out)
		out.WriteString(`unimplemented!("type info required for unsafe.String byte-slice address")`)
		WriteWrapperSuffix(out)
		return true
	}
	sliceType, ok := types.Unalias(seqType).Underlying().(*types.Slice)
	if !ok || !isByteType(sliceType.Elem()) {
		return false
	}

	WriteWrapperPrefix(out)
	out.WriteString("{ let __bytes_holder = ")
	TranspileExpressionContext(out, indexExpr.X, LValue)
	out.WriteString(".clone(); let __bytes_guard = __bytes_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __bytes = __bytes_guard.as_ref().unwrap(); let __start = ")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("; let __len = ")
	if unsafeStringLengthUsesBorrowedSlice(call.Args[1], indexExpr.X) {
		out.WriteString("__bytes.len()")
	} else {
		writeExpressionAsUsize(out, call.Args[1])
	}
	out.WriteString("; let __end = __start + __len; String::from_utf8(__bytes[__start..__end].to_vec()).unwrap() }")
	WriteWrapperSuffix(out)
	return true
}

func unsafeStringLengthUsesBorrowedSlice(length ast.Expr, slice ast.Expr) bool {
	call, ok := unwrapParens(length).(*ast.CallExpr)
	if !ok || len(call.Args) != 1 || !isBareBuiltinCallName(call, "len") {
		return false
	}
	return sameExpressionSyntax(call.Args[0], slice)
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

func appendPointerToSliceDerefTarget(expr ast.Expr) (*ast.StarExpr, bool) {
	star, ok := unwrapParens(expr).(*ast.StarExpr)
	if !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	if _, ok := coreUnderlyingType(typeInfo.GetType(star)).(*types.Slice); !ok {
		return nil, false
	}
	ptr, ok := types.Unalias(typeInfo.GetType(star.X)).Underlying().(*types.Pointer)
	if !ok {
		return nil, false
	}
	if _, ok := coreUnderlyingType(ptr.Elem()).(*types.Slice); !ok {
		return nil, false
	}
	return star, true
}

func writeNilSliceAppendTarget(out *strings.Builder, expr ast.Expr) bool {
	call, ok := expr.(*ast.CallExpr)
	if !ok || len(call.Args) != 1 {
		return false
	}
	ident, ok := call.Args[0].(*ast.Ident)
	if !ok || ident.Name != "nil" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return false
	}
	typ := typeInfo.GetType(call)
	if typ == nil {
		return false
	}
	if _, ok := types.Unalias(typ).Underlying().(*types.Slice); !ok {
		return false
	}
	WriteWrappedNone(out)
	return true
}

func writeConcreteLocalInterfaceValue(out *strings.Builder, expr ast.Expr, expected types.Type, ifaceName string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expected == nil || expr == nil {
		return false
	}
	sourceType := typeInfo.GetType(expr)
	if sourceType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(sourceType); ok {
		return false
	}
	targetNamed, ok := types.Unalias(expected).(*types.Named)
	if !ok {
		return false
	}
	targetInterface, ok := targetNamed.Underlying().(*types.Interface)
	if !ok {
		return false
	}
	targetInterface.Complete()
	if !types.Implements(sourceType, targetInterface) {
		return false
	}
	out.WriteString("Box::new(")
	if funcTypeName, ok := functionTypeAliasNameFromTypes(sourceType); ok {
		out.WriteString(functionTypeInterfaceWrapperName(funcTypeName, ifaceName))
		out.WriteString("(")
		writeFunctionTypeAliasInnerValue(out, expr)
		out.WriteString(")")
	} else if writePointerLocalInterfaceWrapperValue(out, expr, expected, ifaceName) {
		// Pointer dynamic values compare by handle identity when stored in an interface.
	} else if writeCurrentPackagePointerTranspiledInterfaceWrapperValue(out, expr, expected) {
		// Current-package pointers implementing source-mapped interfaces need the same identity wrapper.
	} else if writeSourceMappedPointerInterfaceWrapperValue(out, expr, expected) {
		// Imported source-mapped pointer dynamic values must keep their *T identity.
	} else if ident, ok := expr.(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		out.WriteString(currentReceiverRustName())
		out.WriteString(".clone()")
	} else if globalIdent, ok := packageGlobalPointerIdent(expr); ok {
		writePackageGlobalPointerPointeeClone(out, globalIdent)
	} else if ident, ok := expr.(*ast.Ident); ok && ident.Name != "_" && ident.Name != "nil" && !isVarBare(ident.Name) {
		out.WriteString("(*")
		TranspileExpressionContext(out, expr, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
	} else if typeInfo.ReturnsWrappedValue(expr) || typeInfo.IsPointer(expr) {
		out.WriteString("(*")
		TranspileExpressionContext(out, expr, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
	} else {
		TranspileExpression(out, expr)
	}
	out.WriteString(") as ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	return true
}

func writeLocalInterfaceHandleClone(out *strings.Builder, expr ast.Expr) {
	if ident, ok := expr.(*ast.Ident); ok {
		name := RustIdentForUse(ident)
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[ident.Name]; exists {
				name = RustLocalIdent(renamed)
			}
		}
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar &&
			(strings.HasPrefix(varType, "&Rc<") || strings.HasPrefix(varType, "&Arc<")) {
			out.WriteString("(*")
			out.WriteString(name)
			out.WriteString(").clone()")
			return
		}
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone()")
}

func writeLocalInterfaceWrappedValue(out *strings.Builder, expr ast.Expr, expectedType types.Type) bool {
	if expectedType == nil {
		return false
	}
	ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(expectedType)
	if !ok {
		return false
	}
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return true
	}
	if isBareLocalInterfaceValue(expr) {
		WriteWrapperPrefix(out)
		writeLocalInterfaceBareClone(out, expr)
		WriteWrapperSuffix(out)
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	exprType := typeInfo.GetType(expr)
	if exprType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(exprType); ok {
		writeLocalInterfaceHandleClone(out, expr)
		return true
	}
	if types.AssignableTo(exprType, expectedType) {
		WriteWrapperPrefix(out)
		if !writeConcreteLocalInterfaceValue(out, expr, expectedType, ifaceName) {
			return false
		}
		WriteWrapperSuffix(out)
		return true
	}
	return false
}

func writeLocalInterfaceSliceElementValue(out *strings.Builder, expr ast.Expr, elemType types.Type) bool {
	return writeLocalInterfaceWrappedValue(out, expr, elemType)
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
				// A nil element whose slot is a bare (slice) value — e.g.
				// append([][]T, nil), where the inner []T is stored as a raw Vec —
				// is the element's zero value, not a wrapped None to deref.
				if nilIdent, ok := expr.(*ast.Ident); ok && nilIdent.Name == "nil" && elemType != nil {
					if _, isSlice := types.Unalias(elemType).Underlying().(*types.Slice); isSlice {
						out.WriteString("Default::default()")
						return
					}
					if _, isPointer := types.Unalias(elemType).Underlying().(*types.Pointer); isPointer {
						WriteWrappedNone(out)
						return
					}
				}
				if isGoErrorType(elemType) && writeGoErrorHandleValue(out, expr) {
					return
				}
				if writeNilStdlibInterfaceBareValue(out, expr, elemType) {
					return
				}
				if isEmptyInterfaceType(elemType) && !isEmptyInterfaceValueExpr(expr) {
					if nilIdent, ok := expr.(*ast.Ident); !ok || nilIdent.Name != "nil" {
						writeBareAnyBox(out, expr)
						return
					}
				}
				if writeStdlibInterfaceBareConversion(out, expr, elemType) {
					return
				}
				if elemType != nil {
					if _, ok := types.Unalias(elemType).Underlying().(*types.Pointer); ok {
						if writePointerHandleCallArgument(out, expr, elemType) {
							return
						}
					}
					if _, ok := transpiledNamedInterfaceTypeNameFromTypes(elemType); ok {
						if writeLocalInterfaceSliceElementValue(out, expr, elemType) {
							return
						}
					}
					if typeIsRegisteredBareStructAlias(elemType) {
						writeBareStructAliasValue(out, expr)
						return
					}
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
					if writeLocalInterfaceSliceElementValue(out, expr, elemType) {
						return
					}
					if isFunctionSignatureType(elemType) && writeFunctionValueHandle(out, expr) {
						return
					}
					if writeLenCapCallArgumentForExpectedType(out, expr, elemType) {
						return
					}
					if writeRangeIndexForExpectedType(out, expr, elemType) {
						return
					}
					if writeRangeCharForExpectedType(out, expr, elemType) {
						return
					}
					if writeBareFixedArrayCompositeLiteral(out, expr, elemType) {
						return
					}
					if writeConstExpressionForExpectedGoType(out, expr, elemType) {
						return
					}
					if basic, ok := types.Unalias(elemType).Underlying().(*types.Basic); ok && basic.Kind() == types.String {
						if sliceExpr, ok := expr.(*ast.SliceExpr); ok && typeInfo.IsString(expr) {
							writeStringSliceValue(out, sliceExpr.X, sliceExpr.Low, sliceExpr.High)
							return
						}
						if writeRangeStringValue(out, expr) {
							return
						}
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
					if _, ok := transpiledNamedInterfaceTypeNameFromTypes(elemType); ok && isBareLocalInterfaceValue(expr) {
						writeLocalInterfaceBareClone(out, expr)
						return
					}
					if ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(elemType); ok && writeConcreteLocalInterfaceValue(out, expr, elemType, ifaceName) {
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
			if writeNilSliceAppendTarget(out, expr) {
				return
			}
			if callExpr, ok := expr.(*ast.CallExpr); ok && sourceTypeParamSliceCallReturnsConcreteSlice(callExpr) {
				writeSourceTypeParamSliceCallAsConcreteSlice(out, callExpr)
				return
			}
			if star, ok := appendPointerToSliceDerefTarget(expr); ok {
				TranspileExpressionContext(out, star.X, LValue)
				return
			}
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
			} else if typeInfo := GetTypeInfo(); typeInfo != nil && isExpressionResultBare(expr) {
				if typ := typeInfo.GetType(expr); typ != nil {
					if _, ok := types.Unalias(typ).Underlying().(*types.Slice); ok {
						TranspileExpression(out, expr)
						out.WriteString(".iter().cloned()")
						return
					}
				}
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
			writeMapHandleClone := func() {
				if isNamedMapExpression(indexExpr.X) {
					writeNamedMapInnerHandleClone(out, indexExpr.X)
					return
				}
				writeMapHandleForOp(out, indexExpr.X)
				out.WriteString(".clone()")
			}
			out.WriteString("{ let __slice = { let __map_holder = ")
			writeMapHandleClone()
			out.WriteString("; let __map_guard = __map_holder")
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
		if writeLocalInterfaceSliceElementValue(out, expr, sliceType.Elem()) {
			return
		}
		if named, ok := types.Unalias(sliceType.Elem()).(*types.Named); ok && writeNamedIntegerValueForExpected(out, expr, named) {
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

		if writeConstStringLen(out, call.Args[0]) {
			return
		}

		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.IsChannel(call.Args[0]) {
			writeChannelExpression(out, call.Args[0])
			out.WriteString(".len()")
			return
		}

		if typeInfo != nil && goTypeParamHasStringByteSliceConstraint(typeInfo.GetType(call.Args[0])) {
			writeGoByteSequenceLen(out, call.Args[0])
			return
		}

		if writeNamedSliceLen(out, call.Args[0]) {
			return
		}

		if writeNamedMapLen(out, call.Args[0]) {
			return
		}

		if writePointerDerefSliceLen(out, call.Args[0]) {
			return
		}

		if writePointerToArrayLen(out, call.Args[0]) {
			return
		}

		// len() returns the length of arrays, slices, maps, strings, or channels
		if isExpressionResultBare(call.Args[0]) {
			// Bare value (range var, index result, etc.) - access directly
			TranspileExpressionContext(out, call.Args[0], LValue)
			out.WriteString(".len()")
		} else if typeInfo != nil && (typeInfo.IsSlice(call.Args[0]) || typeInfo.IsMap(call.Args[0])) {
			if sel, ok := call.Args[0].(*ast.SelectorExpr); ok {
				out.WriteString("({ let __len_target = ")
				writeSelectorHandleClone(out, sel)
				out.WriteString("; let __len_guard = __len_target")
				WriteBorrowMethod(out, false)
				out.WriteString("; __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })")
			} else {
				out.WriteString("(*")
				TranspileExpressionContext(out, call.Args[0], LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(").as_ref().map(|__v| __v.len()).unwrap_or(0)")
			}
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

func writePointerToArrayLen(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointerToArray(expr) {
		return false
	}
	arrayType, ok := arrayTypeForExpr(expr, typeInfo)
	if !ok {
		return false
	}
	out.WriteString(strconv.FormatInt(arrayType.Len(), 10))
	return true
}

func writePointerDerefSliceLen(out *strings.Builder, expr ast.Expr) bool {
	star, ok := unwrapParens(expr).(*ast.StarExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if _, ok := types.Unalias(typeInfo.GetType(star)).Underlying().(*types.Slice); !ok {
		return false
	}
	operandType := typeInfo.GetType(star.X)
	ptr, ok := types.Unalias(operandType).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	if _, ok := types.Unalias(ptr.Elem()).Underlying().(*types.Slice); !ok {
		return false
	}
	out.WriteString("{ let __slice_holder = ")
	TranspileExpressionContext(out, star.X, LValue)
	out.WriteString(".clone(); let __slice_guard = __slice_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }")
	return true
}

func writeConstStringLen(out *strings.Builder, expr ast.Expr) bool {
	if _, ok := constStringLiteral(expr); !ok {
		return false
	}
	TranspileExpression(out, expr)
	out.WriteString(".len()")
	return true
}

func transpileMake(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 1 {
		// For named types (type Foo map[K]V), the arg is not a literal type
		// expression. Use go/types to find the underlying type and emit the
		// matching make() shape.
		if _, isChan := call.Args[0].(*ast.ChanType); !isChan {
			if _, isMap := call.Args[0].(*ast.MapType); !isMap {
				if arrayType, isArray := call.Args[0].(*ast.ArrayType); !isArray || arrayType.Len != nil {
					if writeMakeNamedType(out, call) {
						return
					}
				}
			}
		}
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
			elementRustType := goCollectionElemTypeToRust(arrayType.Elt)
			if typeInfo := GetTypeInfo(); typeInfo != nil {
				if typ := typeInfo.GetType(call.Args[0]); typ != nil {
					if sliceType, ok := types.Unalias(typ).Underlying().(*types.Slice); ok {
						elementType = zeroValueForTypesType(sliceType.Elem())
						elementRustType = goTypesCollectionElemTypeToRust(sliceType.Elem())
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
					out.WriteString("Vec::<")
					out.WriteString(elementRustType)
					out.WriteString(">::with_capacity(")
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

// writeMakeNamedType handles make(T) where T is a named type whose underlying
// type is map/slice/chan. Returns true if it emitted output. When the type is
// a *types.Named (defined type with potential methods, not an alias), the
// inner collection constructor is wrapped in the named tuple struct so the
// resulting value carries the named type identity. Aliases collapse via
// types.Unalias and emit the underlying form only.
func writeMakeNamedType(out *strings.Builder, call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(call.Args[0])
	if typ == nil {
		return false
	}
	unaliased := types.Unalias(typ)
	var namedName string
	if named, ok := unaliased.(*types.Named); ok && named.Obj() != nil {
		namedName = goTypesNamedTypeToRust(named)
	}
	if elemType, ok := goTypeParamSliceConstraintElem(unaliased); ok {
		WriteWrapperPrefix(out)
		writeSliceMakeBody(out, call.Args, zeroValueForTypesType(elemType), goTypesCollectionElemTypeToRust(elemType))
		WriteWrapperSuffix(out)
		return true
	}
	switch ut := unaliased.Underlying().(type) {
	case *types.Map:
		WriteWrapperPrefix(out)
		if namedName != "" {
			out.WriteString(namedName)
			out.WriteString("(")
			WriteWrapperPrefix(out)
		}
		TrackImport("BTreeMap")
		out.WriteString("BTreeMap::<")
		out.WriteString(goTypesMapKeyToRust(ut.Key()))
		out.WriteString(", ")
		out.WriteString(goTypesMapValueToRust(ut.Elem()))
		out.WriteString(">::new()")
		if namedName != "" {
			WriteWrapperSuffix(out)
			out.WriteString(")")
		}
		WriteWrapperSuffix(out)
		return true
	case *types.Slice:
		elementType := zeroValueForTypesType(ut.Elem())
		WriteWrapperPrefix(out)
		if namedName != "" {
			out.WriteString(namedName)
			out.WriteString("(")
			WriteWrapperPrefix(out)
		}
		writeSliceMakeBody(out, call.Args, elementType, goTypesCollectionElemTypeToRust(ut.Elem()))
		if namedName != "" {
			WriteWrapperSuffix(out)
			out.WriteString(")")
		}
		WriteWrapperSuffix(out)
		return true
	case *types.Chan:
		NeedGoChannel()
		elemType := goTypesTypeToRust(ut.Elem())
		out.WriteString("GoChannel::<")
		out.WriteString(elemType)
		if len(call.Args) > 1 {
			out.WriteString(">::new_buffered(")
			writeChannelCapacityAsUsize(out, call.Args[1])
			out.WriteString(")")
		} else {
			out.WriteString(">::new()")
		}
		return true
	}
	return false
}

// writeSliceMakeBody emits the inner Vec expression for make([]T, ...) variants.
// Caller wraps with WriteWrapperPrefix/Suffix.
func writeSliceMakeBody(out *strings.Builder, args []ast.Expr, elementType string, elementRustType string) {
	if len(args) < 2 {
		out.WriteString("Vec::new()")
		return
	}
	if lit, ok := args[1].(*ast.BasicLit); ok && lit.Value == "0" {
		out.WriteString("Vec::<")
		out.WriteString(elementRustType)
		out.WriteString(">::with_capacity(")
		if len(args) >= 3 {
			writeExpressionAsUsize(out, args[2])
		} else {
			out.WriteString("0")
		}
		out.WriteString(")")
		return
	}
	if len(args) >= 3 {
		out.WriteString("{ let mut v = Vec::with_capacity(")
		writeExpressionAsUsize(out, args[2])
		out.WriteString("); v.resize(")
		writeExpressionAsUsize(out, args[1])
		out.WriteString(", ")
		out.WriteString(elementType)
		out.WriteString("); v }")
		return
	}
	out.WriteString("vec![")
	out.WriteString(elementType)
	out.WriteString("; ")
	writeExpressionAsUsize(out, args[1])
	out.WriteString("]")
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
			if sel, ok := call.Args[0].(*ast.SelectorExpr); ok {
				out.WriteString("({ let __cap_target = ")
				writeSelectorHandleClone(out, sel)
				out.WriteString("; let __cap_guard = __cap_target")
				WriteBorrowMethod(out, false)
				out.WriteString("; __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) })")
			} else {
				out.WriteString("(*")
				TranspileExpressionContext(out, call.Args[0], LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(").as_ref().map(|__v| __v.capacity()).unwrap_or(0)")
			}
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
		if isNamedMapExpression(call.Args[0]) {
			writeNamedMapInnerHandleClone(out, call.Args[0])
		} else {
			TranspileExpressionContext(out, call.Args[0], LValue)
			out.WriteString(".clone()")
		}
		out.WriteString("; let mut __map_guard = __map_handle")
		WriteBorrowMethod(out, true)
		out.WriteString("; __map_guard.as_mut().unwrap().remove(")
		writeMapLookupKeyWithType(out, call.Args[1], keyType)
		out.WriteString("); }")
	}
}

func transpileClear(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) == 0 {
		return
	}
	arg := call.Args[0]
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("unimplemented!(\"type info required for clear\")")
		return
	}
	argType := typeInfo.GetType(arg)
	if argType == nil {
		out.WriteString("unimplemented!(\"type info required for clear\")")
		return
	}
	switch typ := types.Unalias(argType).Underlying().(type) {
	case *types.Map:
		writeClearMap(out, arg)
	case *types.Slice:
		writeClearSlice(out, arg, typ.Elem())
	default:
		out.WriteString("unimplemented!(\"clear requires map or slice type\")")
	}
}

func writeClearMap(out *strings.Builder, arg ast.Expr) {
	out.WriteString("{ let __clear_holder = ")
	if isNamedMapExpression(arg) {
		writeNamedMapInnerHandleClone(out, arg)
	} else {
		TranspileExpressionContext(out, arg, LValue)
		out.WriteString(".clone()")
	}
	out.WriteString("; let mut __clear_guard = __clear_holder")
	WriteBorrowMethod(out, true)
	out.WriteString("; if let Some(__clear_map) = __clear_guard.as_mut() { __clear_map.clear(); } }")
}

func writeClearSlice(out *strings.Builder, arg ast.Expr, elemType types.Type) {
	zeroValue := zeroValueForTypesType(elemType)
	if slice, ok := unwrapParens(arg).(*ast.SliceExpr); ok {
		writeClearSliceExpr(out, slice, zeroValue)
		return
	}
	out.WriteString("{ let __clear_holder = ")
	writeClearSliceHandle(out, arg)
	out.WriteString("; let mut __clear_guard = __clear_holder")
	WriteBorrowMethod(out, true)
	out.WriteString("; if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = ")
	out.WriteString(zeroValue)
	out.WriteString("; } } }")
}

func writeClearSliceExpr(out *strings.Builder, slice *ast.SliceExpr, zeroValue string) {
	out.WriteString("{ let __clear_start = ")
	if slice.Low != nil {
		writeExpressionAsUsize(out, slice.Low)
	} else {
		out.WriteString("0usize")
	}
	out.WriteString("; let __clear_end = ")
	if slice.High != nil {
		writeExpressionAsUsize(out, slice.High)
	} else {
		writeClearSliceLen(out, slice.X)
	}
	out.WriteString("; let __clear_holder = ")
	writeClearSliceHandle(out, slice.X)
	out.WriteString("; let mut __clear_guard = __clear_holder")
	WriteBorrowMethod(out, true)
	out.WriteString("; if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = ")
	out.WriteString(zeroValue)
	out.WriteString("; } } }")
}

func writeClearSliceHandle(out *strings.Builder, expr ast.Expr) {
	if isNamedSliceExpression(expr) {
		writeNamedSliceInnerHandleClone(out, expr)
		return
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone()")
}

func writeClearSliceLen(out *strings.Builder, expr ast.Expr) {
	if isNamedSliceExpression(expr) {
		writeNamedSliceLen(out, expr)
		return
	}
	out.WriteString("{ let __clear_len_holder = ")
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone(); let __clear_len_guard = __clear_len_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; __clear_len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }")
}

func transpileCopy(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) >= 2 {
		// Go: copy(dst, src) copies min(len(dst), len(src)) elements, returns count
		typeInfo := GetTypeInfo()
		srcIsString := typeInfo != nil && typeInfo.IsString(call.Args[1])

		if dstSlice, ok := call.Args[0].(*ast.SliceExpr); ok && writeCopyNamedSliceDestinationSlice(out, dstSlice, call.Args[1], srcIsString) {
			return
		}

		if isNamedSliceExpression(call.Args[0]) {
			out.WriteString("{ let _dst_holder = ")
			writeNamedSliceInnerHandleClone(out, call.Args[0])
			out.WriteString("; let _src = ")
			writeCopySourceValue(out, call.Args[1], srcIsString)
			out.WriteString("; let _dst_len = { let _dst_guard = _dst_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder")
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap())[_i] = _src[_i].clone(); } ")
			WriteWrapperPrefix(out)
			out.WriteString("_n as i32")
			WriteWrapperSuffix(out)
			out.WriteString(" }")
			return
		}

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

		if copyDestinationIsBareSlice(call.Args[0]) {
			out.WriteString("{ let mut _dst = ")
			TranspileExpression(out, call.Args[0])
			out.WriteString("; let _src = ")
			writeCopySourceValue(out, call.Args[1], srcIsString)
			out.WriteString("; let _n = std::cmp::min(_dst.len(), _src.len()); for _i in 0.._n { _dst[_i] = _src[_i].clone(); } ")
			WriteWrapperPrefix(out)
			out.WriteString("_n as i32")
			WriteWrapperSuffix(out)
			out.WriteString(" }")
			return
		}

		out.WriteString("{ let _src = ")
		writeCopySourceValue(out, call.Args[1], srcIsString)
		out.WriteString("; let _n = std::cmp::min(")
		writeCopyDestination(out, call.Args[0], false)
		out.WriteString(".len(), _src.len()); for _i in 0.._n { ")
		// Destination needs mutable borrow for assignment.
		writeCopyDestination(out, call.Args[0], true)
		out.WriteString("[_i] = _src[_i].clone(); } ")
		WriteWrapperPrefix(out)
		out.WriteString("_n as i32")
		WriteWrapperSuffix(out)
		out.WriteString(" }")
	}
}

func writeCopyNamedSliceDestinationSlice(out *strings.Builder, dstSlice *ast.SliceExpr, src ast.Expr, srcIsString bool) bool {
	sliceSubject := unwrapParens(dstSlice.X)
	if !isNamedSliceExpression(sliceSubject) {
		return false
	}
	out.WriteString("{ let _dst_holder = ")
	writeNamedSliceInnerHandleClone(out, sliceSubject)
	out.WriteString("; let _dst_start = ")
	writeCopySliceLow(out, dstSlice)
	out.WriteString("; let _dst_len = ")
	if dstSlice.High != nil {
		out.WriteString("(")
		writeExpressionAsUsize(out, dstSlice.High)
		out.WriteString(") - _dst_start")
	} else {
		out.WriteString("{ let _dst_guard = _dst_holder")
		WriteBorrowMethod(out, false)
		out.WriteString("; _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } - _dst_start")
	}
	out.WriteString("; let _src = ")
	writeCopySourceValue(out, src, srcIsString)
	out.WriteString("; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder")
	WriteBorrowMethod(out, true)
	out.WriteString(".as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } ")
	WriteWrapperPrefix(out)
	out.WriteString("_n as i32")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
	return true
}

func copyDestinationIsBareSlice(dst ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsSlice(dst) {
		return false
	}
	if isExpressionResultBare(dst) {
		return true
	}
	_, ok := unwrapParens(dst).(*ast.TypeAssertExpr)
	return ok
}

// writeCopyDestination emits the unwrapped slice place for a copy() destination
// that is not itself a slice expression (a bare variable or a struct field). The
// destination of copy() is a wrapped slice handle in our model, so both the
// length term and the per-element assignment must reach through the wrapper to
// the inner Vec. Using TranspileExpressionContext(LValue) yields the handle
// without cloning, so the mutable borrow writes into the live slice rather than
// a discarded temporary (the prior else-branch cloned field selectors, silently
// dropping the copy).
func writeCopyDestination(out *strings.Builder, dst ast.Expr, mutable bool) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.IsSlice(dst) && !isExpressionResultBare(dst) {
		out.WriteString("(*")
		TranspileExpressionContext(out, dst, LValue)
		WriteBorrowMethod(out, mutable)
		if mutable {
			out.WriteString(".as_mut().unwrap())")
		} else {
			out.WriteString(".as_ref().unwrap())")
		}
		return
	}
	out.WriteString("(")
	TranspileExpression(out, dst)
	out.WriteString(")")
}

func writeCopySourceValue(out *strings.Builder, expr ast.Expr, isString bool) {
	if isString {
		writeStringSequenceValue(out, expr)
		out.WriteString(".as_bytes().to_vec()")
		return
	}
	if writeNamedSliceCopySourceValue(out, expr) {
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsSlice(expr) && !isExpressionResultBare(expr) {
		if _, isSliceExpr := unwrapParens(expr).(*ast.SliceExpr); isSliceExpr {
			out.WriteString("(*")
			TranspileExpression(out, expr)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()")
			return
		}
		out.WriteString("{ let __copy_src_holder = ")
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone(); let __copy_src_guard = __copy_src_holder")
		WriteBorrowMethod(out, false)
		out.WriteString("; __copy_src_guard.as_ref().cloned().unwrap_or_default() }")
		return
	}
	out.WriteString("(")
	TranspileExpression(out, expr)
	out.WriteString(").clone()")
}

func writeNamedSliceCopySourceValue(out *strings.Builder, expr ast.Expr) bool {
	expr = unwrapParens(expr)
	if slice, ok := expr.(*ast.SliceExpr); ok {
		sliceSubject := unwrapParens(slice.X)
		if isNamedSliceExpression(sliceSubject) {
			out.WriteString("{ let __slice_holder = ")
			writeNamedSliceInnerHandleClone(out, sliceSubject)
			out.WriteString("; let __slice_guard = __slice_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[")
			if slice.Low != nil {
				writeExpressionAsUsize(out, slice.Low)
			}
			out.WriteString("..")
			if slice.High != nil {
				writeExpressionAsUsize(out, slice.High)
			}
			out.WriteString("].to_vec() }")
			return true
		}
		if isNamedArrayExpression(sliceSubject) {
			out.WriteString("{ let __array_holder = ")
			writeNamedArrayInnerHandleClone(out, sliceSubject)
			out.WriteString("; let __array_guard = __array_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __seq = __array_guard.as_ref().unwrap(); __seq[")
			if slice.Low != nil {
				writeExpressionAsUsize(out, slice.Low)
			}
			out.WriteString("..")
			if slice.High != nil {
				writeExpressionAsUsize(out, slice.High)
			}
			out.WriteString("].to_vec() }")
			return true
		}
		return false
	}
	if !isNamedSliceExpression(expr) {
		return false
	}
	out.WriteString("{ let __slice_holder = ")
	writeNamedSliceInnerHandleClone(out, expr)
	out.WriteString("; let __slice_guard = __slice_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; __slice_guard.as_ref().cloned().unwrap_or_default() }")
	return true
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
		if newCallAllocatesFunctionSlot(call) {
			WriteWrappedNone(out)
			return
		}
		WriteWrapperPrefix(out)
		out.WriteString(rustDefaultConstructorExpression(goTypeToRustBase(call.Args[0])))
		out.WriteString(")))")
	}
}

func newCallAllocatesFunctionSlot(call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(call)
	if typ == nil {
		return false
	}
	ptr, ok := types.Unalias(typ).(*types.Pointer)
	if !ok {
		return false
	}
	return isFunctionSignatureType(ptr.Elem())
}

func rustDefaultConstructorExpression(rustType string) string {
	if idx := strings.Index(rustType, "<"); idx != -1 {
		return rustType[:idx] + "::" + rustType[idx:] + "::default()"
	}
	return rustType + "::default()"
}

// transpileComplex handles the complex() builtin function
func transpileComplex(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 2 {
		return
	}

	componentType := complexBuiltinComponentRustType(call)
	WriteWrapperPrefix(out)
	out.WriteString("num::Complex::new(")
	writeNumericConversionValue(out, call.Args[0])
	out.WriteString(" as ")
	out.WriteString(componentType)
	out.WriteString(", ")
	writeNumericConversionValue(out, call.Args[1])
	out.WriteString(" as ")
	out.WriteString(componentType)
	out.WriteString(")")
	WriteWrapperSuffix(out)
}

func complexBuiltinComponentRustType(call *ast.CallExpr) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "f64"
	}
	typ := typeInfo.GetType(call)
	if typ == nil {
		return "f64"
	}
	basic, ok := types.Unalias(typ).Underlying().(*types.Basic)
	if !ok {
		return "f64"
	}
	if basic.Kind() == types.Complex64 {
		return "f32"
	}
	return "f64"
}

// transpileReal handles the real() builtin function
func transpileReal(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 1 {
		return
	}

	WriteWrapperPrefix(out)
	writeComplexComponentValue(out, call.Args[0], "re")
	out.WriteString(")))")
}

// transpileImag handles the imag() builtin function
func transpileImag(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 1 {
		return
	}

	WriteWrapperPrefix(out)
	writeComplexComponentValue(out, call.Args[0], "im")
	out.WriteString(")))")
}

func writeComplexComponentValue(out *strings.Builder, expr ast.Expr, component string) {
	if isExpressionResultBare(expr) {
		TranspileExpression(out, expr)
		out.WriteString(".")
		out.WriteString(component)
		return
	}
	out.WriteString("(*")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).")
	out.WriteString(component)
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

// generateNestedSliceWrappedFormatter emits format_nested_slice_wrapped, the
// variant for a nested slice whose innermost elements are wrapped handles
// (e.g. [][]*T). It formats each inner row via format_slice_wrapped_values,
// which unwraps the handles, so the innermost T only needs Display.
func generateNestedSliceWrappedFormatter(out *strings.Builder) {
	TrackImport("Display")
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`fn format_nested_slice_wrapped<T, C, Inner>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s
            .as_ref()
            .iter()
            .map(|inner| format_slice_wrapped_values(inner.as_ref()))
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
		out.WriteString(`fn format_nested_slice_wrapped<T, C, Inner>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s
            .as_ref()
            .iter()
            .map(|inner| format_slice_wrapped_values(inner.as_ref()))
            .collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}
`)
	}
}

func generateNestedPointerSliceFormatter(out *strings.Builder) {
	TrackImport("Display")
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`fn format_nested_pointer_slice<T, C, Inner>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<Inner>>>]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s
            .as_ref()
            .iter()
            .map(|inner| {
                let inner_guard = inner.lock().unwrap();
                match inner_guard.as_ref() {
                    Some(values) => format!("&{}", format_slice_values(values.as_ref())),
                    None => "<nil>".to_string(),
                }
            })
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
		out.WriteString(`fn format_nested_pointer_slice<T, C, Inner>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<Inner>>>]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s
            .as_ref()
            .iter()
            .map(|inner| {
                let inner_guard = inner.borrow();
                match inner_guard.as_ref() {
                    Some(values) => format!("&{}", format_slice_values(values.as_ref())),
                    None => "<nil>".to_string(),
                }
            })
            .collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}
`)
	}
}

func generateNestedPointerSliceWrappedFormatter(out *strings.Builder) {
	TrackImport("Display")
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`fn format_nested_pointer_slice_wrapped<T, C, Inner>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<Inner>>>]>,
    Inner: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s
            .as_ref()
            .iter()
            .map(|inner| {
                let inner_guard = inner.lock().unwrap();
                match inner_guard.as_ref() {
                    Some(values) => format!("&{}", format_slice_wrapped_values(values.as_ref())),
                    None => "<nil>".to_string(),
                }
            })
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
		out.WriteString(`fn format_nested_pointer_slice_wrapped<T, C, Inner>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<Inner>>>]>,
    Inner: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s
            .as_ref()
            .iter()
            .map(|inner| {
                let inner_guard = inner.borrow();
                match inner_guard.as_ref() {
                    Some(values) => format!("&{}", format_slice_wrapped_values(values.as_ref())),
                    None => "<nil>".to_string(),
                }
            })
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
	if len(call.Args) > 0 && writeConcurrentPanicPayloadArg(out, call.Args[0]) {
		return
	}
	if len(call.Args) > 0 && writePanicAnyArg(out, call.Args[0]) {
		return
	}
	out.WriteString("panic!(")
	if len(call.Args) > 0 {
		// Check if the argument is a string literal
		if lit, ok := call.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
			// String literal - use it directly
			out.WriteString(RustStringLiteral(lit.Value))
		} else if callExpr, ok := call.Args[0].(*ast.CallExpr); ok {
			// fmt.Errorf / fmt.Sprintf - extract the format string directly
			// so the panic message is a bare string rather than the wrapped
			// Arc<Mutex<Option<String>>> that fmt.Sprintf would normally emit.
			if sel, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
				if pkg, ok := sel.X.(*ast.Ident); ok && pkg.Name == "fmt" && (sel.Sel.Name == "Errorf" || sel.Sel.Name == "Sprintf") {
					if len(callExpr.Args) > 0 {
						if lit, ok := callExpr.Args[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
							format := convertFormatString(lit.Value)
							out.WriteString(format)
							for i := 1; i < len(callExpr.Args); i++ {
								out.WriteString(", ")
								transpilePrintArg(out, callExpr.Args[i])
							}
						} else {
							out.WriteString("\"{}\", ")
							writeOwnedStringStdlibArg(out, callExpr.Args[0])
						}
					}
				} else {
					writePanicDisplayArg(out, call.Args[0])
				}
			} else {
				writePanicDisplayArg(out, call.Args[0])
			}
		} else {
			writePanicDisplayArg(out, call.Args[0])
		}
	} else {
		out.WriteString("\"explicit panic\"")
	}
	out.WriteString(")")
}

func writeConcurrentPanicPayloadArg(out *strings.Builder, arg ast.Expr) bool {
	if !NeedsConcurrentWrapper() {
		return false
	}
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		return false
	}
	out.WriteString("std::panic::panic_any(")
	if isEmptyInterfaceValueExpr(arg) {
		if !writeExistingAnyBoxClone(out, arg) {
			return false
		}
	} else {
		writeInterfaceBoxedValue(out, arg)
	}
	out.WriteString(")")
	return true
}

func writePanicAnyArg(out *strings.Builder, arg ast.Expr) bool {
	if !isEmptyInterfaceValueExpr(arg) {
		return false
	}
	if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
		return false
	}
	if NeedsConcurrentWrapper() {
		out.WriteString("std::panic::panic_any(")
		if !writeExistingAnyBoxClone(out, arg) {
			return false
		}
		out.WriteString(")")
		return true
	}
	NeedFormatAny()
	out.WriteString("panic!(\"{}\", { let __any_holder = ")
	TranspileExpressionContext(out, arg, LValue)
	out.WriteString(".clone(); let __any_guard = __any_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; format_any(__any_guard.as_ref().expect(\"nil interface in panic argument\").as_ref()) })")
	return true
}

// writePanicDisplayArg emits `"{}", <unwrapped arg>` so panic! can format
// any Go value via its Display impl. Wrapped values (variables, calls,
// field accesses) get .borrow().as_ref().unwrap() to reach the inner T;
// bare values (struct/composite literals) are passed through directly.
func writePanicDisplayArg(out *strings.Builder, arg ast.Expr) {
	out.WriteString("\"{}\", ")
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.ReturnsWrappedValue(arg) {
		out.WriteString("(*")
		TranspileExpressionContext(out, arg, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
		return
	}
	TranspileExpression(out, arg)
}

// transpileRecover handles the recover() builtin function
func transpileRecover(out *strings.Builder, call *ast.CallExpr) {
	if NeedsConcurrentWrapper() {
		NeedPanicRecover()
		out.WriteString("go_recover()")
		return
	}
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
	if key == "os.Args" {
		NeedOsArgs()
		return "go_os_args()"
	}
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
	if writeTimeUnixExpandedMultiResult(out, call) {
		return
	}
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

func writeTimeUnixExpandedMultiResult(out *strings.Builder, call *ast.CallExpr) bool {
	sig, ok := callSignatureFromTypeInfo(call)
	if !ok || sig == nil || sig.Params() == nil || sig.Variadic() || sig.Params().Len() != 2 {
		return false
	}
	inner, innerSig, ok := singleMultiResultCallArgumentForParams(call, sig.Params())
	if !ok || innerSig == nil || innerSig.Results() == nil || innerSig.Results().Len() != 2 {
		return false
	}

	WriteWrapperPrefix(out)
	out.WriteString("{ ")
	writeExpandedMultiResultArgBinding(out, inner, innerSig.Results())
	out.WriteString("GoTime::from_unix(__multi_arg_0 as i64, __multi_arg_1 as i64) }")
	out.WriteString(")))")
	return true
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

func transpileTimeAfterFunc(out *strings.Builder, call *ast.CallExpr) {
	NeedGoTimer()
	TrackImport("time::Duration")
	WriteWrapperPrefix(out)
	out.WriteString("go_after_func(")
	if len(call.Args) > 0 {
		transpileDurationArg(out, call.Args[0])
	} else {
		out.WriteString("std::time::Duration::from_secs(0)")
	}
	out.WriteString(", ")
	if len(call.Args) > 1 {
		writeTimeAfterFuncCallback(out, call.Args[1])
	} else {
		writeUnsupportedTimeAfterFuncCallback(out)
	}
	out.WriteString(")")
	out.WriteString(")))")
}

func writeTimeAfterFuncCallback(out *strings.Builder, arg ast.Expr) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		writeUnsupportedTimeAfterFuncCallback(out)
		return
	}
	sig, ok := signatureFromType(typeInfo.GetType(arg))
	if !ok || sig == nil || sig.Params().Len() != 0 || sig.Results().Len() != 0 {
		writeUnsupportedTimeAfterFuncCallback(out)
		return
	}
	if ident, ok := arg.(*ast.Ident); ok && typeInfo.IsFunction(ident) {
		out.WriteString(rustFunctionNameForUse(ident.Name))
		return
	}
	writeUnsupportedTimeAfterFuncCallback(out)
}

func writeUnsupportedTimeAfterFuncCallback(out *strings.Builder) {
	out.WriteString("move || { unimplemented!(\"time.AfterFunc callback lowering requires a named zero-argument function\") }")
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
				writeTimeDurationUnitMultiplier(out, multiplier)
				out.WriteString(")")
			case "Minute":
				out.WriteString("from_secs(60 * ")
				writeTimeDurationUnitMultiplier(out, multiplier)
				out.WriteString(")")
			case "Second":
				out.WriteString("from_secs(")
				writeTimeDurationUnitMultiplier(out, multiplier)
				out.WriteString(")")
			case "Millisecond":
				out.WriteString("from_millis(")
				writeTimeDurationUnitMultiplier(out, multiplier)
				out.WriteString(")")
			case "Microsecond":
				out.WriteString("from_micros(")
				writeTimeDurationUnitMultiplier(out, multiplier)
				out.WriteString(")")
			case "Nanosecond":
				out.WriteString("from_nanos(")
				writeTimeDurationUnitMultiplier(out, multiplier)
				out.WriteString(")")
			default:
				out.WriteString("from_millis(")
				writeTimeDurationUnitMultiplier(out, multiplier)
				out.WriteString(")")
			}
			return
		}
	}
	if writeTimeDurationRawValue(out, arg) {
		return
	}
	// Fallback: treat as raw expression
	TranspileExpression(out, arg)
}

func writeTimeDurationUnitMultiplier(out *strings.Builder, multiplier ast.Expr) {
	if writeTimeDurationConversionMultiplier(out, multiplier) {
		return
	}
	TranspileExpression(out, multiplier)
}

func writeTimeDurationBinaryExpression(out *strings.Builder, expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.MUL {
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
	if writeTimeDurationRawValue(out, value) {
		return
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

func writeTimeDurationRawValue(out *strings.Builder, value ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !timeDurationUsesStdTimeDuration(typeInfo.GetType(value)) {
		return false
	}
	if hasStdlibSelectorMapping(value) {
		TranspileExpression(out, value)
		return true
	}
	if isConstantExpression(value) {
		out.WriteString("std::time::Duration::from_nanos(")
		writeConstExpressionCastValue(out, value)
		out.WriteString(" as u64)")
		return true
	}
	if typeInfo.ReturnsWrappedValue(value) {
		out.WriteString("(*")
		TranspileExpressionContext(out, value, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return true
	}
	TranspileExpression(out, value)
	return true
}

func isTimeDurationType(typ types.Type) bool {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	return named.Obj().Pkg().Path() == "time" && named.Obj().Name() == "Duration"
}

func timeDurationUsesStdTimeDuration(typ types.Type) bool {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || !isTimeDurationType(named) {
		return false
	}
	return !useStubBackedStdlibNamedIntegerInSourceMappedStdlib(named)
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
