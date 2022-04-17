#!/bin/sh

NODE=node:17.9.0-alpine3.15
REGISTRY=registry:2.8.1
RUST_BUILDER=ekidd/rust-musl-builder:latest
MONGO=mongo:5.0.7-focal

for i in  "$NODE node" "$REGISTRY registry" "$RUST_BUILDER rust_builder" "$MONGO mongo" ; do
    a=( $i );
    docker pull "${a[0]}"
    docker save ${a[0]} > ${a[1]}.tar
    xz -T32 -9 ${a[1]}.tar
done



