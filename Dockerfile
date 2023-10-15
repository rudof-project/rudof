ARG BINARY_NAME

FROM clux/muslrust:latest as builder
RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser

ARG BINARY_NAME

# Now add the rest of the project and build the real main
COPY . ./
RUN set -x && cargo build --target x86_64-unknown-linux-musl --release
RUN mkdir -p /build-out
RUN set -x && cp target/x86_64-unknown-linux-musl/release/$BINARY_NAME /build-out/

# Create a minimal docker image 
FROM scratch

ARG BINARY_NAME

COPY --from=0 /etc/passwd /etc/passwd
USER dockeruser

ENV RUST_LOG="error,$BINARY_NAME=info"
COPY --from=builder /build-out/$BINARY_NAME /

ENTRYPOINT ["/$BINARY_NAME"]
