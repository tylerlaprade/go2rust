package main

type siblingError struct{}

func (receiver *siblingError) Error() string {
	if receiver == nil {
		return "sibling-nil"
	}
	return "sibling"
}
