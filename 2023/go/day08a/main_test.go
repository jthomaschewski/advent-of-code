package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExampleA1(t *testing.T) {
	solution := solve("../../inputs/day08_example_a1.txt")
	assert.Equal(t, 2, solution)
}

func TestExampleA2(t *testing.T) {
	solution := solve("../../inputs/day08_example_a2.txt")
	assert.Equal(t, 6, solution)
}

func TestInput(t *testing.T) {
	solution := solve("../../inputs/day08.txt")
	assert.Equal(t, 17141, solution)
}

func BenchmarkInput(b *testing.B) {
	for n := 0; n < b.N; n++ {
		solve("../../inputs/day08.txt")
	}
}
