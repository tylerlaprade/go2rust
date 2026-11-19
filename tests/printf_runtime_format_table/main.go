package main

import "fmt"

// sprintf receives its format as a value, so the format is only known when
// the program runs.
func sprintf(format string, args ...interface{}) string {
	return fmt.Sprintf(format, args...)
}

func show(format string, text string) {
	fmt.Println(format, "=>", "["+text+"]")
}

func main() {
	intFormats := []string{
		"%d", "%5d", "%-5d|", "%05d", "%+d", "% d", "%x", "%X", "%#x", "%o", "%#o", "%O", "%b",
		"%c", "%q", "%U", "%#U", "%v", "%3v", "%.3d", "%8.3d", "%08.3d", "%-8.3d|", "%+05d", "%.0d",
		"%s", "%t", "%f", "%e", "%T",
	}
	ints := []int{0, 7, -42, 65, 255, 123456789}
	for _, format := range intFormats {
		for _, value := range ints {
			show(format, sprintf(format, value))
		}
	}
	show("%d %d", sprintf("%d %d", int64(-9000000000), uint8(200)))
	show("%x %X %#x", sprintf("%x %X %#x", int64(-255), uint8(200), int64(4096)))

	floatFormats := []string{
		"%v", "%g", "%G", "%e", "%E", "%f", "%F", "%.2f", "%8.2f|", "%-8.2f|", "%08.2f", "%+.2f",
		"% .2f", "%.0f", "%.3e", "%.3g", "%.10g", "%.1g", "%10.3g|", "%6.2f", "%d", "%s", "%T",
		"%5v|", "%.0e", "%.0g",
	}
	floats := []float64{
		0.0, 1.0, 2.5, -2.5, 0.1, 100000.0, 1000000.0, 1234567.0, 123456789.0, 1e20, 1e21, 1e-4,
		1e-5, 0.000123456, 3.141592653589793, 1e100, 123.456, 0.5, 1.5, 2.675, 99.995,
	}
	for _, format := range floatFormats {
		for _, value := range floats {
			show(format, sprintf(format, value))
		}
	}
	narrowFormats := []string{"%v", "%g", "%f", "%.2f", "%e", "%T"}
	for _, format := range narrowFormats {
		show(format, sprintf(format, float32(0.1)))
		show(format, sprintf(format, float32(2.5)))
		show(format, sprintf(format, float32(16777216.0)))
		show(format, sprintf(format, float32(3.1415927)))
		show(format, sprintf(format, float32(1e10)))
		show(format, sprintf(format, float32(1e-5)))
	}

	stringFormats := []string{
		"%v", "%s", "%q", "%10s|", "%-10s|", "%.2s", "%10.2s|", "%x", "%X", "% x", "%#x", "%d",
		"%t", "%T", "%05s", "%#q", "%c",
	}
	texts := []string{"hi", "", "héllo wörld", "tab\there", "quote\"d", "back\\slash", "new\nline", "bell\a", "`tick`"}
	for _, format := range stringFormats {
		for i := 0; i < len(texts); i++ {
			show(format, sprintf(format, texts[i]))
		}
	}

	boolFormats := []string{"%v", "%t", "%5t|", "%-6t|", "%d", "%s", "%T", "%q"}
	for _, format := range boolFormats {
		show(format, sprintf(format, true))
		show(format, sprintf(format, false))
	}

	show("plain text", sprintf("plain text"))
	show("100%%", sprintf("100%%"))
	show("%d%%", sprintf("%d%%", 50))
	show("%d %s %v %t", sprintf("%d %s %v %t", 1, "two", 3.5, true))
	show("%d %d", sprintf("%d %d", 1))
	show("%d", sprintf("%d", 1, 2, "x"))
	show("%!", sprintf("%!", 1))
	show("%z", sprintf("%z", 1))
	show("trailing %", sprintf("trailing %"))
	show("%5", sprintf("%5", 1))
	show("%*d|", sprintf("%*d|", 6, 42))
	show("%-*d|", sprintf("%-*d|", 6, 42))
	show("%.*f", sprintf("%.*f", 2, 3.14159))
	show("%*.*f|", sprintf("%*.*f|", 8, 3, 3.14159))
	show("%*d", sprintf("%*d", "x", 42))
	show("%w", sprintf("%w", "wrapped"))
	show("%v %v", sprintf("%v %v", 1))
	show("%-5d|%5s|%05.1f", sprintf("%-5d|%5s|%05.1f", 3, "ab", 2.25))

	// A negative width from an argument means left alignment. A negative
	// precision is reported and then ignored.
	show("%*d|", sprintf("%*d|", -6, 42))
	show("%*s|", sprintf("%*s|", -6, "ab"))
	show("%-*d|", sprintf("%-*d|", -6, 42))
	show("%.*f|", sprintf("%.*f|", -2, 3.14159))

	// Precision counts bytes for %x and characters for %s.
	show("%.1x", sprintf("%.1x", "é"))
	show("%.3x", sprintf("%.3x", "héllo"))
	show("%.2X", sprintf("%.2X", "héllo"))
	show("%.3s", sprintf("%.3s", "héllo"))
	show("% .2x", sprintf("% .2x", "héllo"))

	// %#v writes a value the way Go source does. %+v changes nothing for
	// a number or a string.
	show("%#v", sprintf("%#v", "hello"))
	show("%#v", sprintf("%#v", "new\nline"))
	show("%#v", sprintf("%#v", 42))
	show("%#v", sprintf("%#v", int64(-5)))
	show("%#v", sprintf("%#v", uint8(200)))
	show("%#v", sprintf("%#v", 2.5))
	show("%#v", sprintf("%#v", 1000000.0))
	show("%#v", sprintf("%#v", true))
	show("%#10v|", sprintf("%#10v|", "hi"))
	show("%+v", sprintf("%+v", "hello"))
	show("%+v", sprintf("%+v", 42))

	// Zero padding and a precision count digits, and 0x, 0b or 0o comes on
	// top of them. The 0 that # puts before an octal number is not added to
	// a number that already starts with one. A precision of zero prints
	// nothing for zero.
	prefixFormats := []string{
		"%#08x", "%#08X", "%#08b", "%#08o", "%08O", "%#8x|", "%#-8x|", "%#-08x|", "%#.3o", "%#.1o",
		"%#.3x", "%#8.3x|", "%+08d", "% 08d", "%+.0d|", "%#.0x|", "%5.0d|", "%#o", "%#b", "%O",
		"%#O", "%#.3O", "%#08O",
	}
	prefixValues := []int{0, 7, 8, -42, 255}
	for _, format := range prefixFormats {
		for _, value := range prefixValues {
			show(format, sprintf(format, value))
		}
	}

	// A width or a precision from an argument can be any integer type.
	show("%*d|", sprintf("%*d|", uint32(6), 42))
	show("%*d|", sprintf("%*d|", int8(-6), 42))
	show("%*d|", sprintf("%*d|", int64(6), 42))
	show("%*d|", sprintf("%*d|", uint64(6), 42))
	show("%*d|", sprintf("%*d|", int16(6), 42))
	show("%.*f", sprintf("%.*f", uint8(2), 3.14159))
	show("%.*f", sprintf("%.*f", uint(3), 3.14159))
	show("%*d|", sprintf("%*d|", 2.5, 42))

	// %+q writes characters outside ASCII as escapes.
	show("%+q", sprintf("%+q", "é"))
	show("%+q", sprintf("%+q", "a\tb☺"))
	show("%+q", sprintf("%+q", "😀"))
	show("%+q", sprintf("%+q", "plain"))
	show("%+q", sprintf("%+q", 'é'))
	show("%+q", sprintf("%+q", 'x'))
	show("%+q", sprintf("%+q", '😀'))
	show("%q", sprintf("%q", "é☺"))
	show("%#U", sprintf("%#U", '\''))
	show("%#U", sprintf("%#U", '\\'))

	// Zero padding applies to a character, a quoted character, a bool and
	// the name of a type as it does to a string.
	show("%05c|%05q|%-05c|%05U", sprintf("%05c|%05q|%-05c|%05U", 65, 65, 65, 65))
	show("%05c|%05q", sprintf("%05c|%05q", 'é', 'é'))
	show("%05T|%05t", sprintf("%05T|%05t", 65, true))

	// %q writes a character that cannot be seen as an escape.
	show("%q", sprintf("%q", "zero\u200bwidth"))
	show("%q", sprintf("%q", "soft\u00adhyphen"))
	show("%q", sprintf("%q", "\ufeffmark"))
	show("%q", sprintf("%q", "no\u00a0break"))
	show("%q", sprintf("%q", '\u200b'))
	show("%#v", sprintf("%#v", "zero\u200bwidth"))
	show("%#U", sprintf("%#U", '\u200b'))
	show("%#U", sprintf("%#U", ' '))

	// %q writes a byte that is not UTF-8 as an escape too.
	broken := []byte{0xff, 0x41, 0xc3, 0xa9}
	show("%q", sprintf("%q", broken))
	show("%+q", sprintf("%+q", broken))
	show("%x", sprintf("%x", broken))
	show("%#q", sprintf("%#q", broken))
	show("%.1q", sprintf("%.1q", broken))
	show("%.3q", sprintf("%.3q", broken))
	show("%12q|", sprintf("%12q|", broken))

	// A precision is applied before %#q chooses its quotes, so a byte that
	// is cut off has no say in them.
	show("%#.1q", sprintf("%#.1q", []byte{0x41, 0xff}))
	show("%#.1q", sprintf("%#.1q", []byte{0x41, 0x42}))
	show("%#.2q", sprintf("%#.2q", []byte{0x41, 0xff}))
	show("%.2s|", sprintf("%.2s|", []byte{0x41, 0x42, 0xff}))
	show("%.3q", sprintf("%.3q", []byte{0xe2, 0x82, 0x41, 0x42}))

	// %#q writes a raw string unless the text has a backquote, an ASCII
	// control character other than tab, or a byte order mark.
	show("%#q", sprintf("%#q", "\ufeffmark"))
	show("%#q", sprintf("%#q", "next\u0085line"))
	show("%#q", sprintf("%#q", "del\x7f"))
	show("%#q", sprintf("%#q", "zero\u200bwidth"))

	// A precision above four gives %U more digits.
	show("%.8U|%.2U|%#.6U|%10.6U|", sprintf("%.8U|%.2U|%#.6U|%10.6U|", 65, 65, 65, 65))

	// Infinity keeps its sign and is never padded with zeros. NaN has no
	// sign unless a flag asks for one.
	var zero float64
	edges := []float64{1 / zero, -1 / zero, zero / zero}
	edgeFormats := []string{
		"%f", "%+f", "% f", "%+ f", "%8.2f|", "%-8.2f|", "%08.2f|", "% 08.2f|", "%v", "%+v", "%g",
		"%e", "%E", "%G", "%.0f",
	}
	for _, format := range edgeFormats {
		for _, value := range edges {
			show(format, sprintf(format, value))
		}
	}

	// A []byte prints as text with %s, %q and %x, and as numbers with %v.
	data := []byte("Hello")
	show("%s", sprintf("%s", data))
	show("%q", sprintf("%q", data))
	show("%x", sprintf("%x", data))
	show("%X", sprintf("%X", data))
	show("% x", sprintf("% x", data))
	show("%v", sprintf("%v", data))
	show("%s", sprintf("%s", []byte("direct")))
	show("%10s|", sprintf("%10s|", data))
	show("%.2s", sprintf("%.2s", data))

	// A verb that does not print a []byte as text is applied to each byte.
	pair := []byte{1, 2}
	show("%d", sprintf("%d", data))
	show("%c", sprintf("%c", data))
	show("%5v", sprintf("%5v", pair))
	show("%03d|%-4d|%+d", sprintf("%03d|%-4d|%+d", pair, pair, pair))
	show("%o %b %U", sprintf("%o %b %U", []byte{8}, []byte{5}, []byte{65}))
	show("%t", sprintf("%t", pair))
	show("%w", sprintf("%w", pair))
	show("%v|%d", sprintf("%v|%d", []byte{}, []byte{}))
}
