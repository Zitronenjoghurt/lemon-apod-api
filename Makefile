up:
	cargo run --bin lemon-apod-server

test:
	grpcurl -plaintext -import-path ./proto -proto apod.proto -d '{}' '[::1]:50051' lemon_apod.Apod/GetToday