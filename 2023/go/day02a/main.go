package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	file, err := os.Open("../../inputs/day02.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	validBag := Set{red: 12, green: 13, blue: 14}
	idSum := 0
	gameNr := 1

	for scanner.Scan() {
		gameLine := scanner.Text()
		game := parseGame(gameLine)

		if game.isValid(&validBag) {
			idSum += gameNr
		}

		gameNr++
	}

	fmt.Println(idSum)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
