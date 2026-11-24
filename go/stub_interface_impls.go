package main

import (
	"go/ast"
	"go/types"
	"slices"

	"golang.org/x/tools/go/packages"
)

// stubTypeInterfaceImpls returns, for a package translated from source, the
// impls of its interfaces for the bridged types the program can hold in
// them: *os.File is an io.Reader, an io.Writer, an io.Closer and the rest.
// The interfaces are the source package's traits, so only that package can
// implement them for the stub types. A bridged package counts when a
// package of the program, or a standard library package translated from
// source, imports it: flag hands os.Stderr to its io.Writer output.
func stubTypeInterfaceImpls(pkg *types.Package, interfaces map[string]*ast.InterfaceType, loaded map[string]*packages.Package) map[string]map[string]externalLocalInterfaceImpl {
	if pkg == nil || len(interfaces) == 0 || !isSourceMappedPackagePath(pkg.Path()) {
		return nil
	}
	imported := map[string]*types.Package{}
	for path, loadedPkg := range loaded {
		if loadedPkg == nil || loadedPkg.Types == nil || (isStdlibPackage(path) && !isSourceMappedPackagePath(path)) {
			continue
		}
		for importPath, importedPkg := range loadedPkg.Imports {
			if importedPkg != nil && importedPkg.Types != nil && isStubBackedStdlibPackagePath(importPath) {
				imported[importPath] = importedPkg.Types
			}
		}
	}
	if len(imported) == 0 {
		return nil
	}
	var paths []string
	for path := range imported {
		paths = append(paths, path)
	}
	slices.Sort(paths)
	impls := make(map[string]map[string]externalLocalInterfaceImpl)
	for ifaceName := range interfaces {
		obj, ok := pkg.Scope().Lookup(ifaceName).(*types.TypeName)
		if !ok || sourceMappedDeclIsPruned(obj) {
			continue
		}
		ifaceType, ok := types.Unalias(obj.Type()).Underlying().(*types.Interface)
		if !ok || ifaceType.NumMethods() == 0 {
			continue
		}
		ifaceType.Complete()
		for _, path := range paths {
			stubPkg := imported[path]
			for _, name := range stubPkg.Scope().Names() {
				if !isKnownStdlibHelperType(path, name) {
					continue
				}
				typeObj, ok := stubPkg.Scope().Lookup(name).(*types.TypeName)
				if !ok {
					continue
				}
				named, ok := types.Unalias(typeObj.Type()).(*types.Named)
				if !ok {
					continue
				}
				if _, isInterface := named.Underlying().(*types.Interface); isInterface {
					continue
				}
				if !types.Implements(types.NewPointer(named), ifaceType) || !stubHelperHasMethods(path, name, ifaceType) {
					continue
				}
				rustType := goTypesNamedTypeToRust(named)
				RegisterExternalTypeStubNamed(named, rustType)
				methods := map[string]*types.Signature{}
				methodSet := types.NewMethodSet(types.NewPointer(named))
				for i := 0; i < ifaceType.NumMethods(); i++ {
					method := ifaceType.Method(i)
					selection := methodSet.Lookup(method.Pkg(), method.Name())
					if selection == nil {
						continue
					}
					if fn, ok := selection.Obj().(*types.Func); ok {
						if sig, ok := fn.Type().(*types.Signature); ok {
							methods[method.Name()] = sig
							RegisterExternalTypeStubMethod(rustType, method.Name(), sig)
						}
					}
				}
				if impls[ifaceName] == nil {
					impls[ifaceName] = make(map[string]externalLocalInterfaceImpl)
				}
				impls[ifaceName][rustType] = externalLocalInterfaceImpl{
					ifaceAST:              interfaces[ifaceName],
					ifaceType:             ifaceType,
					sourcePointeeRustType: rustType,
					sourcePkgPath:         path,
					sourceTypeName:        name,
					sourceIsStub:          true,
					sourceMethods:         methods,
				}
			}
		}
	}
	return impls
}

// stubHelperMethods lists the methods each hand-written stub type defines
// (writeOsFileStub). An interface needing any other method, such as
// io.ReaderFrom, gets no impl: the stub would not have the method to call.
var stubHelperMethods = map[string]map[string]bool{
	"os.File": {"Close": true, "Read": true, "ReadAt": true, "Write": true, "WriteString": true},
}

func stubHelperHasMethods(pkgPath, typeName string, iface *types.Interface) bool {
	methods := stubHelperMethods[pkgPath+"."+typeName]
	if methods == nil {
		return false
	}
	for i := 0; i < iface.NumMethods(); i++ {
		if !methods[iface.Method(i).Name()] {
			return false
		}
	}
	return true
}
