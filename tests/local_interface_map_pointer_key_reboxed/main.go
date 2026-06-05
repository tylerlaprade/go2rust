package main

import "fmt"

type Object interface {
	Name() string
}

type Var struct {
	name string
}

func (v *Var) Name() string {
	return v.name
}

type declInfo struct {
	seen bool
}

func declare(objMap map[Object]*declInfo, obj Object) {
	objMap[obj] = &declInfo{seen: true}
}

func lookup(objMap map[Object]*declInfo, obj Object) bool {
	info := objMap[obj]
	return info != nil && info.seen
}

func main() {
	objMap := map[Object]*declInfo{}
	v := &Var{name: "x"}

	declare(objMap, v)
	fmt.Println(lookup(objMap, v))

	var same Object = v
	fmt.Println(lookup(objMap, same))
}
