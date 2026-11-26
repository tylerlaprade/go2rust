package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

const goTypeNameHelperRustName = "__go_type_name"

// ImportTracker tracks which imports are needed during transpilation
type ImportTracker struct {
	needs         map[string]bool
	reservedNames map[string]bool
}

// NewImportTracker creates a new import tracker
func NewImportTracker() *ImportTracker {
	return &ImportTracker{
		needs:         make(map[string]bool),
		reservedNames: make(map[string]bool),
	}
}

// Add marks an import as needed with a reason
func (it *ImportTracker) Add(importName string) {
	it.needs[importName] = true
}

// ReserveName marks a Rust name emitted in this file so imports can avoid it.
func (it *ImportTracker) ReserveName(name string) {
	if name == "" {
		return
	}
	if it.reservedNames == nil {
		it.reservedNames = make(map[string]bool)
	}
	it.reservedNames[name] = true
}

func (it *ImportTracker) IsReservedName(name string) bool {
	return it != nil && it.reservedNames[name]
}

// GenerateImports returns the import statements for the file
func (it *ImportTracker) GenerateImports() string {
	if len(it.needs) == 0 {
		return ""
	}

	var imports []string

	// Check for std::sync imports
	var syncImports []string
	if it.needs["Arc"] {
		syncImports = append(syncImports, "Arc")
	}
	if it.needs["Mutex"] {
		if it.IsReservedName("Mutex") {
			syncImports = append(syncImports, "Mutex as StdMutex")
		} else {
			syncImports = append(syncImports, "Mutex")
		}
	}
	if len(syncImports) > 0 {
		imports = append(imports, fmt.Sprintf("use std::sync::{%s};", strings.Join(syncImports, ", ")))
	}

	// Check for std::rc and std::cell imports
	var rcImports []string
	if it.needs["Rc"] {
		rcImports = append(rcImports, "Rc")
	}
	if len(rcImports) > 0 {
		imports = append(imports, fmt.Sprintf("use std::rc::{%s};", strings.Join(rcImports, ", ")))
	}

	var cellImports []string
	if it.needs["RefCell"] {
		cellImports = append(cellImports, "RefCell")
	}
	if len(cellImports) > 0 {
		imports = append(imports, fmt.Sprintf("use std::cell::{%s};", strings.Join(cellImports, ", ")))
	}

	// BTreeMap (deterministic iteration order, matching Go test expectations)
	if it.needs["BTreeMap"] {
		imports = append(imports, "use std::collections::BTreeMap;")
	}

	// fmt imports
	var fmtImports []string
	if it.needs["fmt::self"] {
		fmtImports = append(fmtImports, "self")
	}
	if it.needs["Display"] {
		fmtImports = append(fmtImports, "Display")
	}
	if it.needs["Debug"] {
		fmtImports = append(fmtImports, "Debug")
	}
	if it.needs["Formatter"] {
		fmtImports = append(fmtImports, "Formatter")
	}
	if len(fmtImports) > 0 {
		imports = append(imports, fmt.Sprintf("use std::fmt::{%s};", strings.Join(fmtImports, ", ")))
	}

	// Thread and time imports
	if it.needs["thread"] {
		imports = append(imports, "use std::thread;")
	}
	if it.needs["time::Duration"] {
		imports = append(imports, "use std::time::Duration;")
	}

	// Other imports
	if it.needs["Error"] {
		imports = append(imports, "use std::error::Error as StdError;")
	}
	if it.needs["Any"] {
		imports = append(imports, "use std::any::Any;")
	}
	if it.needs["Ord"] {
		imports = append(imports, "use std::cmp::Ord;")
	}

	// External crate imports
	if it.needs["num::Complex"] {
		imports = append(imports, "use num::Complex;")
	}

	// Sort for consistent output
	slices.Sort(imports)

	return strings.Join(imports, "\n") + "\n"
}

// HelperTracker tracks which helper functions are needed
type HelperTracker struct {
	needsFormatMap                       bool
	needsFormatSlice                     bool
	needsFormatSliceWrappedValues        bool
	needsFormatSliceWrappedStringer      bool
	needsFormatNestedSlice               bool
	needsFormatNestedSliceWrapped        bool
	needsFormatNestedPointerSlice        bool
	needsFormatNestedPointerSliceWrapped bool
	needsFormatAny                       bool
	needsFormatAnySlice                  bool
	needsFormatAnyNested                 bool
	needsAnyEq                           bool
	needsAnyClone                        bool
	needsAnyHandleCopy                   bool
	needsPanicRecover                    bool
	needsPanicExit                       bool
	needsGoPointerError                  bool
	needsGoValueClone                    bool
	needsGoTypeParamFieldClone           bool
	needsGoComparable                    bool
	needsGoConstStrEq                    bool
	needsGoAnyTypeMetadata               bool
	needsGoAnyRuntimeOps                 bool
	// needsAnyPointerFormatter says a run-time registration of a pointer
	// type formats it with go_format_any_pointer.
	needsAnyPointerFormatter   bool
	needsEmbeddedOwnerRegistry bool
	needsGoByteSequence        bool
	needsGoInteger             bool
	needsGoTimeRuntime         bool
	needsGoChannelIdentity     bool
	needsGoChannel             bool
	needsGoSelectSend          bool
	needsWaitGroup             bool
	needsGoMutex               bool
	needsGoRWMutex             bool
	needsGoOnce                bool
	needsGoAtomicPointer       bool
	needsGoTypeName            bool
	needsGoFloatString         bool
	needsGoComplexDisplay      bool // the program has complex numbers
	needsGoErrorChain          bool
	needsGoSprintf             bool
	needsGoFmtScan             bool
	needsGoSprintfSpread       bool
	needsBase64                bool
	needsSha256                bool
	needsHexFormat             bool
	needsStrconvFormat         bool
	needsUrl                   bool
	needsRegexp                bool
	needsJsonEscape            bool
	needsGoNilAny              bool
	needsGoSliceAppendCopying  bool
	needsGoFuncValue           bool
	needsOsArgs                bool
	needsSliceElemPtr          bool
	needsGoTime                bool
	needsGoTimer               bool
	needsGoAfter               bool
	needsGoTicker              bool
	needsGoTick                bool
	needsGoContext             bool
	needsGoRand                bool
	needsReflect               bool
	needsGoHttpResponse        bool
	needsGoPtrKey              bool
	needsGoAnyPtrKey           bool
	// omitGoSlice is set on the copy that leaves out the helpers a shared
	// stub crate provides; GoSlice is one of them.
	omitGoSlice   bool
	anyCloneTypes map[string]bool
	// anyTypeNames maps the Rust payload type of a value boxed into an empty
	// interface to the name Go's %T prints for it.
	anyTypeNames map[string]string
	// anyFormatArms maps the same payload types to how format_any prints
	// them.
	anyFormatArms map[string]anyFormatArm
}

// anyFormatArm is one case of format_any: how to print a value of one type
// that the program boxes into an empty interface.
type anyFormatArm struct {
	// expr is a Rust expression of type String. It formats v, a reference to
	// the boxed payload, with the helper the typed print path uses for a
	// value of the same Go type.
	expr             string
	needsFormatMap   bool
	needsFormatSlice bool
	// needsFormatPointer says expr calls go_format_any_pointer.
	needsFormatPointer bool
	// text says the value has a String or an Error method. fmt prints such a
	// value as the one text the method returns, so a width, a precision, %q
	// and %x apply to that text and not to the fields of the value.
	text bool
	// namedBasic says the value is a newtype around one basic value, "type
	// Level int". A format known only at run time formats that value for any
	// verb the type's own text does not take.
	namedBasic bool
	// fields says the value is a struct whose Display writes the field
	// names when asked for the alternate form, which is how %+v prints it.
	fields bool
	// elements says the value is an array of basic values, boxed as the Rust
	// array. A format known only at run time applies a verb such as %q or
	// %x to each element, as it does for a slice.
	elements bool
}

var generatingPublicHelpers bool

func (ht *HelperTracker) withoutSharedStdlibHelpers() *HelperTracker {
	if ht == nil {
		return nil
	}
	helperCopy := *ht
	helperCopy.needsGoChannel = false
	helperCopy.needsGoChannelIdentity = false
	helperCopy.needsGoSelectSend = false
	helperCopy.needsGoContext = false
	helperCopy.needsGoTime = false
	if helperCopy.needsGoValueClone && len(helperCopy.anyCloneTypes) == 0 {
		helperCopy.needsAnyClone = false
	}
	helperCopy.needsGoValueClone = false
	helperCopy.needsGoTypeParamFieldClone = false
	helperCopy.needsGoComparable = false
	helperCopy.needsGoAnyTypeMetadata = false
	helperCopy.needsGoAnyRuntimeOps = false
	helperCopy.needsEmbeddedOwnerRegistry = false
	helperCopy.needsGoAnyPtrKey = false
	helperCopy.omitGoSlice = true
	return &helperCopy
}

func (ht *HelperTracker) withoutGoSlice() *HelperTracker {
	if ht == nil {
		return &HelperTracker{omitGoSlice: true}
	}
	helperCopy := *ht
	helperCopy.omitGoSlice = true
	return &helperCopy
}

// goSlice reports whether this tracker's helpers define GoSlice.
func (ht *HelperTracker) goSlice() bool {
	return ht != nil && !ht.omitGoSlice && goSliceModeEnabled()
}

func (ht *HelperTracker) sharedStdlibHelpersOnly() *HelperTracker {
	if ht == nil {
		return nil
	}
	helperCopy := &HelperTracker{needsGoSelectSend: ht.needsGoSelectSend}
	if ht.needsGoChannel || ht.needsGoContext {
		helperCopy.needsGoChannel = true
	}
	if ht.needsGoChannelIdentity {
		helperCopy.needsGoChannel = true
		helperCopy.needsGoChannelIdentity = true
	}
	if ht.needsGoContext {
		helperCopy.needsGoContext = true
	}
	if ht.needsGoTime {
		helperCopy.needsGoTime = true
	}
	if ht.needsGoValueClone {
		helperCopy.needsAnyClone = true
		helperCopy.needsGoValueClone = true
	}
	if ht.needsGoTypeParamFieldClone {
		helperCopy.needsGoTypeParamFieldClone = true
	}
	if ht.needsGoComparable {
		helperCopy.needsGoComparable = true
	}
	if ht.needsGoAnyTypeMetadata {
		helperCopy.needsGoAnyTypeMetadata = true
	}
	if ht.needsGoAnyRuntimeOps {
		helperCopy.needsGoAnyRuntimeOps = true
	}
	if ht.needsEmbeddedOwnerRegistry {
		helperCopy.needsEmbeddedOwnerRegistry = true
	}
	if ht.needsGoRWMutex {
		helperCopy.needsGoRWMutex = true
	}
	if ht.needsGoAnyPtrKey {
		helperCopy.needsGoAnyPtrKey = true
	}
	return helperCopy
}

// GenerateHelpers returns the helper function definitions
func (ht *HelperTracker) GenerateHelpers() string {
	var result strings.Builder

	if ht.goSlice() {
		generateGoSliceHelper(&result)
	}

	// In a program of several packages, each helper also carries the cases
	// the other packages need it to have.
	anyCloneTypes, anyFormatArms, anyTypeNames := ht.anyCloneTypes, ht.anyFormatArms, ht.anyTypeNames
	if shared := currentProgramAnyCases; shared != nil {
		anyCloneTypes, anyFormatArms, anyTypeNames = shared.casesFor(ht)
	}

	// format_any prints boxed slices and maps through the typed formatters,
	// so it needs them even when no typed print site asked for them.
	needsFormatAnyPointer := ht.needsAnyPointerFormatter
	if ht.needsFormatAny {
		for _, arm := range anyFormatArms {
			ht.needsFormatMap = ht.needsFormatMap || arm.needsFormatMap
			ht.needsFormatSlice = ht.needsFormatSlice || arm.needsFormatSlice
			needsFormatAnyPointer = needsFormatAnyPointer || arm.needsFormatPointer
		}
	}
	if needsFormatAnyPointer {
		generateAnyPointerFormatter(&result)
	}

	if ht.needsFormatMap {
		generateMapFormatter(&result)
	}

	if ht.needsFormatSlice {
		generateSliceFormatter(&result, ht.needsFormatSliceWrappedValues, ht.needsFormatSliceWrappedStringer)
	}

	if ht.needsFormatNestedSlice {
		generateNestedSliceFormatter(&result)
	}

	if ht.needsFormatNestedSliceWrapped {
		generateNestedSliceWrappedFormatter(&result)
	}

	if ht.needsFormatNestedPointerSlice {
		generateNestedPointerSliceFormatter(&result)
	}

	if ht.needsFormatNestedPointerSliceWrapped {
		generateNestedPointerSliceWrappedFormatter(&result)
	}

	if ht.needsFormatAny {
		// format_any_slice prints its elements as nested values, and only a
		// pointer case makes that differ from format_any.
		needsNestedEntry := ht.needsFormatAnyNested || (needsFormatAnyPointer && ht.needsFormatAnySlice)
		generateAnyFormatter(&result, anyFormatArms, needsFormatAnyPointer, needsNestedEntry, programAnyRuntimeOpsEnabled())
	}

	if ht.needsFormatAnySlice {
		generateAnySliceFormatter(&result, needsFormatAnyPointer)
	}

	if ht.needsAnyEq {
		generateAnyEquality(&result)
	}

	if ht.needsGoNilAny {
		generateGoNilAny(&result)
	}

	if ht.needsGoSliceAppendCopying {
		generateGoSliceAppendCopying(&result)
	}

	if ht.needsGoFuncValue {
		generateGoFuncValue(&result)
	}

	if ht.needsAnyClone || ht.needsAnyHandleCopy {
		generateAnyClone(&result, anyCloneTypes, ht.needsAnyHandleCopy)
	}

	if ht.needsPanicRecover {
		generatePanicRecoverHelper(&result)
	}

	if ht.needsPanicExit {
		boxed := NeedsConcurrentWrapper() || ht.needsPanicRecover
		generatePanicExitHelper(&result, boxed, boxed && ht.needsFormatAny && NeedsConcurrentWrapper())
	}

	if ht.needsPanicRecover || ht.needsPanicExit {
		visibility := ""
		if generatingPublicHelpers {
			visibility = "pub "
		}
		writeGoRuntimeErrorText(&result, visibility)
	}

	if ht.needsGoPointerError {
		generateGoPointerError(&result)
	}

	if ht.needsGoValueClone {
		generateGoValueClone(&result)
	}

	if ht.needsGoTypeParamFieldClone {
		generateGoTypeParamFieldClone(&result)
	}

	if ht.needsGoComparable {
		generateGoComparable(&result)
	}

	if ht.needsGoConstStrEq {
		generateGoConstStrEq(&result)
	}

	if ht.needsGoAnyTypeMetadata {
		generateGoAnyTypeMetadata(&result)
	}
	if ht.needsGoAnyRuntimeOps {
		generateGoAnyRuntimeOps(&result)
	}

	if ht.needsEmbeddedOwnerRegistry {
		generateEmbeddedOwnerRegistry(&result)
	}

	if ht.needsGoByteSequence {
		generateGoByteSequence(&result)
	}

	if ht.needsGoInteger {
		generateGoInteger(&result)
	}

	if ht.needsGoTimeRuntime {
		generateGoTimeRuntime(&result)
	}

	if ht.needsGoChannel {
		generateGoChannelHelper(&result, ht.needsGoTypeParamFieldClone, ht.needsGoChannelIdentity)
		if ht.needsGoSelectSend {
			result.WriteString(goSelectSendHelper)
		}
	}

	if ht.needsWaitGroup {
		generateWaitGroupHelper(&result)
	}

	if ht.needsGoMutex {
		generateGoMutexHelper(&result)
	}

	if ht.needsGoRWMutex {
		generateGoRWMutexHelper(&result)
	}

	if ht.needsGoOnce {
		generateGoOnceHelper(&result)
	}

	if ht.needsGoAtomicPointer {
		generateGoAtomicPointerHelper(&result)
	}

	if ht.needsGoTypeName {
		generateGoTypeNameHelper(&result, anyTypeNames, programAnyRuntimeOpsEnabled())
	}

	if ht.needsGoFloatString || ht.needsFormatAny || ht.needsFormatSlice || ht.needsFormatMap {
		generateGoFloatStringHelper(&result, ht.needsGoComplexDisplay)
	}

	if ht.needsGoErrorChain {
		generateGoErrorChainHelper(&result)
	}

	if ht.needsGoSprintf {
		generateGoSprintf(&result, ht.needsGoSprintfSpread, anyFormatArms)
	}

	if ht.needsGoFmtScan {
		generateGoFmtScan(&result)
	}

	if ht.needsBase64 {
		generateBase64Helper(&result)
	}

	if ht.needsSha256 {
		generateSha256Helper(&result)
	}

	if ht.needsHexFormat {
		generateHexFormatHelper(&result)
	}

	if ht.needsStrconvFormat {
		generateStrconvFormatHelper(&result)
	}

	if ht.needsUrl {
		generateUrlHelper(&result)
	}

	if ht.needsRegexp {
		generateRegexpHelper(&result)
	}

	if ht.needsJsonEscape {
		generateJsonEscapeHelper(&result)
	}

	if ht.needsOsArgs {
		generateOsArgsHelper(&result)
	}

	if ht.needsSliceElemPtr {
		generateSliceElemPtrHelper(&result)
	}

	if ht.needsGoTime {
		generateGoTimeHelper(&result)
	}

	if ht.needsGoTimer {
		generateGoTimerHelper(&result)
	}

	if ht.needsGoAfter {
		generateGoAfterHelper(&result)
	}

	if ht.needsGoTick {
		generateGoTickHelper(&result)
	}

	if ht.needsGoTicker {
		generateGoTickerHelper(&result)
	}

	if ht.needsGoContext {
		generateGoContextHelper(&result)
	}

	if ht.needsGoRand {
		generateGoRandHelper(&result)
	}

	if ht.needsReflect {
		generateReflectHelper(&result)
	}

	if ht.needsGoPtrKey {
		generateGoPtrKeyHelper(&result, "GoLocalPtrKey", ht.needsSliceElemPtr)
	}
	if ht.needsGoAnyPtrKey {
		generateGoAnyPtrKeyHelper(&result)
	}

	return goSliceHelperText(result.String())
}

func (ht *HelperTracker) HasAnyOmittingSharedStdlibHelpers() bool {
	filtered := ht.withoutSharedStdlibHelpers()
	return filtered != nil && filtered.HasAny()
}

func (ht *HelperTracker) HasAny() bool {
	return ht != nil && (ht.goSlice() ||
		ht.needsFormatMap ||
		ht.needsFormatSlice ||
		ht.needsFormatAny ||
		ht.needsFormatAnySlice ||
		ht.needsAnyEq ||
		ht.needsAnyClone ||
		ht.needsAnyHandleCopy ||
		ht.needsPanicRecover ||
		ht.needsPanicExit ||
		ht.needsGoPointerError ||
		ht.needsGoValueClone ||
		ht.needsGoTypeParamFieldClone ||
		ht.needsGoComparable ||
		ht.needsGoConstStrEq ||
		ht.needsGoAnyTypeMetadata ||
		ht.needsGoAnyRuntimeOps ||
		ht.needsAnyPointerFormatter ||
		ht.needsEmbeddedOwnerRegistry ||
		ht.needsGoByteSequence ||
		ht.needsGoInteger ||
		ht.needsGoTimeRuntime ||
		ht.needsGoChannel ||
		ht.needsWaitGroup ||
		ht.needsGoMutex ||
		ht.needsGoRWMutex ||
		ht.needsGoOnce ||
		ht.needsGoAtomicPointer ||
		ht.needsGoTypeName ||
		ht.needsGoFloatString ||
		ht.needsGoErrorChain ||
		ht.needsBase64 ||
		ht.needsSha256 ||
		ht.needsHexFormat ||
		ht.needsStrconvFormat ||
		ht.needsUrl ||
		ht.needsRegexp ||
		ht.needsJsonEscape ||
		ht.needsGoNilAny ||
		ht.needsGoSliceAppendCopying ||
		ht.needsGoFuncValue ||
		ht.needsSliceElemPtr ||
		ht.needsGoTime ||
		ht.needsGoTimer ||
		ht.needsGoAfter ||
		ht.needsGoTicker ||
		ht.needsGoTick ||
		ht.needsGoContext ||
		ht.needsGoRand ||
		ht.needsReflect ||
		ht.needsGoHttpResponse ||
		ht.needsGoPtrKey ||
		ht.needsGoAnyPtrKey)
}

func (ht *HelperTracker) GenerateHelperModule() string {
	return ht.generateHelperModule(false)
}

func (ht *HelperTracker) GenerateHelperModuleOmittingSharedStdlibHelpers() string {
	return ht.withoutSharedStdlibHelpers().generateHelperModule(false)
}

func (ht *HelperTracker) GenerateSharedStdlibHelperModule() string {
	return ht.sharedStdlibHelpersOnly().generateHelperModule(true)
}

func (ht *HelperTracker) generateHelperModule(publicHelpers bool) string {
	if ht == nil {
		return ""
	}
	if !ht.HasAny() {
		return ""
	}

	imports := NewImportTracker()
	helperCopy := *ht
	fileState := NewFileState(imports, &helperCopy, nil)
	parentCtx := GetTranspileContext()
	prevPublicHelpers := generatingPublicHelpers
	generatingPublicHelpers = publicHelpers
	defer func() {
		generatingPublicHelpers = prevPublicHelpers
	}()
	SetTranspileContext(&TranspileContext{
		File:    fileState,
		Imports: imports,
		Helpers: &helperCopy,
	})
	helpersStr := helperCopy.GenerateHelpers()
	SetTranspileContext(parentCtx)

	var output strings.Builder
	importsStr := imports.GenerateImports()
	output.WriteString(importsStr)
	if importsStr != "" && helpersStr != "" {
		output.WriteString("\n")
	}
	output.WriteString(helpersStr)
	return output.String()
}

func (ht *HelperTracker) ImportNamesOmittingSharedStdlibHelpers() []string {
	return ht.withoutSharedStdlibHelpers().ImportNames()
}

func (ht *HelperTracker) ImportNames() []string {
	if ht == nil {
		return nil
	}
	seen := make(map[string]bool)
	add := func(names ...string) {
		for _, name := range names {
			if name != "" {
				seen[name] = true
			}
		}
	}

	if ht.goSlice() {
		add(goSliceImportNames...)
	}
	if ht.needsFormatMap {
		add("format_map")
	}
	if ht.needsFormatSlice {
		add("format_slice", "format_slice_values", "format_slice_wrapped")
		if ht.needsFormatSliceWrappedValues {
			add("format_slice_wrapped_values")
		}
		if ht.needsFormatSliceWrappedStringer {
			add("format_slice_wrapped_stringer", "format_slice_wrapped_stringer_values")
		}
	}
	if ht.needsFormatNestedSlice {
		add("format_nested_slice")
	}
	if ht.needsFormatNestedSliceWrapped {
		add("format_nested_slice_wrapped", "format_slice_wrapped_values")
	}
	if ht.needsFormatNestedPointerSlice {
		add("format_nested_pointer_slice", "format_slice_values")
	}
	if ht.needsFormatNestedPointerSliceWrapped {
		add("format_nested_pointer_slice_wrapped", "format_slice_wrapped_values")
	}
	if ht.needsFormatAny {
		add("format_any")
	}
	if ht.needsAnyPointerFormatter {
		add("go_format_any_pointer")
	}
	if ht.needsFormatAnySlice {
		add("format_any_slice", "format_any_variadic")
	}
	if ht.needsFormatAnyNested {
		add("format_any_nested")
	}
	if ht.needsAnyEq {
		add("go_any_eq")
	}
	if ht.needsAnyClone || ht.needsAnyHandleCopy {
		add("go_any_clone", "go_any_try_clone")
	}
	if ht.needsAnyHandleCopy {
		add("go_any_handle_copy", "go_any_store")
	}
	// Only main calls go_install_panic_exit, and main sees the helpers it
	// includes without importing them, so the panic exit helpers are not
	// imported into the other modules.
	if ht.needsPanicRecover {
		add("go_recover", "go_resume_unrecovered_panic", "go_store_panic_payload", "go_runtime_error_text", "go_silence_panic_hook", "go_restore_panic_hook")
	}
	if ht.needsGoPointerError {
		add("GoPointerError")
	}
	if ht.needsGoValueClone {
		add("GoValueClone")
	}
	if ht.needsGoTypeParamFieldClone {
		add("GoTypeParamFieldClone")
	}
	if ht.needsGoComparable {
		add("GoComparable")
	}
	if ht.needsGoConstStrEq {
		add("go_const_str_eq")
	}
	if ht.needsGoAnyTypeMetadata {
		add("GoAnyTypeMetadata", "go_any_type_metadata", "go_box_any_with_metadata", "go_register_any_type", "go_register_any_type_with_elem", "go_register_any_value_metadata")
	}
	if ht.needsGoAnyRuntimeOps {
		add("go_register_any_runtime_type", "go_format_registered_any", "go_clone_registered_any", "go_registered_any_type_name", "go_registered_any_is_error", "go_registered_any_error")
	}
	if ht.needsEmbeddedOwnerRegistry {
		add("go_lookup_embedded_owner", "go_register_embedded_owner")
	}
	if ht.needsGoByteSequence {
		add("GoByteSequence")
	}
	if ht.needsGoInteger {
		add("GoInteger", "go_integer_from_i128", "go_integer_cast", "go_integer_add_one", "go_integer_sub_one")
	}
	if ht.needsGoTimeRuntime {
		add("go_time_runtime_nano")
		if NeedsConcurrentWrapper() {
			add("go_time_timer_start", "go_time_timer_stop", "go_time_timer_reset")
		}
	}
	if ht.needsGoChannel {
		add("GoChannel")
	}
	if ht.needsWaitGroup {
		add("WaitGroup")
	}
	if ht.needsGoMutex {
		add("GoMutex")
	}
	if ht.needsGoRWMutex {
		add("GoRWMutex")
	}
	if ht.needsGoOnce {
		add("GoOnce")
	}
	if ht.needsGoAtomicPointer {
		add("GoAtomicPointer")
	}
	if ht.needsGoTypeName {
		add(goTypeNameHelperRustName)
	}
	// format_any and the slice and map formatters call these themselves; a
	// module only names go_float_string when it prints a float directly.
	if ht.needsGoFloatString {
		add("go_float_string")
	}
	if ht.needsGoErrorChain {
		add("GoWrapError", "go_error_wrap", "go_errors_join", "go_error_chain", "go_errors_unwrap", "go_errors_is")
	}
	if ht.needsGoSprintf {
		add("go_sprintf", "GoFormatNil")
	}
	if ht.needsGoSprintfSpread {
		add("go_sprintf_spread")
	}
	if ht.needsGoFmtScan {
		add("go_fmt_scan", "GoScanError", "GoScanRunes", "GoScanRuneFns", "GoScanTarget", "go_scan_string_source", "go_scan_stdin_source")
	}
	if ht.needsBase64 {
		add("go_base64_encode", "go_base64_decode", "go_base64_value")
	}
	if ht.needsSha256 {
		add("go_sha256_sum256")
	}
	if ht.needsHexFormat {
		add("go_format_hex_bytes")
	}
	if ht.needsStrconvFormat {
		add("go_strconv_format_int", "go_strconv_format_float")
	}
	if ht.needsUrl {
		add("GoUrl", "go_url_parse")
	}
	if ht.needsRegexp {
		add("GoRegexp", "go_regexp_find_all_string")
	}
	if ht.needsJsonEscape {
		add("go_json_escape")
	}
	if ht.needsGoNilAny {
		add(goNilAnyType)
	}
	if ht.needsGoSliceAppendCopying {
		add("GoSliceAppendCopying")
	}
	if ht.needsGoFuncValue {
		add("GoFuncValue")
	}
	if ht.needsSliceElemPtr {
		add("GoPtr", "GoSliceElemPtr", "GoSliceElemRef", "GoSliceElemMutRef", "GoArrayElemPtr", "GoArrayElemRef", "GoArrayElemMutRef")
	}
	if ht.needsGoTime {
		add("GoTime", "go_time_civil_from_days")
	}
	if ht.needsGoTimer {
		add("GoTimer", "go_new_timer", "go_after_func")
	}
	if ht.needsGoAfter {
		add("go_channel_after")
	}
	if ht.needsGoTicker {
		add("GoTicker", "go_new_ticker")
	}
	if ht.needsGoTick {
		add("go_tick")
	}
	if ht.needsGoContext {
		add("GoContext", "GoCancelFunc", "GoCancelCauseFunc")
	}
	if ht.needsGoRand {
		add("go_rand_seed", "go_rand_intn", "go_rand_float64", "go_rand_state", "go_rand_next_u64")
	}
	if ht.needsReflect {
		add("GoReflectStructTag", "GoReflectField", "GoReflectType", "GoReflectValue", "GoReflectBoolGetter", "GoReflectBoolSetter", "go_reflect_tag_get")
	}
	if ht.needsGoPtrKey {
		add("GoLocalPtrKey")
	}
	if ht.needsGoAnyPtrKey {
		add("GoAnyPtrKey")
	}

	names := make([]string, 0, len(seen))
	for name := range seen {
		names = append(names, name)
	}
	slices.Sort(names)
	return names
}

func generateGoPtrKeyHelper(out *strings.Builder, name string, includeSliceElemPtr bool) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		if includeSliceElemPtr {
			out.WriteString(`
enum ` + name + `Repr<T> {
    Nil,
    Local(Arc<Mutex<Option<T>>>),
    SliceElem(Arc<Mutex<Option<Vec<T>>>>, usize),
}

pub struct ` + name + `<T>(` + name + `Repr<T>);

impl<T> Clone for ` + name + `<T> {
    fn clone(&self) -> Self {
        match &self.0 {
            ` + name + `Repr::Nil => ` + name + `(` + name + `Repr::Nil),
            ` + name + `Repr::Local(value) => ` + name + `(` + name + `Repr::Local(value.clone())),
            ` + name + `Repr::SliceElem(slice, index) => ` + name + `(` + name + `Repr::SliceElem(slice.clone(), *index)),
        }
    }
}

impl<T> ` + name + `<T> {
    pub fn new(value: Arc<Mutex<Option<T>>>) -> Self {
        if value.lock().unwrap().is_none() {
            ` + name + `(` + name + `Repr::Nil)
        } else {
            ` + name + `(` + name + `Repr::Local(value))
        }
    }

    pub fn value(&self) -> Arc<Mutex<Option<T>>> {
        match &self.0 {
            ` + name + `Repr::Nil => Arc::new(Mutex::new(None)),
            ` + name + `Repr::Local(value) => value.clone(),
            ` + name + `Repr::SliceElem(_, _) => panic!("pointer map key from slice element cannot be converted to a local pointer handle"),
        }
    }

    fn identity(&self) -> (u8, usize, usize) {
        match &self.0 {
            ` + name + `Repr::Nil => (0, 0, 0),
            ` + name + `Repr::Local(value) => (1, Arc::as_ptr(value) as usize, 0),
            ` + name + `Repr::SliceElem(slice, index) => (2, Arc::as_ptr(slice) as usize, *index),
        }
    }

    fn addr(&self) -> usize { let (_, addr, index) = self.identity(); addr ^ index }
}

impl<T: Clone> ` + name + `<T> {
    pub fn from_slice_elem(value: Option<GoSliceElemPtr<T>>) -> Self {
        match value {
            Some(ptr) => ` + name + `(` + name + `Repr::SliceElem(ptr.slice.clone(), ptr.index)),
            None => ` + name + `(` + name + `Repr::Nil),
        }
    }
}

impl<T> PartialEq for ` + name + `<T> {
    fn eq(&self, other: &Self) -> bool { self.identity() == other.identity() }
}
impl<T> Eq for ` + name + `<T> {}
impl<T> PartialOrd for ` + name + `<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for ` + name + `<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.identity().cmp(&other.identity()) }
}
impl<T> std::fmt::Debug for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
`)
			return
		}
		out.WriteString(`
pub struct ` + name + `<T>(pub Arc<Mutex<Option<T>>>);

impl<T> Clone for ` + name + `<T> {
    fn clone(&self) -> Self { ` + name + `(self.0.clone()) }
}

impl<T> ` + name + `<T> {
    pub fn new(value: Arc<Mutex<Option<T>>>) -> Self { ` + name + `(value) }
    pub fn value(&self) -> Arc<Mutex<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Arc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for ` + name + `<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for ` + name + `<T> {}
impl<T> PartialOrd for ` + name + `<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for ` + name + `<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		if includeSliceElemPtr {
			out.WriteString(`
enum ` + name + `Repr<T> {
    Nil,
    Local(Rc<RefCell<Option<T>>>),
    SliceElem(Rc<RefCell<Option<Vec<T>>>>, usize),
}

pub struct ` + name + `<T>(` + name + `Repr<T>);

impl<T> Clone for ` + name + `<T> {
    fn clone(&self) -> Self {
        match &self.0 {
            ` + name + `Repr::Nil => ` + name + `(` + name + `Repr::Nil),
            ` + name + `Repr::Local(value) => ` + name + `(` + name + `Repr::Local(value.clone())),
            ` + name + `Repr::SliceElem(slice, index) => ` + name + `(` + name + `Repr::SliceElem(slice.clone(), *index)),
        }
    }
}

impl<T> ` + name + `<T> {
    pub fn new(value: Rc<RefCell<Option<T>>>) -> Self {
        if value.borrow().is_none() {
            ` + name + `(` + name + `Repr::Nil)
        } else {
            ` + name + `(` + name + `Repr::Local(value))
        }
    }

    pub fn value(&self) -> Rc<RefCell<Option<T>>> {
        match &self.0 {
            ` + name + `Repr::Nil => Rc::new(RefCell::new(None)),
            ` + name + `Repr::Local(value) => value.clone(),
            ` + name + `Repr::SliceElem(_, _) => panic!("pointer map key from slice element cannot be converted to a local pointer handle"),
        }
    }

    fn identity(&self) -> (u8, usize, usize) {
        match &self.0 {
            ` + name + `Repr::Nil => (0, 0, 0),
            ` + name + `Repr::Local(value) => (1, Rc::as_ptr(value) as usize, 0),
            ` + name + `Repr::SliceElem(slice, index) => (2, Rc::as_ptr(slice) as usize, *index),
        }
    }

    fn addr(&self) -> usize { let (_, addr, index) = self.identity(); addr ^ index }
}

impl<T: Clone> ` + name + `<T> {
    pub fn from_slice_elem(value: Option<GoSliceElemPtr<T>>) -> Self {
        match value {
            Some(ptr) => ` + name + `(` + name + `Repr::SliceElem(ptr.slice.clone(), ptr.index)),
            None => ` + name + `(` + name + `Repr::Nil),
        }
    }
}

impl<T> PartialEq for ` + name + `<T> {
    fn eq(&self, other: &Self) -> bool { self.identity() == other.identity() }
}
impl<T> Eq for ` + name + `<T> {}
impl<T> PartialOrd for ` + name + `<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for ` + name + `<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.identity().cmp(&other.identity()) }
}
impl<T> std::fmt::Debug for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
`)
			return
		}
		out.WriteString(`
pub struct ` + name + `<T>(pub Rc<RefCell<Option<T>>>);

impl<T> Clone for ` + name + `<T> {
    fn clone(&self) -> Self { ` + name + `(self.0.clone()) }
}

impl<T> ` + name + `<T> {
    pub fn new(value: Rc<RefCell<Option<T>>>) -> Self { ` + name + `(value) }
    pub fn value(&self) -> Rc<RefCell<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Rc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for ` + name + `<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for ` + name + `<T> {}
impl<T> PartialOrd for ` + name + `<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for ` + name + `<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for ` + name + `<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
`)
	}
}

func generateGoAnyPtrKeyHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GoAnyPtrKey(usize);

impl GoAnyPtrKey {
    pub fn new<T>(value: Arc<Mutex<Option<T>>>) -> Self {
        if value.lock().unwrap().is_none() {
            GoAnyPtrKey(0)
        } else {
            GoAnyPtrKey(Arc::as_ptr(&value) as usize)
        }
    }

    fn addr(&self) -> usize { self.0 }
}

impl std::fmt::Debug for GoAnyPtrKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl std::fmt::Display for GoAnyPtrKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
`)
		return
	}
	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString(`
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GoAnyPtrKey(usize);

impl GoAnyPtrKey {
    pub fn new<T>(value: Rc<RefCell<Option<T>>>) -> Self {
        if value.borrow().is_none() {
            GoAnyPtrKey(0)
        } else {
            GoAnyPtrKey(Rc::as_ptr(&value) as usize)
        }
    }

    fn addr(&self) -> usize { self.0 }
}

impl std::fmt::Debug for GoAnyPtrKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl std::fmt::Display for GoAnyPtrKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
`)
}

// generateAnyPointerFormatter emits go_format_any_pointer, which format_any
// uses to print a pointer to a struct held in an interface.
func generateAnyPointerFormatter(out *strings.Builder) {
	out.WriteString(`
// Go expands a pointer to a struct only when the pointer is the value being
// printed. Inside a struct or a slice it prints the address. That is also
// what keeps a value that points back to itself from printing forever.
fn go_format_any_pointer(nested: bool, address: *const (), is_nil: impl FnOnce() -> bool, expand: impl FnOnce() -> String) -> String {
    if !nested {
        return expand();
    }
    if is_nil() { "<nil>".to_string() } else { format!("{:p}", address) }
}
`)
}

// generateAnyFormatter emits format_any, which prints the value an interface
// holds, and format_any_nested when nestedEntry asks for it. format_any is
// for a value that is itself an argument of a print call. format_any_nested
// is for a value held inside the one being printed: a struct's interface
// field, an element of a []interface{}.
//
// The two differ only for a pointer to a struct, so the distinction is
// generated only when pointerArms says the program boxes one. Each place
// that prints says which of the two it is, and nothing is remembered between
// calls: a String or Error method that prints starts at the top level again,
// as it does in Go.
func generateAnyFormatter(out *strings.Builder, anyFormatArms map[string]anyFormatArm, pointerArms bool, nestedEntry bool, runtimeOps bool) {
	TrackImport("Any")
	valueType := "&dyn Any"
	if NeedsConcurrentWrapper() {
		valueType = "&(dyn Any + Send + Sync)"
	}
	if pointerArms {
		out.WriteString("\nfn format_any(value: " + valueType + ") -> String {\n")
		out.WriteString("    format_any_at(value, false)\n")
		out.WriteString("}\n")
		if nestedEntry {
			out.WriteString("\n// Prints a value held inside the one being printed: a struct's interface\n")
			out.WriteString("// field, an element of a []interface{}.\n")
			out.WriteString("fn format_any_nested(value: " + valueType + ") -> String {\n")
			out.WriteString("    format_any_at(value, true)\n")
			out.WriteString("}\n")
		}
		out.WriteString("\nfn format_any_at(value: " + valueType + ", nested: bool) -> String {\n")
	} else {
		out.WriteString("\nfn format_any(value: " + valueType + ") -> String {\n")
	}
	out.WriteString("    if let Some(v) = value.downcast_ref::<i32>() {\n")
	out.WriteString("        v.to_string()\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<i64>() {\n")
	out.WriteString("        v.to_string()\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<f64>() {\n")
	out.WriteString("        go_float_string(*v, false)\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<f32>() {\n")
	out.WriteString("        go_float_string(*v as f64, true)\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<String>() {\n")
	out.WriteString("        v.clone()\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<&str>() {\n")
	out.WriteString("        v.to_string()\n")
	out.WriteString("    } else if let Some(v) = value.downcast_ref::<bool>() {\n")
	out.WriteString("        v.to_string()\n")
	// The other integer types print as plain numbers too.
	for _, intType := range []string{"i8", "i16", "isize", "u8", "u16", "u32", "u64", "usize"} {
		out.WriteString("    } else if let Some(v) = value.downcast_ref::<" + intType + ">() {\n")
		out.WriteString("        v.to_string()\n")
	}
	rustTypes := make([]string, 0, len(anyFormatArms))
	for rustType := range anyFormatArms {
		rustTypes = append(rustTypes, rustType)
	}
	slices.Sort(rustTypes)
	for _, rustType := range rustTypes {
		trackImportsForAnyPayloadType(rustType)
		out.WriteString("    } else if let Some(v) = value.downcast_ref::<")
		out.WriteString(rustType)
		out.WriteString(">() {\n")
		out.WriteString("        ")
		out.WriteString(anyFormatArms[rustType].expr)
		out.WriteString("\n")
	}
	// A value of a type with no case above cannot be printed yet. Stop
	// with a message that says so: printing a placeholder would be a wrong
	// answer that looks like a right one.
	if runtimeOps {
		out.WriteString("    } else if let Some(__go_text) = go_format_registered_any(value) {\n")
		out.WriteString("        __go_text\n")
	}
	out.WriteString("    } else {\n")
	out.WriteString("        panic!(\"go2rust: cannot print this value held in an interface: its type has no formatting case yet\")\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
	if !pointerArms && nestedEntry {
		out.WriteString("\n// Prints a value held inside the one being printed. Nothing this program\n")
		out.WriteString("// puts in an interface prints differently there.\n")
		out.WriteString("fn format_any_nested(value: " + valueType + ") -> String {\n")
		out.WriteString("    format_any(value)\n")
		out.WriteString("}\n")
	}
}

func sortedAnyCloneTypes(anyCloneTypes map[string]bool) []string {
	names := make([]string, 0, len(anyCloneTypes))
	for name := range anyCloneTypes {
		if name != "" {
			names = append(names, name)
		}
	}
	slices.Sort(names)
	return names
}

func writeAnyCloneTypeArms(out *strings.Builder, anyCloneTypes map[string]bool, traitObject string) {
	for _, rustType := range sortedAnyCloneTypes(anyCloneTypes) {
		trackImportsForAnyPayloadType(rustType)
		out.WriteString("    if let Some(v) = value.downcast_ref::<")
		out.WriteString(rustType)
		out.WriteString(">() { return Some(Box::new(v.clone()) as ")
		out.WriteString(traitObject)
		out.WriteString("); }\n")
	}
}

// generateAnyClone writes the copies of interface{} values. go_any_try_clone
// copies a dynamic value whose type the program boxes; go_any_clone does the
// same and stops the program for any other type. go_any_handle_copy and
// go_any_store give a variable Go's copy of an interface value, the same
// dynamic value in a cell of its own; a value whose type has no copy keeps
// sharing the source's cell, as every interface copy did before.
func generateAnyClone(out *strings.Builder, anyCloneTypes map[string]bool, withHandles bool) {
	TrackImport("Any")
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	traitObject := "Box<dyn Any>"
	valueRef := "&dyn Any"
	handle := "Rc<RefCell<Option<Box<dyn Any>>>>"
	newCell := "Rc::new(RefCell::new("
	read := ".borrow()"
	write := ".borrow_mut()"
	pointer := "Rc"
	if NeedsConcurrentWrapper() {
		traitObject = "Box<dyn Any + Send + Sync>"
		valueRef = "&(dyn Any + Send + Sync)"
		handle = "Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>"
		newCell = "Arc::new(Mutex::new("
		read = ".lock().unwrap()"
		write = ".lock().unwrap()"
		pointer = "Arc"
		TrackImport("Arc")
		TrackImport("Mutex")
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
	}
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString("fn go_any_try_clone(value: " + valueRef + ") -> Option<" + traitObject + "> {\n")
	for _, primitive := range []string{"i32", "i64", "i8", "i16", "u32", "u64", "u8", "u16", "usize", "isize", "f64", "f32", "&'static str", "bool", "char"} {
		out.WriteString("    if let Some(v) = value.downcast_ref::<" + primitive + ">() { return Some(Box::new(*v) as " + traitObject + "); }\n")
	}
	out.WriteString("    if let Some(v) = value.downcast_ref::<String>() { return Some(Box::new(v.clone()) as " + traitObject + "); }\n")
	writeAnyCloneTypeArms(out, anyCloneTypes, traitObject)
	if programAnyRuntimeOpsEnabled() {
		out.WriteString("    go_clone_registered_any(value)\n}\n\n")
	} else {
		out.WriteString("    None\n}\n\n")
	}
	out.WriteString(visibility)
	out.WriteString("fn go_any_clone(value: " + valueRef + ") -> " + traitObject + " {\n")
	out.WriteString("    go_any_try_clone(value).unwrap_or_else(|| panic!(\"go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>\"))\n}\n")
	if !withHandles {
		return
	}
	out.WriteString("\n#[allow(dead_code)]\n")
	out.WriteString(visibility)
	out.WriteString("fn go_any_handle_copy(handle: &" + handle + ") -> " + handle + " {\n")
	out.WriteString("    let __copy = match handle" + read + ".as_ref() {\n")
	out.WriteString("        None => None,\n")
	out.WriteString("        Some(value) => match go_any_try_clone(&**value) {\n")
	out.WriteString("            Some(copy) => Some(copy),\n")
	out.WriteString("            None => return handle.clone(),\n")
	out.WriteString("        },\n")
	out.WriteString("    };\n")
	out.WriteString("    " + newCell + "__copy))\n}\n\n")
	out.WriteString("/// Stores Go's copy of source's value in target's cell. A source that no\n")
	out.WriteString("/// one else holds gives up its value; a value whose type has no copy comes\n")
	out.WriteString("/// back for the caller to share instead.\n")
	out.WriteString("#[allow(dead_code)]\n")
	out.WriteString(visibility)
	out.WriteString("fn go_any_store(target: &" + handle + ", source: " + handle + ") -> Option<" + handle + "> {\n")
	out.WriteString("    if " + pointer + "::ptr_eq(target, &source) {\n        return None;\n    }\n")
	out.WriteString("    let __value = if " + pointer + "::strong_count(&source) == 1 {\n")
	out.WriteString("        source" + write + ".take()\n")
	out.WriteString("    } else {\n")
	out.WriteString("        let __copy = match source" + read + ".as_ref() {\n")
	out.WriteString("            None => None,\n")
	out.WriteString("            Some(value) => match go_any_try_clone(&**value) {\n")
	out.WriteString("                Some(copy) => Some(copy),\n")
	out.WriteString("                None => return Some(source.clone()),\n")
	out.WriteString("            },\n")
	out.WriteString("        };\n")
	out.WriteString("        __copy\n")
	out.WriteString("    };\n")
	out.WriteString("    *target" + write + " = __value;\n")
	out.WriteString("    None\n}\n")
}

// generateGoPointerError emits an error adapter that keeps the original Go
// pointer handle. Error interface bindings must observe later whole-value
// replacement through that handle instead of snapshotting the pointee.
func generateGoPointerError(out *strings.Builder) {
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString("\n")
		out.WriteString(visibility)
		out.WriteString(`struct GoPointerError<T> {
    handle: Arc<Mutex<Option<T>>>,
}

impl<T> GoPointerError<T> {
    fn new(handle: Arc<Mutex<Option<T>>>) -> Self { Self { handle } }
}

impl<T: std::fmt::Display> std::fmt::Display for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.handle.lock().unwrap();
        match guard.as_ref() {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<T: std::fmt::Display + Send + Sync + 'static> std::error::Error for GoPointerError<T> {}
`)
		return
	}
	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`struct GoPointerError<T> {
    handle: Rc<RefCell<Option<T>>>,
}

impl<T> GoPointerError<T> {
    fn new(handle: Rc<RefCell<Option<T>>>) -> Self { Self { handle } }
}

impl<T: std::fmt::Display> std::fmt::Display for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.handle.borrow();
        match guard.as_ref() {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<T: std::fmt::Display + 'static> std::error::Error for GoPointerError<T> {}
`)
}

func generatePanicRecoverHelper(out *strings.Builder) {
	if !NeedsConcurrentWrapper() {
		generateSingleThreadPanicRecoverHelper(out)
		return
	}
	TrackImport("Any")
	TrackImport("Arc")
	TrackImport("Mutex")
	TrackImport("RefCell")
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	out.WriteString("\n")
	out.WriteString(`thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send + Sync>>> = RefCell::new(None);
}
`)
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Arc::new(Mutex::new(slot.borrow_mut().take())))
}
`)
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_store_panic_payload(payload: Box<dyn Any + Send>) {
    let payload = match payload.downcast::<Box<dyn Any + Send + Sync>>() {
        Ok(boxed) => {
            let mut payload = *boxed;
            loop {
                match payload.downcast::<Box<dyn Any + Send + Sync>>() {
                    Ok(boxed) => {
                        payload = *boxed;
                    }
                    Err(payload) => {
                        __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(payload));
                        return;
                    }
                }
            }
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<String>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text_of_string(*value)) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(value.to_string())) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i32>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i64>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let _payload = match payload.downcast::<bool>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(_payload) => _payload,
    };
    panic!("recover: unsupported Rust panic payload; emit panic_any with a Go any payload instead")
}
`)
	writePanicHookSilencer(out, visibility)
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_resume_unrecovered_panic() {
    if let Some(payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        std::panic::panic_any(payload);
    }
}
`)
}

// generatePanicExitHelper writes go_install_panic_exit, which main calls
// before anything else. Its panic hook ends the program the way Go's
// runtime does for a panic that nothing recovers: it prints "panic: " and
// the value on stderr and exits with status 2 (DEV-194). A function with a
// defer runs its body under catch_unwind, and go_silence_panic_hook wraps
// this hook so that it runs only when no such catch is active on the
// thread; by then every deferred call on the way has run (DEV-162). Each
// crate of the program wraps the hook it finds, so main installs it first.
//
// A panic's value prints as Go prints it for the shapes the panic site
// raises. Without goroutines or recover() a panic raises its text. With
// them it can raise the Go value boxed as an interface value, or raise a
// recovered payload again: a string, an error's Error() or a number print
// as Go prints them, and when the program formats interface values the
// other kinds print as format_any does.
func generatePanicExitHelper(out *strings.Builder, withBoxedValues bool, withFormatAny bool) {
	TrackImport("Any")
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_install_panic_exit() {
    // Go lets a program call main again; the hook goes in once, so a
    // later call keeps the hook that go_silence_panic_hook wrapped around it.
    static __GO_PANIC_EXIT: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_EXIT.call_once(|| {
        std::panic::set_hook(Box::new(|__info| {
            let __text = go_panic_payload_text(__info.payload());
            {
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
            eprintln!("panic: {}", __text);
            std::process::exit(2);
        }));
    });
}
`)
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text_of_string(__text.clone());
    }
`)
	if withBoxedValues {
		out.WriteString(`    go_panic_value_text(__payload)
}

`)
		out.WriteString(visibility)
		out.WriteString(`fn go_panic_value_text(__payload: &dyn Any) -> String {
    // A boxed payload holds the Go value itself, so its string is printed
    // as it is.
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return __text.clone();
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return __text.to_string();
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send + Sync>>() {
        let __text = go_panic_value_text(&**__inner);
`)
		if withFormatAny {
			out.WriteString(`        if __text == GO_UNPRINTABLE_PANIC_VALUE {
            return format_any(&**__inner);
        }
`)
		}
		out.WriteString(`        return __text;
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__err) = __payload.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>>() {
        return __err.lock().unwrap().as_ref().map(|__err| __err.to_string()).unwrap_or_else(|| String::from("nil"));
    }
    macro_rules! go_panic_display_text {
        ($($t:ty),*) => {
            $(if let Some(__value) = __payload.downcast_ref::<$t>() {
                return __value.to_string();
            })*
        };
    }
    go_panic_display_text!(i64, i32, i16, i8, u64, u32, u16, u8, usize, isize, bool, char);
    if let Some(__value) = __payload.downcast_ref::<f64>() {
        // Go prints a float in a panic as +1.500000e+000.
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if *__value > 0.0 { "+Inf" } else { "-Inf" });
        }
        let __text = format!("{:+.6e}", __value);
        if let Some((__mantissa, __exponent)) = __text.split_once('e') {
            let __exponent: i32 = __exponent.parse().unwrap_or(0);
            return format!("{}e{}{:03}", __mantissa, if __exponent < 0 { '-' } else { '+' }, __exponent.abs());
        }
        return __text;
    }
`)
	}
	out.WriteString(`    String::from(GO_UNPRINTABLE_PANIC_VALUE)
}

const GO_UNPRINTABLE_PANIC_VALUE: &str = "(unprintable panic value)";
`)
}

// generateSingleThreadPanicRecoverHelper is the panic/recover runtime for a
// program without goroutines, where an interface value is a
// Rc<RefCell<Option<Box<dyn Any>>>>. A function with a defer runs its body
// under catch_unwind; a panic's payload is kept here until a deferred
// function calls recover(), which takes it. If none does, the panic goes on.
//
// The payload of panic("x") is the string. A panic the Rust code raises
// itself, such as a division by zero or an index out of range, is given the
// text Go's runtime error has.
func generateSingleThreadPanicRecoverHelper(out *strings.Builder) {
	TrackImport("Any")
	TrackImport("Rc")
	TrackImport("RefCell")
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	out.WriteString(`
thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send>>> = RefCell::new(None);
}
`)
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_recover() -> Rc<RefCell<Option<Box<dyn Any>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Rc::new(RefCell::new(slot.borrow_mut().take().map(|__payload| __payload as Box<dyn Any>))))
}
`)
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_store_panic_payload(mut __payload: Box<dyn Any + Send>) {
    // A payload raised again by go_resume_unrecovered_panic arrives boxed
    // once more.
    loop {
        match __payload.downcast::<Box<dyn Any + Send>>() {
            Ok(__inner) => __payload = *__inner,
            Err(__outer) => {
                __payload = __outer;
                break;
            }
        }
    }
    let __payload: Box<dyn Any + Send> = match __payload.downcast::<&'static str>() {
        Ok(__text) => Box::new(go_runtime_error_text(__text.to_string())),
        Err(__payload) => match __payload.downcast::<String>() {
            Ok(__text) => Box::new(go_runtime_error_text_of_string(*__text)),
            Err(__payload) => __payload,
        },
    };
    __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(__payload));
}
`)
	writePanicHookSilencer(out, visibility)
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_resume_unrecovered_panic() {
    if let Some(__payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        match __payload.downcast::<String>() {
            Ok(__text) => std::panic::panic_any(*__text),
            // panic_any, not resume_unwind: the panic hook must see a panic
            // that nothing recovers, to end the program as Go does.
            Err(__payload) => std::panic::panic_any(__payload),
        }
    }
}
`)
}

// writePanicHookSilencer writes go_silence_panic_hook and
// go_restore_panic_hook. While a function with a defer runs its body under
// catch_unwind, Rust's panic hook must not print a panic that a deferred
// recover() may still stop. The hook is one per process, so swapping it in
// and out races when goroutines on several threads do it at once: one
// thread puts the printing hook back while another is panicking. Instead a
// hook installed once checks a per-thread depth, and each catch raises the
// depth on its own thread and puts it back afterwards.
func writePanicHookSilencer(out *strings.Builder, visibility string) {
	out.WriteString(`
thread_local! {
    static __GO_PANIC_HOOK_SILENCE: std::cell::Cell<usize> = std::cell::Cell::new(0);
}
`)
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_silence_panic_hook() -> usize {
    static __GO_PANIC_HOOK: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_HOOK.call_once(|| {
        let __previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |__info| {
            if __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.get()) == 0 {
                __previous(__info);
            }
        }));
    });
    __GO_PANIC_HOOK_SILENCE.with(|__depth| {
        let __previous = __depth.get();
        __depth.set(__previous + 1);
        __previous
    })
}
`)
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_restore_panic_hook(__previous: usize) {
    __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.set(__previous));
}
`)
}

// writeGoRuntimeErrorText writes go_runtime_error_text, which gives a panic
// the Rust code raises itself the text of Go's runtime error.
func writeGoRuntimeErrorText(out *strings.Builder, visibility string) {
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    // A nil pointer's cell is empty, so reading through it unwraps None.
    if __text == "called ` + "`Option::unwrap()`" + ` on a ` + "`None`" + ` value" || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return "runtime error: invalid memory address or nil pointer dereference".to_string();
    }
    __text
}
`)
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_runtime_error_text_of_string(__text: String) -> String {
    // panic(s) in Go raises a String, and so does a runtime error whose text
    // Rust formats: an index out of range, or a nil pointer read through
    // expect(). The constant runtime errors arrive as a &'static str, so a
    // Go string that reads like one keeps its text.
    if __text.starts_with("index out of bounds: ") || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return go_runtime_error_text(__text);
    }
    __text
}
`)
}

func generateGoValueClone(out *strings.Builder) {
	TrackImport("Any")
	if NeedsConcurrentWrapper() {
		out.WriteString(`
pub trait GoValueClone {
    fn go_value_clone(&self) -> Self;
}

macro_rules! impl_go_value_clone_copy {
    ($($t:ty),* $(,)?) => {
        $(impl GoValueClone for $t {
            fn go_value_clone(&self) -> Self { *self }
        })*
    };
}

impl_go_value_clone_copy!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, &'static str);

impl GoValueClone for String {
    fn go_value_clone(&self) -> Self { self.clone() }
}

impl GoValueClone for Box<dyn Any + Send + Sync> {
    fn go_value_clone(&self) -> Self { go_any_clone(self.as_ref()) }
}
`)
		return
	}
	out.WriteString(`
pub trait GoValueClone {
    fn go_value_clone(&self) -> Self;
}

macro_rules! impl_go_value_clone_copy {
    ($($t:ty),* $(,)?) => {
        $(impl GoValueClone for $t {
            fn go_value_clone(&self) -> Self { *self }
        })*
    };
}

impl_go_value_clone_copy!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, &'static str);

impl GoValueClone for String {
    fn go_value_clone(&self) -> Self { self.clone() }
}

impl GoValueClone for Box<dyn Any> {
    fn go_value_clone(&self) -> Self { go_any_clone(self.as_ref()) }
}
	`)
}

func generateGoTypeParamFieldClone(out *strings.Builder) {
	// The legacy GoValueClone helper ends with an indentation tab. Preserve
	// existing generated snapshots when this helper is not needed, but do not
	// leave a whitespace-only line before the extension trait.
	if current := out.String(); strings.HasSuffix(current, "\n\t") {
		out.Reset()
		out.WriteString(strings.TrimSuffix(current, "\t"))
	}
	NeedGoValueClone()
	if NeedsConcurrentWrapper() {
		out.WriteString(goTypeParamFieldCloneConcurrent)
		return
	}
	out.WriteString(goTypeParamFieldCloneLocal)
}

const goTypeParamFieldCloneConcurrent = `
pub trait GoTypeParamFieldClone: Sized {
    const GO_COPY_KEEPS_HANDLE: bool;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>>;
}

macro_rules! impl_go_type_param_field_clone_value {
    ($($t:ty),* $(,)?) => {
        $(impl GoTypeParamFieldClone for $t {
            const GO_COPY_KEEPS_HANDLE: bool = false;
            fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> {
                let guard = field.lock().unwrap();
                std::sync::Arc::new(std::sync::Mutex::new(guard.as_ref().map(|value| value.go_value_clone())))
            }
        })*
    };
}

impl_go_type_param_field_clone_value!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, &'static str, String, Box<dyn Any + Send + Sync>);

impl<T: GoTypeParamFieldClone + GoValueClone, const N: usize> GoValueClone for [T; N] {
    fn go_value_clone(&self) -> Self { self.each_ref().map(|value| value.go_value_clone()) }
}
impl<T: GoTypeParamFieldClone + GoValueClone, const N: usize> GoTypeParamFieldClone for [T; N] {
    const GO_COPY_KEEPS_HANDLE: bool = T::GO_COPY_KEEPS_HANDLE;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> {
        if Self::GO_COPY_KEEPS_HANDLE { field.clone() } else {
            let guard = field.lock().unwrap();
            std::sync::Arc::new(std::sync::Mutex::new(guard.as_ref().map(|value| value.go_value_clone())))
        }
    }
}

impl<T: Clone> GoValueClone for std::vec::Vec<T> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<T: Clone> GoTypeParamFieldClone for std::vec::Vec<T> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> { field.clone() }
}

impl<K: Clone + Ord, V: Clone> GoValueClone for std::collections::BTreeMap<K, V> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<K: Clone + Ord, V: Clone> GoTypeParamFieldClone for std::collections::BTreeMap<K, V> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> { field.clone() }
}

impl<T> GoValueClone for std::sync::Arc<std::sync::Mutex<Option<T>>> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<T> GoTypeParamFieldClone for std::sync::Arc<std::sync::Mutex<Option<T>>> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> { field.clone() }
}

// A func value is a shared closure: a copy of the field is a new cell
// around the same closure.
macro_rules! impl_go_type_param_field_clone_fn {
    ($(($($arg:ident),*)),* $(,)?) => { $(impl<R: 'static, $($arg: 'static),*> GoTypeParamFieldClone for std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync> {
        const GO_COPY_KEEPS_HANDLE: bool = false;
        fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> {
            std::sync::Arc::new(std::sync::Mutex::new(field.lock().unwrap().clone()))
        }
    })* };
}
impl_go_type_param_field_clone_fn!((), (A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7));
`

const goTypeParamFieldCloneLocal = `
pub trait GoTypeParamFieldClone: Sized {
    const GO_COPY_KEEPS_HANDLE: bool;
    fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>>;
}

macro_rules! impl_go_type_param_field_clone_value {
    ($($t:ty),* $(,)?) => {
        $(impl GoTypeParamFieldClone for $t {
            const GO_COPY_KEEPS_HANDLE: bool = false;
            fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> {
                let guard = field.borrow();
                std::rc::Rc::new(std::cell::RefCell::new(guard.as_ref().map(|value| value.go_value_clone())))
            }
        })*
    };
}

impl_go_type_param_field_clone_value!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, &'static str, String, Box<dyn Any>);

impl<T: GoTypeParamFieldClone + GoValueClone, const N: usize> GoValueClone for [T; N] {
    fn go_value_clone(&self) -> Self { self.each_ref().map(|value| value.go_value_clone()) }
}
impl<T: GoTypeParamFieldClone + GoValueClone, const N: usize> GoTypeParamFieldClone for [T; N] {
    const GO_COPY_KEEPS_HANDLE: bool = T::GO_COPY_KEEPS_HANDLE;
    fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> {
        if Self::GO_COPY_KEEPS_HANDLE { field.clone() } else {
            let guard = field.borrow();
            std::rc::Rc::new(std::cell::RefCell::new(guard.as_ref().map(|value| value.go_value_clone())))
        }
    }
}

impl<T: Clone> GoValueClone for std::vec::Vec<T> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<T: Clone> GoTypeParamFieldClone for std::vec::Vec<T> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> { field.clone() }
}

impl<K: Clone + Ord, V: Clone> GoValueClone for std::collections::BTreeMap<K, V> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<K: Clone + Ord, V: Clone> GoTypeParamFieldClone for std::collections::BTreeMap<K, V> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> { field.clone() }
}

impl<T> GoValueClone for std::rc::Rc<std::cell::RefCell<Option<T>>> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<T> GoTypeParamFieldClone for std::rc::Rc<std::cell::RefCell<Option<T>>> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> { field.clone() }
}

// A func value is a shared closure: a copy of the field is a new cell
// around the same closure.
macro_rules! impl_go_type_param_field_clone_fn {
    ($(($($arg:ident),*)),* $(,)?) => { $(impl<R: 'static, $($arg: 'static),*> GoTypeParamFieldClone for std::rc::Rc<dyn Fn($($arg),*) -> R> {
        const GO_COPY_KEEPS_HANDLE: bool = false;
        fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> {
            std::rc::Rc::new(std::cell::RefCell::new(field.borrow().clone()))
        }
    })* };
}
impl_go_type_param_field_clone_fn!((), (A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7));
`

func generateGoConstStrEq(out *strings.Builder) {
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	fmt.Fprintf(out, `
%[1]sconst fn go_const_str_eq(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() != right.len() {
        return false;
    }
    let mut i = 0;
    while i < left.len() {
        if left[i] != right[i] {
            return false;
        }
        i += 1;
    }
    true
}
`, visibility)
}

func generateGoAnyTypeMetadata(out *strings.Builder) {
	TrackImport("Any")
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	if NeedsConcurrentWrapper() {
		fmt.Fprintf(out, `
#[derive(Clone, Copy)]
%[1]sstruct GoAnyTypeMetadata {
    pub kind: &'static str,
    pub comparable: bool,
    pub elem_kind: Option<&'static str>,
    pub elem_comparable: bool,
}

fn go_any_type_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_key(value: &(dyn Any + Send + Sync)) -> usize {
    value as *const (dyn Any + Send + Sync) as *const () as usize
}

%[1]sfn go_register_any_type<T: Any + Send + Sync + 'static>(kind: &'static str, comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

%[1]sfn go_register_any_type_with_elem<T: Any + Send + Sync + 'static>(kind: &'static str, comparable: bool, elem_kind: &'static str, elem_comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: Some(elem_kind), elem_comparable });
}

// Boxes value as itself and records what kind of Go type it is. Every reader
// of an interface value looks for the value's own type in the box, so the
// box must hold nothing else: a wrapper around the value would be a type no
// reader knows.
%[1]sfn go_box_any_with_metadata<T: Any + Send + Sync + 'static>(value: T, kind: &'static str, comparable: bool) -> Box<dyn Any + Send + Sync> {
    go_register_any_type::<T>(kind, comparable);
    Box::new(value) as Box<dyn Any + Send + Sync>
}

%[1]sfn go_register_any_value_metadata(value: &(dyn Any + Send + Sync), kind: &'static str, comparable: bool) {
    go_any_value_metadata_registry().lock().unwrap().insert(go_any_value_metadata_key(value), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

%[1]sfn go_any_type_metadata(value: &(dyn Any + Send + Sync)) -> Option<GoAnyTypeMetadata> {
    go_any_value_metadata_registry().lock().unwrap().get(&go_any_value_metadata_key(value)).copied()
        .or_else(|| go_any_type_metadata_registry().lock().unwrap().get(&value.type_id()).copied())
}
`, visibility)
		return
	}
	fmt.Fprintf(out, `
#[derive(Clone, Copy)]
%[1]sstruct GoAnyTypeMetadata {
    pub kind: &'static str,
    pub comparable: bool,
    pub elem_kind: Option<&'static str>,
    pub elem_comparable: bool,
}

fn go_any_type_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_key(value: &dyn Any) -> usize {
    value as *const dyn Any as *const () as usize
}

%[1]sfn go_register_any_type<T: Any + 'static>(kind: &'static str, comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

%[1]sfn go_register_any_type_with_elem<T: Any + 'static>(kind: &'static str, comparable: bool, elem_kind: &'static str, elem_comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: Some(elem_kind), elem_comparable });
}

// Boxes value as itself and records what kind of Go type it is. Every reader
// of an interface value looks for the value's own type in the box, so the
// box must hold nothing else: a wrapper around the value would be a type no
// reader knows.
%[1]sfn go_box_any_with_metadata<T: Any + 'static>(value: T, kind: &'static str, comparable: bool) -> Box<dyn Any> {
    go_register_any_type::<T>(kind, comparable);
    Box::new(value) as Box<dyn Any>
}

%[1]sfn go_register_any_value_metadata(value: &dyn Any, kind: &'static str, comparable: bool) {
    go_any_value_metadata_registry().lock().unwrap().insert(go_any_value_metadata_key(value), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

%[1]sfn go_any_type_metadata(value: &dyn Any) -> Option<GoAnyTypeMetadata> {
    go_any_value_metadata_registry().lock().unwrap().get(&go_any_value_metadata_key(value)).copied()
        .or_else(|| go_any_type_metadata_registry().lock().unwrap().get(&value.type_id()).copied())
}
`, visibility)
}

// generateGoAnyRuntimeOps emits a TypeId keyed registry of erased operations
// for interface payloads whose concrete Rust type belongs to another generated
// crate. The declaring crate registers non-capturing downcast functions before
// boxing a value; consumers can then preserve Go formatting, copy, %T, and
// error-assertion semantics without naming that concrete type.
func generateGoAnyRuntimeOps(out *strings.Builder) {
	TrackImport("Any")
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	if NeedsConcurrentWrapper() {
		fmt.Fprintf(out, `
#[derive(Clone, Copy)]
%[1]sstruct GoAnyRuntimeOps {
    format: fn(&(dyn Any + Send + Sync)) -> String,
    clone_value: fn(&(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync>,
    type_name: &'static str,
    as_error: Option<fn(&(dyn Any + Send + Sync)) -> std::sync::Arc<dyn std::error::Error + Send + Sync>>,
}

fn go_any_runtime_ops_registry() -> &'static std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyRuntimeOps>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyRuntimeOps>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

%[1]sfn go_register_any_runtime_type<T: Any + Send + Sync + 'static>(
    type_name: &'static str,
    format: fn(&(dyn Any + Send + Sync)) -> String,
    clone_value: fn(&(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync>,
    as_error: Option<fn(&(dyn Any + Send + Sync)) -> std::sync::Arc<dyn std::error::Error + Send + Sync>>,
) {
    go_any_runtime_ops_registry().lock().unwrap().entry(std::any::TypeId::of::<T>())
        .or_insert(GoAnyRuntimeOps { format, clone_value, type_name, as_error });
}

%[1]sfn go_format_registered_any(value: &(dyn Any + Send + Sync)) -> Option<String> {
    let op = go_any_runtime_ops_registry().lock().unwrap().get(&value.type_id()).copied().map(|ops| ops.format);
    op.map(|format| format(value))
}

%[1]sfn go_clone_registered_any(value: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
    let op = go_any_runtime_ops_registry().lock().unwrap().get(&value.type_id()).copied().map(|ops| ops.clone_value);
    op.map(|clone_value| clone_value(value))
}

%[1]sfn go_registered_any_type_name(value: &(dyn Any + Send + Sync)) -> Option<&'static str> {
    go_any_runtime_ops_registry().lock().unwrap().get(&value.type_id()).copied().map(|ops| ops.type_name)
}

%[1]sfn go_registered_any_is_error(value: &(dyn Any + Send + Sync)) -> bool {
    go_any_runtime_ops_registry().lock().unwrap().get(&value.type_id()).copied().and_then(|ops| ops.as_error).is_some()
}

%[1]sfn go_registered_any_error(value: &(dyn Any + Send + Sync)) -> Option<std::sync::Arc<dyn std::error::Error + Send + Sync>> {
    let op = go_any_runtime_ops_registry().lock().unwrap().get(&value.type_id()).copied().and_then(|ops| ops.as_error);
    op.map(|as_error| as_error(value))
}
`, visibility)
		return
	}
	fmt.Fprintf(out, `
#[derive(Clone, Copy)]
%[1]sstruct GoAnyRuntimeOps {
    format: fn(&dyn Any) -> String,
    clone_value: fn(&dyn Any) -> Box<dyn Any>,
    type_name: &'static str,
    as_error: Option<fn(&dyn Any) -> std::rc::Rc<dyn std::error::Error>>,
}

fn go_any_runtime_ops_registry() -> &'static std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyRuntimeOps>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyRuntimeOps>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

%[1]sfn go_register_any_runtime_type<T: Any + 'static>(
    type_name: &'static str,
    format: fn(&dyn Any) -> String,
    clone_value: fn(&dyn Any) -> Box<dyn Any>,
    as_error: Option<fn(&dyn Any) -> std::rc::Rc<dyn std::error::Error>>,
) {
    go_any_runtime_ops_registry().lock().unwrap().entry(std::any::TypeId::of::<T>())
        .or_insert(GoAnyRuntimeOps { format, clone_value, type_name, as_error });
}

%[1]sfn go_format_registered_any(value: &dyn Any) -> Option<String> {
    let op = go_any_runtime_ops_registry().lock().unwrap().get(&value.type_id()).copied().map(|ops| ops.format);
    op.map(|format| format(value))
}

%[1]sfn go_clone_registered_any(value: &dyn Any) -> Option<Box<dyn Any>> {
    let op = go_any_runtime_ops_registry().lock().unwrap().get(&value.type_id()).copied().map(|ops| ops.clone_value);
    op.map(|clone_value| clone_value(value))
}

%[1]sfn go_registered_any_type_name(value: &dyn Any) -> Option<&'static str> {
    go_any_runtime_ops_registry().lock().unwrap().get(&value.type_id()).copied().map(|ops| ops.type_name)
}

%[1]sfn go_registered_any_is_error(value: &dyn Any) -> bool {
    go_any_runtime_ops_registry().lock().unwrap().get(&value.type_id()).copied().and_then(|ops| ops.as_error).is_some()
}

%[1]sfn go_registered_any_error(value: &dyn Any) -> Option<std::rc::Rc<dyn std::error::Error>> {
    let op = go_any_runtime_ops_registry().lock().unwrap().get(&value.type_id()).copied().and_then(|ops| ops.as_error);
    op.map(|as_error| as_error(value))
}
`, visibility)
}

func generateAnyEquality(out *strings.Builder) {
	TrackImport("Any")
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
fn go_any_eq(left: &Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, right: &Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool {
    if Arc::ptr_eq(left, right) {
        let guard = left.lock().unwrap();
        return match guard.as_ref() {
            None => true,
            Some(value) => go_any_values_eq(value.as_ref(), value.as_ref()),
        };
    }
    let left_guard = left.lock().unwrap();
    let right_guard = right.lock().unwrap();
    match (left_guard.as_ref(), right_guard.as_ref()) {
        (None, None) => true,
        (None, Some(_)) | (Some(_), None) => false,
        (Some(left), Some(right)) => go_any_values_eq(left.as_ref(), right.as_ref()),
    }
}

fn go_any_values_eq(left: &(dyn Any + Send + Sync), right: &(dyn Any + Send + Sync)) -> bool {
    if left.type_id() != right.type_id() {
        return false;
    }
    if let Some(v) = left.downcast_ref::<i32>() { return right.downcast_ref::<i32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i64>() { return right.downcast_ref::<i64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i8>() { return right.downcast_ref::<i8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i16>() { return right.downcast_ref::<i16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u32>() { return right.downcast_ref::<u32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u64>() { return right.downcast_ref::<u64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u8>() { return right.downcast_ref::<u8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u16>() { return right.downcast_ref::<u16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<usize>() { return right.downcast_ref::<usize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<isize>() { return right.downcast_ref::<isize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f64>() { return right.downcast_ref::<f64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f32>() { return right.downcast_ref::<f32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<String>() { return right.downcast_ref::<String>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<&str>() { return right.downcast_ref::<&str>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<bool>() { return right.downcast_ref::<bool>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<char>() { return right.downcast_ref::<char>().map_or(false, |r| v == r); }
    panic!("interface comparison with uncomparable dynamic type")
}
`)
		return
	}
	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString(`
fn go_any_eq(left: &Rc<RefCell<Option<Box<dyn Any>>>>, right: &Rc<RefCell<Option<Box<dyn Any>>>>) -> bool {
    if Rc::ptr_eq(left, right) {
        let guard = left.borrow();
        return match guard.as_ref() {
            None => true,
            Some(value) => go_any_values_eq(value.as_ref(), value.as_ref()),
        };
    }
    let left_guard = left.borrow();
    let right_guard = right.borrow();
    match (left_guard.as_ref(), right_guard.as_ref()) {
        (None, None) => true,
        (None, Some(_)) | (Some(_), None) => false,
        (Some(left), Some(right)) => go_any_values_eq(left.as_ref(), right.as_ref()),
    }
}

fn go_any_values_eq(left: &dyn Any, right: &dyn Any) -> bool {
    if left.type_id() != right.type_id() {
        return false;
    }
    if let Some(v) = left.downcast_ref::<i32>() { return right.downcast_ref::<i32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i64>() { return right.downcast_ref::<i64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i8>() { return right.downcast_ref::<i8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i16>() { return right.downcast_ref::<i16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u32>() { return right.downcast_ref::<u32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u64>() { return right.downcast_ref::<u64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u8>() { return right.downcast_ref::<u8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u16>() { return right.downcast_ref::<u16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<usize>() { return right.downcast_ref::<usize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<isize>() { return right.downcast_ref::<isize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f64>() { return right.downcast_ref::<f64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f32>() { return right.downcast_ref::<f32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<String>() { return right.downcast_ref::<String>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<&str>() { return right.downcast_ref::<&str>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<bool>() { return right.downcast_ref::<bool>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<char>() { return right.downcast_ref::<char>().map_or(false, |r| v == r); }
    panic!("interface comparison with uncomparable dynamic type")
}
`)
}

func generateGoComparable(out *strings.Builder) {
	TrackImport("Any")
	anyType := "dyn Any"
	if NeedsConcurrentWrapper() {
		anyType = "dyn Any + Send + Sync"
	}
	boxType := "Box<" + anyType + ">"
	out.WriteString(`
pub trait GoComparable {
    fn go_eq(&self, other: &Self) -> bool;
    fn go_hash(&self, seed: usize) -> usize;
}

fn go_hash_value<T: std::hash::Hash>(value: &T, seed: usize) -> usize {
    let mut __hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&seed, &mut __hasher);
    std::hash::Hash::hash(value, &mut __hasher);
    std::hash::Hasher::finish(&__hasher) as usize
}

macro_rules! impl_go_comparable_hash {
    ($($t:ty),* $(,)?) => {
        $(impl GoComparable for $t {
            fn go_eq(&self, other: &Self) -> bool { self == other }
            fn go_hash(&self, seed: usize) -> usize { go_hash_value(self, seed) }
        })*
    };
}

impl_go_comparable_hash!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, String, &'static str);

impl GoComparable for f32 {
    fn go_eq(&self, other: &Self) -> bool { self == other }
    fn go_hash(&self, seed: usize) -> usize { go_hash_value(&self.to_bits(), seed) }
}

impl GoComparable for f64 {
    fn go_eq(&self, other: &Self) -> bool { self == other }
    fn go_hash(&self, seed: usize) -> usize { go_hash_value(&self.to_bits(), seed) }
}

fn go_any_comparable_eq(left: &(` + anyType + `), right: &(` + anyType + `)) -> bool {
    if left.type_id() != right.type_id() {
        return false;
    }
    if let Some(v) = left.downcast_ref::<i32>() { return right.downcast_ref::<i32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i64>() { return right.downcast_ref::<i64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i8>() { return right.downcast_ref::<i8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i16>() { return right.downcast_ref::<i16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u32>() { return right.downcast_ref::<u32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u64>() { return right.downcast_ref::<u64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u8>() { return right.downcast_ref::<u8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u16>() { return right.downcast_ref::<u16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<usize>() { return right.downcast_ref::<usize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<isize>() { return right.downcast_ref::<isize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f64>() { return right.downcast_ref::<f64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f32>() { return right.downcast_ref::<f32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<String>() { return right.downcast_ref::<String>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<&str>() { return right.downcast_ref::<&str>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<bool>() { return right.downcast_ref::<bool>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<char>() { return right.downcast_ref::<char>().map_or(false, |r| v == r); }
    panic!("interface comparison with uncomparable dynamic type")
}

fn go_any_comparable_hash(value: &(` + anyType + `), seed: usize) -> usize {
    if let Some(v) = value.downcast_ref::<i32>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i64>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i8>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i16>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u32>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u64>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u8>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u16>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<usize>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<isize>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<f64>() { return go_hash_value(&(value.type_id(), v.to_bits()), seed); }
    if let Some(v) = value.downcast_ref::<f32>() { return go_hash_value(&(value.type_id(), v.to_bits()), seed); }
    if let Some(v) = value.downcast_ref::<String>() { return go_hash_value(&(value.type_id(), v), seed); }
    if let Some(v) = value.downcast_ref::<&str>() { return go_hash_value(&(value.type_id(), v), seed); }
    if let Some(v) = value.downcast_ref::<bool>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<char>() { return go_hash_value(&(value.type_id(), *v), seed); }
    panic!("interface hash with uncomparable dynamic type")
}

impl GoComparable for ` + boxType + ` {
    fn go_eq(&self, other: &Self) -> bool { go_any_comparable_eq(self.as_ref(), other.as_ref()) }
    fn go_hash(&self, seed: usize) -> usize { go_any_comparable_hash(self.as_ref(), seed) }
}
`)
}

func generateEmbeddedOwnerRegistry(out *strings.Builder) {
	TrackImport("Any")
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		fmt.Fprintf(out, `
fn go_embedded_owner_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, Box<dyn Any + Send + Sync>>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, Box<dyn Any + Send + Sync>>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

%[1]sfn go_register_embedded_owner<T: Send + Sync + 'static>(embedded_key: usize, owner: Arc<Mutex<Option<T>>>) {
    go_embedded_owner_registry().lock().unwrap().insert(embedded_key, Box::new(owner));
}

%[1]sfn go_lookup_embedded_owner<T: Send + Sync + 'static>(embedded_key: usize, target: &str) -> Arc<Mutex<Option<T>>> {
    let registry = go_embedded_owner_registry().lock().unwrap();
    let owner = registry.get(&embedded_key).unwrap_or_else(|| panic!("embedded owner registry missing {}", target));
    owner
        .downcast_ref::<Arc<Mutex<Option<T>>>>()
        .unwrap_or_else(|| panic!("embedded owner registry type mismatch for {}", target))
        .clone()
}
`, visibility)
		return
	}
	TrackImport("Rc")
	TrackImport("RefCell")
	fmt.Fprintf(out, `
thread_local! {
    static GO_EMBEDDED_OWNER_REGISTRY: RefCell<std::collections::HashMap<usize, Box<dyn Any>>> = RefCell::new(std::collections::HashMap::new());
}

%[1]sfn go_register_embedded_owner<T: 'static>(embedded_key: usize, owner: Rc<RefCell<Option<T>>>) {
    GO_EMBEDDED_OWNER_REGISTRY.with(|registry| {
        registry.borrow_mut().insert(embedded_key, Box::new(owner));
    });
}

%[1]sfn go_lookup_embedded_owner<T: 'static>(embedded_key: usize, target: &str) -> Rc<RefCell<Option<T>>> {
    GO_EMBEDDED_OWNER_REGISTRY.with(|registry| {
        let registry = registry.borrow();
        let owner = registry.get(&embedded_key).unwrap_or_else(|| panic!("embedded owner registry missing {}", target));
        owner
            .downcast_ref::<Rc<RefCell<Option<T>>>>()
            .unwrap_or_else(|| panic!("embedded owner registry type mismatch for {}", target))
            .clone()
    })
}
`, visibility)
}

func generateGoByteSequence(out *strings.Builder) {
	// go_slice is s[start:end] of the sequence's own type: a substring of a
	// string, and of a []byte a slice that shares its backing array.
	byteSliceReslice := "self[start..end].to_vec()"
	if goSliceModeEnabled() {
		// s[i:j] may reach past len(s) up to cap(s): those bytes are
		// zero, as reslice3_zeroed stores them.
		byteSliceReslice = "self.reslice3_zeroed(start, end, self.capacity())"
	}
	// go_to_bytes is []byte(s): a string's bytes in a new slice, and a
	// []byte itself, sharing its backing array.
	stringToBytes := "self.as_bytes()" + sliceToVecMethod()
	out.WriteString(`
pub trait GoByteSequence: Clone {
    fn go_len(&self) -> usize;
    fn go_byte(&self, index: usize) -> u8;
    fn go_slice_to_string(&self, start: usize, end: Option<usize>) -> String;
    fn go_slice(&self, start: usize, end: Option<usize>) -> Self;
    fn go_to_bytes(&self) -> Vec<u8>;

    fn go_to_string(&self) -> String {
        self.go_slice_to_string(0, None)
    }
}

impl GoByteSequence for String {
    fn go_len(&self) -> usize {
        self.len()
    }

    fn go_byte(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }

    fn go_slice_to_string(&self, start: usize, end: Option<usize>) -> String {
        let end = end.unwrap_or_else(|| self.len());
        self[start..end].to_string()
    }

    fn go_slice(&self, start: usize, end: Option<usize>) -> Self {
        let end = end.unwrap_or_else(|| self.len());
        self[start..end].to_string()
    }

    fn go_to_bytes(&self) -> Vec<u8> {
        ` + stringToBytes + `
    }

    fn go_to_string(&self) -> String {
        self.clone()
    }
}

impl GoByteSequence for Vec<u8> {
    fn go_len(&self) -> usize {
        self.len()
    }

    fn go_byte(&self, index: usize) -> u8 {
        self[index]
    }

    fn go_slice_to_string(&self, start: usize, end: Option<usize>) -> String {
        let end = end.unwrap_or_else(|| self.len());
        String::from_utf8(self[start..end].to_vec()).unwrap()
    }

    fn go_slice(&self, start: usize, end: Option<usize>) -> Self {
        let end = end.unwrap_or_else(|| self.len());
        ` + byteSliceReslice + `
    }

    fn go_to_bytes(&self) -> Vec<u8> {
        self.clone()
    }

    fn go_to_string(&self) -> String {
        String::from_utf8(self.clone()).unwrap()
    }
}
`)
}

func generateGoInteger(out *strings.Builder) {
	out.WriteString(`
pub trait GoInteger: Copy + Clone + PartialOrd + 'static {
    fn go_from_i128(value: i128) -> Self;
    fn go_to_i128(self) -> i128;
}

macro_rules! impl_go_integer {
    ($($t:ty),* $(,)?) => {
        $(
            impl GoInteger for $t {
                fn go_from_i128(value: i128) -> Self {
                    value as $t
                }

                fn go_to_i128(self) -> i128 {
                    self as i128
                }
            }
        )*
    };
}

impl_go_integer!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

fn go_integer_from_i128<T: GoInteger>(value: i128) -> T {
    T::go_from_i128(value)
}

fn go_integer_cast<T: GoInteger, U: GoInteger>(value: U) -> T {
    T::go_from_i128(value.go_to_i128())
}

fn go_integer_add_one<T: GoInteger>(value: T) -> T {
    T::go_from_i128(value.go_to_i128() + 1)
}

fn go_integer_sub_one<T: GoInteger>(value: T) -> T {
    T::go_from_i128(value.go_to_i128() - 1)
}
`)
}

// generateAnySliceFormatter emits the printers for a []interface{}:
// format_any_slice for the slice as a value, and format_any_variadic for a
// slice spread into the arguments of a print call. The elements of the first
// are inside the slice and the elements of the second are arguments, which
// matters only when nestedElements says format_any tells the two apart.
func generateAnySliceFormatter(out *strings.Builder, nestedElements bool) {
	TrackImport("Any")
	sliceType := "&Rc<RefCell<Option<Vec<Box<dyn Any>>>>>"
	valueType := "&dyn Any"
	guard := "slice.borrow()"
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		sliceType = "&Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>"
		valueType = "&(dyn Any + Send + Sync)"
		guard = "slice.lock().unwrap()"
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
	}
	valuesParams, formatValue, nestedArg, topLevelArg := "", "format_any", "", ""
	if nestedElements {
		valuesParams = ", format_value: fn(" + valueType + ") -> String"
		formatValue = "format_value"
		nestedArg = ", format_any_nested"
		topLevelArg = ", format_any"
	}
	out.WriteString(`
fn format_any_slice_values(slice: ` + sliceType + valuesParams + `) -> String {
    let guard = ` + guard + `;
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| ` + formatValue + `(v.as_ref())).collect();
        formatted.join(" ")
    } else {
        String::new()
    }
}

fn format_any_slice(slice: ` + sliceType + `) -> String {
    format!("[{}]", format_any_slice_values(slice` + nestedArg + `))
}

fn format_any_variadic(slice: ` + sliceType + `) -> String {
    format_any_slice_values(slice` + topLevelArg + `)
}
`)
}

// Keep ownership of a pending send across polls, including non-Clone interface
// values. A full channel gives the original Rust value back to the caller.
const goSelectSendHelper = `
impl<T> GoChannel<T> {
    pub fn try_send_select(&self, pending: &mut Option<T>) -> bool {
        if self.is_nil() {
            return false;
        }
        let sender = self.tx.lock().unwrap().clone().expect("send on closed channel");
        match sender.try_send(pending.take().unwrap()) {
            Ok(()) => {
                if self.capacity > 0 {
                    self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                true
            }
            Err(std::sync::mpsc::TrySendError::Full(value)) => {
                *pending = Some(value);
                false
            }
            Err(std::sync::mpsc::TrySendError::Disconnected(_)) => panic!("send on closed channel"),
        }
    }
}
`

func generateGoChannelHelper(out *strings.Builder, needsGoTypeParamFieldClone bool, needsIdentity bool) {
	code := `
struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
    is_nil: std::sync::Arc<std::sync::atomic::AtomicBool>,
    len: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    capacity: usize,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: cap,
        }
    }

    fn send(&self, val: T) {
        if self.is_nil() {
            return;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.try_send(val).is_ok() {
                if self.capacity > 0 {
                    self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn try_recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().try_recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    // A select case's receive: Some(Some(v)) for a value, Some(None) when
    // the channel is closed and drained, None when nothing is ready yet.
    fn try_recv_select(&self) -> Option<Option<T>> {
        if self.is_nil() {
            return None;
        }
        match self.rx.lock().unwrap().try_recv() {
            Ok(value) => {
                if self.capacity > 0 {
                    let _ = self.len.fetch_update(
                        std::sync::atomic::Ordering::SeqCst,
                        std::sync::atomic::Ordering::SeqCst,
                        |__go_current| __go_current.checked_sub(1),
                    );
                }
                Some(Some(value))
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(None),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
        }
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }

    fn is_nil(&self) -> bool {
        self.is_nil.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
            is_nil: self.is_nil.clone(),
            len: self.len.clone(),
            capacity: self.capacity,
        }
    }
}

impl<T> Default for GoChannel<T> {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }
}

impl<T> std::fmt::Debug for GoChannel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoChannel")
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
    }
}
`
	if needsIdentity {
		code += `
impl<T> GoChannel<T> {
    /// The address an unsafe.Pointer to this channel holds in Go: 0 for a
    /// nil channel, otherwise one address that every handle shares.
    fn go_identity(&self) -> usize {
        if self.is_nil() {
            0
        } else {
            std::sync::Arc::as_ptr(&self.rx) as *const () as usize
        }
    }
}
`
	}
	if generatingPublicHelpers {
		code = strings.ReplaceAll(code, "    fn go_identity(", "    pub fn go_identity(")
		code = strings.ReplaceAll(code, "struct GoChannel<T>", "pub struct GoChannel<T>")
		code = strings.ReplaceAll(code, "    fn new(", "    pub fn new(")
		code = strings.ReplaceAll(code, "    fn new_buffered(", "    pub fn new_buffered(")
		code = strings.ReplaceAll(code, "    fn send(", "    pub fn send(")
		code = strings.ReplaceAll(code, "    fn try_send(", "    pub fn try_send(")
		code = strings.ReplaceAll(code, "    fn recv(", "    pub fn recv(")
		code = strings.ReplaceAll(code, "    fn try_recv(", "    pub fn try_recv(")
		code = strings.ReplaceAll(code, "    fn try_recv_select(", "    pub fn try_recv_select(")
		code = strings.ReplaceAll(code, "    fn close(", "    pub fn close(")
		code = strings.ReplaceAll(code, "    fn is_nil(", "    pub fn is_nil(")
		code = strings.ReplaceAll(code, "    fn len(", "    pub fn len(")
		code = strings.ReplaceAll(code, "    fn capacity(", "    pub fn capacity(")
	}
	out.WriteString(code)
	if needsGoTypeParamFieldClone {
		out.WriteString(`

impl<T> GoValueClone for GoChannel<T> {
    fn go_value_clone(&self) -> Self { self.clone() }
}

impl<T> GoTypeParamFieldClone for GoChannel<T> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> { field.clone() }
}
`)
	}
}

func generateWaitGroupHelper(out *strings.Builder) {
	out.WriteString(`
struct WaitGroup {
    count: std::sync::Arc<(std::sync::Mutex<i64>, std::sync::Condvar)>,
}

impl WaitGroup {
    fn new() -> Self {
        WaitGroup {
            count: std::sync::Arc::new((std::sync::Mutex::new(0), std::sync::Condvar::new())),
        }
    }

    fn add(&self, n: i64) {
        let (lock, _) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count += n;
    }

    fn done(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count -= 1;
        if *count <= 0 {
            cvar.notify_all();
        }
    }

    fn wait(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        while *count > 0 {
            count = cvar.wait(count).unwrap();
        }
    }
}

impl Clone for WaitGroup {
    fn clone(&self) -> Self {
        WaitGroup {
            count: self.count.clone(),
        }
    }
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WaitGroup")
    }
}
`)
}

func generateGoMutexHelper(out *strings.Builder) {
	out.WriteString(`
struct GoMutex {
    inner: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
    is_nil: bool,
}

struct GoMutexGuard {
    mutex: GoMutex,
    active: bool,
}

impl GoMutex {
    fn new() -> Self {
        GoMutex {
            inner: std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new())),
            is_nil: false,
        }
    }

    fn nil() -> Self {
        GoMutex {
            inner: std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new())),
            is_nil: true,
        }
    }

    fn go_ptr_eq(left: &Self, right: &Self) -> bool {
        if left.is_nil || right.is_nil {
            return left.is_nil && right.is_nil;
        }
        std::sync::Arc::ptr_eq(&left.inner, &right.inner)
    }

    fn lock(&self) {
        if self.is_nil { panic!("nil *sync.Mutex"); }
        let (state, ready) = &*self.inner;
        let mut locked = state.lock().unwrap();
        while *locked {
            locked = ready.wait(locked).unwrap();
        }
        *locked = true;
    }

    fn unlock(&self) {
        if self.is_nil { panic!("nil *sync.Mutex"); }
        let (state, ready) = &*self.inner;
        let mut locked = state.lock().unwrap();
        if !*locked {
            panic!("sync.Mutex: unlock of unlocked mutex");
        }
        *locked = false;
        ready.notify_one();
    }

    fn guard(&self) -> GoMutexGuard {
        self.lock();
        GoMutexGuard {
            mutex: self.clone(),
            active: true,
        }
    }
}

impl Drop for GoMutexGuard {
    fn drop(&mut self) {
        if self.active {
            self.mutex.unlock();
            self.active = false;
        }
    }
}

impl Default for GoMutex {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GoMutex {
    fn clone(&self) -> Self {
        GoMutex {
            inner: self.inner.clone(),
            is_nil: self.is_nil,
        }
    }
}

impl std::fmt::Debug for GoMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mutex")
    }
}
`)
}

func generateGoRWMutexHelper(out *strings.Builder) {
	code := `
// A readers-writer lock: any number of readers, or one writer. A clone is
// the same lock, as a *sync.RWMutex is.
#[derive(Clone, Debug, Default)]
pub struct GoRWMutex {
    // (a writer holds it, how many readers hold it), and the waiters.
    inner: std::sync::Arc<(std::sync::Mutex<(bool, usize)>, std::sync::Condvar)>,
}

impl GoRWMutex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        while held.0 || held.1 > 0 {
            held = ready.wait(held).unwrap();
        }
        held.0 = true;
    }

    pub fn unlock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        if !held.0 {
            panic!("sync: Unlock of unlocked RWMutex");
        }
        held.0 = false;
        ready.notify_all();
    }

    pub fn r_lock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        while held.0 {
            held = ready.wait(held).unwrap();
        }
        held.1 += 1;
    }

    pub fn r_unlock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        if held.1 == 0 {
            panic!("sync: RUnlock of unlocked RWMutex");
        }
        held.1 -= 1;
        if held.1 == 0 {
            ready.notify_all();
        }
    }
}

impl std::fmt::Display for GoRWMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RWMutex")
    }
}
`
	out.WriteString(code)
}

func generateGoOnceHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		out.WriteString(`
#[derive(Clone, Debug)]
struct GoOnce {
    done: std::sync::Arc<std::sync::atomic::AtomicBool>,
    m: std::sync::Arc<std::sync::Mutex<()>>,
}

impl GoOnce {
    fn new() -> Self {
        GoOnce {
            done: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            m: std::sync::Arc::new(std::sync::Mutex::new(())),
        }
    }

    // Go's Once: the first caller runs f while it holds m, so no call
    // returns before f has finished, and f counts as done even if it
    // panics.
    fn r#do<F>(&self, mut f: F)
    where
        F: FnMut(),
    {
        if self.done.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        let _running = self.m.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if !self.done.load(std::sync::atomic::Ordering::Acquire) {
            struct MarkDone<'a>(&'a std::sync::atomic::AtomicBool);
            impl Drop for MarkDone<'_> {
                fn drop(&mut self) {
                    self.0.store(true, std::sync::atomic::Ordering::Release);
                }
            }
            let _mark = MarkDone(&self.done);
            f();
        }
    }
}

impl Default for GoOnce {
    fn default() -> Self {
        Self::new()
    }
}
`)
		return
	}
	out.WriteString(`
#[derive(Clone, Debug)]
struct GoOnce {
    done: std::rc::Rc<std::cell::RefCell<bool>>,
}

impl GoOnce {
    fn new() -> Self {
        GoOnce {
            done: std::rc::Rc::new(std::cell::RefCell::new(false)),
        }
    }

    fn r#do<F>(&self, mut f: F)
    where
        F: FnMut(),
    {
        let mut done = self.done.borrow_mut();
        if !*done {
            *done = true;
            std::mem::drop(done);
            f();
        }
    }
}

impl Default for GoOnce {
    fn default() -> Self {
        Self::new()
    }
}
	`)
}

func generateGoAtomicPointerHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
struct GoAtomicPointer<T: Send + Sync + 'static> {
    value: Arc<Mutex<Option<GoPtr<T>>>>,
}

impl<T: Send + Sync + 'static> GoAtomicPointer<T> {
    fn load(&self) -> GoPtr<T> {
        self.value.lock().unwrap().as_ref().cloned().unwrap_or_else(|| GoPtr::nil())
    }

    fn store(&self, value: GoPtr<T>) {
        *self.value.lock().unwrap() = if value.is_nil() { None } else { Some(value) };
    }

    fn swap(&self, value: GoPtr<T>) -> GoPtr<T> {
        let mut current = self.value.lock().unwrap();
        let old = current.as_ref().cloned().unwrap_or_else(|| GoPtr::nil());
        *current = if value.is_nil() { None } else { Some(value) };
        old
    }

    fn compare_and_swap(&self, old: GoPtr<T>, new: GoPtr<T>) -> bool {
        let mut current = self.value.lock().unwrap();
        let matched = match current.as_ref() {
            Some(value) if old.is_nil() => value.is_nil(),
            Some(value) => GoPtr::ptr_eq(value, &old),
            None => old.is_nil(),
        };
        if matched {
            *current = if new.is_nil() { None } else { Some(new) };
        }
        matched
    }
}

impl<T: Send + Sync + 'static> Clone for GoAtomicPointer<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

impl<T: Send + Sync + 'static> Default for GoAtomicPointer<T> {
    fn default() -> Self {
        Self { value: Arc::new(Mutex::new(None)) }
    }
}

impl<T: Send + Sync + 'static> std::fmt::Debug for GoAtomicPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoAtomicPointer")
    }
}

impl<T: Send + Sync + 'static> std::fmt::Display for GoAtomicPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoAtomicPointer")
    }
}
`)
		return
	}

	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString(`
struct GoAtomicPointer<T: 'static> {
    value: Rc<RefCell<Option<GoPtr<T>>>>,
}

impl<T: 'static> GoAtomicPointer<T> {
    fn load(&self) -> GoPtr<T> {
        self.value.borrow().as_ref().cloned().unwrap_or_else(|| GoPtr::nil())
    }

    fn store(&self, value: GoPtr<T>) {
        *self.value.borrow_mut() = if value.is_nil() { None } else { Some(value) };
    }

    fn swap(&self, value: GoPtr<T>) -> GoPtr<T> {
        let mut current = self.value.borrow_mut();
        let old = current.as_ref().cloned().unwrap_or_else(|| GoPtr::nil());
        *current = if value.is_nil() { None } else { Some(value) };
        old
    }

    fn compare_and_swap(&self, old: GoPtr<T>, new: GoPtr<T>) -> bool {
        let mut current = self.value.borrow_mut();
        let matched = match current.as_ref() {
            Some(value) if old.is_nil() => value.is_nil(),
            Some(value) => GoPtr::ptr_eq(value, &old),
            None => old.is_nil(),
        };
        if matched {
            *current = if new.is_nil() { None } else { Some(new) };
        }
        matched
    }
}

impl<T: 'static> Clone for GoAtomicPointer<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

impl<T: 'static> Default for GoAtomicPointer<T> {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(None)) }
    }
}

impl<T: 'static> std::fmt::Debug for GoAtomicPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoAtomicPointer")
    }
}

impl<T: 'static> std::fmt::Display for GoAtomicPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoAtomicPointer")
    }
}
`)
}

// trackImportsForAnyPayloadType makes sure a helper that names rustType, the
// payload of a boxed interface value, also imports every std type that
// spelling uses. The file that boxed the value imported them for itself, but
// the helper may be emitted into a different file, which sees none of that
// file's imports.
func trackImportsForAnyPayloadType(rustType string) {
	for i := 0; i < len(rustType); {
		if !isRustIdentStart(rustType[i]) {
			i++
			continue
		}
		start := i
		for i < len(rustType) && isRustIdentPart(rustType[i]) {
			i++
		}
		// A path segment (std::sync::Mutex) is already fully spelled.
		if start >= 2 && rustType[start-2:start] == "::" {
			continue
		}
		switch name := rustType[start:i]; name {
		case "Rc", "RefCell", "Arc", "Mutex", "BTreeMap", "Any":
			TrackImport(name)
		case "StdError":
			TrackImport("Error")
		}
	}
}

func isRustIdentStart(c byte) bool {
	return c == '_' || (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
}

func isRustIdentPart(c byte) bool {
	return isRustIdentStart(c) || (c >= '0' && c <= '9')
}

func generateGoTypeNameHelper(out *strings.Builder, anyTypeNames map[string]string, runtimeOps bool) {
	TrackImport("Any")
	valueType := "&dyn Any"
	if runtimeOps && NeedsConcurrentWrapper() {
		valueType = "&(dyn Any + Send + Sync)"
	}
	out.WriteString(`
fn `)
	out.WriteString(goTypeNameHelperRustName)
	out.WriteString("(val: " + valueType + `) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<char>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
`)
	rustTypes := make([]string, 0, len(anyTypeNames))
	for rustType := range anyTypeNames {
		rustTypes = append(rustTypes, rustType)
	}
	slices.Sort(rustTypes)
	for _, rustType := range rustTypes {
		trackImportsForAnyPayloadType(rustType)
		out.WriteString("    if val.is::<")
		out.WriteString(rustType)
		out.WriteString(">() { return ")
		out.WriteString(strconv.Quote(anyTypeNames[rustType]))
		out.WriteString(" }\n")
	}
	if runtimeOps {
		out.WriteString("    if let Some(__go_name) = go_registered_any_type_name(val) { return __go_name }\n")
	}
	out.WriteString("    std::any::type_name_of_val(val)\n}\n")
}

func generateBase64Helper(out *strings.Builder) {
	out.WriteString(`
fn go_base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] } else { 0 };

        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if i + 1 < data.len() {
            out.push(TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(TABLE[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }

        i += 3;
    }
    out
}

fn go_base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn go_base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let bytes = input.as_bytes();
    if bytes.len() % 4 != 0 {
        return Err(format!("illegal base64 data at input byte {}", bytes.len()));
    }

    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c0 = go_base64_value(bytes[i])
            .ok_or_else(|| format!("illegal base64 data at input byte {}", i))?;
        let c1 = go_base64_value(bytes[i + 1])
            .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 1))?;
        let pad2 = bytes[i + 2] == b'=';
        let pad3 = bytes[i + 3] == b'=';
        let c2 = if pad2 {
            0
        } else {
            go_base64_value(bytes[i + 2])
                .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 2))?
        };
        let c3 = if pad3 {
            0
        } else {
            go_base64_value(bytes[i + 3])
                .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 3))?
        };

        out.push((c0 << 2) | (c1 >> 4));
        if !pad2 {
            out.push((c1 << 4) | (c2 >> 2));
        }
        if !pad3 {
            out.push((c2 << 6) | c3);
        }

        i += 4;
    }
    Ok(out)
}
`)
}

func generateSha256Helper(out *strings.Builder) {
	out.WriteString(`
fn go_sha256_sum256(data: &[u8]) -> Vec<u8> {
    const H0: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    let mut h = H0;
    let mut msg = data.to_vec();
    let bit_len = (msg.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            let j = i * 4;
            w[i] = u32::from_be_bytes([chunk[j], chunk[j + 1], chunk[j + 2], chunk[j + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut out = Vec::with_capacity(32);
    for word in h {
        out.extend_from_slice(&word.to_be_bytes());
    }
    out
}
`)
}

func generateHexFormatHelper(out *strings.Builder) {
	out.WriteString(`
fn go_format_hex_bytes(data: &[u8], upper: bool) -> String {
    let table = if upper {
        b"0123456789ABCDEF"
    } else {
        b"0123456789abcdef"
    };
    let mut out = String::with_capacity(data.len() * 2);
    for b in data {
        out.push(table[(b >> 4) as usize] as char);
        out.push(table[(b & 0x0f) as usize] as char);
    }
    out
}
`)
}

func generateUrlHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoUrl {
    scheme: Arc<Mutex<Option<String>>>,
    host: Arc<Mutex<Option<String>>>,
    path: Arc<Mutex<Option<String>>>,
    raw_query: Arc<Mutex<Option<String>>>,
}

fn go_url_parse(input: &str) -> GoUrl {
    let (scheme, rest) = input.split_once("://").unwrap_or(("", input));
    let (before_query, raw_query) = rest.split_once('?').unwrap_or((rest, ""));
    let slash = before_query.find('/').unwrap_or(before_query.len());
    let host = &before_query[..slash];
    let path = if slash < before_query.len() { &before_query[slash..] } else { "" };
    GoUrl {
        scheme: Arc::new(Mutex::new(Some(scheme.to_string()))),
        host: Arc::new(Mutex::new(Some(host.to_string()))),
        path: Arc::new(Mutex::new(Some(path.to_string()))),
        raw_query: Arc::new(Mutex::new(Some(raw_query.to_string()))),
    }
}
`)
		return
	}

	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoUrl {
    scheme: Rc<RefCell<Option<String>>>,
    host: Rc<RefCell<Option<String>>>,
    path: Rc<RefCell<Option<String>>>,
    raw_query: Rc<RefCell<Option<String>>>,
}

fn go_url_parse(input: &str) -> GoUrl {
    let (scheme, rest) = input.split_once("://").unwrap_or(("", input));
    let (before_query, raw_query) = rest.split_once('?').unwrap_or((rest, ""));
    let slash = before_query.find('/').unwrap_or(before_query.len());
    let host = &before_query[..slash];
    let path = if slash < before_query.len() { &before_query[slash..] } else { "" };
    GoUrl {
        scheme: Rc::new(RefCell::new(Some(scheme.to_string()))),
        host: Rc::new(RefCell::new(Some(host.to_string()))),
        path: Rc::new(RefCell::new(Some(path.to_string()))),
        raw_query: Rc::new(RefCell::new(Some(raw_query.to_string()))),
    }
}
`)
}

func generateRegexpHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoRegexp {
    pattern: Arc<Mutex<Option<String>>>,
}

impl GoRegexp {
    fn find_all_string(&self, text: Arc<Mutex<Option<String>>>, n: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Vec<String>>>> {
        let pattern = (*self.pattern.lock().unwrap().as_ref().unwrap()).clone();
        let text = (*text.lock().unwrap().as_ref().unwrap()).clone();
        let limit = *n.lock().unwrap().as_ref().unwrap();
        Arc::new(Mutex::new(Some(go_regexp_find_all_string(&pattern, &text, limit))))
    }

    fn match_string(&self, text: Arc<Mutex<Option<String>>>) -> bool {
        let pattern = (*self.pattern.lock().unwrap().as_ref().unwrap()).clone();
        let text = (*text.lock().unwrap().as_ref().unwrap()).clone();
        go_regexp_match_string(&pattern, &text)
    }

    fn find_string_submatch(&self, text: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> {
        let pattern = (*self.pattern.lock().unwrap().as_ref().unwrap()).clone();
        let text = (*text.lock().unwrap().as_ref().unwrap()).clone();
        Arc::new(Mutex::new(Some(go_regexp_find_string_submatch(&pattern, &text))))
    }

    fn replace_all_string(&self, src: Arc<Mutex<Option<String>>>, repl: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        let pattern = (*self.pattern.lock().unwrap().as_ref().unwrap()).clone();
        let src = (*src.lock().unwrap().as_ref().unwrap()).clone();
        let repl = (*repl.lock().unwrap().as_ref().unwrap()).clone();
        Arc::new(Mutex::new(Some(go_regexp_replace_all_string(&pattern, &src, &repl))))
    }
}

fn go_regexp_find_all_string(pattern: &str, text: &str, limit: i64) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }

    if pattern == r"\d+" {
        let mut matches = Vec::new();
        let mut current = String::new();
        for ch in text.chars() {
            if ch.is_ascii_digit() {
                current.push(ch);
            } else if !current.is_empty() {
                matches.push(std::mem::take(&mut current));
                if limit > 0 && matches.len() >= limit as usize {
                    return matches;
                }
            }
        }
        if !current.is_empty() {
            matches.push(current);
        }
        if limit > 0 {
            matches.truncate(limit as usize);
        }
        return matches;
    }

    if pattern.is_empty() {
        return Vec::new();
    }

    let mut matches = Vec::new();
    let mut rest = text;
    while let Some(index) = rest.find(pattern) {
        matches.push(pattern.to_string());
        if limit > 0 && matches.len() >= limit as usize {
            break;
        }
        rest = &rest[index + pattern.len()..];
    }
    matches
}

fn go_regexp_match_string(pattern: &str, text: &str) -> bool {
    !go_regexp_find_string_submatch(pattern, text).is_empty()
}

fn go_regexp_find_string_submatch(pattern: &str, text: &str) -> Vec<String> {
    if pattern == r"-mod[ =](\w+)" {
        for marker in ["-mod=", "-mod "] {
            if let Some(start) = text.find(marker) {
                let value_start = start + marker.len();
                let value: String = text[value_start..].chars().take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_').collect();
                if !value.is_empty() {
                    return vec![format!("{}{}", marker, value), value];
                }
            }
        }
        return Vec::new();
    }

    if pattern == r"^go version (go\S+|devel \S+)" {
        let prefix = "go version ";
        if let Some(rest) = text.strip_prefix(prefix) {
            if let Some(first) = rest.split_whitespace().next() {
                if first.starts_with("go") {
                    return vec![format!("{}{}", prefix, first), first.to_string()];
                }
                if first == "devel" {
                    if let Some(second) = rest.split_whitespace().nth(1) {
                        let capture = format!("devel {}", second);
                        return vec![format!("{}{}", prefix, capture), capture];
                    }
                }
            }
        }
        return Vec::new();
    }

    if pattern == r"go:.*go.mod.*contents have changed" {
        if let Some(go_index) = text.find("go:") {
            if let Some(mod_index) = text[go_index..].find("go.mod") {
                let after_mod = go_index + mod_index;
                if text[after_mod..].contains("contents have changed") {
                    return vec![text.to_string()];
                }
            }
        }
        return Vec::new();
    }

    let matches = go_regexp_find_all_string(pattern, text, 1);
    if matches.is_empty() {
        Vec::new()
    } else {
        vec![matches[0].clone()]
    }
}

fn go_regexp_replace_all_string(pattern: &str, text: &str, repl: &str) -> String {
    if pattern == r"[$,]" {
        let mut out = String::new();
        for ch in text.chars() {
            if ch == '$' || ch == ',' {
                out.push_str(repl);
            } else {
                out.push(ch);
            }
        }
        return out;
    }
    if pattern == r"[_]" {
        return text.replace('_', repl);
    }
    if pattern == r"[USD\s]" {
        let mut out = String::new();
        for ch in text.chars() {
            if ch == 'U' || ch == 'S' || ch == 'D' || ch.is_whitespace() {
                out.push_str(repl);
            } else {
                out.push(ch);
            }
        }
        return out;
    }
    text.replace(pattern, repl)
}
`)
		return
	}

	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoRegexp {
    pattern: Rc<RefCell<Option<String>>>,
}

impl GoRegexp {
    fn find_all_string(&self, text: Rc<RefCell<Option<String>>>, n: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Vec<String>>>> {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let text = (*text.borrow().as_ref().unwrap()).clone();
        let limit = *n.borrow().as_ref().unwrap();
        Rc::new(RefCell::new(Some(go_regexp_find_all_string(&pattern, &text, limit))))
    }

    fn match_string(&self, text: Rc<RefCell<Option<String>>>) -> bool {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let text = (*text.borrow().as_ref().unwrap()).clone();
        go_regexp_match_string(&pattern, &text)
    }

    fn find_string_submatch(&self, text: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Vec<String>>>> {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let text = (*text.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_regexp_find_string_submatch(&pattern, &text))))
    }

    fn replace_all_string(&self, src: Rc<RefCell<Option<String>>>, repl: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        let pattern = (*self.pattern.borrow().as_ref().unwrap()).clone();
        let src = (*src.borrow().as_ref().unwrap()).clone();
        let repl = (*repl.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_regexp_replace_all_string(&pattern, &src, &repl))))
    }
}

fn go_regexp_find_all_string(pattern: &str, text: &str, limit: i64) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }

    if pattern == r"\d+" {
        let mut matches = Vec::new();
        let mut current = String::new();
        for ch in text.chars() {
            if ch.is_ascii_digit() {
                current.push(ch);
            } else if !current.is_empty() {
                matches.push(std::mem::take(&mut current));
                if limit > 0 && matches.len() >= limit as usize {
                    return matches;
                }
            }
        }
        if !current.is_empty() {
            matches.push(current);
        }
        if limit > 0 {
            matches.truncate(limit as usize);
        }
        return matches;
    }

    if pattern.is_empty() {
        return Vec::new();
    }

    let mut matches = Vec::new();
    let mut rest = text;
    while let Some(index) = rest.find(pattern) {
        matches.push(pattern.to_string());
        if limit > 0 && matches.len() >= limit as usize {
            break;
        }
        rest = &rest[index + pattern.len()..];
    }
    matches
}

fn go_regexp_match_string(pattern: &str, text: &str) -> bool {
    !go_regexp_find_string_submatch(pattern, text).is_empty()
}

fn go_regexp_find_string_submatch(pattern: &str, text: &str) -> Vec<String> {
    if pattern == r"-mod[ =](\w+)" {
        for marker in ["-mod=", "-mod "] {
            if let Some(start) = text.find(marker) {
                let value_start = start + marker.len();
                let value: String = text[value_start..].chars().take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_').collect();
                if !value.is_empty() {
                    return vec![format!("{}{}", marker, value), value];
                }
            }
        }
        return Vec::new();
    }

    if pattern == r"^go version (go\S+|devel \S+)" {
        let prefix = "go version ";
        if let Some(rest) = text.strip_prefix(prefix) {
            if let Some(first) = rest.split_whitespace().next() {
                if first.starts_with("go") {
                    return vec![format!("{}{}", prefix, first), first.to_string()];
                }
                if first == "devel" {
                    if let Some(second) = rest.split_whitespace().nth(1) {
                        let capture = format!("devel {}", second);
                        return vec![format!("{}{}", prefix, capture), capture];
                    }
                }
            }
        }
        return Vec::new();
    }

    if pattern == r"go:.*go.mod.*contents have changed" {
        if let Some(go_index) = text.find("go:") {
            if let Some(mod_index) = text[go_index..].find("go.mod") {
                let after_mod = go_index + mod_index;
                if text[after_mod..].contains("contents have changed") {
                    return vec![text.to_string()];
                }
            }
        }
        return Vec::new();
    }

    let matches = go_regexp_find_all_string(pattern, text, 1);
    if matches.is_empty() {
        Vec::new()
    } else {
        vec![matches[0].clone()]
    }
}

fn go_regexp_replace_all_string(pattern: &str, text: &str, repl: &str) -> String {
    if pattern == r"[$,]" {
        let mut out = String::new();
        for ch in text.chars() {
            if ch == '$' || ch == ',' {
                out.push_str(repl);
            } else {
                out.push(ch);
            }
        }
        return out;
    }
    if pattern == r"[_]" {
        return text.replace('_', repl);
    }
    if pattern == r"[USD\s]" {
        let mut out = String::new();
        for ch in text.chars() {
            if ch == 'U' || ch == 'S' || ch == 'D' || ch.is_whitespace() {
                out.push_str(repl);
            } else {
                out.push(ch);
            }
        }
        return out;
    }
    text.replace(pattern, repl)
}
`)
}

func generateJsonEscapeHelper(out *strings.Builder) {
	out.WriteString(`
fn go_json_escape(input: &str) -> String {
    let mut escaped = String::new();
    for ch in input.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c < ' ' => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            c => escaped.push(c),
        }
    }
    escaped
}
`)
}

func generateStrconvFormatHelper(out *strings.Builder) {
	out.WriteString(`
fn go_strconv_format_int(value: i64, base: i32) -> String {
    if base == 10 {
        return value.to_string();
    }
    if !(2..=36).contains(&base) {
        return value.to_string();
    }

    let negative = value < 0;
    let mut n = if negative {
        value.wrapping_neg() as u64
    } else {
        value as u64
    };
    let base = base as u64;
    let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut out = Vec::new();
    if n == 0 {
        out.push(b'0');
    }
    while n > 0 {
        out.push(digits[(n % base) as usize]);
        n /= base;
    }
    if negative {
        out.push(b'-');
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

fn go_strconv_format_float(value: f64, fmt: char, precision: i32) -> String {
    let precision = if precision < 0 { 6 } else { precision as usize };
    match fmt {
        'e' => format!("{:.*e}", precision, value),
        'E' => format!("{:.*E}", precision, value),
        'f' => format!("{:.*}", precision, value),
        'g' | 'G' => {
            if precision == 0 {
                format!("{:.0}", value)
            } else {
                format!("{:.*}", precision, value)
            }
        }
        _ => value.to_string(),
    }
}
`)
}

// writeSliceElemPtrConstructor writes GoSliceElemPtr::new, which checks the
// index as &s[i] does, and identity, which pointer comparison uses.
//
// With GoSlice the pointer holds a copy of the slice's header in a cell of
// its own: the copy shares the backing array, so writes through the pointer
// reach every slice over that array, and a later assignment to the slice
// variable (items = items[1:], or an append that grows it) leaves the
// pointer on the element it was taken from, as in Go (DEV-149). Two
// pointers are equal when they name the same element of the same array.
// Without GoSlice each slice value is its own Vec, so the pointer holds the
// variable's cell.
func writeSliceElemPtrConstructor(out *strings.Builder, outer string, cell string, tryBorrow string) {
	out.WriteString("    pub fn new(slice: " + outer + "<" + cell + "<Option<Vec<T>>>>, index: usize) -> Self {\n")
	out.WriteString(`        // &s[i] panics, as indexing does, when i is not below len(s). A
        // slice already borrowed for the expression is not checked.
`)
	out.WriteString("        if let Ok(__guard) = slice." + tryBorrow + "() {\n")
	out.WriteString(`            let __len = __guard.as_ref().map_or(0, |__values| __values.len());
            if index >= __len {
                drop(__guard);
                panic!("index out of bounds: the len is {} but the index is {}", __len, index);
            }
`)
	if goSliceModeEnabled() {
		out.WriteString(`            // Hold the backing array the slice has now, not the variable.
            let __header = (*__guard).clone();
            drop(__guard);
            // The element's address, taken now: nothing else can lock
            // the copy, and == need not wait for it.
            let __id = __header.as_ref().map(|__slice| ((__slice.as_slice().as_ptr() as usize).wrapping_add(index.wrapping_mul(std::mem::size_of::<T>())), 0));
`)
		// Spelled with std paths: a package can declare a Mutex of its own.
		cellPath := map[string]string{"Mutex": "std::sync::Mutex", "RefCell": "std::cell::RefCell"}[cell]
		outerPath := map[string]string{"Arc": "std::sync::Arc", "Rc": "std::rc::Rc"}[outer]
		out.WriteString("            return GoSliceElemPtr { slice: " + outerPath + "::new(" + cellPath + "::new(__header)), index, id: __id };\n")
	}
	out.WriteString(`        }
        GoSliceElemPtr { slice, index, id: None }
    }

    /// The array and element the pointer names, for == between pointers.
    pub fn identity(&self) -> (usize, usize) {
        if let Some(__id) = self.id {
            return __id;
        }
`)
	if goSliceModeEnabled() {
		out.WriteString("        if let Ok(__guard) = self.slice." + tryBorrow + "() {\n")
		// The element's address in the backing array, which every header
		// over that array shares.
		out.WriteString(`            if let Some(__slice) = __guard.as_ref() {
                let __base = __slice.as_slice().as_ptr() as usize;
                return (__base.wrapping_add(self.index.wrapping_mul(std::mem::size_of::<T>())), 0);
            }
        }
`)
	}
	out.WriteString("        (" + outer + "::as_ptr(&self.slice) as *const () as usize, self.index)\n    }\n\n")
}

func generateSliceElemPtrHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Clone)]
pub struct GoSliceElemPtr<T> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
    /// The element's identity when the pointer was made with the slice
    /// free to borrow; otherwise identity reads it from the slice.
    id: Option<(usize, usize)>,
}

pub struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoSliceElemMutRef<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

pub trait GoArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize>: Send + Sync {
    fn borrow_at(&self, index: usize) -> Option<T>;
    fn assign_at(&self, index: usize, value: Option<T>);
    fn identity_at(&self, index: usize) -> (*const (), usize);
}

#[derive(Clone)]
pub struct GoDirectArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize> {
    array: Arc<Mutex<Option<[T; N]>>>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemBacking<T, N> for GoDirectArrayElemBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.array.lock().unwrap();
        guard.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.array.lock().unwrap().as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Arc::as_ptr(&self.array) as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoNestedArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> {
    outer: Arc<Mutex<Option<[[T; N]; OUT]>>>,
    outer_index: usize,
}

impl<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoNestedArrayElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.outer.lock().unwrap();
        guard.as_ref().and_then(|values| values.get(self.outer_index)).and_then(|inner| inner.get(index)).cloned()
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.outer.lock().unwrap().as_mut() {
                values[self.outer_index][index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Arc::as_ptr(&self.outer) as *const (), self.outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromElemBacking<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> {
    parent: GoArrayElemPtr<[T; N], OUT>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoArrayElemFromElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            let mut inner = self.parent.borrow_mut();
            if let Some(values) = inner.as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        let (base, outer_index) = self.parent.identity();
        (base, outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromGoPtrBacking<T: Clone + Send + Sync + 'static, const N: usize> {
    parent: GoPtr<[T; N]>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemBacking<T, N> for GoArrayElemFromGoPtrBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            self.parent.with_mut(|values| {
                values[index] = value;
            });
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (self.parent.addr() as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoArrayElemPtr<T: Clone + Send + Sync + 'static, const N: usize> {
    backing: Arc<dyn GoArrayElemBacking<T, N> + Send + Sync>,
    index: usize,
}

pub struct GoArrayElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoArrayElemMutRef<T: Clone + Send + Sync + 'static, const N: usize> {
    backing: Arc<dyn GoArrayElemBacking<T, N> + Send + Sync>,
    index: usize,
    value: Option<T>,
}

impl<T> GoSliceElemPtr<T> {
`)
		writeSliceElemPtrConstructor(out, "Arc", "Mutex", "try_lock")
		out.WriteString(`    pub fn slice_handle(&self) -> Arc<Mutex<Option<Vec<T>>>> {
        self.slice.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<T: Clone> GoSliceElemPtr<T> {
    pub fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    pub fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemPtr<T, N> {
    pub fn new(array: Arc<Mutex<Option<[T; N]>>>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoDirectArrayElemBacking { array }),
            index,
        }
    }

    pub fn nested<const OUT: usize>(outer: Arc<Mutex<Option<[[T; N]; OUT]>>>, outer_index: usize, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoNestedArrayElemBacking { outer, outer_index }),
            index,
        }
    }

    pub fn from_array_elem<const OUT: usize>(parent: GoArrayElemPtr<[T; N], OUT>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoArrayElemFromElemBacking { parent }),
            index,
        }
    }

    pub fn from_go_ptr(parent: GoPtr<[T; N]>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoArrayElemFromGoPtrBacking { parent }),
            index,
        }
    }

    pub fn borrow(&self) -> GoArrayElemRef<T> {
        GoArrayElemRef {
            value: self.backing.borrow_at(self.index),
        }
    }

    pub fn borrow_mut(&self) -> GoArrayElemMutRef<T, N> {
        GoArrayElemMutRef {
            backing: self.backing.clone(),
            index: self.index,
            value: self.backing.borrow_at(self.index),
        }
    }

    /// The element as the guard of a cell, for code written against a
    /// Mutex-held value: sync/atomic's Load and Store on &a[i].
    #[allow(dead_code)]
    pub fn lock(&self) -> Result<GoArrayElemMutRef<T, N>, std::convert::Infallible> {
        Ok(self.borrow_mut())
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut value = self.backing.borrow_at(self.index).expect("nil pointer dereference");
        let result = f(&mut value);
        self.backing.assign_at(self.index, Some(value));
        result
    }

    pub fn identity(&self) -> (*const (), usize) {
        self.backing.identity_at(self.index)
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemMutRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::DerefMut for GoSliceElemMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone> Drop for GoSliceElemMutRef<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.slice.lock().unwrap().as_mut() {
                values[self.index] = value;
            }
        }
    }
}

impl<T: Clone> std::ops::Deref for GoArrayElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> std::ops::Deref for GoArrayElemMutRef<T, N> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> std::ops::DerefMut for GoArrayElemMutRef<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> Drop for GoArrayElemMutRef<T, N> {
    fn drop(&mut self) {
        self.backing.assign_at(self.index, self.value.clone());
    }
}

pub trait GoArrayElemPtrDyn<T: Send + Sync + 'static>: Send + Sync {
    fn borrow_dyn(&self) -> Option<T>;
    fn assign_dyn(&self, value: Option<T>);
    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T));
    fn identity_dyn(&self) -> (*const (), usize);
}

pub struct GoForeignArrayElemPtrDyn<T: Send + Sync + 'static> {
    borrow: Arc<dyn Fn() -> Option<T> + Send + Sync>,
    assign: Arc<dyn Fn(Option<T>) + Send + Sync>,
    with_mut: Arc<dyn Fn(&mut dyn FnMut(&mut T)) + Send + Sync>,
    identity: Arc<dyn Fn() -> (*const (), usize) + Send + Sync>,
}

impl<T: Send + Sync + 'static> GoArrayElemPtrDyn<T> for GoForeignArrayElemPtrDyn<T> {
    fn borrow_dyn(&self) -> Option<T> {
        (self.borrow)()
    }

    fn assign_dyn(&self, value: Option<T>) {
        (self.assign)(value)
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        (self.with_mut)(f)
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        (self.identity)()
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemPtrDyn<T> for GoArrayElemPtr<T, N> {
    fn borrow_dyn(&self) -> Option<T> {
        (*self.borrow()).clone()
    }

    fn assign_dyn(&self, value: Option<T>) {
        *self.borrow_mut() = value;
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        self.with_mut(|value| f(value));
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        self.identity()
    }
}

pub enum GoPtr<T: Send + Sync + 'static> {
    Nil,
    Raw(usize),
    Local(Arc<Mutex<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
    ArrayElem(Arc<dyn GoArrayElemPtrDyn<T> + Send + Sync>),
}

impl<T: Send + Sync + 'static> Clone for GoPtr<T> {
    fn clone(&self) -> Self {
        match self {
            GoPtr::Nil => GoPtr::Nil,
            GoPtr::Raw(addr) => GoPtr::Raw(*addr),
            GoPtr::Local(value) => GoPtr::Local(value.clone()),
            GoPtr::SliceElem(value) => GoPtr::SliceElem(GoSliceElemPtr { slice: value.slice.clone(), index: value.index, id: value.id }),
            GoPtr::ArrayElem(value) => GoPtr::ArrayElem(value.clone()),
        }
    }
}

impl<T: Send + Sync + 'static> GoPtr<T> {
    pub fn nil() -> Self {
        GoPtr::Nil
    }

    pub fn raw(addr: usize) -> Self {
        if addr == 0 {
            GoPtr::Nil
        } else {
            GoPtr::Raw(addr)
        }
    }

    pub fn local(value: Arc<Mutex<Option<T>>>) -> Self {
        if value.lock().unwrap().is_none() {
            GoPtr::Nil
        } else {
            GoPtr::Local(value)
        }
    }

    pub fn slice_elem(value: GoSliceElemPtr<T>) -> Self {
        GoPtr::SliceElem(value)
    }

    pub fn slice_elem_opt(value: Option<GoSliceElemPtr<T>>) -> Self {
        match value {
            Some(value) => GoPtr::SliceElem(value),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem<const N: usize>(value: GoArrayElemPtr<T, N>) -> Self
    where
        T: Clone,
    {
        GoPtr::ArrayElem(Arc::new(value))
    }

    pub fn array_elem_opt<const N: usize>(value: Option<GoArrayElemPtr<T, N>>) -> Self
    where
        T: Clone,
    {
        match value {
            Some(value) => GoPtr::ArrayElem(Arc::new(value)),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem_foreign(
        borrow: Arc<dyn Fn() -> Option<T> + Send + Sync>,
        assign: Arc<dyn Fn(Option<T>) + Send + Sync>,
        with_mut: Arc<dyn Fn(&mut dyn FnMut(&mut T)) + Send + Sync>,
        identity: Arc<dyn Fn() -> (*const (), usize) + Send + Sync>,
    ) -> Self {
        GoPtr::ArrayElem(Arc::new(GoForeignArrayElemPtrDyn {
            borrow,
            assign,
            with_mut,
            identity,
        }))
    }

    pub fn is_nil(&self) -> bool {
        match self {
            GoPtr::Nil => true,
            GoPtr::Raw(addr) => *addr == 0,
            GoPtr::Local(value) => value.lock().unwrap().is_none(),
            GoPtr::SliceElem(value) => {
                let guard = value.slice.lock().unwrap();
                guard.as_ref().and_then(|values| values.get(value.index)).is_none()
            }
            GoPtr::ArrayElem(value) => value.identity_dyn().0.is_null(),
        }
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer mutable borrow requires unsafe pointee support"),
            GoPtr::Local(slot) => {
                let mut guard = slot.lock().unwrap();
                f(guard.as_mut().unwrap())
            }
            GoPtr::SliceElem(slot) => {
                let mut guard = slot.slice.lock().unwrap();
                let values = guard.as_mut().expect("nil pointer dereference");
                f(values.get_mut(slot.index).expect("nil pointer dereference"))
            }
            GoPtr::ArrayElem(slot) => {
                let mut result = None;
                let mut callback = Some(f);
                slot.with_mut_dyn(&mut |value| {
                    let f = callback.take().expect("array element pointer mutable borrow called twice");
                    result = Some(f(value));
                });
                result.expect("nil pointer dereference")
            }
        }
    }

    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        match (left, right) {
            (GoPtr::Nil, GoPtr::Nil) => true,
            (GoPtr::Raw(_), _) | (_, GoPtr::Raw(_)) => left.addr() == right.addr(),
            (GoPtr::Local(left), GoPtr::Local(right)) => Arc::ptr_eq(left, right),
            (GoPtr::SliceElem(left), GoPtr::SliceElem(right)) => {
                left.identity() == right.identity()
            }
            (GoPtr::ArrayElem(left), GoPtr::ArrayElem(right)) => left.identity_dyn() == right.identity_dyn(),
            _ => false,
        }
    }

    pub fn addr(&self) -> usize {
        match self {
            GoPtr::Nil => 0,
            GoPtr::Raw(addr) => *addr,
            GoPtr::Local(value) => Arc::as_ptr(value) as usize,
            GoPtr::SliceElem(value) => {
                let (base, index) = value.identity();
                base.wrapping_add(index)
            }
            GoPtr::ArrayElem(value) => {
                let (base, index) = value.identity_dyn();
                (base as usize).wrapping_add(index)
            }
        }
    }
}

impl<T: Clone + Send + Sync + 'static> GoPtr<T> {
    /// The pointee, read the way the generated code reads a cell in a
    /// program with goroutines, ptr.lock().unwrap(), as borrow() is in one
    /// without: a []*T or map[K]*T element read through the cell path.
    pub fn lock(&self) -> Result<Option<T>, std::convert::Infallible> {
        Ok(self.borrow())
    }

    pub fn borrow(&self) -> Option<T> {
        match self {
            GoPtr::Nil => None,
            GoPtr::Raw(_) => panic!("raw unsafe pointer dereference requires unsafe pointee support"),
            GoPtr::Local(value) => (*value.lock().unwrap()).clone(),
            GoPtr::SliceElem(value) => (*value.borrow()).clone(),
            GoPtr::ArrayElem(value) => value.borrow_dyn(),
        }
    }

    pub fn assign(&self, value: Option<T>) {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer assignment requires unsafe pointee support"),
            GoPtr::Local(slot) => *slot.lock().unwrap() = value,
            GoPtr::SliceElem(slot) => *slot.borrow_mut() = value,
            GoPtr::ArrayElem(slot) => slot.assign_dyn(value),
        }
    }
}

impl<T: Send + Sync + 'static> Default for GoPtr<T> {
    fn default() -> Self {
        GoPtr::Nil
    }
}

impl<T: Send + Sync + 'static> std::fmt::Debug for GoPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nil() {
            write!(f, "<nil>")
        } else {
            write!(f, "<ptr>")
        }
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString(`
#[derive(Clone)]
pub struct GoSliceElemPtr<T> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
    /// The element's identity when the pointer was made with the slice
    /// free to borrow; otherwise identity reads it from the slice.
    id: Option<(usize, usize)>,
}

pub struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoSliceElemMutRef<T: Clone> {
    slice: Rc<RefCell<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

pub trait GoArrayElemBacking<T: Clone + 'static, const N: usize> {
    fn borrow_at(&self, index: usize) -> Option<T>;
    fn assign_at(&self, index: usize, value: Option<T>);
    fn identity_at(&self, index: usize) -> (*const (), usize);
}

#[derive(Clone)]
pub struct GoDirectArrayElemBacking<T: Clone + 'static, const N: usize> {
    array: Rc<RefCell<Option<[T; N]>>>,
}

impl<T: Clone + 'static, const N: usize> GoArrayElemBacking<T, N> for GoDirectArrayElemBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.array.borrow();
        guard.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.array.borrow_mut().as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Rc::as_ptr(&self.array) as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoNestedArrayElemBacking<T: Clone + 'static, const N: usize, const OUT: usize> {
    outer: Rc<RefCell<Option<[[T; N]; OUT]>>>,
    outer_index: usize,
}

impl<T: Clone + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoNestedArrayElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.outer.borrow();
        guard.as_ref().and_then(|values| values.get(self.outer_index)).and_then(|inner| inner.get(index)).cloned()
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.outer.borrow_mut().as_mut() {
                values[self.outer_index][index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Rc::as_ptr(&self.outer) as *const (), self.outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromElemBacking<T: Clone + 'static, const N: usize, const OUT: usize> {
    parent: GoArrayElemPtr<[T; N], OUT>,
}

impl<T: Clone + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoArrayElemFromElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            let mut inner = self.parent.borrow_mut();
            if let Some(values) = inner.as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        let (base, outer_index) = self.parent.identity();
        (base, outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromGoPtrBacking<T: Clone + 'static, const N: usize> {
    parent: GoPtr<[T; N]>,
}

impl<T: Clone + 'static, const N: usize> GoArrayElemBacking<T, N> for GoArrayElemFromGoPtrBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            self.parent.with_mut(|values| {
                values[index] = value;
            });
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (self.parent.addr() as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoArrayElemPtr<T: Clone + 'static, const N: usize> {
    backing: Rc<dyn GoArrayElemBacking<T, N>>,
    index: usize,
}

pub struct GoArrayElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoArrayElemMutRef<T: Clone + 'static, const N: usize> {
    backing: Rc<dyn GoArrayElemBacking<T, N>>,
    index: usize,
    value: Option<T>,
}

impl<T> GoSliceElemPtr<T> {
`)
		writeSliceElemPtrConstructor(out, "Rc", "RefCell", "try_borrow")
		out.WriteString(`    pub fn slice_handle(&self) -> Rc<RefCell<Option<Vec<T>>>> {
        self.slice.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<T: Clone> GoSliceElemPtr<T> {
    pub fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    pub fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.borrow();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone + 'static, const N: usize> GoArrayElemPtr<T, N> {
    pub fn new(array: Rc<RefCell<Option<[T; N]>>>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoDirectArrayElemBacking { array }),
            index,
        }
    }

    pub fn nested<const OUT: usize>(outer: Rc<RefCell<Option<[[T; N]; OUT]>>>, outer_index: usize, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoNestedArrayElemBacking { outer, outer_index }),
            index,
        }
    }

    pub fn from_array_elem<const OUT: usize>(parent: GoArrayElemPtr<[T; N], OUT>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoArrayElemFromElemBacking { parent }),
            index,
        }
    }

    pub fn from_go_ptr(parent: GoPtr<[T; N]>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Rc::new(GoArrayElemFromGoPtrBacking { parent }),
            index,
        }
    }

    pub fn borrow(&self) -> GoArrayElemRef<T> {
        GoArrayElemRef {
            value: self.backing.borrow_at(self.index),
        }
    }

    pub fn borrow_mut(&self) -> GoArrayElemMutRef<T, N> {
        GoArrayElemMutRef {
            backing: self.backing.clone(),
            index: self.index,
            value: self.backing.borrow_at(self.index),
        }
    }

    /// The element as the guard of a cell, for code written against a
    /// Mutex-held value: sync/atomic's Load and Store on &a[i].
    #[allow(dead_code)]
    pub fn lock(&self) -> Result<GoArrayElemMutRef<T, N>, std::convert::Infallible> {
        Ok(self.borrow_mut())
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut value = self.backing.borrow_at(self.index).expect("nil pointer dereference");
        let result = f(&mut value);
        self.backing.assign_at(self.index, Some(value));
        result
    }

    pub fn identity(&self) -> (*const (), usize) {
        self.backing.identity_at(self.index)
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemMutRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::DerefMut for GoSliceElemMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone> Drop for GoSliceElemMutRef<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.slice.borrow_mut().as_mut() {
                values[self.index] = value;
            }
        }
    }
}

impl<T: Clone> std::ops::Deref for GoArrayElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + 'static, const N: usize> std::ops::Deref for GoArrayElemMutRef<T, N> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + 'static, const N: usize> std::ops::DerefMut for GoArrayElemMutRef<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone + 'static, const N: usize> Drop for GoArrayElemMutRef<T, N> {
    fn drop(&mut self) {
        self.backing.assign_at(self.index, self.value.clone());
    }
}

pub trait GoArrayElemPtrDyn<T: 'static> {
    fn borrow_dyn(&self) -> Option<T>;
    fn assign_dyn(&self, value: Option<T>);
    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T));
    fn identity_dyn(&self) -> (*const (), usize);
}

pub struct GoForeignArrayElemPtrDyn<T: 'static> {
    borrow: Rc<dyn Fn() -> Option<T>>,
    assign: Rc<dyn Fn(Option<T>)>,
    with_mut: Rc<dyn Fn(&mut dyn FnMut(&mut T))>,
    identity: Rc<dyn Fn() -> (*const (), usize)>,
}

impl<T: 'static> GoArrayElemPtrDyn<T> for GoForeignArrayElemPtrDyn<T> {
    fn borrow_dyn(&self) -> Option<T> {
        (self.borrow)()
    }

    fn assign_dyn(&self, value: Option<T>) {
        (self.assign)(value)
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        (self.with_mut)(f)
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        (self.identity)()
    }
}

impl<T: Clone + 'static, const N: usize> GoArrayElemPtrDyn<T> for GoArrayElemPtr<T, N> {
    fn borrow_dyn(&self) -> Option<T> {
        (*self.borrow()).clone()
    }

    fn assign_dyn(&self, value: Option<T>) {
        *self.borrow_mut() = value;
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        self.with_mut(|value| f(value));
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        self.identity()
    }
}

pub enum GoPtr<T: 'static> {
    Nil,
    Raw(usize),
    Local(Rc<RefCell<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
    ArrayElem(Rc<dyn GoArrayElemPtrDyn<T>>),
}

impl<T: 'static> Clone for GoPtr<T> {
    fn clone(&self) -> Self {
        match self {
            GoPtr::Nil => GoPtr::Nil,
            GoPtr::Raw(addr) => GoPtr::Raw(*addr),
            GoPtr::Local(value) => GoPtr::Local(value.clone()),
            GoPtr::SliceElem(value) => GoPtr::SliceElem(GoSliceElemPtr { slice: value.slice.clone(), index: value.index, id: value.id }),
            GoPtr::ArrayElem(value) => GoPtr::ArrayElem(value.clone()),
        }
    }
}

impl<T: 'static> GoPtr<T> {
    pub fn nil() -> Self {
        GoPtr::Nil
    }

    pub fn raw(addr: usize) -> Self {
        if addr == 0 {
            GoPtr::Nil
        } else {
            GoPtr::Raw(addr)
        }
    }

    pub fn local(value: Rc<RefCell<Option<T>>>) -> Self {
        if value.borrow().is_none() {
            GoPtr::Nil
        } else {
            GoPtr::Local(value)
        }
    }

    pub fn slice_elem(value: GoSliceElemPtr<T>) -> Self {
        GoPtr::SliceElem(value)
    }

    pub fn slice_elem_opt(value: Option<GoSliceElemPtr<T>>) -> Self {
        match value {
            Some(value) => GoPtr::SliceElem(value),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem<const N: usize>(value: GoArrayElemPtr<T, N>) -> Self
    where
        T: Clone,
    {
        GoPtr::ArrayElem(Rc::new(value))
    }

    pub fn array_elem_opt<const N: usize>(value: Option<GoArrayElemPtr<T, N>>) -> Self
    where
        T: Clone,
    {
        match value {
            Some(value) => GoPtr::ArrayElem(Rc::new(value)),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem_foreign(
        borrow: Rc<dyn Fn() -> Option<T>>,
        assign: Rc<dyn Fn(Option<T>)>,
        with_mut: Rc<dyn Fn(&mut dyn FnMut(&mut T))>,
        identity: Rc<dyn Fn() -> (*const (), usize)>,
    ) -> Self {
        GoPtr::ArrayElem(Rc::new(GoForeignArrayElemPtrDyn {
            borrow,
            assign,
            with_mut,
            identity,
        }))
    }

    pub fn is_nil(&self) -> bool {
        match self {
            GoPtr::Nil => true,
            GoPtr::Raw(addr) => *addr == 0,
            GoPtr::Local(value) => value.borrow().is_none(),
            GoPtr::SliceElem(value) => {
                let guard = value.slice.borrow();
                guard.as_ref().and_then(|values| values.get(value.index)).is_none()
            }
            GoPtr::ArrayElem(value) => value.identity_dyn().0.is_null(),
        }
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer mutable borrow requires unsafe pointee support"),
            GoPtr::Local(slot) => {
                let mut guard = slot.borrow_mut();
                f(guard.as_mut().unwrap())
            }
            GoPtr::SliceElem(slot) => {
                let mut guard = slot.slice.borrow_mut();
                let values = guard.as_mut().expect("nil pointer dereference");
                f(values.get_mut(slot.index).expect("nil pointer dereference"))
            }
            GoPtr::ArrayElem(slot) => {
                let mut result = None;
                let mut callback = Some(f);
                slot.with_mut_dyn(&mut |value| {
                    let f = callback.take().expect("array element pointer mutable borrow called twice");
                    result = Some(f(value));
                });
                result.expect("nil pointer dereference")
            }
        }
    }

    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        match (left, right) {
            (GoPtr::Nil, GoPtr::Nil) => true,
            (GoPtr::Raw(_), _) | (_, GoPtr::Raw(_)) => left.addr() == right.addr(),
            (GoPtr::Local(left), GoPtr::Local(right)) => Rc::ptr_eq(left, right),
            (GoPtr::SliceElem(left), GoPtr::SliceElem(right)) => {
                left.identity() == right.identity()
            }
            (GoPtr::ArrayElem(left), GoPtr::ArrayElem(right)) => left.identity_dyn() == right.identity_dyn(),
            _ => false,
        }
    }

    pub fn addr(&self) -> usize {
        match self {
            GoPtr::Nil => 0,
            GoPtr::Raw(addr) => *addr,
            GoPtr::Local(value) => Rc::as_ptr(value) as usize,
            GoPtr::SliceElem(value) => {
                let (base, index) = value.identity();
                base.wrapping_add(index)
            }
            GoPtr::ArrayElem(value) => {
                let (base, index) = value.identity_dyn();
                (base as usize).wrapping_add(index)
            }
        }
    }
}

impl<T: Clone + 'static> GoPtr<T> {
    pub fn borrow(&self) -> Option<T> {
        match self {
            GoPtr::Nil => None,
            GoPtr::Raw(_) => panic!("raw unsafe pointer dereference requires unsafe pointee support"),
            GoPtr::Local(value) => (*value.borrow()).clone(),
            GoPtr::SliceElem(value) => (*value.borrow()).clone(),
            GoPtr::ArrayElem(value) => value.borrow_dyn(),
        }
    }

    pub fn assign(&self, value: Option<T>) {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer assignment requires unsafe pointee support"),
            GoPtr::Local(slot) => *slot.borrow_mut() = value,
            GoPtr::SliceElem(slot) => *slot.borrow_mut() = value,
            GoPtr::ArrayElem(slot) => slot.assign_dyn(value),
        }
    }
}

impl<T: 'static> Default for GoPtr<T> {
    fn default() -> Self {
        GoPtr::Nil
    }
}

impl<T: 'static> std::fmt::Debug for GoPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nil() {
            write!(f, "<nil>")
        } else {
            write!(f, "<ptr>")
        }
    }
}
`)
	}
}

func generateGoTimeHelper(out *strings.Builder) {
	var code strings.Builder
	code.WriteString(`
#[derive(Clone, Debug, Default)]
struct GoTime {
    seconds: i64,
    nanos: i32,
}

fn go_time_civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (year, m as u32, d as u32)
}

impl GoTime {
    fn now() -> Self {
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        GoTime {
            seconds: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        }
    }

    fn from_unix(seconds: i64, nanos: i64) -> Self {
        let seconds = seconds + nanos.div_euclid(1_000_000_000);
        let nanos = nanos.rem_euclid(1_000_000_000);
        GoTime {
            seconds,
            nanos: nanos as i32,
        }
    }
`)
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		code.WriteString(`
    fn add(&self, duration: Arc<Mutex<Option<std::time::Duration>>>) -> Arc<Mutex<Option<GoTime>>> {
        let duration = *duration.lock().unwrap().as_ref().unwrap();
        Arc::new(Mutex::new(Some(GoTime::from_unix(
            self.seconds + duration.as_secs() as i64,
            self.nanos as i64 + duration.subsec_nanos() as i64,
        ))))
    }

    fn u_t_c(&self) -> Arc<Mutex<Option<GoTime>>> {
        Arc::new(Mutex::new(Some(self.clone())))
    }

    fn unix(&self) -> i64 {
        self.seconds
    }

    fn unix_nano(&self) -> i64 {
        self.seconds * 1_000_000_000 + self.nanos as i64
    }

    fn is_zero(&self) -> bool {
        self.seconds == 0 && self.nanos == 0
    }

    fn format(&self, _layout: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(self.to_string())))
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		code.WriteString(`
    fn add(&self, duration: Rc<RefCell<Option<std::time::Duration>>>) -> Rc<RefCell<Option<GoTime>>> {
        let duration = *duration.borrow().as_ref().unwrap();
        Rc::new(RefCell::new(Some(GoTime::from_unix(
            self.seconds + duration.as_secs() as i64,
            self.nanos as i64 + duration.subsec_nanos() as i64,
        ))))
    }

    fn u_t_c(&self) -> Rc<RefCell<Option<GoTime>>> {
        Rc::new(RefCell::new(Some(self.clone())))
    }

    fn unix(&self) -> i64 {
        self.seconds
    }

    fn unix_nano(&self) -> i64 {
        self.seconds * 1_000_000_000 + self.nanos as i64
    }

    fn is_zero(&self) -> bool {
        self.seconds == 0 && self.nanos == 0
    }

    fn format(&self, _layout: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(self.to_string())))
    }
}
`)
	}
	code.WriteString(`
impl std::fmt::Display for GoTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let days = self.seconds.div_euclid(86_400);
        let secs_of_day = self.seconds.rem_euclid(86_400);
        let (year, month, day) = go_time_civil_from_days(days);
        let hour = secs_of_day / 3_600;
        let minute = (secs_of_day % 3_600) / 60;
        let second = secs_of_day % 60;
        if self.nanos == 0 {
            write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02} +0000 UTC",
                year, month, day, hour, minute, second
            )
        } else {
            let mut fraction = format!("{:09}", self.nanos);
            while fraction.ends_with('0') {
                fraction.pop();
            }
            write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{} +0000 UTC",
                year, month, day, hour, minute, second, fraction
            )
        }
    }
}
`)
	helper := code.String()
	if generatingPublicHelpers {
		helper = strings.ReplaceAll(helper, "struct GoTime", "pub struct GoTime")
		helper = strings.ReplaceAll(helper, "fn go_time_civil_from_days(", "pub fn go_time_civil_from_days(")
		helper = strings.ReplaceAll(helper, "    fn now(", "    pub fn now(")
		helper = strings.ReplaceAll(helper, "    fn from_unix(", "    pub fn from_unix(")
		helper = strings.ReplaceAll(helper, "    fn add(", "    pub fn add(")
		helper = strings.ReplaceAll(helper, "    fn u_t_c(", "    pub fn u_t_c(")
		helper = strings.ReplaceAll(helper, "    fn unix(", "    pub fn unix(")
		helper = strings.ReplaceAll(helper, "    fn unix_nano(", "    pub fn unix_nano(")
		helper = strings.ReplaceAll(helper, "    fn is_zero(", "    pub fn is_zero(")
		helper = strings.ReplaceAll(helper, "    fn format(", "    pub fn format(")
	}
	out.WriteString(helper)
}

func generateGoTimerHelper(out *strings.Builder) {
	NeedGoChannel()
	NeedGoTime()
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Clone)]
struct GoTimer {
    c: GoChannel<GoTime>,
    stopped: Arc<std::sync::atomic::AtomicBool>,
}

fn go_new_timer(duration: std::time::Duration) -> GoTimer {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    let stopped = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let thread_stopped = stopped.clone();
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        if !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
            thread_channel.send(GoTime::now());
        }
        thread_channel.close();
    });

    GoTimer {
        c: channel,
        stopped,
    }
}

fn go_after_func<F>(duration: std::time::Duration, callback: F) -> GoTimer
where
    F: FnOnce() + Send + 'static,
{
    let timer = go_new_timer(duration);
    let stopped = timer.stopped.clone();
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        if !stopped.load(std::sync::atomic::Ordering::SeqCst) {
            callback();
        }
    });
    timer
}

impl GoTimer {
    fn stop(&self) -> bool {
        let was_stopped = self.stopped.swap(true, std::sync::atomic::Ordering::SeqCst);
        !was_stopped
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Clone)]
struct GoTimer {
    c: GoChannel<GoTime>,
    stopped: Arc<std::sync::atomic::AtomicBool>,
}

fn go_new_timer(duration: std::time::Duration) -> GoTimer {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    let stopped = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let thread_stopped = stopped.clone();
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        if !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
            thread_channel.send(GoTime::now());
        }
        thread_channel.close();
    });

    GoTimer {
        c: channel,
        stopped,
    }
}

fn go_after_func<F>(duration: std::time::Duration, callback: F) -> GoTimer
where
    F: FnOnce() + Send + 'static,
{
    let timer = go_new_timer(duration);
    let stopped = timer.stopped.clone();
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        if !stopped.load(std::sync::atomic::Ordering::SeqCst) {
            callback();
        }
    });
    timer
}

impl GoTimer {
    fn stop(&self) -> bool {
        let was_stopped = self.stopped.swap(true, std::sync::atomic::Ordering::SeqCst);
        !was_stopped
    }
}
`)
	}
}

func generateGoAfterHelper(out *strings.Builder) {
	NeedGoChannel()
	NeedGoTime()
	out.WriteString(`
fn go_channel_after(duration: std::time::Duration) -> GoChannel<GoTime> {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        thread_channel.send(GoTime::now());
        thread_channel.close();
    });
    channel
}
`)
}

func generateGoTickHelper(out *strings.Builder) {
	NeedGoChannel()
	NeedGoTime()
	out.WriteString(`
fn go_tick(duration: std::time::Duration) -> GoChannel<GoTime> {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(duration);
            let _ = thread_channel.try_send(GoTime::now());
        }
    });
    channel
}
`)
}

func generateGoTickerHelper(out *strings.Builder) {
	NeedGoChannel()
	NeedGoTime()
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Clone)]
struct GoTicker {
    c: GoChannel<GoTime>,
    stopped: Arc<std::sync::atomic::AtomicBool>,
}

fn go_new_ticker(duration: std::time::Duration) -> GoTicker {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    let stopped = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let thread_stopped = stopped.clone();
    std::thread::spawn(move || {
        while !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
            std::thread::sleep(duration);
            if !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
                let _ = thread_channel.try_send(GoTime::now());
            }
        }
        thread_channel.close();
    });

    GoTicker {
        c: channel,
        stopped,
    }
}

impl GoTicker {
    fn stop(&self) {
        self.stopped.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
`)
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Clone)]
struct GoTicker {
    c: GoChannel<GoTime>,
    stopped: Arc<std::sync::atomic::AtomicBool>,
}

fn go_new_ticker(duration: std::time::Duration) -> GoTicker {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    let stopped = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let thread_stopped = stopped.clone();
    std::thread::spawn(move || {
        while !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
            std::thread::sleep(duration);
            if !thread_stopped.load(std::sync::atomic::Ordering::SeqCst) {
                let _ = thread_channel.try_send(GoTime::now());
            }
        }
        thread_channel.close();
    });

    GoTicker {
        c: channel,
        stopped,
    }
}

impl GoTicker {
    fn stop(&self) {
        self.stopped.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
`)
	}
}

func generateGoContextHelper(out *strings.Builder) {
	NeedGoChannel()
	TrackImport("Arc")
	TrackImport("Mutex")
	code := `
#[derive(Clone)]
struct GoContext {
    done: GoChannel<bool>,
    err: std::sync::Arc<std::sync::Mutex<Option<GO_ERROR_PAYLOAD>>>,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    label: String,
}

type GoCancelFunc = std::sync::Arc<dyn Fn() + Send + Sync>;
type GoCancelCauseFunc = Arc<dyn Fn(Arc<Mutex<Option<GO_ERROR_PAYLOAD>>>) -> () + Send + Sync>;

impl GoContext {
    fn background() -> GoContext {
        GoContext {
            done: GoChannel::<bool>::new(),
            err: std::sync::Arc::new(std::sync::Mutex::new(None)),
            cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            label: "context.Background".to_string(),
        }
    }

    fn parent_label(parent: &Arc<Mutex<Option<GoContext>>>) -> String {
        parent
            .lock()
            .unwrap()
            .as_ref()
            .map(|ctx| ctx.label.clone())
            .unwrap_or_else(|| "context.Context".to_string())
    }

    fn with_timeout(parent: Arc<Mutex<Option<GoContext>>>, duration: std::time::Duration) -> (Arc<Mutex<Option<GoContext>>>, Arc<Mutex<Option<GoCancelFunc>>>) {
        let label = format!("{}.WithDeadline", GoContext::parent_label(&parent));
        let done = GoChannel::<bool>::new_buffered(1);
        let err = std::sync::Arc::new(std::sync::Mutex::new(None::<GO_ERROR_PAYLOAD>));
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let context = GoContext {
            done: done.clone(),
            err: err.clone(),
            cancelled: cancelled.clone(),
            label,
        };

        let timeout_done = done.clone();
        let timeout_err = err.clone();
        let timeout_cancelled = cancelled.clone();
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            if !timeout_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *timeout_err.lock().unwrap() = Some(GO_ERROR_FROM("context deadline exceeded".to_string()));
                timeout_done.send(true);
                timeout_done.close();
            }
        });

        let cancel_done = done.clone();
        let cancel_err = err.clone();
        let cancel_cancelled = cancelled.clone();
        let cancel: GoCancelFunc = std::sync::Arc::new(move || {
            if !cancel_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *cancel_err.lock().unwrap() = Some(GO_ERROR_FROM("context canceled".to_string()));
                cancel_done.send(true);
                cancel_done.close();
            }
        });

        (
            Arc::new(Mutex::new(Some(context))),
            Arc::new(Mutex::new(Some(cancel))),
        )
    }

    fn with_cancel(parent: Arc<Mutex<Option<GoContext>>>) -> (Arc<Mutex<Option<GoContext>>>, Arc<Mutex<Option<GoCancelFunc>>>) {
        let label = format!("{}.WithCancel", GoContext::parent_label(&parent));
        let done = GoChannel::<bool>::new_buffered(1);
        let err = std::sync::Arc::new(std::sync::Mutex::new(None::<GO_ERROR_PAYLOAD>));
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let context = GoContext {
            done: done.clone(),
            err: err.clone(),
            cancelled: cancelled.clone(),
            label,
        };

        let cancel_done = done.clone();
        let cancel_err = err.clone();
        let cancel_cancelled = cancelled.clone();
        let cancel: GoCancelFunc = std::sync::Arc::new(move || {
            if !cancel_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *cancel_err.lock().unwrap() = Some(GO_ERROR_FROM("context canceled".to_string()));
                cancel_done.send(true);
                cancel_done.close();
            }
        });

        (
            Arc::new(Mutex::new(Some(context))),
            Arc::new(Mutex::new(Some(cancel))),
        )
    }

    fn with_cancel_cause(parent: Arc<Mutex<Option<GoContext>>>) -> (Arc<Mutex<Option<GoContext>>>, Arc<Mutex<Option<GoCancelCauseFunc>>>) {
        let label = format!("{}.WithCancelCause", GoContext::parent_label(&parent));
        let done = GoChannel::<bool>::new_buffered(1);
        let err = std::sync::Arc::new(std::sync::Mutex::new(None::<GO_ERROR_PAYLOAD>));
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let context = GoContext {
            done: done.clone(),
            err: err.clone(),
            cancelled: cancelled.clone(),
            label,
        };

        let cancel_done = done.clone();
        let cancel_err = err.clone();
        let cancel_cancelled = cancelled.clone();
        let cancel: GoCancelCauseFunc = Arc::new(move |_cause| {
            if !cancel_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *cancel_err.lock().unwrap() = Some(GO_ERROR_FROM("context canceled".to_string()));
                cancel_done.send(true);
                cancel_done.close();
            }
        });

        (
            Arc::new(Mutex::new(Some(context))),
            Arc::new(Mutex::new(Some(cancel))),
        )
    }

    fn done(&self) -> GoChannel<bool> {
        self.done.clone()
    }

    fn err(&self) -> Arc<Mutex<Option<GO_ERROR_PAYLOAD>>> {
        self.err.clone()
    }
}

impl std::fmt::Display for GoContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl std::fmt::Debug for GoContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
`
	if generatingPublicHelpers {
		code = strings.ReplaceAll(code, "struct GoContext", "pub struct GoContext")
		code = strings.ReplaceAll(code, "type GoCancelFunc", "pub type GoCancelFunc")
		code = strings.ReplaceAll(code, "type GoCancelCauseFunc", "pub type GoCancelCauseFunc")
		code = strings.ReplaceAll(code, "    fn background(", "    pub fn background(")
		code = strings.ReplaceAll(code, "    fn with_timeout(", "    pub fn with_timeout(")
		code = strings.ReplaceAll(code, "    fn with_cancel(", "    pub fn with_cancel(")
		code = strings.ReplaceAll(code, "    fn with_cancel_cause(", "    pub fn with_cancel_cause(")
		code = strings.ReplaceAll(code, "    fn done(", "    pub fn done(")
		code = strings.ReplaceAll(code, "    fn err(", "    pub fn err(")
	}
	out.WriteString(expandErrorRepr(code, true))
}

func generateGoRandHelper(out *strings.Builder) {
	out.WriteString(`
fn go_rand_state() -> &'static std::sync::Mutex<u64> {
    static STATE: std::sync::OnceLock<std::sync::Mutex<u64>> = std::sync::OnceLock::new();
    STATE.get_or_init(|| std::sync::Mutex::new(1))
}

fn go_rand_seed(seed: i64) {
    *go_rand_state().lock().unwrap() = seed as u64;
}

fn go_rand_next_u64() -> u64 {
    let mut state = go_rand_state().lock().unwrap();
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

fn go_rand_intn(n: i64) -> i64 {
    if n <= 0 {
        panic!("invalid argument to Intn");
    }
    (go_rand_next_u64() % n as u64) as i64
}

fn go_rand_float64() -> f64 {
    ((go_rand_next_u64() >> 11) as f64) / ((1u64 << 53) as f64)
}
`)
}

func generateOsArgsHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		out.WriteString(`
static __GO_OS_ARGS: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> =
    std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(Some(std::env::args().collect::<Vec<String>>()))));

fn go_os_args() -> std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>> {
    __GO_OS_ARGS.clone()
}
`)
		return
	}

	out.WriteString(`
thread_local! {
    static __GO_OS_ARGS: std::rc::Rc<std::cell::RefCell<Option<Vec<String>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(Some(std::env::args().collect::<Vec<String>>())));
}

fn go_os_args() -> std::rc::Rc<std::cell::RefCell<Option<Vec<String>>>> {
    __GO_OS_ARGS.with(|args| args.clone())
}
`)
}

func generateReflectHelper(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoReflectStructTag {
    raw: Arc<Mutex<Option<String>>>,
}

impl GoReflectStructTag {
    fn get(&self, key: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        let raw = (*self.raw.lock().unwrap().as_ref().unwrap()).clone();
        let key = (*key.lock().unwrap().as_ref().unwrap()).clone();
        Arc::new(Mutex::new(Some(go_reflect_tag_get(&raw, &key))))
    }
}

#[derive(Debug, Clone, Default)]
struct GoReflectField {
    name: Arc<Mutex<Option<String>>>,
    tag: Arc<Mutex<Option<GoReflectStructTag>>>,
}

#[derive(Debug, Clone, Default)]
struct GoReflectType {
    name: Arc<Mutex<Option<String>>>,
    kind: Arc<Mutex<Option<reflect_Kind>>>,
    elem: Arc<Mutex<Option<Box<GoReflectType>>>>,
    fields: Arc<Mutex<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for GoReflectType {
    fn eq(&self, other: &Self) -> bool {
        *self.name.lock().unwrap() == *other.name.lock().unwrap() &&
            *self.kind.lock().unwrap() == *other.kind.lock().unwrap()
    }
}

impl Eq for GoReflectType {}

impl GoReflectType {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some((*self.name.lock().unwrap().as_ref().unwrap()).clone())))
    }

    fn kind(&self) -> Arc<Mutex<Option<reflect_Kind>>> {
        self.kind.clone()
    }

    fn elem(&self) -> Arc<Mutex<Option<GoReflectType>>> {
        let elem_guard = self.elem.lock().unwrap();
        let elem = elem_guard.as_ref().expect("reflect.Type.Elem requires an element type").as_ref().clone();
        Arc::new(Mutex::new(Some(elem)))
    }

    fn num_field(&self) -> i64 {
        self.fields.lock().unwrap().as_ref().unwrap().len() as i64
    }

    fn field(&self, index: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<GoReflectField>>> {
        let index = *index.lock().unwrap().as_ref().unwrap() as usize;
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap()[index].clone())))
    }
}

type GoReflectBoolGetter = Box<dyn Fn() -> bool + Send + Sync>;
type GoReflectBoolSetter = Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> () + Send + Sync>;

#[derive(Clone)]
struct GoReflectValue {
    typ: Arc<Mutex<Option<GoReflectType>>>,
    fields: Arc<Mutex<Option<Vec<GoReflectValue>>>>,
    bool_getter: Arc<Mutex<Option<GoReflectBoolGetter>>>,
    bool_setter: Arc<Mutex<Option<GoReflectBoolSetter>>>,
    unsupported: Option<&'static str>,
}

impl GoReflectValue {
    fn panic_if_unsupported(&self, op: &str) {
        if let Some(message) = self.unsupported {
            panic!("{}: {}", op, message);
        }
    }

    fn elem(&self) -> Arc<Mutex<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Elem");
        Arc::new(Mutex::new(Some(self.clone())))
    }

    fn r#type(&self) -> Arc<Mutex<Option<GoReflectType>>> {
        self.panic_if_unsupported("reflect.Value.Type");
        self.typ.clone()
    }

    fn kind(&self) -> Arc<Mutex<Option<reflect_Kind>>> {
        self.panic_if_unsupported("reflect.Value.Kind");
        self.typ.lock().unwrap().as_ref().unwrap().kind()
    }

    fn field(&self, index: i64) -> Arc<Mutex<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Field");
        let index = index as usize;
        Arc::new(Mutex::new(Some(self.fields.lock().unwrap().as_ref().unwrap()[index].clone())))
    }

    fn set<T>(&self, _value: T) {
        self.panic_if_unsupported("reflect.Value.Set");
        panic!("reflect.Value.Set requires typed lowering")
    }

    fn set_bool(&mut self, value: Arc<Mutex<Option<bool>>>) {
        self.panic_if_unsupported("reflect.Value.SetBool");
        let mut setter_guard = self.bool_setter.lock().unwrap();
        let setter = setter_guard.as_mut().expect("reflect.Value.SetBool requires a settable bool field");
        setter(value);
    }

    fn bool(&self) -> bool {
        self.panic_if_unsupported("reflect.Value.Bool");
        let getter_guard = self.bool_getter.lock().unwrap();
        let getter = getter_guard.as_ref().expect("reflect.Value.Bool requires a bool field");
        getter()
    }
}

fn go_reflect_tag_get(raw: &str, key: &str) -> String {
    let prefix = format!("{}:\"", key);
    let Some(start) = raw.find(&prefix) else {
        return String::new();
    };
    let rest = &raw[start + prefix.len()..];
    let mut value = String::new();
    let mut escaped = false;
    for ch in rest.chars() {
        if escaped {
            value.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            break;
        } else {
            value.push(ch);
        }
    }
    value
}
`)
		return
	}

	TrackImport("Rc")
	TrackImport("RefCell")
	out.WriteString(`
#[derive(Debug, Clone, Default)]
struct GoReflectStructTag {
    raw: Rc<RefCell<Option<String>>>,
}

impl GoReflectStructTag {
    fn get(&self, key: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        let raw = (*self.raw.borrow().as_ref().unwrap()).clone();
        let key = (*key.borrow().as_ref().unwrap()).clone();
        Rc::new(RefCell::new(Some(go_reflect_tag_get(&raw, &key))))
    }
}

#[derive(Debug, Clone, Default)]
struct GoReflectField {
    name: Rc<RefCell<Option<String>>>,
    tag: Rc<RefCell<Option<GoReflectStructTag>>>,
}

#[derive(Debug, Clone, Default)]
struct GoReflectType {
    name: Rc<RefCell<Option<String>>>,
    kind: Rc<RefCell<Option<reflect_Kind>>>,
    elem: Rc<RefCell<Option<Box<GoReflectType>>>>,
    fields: Rc<RefCell<Option<Vec<GoReflectField>>>>,
}

impl std::fmt::Display for GoReflectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.borrow().as_ref().unwrap())
    }
}

impl PartialEq for GoReflectType {
    fn eq(&self, other: &Self) -> bool {
        *self.name.borrow() == *other.name.borrow() &&
            *self.kind.borrow() == *other.kind.borrow()
    }
}

impl Eq for GoReflectType {}

impl GoReflectType {
    fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some((*self.name.borrow().as_ref().unwrap()).clone())))
    }

    fn kind(&self) -> Rc<RefCell<Option<reflect_Kind>>> {
        self.kind.clone()
    }

    fn elem(&self) -> Rc<RefCell<Option<GoReflectType>>> {
        let elem_guard = self.elem.borrow();
        let elem = elem_guard.as_ref().expect("reflect.Type.Elem requires an element type").as_ref().clone();
        Rc::new(RefCell::new(Some(elem)))
    }

    fn num_field(&self) -> i64 {
        self.fields.borrow().as_ref().unwrap().len() as i64
    }

    fn field(&self, index: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<GoReflectField>>> {
        let index = *index.borrow().as_ref().unwrap() as usize;
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap()[index].clone())))
    }
}

type GoReflectBoolGetter = Box<dyn Fn() -> bool>;
type GoReflectBoolSetter = Box<dyn FnMut(Rc<RefCell<Option<bool>>>) -> ()>;

#[derive(Clone)]
struct GoReflectValue {
    typ: Rc<RefCell<Option<GoReflectType>>>,
    fields: Rc<RefCell<Option<Vec<GoReflectValue>>>>,
    bool_getter: Rc<RefCell<Option<GoReflectBoolGetter>>>,
    bool_setter: Rc<RefCell<Option<GoReflectBoolSetter>>>,
    unsupported: Option<&'static str>,
}

impl GoReflectValue {
    fn panic_if_unsupported(&self, op: &str) {
        if let Some(message) = self.unsupported {
            panic!("{}: {}", op, message);
        }
    }

    fn elem(&self) -> Rc<RefCell<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Elem");
        Rc::new(RefCell::new(Some(self.clone())))
    }

    fn r#type(&self) -> Rc<RefCell<Option<GoReflectType>>> {
        self.panic_if_unsupported("reflect.Value.Type");
        self.typ.clone()
    }

    fn kind(&self) -> Rc<RefCell<Option<reflect_Kind>>> {
        self.panic_if_unsupported("reflect.Value.Kind");
        self.typ.borrow().as_ref().unwrap().kind()
    }

    fn field(&self, index: i64) -> Rc<RefCell<Option<GoReflectValue>>> {
        self.panic_if_unsupported("reflect.Value.Field");
        let index = index as usize;
        Rc::new(RefCell::new(Some(self.fields.borrow().as_ref().unwrap()[index].clone())))
    }

    fn set<T>(&self, _value: T) {
        self.panic_if_unsupported("reflect.Value.Set");
        panic!("reflect.Value.Set requires typed lowering")
    }

    fn set_bool(&mut self, value: Rc<RefCell<Option<bool>>>) {
        self.panic_if_unsupported("reflect.Value.SetBool");
        let mut setter_guard = self.bool_setter.borrow_mut();
        let setter = setter_guard.as_mut().expect("reflect.Value.SetBool requires a settable bool field");
        setter(value);
    }

    fn bool(&self) -> bool {
        self.panic_if_unsupported("reflect.Value.Bool");
        let getter_guard = self.bool_getter.borrow();
        let getter = getter_guard.as_ref().expect("reflect.Value.Bool requires a bool field");
        getter()
    }
}

fn go_reflect_tag_get(raw: &str, key: &str) -> String {
    let prefix = format!("{}:\"", key);
    let Some(start) = raw.find(&prefix) else {
        return String::new();
    };
    let rest = &raw[start + prefix.len()..];
    let mut value = String::new();
    let mut escaped = false;
    for ch in rest.chars() {
        if escaped {
            value.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            break;
        } else {
            value.push(ch);
        }
    }
    value
}
`)
}

// generateGoFloatStringHelper writes go_float_string, which prints a float
// as fmt's %v does: strconv's shortest %g, with an exponent when the
// exponent is below -4 or at least 6, and +Inf, -Inf and NaN. single prints
// the shortest digits of the float32 the value was widened from.
func generateGoFloatStringHelper(out *strings.Builder, complexDisplay bool) {
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	out.WriteString("\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_float_string(__value: f64, __single: bool) -> String {
    // The names are prefixed: this helper is at the crate root, where a Go
    // package variable of the same name would be a static it cannot shadow.
    if __value.is_nan() {
        return "NaN".to_string();
    }
    if __value.is_infinite() {
        return if __value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let __sign = if __value.is_sign_negative() { "-" } else { "" };
    let __shortest = if __single { format!("{:e}", __value.abs() as f32) } else { format!("{:e}", __value.abs()) };
    let (__mantissa, __exponent_text) = __shortest.split_once('e').unwrap();
    let __digits: String = __mantissa.chars().filter(|__c| __c.is_ascii_digit()).collect();
    let __exponent: i32 = __exponent_text.parse().unwrap();
    if __exponent < -4 || __exponent >= 6 {
        let mut __text = String::from(__sign);
        __text.push_str(&__digits[..1]);
        if __digits.len() > 1 {
            __text.push('.');
            __text.push_str(&__digits[1..]);
        }
        __text.push('e');
        __text.push(if __exponent < 0 { '-' } else { '+' });
        __text.push_str(&format!("{:02}", __exponent.abs()));
        return __text;
    }
    if __exponent < 0 {
        return format!("{}0.{}{}", __sign, "0".repeat((-__exponent - 1) as usize), __digits);
    }
    let __whole = (__exponent + 1) as usize;
    if __digits.len() <= __whole {
        return format!("{}{}{}", __sign, __digits, "0".repeat(__whole - __digits.len()));
    }
    format!("{}{}.{}", __sign, &__digits[..__whole], &__digits[__whole..])
}
`)
	TrackImport("Display")
	out.WriteString("\n// A value inside a slice or a map prints as fmt prints it: a float through\n")
	out.WriteString("// go_float_string, anything else through its Display. The element type may\n")
	out.WriteString("// borrow, so it is recognized by name rather than by TypeId.\n")
	out.WriteString(visibility)
	out.WriteString(`fn go_display_value<T: Display>(__value: &T) -> String {
    let __name = std::any::type_name::<T>();
    if __name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(__value as *const T as *const f64) }, false);
    }
    if __name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(__value as *const T as *const f32) } as f64, true);
    }
`)
	if complexDisplay {
		// num::Complex is #[repr(C)] with re and im, so it has the layout
		// of a two-element array. fmt prints (1-2i).
		out.WriteString(`    if __name == "num_complex::Complex<f64>" || __name == "num_complex::Complex<f32>" {
        let __single = __name.ends_with("f32>");
        let (__re, __im) = if __single {
            let __parts = unsafe { &*(__value as *const T as *const [f32; 2]) };
            (__parts[0] as f64, __parts[1] as f64)
        } else {
            let __parts = unsafe { &*(__value as *const T as *const [f64; 2]) };
            (__parts[0], __parts[1])
        };
        let __im_text = go_float_string(__im, __single);
        let __plus = if __im_text.starts_with('-') || __im_text.starts_with('+') { "" } else { "+" };
        return format!("({}{}{}i)", go_float_string(__re, __single), __plus, __im_text);
    }
`)
	}
	out.WriteString(`    __value.to_string()
}
`)
}

// goNilAnyType is the payload of a bare Box<dyn Any> that holds a nil
// interface value (see NeedGoNilAny).
const goNilAnyType = "GoNilAny"

func generateGoSliceAppendCopying(out *strings.Builder) {
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	out.WriteString(`
/// Runs an append, __store, for elements whose Go copy is not their Clone,
/// such as a struct of cells. When the append moves the slice to a new
/// backing array, the elements it carried over are replaced with __copy of
/// them, so the old array and the new one share nothing.
`)
	out.WriteString(visibility)
	out.WriteString(`trait GoSliceAppendCopying<T> {
    fn go_append_copying(&mut self, __store: impl FnOnce(&mut Self), __copy: impl Fn(&T) -> T);
}

impl<T: Clone> GoSliceAppendCopying<T> for GoSlice<T> {
    fn go_append_copying(&mut self, __store: impl FnOnce(&mut Self), __copy: impl Fn(&T) -> T) {
        let __before = self.clone();
        let __len = self.len();
        __store(self);
        if !self.shares_backing_with(&__before) {
            for __i in 0..__len.min(self.len()) {
                let __value = __copy(&self.as_slice()[__i]);
                self.as_mut_slice()[__i] = __value;
            }
        }
    }
}
`)
}

func generateGoNilAny(out *strings.Builder) {
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	out.WriteString("\n/// A nil interface value in a slot that holds a Box<dyn Any>, such as a\n")
	out.WriteString("/// []any element: it copies, prints and compares as nil.\n")
	out.WriteString("#[derive(Clone, Copy, Debug, PartialEq)]\n")
	out.WriteString(visibility)
	out.WriteString("struct GoNilAny;\n")
}

// writeNilAnyBox writes a Box<dyn Any> holding a nil interface value.
func writeNilAnyBox(out *strings.Builder) {
	NeedGoNilAny()
	TrackImport("Any")
	out.WriteString("Box::new(GoNilAny) as ")
	out.WriteString(rustAnyTraitObject())
}
