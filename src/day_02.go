//
// 1.go
// Copyright (C) 2022 Stephan Seitz <stephan.seitz@fau.de>
//
// Distributed under terms of the GPLv3 license.
//

package advent

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func decode(char byte, rps [3]byte) byte {
	if char == rps[0] {
		return 'R'
	}
	if char == rps[1] {
		return 'P'
	}
	if char == rps[2] {
		return 'S'
	}
	panic("Invalid argument")
}

func aWins(a, b byte, rpsA, rpsB [3]byte) bool {
	return (decode(a, rpsA) == 'R' && decode(b, rpsB) == 'S') || (decode(a, rpsA) == 'P' && decode(b, rpsB) == 'R') || (decode(a, rpsA) == 'S' && decode(b, rpsB) == 'P')

}

func bWins(a, b byte, rpsA, rpsB [3]byte) bool {

	return aWins(b, a, rpsB, rpsA)
}

func Day02() {
	var ABC = [3]byte{'A', 'B', 'C'}
	var XYZ = [3]byte{'X', 'Y', 'Z'}
	file, err := os.Open("input/day02.txt")

	if err != nil {
		log.Fatal(err)
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	strategies := make([][]byte, 0)

	for scanner.Scan() {
		text := scanner.Bytes()
		parsed := []byte{text[0], text[2]}
		strategies = append(strategies, parsed)
	}

	// Oh no, is this ugly!
	part1 := 0
	for _, strategy := range strategies {
		a := strategy[0]
		b := strategy[1]
		score := int(b - 'X' + 1)
		if bWins(a, b, ABC, XYZ) {
			score += 6
		} else if !aWins(a, b, ABC, XYZ) {
			score += 3
		}
		part1 += score

	}
	fmt.Println("Part 1: ", part1)

	part2 := 0
	for _, strategy := range strategies {
		a := strategy[0]
		whatToDo := strategy[1]
		var b byte
		switch whatToDo {
		case 'X':
			switch decode(a, ABC) {
			case 'R':
				b = 'C'
			case 'P':
				b = 'A'
			case 'S':
				b = 'B'
			default:
				panic("noooo")
			}
		case 'Y':
			b = a
		case 'Z':
			switch decode(a, ABC) {
			case 'R':
				b = 'B'
			case 'P':
				b = 'C'
			case 'S':
				b = 'A'
			default:
				panic("noooo")
			}

		default:
			panic("invalid instruction")
		}
		score := int(b - 'A' + 1)
		if bWins(a, b, ABC, ABC) {
			score += 6
		} else if !aWins(a, b, ABC, ABC) {
			score += 3
		}
		part2 += score
	}
	fmt.Println("Part 2: ", part2)
}
