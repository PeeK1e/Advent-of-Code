package main

import (
	"bufio"
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"

	aocio "github.com/PeeK1e/Advent-of-Code/lib/pkg/aocio"
)

func main() {
	s := aocio.FileScanner("./input")
	n := SolveT1(s)
	fmt.Printf("\nSolved T1: %d", n)

	s = aocio.FileScanner("./input")
	m := SolveT2(s)
	fmt.Printf("\nSolved T2: %d", m)
}

func SolveT1(s *bufio.Scanner) int {
	l1 := make([]string, 100)
	l2 := make([]string, 100)

	for s.Scan() {
		t := s.Text()
		if t == "" {
			continue
		}
		l := strings.Split(t, "   ")
		l1 = append(l1, l[0])
		l2 = append(l2, l[1])
	}

	slices.Sort(l1)
	slices.Sort(l2)

	n := 0
	for i := 0; i < len(l1); i++ {
		a, _ := strconv.Atoi(l1[i])
		b, _ := strconv.Atoi(l2[i])
		c := a - b
		n += int(math.Abs(float64(c)))
	}

	return n
}

func SolveT2(s *bufio.Scanner) int {
	l1 := make([]string, 0)
	l2 := make([]string, 0)

	for s.Scan() {
		t := s.Text()
		if t == "" {
			continue
		}
		l := strings.Split(t, "   ")
		l1 = append(l1, l[0])
		l2 = append(l2, l[1])
	}

	slices.Sort(l1)
	slices.Sort(l2)

	n := 0
	for i := 0; i < len(l1); i++ {
		sim := 0
		for j := 0; j < len(l2); j++ {
			if l1[i] == l2[j] {
				sim++
			}
		}

		a, _ := strconv.Atoi(l1[i])
		n += int(math.Abs(float64(a * sim)))
	}

	return n
}
