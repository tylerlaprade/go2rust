package worker

func Fail(value any) {
	panic(value)
}

func RunNormalDefer(callback func()) {
	defer callback()
}
