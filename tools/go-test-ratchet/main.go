package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"io"
	"os"
	"os/exec"
	"sort"
	"strings"
)

type testEvent struct {
	Action  string
	Package string
	Test    string
	Output  string
}

func main() {
	knownPath := flag.String("known", ".github/go-test-known-failures.txt", "path to the checked-in known-failures list")
	packagePattern := flag.String("package", "./go", "package pattern to pass to go test")
	flag.Parse()

	known, err := loadKnownFailures(*knownPath)
	if err != nil {
		fmt.Fprintln(os.Stderr, "go-test-ratchet:", err)
		os.Exit(1)
	}
	actual, testErr, err := runGoTests(*packagePattern, os.Stdout, os.Stderr)
	if err != nil {
		fmt.Fprintln(os.Stderr, "go-test-ratchet:", err)
		os.Exit(1)
	}

	newFailures, unexpectedPasses := compareFailures(known, actual)
	if len(newFailures) == 0 && len(unexpectedPasses) == 0 {
		if testErr == nil && len(known) != 0 {
			fmt.Fprintln(os.Stderr, "go-test-ratchet: go test passed even though the known-failures list is not empty")
			os.Exit(1)
		}
		fmt.Printf("go-test-ratchet: failure set unchanged (%d known top-level tests)\n", len(actual))
		return
	}

	if len(newFailures) != 0 {
		fmt.Fprintln(os.Stderr, "\nNEW GO UNIT-TEST FAILURES:")
		for _, name := range newFailures {
			fmt.Fprintln(os.Stderr, name)
		}
	}
	if len(unexpectedPasses) != 0 {
		fmt.Fprintln(os.Stderr, "\nKNOWN FAILURES THAT NOW PASS (remove them from the list):")
		for _, name := range unexpectedPasses {
			fmt.Fprintln(os.Stderr, name)
		}
	}
	fmt.Fprintf(os.Stderr, "\nexpected %d failing top-level tests; observed %d\n", len(known), len(actual))
	os.Exit(1)
}

func loadKnownFailures(path string) (map[string]struct{}, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("open known-failures list: %w", err)
	}
	defer file.Close()

	known := make(map[string]struct{})
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		if !strings.HasPrefix(line, "Test") || strings.Contains(line, "/") {
			return nil, fmt.Errorf("invalid top-level test name %q", line)
		}
		if _, duplicate := known[line]; duplicate {
			return nil, fmt.Errorf("duplicate known failure %q", line)
		}
		known[line] = struct{}{}
	}
	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("read known-failures list: %w", err)
	}
	return known, nil
}

func runGoTests(packagePattern string, stdout, stderr io.Writer) (map[string]struct{}, error, error) {
	cmd := exec.Command("go", "test", "-json", packagePattern)
	pipe, err := cmd.StdoutPipe()
	if err != nil {
		return nil, nil, fmt.Errorf("capture go test output: %w", err)
	}
	var commandStderr bytes.Buffer
	cmd.Stderr = io.MultiWriter(stderr, &commandStderr)
	if err := cmd.Start(); err != nil {
		return nil, nil, fmt.Errorf("start go test: %w", err)
	}

	failures := make(map[string]struct{})
	activeTests := make(map[string]struct{})
	packages := make(map[string]struct{})
	outputByTest := make(map[string]*strings.Builder)
	var packageOutput strings.Builder
	decoder := json.NewDecoder(pipe)
	for {
		var event testEvent
		err := decoder.Decode(&event)
		if errors.Is(err, io.EOF) {
			break
		}
		if err != nil {
			_ = cmd.Process.Kill()
			_ = cmd.Wait()
			return nil, nil, fmt.Errorf("decode go test JSON: %w", err)
		}
		if event.Package != "" {
			packages[event.Package] = struct{}{}
		}
		if event.Output != "" {
			if event.Test == "" {
				packageOutput.WriteString(event.Output)
			} else {
				name := topLevelTestName(event.Test)
				if outputByTest[name] == nil {
					outputByTest[name] = &strings.Builder{}
				}
				outputByTest[name].WriteString(event.Output)
			}
		}
		if event.Action == "fail" && event.Test != "" {
			failures[topLevelTestName(event.Test)] = struct{}{}
		}
		trackTestCompletion(activeTests, event)
	}

	testErr := cmd.Wait()
	if err := validateSinglePackage(packages); err != nil {
		return nil, testErr, err
	}
	if len(activeTests) != 0 {
		return nil, testErr, fmt.Errorf("go test ended without terminal events for: %s", strings.Join(sortedSet(activeTests), ", "))
	}
	if testErr != nil && len(failures) == 0 {
		message := strings.TrimSpace(packageOutput.String() + commandStderr.String())
		if message == "" {
			message = testErr.Error()
		}
		return nil, testErr, fmt.Errorf("go test failed without a test failure event: %s", message)
	}
	for _, name := range sortedSet(failures) {
		fmt.Fprintf(stdout, "\n--- %s ---\n", name)
		fmt.Fprint(stdout, outputByTest[name].String())
	}
	return failures, testErr, nil
}

func validateSinglePackage(packages map[string]struct{}) error {
	if len(packages) <= 1 {
		return nil
	}
	return fmt.Errorf("package pattern selected multiple packages; run the ratchet once per package: %s", strings.Join(sortedSet(packages), ", "))
}

func topLevelTestName(name string) string {
	if slash := strings.IndexByte(name, '/'); slash >= 0 {
		return name[:slash]
	}
	return name
}

func trackTestCompletion(active map[string]struct{}, event testEvent) {
	if event.Test == "" || topLevelTestName(event.Test) != event.Test {
		return
	}
	switch event.Action {
	case "run":
		active[event.Test] = struct{}{}
	case "pass", "fail", "skip":
		delete(active, event.Test)
	}
}

func compareFailures(known, actual map[string]struct{}) (newFailures, unexpectedPasses []string) {
	for name := range actual {
		if _, ok := known[name]; !ok {
			newFailures = append(newFailures, name)
		}
	}
	for name := range known {
		if _, ok := actual[name]; !ok {
			unexpectedPasses = append(unexpectedPasses, name)
		}
	}
	sort.Strings(newFailures)
	sort.Strings(unexpectedPasses)
	return newFailures, unexpectedPasses
}

func sortedSet(values map[string]struct{}) []string {
	result := make([]string, 0, len(values))
	for value := range values {
		result = append(result, value)
	}
	sort.Strings(result)
	return result
}
