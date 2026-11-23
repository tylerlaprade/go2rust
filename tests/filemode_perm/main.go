package main

import (
	"fmt"
	"os"
)

func main() {
	withTypeBits := os.ModeDir | os.ModeSetuid | os.ModePerm

	fmt.Println(os.ModePerm.Perm() == os.ModePerm)
	fmt.Println(os.ModeDir.Perm() == 0)
	fmt.Println(withTypeBits.Perm() == os.ModePerm)
	fmt.Println(os.ModeType.Perm() == 0)
}
