FROM python:3.8.2-slim-buster as build-stage

RUN set -x \
  && apt-get update \
  && apt-get install -y --no-install-recommends pkg-config make gcc build-essential \
  && rm -rf /var/lib/apt/lists/*

RUN mkdir /puzzles
WORKDIR /puzzles
COPY ./puzzles/ .

# build instructions based on puzzles PKGBUILD from arch
RUN set -x \
  && ./mkfiles.pl \
  && sed -i 's|\$(gamesdir)|\$(bindir)|' Makefile.gtk \
  && sed -i 's_-Werror __' Makefile.gtk \
  && sed -i 's_CFLAGS := -O2_CFLAGS := -O2 -Wno-variadic-macros -Wno-long-long_g' Makefile.gtk \
  && make -f Makefile.gtk loopygenerator \
  && strip loopygenerator \
  && mv loopygenerator .. \
  && rm -rf *

FROM python:3.8.2-slim-buster

COPY --from=build-stage /loopygenerator /usr/bin

RUN loopygenerator 10x10
