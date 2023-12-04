package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	// file, err := os.ReadFile("../../inputs/day03_example_a.txt")
	file, err := os.ReadFile("../../inputs/day03.txt")
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(file), "\n")

	reNum := regexp.MustCompile("(\\d+)")
	reSym := regexp.MustCompile("([^\\d.]+)")

	var partCandiates [][][]int
	var symbols [][][]int

	for _, line := range lines {
		matches := reNum.FindAllStringIndex(line, -1)
		matchesSym := reSym.FindAllStringIndex(line, -1)

		partCandiates = append(partCandiates, matches)
		symbols = append(symbols, matchesSym)
	}

	var parts []int
	fmt.Println(symbols)

	partSum := 0

	for i, candidateLine := range partCandiates {
		for _, candidate := range candidateLine {

			leftIdx := candidate[0]
			rightIdx := candidate[1]

			for _, symbolline := range symbols[max(0, i-1):min(len(lines), i+2)] {
				if i == 0 {
					fmt.Println(symbolline, max(0, i-1), min(len(lines), i+1))
				}
				for _, asym := range symbolline {
					if asym[0] > leftIdx-2 && asym[1] < rightIdx+2 {
						// found adjacent symbol
						candidateNum, err := strconv.Atoi(lines[i][leftIdx:rightIdx])
						if err != nil {
							log.Fatalln("Unalbe to parse partnum", lines[i][leftIdx:rightIdx])
						}
						parts = append(parts, candidateNum)
						partSum += candidateNum
						continue
					}
				}
			}
		}
	}

	fmt.Println(parts, partSum)
}
