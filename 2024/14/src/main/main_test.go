package main

import (
	"fmt"
	"testing"

	"gotest.tools/assert"
)

func TestTest(t *testing.T) {
	solve1 := solve_1("../../example", 11, 7)
	fmt.Println("T1: ", solve1)
	assert.Equal(t, 12, solve1)
}
