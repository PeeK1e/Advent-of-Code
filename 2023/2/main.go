package main

import (
	"fmt"
	"strconv"
	"strings"

	aocio "github.com/PeeK1e/Advent-of-Code/lib/pkg/aocio"
)

const (
	red   int = 12
	green int = 13
	blue  int = 14
)

var power int = 0

func main() {
	s := aocio.FileScanner("input.txt")

	count := 1
	sum := 0
	power = 0

	for s.Scan() {
		l := strings.Split(s.Text(), ":")

		if validGame(l[1]) {
			sum += count
		}

		count++
	}

	fmt.Printf("Sum of Valid Games %d\n", sum)
	fmt.Printf("Power of Games %d", power)
}

func validGame(s string) bool {
	sl := strings.Split(s, ";")
	power += gamePower(sl)
	return checkGames(sl)
}

func checkGames(sl []string) bool {
	for _, v := range sl {
		sl := strings.Split(strings.Trim(v, " "), ",")
		if !checkSet(sl) {
			return false
		}
	}
	return true
}

func checkSet(sl []string) bool {
	for _, v := range sl {
		n, c := parseNumberColor(v)
		switch c {
		case "blue":
			if n > blue {
				return false
			}
			break
		case "green":
			if n > green {
				return false
			}
			break
		case "red":
			if n > red {
				return false
			}
			break
		}
	}
	return true
}

func parseNumberColor(s string) (int, string) {
	st := strings.Trim(s, " ")
	sl := strings.Split(st, " ")
	n, _ := strconv.Atoi(sl[0])
	return n, sl[1]
}

func gamePower(s []string) int {
	r, g, b := 1, 1, 1

	for _, v := range s {
		sl := strings.Split(strings.Trim(v, " "), ",")
		for _, vv := range sl {
			n, c := parseNumberColor(vv)
			switch c {
			case "blue":
				if n > b {
					b = n
				}
				break
			case "green":
				if n > g {
					g = n
				}
				break
			case "red":
				if n > r {
					r = n
				}
				break
			}
		}
	}

	return r * g * b
}
