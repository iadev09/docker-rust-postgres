run:
	docker run \
		--rm -d \
		--network postgresnet \
		--name docker-develop-rust-container \
		-p 3001:8000 \
		-e PG_DBNAME=example \
		-e PG_HOST=db \
		-e PG_USER=postgres \
		-e PG_PASSWORD=mysecretpassword \
		-e ADDRESS=0.0.0.0:8000 \
		-e RUST_LOG=debug \
		rust-backend-image