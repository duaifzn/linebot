FROM ubuntu:20.04
# Update package lists and install necessary dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl

# Install Rust using Rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set the environment variables required by Rust
ENV PATH="/root/.cargo/bin:${PATH}"
ENV USER=root
ENV DEBIAN_FRONTEND=noninteractive

RUN curl -LO https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
RUN apt-get install -y ./google-chrome-stable_current_amd64.deb
RUN rm google-chrome-stable_current_amd64.deb 

RUN apt install -y default-libmysqlclient-dev libssl-dev pkg-config
RUN apt install -y libleptonica-dev libtesseract-dev clang tesseract-ocr tesseract-ocr-eng

RUN cargo install diesel_cli --no-default-features --features mysql
WORKDIR /usr/src/myapp
COPY . ./

RUN cargo build
