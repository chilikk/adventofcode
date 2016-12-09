package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
)

func main() {
	var maps [][]int
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	line := scanner.Text()
	maps = make([][]int, len(line), len(line))
	for i := 0; i < len(line); i++ {
		maps[i] = make([]int, 'z'-'a'+1, 'z'-'a'+1)
	}
	for i, c := range line {
		maps[i][c-'a']++
	}
	for scanner.Scan() {
		line := scanner.Text()
		for i, c := range line {
			maps[i][c-'a']++
		}
	}
	var bufmax, bufmin bytes.Buffer
	for _, map1 := range maps {
		max := -1
		maxc := -1
		min := 1000
		minc := -1
		for c, freq := range map1 {
			if freq > max {
				max = freq
				maxc = c
			}
			if freq != 0 && freq < min {
				min = freq
				minc = c
			}
		}
		bufmax.WriteByte(byte(maxc + 'a'))
		bufmin.WriteByte(byte(minc + 'a'))
	}
	fmt.Println(bufmax.String(), bufmin.String())
}
