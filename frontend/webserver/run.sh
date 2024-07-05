# Function to handle cleanup
cleanup() {
	echo "Terminating processes..."
	kill $CARGO_PID $PYTHON_PID
	wait $CARGO_PID $PYTHON_PID 2>/dev/null
	echo "Cleanup done."
}

# Trap signals and call cleanup
trap cleanup SIGINT SIGTERM

cd webserver-rust || exit

cargo run &
CARGO_PID=$!

python3 -m http.server &
PYTHON_PID=$!

wait $CARGO_PID $PYTHON_PID
