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

func (s *Set) isValid(bag *Set) bool {
	if s.red > bag.red {
		return false
	} else if s.blue > bag.blue {
		return false
	} else if s.green > bag.green {
		return false
	}
	return true
}
