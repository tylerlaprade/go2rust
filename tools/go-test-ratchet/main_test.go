package main

import (
	"os"
	"path/filepath"
	"reflect"
	"testing"
)

func TestCompareFailuresRejectsBothRatchetDirections(t *testing.T) {
	known := set("TestExistingFailure", "TestNowFixed")
	actual := set("TestExistingFailure", "TestNewFailure")

	newFailures, unexpectedPasses := compareFailures(known, actual)
	if want := []string{"TestNewFailure"}; !reflect.DeepEqual(newFailures, want) {
		t.Fatalf("newFailures = %v, want %v", newFailures, want)
	}
	if want := []string{"TestNowFixed"}; !reflect.DeepEqual(unexpectedPasses, want) {
		t.Fatalf("unexpectedPasses = %v, want %v", unexpectedPasses, want)
	}
}

func TestTopLevelTestNameCollapsesSubtests(t *testing.T) {
	if got, want := topLevelTestName("TestParent/case/inner"), "TestParent"; got != want {
		t.Fatalf("topLevelTestName() = %q, want %q", got, want)
	}
}

func TestIncompleteRunIsRejectedAfterKnownFailure(t *testing.T) {
	active := make(map[string]struct{})
	for _, event := range []testEvent{
		{Action: "run", Test: "TestKnownFailure"},
		{Action: "run", Test: "TestKnownFailure/case"},
		{Action: "fail", Test: "TestKnownFailure/case"},
		{Action: "fail", Test: "TestKnownFailure"},
		{Action: "run", Test: "TestHangs"},
	} {
		trackTestCompletion(active, event)
	}

	if want := set("TestHangs"); !reflect.DeepEqual(active, want) {
		t.Fatalf("active tests = %v, want %v", active, want)
	}
}

func TestMultiPackageSelectionsAreRejected(t *testing.T) {
	err := validateSinglePackage(set("example.com/one", "example.com/two"))
	if err == nil {
		t.Fatal("validateSinglePackage() unexpectedly succeeded")
	}
	if want := "package pattern selected multiple packages; run the ratchet once per package: example.com/one, example.com/two"; err.Error() != want {
		t.Fatalf("validateSinglePackage() error = %q, want %q", err, want)
	}
	if err := validateSinglePackage(set("example.com/one")); err != nil {
		t.Fatalf("validateSinglePackage() rejected one package: %v", err)
	}
}

func TestLoadKnownFailuresRejectsDuplicatesAndSubtests(t *testing.T) {
	for _, testCase := range []struct {
		name string
		body string
	}{
		{name: "duplicate", body: "TestOne\nTestOne\n"},
		{name: "subtest", body: "TestOne/case\n"},
		{name: "not a test", body: "helper\n"},
	} {
		t.Run(testCase.name, func(t *testing.T) {
			path := filepath.Join(t.TempDir(), "known.txt")
			if err := os.WriteFile(path, []byte(testCase.body), 0o600); err != nil {
				t.Fatal(err)
			}
			if _, err := loadKnownFailures(path); err == nil {
				t.Fatalf("loadKnownFailures(%q) unexpectedly succeeded", testCase.body)
			}
		})
	}
}

func set(names ...string) map[string]struct{} {
	result := make(map[string]struct{}, len(names))
	for _, name := range names {
		result[name] = struct{}{}
	}
	return result
}
