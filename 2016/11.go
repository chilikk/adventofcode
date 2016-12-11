package main

import (
	"fmt"
)

type State struct {
	// rightmost N bits denote components
	// leftmost N bits are generators
	f [4]uint16
	e uint8
}

type Step struct {
	n int
	s State
}

func validate(s State) bool {
	for i := 0; i < 4; i++ {
		gens := s.f[i] >> 8
		comps := s.f[i] & 0xff
		if (comps^gens)&comps != 0 && gens != 0 {
			return false
		}
	}
	return true
}

func main() {
	//init := State{f: [4]uint16{0x303, 0x1c18, 0x4, 0}, e: 0}
	//final := State{f: [4]uint16{0, 0, 0, 0x1f1f}, e: 3}
	init := State{f: [4]uint16{0x6363, 0x1c18, 0x4, 0}, e: 0}
	final := State{f: [4]uint16{0, 0, 0, 0x7f7f}, e: 3}
	queue := make([]Step, 100, 10000)
	visited := make(map[State]bool)
	queue = append(queue, Step{n: 0, s: init})
	var finalstep int
Out:
	for len(queue) > 0 {
		step := queue[0]
		queue = queue[1:]
		cur := step.s
		for de := -1; de <= 1; de += 2 {
			if int(cur.e)+de < 0 || int(cur.e)+de > 3 {
				continue
			}
			bits := make([]uint16, 0, 257)
			for bit1 := uint8(0); bit1 < 16; bit1++ {
				bit1v := cur.f[cur.e] & (1 << bit1)
				if bit1v == 0 {
					continue
				}
				bits = append(bits, bit1v)
				for bit2 := uint8(0); bit2 < 16; bit2++ {
					bit2v := cur.f[cur.e] & (1 << bit2)
					if bit2v == 0 || bit2v == bit1v {
						continue
					}
					bits = append(bits, bit1v^bit2v)
				}
			}
			for _, bitv := range bits {
				newstate := cur
				newstate.f[cur.e] ^= bitv
				newstate.e = uint8(int(cur.e) + de)
				newstate.f[newstate.e] ^= bitv
				if validate(newstate) && !visited[newstate] {
					if newstate == final {
						finalstep = step.n + 1
						break Out
					}
					newstep := Step{n: step.n + 1, s: newstate}
					//fmt.Printf("step %d e %d f1 %x f2 %x f3 %x f4 %x -> ",
					//    step.n,
					//    step.s.e,
					//    step.s.f[0],
					//    step.s.f[1],
					//    step.s.f[2],
					//    step.s.f[3])
					//fmt.Printf("step %d e %d f1 %x f2 %x f3 %x f4 %x\n",
					//    newstep.n,
					//    newstate.e,
					//    newstate.f[0],
					//    newstate.f[1],
					//    newstate.f[2],
					//    newstate.f[3])
					queue = append(queue, newstep)
					visited[newstate] = true
				}
			}
		}
	}
	fmt.Println(finalstep)
}
