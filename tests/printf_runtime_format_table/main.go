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
}
