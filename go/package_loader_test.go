package main

import "testing"

func TestPackageLoadBuildFlagsDefaultToPureGo(t *testing.T) {
	got := packageLoadBuildFlags("")
	if len(got) != 1 || got[0] != "-tags=purego" {
		t.Fatalf("package loader should default to purego build tags, got %#v", got)
	}
}

func TestPackageLoadBuildFlagsPreserveExplicitTags(t *testing.T) {
	for _, goFlags := range []string{
		"-tags=dev",
		"-mod=mod -tags dev",
		"-tags=dev,purego -trimpath",
	} {
		if got := packageLoadBuildFlags(goFlags); got != nil {
			t.Fatalf("package loader should not add purego when GOFLAGS=%q already has tags, got %#v", goFlags, got)
		}
	}
}
