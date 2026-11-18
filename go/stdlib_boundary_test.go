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
		{name: "runtime subpackage is native", path: "runtime/debug", want: stdlibTierNative},
		{name: "internal runtime package is native", path: "internal/runtime/maps", want: stdlibTierNative},
		{name: "unsafe is native", path: "unsafe", want: stdlibTierNative},
		{name: "internal package with assembly is native", path: "internal/bytealg", signals: asm, want: stdlibTierNative},
		{name: "internal package with hooks is native", path: "internal/poll", signals: hook("runtime_pollOpen"), want: stdlibTierNative},
		{name: "vendored package with assembly is native", path: "vendor/golang.org/x/sys/cpu", signals: asm, want: stdlibTierNative},
		{name: "nested internal package with hooks is native", path: "crypto/internal/sysrand", signals: hook("fatal"), want: stdlibTierNative},
		{name: "policy native wins without signals", path: "sync", want: stdlibTierNative},
		{name: "policy native with signals", path: "sync/atomic", signals: asm, want: stdlibTierNative},
		{name: "listed hook", path: "math/rand", signals: hook("runtime_rand"), want: stdlibTierSourceWithHooks},
		{name: "prefix hook", path: "go/types", signals: hook("badlinkname_Checker_infer"), want: stdlibTierSourceWithHooks},
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

type recv struct{}
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
	if want := []string{"method", "noBody"}; strings.Join(got.BodylessFuncs, ",") != strings.Join(want, ",") {
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
			if len(decision.hooks) > 0 || decision.asm {
				t.Errorf("%s: native entry must not list hooks or asm", path)
			}
		default:
			t.Errorf("%s: policy entries are for native or source+hooks packages; source needs no entry", path)
		}
		if isRuntimeImplementationStdlibPath(path) {
			t.Errorf("%s: already native by path; remove the policy entry", path)
		}
	}
}

type stdlibBoundaryListedPackage struct {
	ImportPath string
	Dir        string
	Standard   bool
	GoFiles    []string
	CgoFiles   []string
	SFiles     []string
}

// listStdlibBoundaryPackages runs `go list` with the self-host build tags and
// returns the standard-library packages among the results.
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
		if pkg.Standard && pkg.Dir != "" {
			listed = append(listed, pkg)
		}
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
		"os":               stdlibTierNative,
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

// stdlibBoundaryFixtureOptIns returns, per fixture name, the standard-library
// packages the fixture names in source_stdlib_packages. Wildcard patterns are
// skipped: they name no single package.
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
			for _, pattern := range strings.FieldsFunc(string(match[1]), func(r rune) bool {
				return r == ',' || r == ';' || r == ':' || r == ' ' || r == '\t' || r == '\n'
			}) {
				pattern = strings.TrimSuffix(strings.TrimSpace(pattern), "+deps")
				if pattern == "" || pattern == "all" || pattern == "std" || strings.HasSuffix(pattern, "/...") {
					continue
				}
				optIns[fixture] = append(optIns[fixture], pattern)
			}
		}
	}
	return optIns
}

// TestStdlibBoundaryFixtureRatchet stops fixtures from source-transpiling
// more native-tier packages while the existing ones are migrated.
//
// The baseline lists every (fixture, native-tier package) pair that exists
// today. A pair outside the baseline fails the test. A baseline line whose
// pair is gone also fails, so the file only shrinks. When it is empty, delete
// it together with this test and let the loader enforce the boundary.
//
// Set GO2RUST_UPDATE_STDLIB_BOUNDARY_BASELINE=1 to rewrite the baseline after
// removing native-tier packages from a fixture.
func TestStdlibBoundaryFixtureRatchet(t *testing.T) {
	optIns := stdlibBoundaryFixtureOptIns(t)
	if len(optIns) == 0 {
		t.Fatalf("found no fixture with source_stdlib_packages; the fixture glob is wrong")
	}

	named := make(map[string]bool)
	for _, pkgs := range optIns {
		for _, pkg := range pkgs {
			named[pkg] = true
		}
	}
	var patterns []string
	for pkg := range named {
		patterns = append(patterns, pkg)
	}
	sort.Strings(patterns)

	tiers := make(map[string]stdlibTier)
	for _, pkg := range listStdlibBoundaryPackages(t, patterns...) {
		// A policy error still yields the safest tier, and the closure test
		// reports the error itself.
		tier, _ := stdlibBoundaryTier(pkg.ImportPath, stdlibBoundarySignalsForListedPackage(t, pkg))
		tiers[pkg.ImportPath] = tier
	}
	// Packages the installed toolchain cannot list for this platform still
	// count when they are native by path.
	for _, pkg := range patterns {
		if _, ok := tiers[pkg]; !ok && isRuntimeImplementationStdlibPath(pkg) {
			tiers[pkg] = stdlibTierNative
		}
	}

	var current []string
	for fixture, pkgs := range optIns {
		for _, pkg := range pkgs {
			if tiers[pkg] == stdlibTierNative {
				current = append(current, fixture+" "+pkg)
			}
		}
	}
	sort.Strings(current)

	if os.Getenv("GO2RUST_UPDATE_STDLIB_BOUNDARY_BASELINE") == "1" {
		content := strings.Join(current, "\n")
		if content != "" {
			content += "\n"
		}
		if err := os.WriteFile(stdlibBoundaryFixtureBaselinePath, []byte(content), 0o644); err != nil {
			t.Fatalf("write %s: %v", stdlibBoundaryFixtureBaselinePath, err)
		}
		t.Logf("rewrote %s with %d entries", stdlibBoundaryFixtureBaselinePath, len(current))
		return
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

	seen := make(map[string]bool, len(current))
	for _, entry := range current {
		seen[entry] = true
		if !baseline[entry] {
			fields := strings.Fields(entry)
			t.Errorf("fixture %s source-transpiles native-tier package %s. Native-tier packages are provided in hand-written Rust, not transpiled; remove it from source_stdlib_packages. See docs/stdlib_boundary.md.", fields[0], fields[1])
		}
	}
	var stale []string
	for entry := range baseline {
		if !seen[entry] {
			stale = append(stale, entry)
		}
	}
	sort.Strings(stale)
	for _, entry := range stale {
		t.Errorf("baseline entry %q no longer applies; delete it from go/%s (or rerun with GO2RUST_UPDATE_STDLIB_BOUNDARY_BASELINE=1)", entry, stdlibBoundaryFixtureBaselinePath)
	}
	t.Logf("%d fixture opt-ins still source-transpile a native-tier package", len(current))
}
