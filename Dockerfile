FROM rust:1.48-slim

WORKDIR /graphql-app/

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

COPY diesel.toml diesel.toml

COPY .env .env

RUN mkdir migrations
COPY migrations/ migrations/

RUN mkdir src
COPY src/ src/

RUN cargo build --release

RUN cp target/release/graphql ./graphql-app
RUN chmod +x ./graphql-app

RUN rm -fr target/
RUN rm -fr src/
RUN rm -fr migrations/
RUN rm -f Cargo.*
RUN rm -f *.toml

CMD ["./graphql-app", "&"]