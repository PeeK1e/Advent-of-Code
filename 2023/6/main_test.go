package main

import (
	"fmt"
	"reflect"
	"testing"

	"gotest.tools/assert"
)

func TestNumberOfWinningGames(t *testing.T) {
	tm := "Time:      7  15   30"
	d := "Distance:  9  40  200"

	p := makePairs(tm, d)

	pairExpected := make(map[int][]int, 3)
	pairExpected[0] = []int{7, 9}
	pairExpected[1] = []int{15, 40}
	pairExpected[2] = []int{30, 200}

	if !reflect.DeepEqual(p, pairExpected) {
		t.Error("values not equal")
	}
}

func TestWinningOptions(t *testing.T) {
	n := calcWinningMoves([]int{7, 9})
	assert.Equal(t, 4, n)
}

func TestIamTooLazyToParseTask2(t *testing.T) {
	n := calcWinningMoves([]int{44826981, 202107611381458})
	fmt.Print(n)
}

func BenchmarkIamTooLazyToParseTask2(b *testing.B) {
	n := calcWinningMoves([]int{44826981, 202107611381458})
	fmt.Print(n)
}
