FROM rust:latest
WORKDIR /home
COPY . .
RUN cargo build --release
EXPOSE 8000
CMD ["target/release/mkproj_com"]