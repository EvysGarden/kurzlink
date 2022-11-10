FROM debian
RUN apt update
RUN apt install openssl
COPY ./target/debug/kurzlink ./bin

