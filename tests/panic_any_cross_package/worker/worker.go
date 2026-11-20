package worker

func Fail(value any) {
	panic(value)
}

func RunNormalDefer(callback func()) {
	defer callback()
}

//go:noinline
func Run(callback func()) {
	callback()
}
