FROM python:3.8.2-slim-buster

RUN set -x \
  && apt-get update \
  && apt-get install -y --no-install-recommends pkg-config \
  && rm -rf /var/lib/apt/lists/*
