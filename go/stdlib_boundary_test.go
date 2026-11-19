package main

import (
	"bytes"
	"encoding/json"
	"go/ast"
	"go/parser"
	"go/token"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"testing"
)

// stdlibBoundaryBuildTags matches the self-host default in
// self_transpile_check.sh (GOFLAGS=-tags=purego), so the test sees the same
// file selection the self-host pipeline does.
const stdlibBoundaryBuildTags = "purego"

func TestStdlibBoundaryTierRules(t *testing.T) {
	hook := func(names ...string) stdlibRuntimeSignals {
		return stdlibRuntimeSignals{BodylessFuncs: names}
	}
	asm := stdlibRuntimeSignals{AsmFiles: []string{"x_amd64.s"}}
	cases := []struct {
		name      string
		path      string
		signals   stdlibRuntimeSignals
		want      stdlibTier
		wantError string
	}{
		{name: "plain public package is source", path: "strconv", want: stdlibTierSource},
		{name: "plain internal package is source", path: "internal/itoa", want: stdlibTierSource},
		{name: "runtime is native without signals", path: "runtime", want: stdlibTierNative},
		{name: "runtime's own internals are native", path: "runtime/internal/atomic", want: stdlibTierNative},
		{name: "internal runtime package is native", path: "internal/runtime/maps", want: stdlibTierNative},
		{name: "a runtime/ path alone does not make a package native", path: "runtime/coverage", want: stdlibTierSource},
		{name: "runtime/debug is ordinary Go around listed runtime calls", path: "runtime/debug", signals: stdlibRuntimeSignals{AsmFiles: []string{"debug.s"}, BodylessFuncs: []string{"modinfo", "setGCPercent"}}, want: stdlibTierSourceWithHooks},
		{name: "a new runtime call in runtime/debug is an error", path: "runtime/debug", signals: hook("modinfo", "runtime_newThing"), want: stdlibTierSourceWithHooks, wantError: "outside its hook list"},
		{name: "runtime/pprof is native by decision", path: "runtime/pprof", signals: hook("readProfile"), want: stdlibTierNative},
		{name: "os is ordinary Go around listed runtime calls", path: "os", signals: hook("runtime_args", "runtime_beforeExit", "sigpipe"), want: stdlibTierSourceWithHooks},
		{name: "a new runtime call in os is an error", path: "os", signals: hook("runtime_args", "runtime_newThing"), want: stdlibTierSourceWithHooks, wantError: "outside its hook list"},
		{name: "net is ordinary Go around listed hooks", path: "net", signals: hook("newUnixFile", "runtime_rand"), want: stdlibTierSourceWithHooks},
		{name: "net's optional cgo resolver is allowed by its entry", path: "net", signals: stdlibRuntimeSignals{UsesCgo: true, BodylessFuncs: []string{"runtime_rand"}}, want: stdlibTierSourceWithHooks},
		{name: "a new hook in net is an error", path: "net", signals: hook("runtime_rand", "runtime_pollNewThing"), want: stdlibTierSourceWithHooks, wantError: "outside its hook list"},
		{name: "unsafe is native", path: "unsafe", want: stdlibTierNative},
		{name: "internal package with assembly is native", path: "internal/bytealg", signals: asm, want: stdlibTierNative},
		{name: "internal package with hooks is native", path: "internal/poll", signals: hook("runtime_pollOpen"), want: stdlibTierNative},
		{name: "vendored package with assembly is native", path: "vendor/golang.org/x/sys/cpu", signals: asm, want: stdlibTierNative},
		{name: "nested internal package with hooks is native", path: "crypto/internal/sysrand", signals: hook("fatal"), want: stdlibTierNative},
		{name: "policy native wins without signals", path: "sync", want: stdlibTierNative},
		{name: "policy native with signals", path: "sync/atomic", signals: asm, want: stdlibTierNative},
		{name: "listed hook", path: "math/rand", signals: hook("runtime_rand"), want: stdlibTierSourceWithHooks},
		{name: "hook names are exact, not prefixes", path: "go/types", signals: hook("badlinkname_Checker_other"), want: stdlibTierSourceWithHooks, wantError: "outside its hook list"},
		{name: "a method does not match a function hook", path: "math/rand", signals: hook("Rand.runtime_rand"), want: stdlibTierSourceWithHooks, wantError: "outside its hook list"},
		{name: "declaration-only hook", path: "go/types", signals: hook("badlinkname_Checker_infer"), want: stdlibTierSourceWithHooks},
		{name: "hooks gone means source", path: "math/big", want: stdlibTierSource},
		{name: "assembly allowed when policy says so", path: "math", signals: stdlibRuntimeSignals{AsmFiles: []string{"floor_asm.s"}, BodylessFuncs: []string{"archFloor"}}, want: stdlibTierSourceWithHooks},
		{name: "internal override to hooks", path: "go/internal/srcimporter", signals: hook("setUsesCgo"), want: stdlibTierSourceWithHooks},
		{name: "unlisted hook is an error", path: "go/types", signals: hook("runtime_newThing"), want: stdlibTierSourceWithHooks, wantError: "outside its hook list"},
		{name: "unexpected assembly is an error", path: "time", signals: stdlibRuntimeSignals{AsmFiles: []string{"now.s"}, BodylessFuncs: []string{"now"}}, want: stdlibTierSourceWithHooks, wantError: "assembly files"},
		{name: "cgo in a hooks package is an error", path: "time", signals: stdlibRuntimeSignals{UsesCgo: true}, want: stdlibTierSourceWithHooks, wantError: "uses cgo"},
		{name: "undecided public package is an error", path: "encoding/newthing", signals: hook("runtime_x"), want: stdlibTierNative, wantError: "no boundary decision"},
		{name: "undecided public cgo package is an error", path: "encoding/newthing", signals: stdlibRuntimeSignals{UsesCgo: true}, want: stdlibTierNative, wantError: "no boundary decision"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got, err := stdlibBoundaryTier(tc.path, tc.signals)
			if got != tc.want {
				t.Errorf("stdlibBoundaryTier(%q) = %v, want %v", tc.path, got, tc.want)
			}
			switch {
			case tc.wantError == "" && err != nil:
				t.Errorf("stdlibBoundaryTier(%q) unexpected error: %v", tc.path, err)
			case tc.wantError != "" && err == nil:
				t.Errorf("stdlibBoundaryTier(%q) = nil error, want one containing %q", tc.path, tc.wantError)
			case tc.wantError != "" && !strings.Contains(err.Error(), tc.wantError):
				t.Errorf("stdlibBoundaryTier(%q) error = %q, want it to contain %q", tc.path, err, tc.wantError)
			}
		})
	}
}

func TestCollectStdlibRuntimeSignals(t *testing.T) {
	const src = `package p

import "C"

func withBody() {}

func noBody(x int) int

func (r recv) method()

func (r *recv) ptrMethod()

func (g *generic[T]) genericMethod()

type recv struct{}

type generic[T any] struct{}
`
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "p.go", src, parser.SkipObjectResolution)
	if err != nil {
		t.Fatalf("parse: %v", err)
	}
	got := collectStdlibRuntimeSignals([]string{"/goroot/src/p/b_arm64.s", "a_amd64.s", "defs.h"}, []*ast.File{file, nil})
	if want := []string{"a_amd64.s", "b_arm64.s"}; strings.Join(got.AsmFiles, ",") != strings.Join(want, ",") {
		t.Errorf("AsmFiles = %v, want %v", got.AsmFiles, want)
	}
	if want := []string{"generic.genericMethod", "noBody", "recv.method", "recv.ptrMethod"}; strings.Join(got.BodylessFuncs, ",") != strings.Join(want, ",") {
		t.Errorf("BodylessFuncs = %v, want %v", got.BodylessFuncs, want)
	}
	if !got.UsesCgo {
		t.Errorf("UsesCgo = false, want true")
	}
	if !got.runtimeBound() {
		t.Errorf("runtimeBound() = false, want true")
	}
	if (stdlibRuntimeSignals{}).runtimeBound() {
		t.Errorf("empty signals report runtimeBound() = true")
	}
}

// Every policy entry must say why, and hook lists belong only on the
// source+hooks tier.
func TestStdlibBoundaryPolicyIsWellFormed(t *testing.T) {
	for path, decision := range stdlibBoundaryPolicy {
		if strings.TrimSpace(decision.reason) == "" {
			t.Errorf("%s: policy entry has no reason", path)
		}
		switch decision.tier {
		case stdlibTierSourceWithHooks:
			if len(decision.hooks) == 0 {
				t.Errorf("%s: source+hooks entry lists no hooks", path)
			}
		case stdlibTierNative:
			if len(decision.hooks) > 0 || decision.asm || decision.cgo {
				t.Errorf("%s: native entry must not list hooks, asm or cgo", path)
			}
		default:
			t.Errorf("%s: policy entries are for native or source+hooks packages; source needs no entry", path)
		}
		if isRuntimeImplementationStdlibPath(path) {
			t.Errorf("%s: already native by path; remove the policy entry", path)
		}
	}
}

// TestStdlibBoundaryIsDecidedPerDeclaration pins two packages where the
// package path or the file name suggests "native" and the declarations say
// otherwise. It reads the installed toolchain's source for each supported
// operating system, so a Go upgrade that moves a body in or out is caught.
//
//   - runtime/debug: ParseBuildInfo is an ordinary parser with a Go body.
//     modinfo, in the same file, has no body: only the runtime can answer it.
//   - os: IsPathSeparator lives in a per-platform file, and on every platform
//     it is a complete Go function. A per-platform file is how Go selects
//     source for a target. It is not a sign that the code calls the system.
//   - net: JoinHostPort is a colon search and a string concatenation. A
//     program that never opens a socket can use it.
func TestStdlibBoundaryIsDecidedPerDeclaration(t *testing.T) {
	if os.Getenv("GO2RUST_STDLIB_BOUNDARY_TARGET") != "" {
		t.Skip("this test sets its own targets")
	}
	cases := []struct {
		pkg      string
		withBody []string
		hooks    []string
	}{
		{pkg: "runtime/debug", withBody: []string{"ParseBuildInfo", "ReadBuildInfo", "Stack"}, hooks: []string{"modinfo"}},
		{pkg: "os", withBody: []string{"IsPathSeparator", "IsNotExist", "Getenv"}, hooks: []string{"runtime_args"}},
		{pkg: "net", withBody: []string{"JoinHostPort", "SplitHostPort", "ParseIP", "Dial"}, hooks: []string{"runtime_rand"}},
	}
	for _, target := range []string{"linux/amd64", "darwin/arm64", "windows/amd64"} {
		t.Setenv("GO2RUST_STDLIB_BOUNDARY_TARGET", target)
		for _, tc := range cases {
			listed := listStdlibBoundaryPackages(t, tc.pkg)
			if len(listed) != 1 {
				t.Fatalf("%s: go list %s returned %d packages, want 1", target, tc.pkg, len(listed))
			}
			pkg := listed[0]
			signals := stdlibBoundarySignalsForListedPackage(t, pkg)
			tier, err := stdlibBoundaryTier(tc.pkg, signals)
			if err != nil {
				t.Errorf("%s: %v", target, err)
			}
			if tier != stdlibTierSourceWithHooks {
				t.Errorf("%s: %s is %v, want %v", target, tc.pkg, tier, stdlibTierSourceWithHooks)
			}
			bodyless := make(map[string]bool, len(signals.BodylessFuncs))
			for _, name := range signals.BodylessFuncs {
				bodyless[name] = true
			}
			declared := stdlibBoundaryDeclaredFuncs(t, pkg)
			for _, name := range tc.withBody {
				if !declared[name] {
					t.Errorf("%s: %s.%s is not declared; pick another function for this test", target, tc.pkg, name)
				}
				if bodyless[name] {
					t.Errorf("%s: %s.%s has no Go body any more; it now needs a boundary decision", target, tc.pkg, name)
				}
			}
			for _, name := range tc.hooks {
				if !bodyless[name] {
					t.Errorf("%s: %s.%s has a Go body now; remove it from the hook list", target, tc.pkg, name)
				}
			}
		}
	}
}

// stdlibBoundaryDeclaredFuncs returns the names of the package-level
// functions a listed package declares, with or without a body.
func stdlibBoundaryDeclaredFuncs(t *testing.T, pkg stdlibBoundaryListedPackage) map[string]bool {
	t.Helper()
	fset := token.NewFileSet()
	declared := make(map[string]bool)
	for _, name := range pkg.GoFiles {
		file, err := parser.ParseFile(fset, filepath.Join(pkg.Dir, name), nil, parser.SkipObjectResolution)
		if err != nil {
			t.Fatalf("parse %s/%s: %v", pkg.ImportPath, name, err)
		}
		for _, decl := range file.Decls {
			if fn, ok := decl.(*ast.FuncDecl); ok && fn.Recv == nil {
				declared[fn.Name.Name] = true
			}
		}
	}
	return declared
}

type stdlibBoundaryListedPackage struct {
	ImportPath string
	Dir        string
	Standard   bool
	Incomplete bool
	GoFiles    []string
	CgoFiles   []string
	SFiles     []string
	Imports    []string
	Error      *struct{ Err string }
}

// listStdlibBoundaryPackages runs `go list` with the self-host build tags and
// returns the standard-library packages among the results. Any package the go
// tool could not load completely fails the test: classifying partial file
// lists would report a package as clean when it is not.
func listStdlibBoundaryPackages(t *testing.T, args ...string) []stdlibBoundaryListedPackage {
	t.Helper()
	cmdArgs := append([]string{"list", "-e", "-json", "-tags=" + stdlibBoundaryBuildTags}, args...)
	cmd := exec.Command("go", cmdArgs...)
	if target := strings.TrimSpace(os.Getenv("GO2RUST_STDLIB_BOUNDARY_TARGET")); target != "" {
		goos, goarch, ok := strings.Cut(target, "/")
		if !ok {
			t.Fatalf("GO2RUST_STDLIB_BOUNDARY_TARGET=%q, want GOOS/GOARCH", target)
		}
		cmd.Env = append(os.Environ(), "GOOS="+goos, "GOARCH="+goarch, "CGO_ENABLED=0")
	}
	var stderr bytes.Buffer
	cmd.Stderr = &stderr
	out, err := cmd.Output()
	if err != nil {
		t.Fatalf("go %s: %v\n%s", strings.Join(cmdArgs, " "), err, stderr.String())
	}
	var listed []stdlibBoundaryListedPackage
	decoder := json.NewDecoder(bytes.NewReader(out))
	for {
		var pkg stdlibBoundaryListedPackage
		if err := decoder.Decode(&pkg); err == io.EOF {
			break
		} else if err != nil {
			t.Fatalf("decode go list output: %v", err)
		}
		if pkg.Error != nil {
			t.Fatalf("go %s: package %s did not load: %s", strings.Join(cmdArgs, " "), pkg.ImportPath, pkg.Error.Err)
		}
		if pkg.Incomplete {
			t.Fatalf("go %s: package %s is incomplete (it or a dependency has errors)", strings.Join(cmdArgs, " "), pkg.ImportPath)
		}
		if !pkg.Standard {
			continue
		}
		if pkg.Dir == "" {
			t.Fatalf("go %s: standard-library package %s has no source directory", strings.Join(cmdArgs, " "), pkg.ImportPath)
		}
		listed = append(listed, pkg)
	}
	sort.Slice(listed, func(i, j int) bool { return listed[i].ImportPath < listed[j].ImportPath })
	return listed
}

func stdlibBoundarySignalsForListedPackage(t *testing.T, pkg stdlibBoundaryListedPackage) stdlibRuntimeSignals {
	t.Helper()
	fset := token.NewFileSet()
	var files []*ast.File
	for _, name := range append(append([]string{}, pkg.GoFiles...), pkg.CgoFiles...) {
		file, err := parser.ParseFile(fset, filepath.Join(pkg.Dir, name), nil, parser.SkipObjectResolution)
		if err != nil {
			t.Fatalf("parse %s/%s: %v", pkg.ImportPath, name, err)
		}
		files = append(files, file)
	}
	return collectStdlibRuntimeSignals(pkg.SFiles, files)
}

// TestStdlibBoundaryCoversSelfHostClosure classifies every standard-library
// package go2rust itself depends on, using the installed Go toolchain. It
// fails when a package needs a boundary decision nobody has made: a public
// package with runtime signals and no policy entry, or a source+hooks package
// with a hook outside its list.
//
// Run with -v to print the full table. Set GO2RUST_STDLIB_BOUNDARY_SCOPE=std
// to check the whole standard library instead of go2rust's closure, and
// GO2RUST_STDLIB_BOUNDARY_TARGET=darwin/arm64 to check another platform.
func TestStdlibBoundaryCoversSelfHostClosure(t *testing.T) {
	scope := []string{"-deps", "."}
	if override := strings.TrimSpace(os.Getenv("GO2RUST_STDLIB_BOUNDARY_SCOPE")); override != "" {
		scope = append([]string{"-deps"}, strings.Fields(override)...)
	}
	listed := listStdlibBoundaryPackages(t, scope...)
	if len(listed) == 0 {
		t.Fatalf("go list returned no standard-library packages for %v", scope)
	}

	tiers := make(map[string]stdlibTier, len(listed))
	byTier := make(map[stdlibTier][]string)
	for _, pkg := range listed {
		signals := stdlibBoundarySignalsForListedPackage(t, pkg)
		tier, err := stdlibBoundaryTier(pkg.ImportPath, signals)
		if err != nil {
			t.Errorf("%v", err)
		}
		tiers[pkg.ImportPath] = tier
		byTier[tier] = append(byTier[tier], pkg.ImportPath)
		if tier != stdlibTierSource {
			t.Logf("%-13s %-32s %s (%d assembly, %d bodyless)", tier, pkg.ImportPath,
				stdlibBoundaryReason(pkg.ImportPath, tier), len(signals.AsmFiles), len(signals.BodylessFuncs))
		}
	}
	t.Logf("%d standard-library packages: %d source, %d source+hooks, %d native",
		len(listed), len(byTier[stdlibTierSource]), len(byTier[stdlibTierSourceWithHooks]), len(byTier[stdlibTierNative]))

	// Anchors: packages whose tier must not drift. Each is checked only when
	// it is part of the listed scope.
	anchors := map[string]stdlibTier{
		"go/ast":           stdlibTierSource,
		"go/parser":        stdlibTierSource,
		"go/scanner":       stdlibTierSource,
		"go/token":         stdlibTierSource,
		"path/filepath":    stdlibTierSource,
		"strconv":          stdlibTierSource,
		"unicode/utf8":     stdlibTierSource,
		"go/types":         stdlibTierSourceWithHooks,
		"time":             stdlibTierSourceWithHooks,
		"internal/abi":     stdlibTierNative,
		"internal/bytealg": stdlibTierNative,
		"os":               stdlibTierSourceWithHooks,
		"reflect":          stdlibTierNative,
		"runtime":          stdlibTierNative,
		"sync":             stdlibTierNative,
		"sync/atomic":      stdlibTierNative,
		"syscall":          stdlibTierNative,
		"unsafe":           stdlibTierNative,
	}
	for path, want := range anchors {
		got, ok := tiers[path]
		if !ok {
			continue
		}
		if got != want {
			t.Errorf("%s is %v, want %v", path, got, want)
		}
	}
	// The transpiler cannot work without these, so their absence means the
	// scope was read wrong rather than that the closure shrank.
	if len(scope) == 2 && scope[1] == "." {
		for _, path := range []string{"go/ast", "go/types", "runtime", "strconv"} {
			if _, ok := tiers[path]; !ok {
				t.Errorf("%s is missing from go2rust's dependency closure; the go list scope is wrong", path)
			}
		}
	}
}

var stdlibBoundaryFixtureOptInPattern = regexp.MustCompile(`(?m)^\s*source_stdlib_packages\s*=\s*"([^"]*)"`)

const stdlibBoundaryFixtureBaselinePath = "testdata/stdlib_boundary_fixture_baseline.txt"

// stdlibBoundaryFixtureOptIns returns, per fixture name, the tokens of the
// fixture's source_stdlib_packages setting exactly as written, split the way
// the loader splits them.
func stdlibBoundaryFixtureOptIns(t *testing.T) map[string][]string {
	t.Helper()
	optIns := make(map[string][]string)
	for _, glob := range []string{
		filepath.Join("..", "tests", "*", ".go2rust.toml"),
		filepath.Join("..", "tests", "XFAIL", "*", ".go2rust.toml"),
	} {
		paths, err := filepath.Glob(glob)
		if err != nil {
			t.Fatalf("glob %s: %v", glob, err)
		}
		for _, path := range paths {
			content, err := os.ReadFile(path)
			if err != nil {
				t.Fatalf("read %s: %v", path, err)
			}
			match := stdlibBoundaryFixtureOptInPattern.FindSubmatch(content)
			if match == nil {
				continue
			}
			fixture := filepath.Base(filepath.Dir(path))
			for _, token := range strings.FieldsFunc(string(match[1]), func(r rune) bool {
				return r == ',' || r == ';' || r == ':' || r == ' ' || r == '\t' || r == '\n'
			}) {
				if token = strings.TrimSpace(token); token != "" {
					optIns[fixture] = append(optIns[fixture], token)
				}
			}
		}
	}
	return optIns
}

// stdlibBoundaryExpandOptIn returns the standard-library packages one
// source_stdlib_packages token lets the loader source-transpile, and whether
// the token is broad (a wildcard or a +deps expansion) rather than one named
// package.
//
// It mirrors the loader: sourceStdlibPackagePatternMatchesWithMode decides
// which packages a pattern names, +deps follows standard-library imports
// transitively, and isSourceTranspilableStdlibPackage is the loader's own
// filter, so the result shrinks by itself once the loader enforces the
// boundary. A wildcard is read as every package it can match, which is the
// most the loader could select.
func stdlibBoundaryExpandOptIn(token string, std map[string]stdlibBoundaryListedPackage) (pkgs []string, broad bool) {
	pattern := strings.TrimSuffix(token, "+deps")
	deps := pattern != token
	if pattern == "" {
		return nil, false
	}
	wildcard := pattern == "all" || pattern == "std" || strings.HasSuffix(pattern, "/...")

	selected := make(map[string]bool)
	var queue []string
	add := func(path string) {
		if _, ok := std[path]; !ok || selected[path] || !isSourceTranspilableStdlibPackage(path) {
			return
		}
		selected[path] = true
		queue = append(queue, path)
	}
	for path := range std {
		if sourceStdlibPackagePatternMatches(path, pattern) {
			add(path)
		}
	}
	if deps {
		for len(queue) > 0 {
			path := queue[0]
			queue = queue[1:]
			for _, imp := range std[path].Imports {
				add(imp)
			}
		}
	}
	for path := range selected {
		pkgs = append(pkgs, path)
	}
	sort.Strings(pkgs)
	return pkgs, deps || wildcard
}

func TestStdlibBoundaryExpandOptIn(t *testing.T) {
	std := map[string]stdlibBoundaryListedPackage{
		"a":        {ImportPath: "a", Imports: []string{"a/sub", "runtime", "unsafe", "C"}},
		"a/sub":    {ImportPath: "a/sub", Imports: []string{"leaf"}},
		"ab":       {ImportPath: "ab"},
		"leaf":     {ImportPath: "leaf"},
		"runtime":  {ImportPath: "runtime", Imports: []string{"hidden"}},
		"hidden":   {ImportPath: "hidden"},
		"unsafe":   {ImportPath: "unsafe"},
		"other":    {ImportPath: "other"},
		"other/in": {ImportPath: "other/in"},
	}
	cases := []struct {
		token     string
		want      string
		wantBroad bool
	}{
		{token: "a", want: "a"},
		{token: "missing", want: ""},
		// The loader never source-transpiles runtime or unsafe, and does not
		// follow imports through them.
		{token: "runtime", want: ""},
		{token: "a+deps", want: "a,a/sub,leaf", wantBroad: true},
		{token: "a/...", want: "a,a/sub", wantBroad: true},
		{token: "a/...+deps", want: "a,a/sub,leaf", wantBroad: true},
		{token: "other/...", want: "other,other/in", wantBroad: true},
		{token: "all", want: "a,a/sub,ab,hidden,leaf,other,other/in", wantBroad: true},
		{token: "std", want: "a,a/sub,ab,hidden,leaf,other,other/in", wantBroad: true},
		{token: "+deps", want: ""},
	}
	for _, tc := range cases {
		got, broad := stdlibBoundaryExpandOptIn(tc.token, std)
		if strings.Join(got, ",") != tc.want {
			t.Errorf("expand %q = %v, want %s", tc.token, got, tc.want)
		}
		if broad != tc.wantBroad {
			t.Errorf("expand %q broad = %v, want %v", tc.token, broad, tc.wantBroad)
		}
	}
}

// TestStdlibBoundaryFixtureRatchet stops fixtures from source-transpiling
// more native-tier packages while the existing ones are migrated.
//
// The baseline holds one line per offending opt-in, "<fixture> <token>":
//
//   - a token that names one native-tier package, or
//   - a broad token (a wildcard or +deps) whose expansion reaches at least one
//     native-tier package. The token is recorded as written, because the
//     packages it reaches change with the platform and the Go version.
//
// An offending opt-in outside the baseline fails the test. A baseline line
// that no longer applies also fails, so the file only shrinks. When it is
// empty, delete it together with this test and let the loader enforce the
// boundary.
//
// GO2RUST_UPDATE_STDLIB_BOUNDARY_BASELINE=1 prunes the baseline: it drops the
// lines that no longer apply. It never adds a line.
func TestStdlibBoundaryFixtureRatchet(t *testing.T) {
	optIns := stdlibBoundaryFixtureOptIns(t)
	if len(optIns) == 0 {
		t.Fatalf("found no fixture with source_stdlib_packages; the fixture glob is wrong")
	}

	std := make(map[string]stdlibBoundaryListedPackage)
	for _, pkg := range listStdlibBoundaryPackages(t, "std") {
		std[pkg.ImportPath] = pkg
	}

	// Classify lazily: only packages some fixture can reach are parsed.
	tiers := make(map[string]stdlibTier)
	tierOf := func(path string) stdlibTier {
		if tier, ok := tiers[path]; ok {
			return tier
		}
		tier, err := stdlibBoundaryTier(path, stdlibBoundarySignalsForListedPackage(t, std[path]))
		if err != nil {
			// The closure test only covers go2rust's own dependencies. A
			// fixture can opt into a package outside them, so an undecided
			// package or an unlisted hook has to fail here too.
			t.Errorf("a fixture opts into a package that needs a boundary decision: %v", err)
		}
		tiers[path] = tier
		return tier
	}

	current := make(map[string][]string)
	for fixture, tokens := range optIns {
		for _, token := range tokens {
			pkgs, broad := stdlibBoundaryExpandOptIn(token, std)
			var native []string
			for _, pkg := range pkgs {
				if tierOf(pkg) == stdlibTierNative {
					native = append(native, pkg)
				}
			}
			if len(native) == 0 {
				continue
			}
			if broad {
				current[fixture+" "+token] = native
			} else {
				current[fixture+" "+pkgs[0]] = native
			}
		}
	}

	baselineBytes, err := os.ReadFile(stdlibBoundaryFixtureBaselinePath)
	if err != nil {
		t.Fatalf("read %s: %v", stdlibBoundaryFixtureBaselinePath, err)
	}
	baseline := make(map[string]bool)
	for _, line := range strings.Split(string(baselineBytes), "\n") {
		if line = strings.TrimSpace(line); line != "" {
			baseline[line] = true
		}
	}

	var added []string
	for entry := range current {
		if !baseline[entry] {
			added = append(added, entry)
		}
	}
	sort.Strings(added)
	for _, entry := range added {
		fixture, token, _ := strings.Cut(entry, " ")
		native := current[entry]
		if len(native) == 1 && native[0] == token {
			t.Errorf("fixture %s source-transpiles native-tier package %s. Native-tier packages are provided in hand-written Rust, not transpiled; remove it from source_stdlib_packages. See docs/stdlib_boundary.md.", fixture, token)
			continue
		}
		shown := native
		more := ""
		if len(shown) > 6 {
			more = " and " + strconv.Itoa(len(shown)-6) + " more"
			shown = shown[:6]
		}
		t.Errorf("fixture %s opts into %q, which makes the loader source-transpile native-tier packages: %s%s. Name the source-tier packages it needs explicitly instead. See docs/stdlib_boundary.md.", fixture, token, strings.Join(shown, ", "), more)
	}

	var kept, stale []string
	for entry := range baseline {
		if _, ok := current[entry]; ok {
			kept = append(kept, entry)
		} else {
			stale = append(stale, entry)
		}
	}
	sort.Strings(kept)
	sort.Strings(stale)

	if os.Getenv("GO2RUST_UPDATE_STDLIB_BOUNDARY_BASELINE") == "1" {
		content := strings.Join(kept, "\n")
		if content != "" {
			content += "\n"
		}
		if err := os.WriteFile(stdlibBoundaryFixtureBaselinePath, []byte(content), 0o644); err != nil {
			t.Fatalf("write %s: %v", stdlibBoundaryFixtureBaselinePath, err)
		}
		t.Logf("pruned %d stale entries from %s; %d remain", len(stale), stdlibBoundaryFixtureBaselinePath, len(kept))
		return
	}
	for _, entry := range stale {
		t.Errorf("baseline entry %q no longer applies; delete it from go/%s (or rerun with GO2RUST_UPDATE_STDLIB_BOUNDARY_BASELINE=1)", entry, stdlibBoundaryFixtureBaselinePath)
	}
	t.Logf("%d fixture opt-ins still source-transpile a native-tier package", len(current))
}
