package main

import "go/types"

// An interface that embeds no other interface becomes a Rust trait that
// declares `__go_as_any`. An interface that embeds others inherits the method
// from its supertraits instead. When it embeds two or more interfaces that each
// declare the method (`type ReadWriter interface { Reader; Writer }`), Rust
// method syntax on its trait object is ambiguous (E0034), so calls name one of
// the declaring traits: `Reader::__go_as_any(x)`. Every declaring trait's impl
// returns the same concrete value, so any of them gives the same answer.

// goAsAnyDeclaringTraits returns the Rust names of the traits that declare
// `__go_as_any` for an interface whose trait is named selfName: the interface
// itself when it embeds no transpiled interface, otherwise the declaring traits
// of what it embeds, in embedding order and without repeats.
func goAsAnyDeclaringTraits(selfName string, iface *types.Interface) []string {
	var roots []string
	seen := map[string]bool{}
	visiting := map[*types.Interface]bool{}
	var visit func(name string, it *types.Interface)
	visit = func(name string, it *types.Interface) {
		if it == nil || visiting[it] {
			return
		}
		visiting[it] = true
		defer delete(visiting, it)
		embeddedTraits := 0
		for i := 0; i < it.NumEmbeddeds(); i++ {
			embedded, ok := types.Unalias(it.EmbeddedType(i)).(*types.Named)
			if !ok {
				continue
			}
			embeddedIface, ok := embedded.Underlying().(*types.Interface)
			if !ok {
				continue
			}
			embeddedName, ok := transpiledNamedInterfaceTypeNameFromTypes(embedded)
			if !ok {
				continue
			}
			embeddedTraits++
			visit(embeddedName, embeddedIface)
		}
		if embeddedTraits == 0 && !seen[name] {
			seen[name] = true
			roots = append(roots, name)
		}
	}
	visit(selfName, iface)
	return roots
}

// goAsAnyQualifyingTrait returns the trait to name in a `__go_as_any` call on
// a trait object of the given interface type, or "" when method syntax is
// unambiguous.
func goAsAnyQualifyingTrait(typ types.Type) string {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return ""
	}
	iface, ok := named.Underlying().(*types.Interface)
	if !ok {
		return ""
	}
	name, ok := transpiledNamedInterfaceTypeNameFromTypes(named)
	if !ok {
		return ""
	}
	return goAsAnyQualifyingTraitFor(name, iface)
}

// goAsAnyQualifyingTraitFor is goAsAnyQualifyingTrait for an interface whose
// Rust trait name and go/types interface are already known.
func goAsAnyQualifyingTraitFor(traitName string, iface *types.Interface) string {
	roots := goAsAnyDeclaringTraits(traitName, iface)
	if len(roots) < 2 {
		return ""
	}
	return roots[0]
}

// goAsAnyQualifyingTraitByName is goAsAnyQualifyingTrait for a local interface
// known by its Go name.
func goAsAnyQualifyingTraitByName(ifaceName string) string {
	named, _ := localInterfaceNamedTypeByName(ifaceName)
	if named == nil {
		return ""
	}
	return goAsAnyQualifyingTrait(named)
}

// goAsAnyCall returns a Rust expression of type &dyn Any for a trait object of
// the named interface. methodRecv is written before `.__go_as_any()` when
// method syntax is unambiguous; refRecv, an expression of type &dyn Trait, is
// passed to the qualified call otherwise.
func goAsAnyCall(qualifyingTrait string, methodRecv string, refRecv string) string {
	if qualifyingTrait == "" {
		return methodRecv + ".__go_as_any()"
	}
	return qualifyingTrait + "::__go_as_any(" + refRecv + ")"
}

// localInterfaceSupportQualifyingTrait picks the qualifying trait for an
// interface support impl, which may know the interface only by name.
func localInterfaceSupportQualifyingTrait(ifaceName string, ifaceType *types.Interface) string {
	if ifaceType != nil {
		return goAsAnyQualifyingTraitFor(ifaceName, ifaceType)
	}
	return goAsAnyQualifyingTraitByName(ifaceName)
}
