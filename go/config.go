package main

// Global configuration for transpilation
var Config struct {
	// WrapEverything enables the conservative Arc<Mutex<Option<>>> wrapping for all variables
	WrapEverything bool
}
