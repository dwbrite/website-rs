FROM rustlang/rust:nightly AS builder
COPY ./common ./common
COPY ./media-dwbrite-com ./media-dwbrite-com
RUN cargo build --release --manifest-path=./media-dwbrite-com/Cargo.toml


FROM debian:bullseye-slim

COPY --from=builder ./media-dwbrite-com/target/release/media-dwbrite-com ./media-dwbrite-com
COPY --from=builder ./media-dwbrite-com/index.html ./index.html
COPY --from=builder ./media-dwbrite-com/media ./media
COPY --from=builder ./media-dwbrite-com/registry.toml ./registry.toml

CMD ["./media-dwbrite-com"]
EXPOSE 41233
