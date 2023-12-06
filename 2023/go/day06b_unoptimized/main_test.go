package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	solution := solve("../../inputs/day06_example_a.txt")
	assert.Equal(t, 71503, solution)
}

func TestInput(t *testing.T) {
	solution := solve("../../inputs/day06.txt")
	assert.Equal(t, 34278221, solution)
}

func TestInputOther(t *testing.T) {
	solution := solve("../../inputs/day06_other.txt")
	assert.Equal(t, 34123437, solution)
}

func BenchmarkInput(b *testing.B) {
	for n := 0; n < b.N; n++ {
		solve("../../inputs/day06.txt")
	}
}

func BenchmarkInputOther(b *testing.B) {
	for n := 0; n < b.N; n++ {
		solve("../../inputs/day06_other.txt")
	}
}
