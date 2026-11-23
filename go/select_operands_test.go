package main

import (
	"strings"
	"testing"
)

// A select evaluates time.After once, on entering, as Go does. Made again on
// each poll, the timer would restart every round and never fire.
func TestSelectEvaluatesChannelCallOnce(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import (
	"fmt"
	"time"
)

func main() {
	result := make(chan string)
	go func() {}()
	select {
	case v := <-result:
		fmt.Println(v)
	case <-time.After(time.Millisecond):
		fmt.Println("timeout")
	}
}
`)
	if !strings.Contains(rust, "{ let __select_chan_") || !strings.Contains(rust, "= go_channel_after(std::time::Duration::from_millis(1)); loop {") {
		t.Errorf("time.After should be evaluated before the polling loop:\n%s", rust)
	}
	if strings.Contains(rust, "go_channel_after(std::time::Duration::from_millis(1)).try_recv_select()") {
		t.Errorf("time.After is evaluated on every poll:\n%s", rust)
	}
}
