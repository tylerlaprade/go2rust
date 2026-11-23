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
	fmt.Println(err == ctx.Err())

	deadlineCtx, deadlineCancel := context.WithTimeout(context.Background(), 0)
	defer deadlineCancel()
	cmd = exec.CommandContext(deadlineCtx, "sh", "-c", "printf deadline-ran > \"$1\"", "sh", marker)
	err = cmd.Run()
	data, readErr = os.ReadFile(marker)
	fmt.Println(err != nil, readErr != nil, len(data) == 0)
	fmt.Println(err == deadlineCtx.Err())

	negativeCtx, negativeCancel := context.WithTimeout(context.Background(), -1)
	defer negativeCancel()
	cmd = exec.CommandContext(negativeCtx, "sh", "-c", "printf negative-ran > \"$1\"", "sh", marker)
	err = cmd.Run()
	data, readErr = os.ReadFile(marker)
	fmt.Println(err != nil, readErr != nil, len(data) == 0)
	fmt.Println(err == negativeCtx.Err())

	parent, parentCancel := context.WithCancel(context.Background())
	child, childCancel := context.WithCancel(parent)
	defer childCancel()
	cmd = exec.CommandContext(child, "sh", "-c", "printf parent-ran > \"$1\"", "sh", marker)
	parentCancel()
	err = cmd.Run()
	data, readErr = os.ReadFile(marker)
	fmt.Println(err != nil, readErr != nil, len(data) == 0)
	fmt.Println(err == child.Err())

	outputCtx, outputCancel := context.WithCancel(context.Background())
	cmd = exec.CommandContext(outputCtx, "sh", "-c", "printf output-ran > \"$1\"", "sh", marker)
	outputCancel()
	output, outputErr := cmd.Output()
	fmt.Println(output == nil, outputErr == outputCtx.Err())

	err = exec.CommandContext(context.Background(), "sh", "-c", "printf live-ran > \"$1\"", "sh", marker).Run()
	data, readErr = os.ReadFile(marker)
	fmt.Println(err == nil, readErr == nil, string(data))
}
