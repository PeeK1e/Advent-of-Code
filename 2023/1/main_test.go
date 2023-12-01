package main

import (
	"testing"

	"gotest.tools/assert"
)

func TestNumericDigets(t *testing.T) {
	s := "4nineeightseven2"

	assert.Equal(t, 42, findDigits(s))
}

func TestComplexRegex(t *testing.T) {
	s := "7ckxjmlpkqqqjtfiveeightbmmdoneighttnv"
	assert.Equal(t, 78, findDigits(s))
}
