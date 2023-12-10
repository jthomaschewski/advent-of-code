package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	solution := solve("../../inputs/day08.txt")
	fmt.Println(solution)
}

func solve(filename string) int {
	file, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	network := parseNetwork(string(file))
	steps := network.navigate()

	return steps
}

type Direction bool

const (
	Left  = false
	Right = true
)

func parseNetwork(input string) *Network {
	lines := strings.Split(strings.TrimSpace(input), "\n")

	instructions := make([]Direction, len(lines[0]))
	for i, line := range lines[0] {
		switch line {
		case 'R':
			instructions[i] = Right
		case 'L':
			instructions[i] = Left
		default:
			log.Fatal("Unknown direction")
		}
	}

	nodes := map[string]*Node{}

	for _, line := range lines[2:] {
		fields := strings.Fields(line)
		name := fields[0]

		r := strings.NewReplacer("(", "", ")", "", ",", "")
		left := r.Replace(fields[2])
		right := r.Replace(fields[3])

		nodes[name] = &Node{name: name, left: left, right: right}
	}

	return &Network{instructions: instructions, nodes: nodes}
}

type Network struct {
	instructions []Direction
	nodes        map[string]*Node
}

type Node struct {
	name  string
	left  string
	right string
}

func (n *Network) navigate() int {
	start := "AAA"
	end := "ZZZ"

	current := n.nodes[start]
	steps := 0

	for current.name != end {
		for _, direction := range n.instructions {
			steps++

			if direction == Left {
				current = n.nodes[current.left]
			} else if direction == Right {
				current = n.nodes[current.right]
			}

			if current.name == end {
				break
			}
		}
	}

	return steps
}
