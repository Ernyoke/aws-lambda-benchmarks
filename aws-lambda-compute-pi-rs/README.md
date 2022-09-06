# aws-lambda-compute-pi-rs

## Build Docker Image

```
docker build --tag rst --progress=plain .
```

## Build Docker Image (buildx)

```
docker buildx build --progress=plain --platform linux/arm64 -t rust-arm64 -f Dockerfile-arm64 .
```