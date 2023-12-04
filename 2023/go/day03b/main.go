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
	file, err := os.ReadFile("../../inputs/day03.txt")
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(file), "\n")

	schematic := parseSchematic(lines)
	_, ratioSum := schematic.gears()

	fmt.Println(ratioSum)
}

type Schematic struct {
	partCandidates [][]Position
	symbols        [][]Position
	lineCount      int
}

type Position struct {
	lineRange []int
	value     string
}

func (p *Position) isAdjacent(otherPos *Position) bool {
	return p.lineRange[0] > otherPos.lineRange[0]-2 && p.lineRange[1] < otherPos.lineRange[1]+2
}

func (s *Schematic) gears() ([]int, int) {
	var gearRatios []int
	var ratioSum int

	for i, symbolLine := range s.symbols {
		for _, symbol := range symbolLine {
			adjacentParts := make([]int, 0)

			for _, candidateLine := range s.partCandidates[max(0, i-1):min(s.lineCount, i+2)] {
				for _, candidate := range candidateLine {
					if symbol.isAdjacent(&candidate) {
						intval, err := strconv.Atoi(candidate.value)
						if err != nil {
							log.Fatal(err)
						}
						adjacentParts = append(adjacentParts, intval)
					}
				}
			}
			if len(adjacentParts) == 2 {
				ratio := adjacentParts[0] * adjacentParts[1]
				gearRatios = append(gearRatios, ratio)
				ratioSum += ratio
			}
		}
	}

	return gearRatios, ratioSum
}

func parseSchematic(lines []string) *Schematic {
	schematic := Schematic{
		lineCount:      len(lines),
		partCandidates: make([][]Position, len(lines)),
		symbols:        make([][]Position, len(lines)),
	}

	reNum := regexp.MustCompile("(\\d+)")
	reSym := regexp.MustCompile("([^\\d.]+)")

	for i, line := range lines {
		matches := reNum.FindAllStringIndex(line, -1)
		matchesSym := reSym.FindAllStringIndex(line, -1)

		for _, candidate := range matches {
			part := Position{
				lineRange: candidate,
				value:     line[candidate[0]:candidate[1]],
			}
			if len(schematic.partCandidates) <= i {
				schematic.partCandidates[i] = make([]Position, len(matches))
			}
			schematic.partCandidates[i] = append(schematic.partCandidates[i], part)
		}
		for _, sym := range matchesSym {
			symbol := Position{
				lineRange: sym,
				value:     line[sym[0]:sym[1]],
			}
			schematic.symbols[i] = append(schematic.symbols[i], symbol)
		}
	}
	return &schematic
}
