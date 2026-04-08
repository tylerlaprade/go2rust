package main

import "testing"

func TestTranspileContextOwnsSessionPackageAndFileState(t *testing.T) {
	typeInfo := &TypeInfo{}
	ctx := &TranspileContext{
		Session: NewTranspileSession(typeInfo, map[string]string{"example.com/dep": "example_com_dep"}),
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}

	SetTranspileContext(ctx)
	defer SetTranspileContext(nil)

	SetTypeInfo(typeInfo)
	RegisterFunctionSignature("fn_name", &FunctionSignature{})
	RegisterErrorImplType("MyError")
	SetPackageImports(map[string]string{
		"dep": "example.com/dep",
		"fmt": "fmt",
	})
	TrackImport("Rc")
	NeedFormatAny()

	if GetTypeInfo() != typeInfo {
		t.Fatalf("GetTypeInfo() should return the session type info")
	}
	if ctx.Package.FunctionSignatures["fn_name"] == nil {
		t.Fatalf("function signature should be recorded in package state")
	}
	if !ctx.Package.ErrorImplTypes["MyError"] {
		t.Fatalf("error implementation type should be recorded in package state")
	}
	if ctx.Package.GoPackageImports["dep"] != "example.com/dep" {
		t.Fatalf("package imports should be stored in package state, got %#v", ctx.Package.GoPackageImports)
	}
	if !ctx.Package.ExternalPackages["example.com/dep"] {
		t.Fatalf("external package set should be stored in package state")
	}
	if !ctx.File.Imports.needs["Rc"] || !ctx.File.Imports.needs["Any"] {
		t.Fatalf("file imports should be tracked in file state, got %#v", ctx.File.Imports.needs)
	}
	if !ctx.File.Helpers.needsFormatAny {
		t.Fatalf("helper usage should be tracked in file state")
	}
}

func TestSetTranspileContextSyncsFileCompatibilityState(t *testing.T) {
	ctx := &TranspileContext{
		Session: NewTranspileSession(nil, nil),
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}

	SetTranspileContext(ctx)
	currentReceiver = "recv"
	currentReceiverType = "Thing"
	currentFunctionHasDefer = true
	pendingLoopLabel = "outer"
	hasInitFunction = true
	SetTranspileContext(nil)

	if ctx.File.CurrentReceiver != "recv" {
		t.Fatalf("CurrentReceiver = %q, want recv", ctx.File.CurrentReceiver)
	}
	if ctx.File.CurrentReceiverType != "Thing" {
		t.Fatalf("CurrentReceiverType = %q, want Thing", ctx.File.CurrentReceiverType)
	}
	if !ctx.File.CurrentFunctionHasDefer {
		t.Fatalf("CurrentFunctionHasDefer should sync back into file state")
	}
	if ctx.File.PendingLoopLabel != "outer" {
		t.Fatalf("PendingLoopLabel = %q, want outer", ctx.File.PendingLoopLabel)
	}
	if !ctx.File.HasInitFunction {
		t.Fatalf("HasInitFunction should sync back into file state")
	}
}
