package main

import "fmt"

type acc struct{ n int }

func (a *acc) add(x, y int) int {
	fmt.Println("add", x, y)
	a.n += x + y
	return a.n
}

func pair() (int, int) {
	fmt.Println("pair")
	return 1, 2
}

func recv(a *acc) *acc {
	fmt.Println("recv")
	return a
}

func note() { fmt.Println("deferred") }

func compute() int {
	fmt.Println("compute")
	return 7
}

func plain() int {
	defer note()
	return compute()
}

func viaMulti(a *acc) int {
	defer note()
	return a.add(pair())
}

func main() {
	collide()
	a := &acc{}
	fmt.Println(plain())
	fmt.Println(viaMulti(a))
	recv(a).add(pair())
	__go_mv_1_0 := 5
	fmt.Println(divmod(7, 3))
	fmt.Println(__go_mv_1_0)
}

func divmod(a, b int) (int, int) { return a / b, a % b }
