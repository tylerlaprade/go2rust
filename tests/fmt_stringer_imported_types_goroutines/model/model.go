package model

import "fmt"

type Label struct{ Text string }

func (l Label) String() string { return fmt.Sprintf("<%s>", l.Text) }
