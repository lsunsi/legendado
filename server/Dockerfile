FROM ekidd/rust-musl-builder:nightly-2019-08-13 AS builder

RUN cargo install migrant --features postgres

COPY Cargo.toml Cargo.lock ./

RUN sudo chown -R rust:rust /home/rust/src
RUN mkdir src && touch src/lib.rs && cargo build --release && rm -rf src

COPY src/ src/

RUN sudo chown -R rust:rust /home/rust/src
RUN cargo build --release

FROM alpine:3.10.2

COPY Rocket.toml Migrant.toml ./
COPY migrations/ migrations/

COPY --from=builder \
	/home/rust/.cargo/bin/migrant \
	/home/rust/src/target/x86_64-unknown-linux-musl/release/legendado-server \
	/usr/local/bin/

CMD sleep 5 &&\
	migrant setup &&\
	if migrant list | grep "\[ \]"; then migrant apply --all; fi &&\
	/usr/local/bin/legendado-server
