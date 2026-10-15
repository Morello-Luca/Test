FROM nvidia/cuda:12.8.1-devel-ubuntu24.04
RUN apt-get update && apt-get install -y --no-install-recommends build-essential cmake ninja-build git git-lfs gdb pkg-config ca-certificates curl && rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get install -y --no-install-recommends qt6-base-dev qt6-base-dev-tools qt6-tools-dev-tools libgl1 libxkbcommon-x11-0 && rm -rf /var/lib/apt/lists/*

ARG USERNAME=ubuntu

RUN apt-get update \
    && apt-get install -y --no-install-recommends sudo \
    && echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/${USERNAME} \
    && chmod 0440 /etc/sudoers.d/${USERNAME} \
    && rm -rf /var/lib/apt/lists/*

USER ubuntu
