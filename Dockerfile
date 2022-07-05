FROM rust:slim
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
CMD [ "mmri-diun-receiver" ]
