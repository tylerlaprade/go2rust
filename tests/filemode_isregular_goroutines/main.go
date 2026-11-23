package main

import (
	"fmt"
	"os"
)

func main() {
	done := make(chan struct{})
	go func() {
		fmt.Println(os.ModePerm.IsRegular())
		fmt.Println((os.ModeSetuid | os.ModeSticky | 0600).IsRegular())
		fmt.Println(!(os.ModeDir | 0600).IsRegular())
		fmt.Println(!os.ModeSymlink.IsRegular())
		fmt.Println(!os.ModeIrregular.IsRegular())
		close(done)
	}()
	<-done
}
