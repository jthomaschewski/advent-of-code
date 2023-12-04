package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("../../inputs/day02.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	sumPower := 0

	for scanner.Scan() {
		game_line := scanner.Text()

		sets := strings.Split(game_line, ": ")
		setsSplit := strings.Split(sets[1], ";")

		parsedSets := make([]Set, len(setsSplit))
		for idx, setStr := range setsSplit {
			parsedSets[idx] = *parseSet(setStr)
		}
		minBag := minBag(parsedSets)
		log.Println(minBag, minBag.power())
		sumPower += minBag.power()
	}
	fmt.Println(sumPower)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
