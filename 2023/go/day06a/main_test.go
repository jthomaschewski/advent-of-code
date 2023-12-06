package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	solution := solve("../../inputs/day06_example_a.txt")
	assert.Equal(t, 288, solution)
}

func TestInput(t *testing.T) {
	solution := solve("../../inputs/day06.txt")
	assert.Equal(t, 32076, solution)
}
