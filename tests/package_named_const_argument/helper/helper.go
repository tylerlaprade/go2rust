package helper

type Kind uint8

const PackageVar Kind = 1

type Var struct{}

func NewVar() *Var {
	return &Var{}
}

func SetKind(v *Var, kind Kind) {}
