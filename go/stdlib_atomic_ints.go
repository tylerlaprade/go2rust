package main

import (
	"go/ast"
	"strings"
)

// sync/atomic is native tier: its Go source is assembly and compiler
// intrinsics, so these emitters are its permanent implementation. Each one
// works on the cell the pointer argument names, under the cell's lock, which
// makes the operation atomic with respect to every other access to it.

// atomicIntTypes maps the suffix of an atomic function to its Rust type.
var atomicIntTypes = map[string]string{
	"Int32":   "i32",
	"Int64":   "i64",
	"Uint32":  "u32",
	"Uint64":  "u64",
	"Uintptr": "usize",
}

func init() {
	for suffix, rustType := range atomicIntTypes {
		rustType := rustType
		if _, ok := stdlibMappings["sync/atomic.Add"+suffix]; !ok {
			stdlibMappings["sync/atomic.Add"+suffix] = func(out *strings.Builder, call *ast.CallExpr) {
				transpileAtomicIntAdd(out, call, rustType)
			}
		}
		if _, ok := stdlibMappings["sync/atomic.Load"+suffix]; !ok {
			stdlibMappings["sync/atomic.Load"+suffix] = func(out *strings.Builder, call *ast.CallExpr) {
				transpileAtomicIntLoad(out, call)
			}
		}
		if _, ok := stdlibMappings["sync/atomic.Store"+suffix]; !ok {
			stdlibMappings["sync/atomic.Store"+suffix] = func(out *strings.Builder, call *ast.CallExpr) {
				transpileAtomicIntStore(out, call, rustType)
			}
		}
		stdlibMappings["sync/atomic.Swap"+suffix] = func(out *strings.Builder, call *ast.CallExpr) {
			transpileAtomicIntSwap(out, call, rustType)
		}
		stdlibMappings["sync/atomic.CompareAndSwap"+suffix] = func(out *strings.Builder, call *ast.CallExpr) {
			transpileAtomicIntCompareAndSwap(out, call, rustType)
		}
	}
}

func writeAtomicIntGuard(out *strings.Builder, mutable bool) {
	if NeedsConcurrentWrapper() {
		out.WriteString("let mut __guard = __target.lock().unwrap(); ")
	} else if mutable {
		out.WriteString("let mut __guard = __target.borrow_mut(); ")
	} else {
		out.WriteString("let __guard = __target.borrow(); ")
	}
}

func writeAtomicIntOperand(out *strings.Builder, expr ast.Expr, rustType string) {
	out.WriteString("(")
	writeNumericConversionValue(out, expr)
	out.WriteString(") as ")
	out.WriteString(rustType)
}

// Add returns the new value; like Go's, the sum wraps around.
func transpileAtomicIntAdd(out *strings.Builder, call *ast.CallExpr, rustType string) {
	if len(call.Args) != 2 {
		out.WriteString("unimplemented!(\"atomic Add takes a pointer and a delta\")")
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __target = ")
	writeAtomicTarget(out, call.Args[0])
	out.WriteString("; let __delta = ")
	writeAtomicIntOperand(out, call.Args[1], rustType)
	out.WriteString("; ")
	writeAtomicIntGuard(out, true)
	out.WriteString("let __value = __guard.as_mut().unwrap(); *__value = __value.wrapping_add(__delta); *__value }")
	WriteWrapperSuffix(out)
}

func transpileAtomicIntLoad(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 1 {
		out.WriteString("unimplemented!(\"atomic Load takes a pointer\")")
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __target = ")
	writeAtomicTarget(out, call.Args[0])
	out.WriteString("; ")
	writeAtomicIntGuard(out, false)
	out.WriteString("*__guard.as_ref().unwrap() }")
	WriteWrapperSuffix(out)
}

func transpileAtomicIntStore(out *strings.Builder, call *ast.CallExpr, rustType string) {
	if len(call.Args) != 2 {
		out.WriteString("unimplemented!(\"atomic Store takes a pointer and a value\")")
		return
	}
	out.WriteString("{ let __target = ")
	writeAtomicTarget(out, call.Args[0])
	out.WriteString("; let __stored = ")
	writeAtomicIntOperand(out, call.Args[1], rustType)
	out.WriteString("; ")
	writeAtomicIntGuard(out, true)
	out.WriteString("*__guard.as_mut().unwrap() = __stored; }")
}

// Swap stores the new value and returns the old one.
func transpileAtomicIntSwap(out *strings.Builder, call *ast.CallExpr, rustType string) {
	if len(call.Args) != 2 {
		out.WriteString("unimplemented!(\"atomic Swap takes a pointer and a value\")")
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __target = ")
	writeAtomicTarget(out, call.Args[0])
	out.WriteString("; let __stored = ")
	writeAtomicIntOperand(out, call.Args[1], rustType)
	out.WriteString("; ")
	writeAtomicIntGuard(out, true)
	out.WriteString("std::mem::replace(__guard.as_mut().unwrap(), __stored) }")
	WriteWrapperSuffix(out)
}

// CompareAndSwap stores new only when the value is old, and reports whether
// it did.
func transpileAtomicIntCompareAndSwap(out *strings.Builder, call *ast.CallExpr, rustType string) {
	if len(call.Args) != 3 {
		out.WriteString("unimplemented!(\"atomic CompareAndSwap takes a pointer, an old and a new value\")")
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __target = ")
	writeAtomicTarget(out, call.Args[0])
	out.WriteString("; let __old = ")
	writeAtomicIntOperand(out, call.Args[1], rustType)
	out.WriteString("; let __new = ")
	writeAtomicIntOperand(out, call.Args[2], rustType)
	out.WriteString("; ")
	writeAtomicIntGuard(out, true)
	out.WriteString("let __value = __guard.as_mut().unwrap(); if *__value == __old { *__value = __new; true } else { false } }")
	WriteWrapperSuffix(out)
}
