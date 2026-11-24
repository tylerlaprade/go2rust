package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// A bridged struct can hold an io.Writer or io.Reader in a field, as
// exec.Cmd's Stdout, Stderr and Stdin do. When io is translated from
// source its interfaces are traits of the io crate, which the stub crate
// cannot name (the io crate depends on it). The field then holds a
// GoStubWriter or GoStubReader instead: a stub-side handle over a closure,
// written where the field is assigned, that calls the io trait method.

const (
	stubWriterTypeName = "GoStubWriter"
	stubReaderTypeName = "GoStubReader"
)

// stubIoFieldBridgeType returns the stub-side type that holds a field of
// type t in a bridged struct, or "" when t needs none.
func stubIoFieldBridgeType(t types.Type) string {
	named, ok := types.Unalias(t).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return ""
	}
	if named.Obj().Pkg().Path() != "io" || !isSourceMappedPackagePath("io") || !NeedsConcurrentWrapper() {
		return ""
	}
	switch named.Obj().Name() {
	case "Writer":
		return stubWriterTypeName
	case "Reader":
		return stubReaderTypeName
	}
	return ""
}

// stubIoFieldAssignmentBridge reports which stub-side type an assignment
// to lhs, a field of a bridged struct, must store.
func stubIoFieldAssignmentBridge(lhs ast.Expr) string {
	sel, ok := lhs.(*ast.SelectorExpr)
	if !ok {
		return ""
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return ""
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok || selection.Kind() != types.FieldVal {
		return ""
	}
	named, ok := externalSelectorReceiverNamed(selection.Recv())
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil || !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return ""
	}
	bridge := stubIoFieldBridgeType(selection.Obj().Type())
	if bridge != "" {
		RegisterExternalSelectorField(sel)
	}
	return bridge
}

// writeStubIoFieldBridgeValue writes, for __iface_value (the Option of the
// boxed io trait object being assigned), the stub-side handle to store.
func writeStubIoFieldBridgeValue(out *strings.Builder, bridge string) {
	outer := GetOuterWrapperType()
	inner := GetInnerWrapperType()
	wrap := func(value string) string {
		return outer + "::new(" + inner + "::new(Some(" + value + ")))"
	}
	borrow := func(cell string) string {
		var b strings.Builder
		b.WriteString(cell)
		WriteBorrowMethod(&b, false)
		return b.String()
	}
	out.WriteString("__iface_value.map(|__io| go2rust_stdlib_stubs::")
	out.WriteString(bridge)
	out.WriteString("::__go_from_fn({ let __io = std::sync::Mutex::new(__io); ")
	if bridge == stubWriterTypeName {
		out.WriteString("move |__data: &[u8]| { let _ = __io.lock().unwrap().write(")
		out.WriteString(wrap("Vec::from(__data)"))
		out.WriteString("); }")
	} else {
		out.WriteString("move || { let mut __all: std::vec::Vec<u8> = std::vec::Vec::new(); loop { let __buf = ")
		out.WriteString(wrap("Vec::from(&[0u8; 4096][..])"))
		out.WriteString("; let (__n, __err) = __io.lock().unwrap().read(__buf.clone()); if __n > 0 { __all.extend_from_slice(&")
		out.WriteString(borrow("__buf"))
		out.WriteString(".as_ref().unwrap().as_slice()[..__n as usize]); } if ")
		out.WriteString(borrow("__err"))
		out.WriteString(".is_some() { break; } } __all }")
	}
	out.WriteString(" }))")
}

// externalStubsNeedIoFieldBridges reports which bridge types the stub
// fields use.
func externalStubsNeedIoFieldBridges(fieldsByType map[string]map[string]string) (writer bool, reader bool) {
	for _, fields := range fieldsByType {
		for _, fieldType := range fields {
			if strings.Contains(fieldType, stubWriterTypeName) {
				writer = true
			}
			if strings.Contains(fieldType, stubReaderTypeName) {
				reader = true
			}
		}
	}
	return writer, reader
}

// PERMANENT: not scaffold — the stub crate cannot depend on the io crate
// translated from source, so a bridged struct's io.Writer field holds this
// closure over the io trait object instead.
func writeStubIoFieldBridges(out *strings.Builder, writer bool, reader bool) {
	if writer {
		out.WriteString(`/// An io.Writer held by a bridged struct, such as exec.Cmd's Stdout.
#[derive(Clone, Default)]
pub struct GoStubWriter {
    pub __go_write: Option<std::sync::Arc<dyn Fn(&[u8]) + Send + Sync>>,
}

impl GoStubWriter {
    pub fn __go_from_fn<F: Fn(&[u8]) + Send + Sync + 'static>(write: F) -> Self {
        GoStubWriter { __go_write: Some(std::sync::Arc::new(write)) }
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        if let Some(write) = &self.__go_write {
            write(data);
        }
    }
}

impl std::fmt::Debug for GoStubWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<io.Writer>")
    }
}

impl std::fmt::Display for GoStubWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<io.Writer>")
    }
}

`)
	}
	if reader {
		out.WriteString(`/// An io.Reader held by a bridged struct, such as exec.Cmd's Stdin.
#[derive(Clone, Default)]
pub struct GoStubReader {
    pub __go_read_all: Option<std::sync::Arc<dyn Fn() -> std::vec::Vec<u8> + Send + Sync>>,
}

impl GoStubReader {
    pub fn __go_from_fn<F: Fn() -> std::vec::Vec<u8> + Send + Sync + 'static>(read_all: F) -> Self {
        GoStubReader { __go_read_all: Some(std::sync::Arc::new(read_all)) }
    }

    /// Reads the reader to its end, as exec.Cmd copies Stdin to the process.
    pub fn __go_read_all(&self) -> std::vec::Vec<u8> {
        match &self.__go_read_all {
            Some(read_all) => read_all(),
            None => std::vec::Vec::new(),
        }
    }
}

impl std::fmt::Debug for GoStubReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<io.Reader>")
    }
}

impl std::fmt::Display for GoStubReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<io.Reader>")
    }
}

`)
	}
}
