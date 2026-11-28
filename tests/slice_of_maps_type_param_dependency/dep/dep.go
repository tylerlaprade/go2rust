// Package dep takes slices of maps through a type parameter, so its
// functions stay generic in Rust and read the elements as map handles.
package dep

func First[S ~[]map[string]int](s S) map[string]int { return s[0] }

func Pick[S ~[]map[string]int, T any](s S, i int, _ T) map[string]int {
	m := s[i]
	return m
}

func Mark[S ~[]map[string]int](s S, k string) {
	for _, m := range s {
		m[k] = len(m)
	}
}

func Swap[S ~[]map[string]int](s S) {
	s[0], s[1] = s[1], s[0]
}

func Fresh[S ~[]map[string]int](s S, i int) {
	s[i] = map[string]int{"fresh": i}
}

func Count[S ~[]map[string]int](s S) int {
	n := 0
	for i := range s {
		n += len(s[i])
	}
	return n
}
