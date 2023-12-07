package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	solution := solve("../../inputs/day07_example_a.txt")
	assert.Equal(t, 5905, solution)
}

func TestHandFourOfAKindJoker(t *testing.T) {
	hand := Hand{cards: []byte("J58JJ"), bid: 5}
	assert.Equal(t, 6, hand.typeValue())
}

func TestHandThreeOfAKindJoker(t *testing.T) {
	hand := Hand{cards: []byte("858JJ"), bid: 5}
	assert.Equal(t, 6, hand.typeValue())
}

func TestHandTwoPair(t *testing.T) {
	hand := Hand{cards: []byte("A3322"), bid: 5}
	assert.Equal(t, 3, hand.typeValue())
}

func TestInput(t *testing.T) {
	solution := solve("../../inputs/day07.txt")
	assert.Equal(t, 253253225, solution)
}

func BenchmarkInput(b *testing.B) {
	for n := 0; n < b.N; n++ {
		solve("../../inputs/day07.txt")
	}
}
