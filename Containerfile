FROM docker.io/library/rust:1.81-alpine3.20 AS cargo
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/train-trouble

COPY train-trouble/ ./train-trouble/
COPY train-trouble-engine/ ./train-trouble-engine/
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

FROM docker.io/library/node:22.9-alpine3.20 as vite
WORKDIR /usr/src/train-trouble

COPY train-trouble-client/ ./train-trouble-client/
COPY train-trouble-engine-client/ ./train-trouble-engine-client/

WORKDIR /usr/src/train-trouble/train-trouble-client/
RUN npm ci --no-audit --no-fund
RUN npm run build

FROM docker.io/library/alpine:3.20 AS runtime
RUN apk add --no-cache tini
RUN adduser -S conductor

COPY --from=cargo /usr/src/train-trouble/target/release/train-trouble /usr/local/bin/
COPY --from=vite /usr/src/train-trouble/train-trouble-client/dist/ /srv/train-trouble/www/
RUN mkdir /var/lib/train-trouble/
RUN chown conductor /var/lib/train-trouble/

ENV PORT=8000
ENV SERVE=/srv/train-trouble/www/
ENV SAVE=/var/lib/train-trouble/save.json

USER conductor
EXPOSE 8000
ENTRYPOINT tini train-trouble
