package main

import (
	"bufio"
	"strings"
	"testing"

	"gotest.tools/assert"
)

var input string = `
3   4
4   3
2   5
1   3
3   9
3   3
`

func TestSampleT1(t *testing.T) {
	scanner := bufio.NewScanner(strings.NewReader(input))
	n := SolveT1(scanner)

	assert.Equal(t, 11, n)
}

func TestSampleT2(t *testing.T) {
	scanner := bufio.NewScanner(strings.NewReader(input))
	n := SolveT2(scanner)

	assert.Equal(t, 31, n)
}
