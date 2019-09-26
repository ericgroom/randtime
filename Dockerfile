FROM clux/muslrust:nightly as build

WORKDIR /app
COPY . .
RUN cargo build --release
RUN ls ./target

FROM alpine:latest
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/randtime /
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/writetime /
COPY --from=build /app /
CMD ["./randtime"]
