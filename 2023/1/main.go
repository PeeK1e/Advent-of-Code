package main

import (
	"log"
	"regexp"
	"strconv"

	aocio "github.com/PeeK1e/Advent-of-Code/lib/pkg/aocio"
)

func main() {
	// s := aocio.FileScanner("")
	s := aocio.FileScanner("prod.txt")
	sum := 0
	for s.Scan() {
		i := findDigits(s.Text())
		sum += i
	}
	printResult(sum)
}

func printResult(v int) {
	log.Printf("Result: %d", v)
}

func findDigits(s string) int {
	re := regexp.MustCompile(`(one|two|three|four|five|six|seven|eight|nine|\d)`)

	// normally you would need a check if there is only one number in the line
	// since we just assume there are always at least two numbers we neglect the check here
	l := convertToNumber(re.FindString(s))
	r := convertToNumber(findMatchBack(s, re))

	i, _ := strconv.Atoi(l + r)

	return i
}

func findMatchBack(s string, rx *regexp.Regexp) string {
	srch := ""
	for i := len(s) - 1; i >= 0; i-- {
		srch = string(s[i]) + srch
		if r := rx.FindString(srch); r != "" {
			return r
		}
	}
	return ""
}

func convertToNumber(s string) string {
	if len(s) <= 1 {
		return s
	}

	switch s {
	case "one":
		return "1"
	case "two":
		return "2"
	case "three":
		return "3"
	case "four":
		return "4"
	case "five":
		return "5"
	case "six":
		return "6"
	case "seven":
		return "7"
	case "eight":
		return "8"
	case "nine":
		return "9"
	default:
		return ""
	}
}
