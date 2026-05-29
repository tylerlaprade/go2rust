package main

// A parallel (swap) assignment whose right-hand side reads value-typed struct
// fields (here int) captures a bare value, not a wrapped handle. The temporary
// holds the raw value, so the target consume must wrap it (Some(tmp)) rather
// than move a handle out of it (tmp.lock().take()).

type pair struct {
	begin int
	end   int
}

func main() {
	p := &pair{begin: 1, end: 2}
	x := 9
	x, p.begin, p.end = p.end, x, p.begin
	println(x)
	println(p.begin)
	println(p.end)
}
