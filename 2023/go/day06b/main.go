package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	solution := solve("../../inputs/day06.txt")
	fmt.Println(solution)
}

func solve(filename string) int {
	file, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	race := parseRace(string(file))
	r := race.findWinningTimes()

	return r
}

func parseRace(input string) *Race {
	values := strings.Split(strings.TrimSpace(input), "\n")

	time := strings.Join(strings.Fields(values[0])[1:], "")
	distance := strings.Join(strings.Fields(values[1])[1:], "")

	race := Race{}

	timeInt, err := strconv.Atoi(time)
	if err != nil {
		log.Fatal(err)
	}
	race.time = timeInt

	distanceInt, err := strconv.Atoi(distance)
	if err != nil {
		log.Fatal(err)
	}
	race.distance = distanceInt

	return &race
}

type Race struct {
	time     int
	distance int
}

func (r *Race) findWinningTimes() int {
	wins := 0
	for holdTime := 1; holdTime < r.time; holdTime++ {
		distance := r.travelDistance(holdTime)
		if distance > r.distance {
			wins += 1
		} else if wins > 0 {
			break
		}
	}
	return wins
}

func (r *Race) travelDistance(holdTime int) int {
	return holdTime * (r.time - holdTime)
}
