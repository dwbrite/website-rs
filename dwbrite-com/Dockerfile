FROM rustlang/rust:nightly
COPY ./common ./common
COPY ./dwbrite-com ./dwbrite-com
RUN cargo build --release --manifest-path=./dwbrite-com/Cargo.toml #insert $MEDIA_BASE_URL

FROM debian:bullseye
COPY --from=0 ./dwbrite-com ./dwbrite-com
RUN apt-get -y update && apt-get install -y libssl-dev libssl1.1
WORKDIR "./dwbrite-com"
CMD ["./target/release/dwbrite-com"]
EXPOSE 41234
