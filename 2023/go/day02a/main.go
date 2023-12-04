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

	idSum := 0
	gameNr := 1
	for scanner.Scan() {
		game_line := scanner.Text()

		bag := Set{red: 12, green: 13, blue: 14}

		sets := strings.Split(game_line, ": ")
		setsSplit := strings.Split(sets[1], ";")

		validGame := true

		for _, setStr := range setsSplit {
			set := parseSet(setStr)
			if !set.isValid(bag) {
				validGame = false
				break
			}
		}

		if validGame {
			idSum += gameNr
		}

		gameNr++
	}

	fmt.Println(idSum)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
