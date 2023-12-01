package io

import (
	"bufio"
	"os"
)

const defaultPath string = "./example.txt"

func ReadFileByte(path string) *[]byte {
	p := getPath(path)
	b, err := os.ReadFile(p)
	if err != nil {
		panic(err)
	}

	return &b
}

func FileScanner(path string) *bufio.Scanner {
	p := getPath(path)
	f, err := os.Open(p)
	if err != nil {
		panic(err)
	}

	s := bufio.NewScanner(f)
	return s
}

func getPath(path string) string {
	if len(path) > 0 {
		return path
	}

	return defaultPath
}
