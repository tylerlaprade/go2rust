package main

import "fmt"

func main() {
	fields := make(map[string]map[string]string)
	typeName := "Thing"
	fieldName := "Name"

	if fields[typeName] == nil {
		fields[typeName] = make(map[string]string)
	}
	fields[typeName][fieldName] = "string"

	conversions := make(map[string]map[string]bool)
	conversions["Target"] = make(map[string]bool)
	conversions["Target"]["Source"] = true

	count := 0
	for targetName, sourceNames := range conversions {
		if len(sourceNames) == 0 {
			continue
		}
		if sourceNames["Source"] {
			count += len(targetName)
		}
	}

	buckets := make(map[int][]string)
	buckets[1] = []string{"alpha", "beta"}
	for _, name := range buckets[1] {
		count += len(name)
	}

	missing := fields["Missing"]
	fmt.Println(fields["Thing"]["Name"])
	fmt.Println(missing == nil)
	fmt.Println(count)
}
