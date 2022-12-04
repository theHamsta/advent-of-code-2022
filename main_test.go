//
// main_test.go
// Copyright (C) 2022 Stephan Seitz <stephan.seitz@fau.de>
//
// Distributed under terms of the GPLv3 license.
//

package main

import (
	"testing"
	advent "theHamsta/advent-of-code-2022/src"
)

func benchmarkDay(day int, b *testing.B) {
	for n := 0; n < b.N; n++ {
		advent.RunDay(day)
	}
}

func BenchmarkDay01(b *testing.B) { benchmarkDay(1, b) }
func BenchmarkDay02(b *testing.B) { benchmarkDay(2, b) }
func BenchmarkDay03(b *testing.B) { benchmarkDay(3, b) }
func BenchmarkDay04(b *testing.B) { benchmarkDay(4, b) }
