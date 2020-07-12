FROM rust:1.44.1 AS builder

RUN useradd -ms /bin/bash poke
# run as non-root user
USER poke

# create directory in order to give it user permissions
RUN mkdir /home/poke/poke-speare
WORKDIR /home/poke/poke-speare

COPY . .

RUN cargo build --release


FROM debian:buster-slim

RUN apt-get update && \
  apt-get install -y \
    libssl1.1 \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash poke
# run as non-root user
USER poke

RUN mkdir /home/poke/bin
WORKDIR /home/poke/bin

COPY --from=builder /home/poke/poke-speare/target/release/poke-speare .

EXPOSE 5000

ENTRYPOINT [ "./poke-speare" ]
