package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	solution := solve("../../inputs/day05.txt")
	fmt.Println(solution)
}

func solve(filename string) int {
	file, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	almanac := parseAlmanac(string(file))
	lowest := -1

	for _, seed := range almanac.seeds {
		location := almanac.locationForSeed(seed)
		if lowest == -1 || location < lowest {
			lowest = location
		}
	}

	return lowest
}

type Almanac struct {
	seeds []int
	maps  []Map
}

func parseAlmanac(input string) *Almanac {
	almanac := Almanac{}

	blocks := strings.Split(strings.TrimSpace(input), "\n\n")
	seeds := strings.Fields(blocks[0])[1:]

	for i := 0; i < len(seeds); i += 2 {
		start, err := strconv.Atoi(seeds[i])
		if err != nil {
			log.Fatal(err)
		}
		length, err := strconv.Atoi(seeds[i+1])
		if err != nil {
			log.Fatal(err)
		}

		for j := 0; j < length; j++ {
			almanac.seeds = append(almanac.seeds, start+j)
		}
	}

	almanac.maps = make([]Map, len(blocks)-1)
	for i, block := range blocks[1:] {
		lines := strings.Split(strings.TrimSpace(block), "\n")
		name := strings.Fields(lines[0])[0]

		ranges := make([]Range, len(lines)-1)
		for j, line := range lines[1:] {
			fields := strings.Fields(line)
			dest, err := strconv.Atoi(fields[0])
			if err != nil {
				log.Fatal(err)
			}
			src, err := strconv.Atoi(fields[1])
			if err != nil {
				log.Fatal(err)
			}
			length, err := strconv.Atoi(fields[2])
			if err != nil {
				log.Fatal(err)
			}
			ranges[j] = Range{
				src:    src,
				dest:   dest,
				length: length,
			}
		}

		almanac.maps[i] = Map{
			name:   name,
			ranges: ranges,
		}
	}

	return &almanac
}

func (a *Almanac) locationForSeed(seed int) int {
	curLocation := seed
	for _, m := range a.maps {
		curLocation = m.getDestinationNumber(curLocation)
	}
	return curLocation
}

type Map struct {
	name   string
	ranges []Range
}

func (m *Map) getDestinationNumber(sourceNumber int) int {
	for _, r := range m.ranges {
		dest := r.getDestinationNumber(sourceNumber)
		if dest != -1 {
			return dest
		}
	}
	return sourceNumber
}

type Range struct {
	src    int
	dest   int
	length int
}

func (r *Range) getDestinationNumber(sourceNumber int) int {
	if sourceNumber >= r.src && sourceNumber < r.src+r.length {
		diff := r.src - r.dest
		return sourceNumber - diff
	}
	return -1
}
