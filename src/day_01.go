package advent

import (
	"fmt"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

type Point struct {
	x, y   int
	dx, dy int
}

func toInt(s string) int {
	i, _ := strconv.Atoi(s)
	return i
}

func Day01() {
	input, err := os.ReadFile("input/day01.txt")
	if err != nil {
		fmt.Fprintln(os.Stderr, "Error:", err)
		os.Exit(1)
	}

	elves := strings.Split(string(input), "\n\n")
	regex := regexp.MustCompile(`(\d+)`)
	argmax := -1
	max := 0
	calories := make([]int, len(elves))
	for i, elf := range elves {
		parsed := regex.FindAllStringSubmatch(elf, -1)
		sum := 0
		for _, c := range parsed {
			sum += toInt(c[1])
		}
		calories[i] = sum
		if calories[i] > max {
			max = calories[i]
			argmax = i + 1
		}
	}

	fmt.Println("part 1:", max, "(argmax: ", argmax, ")")

	sort.Ints(calories)
	fmt.Println("part 2:", calories[len(calories)-1]+calories[len(calories)-2]+calories[len(calories)-3])
}
