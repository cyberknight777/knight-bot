FROM rust:1.69

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["knight-bot"]