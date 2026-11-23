package main

import (
	"fmt"
	"strings"
)

// A Go table such as unicode's RangeTables is a slice literal of hundreds of
// struct literals that differ only in their numbers. Written out element by
// element, each field wrapped in its own cell, the unicode package's tables
// came to 2.7 MB of Rust in one initializer, and rustc needed more than 6 GB
// to compile them. compactHomogeneousVecLiterals rewrites such a vec! into a
// constant table of the numbers and one closure that builds an element from
// a row, which compiles in seconds.

// compactTableMinElements is the fewest elements worth a table.
const compactTableMinElements = 16

var compactTableCastTypes = map[string]bool{
	"u8": true, "u16": true, "u32": true, "u64": true, "usize": true,
	"i8": true, "i16": true, "i32": true, "i64": true, "isize": true,
}

// compactHomogeneousVecLiterals returns code with every vec![...] of at least
// compactTableMinElements elements that are the same text apart from integer
// literals cast with "as" rewritten as a table. A vec! that does not qualify
// is left as it is.
func compactHomogeneousVecLiterals(code string, goSlice bool) string {
	if len(code) < 4096 || !strings.Contains(code, "vec![") {
		return code
	}
	var out strings.Builder
	compactVecLiteralsInto(&out, code, goSlice)
	return out.String()
}

func compactVecLiteralsInto(out *strings.Builder, code string, goSlice bool) {
	i := 0
	for i < len(code) {
		start := findVecMacro(code, i)
		if start < 0 {
			out.WriteString(code[i:])
			return
		}
		open := start + len("vec![")
		close := matchingRustBracket(code, open)
		if close < 0 {
			out.WriteString(code[i:])
			return
		}
		out.WriteString(code[i:start])
		var inner strings.Builder
		compactVecLiteralsInto(&inner, code[open:close], goSlice)
		if table, ok := homogeneousVecTable(inner.String(), goSlice); ok {
			out.WriteString(table)
		} else {
			out.WriteString("vec![")
			out.WriteString(inner.String())
			out.WriteString("]")
		}
		i = close + 1
	}
}

// findVecMacro finds the next "vec![" outside a string or char literal.
func findVecMacro(code string, from int) int {
	for i := from; i < len(code); i++ {
		switch code[i] {
		case '"':
			i = skipRustString(code, i)
		case '\'':
			i = skipRustChar(code, i)
		case 'v':
			if strings.HasPrefix(code[i:], "vec![") && (i == 0 || !isRustIdentByte(code[i-1]) && code[i-1] != ':') {
				return i
			}
		}
	}
	return -1
}

// matchingRustBracket returns the index of the ] that closes the [ just
// before open, or -1.
func matchingRustBracket(code string, open int) int {
	depth := 1
	for i := open; i < len(code); i++ {
		switch code[i] {
		case '"':
			i = skipRustString(code, i)
		case '\'':
			i = skipRustChar(code, i)
		case '(', '[', '{':
			depth++
		case ')', ']', '}':
			depth--
			if depth == 0 {
				return i
			}
		}
	}
	return -1
}

// skipRustString returns the index of the closing quote of the string
// literal that starts at i.
func skipRustString(code string, i int) int {
	for j := i + 1; j < len(code); j++ {
		switch code[j] {
		case '\\':
			j++
		case '"':
			return j
		}
	}
	return len(code)
}

// skipRustChar returns the index of the closing quote of a char literal at
// i, or i itself for a lifetime such as 'static.
func skipRustChar(code string, i int) int {
	if i+2 < len(code) && code[i+1] == '\\' {
		if end := strings.IndexByte(code[i+2:], '\''); end >= 0 {
			return i + 2 + end
		}
		return i
	}
	if i+2 < len(code) && code[i+2] == '\'' {
		return i + 2
	}
	// A multi-byte char, such as 'é'.
	for j := i + 1; j < len(code) && j <= i+5; j++ {
		if code[j] == '\'' {
			if j > i+1 && code[i+1] >= 0x80 {
				return j
			}
			break
		}
	}
	return i
}

// splitTopLevelRustList splits a comma-separated list at depth zero.
func splitTopLevelRustList(list string) []string {
	var parts []string
	depth, start := 0, 0
	for i := 0; i < len(list); i++ {
		switch list[i] {
		case '"':
			i = skipRustString(list, i)
		case '\'':
			i = skipRustChar(list, i)
		case '(', '[', '{':
			depth++
		case ')', ']', '}':
			depth--
		case ',':
			if depth == 0 {
				parts = append(parts, strings.TrimSpace(list[start:i]))
				start = i + 1
			}
		}
	}
	if tail := strings.TrimSpace(list[start:]); tail != "" {
		parts = append(parts, tail)
	}
	return parts
}

// templateCastNumbers replaces each integer literal that is cast with "as"
// to an integer type by a placeholder, and returns the numbers in order.
func templateCastNumbers(elem string) ([]string, []string) {
	var pieces []string
	var numbers []string
	last := 0
	for i := 0; i < len(elem); i++ {
		c := elem[i]
		switch {
		case c == '"':
			i = skipRustString(elem, i)
			continue
		case c == '\'':
			i = skipRustChar(elem, i)
			continue
		case c >= '0' && c <= '9':
			if i > 0 && (isRustIdentByte(elem[i-1]) || elem[i-1] == '.') {
				// Part of an identifier, a suffix or a tuple index: skip it.
				for i+1 < len(elem) && isRustIdentByte(elem[i+1]) {
					i++
				}
				continue
			}
			end := i + 1
			if c == '0' && end < len(elem) && (elem[end] == 'x' || elem[end] == 'X') {
				end++
				for end < len(elem) && isHexByte(elem[end]) {
					end++
				}
			} else {
				for end < len(elem) && elem[end] >= '0' && elem[end] <= '9' {
					end++
				}
			}
			if end < len(elem) && (isRustIdentByte(elem[end]) || elem[end] == '.') {
				// A suffixed or float literal stays part of the template.
				for end < len(elem) && (isRustIdentByte(elem[end]) || elem[end] == '.') {
					end++
				}
				i = end - 1
				continue
			}
			if castType, ok := castTypeAfter(elem, end); ok && compactTableCastTypes[castType] {
				pieces = append(pieces, elem[last:i])
				numbers = append(numbers, elem[i:end])
				last = end
			}
			i = end - 1
		}
	}
	pieces = append(pieces, elem[last:])
	return pieces, numbers
}

func isHexByte(c byte) bool {
	return c >= '0' && c <= '9' || c >= 'a' && c <= 'f' || c >= 'A' && c <= 'F'
}

func castTypeAfter(s string, i int) (string, bool) {
	if !strings.HasPrefix(s[i:], " as ") {
		return "", false
	}
	j := i + len(" as ")
	k := j
	for k < len(s) && isRustIdentByte(s[k]) {
		k++
	}
	return s[j:k], k > j
}

// homogeneousVecTable writes the table form of a vec! with the given
// elements, when every element has the same template.
func homogeneousVecTable(inner string, goSlice bool) (string, bool) {
	elems := splitTopLevelRustList(inner)
	if len(elems) < compactTableMinElements {
		return "", false
	}
	var template []string
	rows := make([][]string, 0, len(elems))
	for i, elem := range elems {
		pieces, numbers := templateCastNumbers(elem)
		if len(numbers) == 0 {
			return "", false
		}
		if i == 0 {
			template = pieces
		} else if !sameStrings(template, pieces) {
			return "", false
		}
		rows = append(rows, numbers)
	}
	width := len(rows[0])
	tuple := func(items []string) string {
		if len(items) == 1 {
			return "(" + items[0] + ",)"
		}
		return "(" + strings.Join(items, ", ") + ")"
	}
	columnTypes := make([]string, width)
	names := make([]string, width)
	for i := range columnTypes {
		columnTypes[i] = "i128"
		names[i] = fmt.Sprintf("__go_c%d", i)
	}
	var body strings.Builder
	body.WriteString(template[0])
	for i, piece := range template[1:] {
		body.WriteString(names[i])
		body.WriteString(piece)
	}
	var table strings.Builder
	table.WriteString("{ const __GO_TABLE: [")
	table.WriteString(tuple(columnTypes))
	table.WriteString(fmt.Sprintf("; %d] = [", len(rows)))
	for i, row := range rows {
		if i > 0 {
			table.WriteString(", ")
		}
		table.WriteString(tuple(row))
	}
	table.WriteString("]; let __go_rows: std::vec::Vec<_> = __GO_TABLE.iter().map(|&")
	table.WriteString(tuple(names))
	table.WriteString("| ")
	table.WriteString(body.String())
	table.WriteString(").collect(); ")
	if goSlice {
		table.WriteString("GoSlice::from(__go_rows)")
	} else {
		table.WriteString("__go_rows")
	}
	table.WriteString(" }")
	return table.String(), true
}

func sameStrings(a, b []string) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}
