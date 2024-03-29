package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	solution := solve("../../inputs/day09.txt")
	fmt.Println(solution)
}

func solve(filename string) int {
	file, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	history := parseHistory(string(file))
	sum := history.predictionSum()

	return sum
}

type History []int

type Oasis struct {
	history []History
}

func parseHistory(input string) *Oasis {
	lines := strings.Split(strings.TrimSpace(input), "\n")

	history := make([]History, len(lines))

	for i, line := range lines {
		fields := strings.Fields(line)
		historyItems := make(History, len(fields))

		for i, field := range fields {
			itemInt, err := strconv.Atoi(field)
			if err != nil {
				log.Fatal(err)
			}
			historyItems[i] = itemInt
		}

		history[i] = historyItems
	}

	return &Oasis{history: history}
}

func (o *Oasis) predictionSum() int {
	sum := 0
	for _, history := range o.history {
		sum += history.nextValue()
	}

	return sum
}

func (h *History) nextValue() int {
	seq := []History{*h}
	isZero := false
	i := 0
	for !isZero {
		nextSeq, nextIsZero := seq[i].nextDiffSequence()
		seq = append(seq, nextSeq)
		isZero = nextIsZero
		i++
	}

	predictionBelow := seq[len(seq)-2][0]

	for i := len(seq) - 3; i >= 0; i-- {
		predictionBelow = seq[i][0] + predictionBelow
	}
	return predictionBelow
}

func (h *History) nextDiffSequence() (History, bool) {
	diff := make(History, len(*h)-1)
	isZero := true
	for i := range diff {
		val := (*h)[i] - (*h)[i+1]
		diff[i] = val
		if val != 0 {
			isZero = false
		}
	}
	return diff, isZero
}
