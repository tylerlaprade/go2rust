package main

import (
	"go/ast"
	"go/types"
)

// TranspileSession holds run-scoped state shared across package transpilation.
type TranspileSession struct {
	TypeInfo       *TypeInfo
	PackageMapping map[string]string // Go import path -> Rust crate name
}

// PackageState holds package-scoped registries that should be shared across files.
type PackageState struct {
	FunctionSignatures           map[string]*FunctionSignature
	FunctionNameOverrides        map[string]string
	MethodNameOverrides          map[string]string
	MethodsByType                map[string][]*ast.FuncDecl
	ErrorImplTypes               map[string]bool
	StringerImplTypes            map[string]bool
	InterfaceTypes               map[string]bool
	TypeDefinitions              map[string]string
	TypeAliases                  map[string]bool
	FunctionTypeAliases          map[string]bool
	FunctionTypeAliasBoxTypes    map[string]string
	MapKeyStructTypes            map[string]bool
	PackageConstants             map[string]string
	GoPackageImports             map[string]string
	ExternalPackages             map[string]bool
	StructDefs                   map[string]*StructDef
	EmbeddedFields               map[string]map[string]string
	AnonymousStructCounter       int
	AnonymousStructs             map[string]*ast.StructType
	AnonymousStructTypeMap       map[string]string
	ImportedInterfaceImpls       map[string]map[string]*types.Interface
	ExternalTypeStubs            map[string]bool
	ExternalTypeStubInterfaces   map[string]bool
	ExternalTypeStubIntegerTypes map[string]string
	ExternalTypeStubTupleTypes   map[string]string
	ExternalTypeStubFields       map[string]map[string]string
	ExternalTypeStubMethods      map[string]map[string]externalTypeStubMethod
	ExternalTypeStubConversions  map[string]map[string]bool
	ExternalPackageStubs         map[string]*externalPackageStub
	Helpers                      *HelperTracker
}

// FileState holds file-scoped scratch state for a single transpilation pass.
type FileState struct {
	Imports                      *ImportTracker
	Helpers                      *HelperTracker
	StatementPreprocessor        *StatementPreprocessor
	RangeLoopVars                map[string]string
	LocalRangeElemRustTypes      map[string]string
	LocalCollectionKinds         map[string]string
	LocalMapKeyRustTypes         map[string]string
	LocalMapValueRustTypes       map[string]string
	LocalConstants               map[string]string
	LocalInterfaces              map[string]bool
	CurrentReceiver              string
	CurrentReceiverType          string
	CurrentTypeMethods           []*ast.FuncDecl
	CurrentFunctionHasDefer      bool
	CurrentCaptureRenames        map[string]string
	ExternalTypeStubs            map[string]bool
	ExternalTypeStubInterfaces   map[string]bool
	ExternalTypeStubIntegerTypes map[string]string
	ExternalTypeStubTupleTypes   map[string]string
	ExternalTypeStubFields       map[string]map[string]string
	ExternalTypeStubMethods      map[string]map[string]externalTypeStubMethod
	ExternalTypeStubConversions  map[string]map[string]bool
	ExternalPackageStubs         map[string]*externalPackageStub
	PendingLoopLabel             string
	BreakTargetStack             []string
	SwitchBreakLabelCounter      int
	HasInitFunction              bool
	LabeledLoopPost              map[string]ast.Stmt
	ForPostStack                 []ast.Stmt
}

// TranspileContext holds the active session/package/file state for a transpilation call.
type TranspileContext struct {
	Session                 *TranspileSession
	Package                 *PackageState
	File                    *FileState
	Imports                 *ImportTracker
	Helpers                 *HelperTracker
	PackageMapping          map[string]string // Go import path -> Rust crate name
	UsePackageExternalStubs bool
	UsePackageHelpers       bool
}

func NewTranspileSession(typeInfo *TypeInfo, packageMapping map[string]string) *TranspileSession {
	return &TranspileSession{
		TypeInfo:       typeInfo,
		PackageMapping: packageMapping,
	}
}

func NewPackageState() *PackageState {
	return &PackageState{
		FunctionSignatures:           make(map[string]*FunctionSignature),
		FunctionNameOverrides:        make(map[string]string),
		MethodNameOverrides:          make(map[string]string),
		MethodsByType:                make(map[string][]*ast.FuncDecl),
		ErrorImplTypes:               make(map[string]bool),
		StringerImplTypes:            make(map[string]bool),
		InterfaceTypes:               make(map[string]bool),
		TypeDefinitions:              make(map[string]string),
		TypeAliases:                  make(map[string]bool),
		FunctionTypeAliases:          make(map[string]bool),
		FunctionTypeAliasBoxTypes:    make(map[string]string),
		MapKeyStructTypes:            make(map[string]bool),
		PackageConstants:             make(map[string]string),
		GoPackageImports:             make(map[string]string),
		ExternalPackages:             make(map[string]bool),
		StructDefs:                   make(map[string]*StructDef),
		EmbeddedFields:               make(map[string]map[string]string),
		AnonymousStructs:             make(map[string]*ast.StructType),
		AnonymousStructTypeMap:       make(map[string]string),
		ImportedInterfaceImpls:       make(map[string]map[string]*types.Interface),
		ExternalTypeStubs:            make(map[string]bool),
		ExternalTypeStubInterfaces:   make(map[string]bool),
		ExternalTypeStubIntegerTypes: make(map[string]string),
		ExternalTypeStubTupleTypes:   make(map[string]string),
		ExternalTypeStubFields:       make(map[string]map[string]string),
		ExternalTypeStubMethods:      make(map[string]map[string]externalTypeStubMethod),
		ExternalTypeStubConversions:  make(map[string]map[string]bool),
		ExternalPackageStubs:         make(map[string]*externalPackageStub),
		Helpers:                      &HelperTracker{},
	}
}

func NewFileState(imports *ImportTracker, helpers *HelperTracker, statementPreprocessor *StatementPreprocessor) *FileState {
	if imports == nil {
		imports = NewImportTracker()
	}
	if helpers == nil {
		helpers = &HelperTracker{}
	}
	return &FileState{
		Imports:                      imports,
		Helpers:                      helpers,
		StatementPreprocessor:        statementPreprocessor,
		RangeLoopVars:                make(map[string]string),
		LocalRangeElemRustTypes:      make(map[string]string),
		LocalCollectionKinds:         make(map[string]string),
		LocalMapKeyRustTypes:         make(map[string]string),
		LocalMapValueRustTypes:       make(map[string]string),
		LocalConstants:               make(map[string]string),
		LocalInterfaces:              make(map[string]bool),
		ExternalTypeStubs:            make(map[string]bool),
		ExternalTypeStubInterfaces:   make(map[string]bool),
		ExternalTypeStubIntegerTypes: make(map[string]string),
		ExternalTypeStubTupleTypes:   make(map[string]string),
		ExternalTypeStubFields:       make(map[string]map[string]string),
		ExternalTypeStubMethods:      make(map[string]map[string]externalTypeStubMethod),
		ExternalTypeStubConversions:  make(map[string]map[string]bool),
		ExternalPackageStubs:         make(map[string]*externalPackageStub),
		CurrentTypeMethods:           []*ast.FuncDecl{},
		LabeledLoopPost:              make(map[string]ast.Stmt),
	}
}

// Global context for the current transpilation
var currentContext *TranspileContext

// SetTranspileContext sets the global transpile context
func SetTranspileContext(ctx *TranspileContext) {
	if currentContext != nil {
		currentContext.captureCompatibilityState()
	}
	currentContext = ctx
	if currentContext != nil {
		currentContext.ensureDefaults()
		currentContext.applyCompatibilityState()
	}
}

// GetTranspileContext returns the current transpile context
func GetTranspileContext() *TranspileContext {
	return currentContext
}

func (ctx *TranspileContext) ensureDefaults() {
	if ctx.Session != nil && ctx.PackageMapping == nil {
		ctx.PackageMapping = ctx.Session.PackageMapping
	}
	if ctx.Package != nil {
		if ctx.Package.FunctionSignatures == nil {
			ctx.Package.FunctionSignatures = make(map[string]*FunctionSignature)
		}
		if ctx.Package.FunctionNameOverrides == nil {
			ctx.Package.FunctionNameOverrides = make(map[string]string)
		}
		if ctx.Package.MethodNameOverrides == nil {
			ctx.Package.MethodNameOverrides = make(map[string]string)
		}
		if ctx.Package.MethodsByType == nil {
			ctx.Package.MethodsByType = make(map[string][]*ast.FuncDecl)
		}
		if ctx.Package.ErrorImplTypes == nil {
			ctx.Package.ErrorImplTypes = make(map[string]bool)
		}
		if ctx.Package.StringerImplTypes == nil {
			ctx.Package.StringerImplTypes = make(map[string]bool)
		}
		if ctx.Package.InterfaceTypes == nil {
			ctx.Package.InterfaceTypes = make(map[string]bool)
		}
		if ctx.Package.TypeDefinitions == nil {
			ctx.Package.TypeDefinitions = make(map[string]string)
		}
		if ctx.Package.TypeAliases == nil {
			ctx.Package.TypeAliases = make(map[string]bool)
		}
		if ctx.Package.FunctionTypeAliases == nil {
			ctx.Package.FunctionTypeAliases = make(map[string]bool)
		}
		if ctx.Package.FunctionTypeAliasBoxTypes == nil {
			ctx.Package.FunctionTypeAliasBoxTypes = make(map[string]string)
		}
		if ctx.Package.MapKeyStructTypes == nil {
			ctx.Package.MapKeyStructTypes = make(map[string]bool)
		}
		if ctx.Package.GoPackageImports == nil {
			ctx.Package.GoPackageImports = make(map[string]string)
		}
		if ctx.Package.ExternalPackages == nil {
			ctx.Package.ExternalPackages = make(map[string]bool)
		}
		if ctx.Package.StructDefs == nil {
			ctx.Package.StructDefs = make(map[string]*StructDef)
		}
		if ctx.Package.EmbeddedFields == nil {
			ctx.Package.EmbeddedFields = make(map[string]map[string]string)
		}
		if ctx.Package.AnonymousStructs == nil {
			ctx.Package.AnonymousStructs = make(map[string]*ast.StructType)
		}
		if ctx.Package.AnonymousStructTypeMap == nil {
			ctx.Package.AnonymousStructTypeMap = make(map[string]string)
		}
		if ctx.Package.ImportedInterfaceImpls == nil {
			ctx.Package.ImportedInterfaceImpls = make(map[string]map[string]*types.Interface)
		}
		if ctx.Package.ExternalTypeStubs == nil {
			ctx.Package.ExternalTypeStubs = make(map[string]bool)
		}
		if ctx.Package.ExternalTypeStubInterfaces == nil {
			ctx.Package.ExternalTypeStubInterfaces = make(map[string]bool)
		}
		if ctx.Package.ExternalTypeStubIntegerTypes == nil {
			ctx.Package.ExternalTypeStubIntegerTypes = make(map[string]string)
		}
		if ctx.Package.ExternalTypeStubTupleTypes == nil {
			ctx.Package.ExternalTypeStubTupleTypes = make(map[string]string)
		}
		if ctx.Package.ExternalTypeStubFields == nil {
			ctx.Package.ExternalTypeStubFields = make(map[string]map[string]string)
		}
		if ctx.Package.ExternalTypeStubMethods == nil {
			ctx.Package.ExternalTypeStubMethods = make(map[string]map[string]externalTypeStubMethod)
		}
		if ctx.Package.ExternalTypeStubConversions == nil {
			ctx.Package.ExternalTypeStubConversions = make(map[string]map[string]bool)
		}
		if ctx.Package.ExternalPackageStubs == nil {
			ctx.Package.ExternalPackageStubs = make(map[string]*externalPackageStub)
		}
		if ctx.Package.Helpers == nil {
			ctx.Package.Helpers = &HelperTracker{}
		}
	}
	if ctx.File != nil {
		if ctx.File.Imports == nil {
			ctx.File.Imports = NewImportTracker()
		}
		if ctx.File.Helpers == nil {
			ctx.File.Helpers = &HelperTracker{}
		}
		if ctx.File.RangeLoopVars == nil {
			ctx.File.RangeLoopVars = make(map[string]string)
		}
		if ctx.File.LocalRangeElemRustTypes == nil {
			ctx.File.LocalRangeElemRustTypes = make(map[string]string)
		}
		if ctx.File.LocalCollectionKinds == nil {
			ctx.File.LocalCollectionKinds = make(map[string]string)
		}
		if ctx.File.LocalMapKeyRustTypes == nil {
			ctx.File.LocalMapKeyRustTypes = make(map[string]string)
		}
		if ctx.File.LocalMapValueRustTypes == nil {
			ctx.File.LocalMapValueRustTypes = make(map[string]string)
		}
		if ctx.File.LocalConstants == nil {
			ctx.File.LocalConstants = make(map[string]string)
		}
		if ctx.File.LocalInterfaces == nil {
			ctx.File.LocalInterfaces = make(map[string]bool)
		}
		if ctx.File.ExternalTypeStubs == nil {
			ctx.File.ExternalTypeStubs = make(map[string]bool)
		}
		if ctx.File.ExternalTypeStubInterfaces == nil {
			ctx.File.ExternalTypeStubInterfaces = make(map[string]bool)
		}
		if ctx.File.ExternalTypeStubIntegerTypes == nil {
			ctx.File.ExternalTypeStubIntegerTypes = make(map[string]string)
		}
		if ctx.File.ExternalTypeStubTupleTypes == nil {
			ctx.File.ExternalTypeStubTupleTypes = make(map[string]string)
		}
		if ctx.File.ExternalTypeStubFields == nil {
			ctx.File.ExternalTypeStubFields = make(map[string]map[string]string)
		}
		if ctx.File.ExternalTypeStubMethods == nil {
			ctx.File.ExternalTypeStubMethods = make(map[string]map[string]externalTypeStubMethod)
		}
		if ctx.File.ExternalTypeStubConversions == nil {
			ctx.File.ExternalTypeStubConversions = make(map[string]map[string]bool)
		}
		if ctx.File.ExternalPackageStubs == nil {
			ctx.File.ExternalPackageStubs = make(map[string]*externalPackageStub)
		}
		if ctx.File.LabeledLoopPost == nil {
			ctx.File.LabeledLoopPost = make(map[string]ast.Stmt)
		}
		ctx.Imports = ctx.File.Imports
		ctx.Helpers = ctx.File.Helpers
	}
}

func (ctx *TranspileContext) captureCompatibilityState() {
	if ctx.Session != nil {
		ctx.Session.TypeInfo = currentTypeInfo
		if ctx.PackageMapping != nil {
			ctx.Session.PackageMapping = ctx.PackageMapping
		}
	}
	if ctx.Package != nil {
		ctx.Package.FunctionSignatures = functionSignatures
		ctx.Package.FunctionNameOverrides = packageFunctionNameOverrides
		ctx.Package.MethodNameOverrides = packageMethodNameOverrides
		ctx.Package.ErrorImplTypes = errorImplTypes
		ctx.Package.StringerImplTypes = stringerImplTypes
		ctx.Package.InterfaceTypes = interfaceTypes
		ctx.Package.TypeDefinitions = typeDefinitions
		ctx.Package.TypeAliases = typeAliases
		ctx.Package.FunctionTypeAliases = functionTypeAliases
		ctx.Package.FunctionTypeAliasBoxTypes = functionTypeAliasBoxTypes
		if ctx.Package.MapKeyStructTypes == nil {
			ctx.Package.MapKeyStructTypes = make(map[string]bool)
		}
		ctx.Package.PackageConstants = packageConstants
		ctx.Package.GoPackageImports = goPackageImports
		ctx.Package.ExternalPackages = externalPackages
		ctx.Package.StructDefs = structDefs
		ctx.Package.EmbeddedFields = embeddedFields
		ctx.Package.AnonymousStructCounter = anonymousStructCounter
		ctx.Package.AnonymousStructs = anonymousStructs
		ctx.Package.AnonymousStructTypeMap = anonymousStructTypeMap
	}
	if ctx.File != nil {
		ctx.File.Imports = ctx.Imports
		ctx.File.Helpers = ctx.Helpers
		ctx.File.StatementPreprocessor = statementPreprocessor
		ctx.File.RangeLoopVars = rangeLoopVars
		ctx.File.LocalRangeElemRustTypes = localRangeElemRustTypes
		ctx.File.LocalCollectionKinds = localCollectionKinds
		ctx.File.LocalMapKeyRustTypes = localMapKeyRustTypes
		ctx.File.LocalMapValueRustTypes = localMapValueRustTypes
		ctx.File.LocalConstants = localConstants
		ctx.File.LocalInterfaces = localInterfaces
		ctx.File.CurrentReceiver = currentReceiver
		ctx.File.CurrentReceiverType = currentReceiverType
		ctx.File.CurrentTypeMethods = currentTypeMethods
		ctx.File.CurrentFunctionHasDefer = currentFunctionHasDefer
		ctx.File.CurrentCaptureRenames = currentCaptureRenames
		ctx.File.ExternalTypeStubs = externalTypeStubs
		ctx.File.ExternalTypeStubInterfaces = externalTypeStubInterfaces
		ctx.File.ExternalTypeStubIntegerTypes = externalTypeStubIntegerTypes
		ctx.File.ExternalTypeStubTupleTypes = externalTypeStubTupleTypes
		ctx.File.ExternalTypeStubFields = externalTypeStubFields
		ctx.File.ExternalTypeStubMethods = externalTypeStubMethods
		ctx.File.ExternalTypeStubConversions = externalTypeStubConversions
		ctx.File.ExternalPackageStubs = externalPackageStubs
		ctx.File.PendingLoopLabel = pendingLoopLabel
		ctx.File.BreakTargetStack = breakTargetStack
		ctx.File.SwitchBreakLabelCounter = switchBreakLabelCounter
		ctx.File.HasInitFunction = hasInitFunction
		ctx.File.LabeledLoopPost = labeledLoopPost
		ctx.File.ForPostStack = forPostStack
	}
}

func (ctx *TranspileContext) applyCompatibilityState() {
	if ctx.Session != nil {
		currentTypeInfo = ctx.Session.TypeInfo
		if ctx.PackageMapping == nil {
			ctx.PackageMapping = ctx.Session.PackageMapping
		}
	}
	if ctx.Package != nil {
		functionSignatures = ctx.Package.FunctionSignatures
		packageFunctionNameOverrides = ctx.Package.FunctionNameOverrides
		packageMethodNameOverrides = ctx.Package.MethodNameOverrides
		errorImplTypes = ctx.Package.ErrorImplTypes
		stringerImplTypes = ctx.Package.StringerImplTypes
		interfaceTypes = ctx.Package.InterfaceTypes
		typeDefinitions = ctx.Package.TypeDefinitions
		typeAliases = ctx.Package.TypeAliases
		functionTypeAliases = ctx.Package.FunctionTypeAliases
		functionTypeAliasBoxTypes = ctx.Package.FunctionTypeAliasBoxTypes
		packageConstants = ctx.Package.PackageConstants
		goPackageImports = ctx.Package.GoPackageImports
		externalPackages = ctx.Package.ExternalPackages
		structDefs = ctx.Package.StructDefs
		embeddedFields = ctx.Package.EmbeddedFields
		anonymousStructCounter = ctx.Package.AnonymousStructCounter
		anonymousStructs = ctx.Package.AnonymousStructs
		anonymousStructTypeMap = ctx.Package.AnonymousStructTypeMap
	}
	if ctx.File != nil {
		ctx.Imports = ctx.File.Imports
		ctx.Helpers = ctx.File.Helpers
		statementPreprocessor = ctx.File.StatementPreprocessor
		rangeLoopVars = ctx.File.RangeLoopVars
		localRangeElemRustTypes = ctx.File.LocalRangeElemRustTypes
		localCollectionKinds = ctx.File.LocalCollectionKinds
		localMapKeyRustTypes = ctx.File.LocalMapKeyRustTypes
		localMapValueRustTypes = ctx.File.LocalMapValueRustTypes
		localConstants = ctx.File.LocalConstants
		localInterfaces = ctx.File.LocalInterfaces
		currentReceiver = ctx.File.CurrentReceiver
		currentReceiverType = ctx.File.CurrentReceiverType
		currentTypeMethods = ctx.File.CurrentTypeMethods
		if currentTypeMethods == nil {
			currentTypeMethods = []*ast.FuncDecl{}
		}
		currentFunctionHasDefer = ctx.File.CurrentFunctionHasDefer
		currentCaptureRenames = ctx.File.CurrentCaptureRenames
		externalTypeStubs = ctx.File.ExternalTypeStubs
		externalTypeStubInterfaces = ctx.File.ExternalTypeStubInterfaces
		externalTypeStubIntegerTypes = ctx.File.ExternalTypeStubIntegerTypes
		externalTypeStubTupleTypes = ctx.File.ExternalTypeStubTupleTypes
		externalTypeStubFields = ctx.File.ExternalTypeStubFields
		externalTypeStubMethods = ctx.File.ExternalTypeStubMethods
		externalTypeStubConversions = ctx.File.ExternalTypeStubConversions
		externalPackageStubs = ctx.File.ExternalPackageStubs
		pendingLoopLabel = ctx.File.PendingLoopLabel
		breakTargetStack = ctx.File.BreakTargetStack
		switchBreakLabelCounter = ctx.File.SwitchBreakLabelCounter
		hasInitFunction = ctx.File.HasInitFunction
		labeledLoopPost = ctx.File.LabeledLoopPost
		forPostStack = ctx.File.ForPostStack
	}
}

// TrackImport adds an import to the current context if available
func TrackImport(importName string) {
	if currentContext != nil && currentContext.Imports != nil {
		currentContext.Imports.Add(importName)
	}
}

func activeHelperTracker() *HelperTracker {
	if currentContext == nil {
		return nil
	}
	if currentContext.UsePackageHelpers && currentContext.Package != nil {
		if currentContext.Package.Helpers == nil {
			currentContext.Package.Helpers = &HelperTracker{}
		}
		return currentContext.Package.Helpers
	}
	return currentContext.Helpers
}

func markSharedStdlibHelper(update func(*HelperTracker)) {
	if currentContext == nil || !currentContext.UsePackageExternalStubs || currentContext.Package == nil {
		return
	}
	if currentContext.Package.Helpers == nil {
		currentContext.Package.Helpers = &HelperTracker{}
	}
	update(currentContext.Package.Helpers)
}

// NeedFormatMap marks that we need the format_map helper
func NeedFormatMap() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsFormatMap = true
	}
}

// NeedFormatSlice marks that we need the format_slice helper
func NeedFormatSlice() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsFormatSlice = true
	}
}

func NeedFormatSliceWrappedValues() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsFormatSlice = true
		helpers.needsFormatSliceWrappedValues = true
	}
}

func NeedFormatSliceWrappedStringer() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsFormatSlice = true
		helpers.needsFormatSliceWrappedStringer = true
	}
}

// NeedFormatNestedSlice marks that we need the format_nested_slice helper
func NeedFormatNestedSlice() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsFormatSlice = true
		helpers.needsFormatNestedSlice = true
	}
}

// NeedFormatAny marks that we need the format_any helper
func NeedFormatAny() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsFormatAny = true
		// Also track the Any import that the helper will need
		TrackImport("Any")
	}
}

// NeedFormatAnySlice marks that we need the format_any_slice helper
func NeedFormatAnySlice() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsFormatAnySlice = true
		// Also need the regular format_any helper
		helpers.needsFormatAny = true
		// Track the Any import that the helpers will need
		TrackImport("Any")
	}
}

func NeedGoPtrKey() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoPtrKey = true
	}
}

// NeedGoChannel marks that we need the GoChannel helper struct
func NeedGoChannel() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoChannel = true
	}
	markSharedStdlibHelper(func(helpers *HelperTracker) {
		helpers.needsGoChannel = true
	})
}

// NeedWaitGroup marks that we need the WaitGroup helper struct
func NeedWaitGroup() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsWaitGroup = true
	}
}

// NeedGoMutex marks that we need the GoMutex helper struct
func NeedGoMutex() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoMutex = true
	}
}

// NeedGoOnce marks that we need the GoOnce helper struct
func NeedGoOnce() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoOnce = true
	}
}

// NeedGoTypeName marks that we need the go_type_name helper function
func NeedGoTypeName() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoTypeName = true
		TrackImport("Any")
	}
}

// NeedBase64 marks that we need the base64 helper functions
func NeedBase64() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsBase64 = true
	}
}

// NeedSha256 marks that we need the SHA-256 helper function
func NeedSha256() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsSha256 = true
	}
}

// NeedHexFormat marks that we need byte-slice hexadecimal formatting
func NeedHexFormat() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsHexFormat = true
	}
}

// NeedStrconvFormat marks that we need strconv formatting helpers
func NeedStrconvFormat() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsStrconvFormat = true
	}
}

// NeedUrl marks that we need URL parsing helpers
func NeedUrl() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsUrl = true
	}
}

// NeedRegexp marks that we need regexp helpers
func NeedRegexp() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsRegexp = true
	}
}

// NeedJsonEscape marks that we need JSON string escaping helpers
func NeedJsonEscape() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsJsonEscape = true
	}
}

// NeedOsFile marks that we need OS file helpers
func NeedOsFile() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsOsFile = true
	}
}

// NeedSliceElemPtr marks that we need slice element pointer helpers.
func NeedSliceElemPtr() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsSliceElemPtr = true
	}
}

// NeedGoTime marks that we need time.Time helpers
func NeedGoTime() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoTime = true
	}
}

// NeedGoTimer marks that we need time.Timer helpers
func NeedGoTimer() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoTimer = true
		helpers.needsGoChannel = true
		helpers.needsGoTime = true
	}
}

// NeedGoAfter marks that we need time.After helpers
func NeedGoAfter() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoAfter = true
		helpers.needsGoChannel = true
		helpers.needsGoTime = true
	}
}

// NeedGoTicker marks that we need time.Ticker helpers
func NeedGoTicker() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoTicker = true
		helpers.needsGoChannel = true
		helpers.needsGoTime = true
	}
}

// NeedGoTick marks that we need time.Tick helpers
func NeedGoTick() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoTick = true
		helpers.needsGoChannel = true
		helpers.needsGoTime = true
	}
}

// NeedGoContext marks that we need context.Context helpers
func NeedGoContext() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoContext = true
		helpers.needsGoChannel = true
	}
	markSharedStdlibHelper(func(helpers *HelperTracker) {
		helpers.needsGoContext = true
		helpers.needsGoChannel = true
	})
}

// NeedGoRand marks that we need math/rand helpers
func NeedGoRand() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoRand = true
	}
}

// NeedReflect marks that we need reflection metadata helpers
func NeedReflect() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsReflect = true
	}
}

// NeedGoHttpResponse marks that we need minimal HTTP response/body helpers.
func NeedGoHttpResponse() {
	if helpers := activeHelperTracker(); helpers != nil {
		helpers.needsGoHttpResponse = true
	}
}
