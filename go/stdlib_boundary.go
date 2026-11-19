package main

import (
	"fmt"
	"go/ast"
	"path/filepath"
	"sort"
	"strings"
)

// The stdlib boundary decides how each Go standard-library package reaches
// generated Rust.
//
// Most of the standard library is ordinary Go and is transpiled from source
// like any user package. A small bottom layer is not ordinary Go: its
// behavior lives in assembly, in compiler intrinsics, or in functions the Go
// runtime supplies at link time. Source-transpiling that layer cannot work,
// because the Go source does not contain the behavior.
//
// docs/stdlib_boundary.md states the rule and the reasoning. This file is the
// single source of truth for which package is in which tier.

// stdlibTier says how go2rust provides a standard-library package.
type stdlibTier int

const (
	// stdlibTierSource is ordinary Go. Transpile it from source. Never
	// hand-write Rust for it.
	stdlibTierSource stdlibTier = iota
	// stdlibTierSourceWithHooks is ordinary Go apart from a few named
	// functions that are declared without a body. Transpile the package from
	// source and resolve only those functions by hand.
	stdlibTierSourceWithHooks
	// stdlibTierNative is bound to the Go runtime, the compiler, or the
	// operating system. Provide it in hand-written Rust. Never
	// source-transpile it.
	stdlibTierNative
)

func (t stdlibTier) String() string {
	switch t {
	case stdlibTierSource:
		return "source"
	case stdlibTierSourceWithHooks:
		return "source+hooks"
	case stdlibTierNative:
		return "native"
	}
	return fmt.Sprintf("stdlibTier(%d)", int(t))
}

// stdlibRuntimeSignals are the mechanical facts that show a package is not
// ordinary Go under the current build configuration.
type stdlibRuntimeSignals struct {
	// AsmFiles are the assembly files the go tool selects for the package.
	AsmFiles []string
	// BodylessFuncs are functions declared in Go without a body. Their
	// implementation comes from assembly, from the compiler, or from another
	// package through //go:linkname. A method is recorded as "Recv.Name", so
	// it can never be mistaken for a package-level function of the same name.
	BodylessFuncs []string
	// UsesCgo is set when a file imports "C".
	UsesCgo bool
}

func (s stdlibRuntimeSignals) runtimeBound() bool {
	return len(s.AsmFiles) > 0 || len(s.BodylessFuncs) > 0 || s.UsesCgo
}

func (s stdlibRuntimeSignals) String() string {
	if !s.runtimeBound() {
		return "no runtime signals"
	}
	var parts []string
	if len(s.AsmFiles) > 0 {
		parts = append(parts, fmt.Sprintf("%d assembly file(s)", len(s.AsmFiles)))
	}
	if s.UsesCgo {
		parts = append(parts, "cgo")
	}
	if len(s.BodylessFuncs) > 0 {
		parts = append(parts, "bodyless funcs: "+strings.Join(s.BodylessFuncs, ", "))
	}
	return strings.Join(parts, "; ")
}

// collectStdlibRuntimeSignals derives the signals for one package. otherFiles
// is the package's non-Go file list (packages.Package.OtherFiles, or SFiles
// from `go list`) and files is its parsed Go syntax.
func collectStdlibRuntimeSignals(otherFiles []string, files []*ast.File) stdlibRuntimeSignals {
	var signals stdlibRuntimeSignals
	for _, name := range otherFiles {
		ext := filepath.Ext(name)
		if ext == ".s" || ext == ".S" {
			signals.AsmFiles = append(signals.AsmFiles, filepath.Base(name))
		}
	}
	for _, file := range files {
		if file == nil {
			continue
		}
		for _, spec := range file.Imports {
			if spec.Path != nil && spec.Path.Value == `"C"` {
				signals.UsesCgo = true
			}
		}
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Body != nil || fn.Name == nil {
				continue
			}
			signals.BodylessFuncs = append(signals.BodylessFuncs, stdlibBoundaryFuncName(fn))
		}
	}
	sort.Strings(signals.AsmFiles)
	sort.Strings(signals.BodylessFuncs)
	return signals
}

// stdlibBoundaryFuncName names a declaration the way hook lists do: "Name"
// for a package-level function and "Recv.Name" for a method.
func stdlibBoundaryFuncName(fn *ast.FuncDecl) string {
	if fn.Recv == nil || len(fn.Recv.List) == 0 {
		return fn.Name.Name
	}
	recv := fn.Recv.List[0].Type
	for {
		switch t := recv.(type) {
		case *ast.StarExpr:
			recv = t.X
			continue
		case *ast.ParenExpr:
			recv = t.X
			continue
		case *ast.IndexExpr:
			recv = t.X
			continue
		case *ast.IndexListExpr:
			recv = t.X
			continue
		case *ast.Ident:
			return t.Name + "." + fn.Name.Name
		}
		return "?." + fn.Name.Name
	}
}

// stdlibBoundaryDecision is a reviewed decision for one package.
type stdlibBoundaryDecision struct {
	tier stdlibTier
	// hooks lists, by exact name, the bodyless functions a source+hooks
	// package may declare. A bodyless function outside this list is an error,
	// so a Go upgrade cannot add a runtime dependency unnoticed.
	hooks []string
	// asm records that the package ships assembly accelerators behind its
	// hooks. Without it, an assembly file in a source+hooks package is an
	// error.
	asm bool
	// reason is why the package sits in this tier.
	reason string
}

// stdlibBoundaryPolicy holds the decisions that signals alone cannot make.
//
// Every public package with runtime signals needs an entry. Internal packages
// with runtime signals default to native and need an entry only to override
// that. Packages without signals are source and need no entry.
//
// Entries cover Go 1.24 through 1.27 with -tags=purego on linux, darwin and
// windows, for amd64 and arm64. Hook names are exact, so another architecture
// or a newer toolchain can add names that need review. TestStdlibBoundaryCoversSelfHostClosure reports what a newer
// toolchain adds.
var stdlibBoundaryPolicy = map[string]stdlibBoundaryDecision{
	// Native: the package is the runtime or OS interface itself.
	"net":       {tier: stdlibTierNative, reason: "OS networking on internal/poll and syscall, with per-platform sources"},
	"os/signal": {tier: stdlibTierNative, reason: "runtime signal delivery"},
	"os/user":   {tier: stdlibTierNative, reason: "OS user database, through cgo on some platforms"},
	"plugin":    {tier: stdlibTierNative, reason: "runtime module loading"},
	"reflect":   {tier: stdlibTierNative, reason: "reads the runtime's type metadata and memory layout"},

	// Native: packages under runtime/ that are part of the runtime. Some show
	// no signals in an ordinary build, because their files are behind a build
	// tag or an experiment; they are still the runtime's. The packages under
	// runtime/ that are ordinary Go around a few runtime calls are source
	// with hooks, below.
	"runtime/asan":   {tier: stdlibTierNative, reason: "address sanitizer runtime, through cgo"},
	"runtime/cgo":    {tier: stdlibTierNative, reason: "the runtime's C call bridge"},
	"runtime/msan":   {tier: stdlibTierNative, reason: "memory sanitizer runtime, through cgo"},
	"runtime/pprof":  {tier: stdlibTierNative, reason: "reads the runtime's profile buffers through more than a dozen runtime functions, a set that changes between releases"},
	"runtime/race":   {tier: stdlibTierNative, reason: "the race detector runtime"},
	"runtime/secret": {tier: stdlibTierNative, reason: "erases registers and stack through the runtime (Go 1.26+, behind an experiment)"},
	"runtime/trace":  {tier: stdlibTierNative, reason: "writes to the runtime's execution tracer"},

	"sync":        {tier: stdlibTierNative, reason: "runtime semaphores, scheduler pinning and notify lists"},
	"sync/atomic": {tier: stdlibTierNative, reason: "compiler intrinsics with assembly bodies"},
	"syscall":     {tier: stdlibTierNative, reason: "raw OS interface: assembly trampolines and per-platform sources"},
	"unique":      {tier: stdlibTierNative, reason: "garbage-collector cleanup hooks"},
	"weak":        {tier: stdlibTierNative, reason: "garbage-collector weak pointers"},

	// Source with hooks: ordinary Go apart from the named declarations.
	"crypto/fips140":              {tier: stdlibTierSourceWithHooks, hooks: []string{"isBypassed", "setBypass", "unsetBypass"}, reason: "bypass flag supplied by the runtime (Go 1.26+)"},
	"crypto/internal/fips140hash": {tier: stdlibTierSourceWithHooks, hooks: []string{"sha3Unwrap"}, reason: "one function implemented in crypto/sha3"},
	"crypto/rand":                 {tier: stdlibTierSourceWithHooks, hooks: []string{"fatal"}, reason: "runtime fatal error"},
	"crypto/subtle":               {tier: stdlibTierSourceWithHooks, hooks: []string{"setDITDisabled", "setDITEnabled"}, reason: "data-independent-timing switch supplied by the runtime (Go 1.26+)"},
	"go/internal/srcimporter":     {tier: stdlibTierSourceWithHooks, hooks: []string{"setUsesCgo"}, reason: "one function implemented in go/types"},
	"go/types":                    {tier: stdlibTierSourceWithHooks, hooks: []string{"badlinkname_Checker_infer"}, reason: "linkname declarations kept for outside callers; never called"},
	"hash/crc32":                  {tier: stdlibTierSourceWithHooks, hooks: []string{"castagnoliSSE42", "castagnoliSSE42Triple", "castagnoliUpdate", "ieeeCLMUL", "ieeeUpdate"}, asm: true, reason: "CPU accelerators; the generic Go path is complete"},
	"hash/maphash":                {tier: stdlibTierSourceWithHooks, hooks: []string{"runtime_memhash", "runtime_rand"}, reason: "runtime hash and random seed (Go 1.27+)"},
	"iter":                        {tier: stdlibTierSourceWithHooks, hooks: []string{"coroswitch", "newcoro"}, reason: "runtime coroutines behind iter.Pull"},
	"maps":                        {tier: stdlibTierSourceWithHooks, hooks: []string{"clone"}, reason: "runtime map clone"},
	"math":                        {tier: stdlibTierSourceWithHooks, hooks: []string{"archCeil", "archExp", "archExp2", "archFloor", "archHypot", "archLog", "archMax", "archMin", "archModf", "archTrunc"}, asm: true, reason: "CPU accelerators; the generic Go path is complete"},
	"math/big":                    {tier: stdlibTierSourceWithHooks, hooks: []string{"addMulVVW", "addMulVVWW", "addVV", "addVW", "lshVU", "mulAddVWW", "rshVU", "shlVU", "shrVU", "subVV", "subVW"}, asm: true, reason: "arithmetic kernels; -tags=math_big_pure_go selects the Go versions and removes every hook"},
	"math/rand":                   {tier: stdlibTierSourceWithHooks, hooks: []string{"runtime_rand"}, reason: "runtime random seed"},
	"math/rand/v2":                {tier: stdlibTierSourceWithHooks, hooks: []string{"runtime_rand"}, reason: "runtime random seed"},
	"mime/multipart":              {tier: stdlibTierSourceWithHooks, hooks: []string{"readMIMEHeader"}, reason: "one function implemented in net/textproto"},
	"net/http":                    {tier: stdlibTierSourceWithHooks, hooks: []string{"badRoundTrip", "badServeHTTP", "readMIMEHeader"}, reason: "linkname declarations kept for outside callers, never called; one function implemented in net/textproto (Go 1.27+)"},
	"net/url":                     {tier: stdlibTierSourceWithHooks, hooks: []string{"badSetPath"}, reason: "linkname declarations kept for outside callers; never called"},
	"os":                          {tier: stdlibTierSourceWithHooks, hooks: []string{"checkClonePidfd", "closedir", "ignoreSIGSYS", "readdir_r", "restoreSIGSYS", "runtime_args", "runtime_beforeExit", "runtime_rand", "sigpipe"}, reason: "arguments, exit and a random seed come from the runtime, and a few libc and pidfd calls; the rest is ordinary Go on top of syscall and internal/poll, which are native"},
	"runtime/debug":               {tier: stdlibTierSourceWithHooks, hooks: []string{"SetTraceback", "WriteHeapDump", "freeOSMemory", "modinfo", "readGCStats", "runtime_setCrashFD", "setGCPercent", "setMaxStack", "setMaxThreads", "setMemoryLimit", "setPanicOnFault"}, asm: true, reason: "garbage-collector and crash controls come from the runtime; build-info parsing and stack formatting are ordinary Go. The assembly file is empty and only lets the declarations compile"},
	"runtime/metrics":             {tier: stdlibTierSourceWithHooks, hooks: []string{"runtime_readMetrics"}, reason: "one function reads the runtime's counters; the descriptions and value types are ordinary Go"},
	"testing/cryptotest":          {tier: stdlibTierSourceWithHooks, hooks: []string{"randSetTestingReader", "testingCheckParallel"}, reason: "functions implemented in crypto/internal/rand and testing (Go 1.26+)"},
	"testing/synctest":            {tier: stdlibTierSourceWithHooks, hooks: []string{"testingSynctestTest"}, reason: "one function implemented in testing (Go 1.25+)"},
	"time":                        {tier: stdlibTierSourceWithHooks, hooks: []string{"Sleep", "newTimer", "now", "resetTimer", "runtimeIsBubbled", "runtimeNano", "runtimeNow", "stopTimer"}, reason: "runtime clock and timers; formatting, parsing and arithmetic are ordinary Go"},
	"time/tzdata":                 {tier: stdlibTierSourceWithHooks, hooks: []string{"registerLoadFromEmbeddedTZData"}, reason: "one function implemented in time"},
}

// isRuntimeImplementationStdlibPath reports packages that are the Go runtime
// or the compiler's own surface. They are native whatever their signals say.
//
// A package is not the runtime merely because its path starts with runtime/.
// runtime/debug.ParseBuildInfo is an ordinary parser, in the same file as
// modinfo, which only the runtime can answer. Public packages under runtime/
// are decided one by one in stdlibBoundaryPolicy.
func isRuntimeImplementationStdlibPath(importPath string) bool {
	switch importPath {
	case "runtime", "unsafe":
		return true
	}
	return strings.HasPrefix(importPath, "runtime/internal/") || strings.HasPrefix(importPath, "internal/runtime/")
}

// isInternalStdlibPath reports implementation-detail packages: anything under
// an internal directory, and the standard library's vendored dependencies.
func isInternalStdlibPath(importPath string) bool {
	if strings.HasPrefix(importPath, "vendor/") {
		return true
	}
	for _, element := range strings.Split(importPath, "/") {
		if element == "internal" {
			return true
		}
	}
	return false
}

// stdlibBoundaryTier returns the tier of a standard-library package, given
// the signals observed for the current build configuration.
//
// It returns an error when the signals contradict the policy. That means a
// person has to decide: a public package has a runtime dependency nobody
// reviewed, or a source+hooks package declares a hook the policy does not
// list. The returned tier is still the safest reading (native, or the
// policy's tier), so callers that only report can keep going.
func stdlibBoundaryTier(importPath string, signals stdlibRuntimeSignals) (stdlibTier, error) {
	if isRuntimeImplementationStdlibPath(importPath) {
		return stdlibTierNative, nil
	}
	if decision, ok := stdlibBoundaryPolicy[importPath]; ok {
		if decision.tier != stdlibTierSourceWithHooks {
			return decision.tier, nil
		}
		if !signals.runtimeBound() {
			// Every hook is gone under this build configuration.
			return stdlibTierSource, nil
		}
		return decision.tier, checkStdlibBoundaryHooks(importPath, decision, signals)
	}
	if !signals.runtimeBound() {
		return stdlibTierSource, nil
	}
	if isInternalStdlibPath(importPath) {
		return stdlibTierNative, nil
	}
	return stdlibTierNative, fmt.Errorf("stdlib package %q is runtime-bound (%s) but has no boundary decision; add it to stdlibBoundaryPolicy in go/stdlib_boundary.go", importPath, signals)
}

func checkStdlibBoundaryHooks(importPath string, decision stdlibBoundaryDecision, signals stdlibRuntimeSignals) error {
	if signals.UsesCgo {
		return fmt.Errorf("stdlib package %q is source+hooks but uses cgo; review its entry in stdlibBoundaryPolicy", importPath)
	}
	if len(signals.AsmFiles) > 0 && !decision.asm {
		return fmt.Errorf("stdlib package %q is source+hooks but has assembly files (%s); review its entry in stdlibBoundaryPolicy", importPath, strings.Join(signals.AsmFiles, ", "))
	}
	var unlisted []string
	for _, name := range signals.BodylessFuncs {
		if !stdlibBoundaryHookListed(decision.hooks, name) {
			unlisted = append(unlisted, name)
		}
	}
	if len(unlisted) > 0 {
		return fmt.Errorf("stdlib package %q declares bodyless funcs outside its hook list (%s); review its entry in stdlibBoundaryPolicy", importPath, strings.Join(unlisted, ", "))
	}
	return nil
}

func stdlibBoundaryHookListed(hooks []string, name string) bool {
	for _, hook := range hooks {
		if hook == name {
			return true
		}
	}
	return false
}

// stdlibBoundaryReason explains a tier for reports and error messages.
func stdlibBoundaryReason(importPath string, tier stdlibTier) string {
	if isRuntimeImplementationStdlibPath(importPath) {
		return "the Go runtime or the compiler's own surface"
	}
	if decision, ok := stdlibBoundaryPolicy[importPath]; ok && decision.tier == tier {
		return decision.reason
	}
	switch tier {
	case stdlibTierNative:
		return "internal package with assembly or runtime-supplied functions"
	case stdlibTierSource:
		return "ordinary Go"
	}
	return ""
}
