package check

func Preserved(value any) bool {
	copied := value
	value = nil
	return copied != nil
}
