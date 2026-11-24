package main

import (
	"strings"
	"testing"
)

// In the shared helper crate a program's select calls try_recv_select on
// the crate's GoChannel, so the method is public there like the channel's
// other operations. A program that imports a package translated from
// source, such as strings, uses that crate's GoChannel.
func TestSharedHelperChannelExposesSelectReceive(t *testing.T) {
	previous := generatingPublicHelpers
	generatingPublicHelpers = true
	defer func() { generatingPublicHelpers = previous }()
	var out strings.Builder
	generateGoChannelHelper(&out, false)
	code := out.String()
	for _, want := range []string{"pub fn try_recv_select(&self)", "pub fn try_recv(&self)"} {
		if !strings.Contains(code, want) {
			t.Errorf("missing %q in the shared channel helper:\n%s", want, code)
		}
	}
}
