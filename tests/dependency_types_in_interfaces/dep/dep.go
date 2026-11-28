package dep

import "fmt"

type tally int

type label string

type point struct{ X, Y int }

type named int

func (n named) String() string { return fmt.Sprintf("n%d", int(n)) }

type Score int

func New() tally      { return 3 }
func Label() label    { return "lbl" }
func P() point        { return point{1, 2} }
func N() named        { return 7 }
func Scores() []tally { return []tally{1, 2} }
