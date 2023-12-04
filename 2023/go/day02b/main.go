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
	sumPower := 0

	for scanner.Scan() {
		gameLine := scanner.Text()

		game := parseGame(gameLine)
		minBag := game.minBag()
		sumPower += minBag.power()
	}
	fmt.Println(sumPower)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
