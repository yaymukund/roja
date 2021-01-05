FROM cimg/rust:1.49
USER root
RUN apt-get update \
  && apt-get install -y \
    libmpv-dev \
    libsqlite3-dev
USER circleci
