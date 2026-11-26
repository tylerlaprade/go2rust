package main

import (
	"go/ast"
	"go/token"
	"sort"
	"strings"
)

var currentGotoLabelModes map[string]string

func functionHasGoto(fn *ast.FuncDecl) bool {
	if fn == nil || fn.Body == nil {
		return false
	}
	hasGoto := false
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		if hasGoto {
			return false
		}
		if branch, ok := node.(*ast.BranchStmt); ok && branch.Tok == token.GOTO {
			hasGoto = true
			return false
		}
		return true
	})
	return hasGoto
}

type gotoPlan struct {
	labelIndex       map[string]int
	firstForwardGoto map[string]int
	backward         map[string]bool
	forward          map[string]bool
	modes            map[string]string
	earlyLoopStart   map[string]int
	forwardOpen      map[string]int
}

func buildGotoPlan(stmts []ast.Stmt) gotoPlan {
	plan := gotoPlan{
		labelIndex:       make(map[string]int),
		firstForwardGoto: make(map[string]int),
		backward:         make(map[string]bool),
		forward:          make(map[string]bool),
		modes:            make(map[string]string),
	}
	labelPos := make(map[string]token.Pos)
	for i, stmt := range stmts {
		if labeled, ok := stmt.(*ast.LabeledStmt); ok {
			label := ToSnakeCase(labeled.Label.Name)
			plan.labelIndex[label] = i
			labelPos[label] = labeled.Pos()
		}
	}

	for i, stmt := range stmts {
		ast.Inspect(stmt, func(node ast.Node) bool {
			branch, ok := node.(*ast.BranchStmt)
			if !ok || branch.Tok != token.GOTO || branch.Label == nil {
				return true
			}
			label := ToSnakeCase(branch.Label.Name)
			pos, exists := labelPos[label]
			if !exists {
				return true
			}
			if pos < branch.Pos() {
				plan.backward[label] = true
				plan.modes[label] = "continue"
			} else if !plan.backward[label] {
				plan.forward[label] = true
				plan.modes[label] = "break"
				if first, ok := plan.firstForwardGoto[label]; !ok || i < first {
					plan.firstForwardGoto[label] = i
				}
			}
			return true
		})
	}
	// Each label becomes a scope: a forward label a block that a goto to
	// it breaks, from its first goto to the label; a backward label a loop
	// that a goto back to it continues, from the label to the end of the
	// list. Scopes must nest. A forward goto before a backward label, to a
	// label after it (goto Skip; Back: ...; Skip: ... goto Back), would
	// open its block outside the label's loop and close it inside. The loop
	// is entered early instead, at the forward goto, and the statements up
	// to its label run in a guard only on the first entry: a goto back to
	// the label sets its flag and skips them (gotoReenterMode).
	plan.earlyLoopStart = make(map[string]int)
	plan.forwardOpen = make(map[string]int)
	for label := range plan.forward {
		if plan.backward[label] {
			// A goto to the label from before it enters the label's loop
			// the way a goto back to it does, so the loop starts there.
			plan.earlyLoopStart[label] = plan.firstForwardGoto[label]
		} else {
			plan.forwardOpen[label] = plan.firstForwardGoto[label]
		}
	}
	settleGotoScopes(&plan)
	if !gotoScopesNest(plan, len(stmts)) {
		// Not expected once the scopes settle; keep the plain lowering.
		plan.earlyLoopStart = make(map[string]int)
		for label := range plan.forwardOpen {
			plan.forwardOpen[label] = plan.firstForwardGoto[label]
		}
	}
	for back := range plan.earlyLoopStart {
		plan.modes[back] = gotoReenterMode
	}
	return plan
}

// settleGotoScopes moves scopes until none crosses another. A scope may
// open earlier than it must: a forward block before its first goto, and a
// loop before its label, when the statements up to the label go in the
// guard of the label's first entry. Go lets no goto jump over a
// declaration into its scope, so the statements a move takes in declare
// nothing that is used past them. Starts only move earlier, so this ends.
func settleGotoScopes(plan *gotoPlan) {
	backs := sortedGotoLabels(plan.backward)
	forwards := make([]string, 0, len(plan.forwardOpen))
	for label := range plan.forwardOpen {
		forwards = append(forwards, label)
	}
	sort.Strings(forwards)
	for changed := true; changed; {
		changed = false
		startEarly := func(label string, start int) {
			if current, ok := plan.earlyLoopStart[label]; !ok || start < current {
				plan.earlyLoopStart[label] = start
				changed = true
			}
		}
		for _, fwd := range forwards {
			open, end := plan.forwardOpen[fwd], plan.labelIndex[fwd]
			for _, back := range backs {
				loop := plan.loopOpen(back)
				if open < loop && loop < end {
					// The block would close inside the loop.
					startEarly(back, open)
				}
				start, early := plan.earlyLoopStart[back]
				if !early {
					continue
				}
				label := plan.labelIndex[back]
				if start < open && open < label && label < end {
					// The block would open inside the guard and close after
					// it: open it with the guard.
					plan.forwardOpen[fwd] = start
					open = start
					changed = true
				}
			}
			for _, other := range forwards {
				otherOpen, otherEnd := plan.forwardOpen[other], plan.labelIndex[other]
				if open < otherOpen && otherOpen < end && end < otherEnd {
					plan.forwardOpen[other] = open
					changed = true
				}
			}
		}
		for _, back := range backs {
			start, early := plan.earlyLoopStart[back]
			if !early {
				continue
			}
			label := plan.labelIndex[back]
			for _, fwd := range forwards {
				open, end := plan.forwardOpen[fwd], plan.labelIndex[fwd]
				if open < start && start < end && end < label {
					// The block would close inside the guard: start the
					// guard with the block.
					startEarly(back, open)
				}
			}
			for _, other := range backs {
				if other == back {
					continue
				}
				if loop := plan.loopOpen(other); start < loop && loop < label {
					// The other loop would open inside the guard and
					// outlast it: enter it at the guard's start too.
					startEarly(other, start)
				}
			}
		}
	}
}

func sortedGotoLabels(set map[string]bool) []string {
	labels := make([]string, 0, len(set))
	for label := range set {
		labels = append(labels, label)
	}
	sort.Strings(labels)
	return labels
}

// loopOpen is the statement before which a backward label's loop opens.
func (plan gotoPlan) loopOpen(label string) int {
	if start, ok := plan.earlyLoopStart[label]; ok {
		return start
	}
	return plan.labelIndex[label]
}

// gotoScope is a scope the lowering opens: a loop, which closes after the
// last statement, or a forward block or guard, which closes before the
// statement at end.
type gotoScope struct {
	kind  gotoScopeKind
	label string
	open  int
	end   int
}

type gotoScopeKind int

const (
	gotoLoopScope gotoScopeKind = iota
	gotoBlockScope
	gotoGuardScope
)

// gotoScopeOpens lists the scopes that open before each statement, in the
// order they open: loops first, the ones whose labels come first outside,
// so a goto to a label resets the flags of the loops inside its loop; then
// the blocks and guards, the one that closes last outside.
func gotoScopeOpens(plan gotoPlan, count int) map[int][]gotoScope {
	opens := make(map[int][]gotoScope)
	for label := range plan.backward {
		open := plan.loopOpen(label)
		opens[open] = append(opens[open], gotoScope{kind: gotoLoopScope, label: label, open: open, end: count})
	}
	for label, start := range plan.earlyLoopStart {
		opens[start] = append(opens[start], gotoScope{kind: gotoGuardScope, label: label, open: start, end: plan.labelIndex[label]})
	}
	for label, open := range plan.forwardOpen {
		opens[open] = append(opens[open], gotoScope{kind: gotoBlockScope, label: label, open: open, end: plan.labelIndex[label]})
	}
	for open, scopes := range opens {
		sort.Slice(scopes, func(i, j int) bool {
			a, b := scopes[i], scopes[j]
			if (a.kind == gotoLoopScope) != (b.kind == gotoLoopScope) {
				return a.kind == gotoLoopScope
			}
			if a.kind == gotoLoopScope {
				return plan.labelIndex[a.label] < plan.labelIndex[b.label]
			}
			return a.end > b.end
		})
		opens[open] = scopes
	}
	return opens
}

// gotoScopesNest reports whether the plan's scopes, opened and closed as
// the lowering does, nest.
func gotoScopesNest(plan gotoPlan, count int) bool {
	opens := gotoScopeOpens(plan, count)
	var stack []gotoScope
	for i := 0; i <= count; i++ {
		for len(stack) > 0 && stack[len(stack)-1].kind != gotoLoopScope && stack[len(stack)-1].end == i {
			stack = stack[:len(stack)-1]
		}
		for _, scope := range stack {
			if scope.kind != gotoLoopScope && scope.end <= i {
				return false
			}
		}
		if i < count {
			stack = append(stack, opens[i]...)
		}
	}
	for _, scope := range stack {
		if scope.kind != gotoLoopScope {
			return false
		}
	}
	return true
}

// gotoReenterMode is the mode of a backward label whose loop is entered
// early: a goto to it sets the label's reentry flag and continues the loop.
const gotoReenterMode = "reenter"

// gotoReenterFlag names the flag that says a label's early loop was
// entered again by a goto to the label. Each label gets its name once, the
// first time it is asked for, apart from the file's names and the flags
// already given out, so two labels never share a flag. A label that is a
// Rust keyword arrives as a raw identifier, whose r# prefix cannot go
// inside another name.
func gotoReenterFlag(label string) string {
	if name, ok := gotoReenterFlagNames[label]; ok {
		return name
	}
	name := "__goto_reenter_" + strings.TrimPrefix(label, "r#")
	for gotoSyntheticNames[name] || gotoReenterFlagTaken[name] {
		name += "_"
	}
	if gotoReenterFlagNames == nil {
		gotoReenterFlagNames = map[string]string{}
		gotoReenterFlagTaken = map[string]bool{}
	}
	gotoReenterFlagNames[label] = name
	gotoReenterFlagTaken[name] = true
	return name
}

// gotoReenterFlagNames maps a label to its reentry flag, and
// gotoReenterFlagTaken holds the flags given out, both for the file being
// translated. Labels are a function's, so one label name in two functions
// shares a flag name, which each function declares for itself.
var (
	gotoReenterFlagNames map[string]string
	gotoReenterFlagTaken map[string]bool
)

// gotoSyntheticNames holds the Rust names of the file's identifiers and
// labels, which a synthetic goto flag or loop label must not take.
var gotoSyntheticNames map[string]bool

func collectGotoSyntheticNameConflicts(file *ast.File) map[string]bool {
	names := map[string]bool{}
	if file == nil {
		return names
	}
	ast.Inspect(file, func(node ast.Node) bool {
		if ident, ok := node.(*ast.Ident); ok {
			names[ident.Name] = true
			names[ToSnakeCase(ident.Name)] = true
			names[RustLocalIdent(ident.Name)] = true
		}
		return true
	})
	return names
}

func TranspileGotoStatementList(out *strings.Builder, stmts []ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet, comments []*ast.CommentGroup, lastPos *token.Pos, indent string) ast.Stmt {
	plan := buildGotoPlan(stmts)
	oldModes := currentGotoLabelModes
	currentGotoLabelModes = mergedGotoLabelModes(oldModes, plan.modes)
	defer func() { currentGotoLabelModes = oldModes }()

	opens := gotoScopeOpens(plan, len(stmts))
	var stack []gotoScope
	currentIndent := func() string {
		return indent + strings.Repeat("    ", len(stack))
	}
	var prevStmt ast.Stmt
	for i, stmt := range stmts {
		if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
			out.WriteString("\n")
		}
		// What ends at this statement closes before anything opens at it:
		// the block of a forward label here, and the first-entry guard of
		// an early label here.
		for len(stack) > 0 && stack[len(stack)-1].kind != gotoLoopScope && stack[len(stack)-1].end == i {
			stack = stack[:len(stack)-1]
			out.WriteString(currentIndent())
			out.WriteString("}\n")
		}
		for _, scope := range opens[i] {
			out.WriteString(currentIndent())
			switch scope.kind {
			case gotoLoopScope:
				if _, early := plan.earlyLoopStart[scope.label]; early {
					out.WriteString("let mut ")
					out.WriteString(gotoReenterFlag(scope.label))
					out.WriteString(" = false;\n")
					out.WriteString(currentIndent())
				}
				out.WriteString("'")
				out.WriteString(scope.label)
				out.WriteString(": loop {\n")
			case gotoBlockScope:
				out.WriteString("'")
				out.WriteString(scope.label)
				out.WriteString(": {\n")
			case gotoGuardScope:
				out.WriteString("if !")
				out.WriteString(gotoReenterFlag(scope.label))
				out.WriteString(" {\n")
			}
			stack = append(stack, scope)
		}
		emitted := stmt
		if labeled, ok := stmt.(*ast.LabeledStmt); ok {
			label := ToSnakeCase(labeled.Label.Name)
			if plan.backward[label] || plan.forward[label] {
				// The label's scope stands for it.
				emitted = labeled.Stmt
			}
		}
		emitGotoStatement(out, emitted, fnType, fileSet, comments, lastPos, currentIndent())
		prevStmt = stmt
	}
	var last ast.Stmt
	if len(stmts) > 0 {
		last = stmts[len(stmts)-1]
	}
	for len(stack) > 0 {
		top := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if top.kind != gotoLoopScope {
			out.WriteString(currentIndent())
			out.WriteString("}\n")
			continue
		}
		if last == nil || !stmtPreventsSyntheticBackwardBreak(last, top.label) {
			out.WriteString(currentIndent())
			out.WriteString("    break '")
			out.WriteString(top.label)
			out.WriteString(";\n")
		}
		out.WriteString(currentIndent())
		out.WriteString("};\n")
	}
	return prevStmt
}

func mergedGotoLabelModes(outer, inner map[string]string) map[string]string {
	if len(outer) == 0 {
		return inner
	}
	if len(inner) == 0 {
		return outer
	}
	merged := make(map[string]string, len(outer)+len(inner))
	for label, mode := range outer {
		merged[label] = mode
	}
	for label, mode := range inner {
		merged[label] = mode
	}
	return merged
}

func stmtListHasPlannedGoto(stmts []ast.Stmt) bool {
	plan := buildGotoPlan(stmts)
	return len(plan.modes) > 0
}

func stmtPreventsSyntheticBackwardBreak(stmt ast.Stmt, label string) bool {
	if stmtTerminates(stmt) {
		return true
	}
	switch s := stmt.(type) {
	case *ast.BranchStmt:
		return s.Tok == token.GOTO && s.Label != nil && ToSnakeCase(s.Label.Name) == label
	case *ast.BlockStmt:
		return stmtListPreventsSyntheticBackwardBreak(s.List, label)
	case *ast.IfStmt:
		if s.Else == nil {
			return false
		}
		return stmtListPreventsSyntheticBackwardBreak(s.Body.List, label) &&
			stmtPreventsSyntheticBackwardBreak(s.Else, label)
	case *ast.LabeledStmt:
		return stmtPreventsSyntheticBackwardBreak(s.Stmt, label)
	default:
		return false
	}
}

func stmtListPreventsSyntheticBackwardBreak(stmts []ast.Stmt, label string) bool {
	for _, stmt := range stmts {
		if _, ok := stmt.(*ast.EmptyStmt); ok {
			continue
		}
		if stmtPreventsSyntheticBackwardBreak(stmt, label) {
			return true
		}
		if stmtTerminates(stmt) {
			return true
		}
	}
	return false
}

func emitGotoStatement(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet, comments []*ast.CommentGroup, lastPos *token.Pos, indent string) {
	out.WriteString(indent)
	TranspileStatement(out, stmt, fnType, fileSet, comments, lastPos, indent)
	out.WriteString("\n")
}
