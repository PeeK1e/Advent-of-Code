package main

import (
	"fmt"
	"strconv"
	"strings"

	aocio "github.com/PeeK1e/Advent-of-Code/lib/pkg/aocio"
)

type Robot struct {
	X  int
	Y  int
	VX int
	VY int
}

type RobotList []Robot

func main() {
	solve1 := solve_1("../../input", 101, 103)
	fmt.Println("T1: ", solve1)

	solve2 := solve_2("../../input", 101, 103)
	fmt.Println("T2: ", solve2)
}

func parse(path string) RobotList {
	lines := aocio.FileScanner(path)

	roboList := make([]Robot, 0, 100)

	for lines.Scan() {
		line := lines.Text()
		line = strings.ReplaceAll(line, "p=", "")
		line = strings.ReplaceAll(line, "v=", "")
		numbers := strings.Split(line, " ")
		robo_pos := strings.Split(numbers[0], ",")
		robo_vel := strings.Split(numbers[1], ",")
		x, _ := strconv.Atoi(robo_pos[0])
		y, _ := strconv.Atoi(robo_pos[1])
		vx, _ := strconv.Atoi(robo_vel[0])
		vy, _ := strconv.Atoi(robo_vel[1])

		roboList = append(roboList, Robot{
			X:  x,
			Y:  y,
			VX: vx,
			VY: vy,
		})
	}

	return roboList
}

func solve_1(path string, lx int, ly int) int {
	robots := parse(path)

	chart := make([][]int, ly)
	for i := range len(chart) {
		row := make([]int, lx)
		chart[i] = row
	}

	for i := 0; i < 100; i++ {
		for i := range robots {
			newpos := (robots[i].X + robots[i].VX)
			if robots[i].VX > 0 {
				robots[i].X = newpos % lx
			} else {
				newpos := (robots[i].X + robots[i].VX) % lx
				if newpos < 0 {
					robots[i].X = lx + newpos
				} else {
					robots[i].X = newpos
				}
			}
			newpos = robots[i].Y + robots[i].VY
			if robots[i].VY > 0 {
				robots[i].Y = newpos % ly
			} else {
				if newpos < 0 {
					robots[i].Y = (ly + newpos) % ly
				} else {
					robots[i].Y = newpos % ly
				}
			}
		}
	}

	for _, robo := range robots {
		chart[robo.Y][robo.X] += 1
	}
	for _, line := range chart {
		for _, field := range line {
			if field == 0 {
				fmt.Print(".")
			} else {
				fmt.Print(field)
			}
		}
		fmt.Println()
	}

	quadrant := make([][]int, 2)
	quadrant[0] = make([]int, 2)
	quadrant[1] = make([]int, 2)

	for _, robo := range robots {
		xq := 0
		yq := 0
		if robo.X == lx/2 || robo.Y == ly/2 {
			continue
		}
		if robo.X > lx/2 {
			xq = 1
		}
		if robo.Y > ly/2 {
			yq = 1
		}
		quadrant[yq][xq] += 1
	}

	return quadrant[0][0] * quadrant[0][1] * quadrant[1][0] * quadrant[1][1]
}

func solve_2(path string, lx int, ly int) int {
	robots := parse(path)

	chart := make([][]int, ly)
	for i := range len(chart) {
		row := make([]int, lx)
		chart[i] = row
	}
	i := 0
	for {
		i++
		// clean map
		for _, robo := range robots {
			chart[robo.Y][robo.X] = 0
		}

		for i := range robots {
			newpos := (robots[i].X + robots[i].VX)
			if robots[i].VX > 0 {
				robots[i].X = newpos % lx
			} else {
				newpos := (robots[i].X + robots[i].VX) % lx
				if newpos < 0 {
					robots[i].X = lx + newpos
				} else {
					robots[i].X = newpos
				}
			}
			newpos = robots[i].Y + robots[i].VY
			if robots[i].VY > 0 {
				robots[i].Y = newpos % ly
			} else {
				if newpos < 0 {
					robots[i].Y = (ly + newpos) % ly
				} else {
					robots[i].Y = newpos % ly
				}
			}
		}
		for _, robo := range robots {
			chart[robo.Y][robo.X] += 1
		}
		outline := ""
		for _, line := range chart {
			for _, field := range line {
				if field == 0 {
					outline += fmt.Sprint(".")
				} else {
					outline += fmt.Sprint("#")
				}
			}
		}
		if strings.Contains(outline, "#....#####################....#") {
			fmt.Println("Tree found at: ", i)
			break
		}
	}

	for _, line := range chart {
		for _, field := range line {
			if field == 0 {
				fmt.Print(".")
			} else {
				fmt.Print(field)
			}
		}
		fmt.Println()
	}

	return i
}
