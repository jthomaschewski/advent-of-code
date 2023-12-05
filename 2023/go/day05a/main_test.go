package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	solution := solve("../../inputs/day05_example_a.txt")
	assert.Equal(t, 35, solution)
}

func TestInput(t *testing.T) {
	solution := solve("../../inputs/day05.txt")
	assert.Equal(t, 825516882, solution)
}
