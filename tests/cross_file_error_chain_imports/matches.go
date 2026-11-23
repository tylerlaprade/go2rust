package main

import "errors"

func matches() bool {
	cause := errors.New("cause")
	return errors.Is(errors.Join(cause), cause)
}
