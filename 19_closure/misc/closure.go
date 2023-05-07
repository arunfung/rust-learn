package main

import "fmt"
import "time"

func intSeq() func() int {
	i := 0
	return func() int {
		i++
		return i
	}
}

func main() {
	nextInt := intSeq()

    go nextInt()

    time.Sleep(1 * time.Second)
	fmt.Println(nextInt())
	fmt.Println(nextInt())

	newInts := intSeq()
	fmt.Println(newInts()) // 1
}
