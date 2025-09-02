package main

import (
	"fmt"
	"strings"
)

// ExternalPackageMode defines how to handle external package imports
type ExternalPackageMode int

const (
	ModeTranspile ExternalPackageMode = iota // Recursively transpile dependencies
	ModeFfi                                  // Generate FFI bridge
	ModeNone                                 // Error on external imports
)

func (m ExternalPackageMode) String() string {
	switch m {
	case ModeTranspile:
		return "transpile"
	case ModeFfi:
		return "ffi"
	case ModeNone:
		return "none"
	default:
		return "unknown"
	}
}

func ParseExternalPackageMode(s string) (ExternalPackageMode, error) {
	switch strings.ToLower(s) {
	case "transpile":
		return ModeTranspile, nil
	case "ffi":
		return ModeFfi, nil
	case "none":
		return ModeNone, nil
	default:
		return ModeTranspile, fmt.Errorf("invalid external package mode: %s (must be 'transpile', 'ffi', or 'none')", s)
	}
}
