# syntax=docker/dockerfile:1.7

ARG UBUNTU_VERSION=24.04
FROM mcr.microsoft.com/devcontainers/base:ubuntu-${UBUNTU_VERSION}@sha256:d94c97dd9cacf183d0a6fd12a8e87b526e9e928307674ae9c94139139c0c6eae AS base

ARG DEBIAN_FRONTEND=noninteractive
ARG USERNAME=vscode
ARG NODE_MAJOR=22
ARG RUST_VERSION=1.93.1
ARG CARGO_NEXTEST_VERSION=0.9.131
ARG CARGO_NEXTEST_X86_64_LINUX_GNU_SHA256=8e38b16299864c9f597c9a1e2caf25b7e8b598ffc659ec014c2c735a9befd8fa
ARG CARGO_NEXTEST_AARCH64_LINUX_GNU_SHA256=ce3bc715344d51e6f0fbb944489e37d17e25d24dfafaf861cc37643cfe7de663
ARG CODEX_NPM_PACKAGE=@openai/codex
ARG CODEX_VERSION=0.115.0

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        git \
        gpg \
        libssl-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

FROM base AS node

ARG NODE_MAJOR=22
ARG CODEX_NPM_PACKAGE=@openai/codex
ARG CODEX_VERSION=0.115.0

RUN mkdir -p /etc/apt/keyrings \
    && curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key \
        | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg \
    && echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_${NODE_MAJOR}.x nodistro main" \
        > /etc/apt/sources.list.d/nodesource.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

RUN npm install --global "${CODEX_NPM_PACKAGE}@${CODEX_VERSION}" \
    && npm cache clean --force

FROM base AS rust

ARG USERNAME=vscode
ARG RUST_VERSION=1.93.1
ARG CARGO_NEXTEST_VERSION=0.9.131
ARG CARGO_NEXTEST_X86_64_LINUX_GNU_SHA256=8e38b16299864c9f597c9a1e2caf25b7e8b598ffc659ec014c2c735a9befd8fa
ARG CARGO_NEXTEST_AARCH64_LINUX_GNU_SHA256=ce3bc715344d51e6f0fbb944489e37d17e25d24dfafaf861cc37643cfe7de663
ARG TARGETARCH

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

USER ${USERNAME}

ENV CARGO_HOME=/home/${USERNAME}/.cargo
ENV RUSTUP_HOME=/home/${USERNAME}/.rustup
ENV PATH=/home/${USERNAME}/.cargo/bin:${PATH}

RUN curl https://sh.rustup.rs -sSf \
        | sh -s -- -y --profile minimal --default-toolchain ${RUST_VERSION} \
    && rustup component add clippy rustfmt

RUN set -eu \
    && case "${TARGETARCH}" in \
        amd64) \
          nextest_target="x86_64-unknown-linux-gnu"; \
          nextest_sha256="${CARGO_NEXTEST_X86_64_LINUX_GNU_SHA256}"; \
          ;; \
        arm64) \
          nextest_target="aarch64-unknown-linux-gnu"; \
          nextest_sha256="${CARGO_NEXTEST_AARCH64_LINUX_GNU_SHA256}"; \
          ;; \
        *) \
          echo "error: unsupported TARGETARCH '${TARGETARCH}' for cargo-nextest" >&2; \
          exit 1; \
          ;; \
      esac \
    && nextest_archive="cargo-nextest-${CARGO_NEXTEST_VERSION}-${nextest_target}.tar.gz" \
    && nextest_url="https://github.com/nextest-rs/nextest/releases/download/cargo-nextest-${CARGO_NEXTEST_VERSION}/${nextest_archive}" \
    && tmpdir="$(mktemp -d)" \
    && trap 'rm -rf "${tmpdir}"' EXIT \
    && mkdir -p "${CARGO_HOME}/bin" \
    && curl -fsSL "${nextest_url}" -o "${tmpdir}/${nextest_archive}" \
    && echo "${nextest_sha256}  ${tmpdir}/${nextest_archive}" | sha256sum -c - \
    && tar -xzf "${tmpdir}/${nextest_archive}" -C "${tmpdir}" cargo-nextest \
    && install -m 0755 "${tmpdir}/cargo-nextest" "${CARGO_HOME}/bin/cargo-nextest"

FROM node

ARG USERNAME=vscode

COPY --from=rust /home/${USERNAME}/.cargo /home/${USERNAME}/.cargo
COPY --from=rust /home/${USERNAME}/.rustup /home/${USERNAME}/.rustup

USER ${USERNAME}

ENV CARGO_HOME=/home/${USERNAME}/.cargo
ENV RUSTUP_HOME=/home/${USERNAME}/.rustup
ENV PATH=/home/${USERNAME}/.cargo/bin:${PATH}

WORKDIR /workspaces/nosqo
