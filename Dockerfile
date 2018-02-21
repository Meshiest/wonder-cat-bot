FROM rust:1.24.0

WORKDIR /usr/src/wondercatbot
COPY . .

RUN cargo install
CMD ["wonder-cat-bot"]