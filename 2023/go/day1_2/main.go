package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("../inputs/day1.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	sum := 0

	for scanner.Scan() {
		line := scanner.Bytes()

		var first_digit int
		var last_digit int

		for i := 0; i < len(line); i++ {
			digit := nextDigit(line[i:])
			if digit != 0 {
				if first_digit == 0 {
					first_digit = digit
				}
				last_digit = digit
			}
		}
		sum += first_digit*10 + last_digit
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(sum)
}

var validDigits = [...]string{
	"1",
	"2",
	"3",
	"4",
	"5",
	"6",
	"7",
	"8",
	"9",
	"one",
	"two",
	"three",
	"four",
	"five",
	"six",
	"seven",
	"eight",
	"nine",
}

func nextDigit(slice []byte) int {
	s := string(slice)
	for idx, digit := range validDigits {
		if strings.HasPrefix(s, digit) {
			n := idx%9 + 1
			return n
		}
	}
	return 0
}
