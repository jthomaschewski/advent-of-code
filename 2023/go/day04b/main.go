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
	pile.process()
	fmt.Println(pile.numCards)
}

type Pile struct {
	cards    []Card
	numCards int
}

func (p *Pile) process() {
	for i, card := range p.cards {
		p.numCards += p.countCopies(i, &card)
	}
}

func (p *Pile) countCopies(cardIndex int, card *Card) int {
	wins := card.wins()
	if wins == 0 {
		return 0
	}

	wonCards := p.cards[cardIndex+1 : cardIndex+wins+1]
	sum := 0

	for i, wonCard := range wonCards {
		sum += 1 + p.countCopies(cardIndex+i+1, &wonCard)
	}
	return sum
}

func parsePile(lines []string) *Pile {
	pile := Pile{
		cards:    make([]Card, len(lines)),
		numCards: len(lines),
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

func (c *Card) wins() int {
	winCnt := 0
	for _, owned := range c.owned {
		if slices.Contains(c.winning, owned) {
			winCnt++
		}
	}
	return winCnt
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
