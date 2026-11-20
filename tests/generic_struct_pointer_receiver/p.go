package main

// These value types deliberately live in a sibling file that is emitted
// before main.rs. Generic-struct copy support must be discovered package-wide
// so both receive the required copy-trait implementations.
type P struct{ X int }

type Number int
