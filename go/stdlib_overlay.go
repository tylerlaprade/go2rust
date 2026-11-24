package main

import (
	"go/build"
	"os"
	"path/filepath"
	"regexp"
)

// The math package declares a few accelerators without a body, implemented
// in assembly on some architectures, and on those architectures sets
// haveArchFloor and the like to true. The generic Go path is complete: on an
// architecture without the assembly, a *_noasm.go file sets the constants to
// false and the package uses its own Go code. Assembly cannot be translated,
// so the package is loaded with each assembly-backed file replaced by the
// file the other architectures use.
//
// Each entry maps a file that the host architecture builds to the file that
// replaces its contents. An empty replacement leaves only the package
// clause: exp_amd64.go only reads CPU features for the assembly.
var stdlibNoAsmOverlays = map[string]map[string]string{
	"math": {
		"dim_asm.go":     "dim_noasm.go",
		"exp2_asm.go":    "exp2_noasm.go",
		"exp_asm.go":     "exp_noasm.go",
		"floor_asm.go":   "floor_noasm.go",
		"hypot_asm.go":   "hypot_noasm.go",
		"log_asm.go":     "log_stub.go",
		"modf_asm.go":    "modf_noasm.go",
		"exp_amd64.go":   "",
		"arith_s390x.go": "",
	},
	// internal/bytealg has the same shape: each *_native.go declares
	// assembly-backed functions, and the *_generic.go file other
	// architectures build has their Go code. index_amd64.go only picks
	// MaxLen from CPU features; without it MaxLen stays 0, as on an
	// architecture with no assembly Index, and strings and bytes use their
	// Go search instead.
	"internal/bytealg": {
		"compare_native.go":   "compare_generic.go",
		"count_native.go":     "count_generic.go",
		"index_native.go":     "index_generic.go",
		"indexbyte_native.go": "indexbyte_generic.go",
		"index_amd64.go":      "",
		"index_arm64.go":      "",
	},
}

// stdlibOverlayTransforms rewrite a file the host architecture builds
// rather than replacing it with another file.
var stdlibOverlayTransforms = map[string]map[string]func([]byte) []byte{
	"internal/bytealg": {
		"bytealg.go": removeAssemblyOffsetConstants,
	},
}

var (
	assemblyOffsetConstants = regexp.MustCompile(`(?s)// Offsets into internal/cpu records for use in assembly\.\nconst \(.*?\n\)\n`)
	internalCPUImport       = regexp.MustCompile(`(?m)^\s*"internal/cpu"\n`)
	unsafeImport            = regexp.MustCompile(`(?m)^(\s*)"unsafe"$`)
)

// removeAssemblyOffsetConstants drops bytealg.go's constants that give the
// assembly field offsets into internal/cpu's records. Nothing in Go reads
// them, and without them the package no longer needs internal/cpu. The
// unsafe import stays for its //go:linkname directives.
func removeAssemblyOffsetConstants(source []byte) []byte {
	source = assemblyOffsetConstants.ReplaceAll(source, nil)
	source = internalCPUImport.ReplaceAll(source, nil)
	return unsafeImport.ReplaceAll(source, []byte(`${1}_ "unsafe"`))
}

var goBuildConstraintLine = regexp.MustCompile(`(?m)^//go:build .*$`)

// stdlibNoAsmOverlay returns the overlay for packages.Config: for each
// standard-library file in stdlibNoAsmOverlays, the contents that replace it.
// A file the GOROOT does not have is left alone.
func stdlibNoAsmOverlay() map[string][]byte {
	goroot := build.Default.GOROOT
	if goroot == "" {
		return nil
	}
	overlay := map[string][]byte{}
	for pkg, files := range stdlibNoAsmOverlays {
		dir := filepath.Join(goroot, "src", filepath.FromSlash(pkg))
		for file, replacement := range files {
			path := filepath.Join(dir, file)
			if _, err := os.Stat(path); err != nil {
				continue
			}
			contents := []byte("package " + filepath.Base(pkg) + "\n")
			if replacement != "" {
				source, err := os.ReadFile(filepath.Join(dir, replacement))
				if err != nil {
					continue
				}
				contents = goBuildConstraintLine.ReplaceAll(source, nil)
			}
			overlay[path] = contents
		}
	}
	for pkg, files := range stdlibOverlayTransforms {
		dir := filepath.Join(goroot, "src", filepath.FromSlash(pkg))
		for file, transform := range files {
			path := filepath.Join(dir, file)
			source, err := os.ReadFile(path)
			if err != nil {
				continue
			}
			overlay[path] = transform(source)
		}
	}
	return overlay
}
