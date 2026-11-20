package main

import (
	"strings"
	"testing"
)

func TestPrintfPlusVWritesFieldNames(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Point struct{ X, Y int }

func main() {
	p := Point{1, 2}
	fmt.Printf("%+v %v\n", p, p)
}
`)
	if !strings.Contains(rust, `print!("{:#} {}\n"`) {
		t.Fatalf("expected %%+v as {:#}:\n%s", rust)
	}
	if !strings.Contains(rust, `"{{X:{} Y:{}}}"`) {
		t.Fatalf("expected the field-name form in Display:\n%s", rust)
	}
}

func TestStructDisplayHasNoFieldNameFormWithoutPlusV(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Point struct{ X, Y int }

func main() {
	fmt.Printf("%v %d%%\n", Point{1, 2}, 3)
}
`)
	if strings.Contains(rust, "__go_plus") {
		t.Fatalf("no %%+v in the program, so no field-name form:\n%s", rust)
	}
}

func TestFormatPlusVArgIndices(t *testing.T) {
	got := formatPlusVArgIndices(`"%d %+v %%+v %-+8v %s %+d"`)
	if len(got) != 2 || !got[1] || !got[2] {
		t.Fatalf("got %v, want arguments 1 and 2", got)
	}
}
