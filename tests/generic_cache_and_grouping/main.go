// A generic Cache[K, V] of structs and of slices, a SumBy whose result
// type is a union constraint, instantiated with int, float64 and a defined
// float type, and a GroupBy that returns a map[K][]T.
package main

import (
	"errors"
	"fmt"
	"sort"
	"strings"
)

type Cache[K comparable, V any] struct {
	data  map[K]V
	order []K
	limit int
}

func NewCache[K comparable, V any](limit int) *Cache[K, V] {
	return &Cache[K, V]{data: make(map[K]V), limit: limit}
}

func (c *Cache[K, V]) Put(k K, v V) {
	if _, ok := c.data[k]; !ok {
		c.order = append(c.order, k)
	}
	c.data[k] = v
	if len(c.order) > c.limit {
		oldest := c.order[0]
		c.order = c.order[1:]
		delete(c.data, oldest)
	}
}

func (c *Cache[K, V]) Get(k K) (V, error) {
	v, ok := c.data[k]
	if !ok {
		var zero V
		return zero, errors.New("miss")
	}
	return v, nil
}

type User struct {
	Name string
	Age  int
}

type Number interface {
	~int | ~int64 | ~float64
}

func SumBy[T any, N Number](xs []T, f func(T) N) N {
	var total N
	for _, x := range xs {
		total += f(x)
	}
	return total
}

func GroupBy[T any, K comparable](xs []T, key func(T) K) map[K][]T {
	out := make(map[K][]T)
	for _, x := range xs {
		k := key(x)
		out[k] = append(out[k], x)
	}
	return out
}

type Celsius float64

func main() {
	c := NewCache[string, User](2)
	c.Put("a", User{"Ann", 30})
	c.Put("b", User{"Bob", 25})
	c.Put("c", User{"Cy", 40})
	_, err := c.Get("a")
	u, err2 := c.Get("c")
	fmt.Println(err, u, err2, len(c.data), c.order)

	users := []User{{"Ann", 30}, {"Bob", 25}, {"Cy", 40}, {"Di", 25}}
	fmt.Println(SumBy(users, func(u User) int { return u.Age }))
	fmt.Println(SumBy(users, func(u User) float64 { return float64(u.Age) / 2 }))
	temps := []Celsius{20.5, 22, 19.5}
	fmt.Println(SumBy(temps, func(c Celsius) Celsius { return c }))
	groups := GroupBy(users, func(u User) int { return u.Age })
	ages := make([]int, 0, len(groups))
	for age := range groups {
		ages = append(ages, age)
	}
	sort.Ints(ages)
	for _, age := range ages {
		var names []string
		for _, u := range groups[age] {
			names = append(names, u.Name)
		}
		fmt.Println(age, strings.Join(names, ","))
	}
	ic := NewCache[int, []string](3)
	ic.Put(1, []string{"x"})
	got, _ := ic.Get(1)
	fmt.Println(got, len(got))
}
