package main

import "go/ast"

// TranspileSession holds run-scoped state shared across package transpilation.
type TranspileSession struct {
	TypeInfo       *TypeInfo
	PackageMapping map[string]string // Go import path -> Rust crate name
}

// PackageState holds package-scoped registries that should be shared across files.
type PackageState struct {
	FunctionSignatures map[string]*FunctionSignature
	ErrorImplTypes     map[string]bool
	StringerImplTypes  map[string]bool
	InterfaceTypes     map[string]bool
	TypeDefinitions    map[string]string
	TypeAliases        map[string]bool
	GoPackageImports   map[string]string
	ExternalPackages   map[string]bool
	StructDefs         map[string]*StructDef
	EmbeddedFields     map[string]map[string]string
}

// FileState holds file-scoped scratch state for a single transpilation pass.
type FileState struct {
	Imports                 *ImportTracker
	Helpers                 *HelperTracker
	StatementPreprocessor   *StatementPreprocessor
	RangeLoopVars           map[string]string
	LocalConstants          map[string]string
	LocalInterfaces         map[string]bool
	CurrentReceiver         string
	CurrentReceiverType     string
	CurrentFunctionHasDefer bool
	CurrentCaptureRenames   map[string]string
	PendingLoopLabel        string
	HasInitFunction         bool
	LabeledLoopPost         map[string]ast.Stmt
}

// TranspileContext holds the active session/package/file state for a transpilation call.
type TranspileContext struct {
	Session        *TranspileSession
	Package        *PackageState
	File           *FileState
	Imports        *ImportTracker
	Helpers        *HelperTracker
	PackageMapping map[string]string // Go import path -> Rust crate name
}

func NewTranspileSession(typeInfo *TypeInfo, packageMapping map[string]string) *TranspileSession {
	return &TranspileSession{
		TypeInfo:       typeInfo,
		PackageMapping: packageMapping,
	}
}

func NewPackageState() *PackageState {
	return &PackageState{
		FunctionSignatures: make(map[string]*FunctionSignature),
		ErrorImplTypes:     make(map[string]bool),
		StringerImplTypes:  make(map[string]bool),
		InterfaceTypes:     make(map[string]bool),
		TypeDefinitions:    make(map[string]string),
		TypeAliases:        make(map[string]bool),
		GoPackageImports:   make(map[string]string),
		ExternalPackages:   make(map[string]bool),
		StructDefs:         make(map[string]*StructDef),
		EmbeddedFields:     make(map[string]map[string]string),
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
		Imports:               imports,
		Helpers:               helpers,
		StatementPreprocessor: statementPreprocessor,
		RangeLoopVars:         make(map[string]string),
		LocalConstants:        make(map[string]string),
		LocalInterfaces:       make(map[string]bool),
		LabeledLoopPost:       make(map[string]ast.Stmt),
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
		if ctx.File.LocalConstants == nil {
			ctx.File.LocalConstants = make(map[string]string)
		}
		if ctx.File.LocalInterfaces == nil {
			ctx.File.LocalInterfaces = make(map[string]bool)
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
		ctx.Package.ErrorImplTypes = errorImplTypes
		ctx.Package.StringerImplTypes = stringerImplTypes
		ctx.Package.InterfaceTypes = interfaceTypes
		ctx.Package.TypeDefinitions = typeDefinitions
		ctx.Package.TypeAliases = typeAliases
		ctx.Package.GoPackageImports = goPackageImports
		ctx.Package.ExternalPackages = externalPackages
		ctx.Package.StructDefs = structDefs
		ctx.Package.EmbeddedFields = embeddedFields
	}
	if ctx.File != nil {
		ctx.File.Imports = ctx.Imports
		ctx.File.Helpers = ctx.Helpers
		ctx.File.StatementPreprocessor = statementPreprocessor
		ctx.File.RangeLoopVars = rangeLoopVars
		ctx.File.LocalConstants = localConstants
		ctx.File.LocalInterfaces = localInterfaces
		ctx.File.CurrentReceiver = currentReceiver
		ctx.File.CurrentReceiverType = currentReceiverType
		ctx.File.CurrentFunctionHasDefer = currentFunctionHasDefer
		ctx.File.CurrentCaptureRenames = currentCaptureRenames
		ctx.File.PendingLoopLabel = pendingLoopLabel
		ctx.File.HasInitFunction = hasInitFunction
		ctx.File.LabeledLoopPost = labeledLoopPost
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
		errorImplTypes = ctx.Package.ErrorImplTypes
		stringerImplTypes = ctx.Package.StringerImplTypes
		interfaceTypes = ctx.Package.InterfaceTypes
		typeDefinitions = ctx.Package.TypeDefinitions
		typeAliases = ctx.Package.TypeAliases
		goPackageImports = ctx.Package.GoPackageImports
		externalPackages = ctx.Package.ExternalPackages
		structDefs = ctx.Package.StructDefs
		embeddedFields = ctx.Package.EmbeddedFields
	}
	if ctx.File != nil {
		ctx.Imports = ctx.File.Imports
		ctx.Helpers = ctx.File.Helpers
		statementPreprocessor = ctx.File.StatementPreprocessor
		rangeLoopVars = ctx.File.RangeLoopVars
		localConstants = ctx.File.LocalConstants
		localInterfaces = ctx.File.LocalInterfaces
		currentReceiver = ctx.File.CurrentReceiver
		currentReceiverType = ctx.File.CurrentReceiverType
		currentFunctionHasDefer = ctx.File.CurrentFunctionHasDefer
		currentCaptureRenames = ctx.File.CurrentCaptureRenames
		pendingLoopLabel = ctx.File.PendingLoopLabel
		hasInitFunction = ctx.File.HasInitFunction
		labeledLoopPost = ctx.File.LabeledLoopPost
	}
}

// TrackImport adds an import to the current context if available
func TrackImport(importName string) {
	if currentContext != nil && currentContext.Imports != nil {
		currentContext.Imports.Add(importName)
	}
}

// NeedFormatMap marks that we need the format_map helper
func NeedFormatMap() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsFormatMap = true
	}
}

// NeedFormatSlice marks that we need the format_slice helper
func NeedFormatSlice() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsFormatSlice = true
	}
}

// NeedFormatAny marks that we need the format_any helper
func NeedFormatAny() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsFormatAny = true
		// Also track the Any import that the helper will need
		TrackImport("Any")
	}
}

// NeedFormatAnySlice marks that we need the format_any_slice helper
func NeedFormatAnySlice() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsFormatAnySlice = true
		// Also need the regular format_any helper
		currentContext.Helpers.needsFormatAny = true
		// Track the Any import that the helpers will need
		TrackImport("Any")
	}
}

// NeedGoChannel marks that we need the GoChannel helper struct
func NeedGoChannel() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsGoChannel = true
	}
}

// NeedWaitGroup marks that we need the WaitGroup helper struct
func NeedWaitGroup() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsWaitGroup = true
	}
}

// NeedGoMutex marks that we need the GoMutex helper struct
func NeedGoMutex() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsGoMutex = true
	}
}

// NeedGoTypeName marks that we need the go_type_name helper function
func NeedGoTypeName() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsGoTypeName = true
		TrackImport("Any")
	}
}

// NeedBase64 marks that we need the base64 helper functions
func NeedBase64() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsBase64 = true
	}
}

// NeedSha256 marks that we need the SHA-256 helper function
func NeedSha256() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsSha256 = true
	}
}

// NeedHexFormat marks that we need byte-slice hexadecimal formatting
func NeedHexFormat() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsHexFormat = true
	}
}

// NeedStrconvFormat marks that we need strconv formatting helpers
func NeedStrconvFormat() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsStrconvFormat = true
	}
}

// NeedUrl marks that we need URL parsing helpers
func NeedUrl() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsUrl = true
	}
}

// NeedRegexp marks that we need regexp helpers
func NeedRegexp() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsRegexp = true
	}
}

// NeedJsonEscape marks that we need JSON string escaping helpers
func NeedJsonEscape() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsJsonEscape = true
	}
}

// NeedOsFile marks that we need OS file helpers
func NeedOsFile() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsOsFile = true
	}
}

// NeedSliceElemPtr marks that we need slice element pointer helpers.
func NeedSliceElemPtr() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsSliceElemPtr = true
	}
}

// NeedGoTime marks that we need time.Time helpers
func NeedGoTime() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsGoTime = true
	}
}

// NeedGoTimer marks that we need time.Timer helpers
func NeedGoTimer() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsGoTimer = true
		currentContext.Helpers.needsGoChannel = true
		currentContext.Helpers.needsGoTime = true
	}
}

// NeedGoAfter marks that we need time.After helpers
func NeedGoAfter() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsGoAfter = true
		currentContext.Helpers.needsGoChannel = true
		currentContext.Helpers.needsGoTime = true
	}
}

// NeedGoTicker marks that we need time.Ticker helpers
func NeedGoTicker() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsGoTicker = true
		currentContext.Helpers.needsGoChannel = true
		currentContext.Helpers.needsGoTime = true
	}
}

// NeedGoTick marks that we need time.Tick helpers
func NeedGoTick() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsGoTick = true
		currentContext.Helpers.needsGoChannel = true
		currentContext.Helpers.needsGoTime = true
	}
}

// NeedGoContext marks that we need context.Context helpers
func NeedGoContext() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsGoContext = true
		currentContext.Helpers.needsGoChannel = true
	}
}

// NeedGoRand marks that we need math/rand helpers
func NeedGoRand() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsGoRand = true
	}
}

// NeedReflect marks that we need reflection metadata helpers
func NeedReflect() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsReflect = true
	}
}

// NeedGoHttpResponse marks that we need minimal HTTP response/body helpers.
func NeedGoHttpResponse() {
	if currentContext != nil && currentContext.Helpers != nil {
		currentContext.Helpers.needsGoHttpResponse = true
	}
}
