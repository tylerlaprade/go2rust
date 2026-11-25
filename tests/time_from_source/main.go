// time translated from Go source: a Duration is an int64 that prints as Go
// prints it, Time formats, adds and compares as in Go, and timers, tickers
// and AfterFunc run on the runtime's timer thread.
package main

import (
	"fmt"
	"time"
)

func main() {
	d := 90*time.Minute + 30*time.Second + 250*time.Millisecond
	fmt.Println(d, d.Hours(), d.Minutes(), int64(d), time.Duration(1500)*time.Microsecond)
	fmt.Println(d.Truncate(time.Minute), d.Round(time.Hour), -d, time.Duration(0))
	p, err := time.ParseDuration("1h15m30.5s")
	fmt.Println(p, err, p.Seconds())
	_, err = time.ParseDuration("bogus")
	fmt.Println(err)

	t := time.Date(2024, time.February, 28, 23, 30, 0, 0, time.UTC)
	fmt.Println(t.Format("2006-01-02 15:04:05 Mon"), t.Weekday(), t.Month(), t.YearDay())
	next := t.Add(2 * time.Hour)
	fmt.Println(next.Format(time.RFC3339), next.Day(), next.Sub(t), t.Before(next), next.Equal(t))
	fmt.Println(t.AddDate(0, 1, 1).Format("Jan 2, 2006"), t.Unix(), time.Unix(0, 0).UTC())
	parsed, err := time.Parse("2006-01-02 15:04", "2023-07-04 09:15")
	fmt.Println(parsed, err, parsed.Hour(), parsed.Minute())

	start := time.Now()
	time.Sleep(10 * time.Millisecond)
	fmt.Println(time.Since(start) >= 10*time.Millisecond)

	select {
	case <-time.After(5 * time.Millisecond):
		fmt.Println("after")
	case <-time.After(time.Second):
		fmt.Println("too late")
	}
	timer := time.NewTimer(time.Hour)
	fmt.Println(timer.Stop(), timer.Stop())
	ticker := time.NewTicker(2 * time.Millisecond)
	ticks := 0
	for range ticker.C {
		ticks++
		if ticks == 3 {
			ticker.Stop()
			break
		}
	}
	fmt.Println("ticks", ticks)
	done := make(chan string)
	time.AfterFunc(time.Millisecond, func() { done <- "fired" })
	fmt.Println(<-done)
}
