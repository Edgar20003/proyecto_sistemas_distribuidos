FROM rust:1.75-slim-bookworm
RUN apt-get update && apt-get install -y iputils-ping iproute2 curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
CMD ["tail", "-f", "/dev/null"]
