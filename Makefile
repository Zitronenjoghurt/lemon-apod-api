up:
	docker compose up --build

clean:
	docker compose down -v --remove-orphans

test:
	grpcurl -plaintext -import-path ./proto -proto apod.proto -d '{}' '[::1]:50051' lemon_apod.Apod/GetToday