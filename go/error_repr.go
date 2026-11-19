package main

import "strings"

// How an error value is represented in the generated Rust.
//
// An error variable is a handle: the wrapper every variable has, around an
// Option, around the error itself. nil is None. Everything that writes the
// type of an error, makes one, or recognizes one by its Rust type goes
// through the functions in this file, so that the representation is spelled
// in one place.

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

// rustErrorPayloadTypeFor is the type that holds the error itself, inside
// the handle's Option.
func rustErrorPayloadTypeFor(errorTrait string, concurrent bool) string {
	return "Box<" + rustErrorTraitObjectFor(errorTrait, concurrent) + ">"
}

// rustStdErrorBoxType is the payload type for the program's mode.
func rustStdErrorBoxType() string {
	return "Box<" + rustErrorTraitObject() + ">"
}

// rustErrorFromTextStart opens the expression that makes an error from a
// String, or from anything else an error can be made from, such as a
// std::io::Error. The caller writes the value and then
// rustErrorFromTextEnd.
func rustErrorFromTextStart() string {
	TrackImport("Error")
	return rustErrorFromTextStartFor("StdError", NeedsConcurrentWrapper())
}

// rustErrorFromTextStartFor is rustErrorFromTextStart with a given path to
// the Error trait.
func rustErrorFromTextStartFor(errorTrait string, concurrent bool) string {
	return "Box::<" + rustErrorTraitObjectFor(errorTrait, concurrent) + ">::from("
}

// rustErrorFromTextEnd closes what rustErrorFromTextStart opened.
func rustErrorFromTextEnd() string {
	return ")"
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
	return "Box::new("
}

// rustErrorFromValueEnd closes what rustErrorFromValueStart opened.
func rustErrorFromValueEnd() string {
	return ") as " + rustStdErrorBoxType()
}

// rustNoErrorPayload is a typed None: no error.
func rustNoErrorPayload() string {
	return "None::<" + rustStdErrorBoxType() + ">"
}

// rustTypeIsGoErrorHandle reports whether a Rust type spells the error
// payload, with either path to the Error trait.
func rustTypeIsGoErrorHandle(rustType string) bool {
	return rustTypeHoldsImportedErrorPayload(rustType) ||
		strings.Contains(rustType, "Box<dyn std::error::Error")
}

// rustTypeHoldsImportedErrorPayload is the same test for the spelling the
// translated program itself uses, with the StdError import.
func rustTypeHoldsImportedErrorPayload(rustType string) bool {
	return strings.Contains(rustType, "Box<dyn StdError")
}

// rustTypeStartsWithErrorPayload is the same test for a type that must
// begin with the payload.
func rustTypeStartsWithErrorPayload(rustType string) bool {
	return strings.HasPrefix(rustType, "Box<dyn StdError") ||
		strings.HasPrefix(rustType, "Box<dyn std::error::Error")
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
