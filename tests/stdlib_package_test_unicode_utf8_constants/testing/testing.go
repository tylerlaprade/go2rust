package testing

// T is the smallest test-state surface needed by unicode/utf8.TestConstants.
// It deliberately records failures without reproducing the standard testing
// package's process and runtime machinery.
type T struct {
	failed bool
}

func (t *T) Errorf(_ string, _ ...any) {
	t.failed = true
}

func (t *T) Failed() bool {
	return t.failed
}

func (t *T) Reset() {
	t.failed = false
}
