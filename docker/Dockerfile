FROM alpine

# Create app folder
RUN mkdir -p /home/flies/auth

# Create a group and user in alpine
RUN addgroup -S flies && adduser -S flies -G flies

# set home dir
ENV HOME=/home/flies
ENV APP_HOME=/home/flies/auth

# change workdir
WORKDIR $APP_HOME

# Copy our binary into root
ADD .deploy/flies-auth ${APP_HOME}

RUN chown -R flies:flies ${APP_HOME}

# Change to app user
USER flies

# Port
ENV FA_PORT=8088
EXPOSE ${FA_PORT}

CMD ["/home/flies/auth/flies-auth"]
