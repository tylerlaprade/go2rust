package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"testing"
)

// The Rust text of the generated helpers lives in this package's string
// literals. Every name it binds, as a parameter, a let, a for, a closure
// parameter or a pattern, must be in helperBindingNames, or a package-level
// variable with that name stops the program from compiling (DEV-65).
func TestHelperBindingNamesCoverTheTemplates(t *testing.T) {
	files, err := filepath.Glob("*.go")
	if err != nil {
		t.Fatal(err)
	}
	var rust strings.Builder
	fset := token.NewFileSet()
	for _, path := range files {
		if strings.HasSuffix(path, "_test.go") {
			continue
		}
		src, err := os.ReadFile(path)
		if err != nil {
			t.Fatal(err)
		}
		file, err := parser.ParseFile(fset, path, src, 0)
		if err != nil {
			t.Fatal(err)
		}
		ast.Inspect(file, func(node ast.Node) bool {
			lit, ok := node.(*ast.BasicLit)
			if !ok || lit.Kind != token.STRING {
				return true
			}
			if text, err := strconv.Unquote(lit.Value); err == nil {
				rust.WriteString(text)
				rust.WriteString("\n")
			}
			return true
		})
	}
	missing := map[string]bool{}
	for name := range rustTemplateBindingNames(rust.String()) {
		if !helperBindingNames[name] {
			missing[name] = true
		}
	}
	if len(missing) > 0 {
		var names []string
		for name := range missing {
			names = append(names, strconv.Quote(name))
		}
		sort.Strings(names)
		t.Fatalf("add these helper binding names to helperBindingNames: %s", strings.Join(names, ", "))
	}
}

// A package-level variable named after a helper binding gets another Rust
// name, so the helpers that print it compile.
func TestPackageGlobalNamedAfterAHelperBindingIsRenamed(t *testing.T) {
	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main

var value = "x"
var Value = "y"
var counter = 1
`, 0)
	if err != nil {
		t.Fatal(err)
	}
	overrides := assignPackageGlobalNameOverrides([]*ast.File{file}, nil)
	if overrides["value"] != "value_1" {
		t.Fatalf("value should be renamed to value_1, overrides %v", overrides)
	}
	if _, ok := overrides["Value"]; ok {
		t.Fatalf("an exported name cannot clash with a helper binding, overrides %v", overrides)
	}
	if _, ok := overrides["counter"]; ok && !helperBindingNames["counter"] {
		t.Fatalf("counter is no helper binding, overrides %v", overrides)
	}
}

var (
	rustIdentPattern    = `[a-z_][a-z0-9_]*`
	rustBindingPatterns = []*regexp.Regexp{
		regexp.MustCompile(`\blet\s+(?:(?:mut|ref)\s+)*(` + rustIdentPattern + `)\b`),
		regexp.MustCompile(`\bfor\s+(?:mut\s+)?(` + rustIdentPattern + `)\s+in\b`),
		regexp.MustCompile(`\b(?:Some|Ok|Err)\(\s*(?:(?:mut|ref)\s+)*(` + rustIdentPattern + `)\s*\)`),
		regexp.MustCompile(`\|\s*(?:mut\s+)?&?\s*(` + rustIdentPattern + `)\s*(?::[^|]*)?\|`),
		regexp.MustCompile(`\b(` + rustIdentPattern + `)\s*@`),
	}
	rustBindingGroupPatterns = []*regexp.Regexp{
		regexp.MustCompile(`\blet\s+(?:mut\s+)?\(([^=;]*)\)\s*[=:]`),
		regexp.MustCompile(`\bfor\s+\(([^)]*)\)\s+in\b`),
		regexp.MustCompile(`\|([^|\n]*,[^|\n]*)\|`),
	}
	rustBindingGroupPart = regexp.MustCompile(`^\s*(?:(?:mut|ref)\s+)*(` + rustIdentPattern + `)\s*(?::.*)?$`)
	rustFnStart          = regexp.MustCompile(`\bfn\s+[A-Za-z_][A-Za-z0-9_]*\s*(?:<[^{;]*?>)?\s*\(`)
	rustParamName        = regexp.MustCompile(`^\s*(?:mut\s+)?(` + rustIdentPattern + `)\s*:`)
)

// rustTemplateBindingNames returns the names rust binds, leaving out the
// ones that start with an underscore, which no Go identifier the helpers
// meet is expected to take.
func rustTemplateBindingNames(rust string) map[string]bool {
	names := map[string]bool{}
	add := func(name string) {
		if name != "" && !strings.HasPrefix(name, "_") && name != "self" && name != "mut" && name != "ref" {
			names[name] = true
		}
	}
	for _, pattern := range rustBindingPatterns {
		for _, match := range pattern.FindAllStringSubmatch(rust, -1) {
			add(match[1])
		}
	}
	for _, pattern := range rustBindingGroupPatterns {
		for _, match := range pattern.FindAllStringSubmatch(rust, -1) {
			for _, part := range strings.FieldsFunc(match[1], func(r rune) bool { return r == ',' || r == '(' || r == ')' || r == '&' }) {
				if m := rustBindingGroupPart.FindStringSubmatch(part); m != nil {
					add(m[1])
				}
			}
		}
	}
	for _, loc := range rustFnStart.FindAllStringIndex(rust, -1) {
		open := loc[1] - 1
		depth, end := 0, -1
		for i := open; i < len(rust); i++ {
			if rust[i] == '(' {
				depth++
			} else if rust[i] == ')' {
				depth--
				if depth == 0 {
					end = i
					break
				}
			}
		}
		if end < 0 {
			continue
		}
		depth = 0
		start := open + 1
		params := rust[open+1 : end]
		for i := 0; i <= len(params); i++ {
			if i < len(params) {
				switch params[i] {
				case '(', '<', '[':
					depth++
					continue
				case '>':
					if i > 0 && params[i-1] == '-' {
						// The arrow of a function type.
						continue
					}
					depth--
					continue
				case ')', ']':
					depth--
					continue
				case ',':
					if depth != 0 {
						continue
					}
				default:
					continue
				}
			}
			part := rust[start : open+1+i]
			if m := rustParamName.FindStringSubmatch(part); m != nil {
				add(m[1])
			}
			start = open + 1 + i + 1
		}
	}
	return names
}
