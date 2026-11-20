package main

import "fmt"

type notFound struct{ name string }

func (e notFound) Error() string { return e.name + " not found" }

type pointerError struct{ code int }

func (e *pointerError) Error() string { return fmt.Sprintf("pointer %d", e.code) }

func errorText(v any) (bool, string) {
	switch err := v.(type) {
	case error:
		return true, err.Error()
	default:
		return false, ""
	}
}

func replaceThenErrorText(v any, p *pointerError) string {
	switch err := v.(type) {
	case error:
		*p = pointerError{10}
		return err.Error()
	default:
		return "not error"
	}
}

func isError(v any) bool {
	switch v.(type) {
	case error:
		return true
	default:
		return false
	}
}

func run() {
	var value any = notFound{"key"}
	valueErr, valueOK := value.(error)
	valueCase, valueText := errorText(value)
	fmt.Println(valueOK, valueCase, valueErr.Error(), value.(error).Error())

	pointerValue := &pointerError{7}
	var pointer any = pointerValue
	pointerErr, pointerOK := pointer.(error)
	pointerCase, pointerText := errorText(pointer)
	fmt.Println(pointerOK, pointerCase, pointerText, pointer.(error).Error())
	*pointerValue = pointerError{8}
	fmt.Println("assertion replacement", pointerErr.Error())
	fmt.Println("switch replacement", replaceThenErrorText(pointer, pointerValue))

	var valuePointer any = &notFound{"pointer key"}
	valuePointerErr, valuePointerOK := valuePointer.(error)
	valuePointerCase, _ := errorText(valuePointer)
	fmt.Println(valuePointerOK, valuePointerCase, valuePointerErr.Error())

	var nilPointer *pointerError
	var typedNil any = nilPointer
	typedNilErr, typedNilOK := typedNil.(error)
	typedNilCase := isError(typedNil)
	fmt.Println(typedNilOK, typedNilCase, typedNilErr == nil)

	var text any = "plain"
	_, textOK := text.(error)
	textCase, _ := errorText(text)
	textErr, textErrOK := text.(error)
	fmt.Println(textOK, textCase, textErrOK, textErr == nil)

	_ = valueText
	_ = pointerErr
}

func main() {
	done := make(chan struct{})
	go func() {
		run()
		close(done)
	}()
	<-done
}
