package main

import (
	"fmt"
	"strconv"
	"strings"

	aocio "github.com/PeeK1e/Advent-of-Code/lib/pkg/aocio"
)

func main() {
	s := aocio.FileScanner("./input.txt")

	s.Scan()
	t := s.Text()

	s.Scan()
	d := s.Text()

	p := makePairs(t, d)

	r := 1
	for _, v := range p {
		r *= calcWinningMoves(v)
	}
	fmt.Println(r)

	p2 := makePairs(strings.Replace(t, " ", "", -1), strings.Replace(d, " ", "", -1))
	r = calcWinningMoves(p2[0])
	fmt.Println(r)
}

func makePairs(t string, d string) map[int][]int {
	b, a, _ := strings.Cut(t, "Time:")
	t = b + a

	b, a, _ = strings.Cut(d, "Distance:")
	d = b + a

	tl := strings.Fields(t)
	dl := strings.Fields(d)

	pairs := make(map[int][]int, len(tl))

	for i, _ := range tl {
		tn, _ := strconv.Atoi(tl[i])
		dn, _ := strconv.Atoi(dl[i])

		l := make([]int, 2)
		l[0] = tn
		l[1] = dn

		pairs[i] = l
	}

	return pairs
}

func calcWinningMoves(pair []int) int {
	w := 0
	for i := 0; i < pair[0]; i++ {
		if (i * (pair[0] - i)) > pair[1] {
			w++
		}
	}
	return w
}
