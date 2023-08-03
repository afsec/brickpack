FROM alpine:latest

COPY ./app/brickpack-2022 /app/

WORKDIR /app

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=30s \
    CMD wget --no-verbose --tries=1 http://localhost:8000/health -O - >/dev/null 2>&1 || exit 1

# APP CONFIGURATION - BEGIN
ENV IPV4_ADDRESS=0.0.0.0
# APP CONFIGURATION - END

ENTRYPOINT ["./brickpack-2022"]

# HELP

# Build image
# docker build -t afsec/brickpack .

# Run container
# docker run --rm --name brickpack -p8000:8000 -d afsec/brickpack

# Show API health check information
# docker inspect --format='{{json .State.Health}}' brickpack | jq ."Status"

# Show running containers
# docker ps

# Show application logs
# docker logs -f brickpack

# Stop Container
# docker kill brickpack

# Delete image
# docker rmi afsec/brickpack
