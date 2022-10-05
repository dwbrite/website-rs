FROM rustlang/rust:nightly
COPY ./common ./common
COPY ./dwbrite-com ./dwbrite-com
#ARG MEDIA_BASE_URL
RUN cargo build --release --manifest-path=./dwbrite-com/Cargo.toml #insert $MEDIA_BASE_URL
WORKDIR "./dwbrite-com"
CMD ["./target/release/dwbrite-com"]
EXPOSE 41234
