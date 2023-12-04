package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"slices"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("../../inputs/day04.txt")
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(strings.TrimSpace(string(file)), "\n")
	sum := 0
	for _, line := range lines {
		card := parseCard(line)
		sum += card.points()
	}
	fmt.Println(sum)
}

type Card struct {
	winning []int
	owned   []int
}

func (c *Card) points() int {
	points := 0
	for _, owned := range c.owned {
		if slices.Contains(c.winning, owned) {
			if points == 0 {
				points = 1
			} else {
				points *= 2
			}
		}
	}
	return points
}

func parseCard(line string) *Card {
	re := regexp.MustCompile("Card +\\d+: ([ \\d]+) | ([ \\d]+)")
	matches := re.FindAllStringSubmatch(line, -1)

	winningStr := matches[0][1]
	ownedStr := matches[1][0]

	winning := strings.Fields(winningStr)
	owned := strings.Fields(ownedStr)

	card := Card{
		winning: make([]int, len(winning)),
		owned:   make([]int, len(owned)),
	}

	for i, numStr := range winning {
		n, err := strconv.Atoi(numStr)
		if err != nil {
			log.Fatal(err)
		}
		card.winning[i] = n
	}

	for i, numStr := range owned {
		n, err := strconv.Atoi(numStr)
		if err != nil {
			log.Fatal(err)
		}
		card.owned[i] = n
	}

	return &card
}
