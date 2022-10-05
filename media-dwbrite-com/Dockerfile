FROM rustlang/rust:nightly
COPY ./common ./common
COPY ./media-dwbrite-com ./media-dwbrite-com
RUN cargo build --release --manifest-path=./media-dwbrite-com/Cargo.toml
CMD ["./dwbrite-com/target/release/media-dwbrite-com"]
EXPOSE 41233
