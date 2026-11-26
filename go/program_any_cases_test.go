package main

import (
	"go/token"
	"go/types"
	"strings"
	"testing"
)

func TestSharedAnyRuntimeRegistryErasesCallerTypes(t *testing.T) {
	helpers := (&HelperTracker{needsGoAnyRuntimeOps: true}).GenerateSharedStdlibHelperModule()
	for _, want := range []string{
		"pub fn go_register_any_runtime_type<T: Any + 'static>",
		"pub fn go_format_registered_any(value: &dyn Any) -> Option<String>",
		"pub fn go_clone_registered_any(value: &dyn Any) -> Option<Box<dyn Any>>",
		"pub fn go_registered_any_type_name(value: &dyn Any) -> Option<&'static str>",
		"pub fn go_registered_any_is_error(value: &dyn Any) -> bool",
		"pub fn go_registered_any_error(value: &dyn Any) -> Option<std::rc::Rc<dyn std::error::Error>>",
	} {
		if !strings.Contains(helpers, want) {
			t.Fatalf("shared any registry missing %q:\n%s", want, helpers)
		}
	}
	if strings.Contains(helpers, "Point") || strings.Contains(helpers, "callerError") {
		t.Fatalf("shared registry must not name caller-defined types:\n%s", helpers)
	}
}

func TestAnyPayloadGoTypeIsShareable(t *testing.T) {
	intType, stringType := types.Typ[types.Int], types.Typ[types.String]
	errorType := types.Universe.Lookup("error").Type()
	pkg := types.NewPackage("example.com/shapes", "shapes")
	point := types.NewNamed(types.NewTypeName(token.NoPos, pkg, "Point", nil), types.NewStruct(nil, nil), nil)
	cases := []struct {
		name string
		typ  types.Type
		want bool
	}{
		{"[]int", types.NewSlice(intType), true},
		{"map[string][]int", types.NewMap(stringType, types.NewSlice(intType)), true},
		{"*string", types.NewPointer(stringType), true},
		{"error", errorType, true},
		{"[]error", types.NewSlice(errorType), true},
		{"func(int) string", types.NewSignatureType(nil, nil, nil, types.NewTuple(types.NewVar(token.NoPos, nil, "", intType)), types.NewTuple(types.NewVar(token.NoPos, nil, "", stringType)), false), true},
		// A declared type is its package's own, however it is wrapped.
		{"shapes.Point", point, false},
		{"*shapes.Point", types.NewPointer(point), false},
		{"[]shapes.Point", types.NewSlice(point), false},
		{"map[string]shapes.Point", types.NewMap(stringType, point), false},
		// Each crate generates its own type for these.
		{"chan int", types.NewChan(types.SendRecv, intType), false},
		{"struct{}", types.NewStruct(nil, nil), false},
		{"complex128", types.Typ[types.Complex128], false},
		{"unsafe.Pointer", types.Typ[types.UnsafePointer], false},
	}
	for _, tc := range cases {
		if got := anyPayloadGoTypeIsShareable(tc.typ); got != tc.want {
			t.Errorf("anyPayloadGoTypeIsShareable(%s) = %v, want %v", tc.name, got, tc.want)
		}
	}
}

func TestAnyPayloadTypeUsesOnlyStandardNames(t *testing.T) {
	cases := []struct {
		rustType string
		want     bool
	}{
		{"Rc<RefCell<Option<Vec<i32>>>>", true},
		{"Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>", true},
		{"Rc<RefCell<Option<Box<dyn StdError>>>>", true},
		{"Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>", true},
		{"Arc<std::sync::Mutex<Option<Vec<String>>>>", true},
		{"Rc<RefCell<Option<Rc<dyn Fn(i32) -> i32 + 'static>>>>", true},
		// A type a package declares is a different type in every other crate,
		// or no type at all.
		{"Point", false},
		{"Rc<RefCell<Option<Point>>>", false},
		{"Rc<RefCell<Option<Vec<Point>>>>", false},
		{"crate::Point", false},
		{"example_com_shapes::Point", false},
		// Helper types are generated once per crate.
		{"GoPtr<i32>", false},
		{"", false},
	}
	for _, tc := range cases {
		if got := anyPayloadTypeUsesOnlyStandardNames(tc.rustType); got != tc.want {
			t.Errorf("anyPayloadTypeUsesOnlyStandardNames(%q) = %v, want %v", tc.rustType, got, tc.want)
		}
	}
}

func TestProgramAnyCasesReachOnlyTheHelpersAPackageHas(t *testing.T) {
	beginProgramAnyCases()
	defer endProgramAnyCases()
	shared := currentProgramAnyCases

	sliceType := "Rc<RefCell<Option<Vec<i32>>>>"
	recordProgramAnyCase(sliceType, anyFormatArm{expr: "format_slice(v)", needsFormatSlice: true}, true, "[]int")
	recordProgramAnyCase("Rc<RefCell<Option<Point>>>", anyFormatArm{expr: `format!("{}", v)`}, true, "main.Point")
	if len(shared.cloneTypes) != 1 || len(shared.formatArms) != 1 || len(shared.typeNames) != 1 {
		t.Fatalf("only the slice is shared, got %d clone types, %d format arms, %d names",
			len(shared.cloneTypes), len(shared.formatArms), len(shared.typeNames))
	}

	// A package that only prints gets the print case and nothing else.
	printer := &HelperTracker{needsFormatAny: true}
	cloneTypes, formatArms, typeNames := shared.casesFor(printer)
	if _, ok := formatArms[sliceType]; !ok {
		t.Errorf("a package that prints interface values did not get the shared slice case")
	}
	if len(cloneTypes) != 0 || len(typeNames) != 0 {
		t.Errorf("a package with no copy or %%T helper got cases for them: %v %v", cloneTypes, typeNames)
	}
	if printer.anyFormatArms != nil {
		t.Errorf("casesFor changed the package's own cases")
	}

	// A package's own case wins over the shared one for the same type.
	own := &HelperTracker{needsFormatAny: true, anyFormatArms: map[string]anyFormatArm{sliceType: {expr: "own(v)"}}}
	_, formatArms, _ = shared.casesFor(own)
	if formatArms[sliceType].expr != "own(v)" {
		t.Errorf("shared case replaced the package's own: %q", formatArms[sliceType].expr)
	}
}

func TestProgramAnyCasesFindThePackagesWrittenTooEarly(t *testing.T) {
	beginProgramAnyCases()
	defer endProgramAnyCases()
	shared := currentProgramAnyCases

	// The dependency is written first, with a print helper and no cases.
	shared.currentPackage = "example.com/logger"
	shared.casesFor(&HelperTracker{needsFormatAny: true})
	// A dependency with no interface helper at all.
	shared.currentPackage = "example.com/plain"
	shared.casesFor(&HelperTracker{})
	shared.currentPackage = ""
	if shared.packageIsMissingCases("example.com/logger") {
		t.Fatalf("nothing has been shared yet, so nothing can be missing")
	}

	// Then main boxes a slice.
	recordProgramAnyCase("Rc<RefCell<Option<Vec<i32>>>>", anyFormatArm{expr: "format_slice(v)", needsFormatSlice: true}, true, "[]int")
	if !shared.packageIsMissingCases("example.com/logger") {
		t.Errorf("the logger's print helper was written before the slice case existed")
	}
	if shared.packageIsMissingCases("example.com/plain") {
		t.Errorf("a package with no interface helper has nothing to be missing")
	}

	// Written again, it has everything.
	shared.currentPackage = "example.com/logger"
	delete(shared.written, "example.com/logger")
	shared.casesFor(&HelperTracker{needsFormatAny: true})
	shared.currentPackage = ""
	if shared.packageIsMissingCases("example.com/logger") {
		t.Errorf("the logger was written again and is still reported as missing a case")
	}
}

func TestProgramAnyCasesLeaveStandardLibraryPackagesAlone(t *testing.T) {
	beginProgramAnyCases()
	defer endProgramAnyCases()
	shared := currentProgramAnyCases
	sliceType := "Rc<RefCell<Option<Vec<i32>>>>"
	arm := anyFormatArm{expr: "format_slice(v)", needsFormatSlice: true}

	// A value boxed inside a translated standard-library package is not
	// offered to the others.
	shared.currentPackage, shared.currentPackageIsStandardLibrary = "bytes", true
	recordProgramAnyCase(sliceType, arm, true, "[]int")
	if len(shared.formatArms) != 0 || len(shared.cloneTypes) != 0 {
		t.Fatalf("a standard-library package added to the shared cases")
	}

	// A value boxed in the program's own code is not written into a
	// standard-library package's helpers either.
	shared.currentPackage, shared.currentPackageIsStandardLibrary = "", false
	recordProgramAnyCase(sliceType, arm, true, "[]int")
	shared.currentPackage, shared.currentPackageIsStandardLibrary = "io", true
	_, formatArms, _ := shared.casesFor(&HelperTracker{needsFormatAny: true})
	if len(formatArms) != 0 {
		t.Errorf("a standard-library package's print helper was given the program's cases: %v", formatArms)
	}
	if shared.packageIsMissingCases("io") {
		t.Errorf("a standard-library package is never written again for the shared cases")
	}

	// The crate of helpers that every package shares is left alone too.
	shared.currentPackage, shared.currentPackageIsStandardLibrary = "", false
	previous := generatingPublicHelpers
	generatingPublicHelpers = true
	cloneTypes, _, _ := shared.casesFor(&HelperTracker{needsAnyClone: true})
	generatingPublicHelpers = previous
	if len(cloneTypes) != 0 {
		t.Errorf("the shared helper crate was given the program's cases: %v", cloneTypes)
	}
}
