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

func (g *Game) minBag() *Set {
	minBag := Set{}

	for _, round := range g.rounds {
		minBag.red = max(round.red, minBag.red)
		minBag.green = max(round.green, minBag.green)
		minBag.blue = max(round.blue, minBag.blue)
	}
	return &minBag
}
