package main

// asyncStdlibFunctions contains stdlib functions that spawn goroutines
// or pass user data to goroutines. When these are called, we must use
// Arc<Mutex<>> instead of Rc<RefCell<>> for thread safety.
var asyncStdlibFunctions = map[string]bool{
	// net/http - HTTP server functions spawn goroutines for handlers
	"net/http.HandleFunc":        true,
	"net/http.Handle":            true,
	"net/http.ListenAndServe":    true,
	"net/http.Serve":             true,
	"net/http.ServeTLS":          true,
	"net/http.ListenAndServeTLS": true,

	// time - Timer/Ticker callbacks run in goroutines
	"time.AfterFunc": true,
	"time.NewTicker": true,
	"time.NewTimer":  true,
	"time.Tick":      true,

	// context - Often used with goroutines
	"context.WithCancel":   true,
	"context.WithTimeout":  true,
	"context.WithDeadline": true,

	// sync - Concurrent primitives
	"sync.Once.Do": true, // Callback might be called from multiple goroutines

	// signal - Signal handlers run in goroutines
	"os/signal.Notify": true,
	"signal.Notify":    true,
}

// syncStdlibFunctions contains stdlib functions known to be synchronous.
// These functions don't spawn goroutines and don't pass user data to
// goroutines, so they're safe to use with Rc<RefCell<>>.
var syncStdlibFunctions = map[string]bool{
	// fmt - All formatting functions are synchronous
	"fmt.Print":    true,
	"fmt.Printf":   true,
	"fmt.Println":  true,
	"fmt.Sprint":   true,
	"fmt.Sprintf":  true,
	"fmt.Sprintln": true,
	"fmt.Fprint":   true,
	"fmt.Fprintf":  true,
	"fmt.Fprintln": true,
	"fmt.Errorf":   true,
	"fmt.Scan":     true,
	"fmt.Scanf":    true,
	"fmt.Scanln":   true,
	"fmt.Sscan":    true,
	"fmt.Sscanf":   true,
	"fmt.Sscanln":  true,
	"fmt.Fscan":    true,
	"fmt.Fscanf":   true,
	"fmt.Fscanln":  true,

	// strings - All string functions are synchronous
	"strings.Contains":   true,
	"strings.HasPrefix":  true,
	"strings.HasSuffix":  true,
	"strings.Index":      true,
	"strings.Join":       true,
	"strings.Split":      true,
	"strings.Replace":    true,
	"strings.ReplaceAll": true,
	"strings.ToLower":    true,
	"strings.ToUpper":    true,
	"strings.Trim":       true,
	"strings.TrimSpace":  true,
	"strings.Fields":     true,
	"strings.Repeat":     true,
	"strings.Builder":    true,

	// strconv - All conversion functions are synchronous
	"strconv.Atoi":        true,
	"strconv.Itoa":        true,
	"strconv.ParseInt":    true,
	"strconv.ParseUint":   true,
	"strconv.ParseFloat":  true,
	"strconv.ParseBool":   true,
	"strconv.FormatInt":   true,
	"strconv.FormatUint":  true,
	"strconv.FormatFloat": true,
	"strconv.FormatBool":  true,
	"strconv.Quote":       true,
	"strconv.Unquote":     true,

	// math - All math functions are synchronous
	"math.Abs":   true,
	"math.Ceil":  true,
	"math.Floor": true,
	"math.Round": true,
	"math.Max":   true,
	"math.Min":   true,
	"math.Pow":   true,
	"math.Sqrt":  true,
	"math.Sin":   true,
	"math.Cos":   true,
	"math.Tan":   true,
	"math.Log":   true,
	"math.Exp":   true,

	// io - Basic I/O is synchronous
	"io.Copy":        true,
	"io.CopyN":       true,
	"io.ReadAll":     true,
	"io.ReadFull":    true,
	"io.WriteString": true,
	"io.ReadAtLeast": true,

	// os - File operations are synchronous
	"os.Open":      true,
	"os.Create":    true,
	"os.Remove":    true,
	"os.RemoveAll": true,
	"os.Rename":    true,
	"os.Stat":      true,
	"os.Mkdir":     true,
	"os.MkdirAll":  true,
	"os.Getenv":    true,
	"os.Setenv":    true,
	"os.Exit":      true,
	"os.Getwd":     true,
	"os.Chdir":     true,

	// bytes - All bytes functions are synchronous
	"bytes.Equal":    true,
	"bytes.Compare":  true,
	"bytes.Contains": true,
	"bytes.Index":    true,
	"bytes.Join":     true,
	"bytes.Split":    true,
	"bytes.Replace":  true,
	"bytes.Buffer":   true,

	// sort - Sorting is synchronous
	"sort.Ints":        true,
	"sort.Strings":     true,
	"sort.Float64s":    true,
	"sort.Sort":        true,
	"sort.Slice":       true,
	"sort.SliceStable": true,
	"sort.Search":      true,

	// json - JSON operations are synchronous
	"encoding/json.Marshal":       true,
	"encoding/json.MarshalIndent": true,
	"encoding/json.Unmarshal":     true,
	"json.Marshal":                true,
	"json.MarshalIndent":          true,
	"json.Unmarshal":              true,

	// time - Most time functions are synchronous
	"time.Now":           true,
	"time.Since":         true,
	"time.Until":         true,
	"time.Parse":         true,
	"time.ParseDuration": true,
	"time.Sleep":         true, // Blocks but doesn't spawn goroutines
	"time.Date":          true,
	"time.Unix":          true,

	// errors - Error creation is synchronous
	"errors.New":    true,
	"errors.Unwrap": true,
	"errors.Is":     true,
	"errors.As":     true,

	// reflect - Reflection is synchronous
	"reflect.TypeOf":  true,
	"reflect.ValueOf": true,

	// regexp - Regex operations are synchronous
	"regexp.Compile":          true,
	"regexp.CompilePOSIX":     true,
	"regexp.MustCompile":      true,
	"regexp.MustCompilePOSIX": true,
	"regexp.MatchString":      true,
	"regexp.Match":            true,
}

// isAsyncStdlibFunction checks if a function call might spawn goroutines
func isAsyncStdlibFunction(pkgName, funcName string) bool {
	// Check the fully qualified name
	fullName := pkgName + "." + funcName

	// If it's explicitly marked as async, it's async
	if asyncStdlibFunctions[fullName] {
		return true
	}

	// If it's explicitly marked as sync, it's safe
	if syncStdlibFunctions[fullName] {
		return false
	}

	// For unknown stdlib functions, be conservative and assume they might be async
	// This handles cases where we haven't catalogued every function
	return isStdlibPackage(pkgName)
}

// isStdlibPackage checks if a package is part of the Go standard library
func isStdlibPackage(pkgName string) bool {
	// List of known stdlib packages (not exhaustive, but covers common ones)
	stdlibPackages := map[string]bool{
		"fmt": true, "strings": true, "strconv": true, "math": true,
		"io": true, "os": true, "bytes": true, "sort": true,
		"time": true, "errors": true, "reflect": true, "regexp": true,
		"net": true, "net/http": true, "net/url": true,
		"encoding/json": true, "encoding/xml": true, "encoding/base64": true,
		"crypto": true, "crypto/rand": true, "crypto/sha256": true,
		"sync": true, "sync/atomic": true,
		"context": true, "database/sql": true,
		"path": true, "path/filepath": true,
		"runtime": true, "unsafe": true,
		"log": true, "flag": true,
		"bufio": true, "compress/gzip": true,
		"container/list": true, "container/heap": true,
		"debug/elf": true, "debug/dwarf": true,
		"image": true, "image/png": true, "image/jpeg": true,
		"testing": true, "testing/quick": true,
	}

	return stdlibPackages[pkgName]
}
