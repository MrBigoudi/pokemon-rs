#!/bin/bash

# Function to display usage information
show_help() {
    echo "Usage: $0 [-c] [-h|--help]"
    echo
    echo "Options:"
    echo "  -c              Run wasm-pack build before starting the server"
    echo "  -h, --help      Display this help message and exit"
}

# Parse command-line options
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -c|--compile) RUN_WASM_PACK=true ;;
        -h|--help) show_help; exit 0 ;;
        *) echo "Unknown option: $1" >&2; show_help; exit 1 ;;
    esac
    shift
done

# Check if pkg directory exists; if not, set RUN_WASM_PACK to true
if [[ ! -d "pkg" ]]; then
    echo "pkg folder not found. Running wasm-pack build..."
    RUN_WASM_PACK=true
fi

# Run wasm-pack if -c option was specified
if [[ "$RUN_WASM_PACK" == "true" ]]; then
    echo "Running wasm-pack..."
    wasm-pack build --target web
    if [ $? -ne 0 ]; then
        echo "wasm-pack build failed!"
        exit 1
    fi
fi

# Start a Python HTTP server in the background
echo "Starting Python HTTP server on port 8080..."
python3 -m http.server 8080 &
SERVER_PID=$!
echo "Python server started with PID $SERVER_PID"

# Open index.html in Firefox
echo "Opening index.html in Firefox..."
firefox http://localhost:8080/index.html &

# Get the PID of the last background command (Firefox)
FIREFOX_PID=$!
echo "Firefox opened with PID $FIREFOX_PID"

# Wait for Firefox to close
wait $FIREFOX_PID

# When Firefox closes, kill the Python server
echo "Closing Python server with PID $SERVER_PID..."
kill $SERVER_PID
echo "Python server stopped"

echo "Script finished"