package main

import (
	"go/token"
	"go/types"
	"strings"
	"testing"
)

func TestGoTypesTypeToRustMapsUnsafePointer(t *testing.T) {
	if got, want := goTypesTypeToRust(types.Typ[types.UnsafePointer]), "usize"; got != want {
		t.Fatalf("goTypesTypeToRust(unsafe.Pointer) = %q, want %q", got, want)
	}
}

func TestGoTypesTypeToRustUsesAnyForUnnamedInterfaces(t *testing.T) {
	method := types.NewFunc(
		token.NoPos,
		nil,
		"Read",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(),
			types.NewTuple(types.NewVar(token.NoPos, nil, "n", types.Typ[types.Int])),
			false,
		),
	)
	iface := types.NewInterfaceType([]*types.Func{method}, nil).Complete()

	got := goTypesTypeToRustWrapped(iface)
	want := "Rc<RefCell<Option<Box<dyn Any>>>>"
	if got != want {
		t.Fatalf("goTypesTypeToRustWrapped(non-empty interface) = %q, want %q", got, want)
	}
}

func TestExternalStubDefaultValueUsesNoneForAnyTraitObjects(t *testing.T) {
	var out strings.Builder
	writeExternalStubDefaultValue(&out, "Rc<RefCell<Option<Box<dyn Any>>>>")

	got := out.String()
	want := "Rc::new(RefCell::new(None::<Box<dyn Any>>))"
	if got != want {
		t.Fatalf("writeExternalStubDefaultValue(Box<dyn Any>) = %q, want %q", got, want)
	}
}
