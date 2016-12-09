package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"strconv"
)

func main() {
	var acc int
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		line := scanner.Text()
		text, num, checksum := parse(line)
		if check(text, checksum) {
			fmt.Println(decode(text, num), num)
		}
	}
	fmt.Println("Result", acc)
}

func parse(line string) (text string, num int, checksum string) {
	state := 1 // name
	var buf bytes.Buffer
	for _, c := range line {
		if state == 1 && (c >= 'a' && c <= 'z' || c == '-') {
			buf.WriteRune(c)
		} else if state == 1 && c >= '0' && c <= '9' {
			text = buf.String()
			buf.Reset()
			state = 2
			buf.WriteRune(c)
		} else if state == 2 && c >= '0' && c <= '9' {
			buf.WriteRune(c)
		} else if state == 2 && c == '[' {
			num, _ = strconv.Atoi(buf.String())
			buf.Reset()
			state = 3
		} else if state == 3 && c >= 'a' && c <= 'z' {
			buf.WriteRune(c)
		}
	}
	checksum = buf.String()
	return
}

func check(text, checksum string) bool {
	freq := make([]int, 'z'-'a'+1, 'z'-'a'+1)
	for _, c := range text {
		if c != '-' {
			freq[c-'a'] += 1
		}
	}
	for _, c := range checksum {
		var max, maxi int
		for i, n := range freq {
			if n > max {
				maxi = i
				max = n
			}
		}
		freq[maxi] = -1
		if maxi+'a' != int(c) {
			return false
		}
	}
	return true
}

func decode(text string, num int) string {
	var buf bytes.Buffer
	for _, c := range text {
		if c == '-' {
			buf.WriteRune(' ')
		} else {
			buf.WriteByte(byte((int(c)-'a'+num)%('z'-'a'+1) + 'a'))
		}
	}
	return buf.String()
}
