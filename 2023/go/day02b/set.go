package main

import (
	"log"
	"strconv"
	"strings"
)

type Set struct {
	red   int
	green int
	blue  int
}

func parseSet(setStr string) *Set {
	splits := strings.Split(setStr, ",")

	red := 0
	green := 0
	blue := 0

	for _, split := range splits {
		splitDraw := strings.Split(strings.TrimSpace(split), " ")
		number, err := strconv.Atoi(splitDraw[0])
		if err != nil {
			log.Fatalln("Unable to parse set", split)
		}
		color := splitDraw[1]

		if color == "red" {
			red = number
		} else if color == "green" {
			green = number
		} else if color == "blue" {
			blue = number
		}
	}

	return &Set{red: red, green: green, blue: blue}
}

func (s *Set) power() int {
	return s.red * s.green * s.blue
}

func minBag(sets []Set) *Set {
	minBag := Set{}

	log.Println(sets)
	for _, set := range sets {
		minBag.red = max(set.red, minBag.red)
		minBag.green = max(set.green, minBag.green)
		minBag.blue = max(set.blue, minBag.blue)
	}
	return &minBag
}
