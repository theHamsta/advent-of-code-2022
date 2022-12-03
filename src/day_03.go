package advent

import (
	"fmt"
	"os"
	"strings"

	set "github.com/golang-collections/collections/set"
)

func Day03() {
	input, err := os.ReadFile("input/day03.txt")
	if err != nil {
		fmt.Fprintln(os.Stderr, "Error:", err)
		os.Exit(1)
	}

	lines := strings.Split(string(input), "\n")

	// Solved in Rust first
	sum := 0
	for _, line := range lines {
		set_a := make(map[rune]bool)
		for i, c := range line {
			if i <= len(line)/2 {
				set_a[c] = true
			} else {
				if set_a[c] {
					if (c - 'a') > 0 {
						sum += int(c-'a') + 1
					} else {
						sum += int(c-'A') + 1 + 26
					}

					break
				}
			}
		}
	}
	fmt.Println("Part 1: ", sum)

	sets := make([](*set.Set), 3)

	sum = 0
	for i, line := range lines {
		sets[i%3] = set.New()
		line_set := sets[i%3]
		for _, c := range line {
			line_set.Insert(c)
		}
		if i%3 == 2 {
			intersection := sets[0].Intersection(sets[1]).Intersection(sets[2])
			intersection.Do(func(c interface{}) {
				if (c.(rune) - 'a') > 0 {
					sum += int(c.(rune)-'a') + 1
				} else {
					sum += int(c.(rune)-'A') + 1 + 26
				}
			})
		}
	}
	fmt.Println("Part 2: ", sum)
}
