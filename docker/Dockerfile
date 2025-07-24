# WIP
FROM ubuntu:latest
RUN  apt-get update && apt-get upgrade -y
RUN  apt-get install -y build-essential git sudo file rustup
RUN rustup default stable
RUN cargo install cargo-deb
RUN  dpkg --add-architecture arm64
COPY ubuntu.sources /etc/apt/sources.list.d/
RUN  apt-get update
RUN  apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu libc6-dev:arm64
RUN rustup target add aarch64-unknown-linux-gnu

COPY sudoer.user* /etc/sudoers.d/$USER 
RUN  chown root /etc/sudoers.d/$USER
RUN  deluser ubuntu
RUN  useradd andrew -u 1000 -G sudo
RUN  echo andrew:password | chpasswd
