#!/bin/bash

# SafeSurf Demo Script

# 1. Start the daemon in the background
echo "Starting SafeSurf Daemon..."
cargo run -p safe_surfd -- --addr 127.0.0.1:3000 &
DAEMON_PID=$!

# Wait for daemon to start
sleep 5

# 2. Check status
echo "Checking Daemon Status..."
cargo run -p safe_surf_cli -- status

# 3. Initialize a session
echo "Initializing Session..."
cargo run -p safe_surf_cli -- init

# 4. Sanitize synthetic "malicious" content
echo "Sanitizing Malicious Content..."
echo "<html><body><h1>Welcome</h1><script>alert('Stealing your coins!')</script><p>Contact us at hacker@evil.com</p></body></html>" > /tmp/malicious.html
cargo run -p safe_surf_cli -- sanitize --url "http://malicious-site.onion" --file /tmp/malicious.html

# 5. Cleanup
echo "Cleaning up..."
kill $DAEMON_PID
rm /tmp/malicious.html

echo "Demo Complete."
