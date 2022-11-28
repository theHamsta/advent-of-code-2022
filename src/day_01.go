package advent

import (
	"fmt"
	"os"
)

func Day01() {
	input, err := os.ReadFile("input/day01.txt")
	if err != nil {
		fmt.Fprintln(os.Stderr, "Error:", err)
		os.Exit(1)
	}
	fmt.Print(input)

}
