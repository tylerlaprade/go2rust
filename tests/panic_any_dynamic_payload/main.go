package main

import (
	"errors"
	"fmt"
)

type payload struct {
	value int
}

func classifyRecovered(result *string, original *payload) {
	switch recovered := recover().(type) {
	case error:
		*result = fmt.Sprintf("error true %s", recovered.Error())
	case payload:
		recovered.value = 11
		*result = fmt.Sprintf("struct true %d %d", recovered.value, original.value)
	case *payload:
		*result = fmt.Sprintf("pointer true %t %d", recovered == original, recovered.value)
		recovered.value = 10
	case int:
		*result = fmt.Sprintf("int true %d", recovered)
	case string:
		*result = fmt.Sprintf("string true %s", recovered)
	default:
		*result = "unknown false"
	}
}

func recoverValue(value any, original *payload) (result string) {
	defer classifyRecovered(&result, original)
	panic(value)
}

func printRecovered(label string) {
	value, ok := recover().(string)
	fmt.Println(label, ok, value)
}

func innerPanic() {
	defer printRecovered("inner")
	panic("B")
}

func outerPanic() {
	defer func() {
		innerPanic()
		fmt.Println("after inner")
	}()
	panic("A")
}

func saveOuterPanic(result *string) {
	value, ok := recover().(string)
	if ok {
		*result = fmt.Sprintf("%s", value)
	}
}

func nestedPanic() (result string) {
	defer saveOuterPanic(&result)
	outerPanic()
	return
}

func explicitRuntimeText() (result string) {
	defer saveOuterPanic(&result)
	panic("attempt to divide by zero")
}

func panicB() {
	panic("B")
}

func replacementPanic() {
	defer printRecovered("replacement")
	defer panicB()
	panic("A")
}

func saveProbe(result *string) {
	value := recover()
	if value == nil {
		*result = "nil"
		return
	}
	*result = fmt.Sprintf("%v", value)
}

func probe() (result string) {
	defer saveProbe(&result)
	return
}

func normalProbeCannotRecoverOuter() (outer, probed string) {
	defer saveOuterPanic(&outer)
	defer func() {
		probed = probe()
	}()
	panic("outer")
}

func indirectRecover() any {
	return recover()
}

func onlyDirectDeferredCallCanRecover() (result string) {
	defer func() {
		indirect := indirectRecover()
		direct := recover()
		result = fmt.Sprintf("depth %t %v", indirect == nil, direct)
	}()
	panic("direct")
}

func recoverNilValue(useLiteral bool) (result string) {
	defer func() {
		recovered := recover()
		err, ok := recovered.(error)
		if !ok {
			result = "nil false"
			return
		}
		result = fmt.Sprintf("nil %t %s", ok, err.Error())
	}()
	if useLiteral {
		panic(nil)
	}
	var value any
	panic(value)
}

func main() {
	fmt.Println(recoverValue(errors.New("boom"), nil))
	structValue := payload{7}
	fmt.Println(recoverValue(structValue, &structValue))

	pointerValue := &payload{8}
	fmt.Println(recoverValue(pointerValue, pointerValue))
	fmt.Println("pointer write", pointerValue.value)

	fmt.Println(recoverValue(9, nil))
	fmt.Println(recoverValue("text", nil))
	fmt.Println("literal", explicitRuntimeText())
	fmt.Println("top", nestedPanic())
	replacementPanic()
	fmt.Println("probe", probe())
	outer, probed := normalProbeCannotRecoverOuter()
	fmt.Println("scoped", outer, probed)
	fmt.Println(onlyDirectDeferredCallCanRecover())
	fmt.Println("dynamic", recoverNilValue(false))
	fmt.Println("literal", recoverNilValue(true))
}
