package event

import (
	"example.com/ifaceeq/keys"
	"example.com/ifaceeq/label"
)

func IsMsg(l label.Label) bool {
	return l.Key() == keys.Msg
}
