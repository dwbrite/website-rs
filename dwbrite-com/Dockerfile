FROM rustlang/rust:nightly-bookworm AS builder
COPY ./common ./common
COPY ./dwbrite-com ./dwbrite-com
RUN cargo build --release --manifest-path=./dwbrite-com/Cargo.toml #insert $MEDIA_BASE_URL


FROM debian:bookworm-slim

COPY --from=builder ./dwbrite-com/.well-known                 /dwbrite-com/.well-known
COPY --from=builder ./dwbrite-com/blog                        /dwbrite-com/blog
COPY --from=builder ./dwbrite-com/resources                   /dwbrite-com/resources
COPY --from=builder ./dwbrite-com/resume                      /dwbrite-com/resume
COPY --from=builder ./dwbrite-com/templates                   /dwbrite-com/templates
COPY --from=builder ./dwbrite-com/target/release/dwbrite-com  /dwbrite-com/dwbrite-com

WORKDIR /dwbrite-com

RUN apt-get -y update && apt-get install -y libssl-dev # libssl3
CMD ["./dwbrite-com"]
EXPOSE 41234
