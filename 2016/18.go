package main

import (
	"fmt"
)

func main() {
	ROWS := 400000
	row := "^.^^^.^..^....^^....^^^^.^^.^...^^.^.^^.^^.^^..^.^...^.^..^.^^.^..^.....^^^.^.^^^..^^...^^^...^...^."
	//row := ".^^.^.^^^^"
	rows := make([][]byte, ROWS, ROWS)
	rows[0] = []byte(row)
	for i := 1; i < ROWS; i++ {
		rows[i] = make([]byte, len(row), len(row))
		for j := 0; j < len(row); j++ {
			left := byte('.')
			right := byte('.')
			if j > 0 {
				left = rows[i-1][j-1]
			}
			if j < len(row)-1 {
				right = rows[i-1][j+1]
			}
			rows[i][j] = '.'
			if (left == '^' && right == '.') || (right == '^' && left == '.') {
				rows[i][j] = '^'
			}
		}
	}
	count := 0
	for i := 0; i < ROWS; i++ {
		for j := 0; j < len(row); j++ {
			if rows[i][j] == byte('.') {
				count += 1
			}
		}
	}

	fmt.Println(count)
}
