package advent

import (
	"fmt"
	"os"
	"regexp"
	"strings"

	"gopkg.in/dnaeon/go-deque.v1"
)

//
// day_05.go
// Copyright (C) 2022 Stephan Seitz <stephan.seitz@fau.de>
//
// Distributed under terms of the GPLv3 license.
//

func stacksToWord(stacks []*deque.Deque[byte]) string {
	word := ""
	for _, stack := range stacks {
		item, err := stack.PeekBack()
		if err != nil {
			panic(err)
		}
		word += string(item)
	}
	return word
}

func Day05() {
	input, err := os.ReadFile("input/day05.txt")
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	stacks := make([]*deque.Deque[byte], 9)
	stacksClone := make([]*deque.Deque[byte], 9)
	for i := range stacks {
		stacks[i] = deque.New[byte]()
		stacksClone[i] = deque.New[byte]()
	}
	lines := strings.Split(string(input), "\n")

	for _, line := range lines {
		i := 0
		for i*4+1 < len(line) {
			if line[i*4+1] != ' ' {
				stacks[i].PushFront(line[i*4+1])
				stacksClone[i].PushFront(line[i*4+1])
			}
			i++
		}
		if strings.HasPrefix(line, " 1") {
			break
		}
	}

	instructions := make([][3]int, 0)
	regex := regexp.MustCompile(`move (\d+) from (\d+) to (\d+)`)
	for _, match := range regex.FindAllStringSubmatch(string(input), -1) {
		instructions = append(instructions, [3]int{toInt(match[1]), toInt(match[2]), toInt(match[3])})
	}

	// Part 1
	for _, instruction := range instructions {
		for i := 0; i < instruction[0]; i++ {
			item, err := stacks[instruction[1]-1].PopBack()
			if err != nil {
				panic(err)
			}
			stacks[instruction[2]-1].PushBack(item)
		}
	}

	fmt.Println("Part 1:", stacksToWord(stacks))

	// Part 2
	stacks = stacksClone
	for _, instruction := range instructions {
		tmp := deque.New[byte]()
		for i := 0; i < instruction[0]; i++ {
			item, err := stacks[instruction[1]-1].PopBack()
			if err != nil {
				panic(err)
			}
			tmp.PushFront(item)
		}
		for i := 0; i < instruction[0]; i++ {
			item, err := tmp.PopFront()
			if err != nil {
				panic(err)
			}
			stacks[instruction[2]-1].PushBack(item)
		}
	}
	fmt.Println("Part 2:", stacksToWord(stacks))
}
