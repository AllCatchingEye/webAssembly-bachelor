package main

import (
	"encoding/json"

	. "parser.com/gen"
)

type ParserImpl struct{}

//go:wasm-module yourmodulename
//export parseIds
func (i ParserImpl) ParseIds(jsonStr string) []uint32 {
	var data []map[string]interface{}
	json.Unmarshal([]byte(jsonStr), &data)

	ids := make([]uint32, len(data))
	for i, item := range data {
		if id, ok := item["id"].(float64); ok {
			ids[i] = uint32(id)
		}
	}

	return ids
}

func (i ParserImpl) ParseTemperatures(jsonStr string) []int32 {
	var data []map[string]interface{}
	json.Unmarshal([]byte(jsonStr), &data)

	temperatures := make([]int32, len(data))
	for i, item := range data {
		if temperature, ok := item["temperature"].(float64); ok {
			temperatures[i] = int32(temperature)
		}
	}

	return temperatures
}

func (i ParserImpl) ParseHumidities(jsonStr string) []uint32 {
	var data []map[string]interface{}
	json.Unmarshal([]byte(jsonStr), &data)

	humidities := make([]uint32, len(data))
	for i, item := range data {
		if humidity, ok := item["humidity"].(float64); ok {
			humidities[i] = uint32(humidity)
		}
	}

	return humidities
}

// To enable our component to be a library, implement the component in the
// `init` function which is always called first when a Go package is run.
func init() {
	parser := ParserImpl{}
	SetExportsFrontendWebserver0_1_0_Parse(parser)
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
