package main

import (
	"flag"
	"testing"

	"github.com/cucumber/godog"
)

var featureTags = flag.String("tags", "~@real-agent", "Gherkin tag filter")

func TestFeatures(t *testing.T) {
	suite := godog.TestSuite{
		ScenarioInitializer: initializeScenario,
		Options: &godog.Options{
			Format:   "progress",
			Paths:    []string{"spec/features"},
			Tags:     *featureTags,
			Strict:   true,
			TestingT: t,
		},
	}
	if suite.Run() != 0 {
		t.Fatal("feature suite has failing or undefined steps")
	}
}

func initializeScenario(sc *godog.ScenarioContext) {
}
