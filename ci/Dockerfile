ARG TARGET
ARG VERSION
FROM rustembedded/cross:${TARGET}-${VERSION}

RUN apt-get update && apt-get install -y libasound2-dev
