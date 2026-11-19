package dep

type Node struct{ Value int }

var Current = &Node{Value: 7}
var Other = &Node{Value: 9}

func Get() *Node { return Current }

type Reader interface{ Read() int }

func (n *Node) Read() int { return n.Value }

func View() Reader { return Current }
