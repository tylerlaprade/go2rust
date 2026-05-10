package keys

import "example.com/ifaceeq/label"

type String struct {
	name string
}

func NewString(name string) *String {
	return &String{name: name}
}

func (k *String) Name() string {
	return k.name
}

func (k *String) Label() label.Label {
	return label.New(k)
}

var (
	Msg   = NewString("message")
	Other = NewString("other")
)
