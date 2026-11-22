package main

import "fmt"

func main() {
	invalid := string([]byte{0xfa, 0x00, 'A'})
	fmt.Printf("%x\n", invalid)
	fmt.Println(fmt.Sprintf("%X", invalid))
	fmt.Printf("%x\n", "\ufdd0\ufdd1fa")
}
