package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	solution := solve("../../inputs/day09_example_a.txt")
	assert.Equal(t, 2, solution)
}

func TestInput(t *testing.T) {
	solution := solve("../../inputs/day09.txt")
	assert.Equal(t, 1031, solution)
}

func BenchmarkInput(b *testing.B) {
	for n := 0; n < b.N; n++ {
		solve("../../inputs/day09.txt")
	}
}
