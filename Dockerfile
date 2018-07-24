FROM rust:1.27.0

WORKDIR /usr/bin/swipe-server
COPY . .

RUN cargo install

EXPOSE 8000

CMD ["swipe-server"]
