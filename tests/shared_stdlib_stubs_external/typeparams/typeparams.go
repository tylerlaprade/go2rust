package typeparams

import "example.com/sharedstubmain/aliases"

func Count() int {
	tuple := aliases.Tuple()
	if tuple == nil {
		return 0
	}
	return tuple.Len()
}
