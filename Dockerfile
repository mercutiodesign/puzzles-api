FROM python:3.10.2-slim-buster as build-stage

RUN set -x \
  && apt-get update \
  && apt-get install -y --no-install-recommends pkg-config make gcc build-essential \
  && rm -rf /var/lib/apt/lists/*

RUN mkdir /puzzles
WORKDIR /puzzles
COPY ./puzzles/ .

# build instructions based on puzzles PKGBUILD from arch
# hadolint ignore=SC2016,SC2035
RUN set -x \
  && ./mkfiles.pl \
  && sed -i 's|\$(gamesdir)|\$(bindir)|' Makefile.gtk \
  && sed -i 's_-Werror __' Makefile.gtk \
  && sed -i 's_CFLAGS := -O2_CFLAGS := -O2 -Wno-variadic-macros -Wno-long-long_g' Makefile.gtk \
  && make -f Makefile.gtk GTK_CONFIG=true loopygenerator \
  && strip loopygenerator \
  && mv loopygenerator .. \
  && rm -rf ./*

FROM python:3.10.2-slim-buster

RUN mkdir /code
WORKDIR /code

COPY requirements.txt .
RUN pip install --disable-pip-version-check --no-cache-dir -r requirements.txt
COPY src .

COPY --from=build-stage /loopygenerator /usr/bin

ENTRYPOINT [ "./main.py" ]
