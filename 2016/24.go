package main

import (
	"bufio"
	"fmt"
	"os"
)

type State struct {
	x, y         int
	collected    []byte
	xinit, yinit int
}

type Step struct {
	n int
	s State
}

func toString(s State) string {
	return string(s.x) + "," + string(s.y) + "," + string(s.collected) + "$"
}

func final(s State) bool {
	return len(s.collected) == 7 && s.x == s.xinit && s.y == s.yinit
}

func readMap() (mapp [][]byte, init State) {
	mapp = make([][]byte, 0, 100)
	scanner := bufio.NewScanner(os.Stdin)
	for i := 0; scanner.Scan(); i++ {
		line := scanner.Text()
		mapp = append(mapp, make([]byte, len(line), len(line)))
		for j, s := range line {
			mapp[i][j] = byte(s) - '.'
			if s == '0' {
				mapp[i][j] = 0
				init = State{x: j, y: i, collected: make([]byte, 0, 7), xinit: j, yinit: i}
			}
		}
	}
	return
}

func main() {
	mapp, init := readMap()
	queue := make([]Step, 0, 10000)
	visited := make(map[string]bool)
	queue = append(queue, Step{n: 0, s: init})
	var finalstep int
Out:
	for len(queue) > 0 {
		step := queue[0]
		queue = queue[1:]
		cur := step.s
		nextstates := make([]State, 4, 4)
		nextstates[0] = cur
		nextstates[0].x += 1
		nextstates[1] = cur
		nextstates[1].x -= 1
		nextstates[2] = cur
		nextstates[2].y += 1
		nextstates[3] = cur
		nextstates[3].y -= 1
		fmt.Println(step)
		for _, newstate := range nextstates {
			newstate.collected = make([]byte, len(cur.collected), 7)
			copy(newstate.collected, cur.collected)
			val := mapp[newstate.y][newstate.x]
			if val != 245 && !visited[toString(newstate)] {
				if val > 0 {
					found := false
					for _, v2 := range newstate.collected {
						if val == v2 {
							found = true
							break
						}
					}
					if !found {
						newstate.collected = append(newstate.collected, val)
					}
				}
				if final(newstate) {
					finalstep = step.n + 1
					break Out
				}
				newstep := Step{n: step.n + 1, s: newstate}
				queue = append(queue, newstep)
				visited[toString(newstate)] = true
			}
		}
	}
	fmt.Println(finalstep)
}
