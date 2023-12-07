package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	solution := solve("../../inputs/day07_example_a.txt")
	assert.Equal(t, 6440, solution)
}

func TestInput(t *testing.T) {
	solution := solve("../../inputs/day07.txt")
	assert.Equal(t, 253638586, solution)
}

func BenchmarkInput(b *testing.B) {
	for n := 0; n < b.N; n++ {
		solve("../../inputs/day07.txt")
	}
}
