package main

func tail[S ~string | ~[]byte](s S) S {
	s = s[1:]
	return s
}

func section[S ~string | ~[]byte](s S, low, high int) S {
	return s[low:high]
}

func asText[S ~string | ~[]byte](s S) string {
	return string(s[:2])
}

func main() {
	println(tail("abc"))
	println(tail([]byte{1, 255, 128})[0])
	println(section("abcd", 1, 3))
	println(section([]byte{1, 255, 128}, 1, 3)[1])
	println(asText("hello"), asText([]byte{103, 111, 33}))
}
