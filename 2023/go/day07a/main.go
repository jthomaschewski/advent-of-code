package main

import (
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

const (
	FIVE_OF_A_KIND  = 7
	FOUR_OF_A_KIND  = 6
	FULL_HOUSE      = 5
	THREE_OF_A_KIND = 4
	TWO_PAIR        = 3
	ONE_PAIR        = 2
	HIGH_CARD       = 1
)

var cardStrength = map[byte]int{
	'A': 14,
	'K': 13,
	'Q': 12,
	'J': 11,
	'T': 10,
	'9': 9,
	'8': 8,
	'7': 7,
	'6': 6,
	'5': 5,
	'4': 4,
	'3': 3,
	'2': 2,
}

func main() {
	solution := solve("../../inputs/day07_example_a.txt")
	fmt.Println(solution)
}

func solve(filename string) int {
	file, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	game := parseGame(string(file))
	result := game.result()

	return result
}

func parseGame(input string) *Game {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	hands := make([]Hand, len(lines))

	for i, line := range lines {

		fields := strings.Fields(line)
		cards := []byte(fields[0])
		bid, err := strconv.Atoi(fields[1])
		if err != nil {
			log.Fatal(err)
		}

		hands[i] = Hand{cards: cards, bid: bid}
	}

	return &Game{hands: hands}
}

type Game struct {
	hands []Hand
}

func (g *Game) result() int {
	slices.SortFunc(g.hands, func(a, b Hand) int {
		return a.compare(&b)
	})

	winnings := 0

	for i, hand := range g.hands {
		winnings += hand.bid * (i + 1)
	}
	return winnings
}

type Hand struct {
	cards []byte
	bid   int
}

func (h *Hand) typeValue() int {
	counts := map[byte]int{}
	for _, card := range h.cards {
		counts[card]++
	}
	numSameOfAKind := 0
	for _, count := range counts {
		if count > numSameOfAKind {
			numSameOfAKind = count
		}
	}

	if numSameOfAKind == 5 {
		return FIVE_OF_A_KIND
	}
	if numSameOfAKind == 4 {
		return FOUR_OF_A_KIND
	}
	if numSameOfAKind == 3 && len(counts) == 2 {
		return FULL_HOUSE
	}
	if numSameOfAKind == 3 && len(counts) == 3 {
		return THREE_OF_A_KIND
	}
	if numSameOfAKind == 2 && len(counts) == 3 {
		return TWO_PAIR
	}
	if numSameOfAKind == 2 {
		return ONE_PAIR
	}
	return HIGH_CARD
}

func (h *Hand) compare(other *Hand) int {
	// compare by type (first criterium)
	if h.typeValue() > other.typeValue() {
		return 1
	}
	if h.typeValue() < other.typeValue() {
		return -1
	}

	// compare by card strength (second criterium)
	for i := 0; i < len(h.cards); i++ {
		if cardStrength[h.cards[i]] > cardStrength[other.cards[i]] {
			return 1
		}
		if cardStrength[h.cards[i]] < cardStrength[other.cards[i]] {
			return -1
		}
	}

	return 0
}
