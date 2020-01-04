FROM clux/muslrust:stable as build

WORKDIR /app
COPY . .
RUN cargo build --release
RUN ls ./target

FROM scratch
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/randtime /
COPY --from=build /app/static /static
COPY --from=build /app/templates /templates
COPY --from=build /app/.env /.env
CMD ["./randtime"]
