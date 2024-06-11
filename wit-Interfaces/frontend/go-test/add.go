package main

import (
	"bufio"
	"fmt"
	"log"
	"net"

	. "example.com/gen"
)

type (
	FrontendAddImpl    struct{}
	FrontendClientImpl struct{}
)

func (i FrontendAddImpl) Add(x, y uint32) uint32 {
	return x + y
}

func (i FrontendClientImpl) GetSensorData() string {
	fmt.Println("Starting server request func...")

	// Connect to the server
	conn, err := net.Dial("tcp", "localhost:12345")
	if err != nil {
		log.Fatal("Error connecting to server:", err)
	}
	fmt.Println("Connected to server")
	defer conn.Close()

	// Send request to the server
	fmt.Fprintf(conn, "GET /sensordata HTTP/1.0\r\n\r\n")

	// Read response from the server
	scanner := bufio.NewScanner(conn)
	for scanner.Scan() {
		fmt.Println(scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		log.Fatal("Error reading from server:", err)
	}

	return scanner.Text()
}

// To enable our component to be a library, implement the component in the
// `init` function which is always called first when a Go package is run.
func init() {
	add := FrontendAddImpl{}
	client := FrontendClientImpl{}

	SetExportsBachelorFrontend0_1_0_Add(add)
	SetExportsBachelorFrontend0_1_0_Client(client)
}

// main is required for the `WASI` target, even if it isn't used.
func main() {
	add := FrontendAddImpl{}
	result := add.Add(1, 2)
	fmt.Println("Result of 1 + 2 = ", result)

	fmt.Println("Getting server response...")
	client := FrontendClientImpl{}
	response := client.GetSensorData()

	fmt.Println("Response of server: ", response)
}
