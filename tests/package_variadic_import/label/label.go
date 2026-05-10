package label

import "fmt"

func Count(prefix string, labels ...string) string {
	return fmt.Sprintf("%s:%d", prefix, len(labels))
}
