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
	lastBackwardGoto map[string]int
	backward         map[string]bool
	forward          map[string]bool
	modes            map[string]string
}

func buildGotoPlan(stmts []ast.Stmt) gotoPlan {
	plan := gotoPlan{
		labelIndex:       make(map[string]int),
		firstForwardGoto: make(map[string]int),
		lastBackwardGoto: make(map[string]int),
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
				if last, ok := plan.lastBackwardGoto[label]; !ok || i > last {
					plan.lastBackwardGoto[label] = i
				}
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
	for label := range plan.backward {
		plan.lastBackwardGoto[label] = len(stmts) - 1
	}
	return plan
}

func TranspileGotoStatementList(out *strings.Builder, stmts []ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet, comments []*ast.CommentGroup, lastPos *token.Pos, indent string) ast.Stmt {
	plan := buildGotoPlan(stmts)
	oldModes := currentGotoLabelModes
	currentGotoLabelModes = mergedGotoLabelModes(oldModes, plan.modes)
	defer func() { currentGotoLabelModes = oldModes }()

	var prevStmt ast.Stmt
	forwardStarts := forwardLabelStarts(plan)
	type backwardBlock struct {
		label string
		end   int
	}
	var forwardStack []string
	var backwardStack []backwardBlock
	currentIndent := func() string {
		return indent + strings.Repeat("    ", len(forwardStack)+len(backwardStack))
	}
	closeForwardLabel := func(label string) {
		for len(forwardStack) > 0 {
			top := forwardStack[len(forwardStack)-1]
			out.WriteString(indent)
			out.WriteString(strings.Repeat("    ", len(forwardStack)+len(backwardStack)-1))
			out.WriteString("}\n")
			forwardStack = forwardStack[:len(forwardStack)-1]
			if top == label {
				return
			}
		}
	}
	openBackwardLabel := func(label string) {
		out.WriteString(currentIndent())
		out.WriteString("'")
		out.WriteString(label)
		out.WriteString(": loop {\n")
		backwardStack = append(backwardStack, backwardBlock{
			label: label,
			end:   plan.lastBackwardGoto[label],
		})
	}
	closeBackwardLabelsEndingAt := func(index int) {
		for len(backwardStack) > 0 && backwardStack[len(backwardStack)-1].end == index {
			top := backwardStack[len(backwardStack)-1]
			if index < 0 || index >= len(stmts) || !stmtPreventsSyntheticBackwardBreak(stmts[index], top.label) {
				out.WriteString(currentIndent())
				out.WriteString("break '")
				out.WriteString(top.label)
				out.WriteString(";\n")
			}
			out.WriteString(indent)
			out.WriteString(strings.Repeat("    ", len(forwardStack)+len(backwardStack)-1))
			out.WriteString("};\n")
			backwardStack = backwardStack[:len(backwardStack)-1]
		}
	}

	for i := 0; i < len(stmts); {
		stmt := stmts[i]
		if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
			out.WriteString("\n")
		}
		for _, label := range forwardStarts[i] {
			out.WriteString(currentIndent())
			out.WriteString("'")
			out.WriteString(label)
			out.WriteString(": {\n")
			forwardStack = append(forwardStack, label)
		}
		if labeled, ok := stmt.(*ast.LabeledStmt); ok {
			label := ToSnakeCase(labeled.Label.Name)
			if plan.backward[label] {
				openBackwardLabel(label)
				emitGotoStatement(out, labeled.Stmt, fnType, fileSet, comments, lastPos, currentIndent())
				closeBackwardLabelsEndingAt(i)
				prevStmt = stmt
				i++
				continue
			}
			if plan.forward[label] {
				closeForwardLabel(label)
				emitGotoStatement(out, labeled.Stmt, fnType, fileSet, comments, lastPos, currentIndent())
				prevStmt = labeled
				i++
				continue
			}
			emitGotoStatement(out, labeled.Stmt, fnType, fileSet, comments, lastPos, currentIndent())
			prevStmt = stmt
			i++
			continue
		}
		emitGotoStatement(out, stmt, fnType, fileSet, comments, lastPos, currentIndent())
		closeBackwardLabelsEndingAt(i)
		prevStmt = stmt
		i++
	}
	for len(backwardStack) > 0 {
		top := backwardStack[len(backwardStack)-1]
		if len(stmts) == 0 || !stmtPreventsSyntheticBackwardBreak(stmts[len(stmts)-1], top.label) {
			out.WriteString(currentIndent())
			out.WriteString("break '")
			out.WriteString(top.label)
			out.WriteString(";\n")
		}
		out.WriteString(indent)
		out.WriteString(strings.Repeat("    ", len(forwardStack)+len(backwardStack)-1))
		out.WriteString("};\n")
		backwardStack = backwardStack[:len(backwardStack)-1]
	}
	for len(forwardStack) > 0 {
		closeForwardLabel(forwardStack[len(forwardStack)-1])
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

func forwardLabelStarts(plan gotoPlan) map[int][]string {
	starts := make(map[int][]string)
	for label := range plan.forward {
		start, ok := plan.firstForwardGoto[label]
		if !ok {
			continue
		}
		starts[start] = append(starts[start], label)
	}
	for start := range starts {
		labels := starts[start]
		sort.Slice(labels, func(i, j int) bool {
			return plan.labelIndex[labels[i]] > plan.labelIndex[labels[j]]
		})
		starts[start] = labels
	}
	return starts
}

func findNextForwardLabel(stmts []ast.Stmt, plan gotoPlan, start int) int {
	for i := start + 1; i < len(stmts); i++ {
		labeled, ok := stmts[i].(*ast.LabeledStmt)
		if !ok {
			continue
		}
		label := ToSnakeCase(labeled.Label.Name)
		if plan.forward[label] {
			return i
		}
		if plan.backward[label] {
			return -1
		}
	}
	return -1
}

func findFirstGotoToLabel(stmts []ast.Stmt, label string, start, end int) int {
	for i := start; i < end; i++ {
		if stmtContainsGotoToLabel(stmts[i], label) {
			return i
		}
	}
	return start
}

func stmtContainsGotoToLabel(stmt ast.Stmt, label string) bool {
	found := false
	ast.Inspect(stmt, func(node ast.Node) bool {
		if found {
			return false
		}
		branch, ok := node.(*ast.BranchStmt)
		if !ok || branch.Tok != token.GOTO || branch.Label == nil {
			return true
		}
		if ToSnakeCase(branch.Label.Name) == label {
			found = true
			return false
		}
		return true
	})
	return found
}

func writeBlankLineBetweenGotoStatements(out *strings.Builder, prevStmt, stmt ast.Stmt, fileSet *token.FileSet) {
	if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
		out.WriteString("\n")
	}
}

func emitGotoStatement(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet, comments []*ast.CommentGroup, lastPos *token.Pos, indent string) {
	out.WriteString(indent)
	TranspileStatement(out, stmt, fnType, fileSet, comments, lastPos, indent)
	out.WriteString("\n")
}
