package main

import (
	"fmt"
	"regexp"
)

var modFlagRegexp = regexp.MustCompile(`-mod[ =](\w+)`)

func capture(re *regexp.Regexp, text string) []string {
	return re.FindStringSubmatch(text)
}

func main() {
	modMatches := capture(modFlagRegexp, "-mod=vendor")
	fmt.Println(modMatches[1])
	fmt.Println(modFlagRegexp.FindStringSubmatch("-mod vendor")[1])

	changed := regexp.MustCompile(`go:.*go.mod.*contents have changed`)
	fmt.Println(changed.MatchString("go: updates to go.mod needed, but contents have changed"))

	version := regexp.MustCompile(`^go version (go\S+|devel \S+)`)
	fmt.Println(version.FindStringSubmatch("go version go1.22.0 darwin/arm64")[1])

	currency := regexp.MustCompile(`[$,]`)
	fmt.Println(currency.ReplaceAllString("$1,234", ""))
}
