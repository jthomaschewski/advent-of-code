package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("../../inputs/day01.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	sum := 0

	for scanner.Scan() {
		line := scanner.Text()

		var first_digit byte
		var last_digit byte

		for i := 0; i < len(line); i++ {
			char := line[i]
			if isDigit(char) {
				if first_digit == 0 {
					first_digit = char
				}
				last_digit = char
			}
		}
		line_sum, err := strconv.Atoi(fmt.Sprintf("%c%c", first_digit, last_digit))
		if err != nil {
			log.Fatal(err)
		}
		sum += line_sum
	}

	fmt.Println(sum)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func isDigit(char byte) bool {
	return char >= '0' && char <= '9'
}
