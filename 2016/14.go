package main

import (
	"crypto/md5"
	"fmt"
	"strconv"
)

func getNext(data []byte, ctr int) [16]byte {
	x := append(data, []byte(strconv.Itoa(ctr))...)
	md5sum := md5.Sum(x)
	for i := 0; i < 2016; i++ {
		md5sum = md5.Sum(hexMD5(&md5sum))
	}
	return md5sum
}

func isNRow(n int, md5 [16]byte, symbol int, checksymbol bool) (int, bool) {
	// 1 hex digit = 4 bit
	// 1 md5sum = 128 bit
	pre := make([]int, n-1, n-1)
	for i := 0; i < 128; i += 4 {
		// 1 byte = 8 bit
		cur := int((md5[i/8] >> uint8((i+4)%8)) & 0xf)
		if i >= (n-1)*4 && (!checksymbol || cur == symbol) {
			for j, v := range pre {
				if j == n-2 && v == cur {
					return cur, true
				} else if v != cur {
					break
				}
			}
		}
		pre = append(pre[1:], cur)
	}
	return 0, false
}

func int2hex(digit byte) byte {
	if digit < 10 {
		return digit + '0'
	} else {
		return digit - 10 + 'a'
	}
}

func hexMD5(md5 *[16]byte) []byte {
	var str [32]byte
	for i, x := range *md5 {
		str[2*i] = int2hex((x >> 4) & 0xf)
		str[2*i+1] = int2hex(x & 0xf)
	}
	return str[:]
}

func main() {
	data := []byte("ihaygndm")
	var ctr1, ctr2, nkeys int
	storage := make([][16]byte, 0, 1000)
	for nkeys < 64 {
		var curmd5 [16]byte
		if len(storage) == 0 {
			curmd5 = getNext(data, ctr1)
		} else {
			curmd5 = storage[0]
			storage = storage[1:]
		}
		symbol, found := isNRow(3, curmd5, 0, false)
		if found {
			fmt.Println("Found", ctr1)
			ctr2 = ctr1 + 1
			for ctr2-ctr1 <= 1000 {
				cur2md5 := getNext(data, ctr2)
				if len(storage) < ctr2-ctr1 {
					storage = append(storage, cur2md5)
				}
				_, found2 := isNRow(5, cur2md5, symbol, true)
				if found2 {
					fmt.Println("Found2", ctr2)
					nkeys++
					break
				}
				ctr2++
			}
		}
		ctr1++
	}
	fmt.Printf("%d\n", ctr1-1)
}
