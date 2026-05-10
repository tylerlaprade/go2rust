package label

type Key interface {
	Name() string
}

type Label struct {
	key Key
}

func New(key Key) Label {
	return Label{key: key}
}

func (l Label) Key() Key {
	return l.key
}
