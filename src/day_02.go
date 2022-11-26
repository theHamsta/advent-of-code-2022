//
// 1.go
// Copyright (C) 2022 Stephan Seitz <stephan.seitz@fau.de>
//
// Distributed under terms of the GPLv3 license.
//

package advent

import (
	"fmt"
	"log"
	"os"
)

func Day02() {
	content, err := os.ReadFile("file.txt")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("content: %", content)
}
