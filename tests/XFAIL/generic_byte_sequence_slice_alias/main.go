package main

func tail[S ~string | ~[]byte](s S) S { return s[1:] }

func main() {
	data := []byte{10, 20, 30}
	view := tail(data)
	view[0] = 99
	println(data[1], view[0])
}
