package main

import (
	"flag"
	"fmt"
	"os"
	advent "theHamsta/advent-of-code-2022/src"
)

func main() {
	var day int

	flag.IntVar(&day, "day", 0, "Which day to solve")
	flag.Parse()
	if day != 0 {
		fmt.Println("Day:", day)
	}

	switch day {
	case 0:
		fmt.Println("Please select a day via \"--day <number>\"")
		os.Exit(1)
	case 1:
		advent.Day01()
	case 2:
		advent.Day02()
	default:
		fmt.Println("Invalid day:", day)
		os.Exit(1)

	}
}
