FROM rust:1.45.2 AS builder

# run as non-root user
RUN useradd -ms /bin/bash poke
USER poke

# needed for cargo init
env USER=poke

# create directory in order to give it user permissions
RUN mkdir /home/poke/poke-speare
WORKDIR /home/poke/poke-speare

# build project dependencies in order to let docker cache them.
RUN cargo init --vcs none
COPY Cargo.toml ./Cargo.toml
RUN cargo build --release

# remove useless files
RUN rm src/*.rs
RUN rm target/release/deps/poke_speare*

# build our project
COPY ./src ./src
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
