FROM rustlang/rust:nightly AS builder
COPY ./common ./common
COPY ./dwbrite-com ./dwbrite-com
RUN cargo build --release --manifest-path=./dwbrite-com/Cargo.toml #insert $MEDIA_BASE_URL


FROM debian:bullseye-slim

COPY --from=builder ./dwbrite-com/.well-known                 ./well-known
COPY --from=builder ./dwbrite-com/blog                        ./blog
COPY --from=builder ./dwbrite-com/resources                   ./resources
COPY --from=builder ./dwbrite-com/resume                      ./resume
COPY --from=builder ./dwbrite-com/templates                   ./templates
COPY --from=builder ./dwbrite-com/target/release/dwbrite-com  ./dwbrite-com

RUN apt-get -y update && apt-get install -y libssl-dev libssl1.1
CMD ["./dwbrite-com"]
EXPOSE 41234
