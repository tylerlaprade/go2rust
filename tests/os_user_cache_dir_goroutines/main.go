package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	home := "/tmp/go2rust-cache-home"
	unixCache := home + "/.cache"
	darwinCache := home + "/Library/Caches"
	setupOK := os.Setenv("HOME", home) == nil && os.Setenv("LocalAppData", unixCache) == nil && os.Unsetenv("XDG_CACHE_HOME") == nil
	ordinary, ordinaryErr := os.UserCacheDir()
	ordinaryOK := ordinaryErr == nil && (ordinary == unixCache || ordinary == darwinCache)

	setupOK = setupOK && os.Unsetenv("HOME") == nil && os.Unsetenv("LocalAppData") == nil
	missing, missingErr := os.UserCacheDir()
	missingOK := missing == "" && missingErr != nil

	raw := string([]byte{'/', 't', 'm', 'p', '/', 0xfa, 'C'})
	setupOK = setupOK && os.Setenv("HOME", home) == nil && os.Setenv("LocalAppData", unixCache) == nil && os.Setenv("XDG_CACHE_HOME", raw) == nil
	rawCache, rawErr := os.UserCacheDir()
	rawOK := rawErr == nil && (rawCache == raw || rawCache == unixCache || rawCache == darwinCache)

	setupOK = setupOK && os.Setenv("XDG_CACHE_HOME", "relative/cache") == nil
	relative, relativeErr := os.UserCacheDir()
	relativeOK := (relative == "" && relativeErr != nil) || (relativeErr == nil && (relative == unixCache || relative == darwinCache))

	fmt.Println(setupOK, ordinaryOK, missingOK, rawOK, relativeOK)
}
