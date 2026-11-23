package main

import (
	"context"
	"fmt"
	"os"
	"os/exec"
)

func main() {
	marker := "exec_command_context_cancel.marker"
	os.Remove(marker)
	defer os.Remove(marker)

	ctx, cancel := context.WithCancel(context.Background())
	cmd := exec.CommandContext(ctx, "sh", "-c", "printf cancelled-ran > \"$1\"", "sh", marker)
	cancel()
	err := cmd.Run()
	data, readErr := os.ReadFile(marker)
	fmt.Println(err != nil, readErr != nil, len(data) == 0)

	deadlineCtx, deadlineCancel := context.WithTimeout(context.Background(), 0)
	defer deadlineCancel()
	cmd = exec.CommandContext(deadlineCtx, "sh", "-c", "printf deadline-ran > \"$1\"", "sh", marker)
	<-deadlineCtx.Done()
	err = cmd.Run()
	data, readErr = os.ReadFile(marker)
	fmt.Println(err != nil, readErr != nil, len(data) == 0)

	err = exec.CommandContext(context.Background(), "sh", "-c", "printf live-ran > \"$1\"", "sh", marker).Run()
	data, readErr = os.ReadFile(marker)
	fmt.Println(err == nil, readErr == nil, string(data))
}
