// sort's own slice types: a conversion to sort.IntSlice or StringSlice
// passed as a sort.Interface, sort.Reverse over one, and a literal of
// sort.Float64Slice with its methods.
package main

import (
	"fmt"
	"sort"
)

type byLen []string

func (s byLen) Len() int           { return len(s) }
func (s byLen) Less(i, j int) bool { return len(s[i]) < len(s[j]) }
func (s byLen) Swap(i, j int)      { s[i], s[j] = s[j], s[i] }

func main() {
	nums := []int{5, 2, 8, 1}
	sort.Sort(sort.Reverse(sort.IntSlice(nums)))
	fmt.Println(nums)
	words := []string{"ccc", "a", "bb"}
	sort.Sort(sort.Reverse(byLen(words)))
	fmt.Println(words)
	strs := []string{"b", "c", "a"}
	sort.Sort(sort.StringSlice(strs))
	fmt.Println(strs, sort.IsSorted(sort.StringSlice(strs)))
	fs := sort.Float64Slice{2.5, 1.5, 2}
	fs.Sort()
	fmt.Println(fs, fs.Len(), fs.Search(2), sort.SearchInts([]int{1, 3, 5}, 4))
	is := sort.IntSlice{3, 1, 2}
	sort.Stable(sort.Reverse(is))
	fmt.Println(is, sort.IsSorted(is))
}
