package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// A channel of a transpiled named interface, or of the empty interface,
// carries the Option inside the interface handle, as a channel of error
// does: nil can be sent, a receive from a closed channel gives nil, and the
// receiver gets a handle of its own holding the value that was sent.

// channelInterfaceElementType returns the element type of a channel that
// carries values of a transpiled named interface or of the empty interface.
func channelInterfaceElementType(ch ast.Expr) (types.Type, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || ch == nil {
		return nil, false
	}
	typ := typeInfo.GetType(ch)
	if typ == nil {
		return nil, false
	}
	chanType, ok := types.Unalias(typ).Underlying().(*types.Chan)
	if !ok {
		return nil, false
	}
	if !isInterfaceChannelElemType(chanType.Elem()) {
		return nil, false
	}
	return chanType.Elem(), true
}

// isInterfaceChannelElemType reports whether a channel of elem carries the
// Option inside an interface handle: elem is a transpiled named interface
// or the empty interface.
func isInterfaceChannelElemType(elem types.Type) bool {
	if elem == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(elem); ok {
		return true
	}
	return isEmptyInterfaceType(elem)
}

// channelElementIsHandleOption reports whether a channel carries the Option
// of an interface handle: a channel of error or of a transpiled named
// interface.
func channelElementIsHandleOption(ch ast.Expr) bool {
	if channelElementIsGoError(ch) {
		return true
	}
	_, ok := channelInterfaceElementType(ch)
	return ok
}

// channelReceiveFillsHandleOf reports whether a value received from ch can be
// stored as it is into the handle of a variable of type target.
func channelReceiveFillsHandleOf(ch ast.Expr, target types.Type) bool {
	if target == nil {
		return false
	}
	if channelElementIsGoError(ch) {
		return isGoErrorType(target)
	}
	elem, ok := channelInterfaceElementType(ch)
	return ok && types.Identical(elem, target)
}

// channelHandleOptionPayloadType returns the type inside the Option that a
// channel of error or of an interface carries.
func channelHandleOptionPayloadType(ch ast.Expr) string {
	if channelElementIsGoError(ch) {
		return rustErrorPayloadType()
	}
	elem, _ := channelInterfaceElementType(ch)
	return goTypesTypeToRust(elem)
}

// interfaceChannelElemTypeToRust returns the Rust element type of a channel
// of a transpiled named interface.
func interfaceChannelElemTypeToRust(elem types.Type) (string, bool) {
	if !isInterfaceChannelElemType(elem) {
		return "", false
	}
	return "Option<" + goTypesTypeToRust(elem) + ">", true
}

// channelElemTypeExprToRust returns the Rust element type of a channel type
// written in the source.
func channelElemTypeExprToRust(elem ast.Expr) string {
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if rustType, ok := interfaceChannelElemTypeToRust(typeInfo.GetType(elem)); ok {
			return rustType
		}
	}
	return goTypeToRustBase(elem)
}

// writeEmptyChannelHandle writes a nil handle for a value received from a
// closed channel of error or of an interface.
func writeEmptyChannelHandle(out *strings.Builder, ch ast.Expr) {
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(Mutex::new(None::<")
	} else {
		out.WriteString("Rc::new(RefCell::new(None::<")
	}
	out.WriteString(channelHandleOptionPayloadType(ch))
	out.WriteString(">))")
}

// writeInterfaceChannelSendValue writes the value that `ch <- value` sends on
// a channel of an interface: the Option inside a handle holding value. The
// handle's lock is released before the send, which may block.
func writeInterfaceChannelSendValue(out *strings.Builder, ch ast.Expr, value ast.Expr) bool {
	elem, ok := channelInterfaceElementType(ch)
	if !ok {
		return false
	}
	if ident, ok := value.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("None::<")
		out.WriteString(goTypesTypeToRust(elem))
		out.WriteString(">")
		return true
	}
	if isEmptyInterfaceType(elem) {
		// The value goes into a fresh interface{} handle, as in an
		// assignment to an interface{} variable; the send takes the Option
		// out of it, since a boxed value cannot be cloned.
		var handle strings.Builder
		if !writeEmptyInterfaceHandleClone(&handle, value) {
			WriteWrapperPrefix(&handle)
			writeInterfaceBoxedValue(&handle, value)
			WriteWrapperSuffix(&handle)
		}
		out.WriteString("{ let __iface_handle = ")
		out.WriteString(handle.String())
		out.WriteString("; let __iface_value = __iface_handle")
		WriteBorrowMethod(out, true)
		out.WriteString(".take(); __iface_value }")
		return true
	}
	var handle strings.Builder
	if !writeLocalInterfaceWrappedValue(&handle, value, elem) {
		return false
	}
	out.WriteString("{ let __iface_handle = ")
	out.WriteString(handle.String())
	out.WriteString("; let __iface_value = (*__iface_handle")
	WriteBorrowMethod(out, false)
	out.WriteString(").clone(); __iface_value }")
	return true
}

// writeInterfaceChannelReceiveHandle writes a new handle holding a value
// received with `<-ch` from a channel of an interface.
func writeInterfaceChannelReceiveHandle(out *strings.Builder, expr ast.Expr) bool {
	unary, ok := expr.(*ast.UnaryExpr)
	if !ok || unary.Op.String() != "<-" {
		return false
	}
	if _, ok := channelInterfaceElementType(unary.X); !ok {
		return false
	}
	writeErrorHandleFromChannelReceive(out, unary.X)
	return true
}

// writeChannelReceiveHandleFor writes `<-ch` as the handle a place of type
// target holds, when the channel carries the Option inside such a handle:
// the received value fills a new handle, which is stored as it is.
func writeChannelReceiveHandleFor(out *strings.Builder, expr ast.Expr, target types.Type) bool {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op.String() != "<-" || !channelReceiveFillsHandleOf(unary.X, target) {
		return false
	}
	writeErrorHandleFromChannelReceive(out, unary.X)
	return true
}
