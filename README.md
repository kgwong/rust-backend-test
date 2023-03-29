# rust-backend-test


## Run locally

1. `RUST_LOG=info cargo run`

## Run in local network

1. Bind service to the IP of the host computer (using `ifconfig`/`ipconfig`)
2. If host computer is Windows, verify that the Firewall rules are allowing this exe, and that the Protocols and Ports section specifies Local port: 8080

## Run in allow all connections

1. Port forward using router
2. Get public IP using something like whatismyip.com