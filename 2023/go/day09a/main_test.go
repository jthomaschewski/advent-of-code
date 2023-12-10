package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	solution := solve("../../inputs/day09_example_a.txt")
	assert.Equal(t, 114, solution)
}

// func TestInput(t *testing.T) {
// 	solution := solve("../../inputs/day09.txt")
// 	assert.Equal(t, 10818234074807, solution)
// }

func BenchmarkInput(b *testing.B) {
	for n := 0; n < b.N; n++ {
		solve("../../inputs/day09.txt")
	}
}
