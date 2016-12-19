package main

import (
	"fmt"
)

func main() {
	const NUM = 3014603
	//const NUM = 5
	elves := make([]int, NUM, NUM)
	for i := 0; i < NUM; i++ {
		elves[i] = i + 1
	}
	for i := 0; len(elves) > 1; i = (i + 1) % len(elves) {
		j := (i + len(elves)/2) % len(elves)
		elves = append(elves[:j], elves[j+1:]...)
		if j < i {
			i--
		}
		fmt.Println(len(elves))
	}
	fmt.Println(elves[0])
}
