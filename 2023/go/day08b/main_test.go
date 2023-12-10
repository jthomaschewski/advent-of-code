package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExampleA1(t *testing.T) {
	solution := solve("../../inputs/day08_example_b.txt")
	assert.Equal(t, 6, solution)
}

func TestInput(t *testing.T) {
	solution := solve("../../inputs/day08.txt")
	assert.Equal(t, 10818234074807, solution)
}

func BenchmarkInput(b *testing.B) {
	for n := 0; n < b.N; n++ {
		solve("../../inputs/day08.txt")
	}
}
