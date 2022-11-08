FROM rustlang/rust:nightly
COPY ./common ./common
COPY ./media-dwbrite-com ./media-dwbrite-com
RUN cargo build --release --manifest-path=./media-dwbrite-com/Cargo.toml

FROM debian:bullseye
COPY --from=0 ./media-dwbrite-com ./media-dwbrite-com
CMD ["./dwbrite-com/target/release/media-dwbrite-com"]
EXPOSE 41233
