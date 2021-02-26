FROM liuchong/rustup

ENV ROCKET_ADDRESS=0.0.0.0

ADD src /app/src
ADD Cargo.toml /app

WORKDIR /app

RUN rustup default nightly

RUN cargo build

CMD ["cargo", "run"]
