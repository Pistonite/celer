FROM alpine:latest
EXPOSE 80
ENV APP_DIR=/opt/app
COPY ./dist $APP_DIR
RUN chmod +x $APP_DIR/bin/celery
RUN chmod +x $APP_DIR/bin/celery-boot

WORKDIR $APP_DIR

ENV CELERSERVER_LOG=INFO \
    CELERSERVER_PORT=80 \
    CELERSERVER_DOCS_DIR=/opt/app/docs \
    CELERSERVER_APP_DIR=/opt/app/app \
    CELERSERVER_GZIP=true

CMD ["./bin/celery-boot"]
