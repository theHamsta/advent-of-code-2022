//
// select-day.go
// Copyright (C) 2022 Stephan Seitz <stephan.seitz@fau.de>
//
// Distributed under terms of the GPLv3 license.
//

package advent

import (
	"fmt"
)

func RunDay(day int) error {
	switch day {
	case 1:
		Day01()
	case 2:
		Day02()
	case 3:
		Day03()
	case 4:
		Day04()
	case 5:
		Day05()
	default:
		return fmt.Errorf("Day %d not implemented", day)
	}
	return nil
}
