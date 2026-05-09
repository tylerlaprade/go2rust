package main

import "fmt"

const (
	opType = '.'
	opElem = 'E'
)

func isDigit(c byte) bool {
	return '0' <= c && c <= '9'
}

func startsWithV(s string) bool {
	return len(s) > 0 && s[0] == 'v'
}

func appendOp(path []byte, op byte) []byte {
	return append(path, op)
}

type opAppender struct{}

func (opAppender) appendOp(path []byte, op byte) []byte {
	return append(path, op)
}

func classifyOp(op byte) string {
	switch op {
	case opType:
		return "type"
	case opElem:
		return "elem"
	default:
		return "unknown"
	}
}

func main() {
	fmt.Println("digit 5:", isDigit('5'))
	fmt.Println("digit x:", isDigit('x'))
	fmt.Println("version v1:", startsWithV("v1.0.0"))
	fmt.Println("version x1:", startsWithV("x1.0.0"))
	path := []byte{}
	path = append(path, opType)
	path = appendOp(path, opElem)
	path = opAppender{}.appendOp(path, opElem)
	fmt.Println("op type:", path[0] == opType, classifyOp(path[0]))
	fmt.Println("op elem:", path[1] == opElem, classifyOp(path[1]))
	fmt.Println("op method:", path[2] == opElem, classifyOp(path[2]))
}
