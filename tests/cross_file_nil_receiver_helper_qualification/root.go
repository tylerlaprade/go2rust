package main

type rootError struct{}

func callRoot(receiver *rootError) string {
	return receiver.Error()
}
