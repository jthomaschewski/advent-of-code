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
	startNames := []string{}

	for _, line := range lines[2:] {
		fields := strings.Fields(line)
		name := fields[0]

		r := strings.NewReplacer("(", "", ")", "", ",", "")
		left := r.Replace(fields[2])
		right := r.Replace(fields[3])

		if strings.HasSuffix(name, "A") {
			startNames = append(startNames, name)
		}

		nodes[name] = &Node{name: name, left: left, right: right}

	}

	start := make([]*Node, len(startNames))
	for i, name := range startNames {
		start[i] = nodes[name]
	}

	return &Network{instructions: instructions, nodes: nodes, start: start}
}

type Network struct {
	instructions []Direction
	nodes        map[string]*Node
	start        []*Node
}

type Node struct {
	name  string
	left  string
	right string
}

func (n *Network) navigate() int {
	stepsToEnd := make([]int, len(n.start))

	for i, start := range n.start {
		current := start
		steps := 0
		for !strings.HasSuffix(current.name, "Z") {
			for _, direction := range n.instructions {
				steps++
				if direction == Left {
					current = n.nodes[current.left]
				} else if direction == Right {
					current = n.nodes[current.right]
				}
				if strings.HasSuffix(current.name, "Z") {
					stepsToEnd[i] = steps
					break
				}
			}
		}
	}
	return lcm(stepsToEnd)
}

// find Least Common Multiple (lcm) via GCD
func lcm(integers []int) int {
	n := int(1)
	for i := 0; i < len(integers); i++ {
		n = lcm2(integers[i], n)
	}

	return n
}

func gcd(a, b int) int {
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

func lcm2(a, b int) int {
	result := a * b / gcd(a, b)
	return result
}

