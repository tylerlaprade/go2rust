package main

import (
	"strings"
	"testing"
)

func TestFmtSscanCallsTheScanner(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	var a int
	var s string
	var f float32
	n, err := fmt.Sscan("1 x 2.5", &a, &s, &f)
	fmt.Println(n, err, a, s, f)
}
`)
	for _, want := range []string{
		"fn go_fmt_scan(",
		"go_scan_string_source(",
		"GoScanTarget::Int(64, ",
		"GoScanTarget::Str(",
		"GoScanTarget::Float(32, ",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "fmt::sscan(") {
		t.Fatalf("fmt.Sscan still calls its stub:\n%s", rust)
	}
}

func TestFmtScanReadsStdinAndSscanfKeepsItsFormat(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)
	var x, y int
	fmt.Sscanf("1,2", "%d,%d", &x, &y)
	fmt.Println(n, x, y)
}
`)
	if !strings.Contains(rust, "go_scan_stdin_source()") {
		t.Errorf("fmt.Scan does not read stdin:\n%s", rust)
	}
	if !strings.Contains(rust, "'f', &__scan_format") || !strings.Contains(rust, `"%d,%d"`) {
		t.Errorf("fmt.Sscanf does not scan with its format:\n%s", rust)
	}
}

func TestFmtScanOfADefinedTypeCompilesAndPanics(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Celsius float64

func main() {
	var c Celsius
	fmt.Sscan("21.5", &c)
	fmt.Println(c)
}
`)
	if !strings.Contains(rust, `panic!("fmt.Sscan: an operand`) {
		t.Fatalf("an operand of a defined type is not refused:\n%s", rust)
	}
}
