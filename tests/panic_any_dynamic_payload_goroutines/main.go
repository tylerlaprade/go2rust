package main

import (
	"errors"
	"fmt"
)

type payload struct {
	value int
}

type lazyPayload struct{}

type label string

func (*lazyPayload) String() string {
	panic("formatted while recover was still possible")
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

func formatterRunsOnlyForUncaughtPanic() (result string) {
	defer func() {
		_, ok := recover().(*lazyPayload)
		result = fmt.Sprintf("lazy %t", ok)
	}()
	panic(&lazyPayload{})
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

func explicitStringIsNotError() (result string) {
	defer func() {
		_, ok := recover().(error)
		result = fmt.Sprintf("string error %t", ok)
	}()
	panic("panic called with nil argument")
}

func recoverNamedString() (result string) {
	defer func() {
		recovered := recover()
		value, namedOK := recovered.(label)
		_, stringOK := recovered.(string)
		result = fmt.Sprintf("named %t %t %s", namedOK, stringOK, value)
	}()
	panic(label("kept"))
}

func recoverShadowedString(nil string) (result string) {
	defer func() {
		value, ok := recover().(string)
		result = fmt.Sprintf("shadow string %t %s", ok, value)
	}()
	panic(nil)
}

func recoverShadowedInt(nil int) (result string) {
	defer func() {
		value, ok := recover().(int)
		result = fmt.Sprintf("shadow int %t %d", ok, value)
	}()
	panic(nil)
}

func recoverSliceElement() (result string) {
	defer func() {
		value, ok := recover().(string)
		result = fmt.Sprintf("slice panic %t %s", ok, value)
	}()
	values := []any{"element"}
	panic(values[0])
}

func recoverNilError() (result string) {
	defer func() {
		recovered, ok := recover().(error)
		result = fmt.Sprintf("nil error %t %v", ok, recovered)
	}()
	var value error
	panic(value)
}

func recoverBoundsError() (result string) {
	defer func() {
		recovered, ok := recover().(error)
		result = fmt.Sprintf("bounds error %t %v", ok, recovered)
	}()
	values := []any{"element"}
	panic(values[1])
}

func recoverInGoroutineLiterals() {
	done := make(chan string)
	go func() {
		defer func() {
			value, ok := recover().(string)
			done <- fmt.Sprintf("go literal %t %s", ok, value)
		}()
		panic("without args")
	}()
	fmt.Println(<-done)

	go func(code int) {
		defer func() {
			value, ok := recover().(string)
			done <- fmt.Sprintf("go literal arg %d %t %s", code, ok, value)
		}()
		panic("with args")
	}(1)
	fmt.Println(<-done)

	go func() int {
		defer func() {
			value, ok := recover().(string)
			done <- fmt.Sprintf("go result recovered %t", ok && value == "result")
		}()
		panic("result")
	}()
	fmt.Println(<-done)

	go func(code int) int {
		defer func() {
			value, ok := recover().(string)
			done <- fmt.Sprintf("go result arg %d recovered %t", code, ok && value == "result arg")
		}()
		panic("result arg")
	}(2)
	fmt.Println(<-done)
}

func recoverAndReadInterface(value any) (result string) {
	saved := value
	defer func() {
		recovered := recover()
		result = fmt.Sprintf("lock %v %v", recovered, saved)
	}()
	panic(value)
}

func main() {
	// Force the synchronized representation; the paired fixture covers the
	// same panic payloads with local Rc/RefCell wrappers.
	go func() {}()

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
	fmt.Println(formatterRunsOnlyForUncaughtPanic())
	fmt.Println("dynamic", recoverNilValue(false))
	fmt.Println("literal", recoverNilValue(true))
	fmt.Println(explicitStringIsNotError())
	fmt.Println(recoverNamedString())
	fmt.Println(recoverShadowedString("shadowed"))
	fmt.Println(recoverShadowedInt(17))
	fmt.Println(recoverSliceElement())
	fmt.Println(recoverAndReadInterface("same"))
	var nilValue any
	fmt.Println(recoverAndReadInterface(nilValue))
	fmt.Println(recoverNilError())
	fmt.Println(recoverBoundsError())
	recoverInGoroutineLiterals()
}
