package main

import (
	"reflect"
	"testing"
)

func TestSourceMappedCrateDependenciesForStubCode(t *testing.T) {
	got := sourceMappedCrateDependenciesForStubCode(
		"go_token::position::Pos\ninternal_types_errors::codes::Code\n",
		map[string]string{
			"go/token":              "go_token",
			"internal/types/errors": "internal_types_errors",
			"go/types":              "go_types",
		},
	)
	want := []string{"go_token", "internal_types_errors"}
	if !reflect.DeepEqual(got, want) {
		t.Fatalf("sourceMappedCrateDependenciesForStubCode() = %#v, want %#v", got, want)
	}
}
