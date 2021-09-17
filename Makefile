mini-redis-server:
	@cargo install mini-redis
	cargo run --example async-mini-redis-server

mini-redis-client:
	cargo run --example async-mini-redis-client