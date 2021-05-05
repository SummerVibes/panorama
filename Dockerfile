# download dependence and build rust code
FROM arm64v8/rust:latest

# 本机docker编译，本机docker运行
WORKDIR /panorama
COPY . .
#COPY ./src ./src/
#COPY ./Cargo.toml .
COPY ./config /usr/local/cargo/
#COPY ./target/debug/panorama .
#RUN cargo run --package panorama --bin panorama Phone
RUN cargo build --release
#RUN cargo build --release
#CMD ["/panorama","Phone"]