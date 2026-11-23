// The same program with a goroutine, so every value is behind a lock.
// The case mapping of package unicode on a small table: a binary search
// over a slice of structs whose deltas are a named array, returning a pointer
// into the slice.
package main

import "fmt"

const MaxCase = 3
const MaxRune = '\U0010FFFF'
const UpperLower = MaxRune + 1

type d [MaxCase]rune

type CaseRange struct {
	Lo    uint32
	Hi    uint32
	Delta d
}

var _CaseRanges = []CaseRange{
	{0x0041, 0x005A, d{0, 32, 0}},
	{0x0061, 0x007A, d{-32, 0, -32}},
	{0x00C0, 0x00D6, d{0, 32, 0}},
	{0x00D8, 0x00DE, d{0, 32, 0}},
	{0x00E0, 0x00F6, d{-32, 0, -32}},
	{0x0100, 0x012F, d{UpperLower, UpperLower, UpperLower}},
}

var CaseRanges = _CaseRanges

func lookupCaseRange(r rune, caseRange []CaseRange) *CaseRange {
	lo := 0
	hi := len(caseRange)
	for lo < hi {
		m := int(uint(lo+hi) >> 1)
		cr := &caseRange[m]
		if rune(cr.Lo) <= r && r <= rune(cr.Hi) {
			return cr
		}
		if r < rune(cr.Lo) {
			hi = m
		} else {
			lo = m + 1
		}
	}
	return nil
}

func convertCase(_case int, r rune, cr *CaseRange) rune {
	delta := cr.Delta[_case]
	if delta > MaxRune {
		return rune(cr.Lo) + ((r-rune(cr.Lo))&^1 | rune(_case&1))
	}
	return r + delta
}

func to(_case int, r rune, caseRange []CaseRange) (mappedRune rune, foundMapping bool) {
	if _case < 0 || MaxCase <= _case {
		return 0xFFFD, false
	}
	if cr := lookupCaseRange(r, caseRange); cr != nil {
		return convertCase(_case, r, cr), true
	}
	return r, false
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	for _, r := range []rune{'a', 'Z', 'ö', 'Ö', 'ā', 'Ā', '!'} {
		u, ok := to(0, r, CaseRanges)
		l, _ := to(1, r, CaseRanges)
		cr := lookupCaseRange(r, CaseRanges)
		fmt.Println(string(r), string(u), string(l), ok, cr != nil)
	}
}
