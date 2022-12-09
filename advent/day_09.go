package advent

import (
	"fmt"
	"os"
	"regexp"
)

//
// day_09.go
// Copyright (C) 2022 Stephan Seitz <stephan.seitz@fau.de>
//
// Distributed under terms of the GPLv3 license.
//

type Movement struct {
	dir  rune
	step int
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func sign(x int) int {
	if x < 0 {
		return -1
	}
	if x > 0 {
		return 1
	}
	return 0
}

func move(movement Movement, head, tail *[2]int, activeHead bool) {
	if activeHead {
		switch movement.dir {
		case 'U':
			(*head)[1] += 1
		case 'D':
			(*head)[1] -= 1
		case 'R':
			(*head)[0] += 1
		case 'L':
			(*head)[0] -= 1
		}
	}

	dx, dy := (*head)[0]-(*tail)[0], (*head)[1]-(*tail)[1]
	if !(abs(dx) > 1 || abs(dy) > 1) {
		return
	}

	(*tail)[0] += sign(dx)
	(*tail)[1] += sign(dy)
}

func Day09() {
	input, err := os.ReadFile("input/day09.txt")
	if err != nil {
		panic(err)
	}

	regex := regexp.MustCompile(`(\w)\s(\d+)`)

	movements := make([]Movement, 0)

	visited := make(map[[2]int]bool)

	for _, c := range regex.FindAllStringSubmatch(string(input), -1) {
		movements = append(movements, Movement{dir: rune(c[1][0]), step: toInt(c[2])})

	}
	head := [2]int{0, 0}
	tail := [2]int{0, 0}

	for _, movement := range movements {
		for i := 0; i < movement.step; i++ {
			move(movement, &head, &tail, true)
			visited[tail] = true
		}

	}
	fmt.Println("Part 1:", len(visited))

	rope := make([][2]int, 10)
	visited = make(map[[2]int]bool)

	for _, movement := range movements {
		for i := 0; i < movement.step; i++ {
			for r := 1; r < len(rope); r++ {
				move(movement, &rope[r-1], &rope[r], r == 1)

				if r == len(rope)-1 {
					visited[rope[r]] = true
				}
			}
		}
	}
	fmt.Println("Part 2:", len(visited))
}
