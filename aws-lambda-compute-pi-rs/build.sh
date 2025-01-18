#!/bin/bash

set -x;

function generate_remote_tag() {
    TAG="$AWS_ACCOUNT.dkr.ecr.us-east-1.amazonaws.com/compute-pi-rst-$1"
}

# IMAGE_NAME="chainguard-glibc-x86"
# docker buildx build --provenance=false --progress=plain --platform linux/x86_64 -t $IMAGE_NAME -f Dockerfile-chainguard-glibc-x86 .
# generate_remote_tag $IMAGE_NAME
# docker tag $IMAGE_NAME $TAG
# docker push $TAG

# IMAGE_NAME="chainguard-musl-x86"
# docker buildx build --provenance=false --progress=plain --platform linux/x86_64 -t $IMAGE_NAME -f Dockerfile-chainguard-musl-x86 .
# generate_remote_tag $IMAGE_NAME
# docker tag $IMAGE_NAME $TAG
# docker push $TAG

IMAGE_NAME="chainguard-glibc-arm64"
docker buildx build --provenance=false --progress=plain --platform linux/arm64 -t $IMAGE_NAME -f Dockerfile-chainguard-glibc-arm64 .
generate_remote_tag $IMAGE_NAME
docker tag $IMAGE_NAME $TAG
docker push $TAG

IMAGE_NAME="chainguard-musl-arm64"
docker buildx build --provenance=false --progress=plain --platform linux/arm64 -t $IMAGE_NAME -f Dockerfile-chainguard-musl-arm64 .
generate_remote_tag $IMAGE_NAME
docker tag $IMAGE_NAME $TAG
docker push $TAG

IMAGE_NAME="distroless-glibc-arm64"
docker buildx build --provenance=false --progress=plain --platform linux/arm64 -t distroless-glibc-arm64 -f Dockerfile-distroless-glibc-arm64 .
generate_remote_tag $IMAGE_NAME
docker tag $IMAGE_NAME $TAG
docker push $TAG

IMAGE_NAME="distroless-musl-arm64"
docker buildx build --provenance=false --progress=plain --platform linux/arm64 -t distroless-musl-arm64 -f Dockerfile-distroless-musl-arm64 .
generate_remote_tag $IMAGE_NAME
docker tag $IMAGE_NAME $TAG
docker push $TAG

IMAGE_NAME="distroless-glibc-x86"
docker buildx build --provenance=false --progress=plain --platform linux/x86_64 -t distroless-glibc-x86 -f Dockerfile-distroless-glibc-x86 .
generate_remote_tag $IMAGE_NAME
docker tag $IMAGE_NAME $TAG
docker push $TAG

IMAGE_NAME="distroless-musl-x86"
docker buildx build --provenance=false --progress=plain --platform linux/x86_64 -t distroless-musl-x86 -f Dockerfile-distroless-musl-x86 .
generate_remote_tag $IMAGE_NAME
docker tag $IMAGE_NAME $TAG
docker push $TAG

IMAGE_NAME="scratch-arm64"
docker buildx build --provenance=false --progress=plain --platform linux/arm64 -t scratch-arm64 -f Dockerfile-scratch-arm64 .
generate_remote_tag $IMAGE_NAME
docker tag $IMAGE_NAME $TAG
docker push $TAG

IMAGE_NAME="scratch-x86"
docker buildx build --provenance=false --progress=plain --platform linux/x86_64 -t scratch-x86 -f Dockerfile-scratch-x86 .
generate_remote_tag $IMAGE_NAME
docker tag $IMAGE_NAME $TAG
docker push $TAG

IMAGE_NAME="aws-arm64"
docker buildx build --provenance=false --progress=plain --platform linux/arm64 -t aws-arm64 -f Dockerfile-arm64 .
generate_remote_tag $IMAGE_NAME
docker tag $IMAGE_NAME $TAG
docker push $TAG

IMAGE_NAME="aws-x86"
docker buildx build --provenance=false --progress=plain --platform linux/x86_64 -t aws-x86 -f Dockerfile-x86 .
generate_remote_tag $IMAGE_NAME
docker tag $IMAGE_NAME $TAG
docker push $TAG