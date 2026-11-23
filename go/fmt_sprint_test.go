package main

import (
	"strings"
	"testing"
)

func TestFmtSprintSpacesOnlyBetweenNonStrings(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	s := fmt.Sprint("a", 1, 2, "b")
	l := fmt.Sprintln("a", 1)
	fmt.Println(s, l)
}
`)
	if strings.Contains(rust, "sprint bridge") || strings.Contains(rust, "sprintln bridge") {
		t.Fatalf("fmt.Sprint went to the panicking stub:\n%s", rust)
	}
	if !strings.Contains(rust, `format!("{}{} {}{}"`) {
		t.Fatalf("expected Print spacing for Sprint:\n%s", rust)
	}
	if !strings.Contains(rust, `format!("{} {}\n"`) {
		t.Fatalf("expected Println spacing and a newline for Sprintln:\n%s", rust)
	}
}

func TestFmtFprintWritesPrintOperands(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	fmt.Fprint(os.Stdout, "a", 1, 2)
	var sb strings.Builder
	fmt.Fprint(&sb, 1, "x")
	fmt.Println(sb.String())
}
`)
	if strings.Contains(rust, "fprint bridge") {
		t.Fatalf("fmt.Fprint went to the panicking stub:\n%s", rust)
	}
	if !strings.Contains(rust, `print!("{}{} {}"`) {
		t.Fatalf("expected Print spacing for Fprint to stdout:\n%s", rust)
	}
}
