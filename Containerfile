FROM docker.io/rust:slim AS build
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

###

FROM docker.io/debian:stable-slim AS deploy

RUN apt update && apt -y upgrade
RUN rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/myapp
COPY --from=build /usr/src/myapp/target/release/mmri-diun-receiver .

RUN mkdir -p templates
COPY ./templates ./templates

EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
CMD [ "./mmri-diun-receiver" ]
