package main

import (
	"bufio"
	"strings"
	"testing"

	"gotest.tools/assert"
)

var input string = `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`

func TestSum(t *testing.T) {
	s := bufio.NewScanner(strings.NewReader(input))

	count := 1
	sum := 0

	for s.Scan() {
		l := strings.Split(s.Text(), ":")

		if validGame(l[1]) {
			sum += count
		}
		count++
	}

	assert.Equal(t, 8, sum)
}

func TestPower(t *testing.T) {
	s := bufio.NewScanner(strings.NewReader(input))
	power := 0
	for s.Scan() {
		l := strings.Split(s.Text(), ":")
		sl := strings.Split(strings.Trim(l[1], " "), ";")

		power += gamePower(sl)

	}

	assert.Equal(t, 2286, power)
}

func TestGameSetPower(t *testing.T) {
	s := " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
	sl := strings.Split(strings.Trim(s, " "), ";")
	pow := gamePower(sl)

	assert.Equal(t, 48, pow)
}

func TestInvalidSets(t *testing.T) {
	s := "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
	sl := strings.Split(s, ";")
	b := checkGames(sl)

	assert.Equal(t, b, false)
}

func TestValidSets(t *testing.T) {
	s := "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
	sl := strings.Split(s, ";")
	b := checkGames(sl)

	assert.Equal(t, b, true)
}

func TestParseNumberColor(t *testing.T) {
	s := " 3 blue"
	n, c := parseNumberColor(s)
	assert.Equal(t, n, 3)
	assert.Equal(t, c, "blue")
}
