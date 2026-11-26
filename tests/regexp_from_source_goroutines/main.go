// regexp and regexp/syntax translate from source, so every method of a
// Regexp works, not only the few a hand-written engine covered. A
// goroutine shares one compiled Regexp.
package main

import (
	"fmt"
	"regexp"
	"strings"
)

func main() {
	re := regexp.MustCompile(`(\w+)@(\w+)\.com`)
	fmt.Println(re.FindAllStringSubmatch("a@b.com, c@d.com", -1))
	fmt.Println(re.FindStringIndex("x a@b.com"), re.NumSubexp())
	fmt.Println(re.ReplaceAllStringFunc("a@b.com", strings.ToUpper))
	fmt.Println(re.ReplaceAllString("a@b.com", "$2 at $1"))
	named := regexp.MustCompile(`(?P<year>\d{4})-(?P<month>\d{2})`)
	m := named.FindStringSubmatch("on 2024-05")
	for i, name := range named.SubexpNames() {
		if name != "" {
			fmt.Println(name, m[i])
		}
	}
	fmt.Println(regexp.MustCompile(`\s*,\s*`).Split("a , b,c", -1))
	fmt.Println(regexp.QuoteMeta("1+1=2?"), regexp.MustCompile(`^a.c$`).MatchString("abc"))
	_, err := regexp.Compile(`a(b`)
	fmt.Println(err)
	ok, _ := regexp.MatchString(`^\d+$`, "12345")
	fmt.Println(ok, re.String())
	done := make(chan bool)
	go func() {
		fmt.Println(re.MatchString("z@y.com"))
		done <- true
	}()
	<-done
}
