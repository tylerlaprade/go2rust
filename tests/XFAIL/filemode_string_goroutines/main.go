package main

import (
	"fmt"
	"os"
)

func main() {
	done := make(chan struct{})
	go func() {
		fmt.Println(os.ModePerm.String())
		fmt.Println(os.ModePerm)
		fmt.Println((os.ModeDir | 0754).String())
		fmt.Println((os.ModeSymlink | os.ModeSetuid | 0777).String())
		fmt.Println((os.ModeAppend | os.ModeExclusive | os.ModeTemporary | os.ModeDevice |
			os.ModeNamedPipe | os.ModeSocket | os.ModeSetgid | os.ModeCharDevice |
			os.ModeSticky).String())
		close(done)
	}()
	<-done
}
