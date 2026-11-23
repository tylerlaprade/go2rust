package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	home := "/tmp/go2rust-config-home"
	unixConfig := home + "/.config"
	darwinConfig := home + "/Library/Application Support"
	setupOK := os.Setenv("HOME", home) == nil && os.Setenv("AppData", unixConfig) == nil && os.Unsetenv("XDG_CONFIG_HOME") == nil
	ordinary, ordinaryErr := os.UserConfigDir()
	ordinaryOK := ordinaryErr == nil && (ordinary == unixConfig || ordinary == darwinConfig)

	setupOK = setupOK && os.Unsetenv("HOME") == nil && os.Unsetenv("AppData") == nil
	missing, missingErr := os.UserConfigDir()
	missingOK := missing == "" && missingErr != nil

	raw := string([]byte{'/', 't', 'm', 'p', '/', 0xfa, 'G'})
	setupOK = setupOK && os.Setenv("HOME", home) == nil && os.Setenv("AppData", unixConfig) == nil && os.Setenv("XDG_CONFIG_HOME", raw) == nil
	rawConfig, rawErr := os.UserConfigDir()
	rawOK := rawErr == nil && (rawConfig == raw || rawConfig == unixConfig || rawConfig == darwinConfig)

	setupOK = setupOK && os.Setenv("XDG_CONFIG_HOME", "relative/config") == nil
	relative, relativeErr := os.UserConfigDir()
	relativeOK := (relative == "" && relativeErr != nil) || (relativeErr == nil && (relative == unixConfig || relative == darwinConfig))

	fmt.Println(setupOK, ordinaryOK, missingOK, rawOK, relativeOK)
}
