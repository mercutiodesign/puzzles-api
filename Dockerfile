FROM python:3.12.0-slim-bookworm as build-stage

RUN set -x \
  && apt-get update \
  && apt-get install -y --no-install-recommends cmake gcc build-essential \
  && rm -rf /var/lib/apt/lists/*

RUN mkdir /puzzles
WORKDIR /puzzles
COPY ./puzzles/ .

# build instructions based on puzzles PKGBUILD from arch
# hadolint ignore=SC2016,SC2035
RUN set -x \
  && cmake . \
  && cmake --build . --target loopygenerator \
  && strip loopygenerator \
  && mv loopygenerator .. \
  && rm -rf ./*

FROM python:3.12.0-slim-bookworm

RUN mkdir /code
WORKDIR /code

COPY requirements.txt .
RUN --mount=type=cache,target=/root/.cache pip install --disable-pip-version-check -r requirements.txt
COPY src .

COPY --from=build-stage /loopygenerator /usr/bin

ENTRYPOINT [ "./main.py" ]
