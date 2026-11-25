package main

import (
	"fmt"
	"strings"
	"time"
)

// A type defined over a named number, string or bool has its underlying
// type and none of its methods.

type Level int

func (l Level) String() string { return fmt.Sprintf("L%d", int(l)) }

type Severity Level

type Label string

func (l Label) Upper() string { return strings.ToUpper(string(l)) }

type Tag Label

type durationValue time.Duration

func newDurationValue(val time.Duration, p *time.Duration) *durationValue {
	*p = val
	return (*durationValue)(p)
}

func (d *durationValue) Set(s string) error {
	v, err := time.ParseDuration(s)
	if err != nil {
		return err
	}
	*d = durationValue(v)
	return nil
}

func (d *durationValue) Get() any { return time.Duration(*d) }

func (d *durationValue) String() string { return (*time.Duration)(d).String() }

func (d durationValue) Seconds() float64 { return time.Duration(d).Seconds() }

type levelValue Level

func newLevelValue(p *Level) *levelValue { return (*levelValue)(p) }

func (l *levelValue) Set(n int) { *l = levelValue(n) }

type holder struct{ d durationValue }

func main() {
	var target time.Duration
	dv := newDurationValue(3*time.Second, &target)
	fmt.Println(dv.String(), target)
	if err := dv.Set("250ms"); err != nil {
		fmt.Println("error:", err)
	}
	fmt.Println(dv.String(), dv.Get(), target)
	fmt.Println(dv.Set("soon") != nil, target)

	local := durationValue(time.Minute)
	fmt.Println(time.Duration(local), local.Seconds(), int64(local))
	h := holder{d: durationValue(2 * time.Hour)}
	fmt.Println(time.Duration(h.d))

	s := Severity(Level(3))
	fmt.Println(Level(s), int(s)+1, s)
	var lvl Level = 1
	lv := newLevelValue(&lvl)
	lv.Set(4)
	fmt.Println(lvl, Level(*lv))

	t := Tag(Label("go"))
	fmt.Println(Label(t).Upper(), string(t)+"!", len(t))
	var anyD any = time.Duration(1500) * time.Millisecond
	fmt.Println(anyD)
}
