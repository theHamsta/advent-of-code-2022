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

	if day == 0 {
		flag.PrintDefaults()
		os.Exit(1)
	} else {
		fmt.Println("Day:", day)
	}

	err := advent.RunDay(day)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
