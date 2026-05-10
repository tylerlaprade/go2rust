package dep

type Exporter func(int) int

func Set(e Exporter) int {
	return e(3)
}
