package main

import "fmt"

func main() {
	messageFormat := "dynamic message"
	errorFormat := "dynamic error"

	fmt.Println(fmt.Sprintf(messageFormat))
	fmt.Println(fmt.Errorf(errorFormat))
}
