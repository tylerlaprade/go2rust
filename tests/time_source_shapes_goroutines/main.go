package main

import (
	"errors"
	"fmt"
	"sync"
	"syscall"
)

// Shapes found compiling the time package from its Go source.

type Duration int64

const (
	minDuration Duration = -1 << 63
	maxDuration Duration = 1<<63 - 1
	Second      Duration = 1000000000
	maxUint32   uint64   = (1<<64 - 1) >> 32
)

func (d Duration) Seconds() float64 {
	sec := d / Second
	nsec := d % Second
	return float64(sec) + float64(nsec)/1e9
}

type Stamp struct {
	Sec  int64
	Nsec int32
}

// A float literal used as an integer: 1e9 is the integer 1000000000.
func (t Stamp) Add(d Duration) Stamp {
	dsec := int64(d / 1e9)
	nsec := t.Nsec + int32(d%1e9)
	if nsec >= 1e9 {
		dsec++
		nsec -= 1e9
	}
	t.Sec += dsec
	t.Nsec = nsec
	return t
}

func (t *Stamp) sec() int64 { return t.Sec }

func parseStamp(s string) (Stamp, error) {
	if s == "" {
		return Stamp{}, errors.New("empty")
	}
	return Stamp{int64(len(s)), 7}, nil
}

// *t, err = f() in a pointer method.
func (t *Stamp) UnmarshalText(data string) error {
	var err error
	*t, err = parseStamp(data)
	return err
}

// A case clause declares a variable of an outer one's name.
func split(t Stamp, d Duration) (uint64, int64) {
	sec := t.sec()
	switch {
	case d < Second && Second%(d+d) == 0:
		return 0, sec
	default:
		sec := uint64(sec)
		return sec >> 1, 0
	}
}

func label(n int) string {
	hour := n
	note := ""
	switch {
	case n > 12:
		var hour string
		hour = "pm"
		note = hour
	}
	if note != "" {
		hour += 12
	}
	return fmt.Sprint(hour, note)
}

// A tail return reads a parameter the body reassigns.
func byteString(p []byte) string {
	for i := 0; i < len(p); i++ {
		if p[i] == 0 {
			p = p[0:i]
			break
		}
	}
	return string(p)
}

func open(name string) (uintptr, error) {
	if name == "" {
		return 0, syscall.ENOENT
	}
	return uintptr(len(name)), nil
}

func closefd(fd uintptr) { fmt.Println("close", fd) }

// defer captures a bare result of a (uintptr, error) call.
func load(name string) (int, error) {
	fd, err := open(name)
	if err != nil {
		return 0, err
	}
	defer closefd(fd)
	return int(fd) * 2, nil
}

func main() {
	fmt.Println(minDuration, maxDuration, maxUint32, Duration(2500000000).Seconds())
	fmt.Println(Stamp{1, 900000000}.Add(2500000000))

	var s Stamp
	fmt.Println(s.UnmarshalText("abc"), s)
	fmt.Println(s.UnmarshalText(""), s)

	fmt.Println(split(Stamp{Sec: 10}, 3))
	fmt.Println(split(Stamp{Sec: 10}, Second/4))
	fmt.Println(label(13), label(2))

	fmt.Println(byteString([]byte{'g', 'o', 0, 'x'}), byteString([]byte("rust")))

	fmt.Println(load("zone"))
	n, err := load("")
	fmt.Println(n, err != nil, err == syscall.ENOENT, err != syscall.EINVAL)

	var wg sync.WaitGroup
	results := make([]string, 2)
	for i, name := range []string{"tz", ""} {
		wg.Add(1)
		go func(i int, name string) {
			defer wg.Done()
			n, err := load(name)
			results[i] = fmt.Sprint(n, err == nil)
		}(i, name)
	}
	wg.Wait()
	fmt.Println(results[0], "|", results[1])
}
