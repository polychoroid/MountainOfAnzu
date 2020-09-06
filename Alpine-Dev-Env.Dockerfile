FROM alpine:latest
RUN apk update \
    && apk add sudo openssh-client git nodejs nodejs-npm curl gcc zlib libc-dev openssl openssl-dev pkgconfig \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && source $HOME/.cargo/env \
    && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh \
    && adduser -h /home/remote -s /bin/ash -G users -G wheel -u 1010 remote

USER remote
WORKDIR /home/remote

ENV PATH=/root/.cargo/bin:${PATH}

EXPOSE 8088

