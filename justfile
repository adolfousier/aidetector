set dotenv-load

default: run

# Build client and server
build: build-client build-server

# Build client extension
build-client:
    cd client && npm install --silent && rm -rf dist && npm run build

# Build server binary
build-server:
    cd server && cargo build

# Build everything and start the server
run: build
    cd server && cargo run

# Stop the server
stop:
    pkill -f "target/debug/aidetector-server" 2>/dev/null; echo "stopped"

# Clean all build artifacts
clean: stop
    rm -rf client/dist client/node_modules server/target
    @echo "cleaned"
