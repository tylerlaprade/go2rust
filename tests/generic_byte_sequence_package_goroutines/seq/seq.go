package seq

// Trim drops the first n bytes of s.
func Trim[B []byte | string](s B, n int) B { return s[n:] }

// Mid is s[i:j].
func Mid[B []byte | string](s B, i, j int) B { return s[i:j] }

// Digits counts the ASCII digits in s.
func Digits[B []byte | string](s B) int {
	n := 0
	for _, c := range []byte(s) {
		if c >= '0' && c <= '9' {
			n++
		}
	}
	return n
}

// Prefix is the first two bytes of s, as a string.
func Prefix[B []byte | string](s B) string { return string(s[:2]) }

// Atoi parses leading digits, as time's atoi does.
func Atoi[B []byte | string](s B) (int, B) {
	x := 0
	for len(s) > 0 && s[0] >= '0' && s[0] <= '9' {
		x = x*10 + int(s[0]-'0')
		s = s[1:]
	}
	return x, s
}
