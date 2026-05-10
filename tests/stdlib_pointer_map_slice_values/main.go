package main

import (
	"fmt"
	"go/types"
	"sort"
)

type pkgObj struct {
	obj  types.Object
	name string
}

type bundle struct {
	localpkg *types.Package
}

type writer struct {
	p *bundle
}

func exportPath(pkg *types.Package) string {
	return pkg.Name()
}

func (w *writer) exportPath(pkg *types.Package) string {
	if pkg == w.p.localpkg {
		return ""
	}
	return pkg.Name()
}

func remember(m map[*types.Package][]pkgObj, pkg *types.Package, obj types.Object) {
	m[pkg] = nil
	m[pkg] = append(m[pkg], pkgObj{obj, "name"})
	w := &writer{&bundle{pkg}}
	var pkgs []*types.Package
	for p, objs := range m {
		pkgs = append(pkgs, p)
		sort.Slice(objs, func(i, j int) bool {
			return objs[i].name < objs[j].name
		})
		_ = len(objs)
	}
	sort.Slice(pkgs, func(i, j int) bool {
		return exportPath(pkgs[i]) < exportPath(pkgs[j])
	})
	sort.Slice(pkgs, func(i, j int) bool {
		return w.exportPath(pkgs[i]) < w.exportPath(pkgs[j])
	})
	for _, p := range pkgs {
		_ = exportPath(p)
		_ = w.exportPath(p)
		_ = p.Name()
		_ = m[p]
	}
	_ = pkgs
}

func main() {
	if false {
		remember(nil, nil, nil)
	}
	fmt.Println("ok")
}
