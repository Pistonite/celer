FROM alpine:latest
EXPOSE 80
ENV APP_DIR=/opt/app 
RUN mkdir -p $APP_DIR
# Copy outputs
COPY ./dist $APP_DIR

WORKDIR $APP_DIR

CMD ["./start-server", "--port", "80"]
