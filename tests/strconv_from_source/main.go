// strconv is translated from its Go source: parsing with bases, bit sizes
// and range errors, float formatting in every verb, quoting and appending.
package main

import (
	"errors"
	"fmt"
	"strconv"
)

func main() {
	for _, s := range []string{"42", "-7", "+3", "0x1f", "abc", "99999999999999999999", ""} {
		n, err := strconv.Atoi(s)
		fmt.Println(n, err)
	}
	for _, tc := range []struct {
		s    string
		base int
		bits int
	}{{"ff", 16, 64}, {"0x_ff", 0, 64}, {"-128", 10, 8}, {"128", 10, 8}, {"777", 8, 16}, {"1010", 2, 64}, {"z", 36, 64}} {
		n, err := strconv.ParseInt(tc.s, tc.base, tc.bits)
		fmt.Println(n, err)
	}
	u, err := strconv.ParseUint("18446744073709551615", 10, 64)
	fmt.Println(u, err)
	_, err = strconv.ParseUint("-1", 10, 64)
	fmt.Println(err)
	for _, s := range []string{"3.14159", "1e10", "-0.0", "inf", "NaN", "1e400", "0x1p-2", "12abc", ".5"} {
		f, err := strconv.ParseFloat(s, 64)
		fmt.Println(f, err)
	}
	f32, err := strconv.ParseFloat("3.4e39", 32)
	fmt.Println(f32, err)
	for _, b := range []string{"true", "F", "1", "yes"} {
		v, err := strconv.ParseBool(b)
		fmt.Println(v, err)
	}
	fmt.Println(strconv.Itoa(-12345), strconv.FormatInt(255, 16), strconv.FormatInt(-255, 2), strconv.FormatUint(1<<63, 36))
	for _, v := range []float64{0, 1, -2.5, 3.14159265358979, 1e21, 1e-7, 123456789.125, 0.1 + 0.2} {
		fmt.Println(strconv.FormatFloat(v, 'f', -1, 64), strconv.FormatFloat(v, 'e', 3, 64), strconv.FormatFloat(v, 'g', -1, 64), strconv.FormatFloat(v, 'f', 2, 64), strconv.FormatFloat(v, 'g', 4, 32))
	}
	fmt.Println(strconv.FormatBool(true), strconv.Quote("hi\n\"there\"\t☺\x00"), strconv.QuoteToASCII("☺ é"), strconv.QuoteRune('☺'), strconv.QuoteRuneToASCII('\n'))
	buf := []byte("n=")
	buf = strconv.AppendInt(buf, -42, 10)
	buf = append(buf, ' ')
	buf = strconv.AppendFloat(buf, 2.5, 'f', 1, 64)
	buf = append(buf, ' ')
	buf = strconv.AppendQuote(buf, "q")
	buf = strconv.AppendBool(buf, false)
	fmt.Println(string(buf))

	_, err = strconv.Atoi("12x")
	var ne *strconv.NumError
	if errors.As(err, &ne) {
		fmt.Println("func", ne.Func, "num", ne.Num, "err", ne.Err, errors.Is(ne.Err, strconv.ErrSyntax))
	}
	fmt.Println(errors.Is(err, strconv.ErrSyntax))
	_, err = strconv.ParseInt("99999999999999999999", 10, 64)
	fmt.Println(errors.Is(err, strconv.ErrRange), err)
	fmt.Println(strconv.IsPrint('a'), strconv.IsPrint('\x07'), strconv.CanBackquote("a`b"), strconv.IntSize)
}
