FROM ubuntu:19.10

# install libqp-dev
RUN apt-get update && apt-get -y -qq install postgresql-client

# set app home dir
ENV APP_HOME=/home/flies/auth

# Create a group and user
RUN useradd -rm -d /home/flies -s /bin/bash -g root -G sudo -u 1000 flies
USER flies

# Create app folder
RUN mkdir -p /home/flies/auth
# change workdir
WORKDIR $APP_HOME

# Copy our binary into root
ADD .deploy/flies-auth ${APP_HOME}

# RUN chown -R flies:flies ${APP_HOME}

# APP Environments
ENV RUST_LOG="actix_web=info,actix_server=info"
ENV FA_DATABASE_URL="postgres://username:password@hostname:port/database"
ENV FA_PORT=8088
EXPOSE ${FA_PORT}

CMD ["/home/flies/auth/flies-auth"]
