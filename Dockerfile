FROM alpine:latest
EXPOSE 80
ENV APP_DIR=/opt/app
RUN mkdir -p $APP_DIR
# Copy outputs
COPY ./target/x86_64-unknown-linux-musl/release/celerserver $APP_DIR/celerserver
COPY ./docs/src/.vitepress/dist $APP_DIR/docs
COPY ./web-client/dist $APP_DIR/app

WORKDIR $APP_DIR

ENV CELERSERVER_LOG=INFO
ENV CELERSERVER_ANSI=0
ENV CELERSERVER_PORT=80
ENV CELERSERVER_DOCS_DIR=/opt/app/docs
ENV CELERSERVER_APP_DIR=/opt/app/app

CMD ["./celerserver"]
