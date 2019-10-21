FROM rustlang/rust:nightly-slim

RUN cargo install migrant --features postgres

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && touch src/lib.rs && cargo build --release && rm -rf src/*

COPY Rocket.toml Migrant.toml ./
COPY migrations/ migrations/

VOLUME /src /tests

CMD sleep 5 &&\
	migrant setup &&\
	if migrant list | grep "\[ \]"; then migrant apply --all; fi &&\
	cargo run --release
