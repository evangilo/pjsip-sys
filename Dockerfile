FROM  rust:1.48-buster

RUN apt-get update
RUN apt-get install -y clang

RUN git clone https://github.com/pjsip/pjproject.git /pjproject && \
    cd /pjproject && \
    ./configure && \
    make dep && \
    make clean && \
    make && \
    make && \
    make install

WORKDIR /app
COPY . .

RUN rustup update stable
RUN rustup component add rustfmt --toolchain 1.48.0-x86_64-unknown-linux-gnu

CMD ["cargo run"]
