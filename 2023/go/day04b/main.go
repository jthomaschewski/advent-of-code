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

	pile := parsePile(lines)
	numCopies := pile.countCopies()

	fmt.Println(numCopies)
}

type Pile struct {
	cards []Card
}

func (p *Pile) countCopies() int {
	copies := make([]int, len(p.cards))
	for i := range copies {
		copies[i] = 1
	}
	for i, card := range p.cards {
		j := 1
		for _, owned := range card.owned {
			if slices.Contains(card.winning, owned) {
				copies[i+j] = copies[i+j] + copies[i]
				j++
			}
		}
	}

	sum := 0
	for _, cpy := range copies {
		sum += cpy
	}
	return sum
}

func parsePile(lines []string) *Pile {
	pile := Pile{
		cards: make([]Card, len(lines)),
	}
	for i, line := range lines {
		pile.cards[i] = *parseCard(line)
	}
	return &pile
}

type Card struct {
	winning []int
	owned   []int
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
