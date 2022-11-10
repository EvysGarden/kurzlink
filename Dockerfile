FROM ubuntu
RUN apt update
# We should change that
COPY ./target/debug/kurzlink ./bin