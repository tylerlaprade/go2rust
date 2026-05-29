package main

import (
	"fmt"
	"strings"
)

// A constant string-typed conversion (string(rune-const)) passed as a string
// stdlib-method argument must lower to a raw String (Rust str Pattern), not a
// wrapped handle (E0277: &Arc<Mutex<Option<String>>>: Pattern). Mirrors
// path/filepath's strings.Contains(name, string(Separator)).
const slash = '/'

func main() {
	fmt.Println(strings.Contains("a/b", string(slash)))
	fmt.Println(strings.Contains("axb", string(slash)))
}
