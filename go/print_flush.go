package main

import "regexp"

// Go's os.Stdout is not buffered: every fmt.Print reaches it at once. Rust's
// stdout is line-buffered, so text without a newline waited for one or for
// the program's end. A prompt such as fmt.Print("Name: ") then showed only
// after the input it asked for, and a partial line came after a write to
// stderr that Go prints after it. A complete line is flushed by the line
// buffering already; goPrintFlushMacro shadows print!, which writes the
// partial ones, with one that flushes after it writes.
const goPrintFlushMacro = `#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}
`

var stdoutPrintMacroUse = regexp.MustCompile(`(^|[^A-Za-z0-9_])print!\(`)

// usesStdoutPrintMacro reports whether rust calls print!, which the file
// then shadows with goPrintFlushMacro.
func usesStdoutPrintMacro(rust string) bool {
	return stdoutPrintMacroUse.MatchString(rust)
}
