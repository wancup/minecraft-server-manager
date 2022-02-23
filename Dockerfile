FROM public.ecr.aws/lambda/provided:al2-arm64

ARG RUST_VERSION=1.58.1
RUN yum install -y gcc zip

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain $RUST_VERSION
ENV PATH $PATH:~/.cargo/bin

COPY build-api.sh /usr/local/bin/
VOLUME ["/code"]
WORKDIR /code
ENTRYPOINT ["/bin/bash", "/usr/local/bin/build-api.sh"]
