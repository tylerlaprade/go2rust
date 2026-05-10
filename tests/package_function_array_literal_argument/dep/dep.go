package dep

type Item struct {
	V int
}

func Of(v int) Item {
	return Item{V: v}
}

func Make(static [3]Item, labels []Item) int {
	total := 0
	for _, item := range static {
		total += item.V
	}
	for _, item := range labels {
		total += item.V
	}
	return total
}
