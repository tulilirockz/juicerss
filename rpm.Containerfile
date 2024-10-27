ARG FEDORA_MAJOR_VERSION="${FEDORA_MAJOR_VERSION:-latest}"
ARG REGISTRY=registry.fedoraproject.org/fedora
FROM ${REGISTRY}:${FEDORA_MAJOR_VERSION} AS builder

ADD . /app
WORKDIR /app 
VOLUME ["/output"]

RUN dnf install --disablerepo='*' --enablerepo='fedora,updates' --setopt install_weak_deps=0 --nodocs --assumeyes 'dnf-command(builddep)' rpkg rpm-build
    
RUN mkdir -p /output/rpms && \
    rpkg spec --outdir /output && \
    dnf builddep -y /output/juicerss.spec && \
    rpkg local --outdir /output
