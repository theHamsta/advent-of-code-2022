//
// day_04.go
// Copyright (C) 2022 Stephan Seitz <stephan.seitz@fau.de>
//
// Distributed under terms of the GPLv3 license.
//

package advent

import (
	"fmt"
	"os"
	"regexp"
)

func contains(lo, hi, item int) bool {
	return item >= lo && item <= hi
}

func Day04() {
	input, err := os.ReadFile("input/day04.txt")
	if err != nil {
		panic(err)
	}
	regex := regexp.MustCompile(`(\d+)-(\d+),(\d+)-(\d+)`)

	matches := regex.FindAllStringSubmatch(string(input), -1)
	part1 := 0
	for _, match := range matches {
		a := toInt(match[1])
		b := toInt(match[2])
		c := toInt(match[3])
		d := toInt(match[4])

		if (contains(a, b, c) && contains(a, b, d)) || (contains(c, d, a) && contains(c, d, b)) {
			part1++
		}
	}
	fmt.Println("Part 1:", part1)
	part2 := 0
	for _, match := range matches {
		a := toInt(match[1])
		b := toInt(match[2])
		c := toInt(match[3])
		d := toInt(match[4])

	outer:
		for i := a; i <= b; i++ {
			for j := c; j <= d; j++ {
				if i == j {
					part2++
					break outer
				}
			}
		}
	}
	fmt.Println("Part 2:", part2)
}
