package main

import (
	"reflect"
	"testing"
)

func TestSourceMappedCrateDependenciesForStubCode(t *testing.T) {
	mapping := map[string]string{
		"go/token":              "go_token",
		"internal/types/errors": "internal_types_errors",
		"go/types":              "go_types",
		"cmp":                   "cmp",
	}

	tests := []struct {
		name     string
		stubCode string
		want     []string
	}{
		{
			name:     "direct crate references",
			stubCode: "go_token::position::Pos\ninternal_types_errors::codes::Code\ncmp::Less\n",
			want:     []string{"cmp", "go_token", "internal_types_errors"},
		},
		{
			name:     "nested path and identifier fragments",
			stubCode: "std::cmp::Ordering\nfoo_cmp::Value\n",
			want:     []string{},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := sourceMappedCrateDependenciesForStubCode(tt.stubCode, mapping)
			if !reflect.DeepEqual(got, tt.want) {
				t.Fatalf("sourceMappedCrateDependenciesForStubCode() = %#v, want %#v", got, tt.want)
			}
		})
	}
}
