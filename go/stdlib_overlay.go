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
	return overlay
}
