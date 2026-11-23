// A query builder whose methods return the receiver, variadic ...any
// arguments stored in a map, pointers shared between a slice and maps, and
// a Write method that fmt.Fprintf writes through.
package main

import (
	"fmt"
	"sort"
	"strings"
)

type Query struct {
	table  string
	wheres []string
	order  string
	limit  int
	params map[string]any
}

func From(t string) *Query { return &Query{table: t} }

func (q *Query) Where(cond string, args ...any) *Query {
	q.wheres = append(q.wheres, cond)
	if q.params == nil {
		q.params = map[string]any{}
	}
	for i, a := range args {
		q.params[fmt.Sprintf("p%d", len(q.params)+i)] = a
	}
	return q
}

func (q *Query) OrderBy(col string) *Query { q.order = col; return q }
func (q *Query) Limit(n int) *Query        { q.limit = n; return q }

func (q *Query) String() string {
	var b strings.Builder
	fmt.Fprintf(&b, "SELECT * FROM %s", q.table)
	if len(q.wheres) > 0 {
		b.WriteString(" WHERE " + strings.Join(q.wheres, " AND "))
	}
	if q.order != "" {
		b.WriteString(" ORDER BY " + q.order)
	}
	if q.limit > 0 {
		fmt.Fprintf(&b, " LIMIT %d", q.limit)
	}
	return b.String()
}

type User struct {
	Name  string
	Age   int
	Roles []string
}

func (u *User) String() string { return fmt.Sprintf("%s(%d)", u.Name, u.Age) }

type Writer interface {
	Write(p []byte) (int, error)
}

type upperWriter struct {
	buf []byte
}

func (w *upperWriter) Write(p []byte) (int, error) {
	w.buf = append(w.buf, []byte(strings.ToUpper(string(p)))...)
	return len(p), nil
}

func emit(w Writer, parts ...string) int {
	total := 0
	for _, p := range parts {
		n, _ := w.Write([]byte(p))
		total += n
	}
	return total
}

func main() {
	q := From("users").Where("age > ?", 18).Where("name LIKE ?", "a%").OrderBy("name").Limit(10)
	fmt.Println(q)
	keys := make([]string, 0)
	for k := range q.params {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		fmt.Print(k, "=", q.params[k], " ")
	}
	fmt.Println()

	users := []*User{{"bob", 30, nil}, {"al", 25, []string{"admin"}}, {"cy", 35, []string{"dev", "ops"}}}
	sort.Slice(users, func(i, j int) bool { return users[i].Age < users[j].Age })
	fmt.Println(users[0], users[1].String(), len(users))
	byRole := map[string][]*User{}
	for _, u := range users {
		for _, r := range u.Roles {
			byRole[r] = append(byRole[r], u)
		}
	}
	byRole["admin"][0].Age++
	fmt.Println(users[0].Age, byRole["admin"][0], len(byRole["dev"]))
	idx := map[string]*User{}
	for _, u := range users {
		idx[u.Name] = u
	}
	idx["cy"].Roles = append(idx["cy"].Roles, "lead")
	fmt.Println(users[2].Roles, idx["bob"])

	w := &upperWriter{}
	n := emit(w, "hello ", "world")
	fmt.Println(n, string(w.buf))
	fmt.Fprintf(w, " %d!", 42)
	fmt.Println(string(w.buf))
}
