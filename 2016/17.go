package main

import (
	"crypto/md5"
	"fmt"
)

type Coord struct {
	x, y int
}

type State struct {
	path string
	pos  Coord
}

func openDoors(s State) (open []State) {
	open = make([]State, 0, 4)
	md5 := md5.Sum([]byte("qljzarfv" + s.path))
	if s.pos.y > 0 && md5[0]>>4 > 10 {
		s1 := s
		s1.pos.y -= 1
		s1.path = s.path + "U"
		open = append(open, s1)
	}
	if s.pos.y < 3 && md5[0]&0xf > 10 {
		s1 := s
		s1.pos.y += 1
		s1.path = s.path + "D"
		open = append(open, s1)
	}
	if s.pos.x > 0 && md5[1]>>4 > 10 {
		s1 := s
		s1.pos.x -= 1
		s1.path = s.path + "L"
		open = append(open, s1)
	}
	if s.pos.x < 3 && md5[1]&0xf > 10 {
		s1 := s
		s1.pos.x += 1
		s1.path = s.path + "R"
		open = append(open, s1)
	}
	return
}

func final(s State) bool {
	return s.pos.x == 3 && s.pos.y == 3
}

func main() {
	queue := make([]State, 0, 10000)
	queue = append(queue, State{path: "", pos: Coord{x: 0, y: 0}})
	for len(queue) > 0 {
		state := queue[0]
		queue = queue[1:]
		if final(state) {
			fmt.Println(queue[0])
			continue
		}
		queue = append(queue, openDoors(state)...)
	}
}
