package main

import "strings"

type Game struct {
	rounds []Set
}

func parseGame(gameLine string) *Game {
	sets := strings.Split(gameLine, ": ")
	setsSplit := strings.Split(sets[1], ";")

	rounds := make([]Set, len(setsSplit))
	game := Game{rounds: rounds}

	for idx, setStr := range setsSplit {
		set := parseSet(setStr)
		game.rounds[idx] = *set
	}

	return &game
}

func (g *Game) isValid(validBag *Set) bool {
	for _, round := range g.rounds {
		if !round.isValid(validBag) {
			return false
		}
	}
	return true
}
