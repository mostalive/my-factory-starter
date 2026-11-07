package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	seed := flag.String("seed", "", "seed file (required)")
	target := flag.String("target", "", "target folder (required)")
	flag.Parse()
	if *seed == "" || *target == "" {
		fmt.Fprintln(os.Stderr, "--seed and --target are required")
		os.Exit(1)
	}
	fmt.Println("factory not built yet")
}
