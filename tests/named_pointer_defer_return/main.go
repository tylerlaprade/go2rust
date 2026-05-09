package main

type Package struct {
	complete bool
}

func (p *Package) Complete() bool {
	return p.complete
}

func Import(path string) (pkg *Package, err error) {
	defer func() {}()
	unsafePkg := &Package{complete: true}
	if path == "unsafe" {
		return unsafePkg, nil
	}
	return nil, nil
}

func main() {
	pkg, err := Import("unsafe")
	if err != nil {
		panic(err)
	}
	println(pkg.Complete())
}
