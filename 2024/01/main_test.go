package main

import (
	"bufio"
	"strings"
	"testing"

	aocio "github.com/PeeK1e/Advent-of-Code/lib/pkg/aocio"
	"gotest.tools/assert"
)

var input string = `3   4
4   3
2   5
1   3
3   9
3   3
`

func BenchmarkMain(b *testing.B) {

	main()
}

var global int

func BenchmarkT1(b *testing.B) {
	s := aocio.FileScanner("./input")
	for range b.N {
		global = SolveT1(s)
	}
}

func BenchmarkT2(b *testing.B) {
	s := aocio.FileScanner("./input")
	for range b.N {
		global = SolveT2(s)
	}
}

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
