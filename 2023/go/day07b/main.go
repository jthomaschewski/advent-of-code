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
	'A': 13,
	'K': 12,
	'Q': 11,
	'T': 10,
	'9': 9,
	'8': 8,
	'7': 7,
	'6': 6,
	'5': 5,
	'4': 4,
	'3': 3,
	'2': 2,
	'J': 1,
}

func main() {
	solution := solve("../../inputs/day07.txt")
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
		// return 1
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

	maxEqual := 0
	maxEqualCard := byte(0)

	for card, count := range counts {
		countWithJoker := count + counts['J']
		if card == 'J' {
			countWithJoker = count
		}
		if countWithJoker > maxEqual {
			maxEqual = countWithJoker
			maxEqualCard = card
		}
	}
	// get count of different rest cards
	restEqual := 0
	for card := range counts {
		if card == 'J' || card == maxEqualCard {
			continue
		}
		restEqual++
	}

	if maxEqual == 5 {
		return FIVE_OF_A_KIND
	}
	if maxEqual == 4 {
		return FOUR_OF_A_KIND
	}
	if maxEqual == 3 && restEqual == 1 {
		return FULL_HOUSE
	}
	if maxEqual == 3 && restEqual == 2 {
		return THREE_OF_A_KIND
	}
	if maxEqual == 2 && restEqual == 2 {
		return TWO_PAIR
	}
	if maxEqual == 2 {
		return ONE_PAIR
	}
	return HIGH_CARD
}

func (h *Hand) compare(other *Hand) int {
	// compare by type
	if h.typeValue() > other.typeValue() {
		return 1
	}
	if h.typeValue() < other.typeValue() {
		return -1
	}

	// compare by highest card (left to right)
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
