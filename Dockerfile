# Cargo build stage

FROM rust:latest as cargo-build

WORKDIR /usr/src/Oasis
RUN apt-get update && apt-get upgrade -y && apt-get install -y build-essential
RUN apt-get install openssl libssl-dev -y && apt-get install clang llvm-dev libclang-dev -y
COPY Cargo.toml Cargo.lock ./
#COPY data ./data
COPY migrations ./migrations
COPY src ./src
ENV OPENSSO_INCLUDE_DIR="/usr/include/openssl"

COPY . .

RUN cargo build --release

RUN cargo install --path .

# Sql setup------------------------

RUN apt-get update && apt-get install -y && apt-get install postgresql -y && apt-get install libpq-dev -y

# Run the rest of the commands as the ``postgres`` user created by the ``postgres-9.3`` package when it was ``apt-get installed``
USER postgres

# Create a PostgreSQL role named ``docker`` with ``docker`` as the password and
# then create a database `docker` owned by the ``docker`` role.
# Note: here we use ``&&\`` to run commands one after the other - the ``\``
#       allows the RUN command to span multiple lines.
RUN    /etc/init.d/postgresql start &&\
    psql --command "CREATE USER docker WITH SUPERUSER PASSWORD 'docker';" &&\
    createdb -O docker docker

# Adjust PostgreSQL configuration so that remote connections to the
# database are possible.
RUN echo "host all  all    0.0.0.0/0  md5" >> /etc/postgresql/13/main/pg_hba.conf

# And add ``listen_addresses`` to ``/etc/postgresql/9.3/main/postgresql.conf``
RUN echo "listen_addresses='*'" >> /etc/postgresql/13/main/postgresql.conf

# Expose the PostgreSQL port
EXPOSE 5432

# Add VOLUMEs to allow backup of config, logs and databases
VOLUME  ["/etc/postgresql", "/var/log/postgresql", "/var/lib/postgresql"]

# Set the default command to run when starting the container
CMD ["/usr/lib/postgresql/13/bin/postgres", "-D", "/var/lib/postgresql/13/main", "-c", "config_file=/etc/postgresql/13/main/postgresql.conf"]

# Final stage-------------------------------

FROM alpine:latest

COPY --from=cargo-build /usr/local/cargo/bin/Oasis /usr/local/bin/Oasis

CMD ["/usr/local/cargo/bin/Oasis"]