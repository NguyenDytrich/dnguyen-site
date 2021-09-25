ARG BUILD_ENV

FROM rust:1.55-buster as base
WORKDIR /app
COPY . .

FROM base as builder_dev
ONBUILD RUN cargo build
ONBUILD RUN mkdir bin
ONBUILD RUN mv target/debug/dnguyen_blog bin

FROM base as builder_prod
ONBUILD RUN cargo build --release
ONBUILD RUN mkdir bin
ONBUILD RUN mv .target/release/dnguyen_blog bin

FROM builder_${BUILD_ENV} as copier


# Note: the final image has the same directory structure as
# the builder images. That ensures that the /static/ and
# /template/ directories will be found by Rocket.
FROM debian:buster-slim as final
WORKDIR /app
COPY --from=copier /app/bin/dnguyen_blog ./
COPY --from=copier /app/templates ./templates
COPY --from=copier /app/static ./static
COPY --from=copier /app/Rocket.toml ./
RUN chmod +x ./dnguyen_blog
CMD ["./dnguyen_blog"]
