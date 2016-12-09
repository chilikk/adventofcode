package main

import (
	"crypto/md5"
	"fmt"
	"strconv"
)

func worker(data []byte, pass *[]byte, ctr *uint8, jobs <-chan int) {
	for i := range jobs {
		x := append(data, []byte(strconv.Itoa(i))...)
		m := md5.Sum(x)
		if m[0] == 0 && m[1] == 0 && m[2] < 8 && (*ctr)&(1<<m[2]) == 0 {
			(*pass)[m[2]] = m[3] >> 4
			(*ctr) ^= 1 << m[2]
			fmt.Printf("%s %032x %08b\n", x, m, (*ctr))
		}
	}
}

func main() {
	pass := make([]byte, 8, 8)
	data := []byte("ojvtpuvg")
	jobs := make(chan int, 10)
	var ctr uint8
	for i := 0; i < 8; i++ {
		go worker(data, &pass, &ctr, jobs)
	}
	for i := 0; ctr != 0xff; i++ {
		jobs <- i
	}
	for i := 0; i < 8; i++ {
		if pass[i] < 10 {
			pass[i] += '0'
		} else {
			pass[i] += 'a' - 10
		}
	}
	fmt.Println(string(pass))
}
