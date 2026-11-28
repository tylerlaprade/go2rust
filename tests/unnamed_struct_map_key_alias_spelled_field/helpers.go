package main

type Count = int

func origin() struct{ X Count } { return struct{ X Count }{0} }
