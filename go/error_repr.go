package main

import "strings"

// How an error value is represented in the generated Rust.
//
// An error variable is a handle: the wrapper every variable has, around an
// Option, around the error itself. nil is None. Everything that writes the
// type of an error, makes one, or recognizes one by its Rust type goes
// through the functions in this file, so that the representation is spelled
// in one place.
//
// The error itself is held by a shared pointer: Rc<dyn Error>, or
// Arc<dyn Error + Send + Sync> in a program with goroutines. Go copies an
// error value freely (last = err, return ErrNotFound, e2 := e1) and both
// copies are then the same error. A shared pointer gives exactly that: a
// copy is a clone of the pointer into the copier's own variable, and two
// copies are the same error because they point to the same allocation. A
// Box could do neither; it cannot be cloned.

// rustErrorTraitObject is the trait object every error is. It names the
// trait by the alias the generated file imports, StdError.
func rustErrorTraitObject() string {
	TrackImport("Error")
	return rustErrorTraitObjectFor("StdError", NeedsConcurrentWrapper())
}

// rustErrorTraitObjectFor spells the trait object with a given path to the
// Error trait. Code that cannot rely on the StdError import, such as a
// helper written out in full, passes std::error::Error.
func rustErrorTraitObjectFor(errorTrait string, concurrent bool) string {
	if concurrent {
		return "dyn " + errorTrait + " + Send + Sync"
	}
	return "dyn " + errorTrait
}

// rustErrorPointerFor is the shared pointer that holds an error. Code that
// names the Error trait by its whole path names the pointer by its whole
// path as well, because it cannot rely on the imports either.
func rustErrorPointerFor(errorTrait string, concurrent bool) string {
	qualified := errorTrait != "StdError"
	switch {
	case concurrent && qualified:
		return "std::sync::Arc"
	case concurrent:
		return "Arc"
	case qualified:
		return "std::rc::Rc"
	default:
		return "Rc"
	}
}

// rustErrorPointer is the shared pointer for the program's mode, by the
// name the generated file imports.
func rustErrorPointer() string {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		return "Arc"
	}
	TrackImport("Rc")
	return "Rc"
}

// rustErrorPayloadTypeFor is the type that holds the error itself, inside
// the handle's Option.
func rustErrorPayloadTypeFor(errorTrait string, concurrent bool) string {
	return rustErrorPointerFor(errorTrait, concurrent) + "<" + rustErrorTraitObjectFor(errorTrait, concurrent) + ">"
}

// rustErrorPayloadType is the payload type for the program's mode.
func rustErrorPayloadType() string {
	return rustErrorPointer() + "<" + rustErrorTraitObject() + ">"
}

// rustErrorFromTextStart opens the expression that makes an error from a
// String, or from anything else an error can be made from, such as a
// std::io::Error. The caller writes the value and then
// rustErrorFromTextEnd.
func rustErrorFromTextStart() string {
	TrackImport("Error")
	rustErrorPointer()
	return rustErrorFromTextStartFor("StdError", NeedsConcurrentWrapper())
}

// rustErrorFromTextStartFor is rustErrorFromTextStart with a given path to
// the Error trait.
//
// The standard library makes a Box<dyn Error> from those, and a shared
// pointer from a Box, so the expression is the two calls nested.
func rustErrorFromTextStartFor(errorTrait string, concurrent bool) string {
	traitObject := rustErrorTraitObjectFor(errorTrait, concurrent)
	return rustErrorPointerFor(errorTrait, concurrent) + "::<" + traitObject + ">::from(Box::<" + traitObject + ">::from("
}

// rustErrorFromTextEnd closes what rustErrorFromTextStart opened.
func rustErrorFromTextEnd() string {
	return "))"
}

// rustErrorFromText is the whole expression for a value given as Rust text.
func rustErrorFromText(value string) string {
	return rustErrorFromTextStart() + value + rustErrorFromTextEnd()
}

// rustErrorFromTextFor is rustErrorFromText with a given path to the Error
// trait.
func rustErrorFromTextFor(errorTrait string, concurrent bool, value string) string {
	return rustErrorFromTextStartFor(errorTrait, concurrent) + value + rustErrorFromTextEnd()
}

// rustErrorFromValueStart opens the expression that makes an error from a
// value of a type that implements error, such as a struct with an Error
// method. The caller writes the value and then rustErrorFromValueEnd.
func rustErrorFromValueStart() string {
	return rustErrorPointer() + "::new("
}

// rustErrorFromValueEnd closes what rustErrorFromValueStart opened.
func rustErrorFromValueEnd() string {
	return ") as " + rustErrorPayloadType()
}

// rustErrorFromValueFor is the whole expression for a value given as Rust
// text, with a given path to the Error trait.
func rustErrorFromValueFor(errorTrait string, concurrent bool, value string) string {
	return rustErrorPointerFor(errorTrait, concurrent) + "::new(" + value + ") as " + rustErrorPayloadTypeFor(errorTrait, concurrent)
}

// rustNoErrorPayload is a typed None: no error.
func rustNoErrorPayload() string {
	return "None::<" + rustErrorPayloadType() + ">"
}

// rustTypeIsGoErrorHandle reports whether a Rust type spells the error
// payload, with either path to the Error trait.
func rustTypeIsGoErrorHandle(rustType string) bool {
	return rustTypeHoldsImportedErrorPayload(rustType) ||
		strings.Contains(rustType, "Rc<dyn std::error::Error") ||
		strings.Contains(rustType, "Arc<dyn std::error::Error")
}

// rustTypeHoldsImportedErrorPayload is the same test for the spelling the
// translated program itself uses, with the StdError import.
func rustTypeHoldsImportedErrorPayload(rustType string) bool {
	return strings.Contains(rustType, "Rc<dyn StdError") ||
		strings.Contains(rustType, "Arc<dyn StdError")
}

// rustTypeStartsWithErrorPayload is the same test for a type that must
// begin with the payload.
func rustTypeStartsWithErrorPayload(rustType string) bool {
	for _, pointer := range []string{"Rc", "Arc", "std::rc::Rc", "std::sync::Arc"} {
		for _, errorTrait := range []string{"StdError", "std::error::Error"} {
			if strings.HasPrefix(rustType, pointer+"<dyn "+errorTrait) {
				return true
			}
		}
	}
	return false
}

// expandErrorRepr fills in the error representation in the text of a helper
// that is written out in full and names the Error trait by its whole path.
// GO_ERROR_PAYLOAD is the payload type. GO_ERROR_FROM(value) is the
// expression that makes an error from value.
func expandErrorRepr(code string, concurrent bool) string {
	const errorTrait = "std::error::Error"
	code = strings.ReplaceAll(code, "GO_ERROR_PAYLOAD", rustErrorPayloadTypeFor(errorTrait, concurrent))
	const marker = "GO_ERROR_FROM("
	var out strings.Builder
	for {
		start := strings.Index(code, marker)
		if start < 0 {
			break
		}
		out.WriteString(code[:start])
		code = code[start+len(marker):]
		// The value ends at the parenthesis that closes the marker's.
		depth, end := 1, -1
		for i, c := range code {
			if c == '(' {
				depth++
			} else if c == ')' {
				depth--
				if depth == 0 {
					end = i
					break
				}
			}
		}
		if end < 0 {
			panic("expandErrorRepr: GO_ERROR_FROM( is not closed")
		}
		out.WriteString(rustErrorFromTextFor(errorTrait, concurrent, code[:end]))
		code = code[end+1:]
	}
	out.WriteString(code)
	return out.String()
}
