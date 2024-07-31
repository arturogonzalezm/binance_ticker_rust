FROM postgres:latest

# Example of installing a PostgreSQL extension
RUN apt-get update && apt-get install -y postgresql-contrib

# Copy custom configuration file (if you have one)
# COPY custom-config.conf /etc/postgresql/postgresql.conf

# You can also add a custom initialization script
# COPY init-db.sh /docker-entrypoint-initdb.d/

# Expose PostgreSQL port
EXPOSE 5432
