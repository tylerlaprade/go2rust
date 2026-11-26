// Package-level variables named after every name the generated helpers
// bind, as parameters or locals: the helpers sit at the crate root, where
// such a variable is a static, and Rust lets no binding shadow a static.
// The program uses many helpers: it prints interface values, slices, maps,
// structs, pointers and errors, and compares interface values. Goroutines
// make every variable a locked one.
package main

import (
	"errors"
	"fmt"
	"sort"
	"strconv"
	"strings"
	"sync"
)

var (
	a, absolute, accept, access, actual, addr, address, after_mod int
	any_val, arg, arg0, arg1, args, array, as_error, ascii int
	assign, at, b, b0, b1, b2, base, base0 int
	basic, before_query, bit_len, bit_size, body, bool_error, borrow, boxed int
	bytes, c, c0, c1, c2, c3, call, callback int
	cancel, cancel_cancelled, cancel_done, cancel_err, cancelled, candidate, candidate_text, candidates int
	capture, cell, channel, char, chars, child, chunk, clean_path int
	cleaned, clone_value, closed, cmd_env, code, command, component, context int
	count, ctx, current, cvar, cw, d, data, day int
	days, delta, depth, digit, digits, dir, dirfd, disk int
	doe, done, doy, drain, drained, duration, e, elem int
	elem_guard, elem_kind, embedded, embedded_ref, end, entries, entry, env int
	eof, era, err, errno, errs, escape, escaped, exp int
	exp_text, expand, exponent, exponent_form, exponent_limit, extra, f, fd int
	field, field_value, file, file_path, file_type, fill, fire, first int
	fixed, flag, flags, fmtc, forget, format, formatted, frac int
	fraction, from_relative_path, g, generation, getter, getter_guard, go_format_integer, go_index int
	guard, h, handle, has_more, has_separator, have, have_digits, held int
	hex, hh, hi, high, host, hour, identity, in_string int
	index, inner, inner_guard, input, inputc, io, is_dir, is_nil int
	item, items, j, kept, key, label, late, least_digits int
	left, left_guard, let, limit, list, lo, lock, locked int
	loop, low, lower, m, magnitude, maj, mantissa, many int
	map_key, marker, matched, matches, message, metadata, method, minus int
	minute, mod_index, mode, month, mp, msg, n, name int
	nanos, narrow, native, negative, nested, new_val, newlines, next int
	next_byte, nextc, nl_is_end, nl_is_space, now, object, octal_zero, offset int
	ok, ok_verbs, old, op, ops, options, other, out int
	outer, outer_index, output, owner, p, pad2, pad3, parent int
	path, path_error, pattern, payload, pending, period, perm, pipe int
	plain, plain_v, plus, plus_v, poisoned, pos, precision, prefix int
	previous, ptr, quote, quoted, r, raw, raw_query, read int
	read_all, ready, registry, relative, repl, rest, result, right int
	right_guard, rows, rs, rx, s, s0, s1, saw int
	scale, scheme, second, seconds, secs_of_day, seed, seen_point, sender int
	setter, setter_guard, shared, sharp, sharp_v, shift, shortest, sign int
	sign_len, simple, single, skip_space, slash, slice, slot, source int
	space, spec, src, stack, start, state, state_ref, status int
	std, stderr, stdin, stdout, step, stopped, stored, string_verb int
	suffix, syntax, t, table, target, target_code, targets, temp1 int
	temp2, text, text_ref, thread_channel, thread_stopped, timeout_cancelled, timeout_done, timeout_err int
	timer, tok, trailing_space, truncated, tx, typ, typed_val, underscores int
	unread, unsigned, upper, use, utf8, v_guard, val, value int
	value_start, values, verb, w, wait, want, was_active, was_stopped int
	when, whole, wid_present, width, with_mut, with_precision, word, wrapped int
	wraps, write, y, year, yoe, z, zero, zero_allowed int
)

type point struct {
	X, Y int
}

func (p point) show() string { return fmt.Sprint(p.X, ":", p.Y) }

func sum() int {
	return a + absolute + accept + access + actual + addr + address + after_mod + any_val + arg + arg0 + arg1 + args + array + as_error + ascii + assign + at + b + b0 + b1 + b2 + base + base0 + basic + before_query + bit_len + bit_size + body + bool_error + borrow + boxed + bytes + c + c0 + c1 + c2 + c3 + call + callback
}

func main() {
	value = 3
	slice = 4
	fmt.Println(value, slice, sum())
	var x any = 1
	var y any = "s"
	fmt.Println(x, y, x == y, []any{1, "a", 2.5, nil})
	m := map[string][]int{"k": {1, 2}, "j": {3}}
	fmt.Println(m, len(m))
	nested := [][]string{{"a"}, {"b", "c"}}
	fmt.Println(nested, [2][2]int{{1, 2}, {3, 4}})
	p := &point{1, 2}
	fmt.Println(*p, p.show())
	fmt.Printf("%v %+v %q %x %5.2f %T\n", point{3, 4}, point{5, 6}, "q", 255, 3.14159, p)
	err := errors.New("boom")
	wrapped := fmt.Errorf("wrap: %w", err)
	fmt.Println(err, wrapped, errors.Is(wrapped, err), errors.Unwrap(wrapped) == err)
	words := strings.Fields(" b a c ")
	sort.Strings(words)
	n, convErr := strconv.Atoi("42")
	fmt.Println(words, strings.Join(words, "-"), n, convErr, strconv.Quote("hi"))
	fmt.Println(fmt.Sprint(x, 2), fmt.Sprintf("%08.3f|%-4d|%x", 3.5, 7, "hi"))
	var wg sync.WaitGroup
	var mu sync.Mutex
	total := 0
	ch := make(chan any, 3)
	for i := 1; i <= 3; i++ {
		wg.Add(1)
		go func(k int) {
			defer wg.Done()
			mu.Lock()
			total += k
			mu.Unlock()
			ch <- k
		}(i)
	}
	wg.Wait()
	close(ch)
	got := 0
	for v := range ch {
		got += v.(int)
	}
	fmt.Println(total, got)
}
