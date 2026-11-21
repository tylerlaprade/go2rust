package main

func callSibling(receiver *siblingError) string {
	return receiver.Error()
}
