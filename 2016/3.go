package main

import (
    "fmt"
    "bufio"
    "os"
    "strings"
    "strconv"
    "sort"
)

func main() {
    var t int
    var line [3]string
    var words [3][]string
    var intsa [3][]int
    for i:=0; i<3; i++ {
        intsa[i] = make([]int, 3, 3)
    }
    scanner := bufio.NewScanner(os.Stdin)
    for scanner.Scan() {
        line[0] = scanner.Text()
        scanner.Scan()
        line[1] = scanner.Text()
        scanner.Scan()
        line[2] = scanner.Text()
        for i, l := range line {
            words[i] = strings.Fields(l)
            for j, w := range words[i] {
                intsa[j][i], _ = strconv.Atoi(w)
            }
        }
        for _, ints := range intsa {
            fmt.Println(ints)
            sort.Ints(ints)
            if ints[0]+ints[1]>ints[2] {
                t++
            }
        }
    }
    fmt.Println("Number of triangles", t)
}
