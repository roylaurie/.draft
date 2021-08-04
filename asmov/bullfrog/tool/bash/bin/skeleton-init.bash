#!/bin/bash

mkdir -p \
    bash \
    bash/lib \
    bash/bin \
    bash/cfg \
    bash/module \
    config \
    doc \
    doc/design \
    json \
    json/cfg \
    json/schema \
    json/schema/cfg \
    files \
    files/_CUSTOM_ \
    test \
    tool

touch \
    .gitignore \
    package.json \
    LICENSE.txt \
    README.md \
    bash/bin/_CUSTOM_.bash \
    bash/lib/_CUSTOM_.lib.bash \
    bash/module/_CUSTOM_.module.bash \
    doc/DESIGN.md \
    json/cfg/namespace.cfg.json \
    json/schema/cfg/namespace.cfg.schema.json \
    tool/build.bash \
    tool/test.bash

touch \
    bash/.gitignore \
    bash/lib/.gitignore \
    bash/bin/.gitignore \
    bash/cfg/.gitignore \
    bash/module/.gitignore \
    config/.gitignore \
    doc/.gitignore \
    doc/design/.gitignore \
    json/.gitignore \
    json/cfg/.gitignore \
    json/schema/.gitignore \
    json/schema/cfg/.gitignore \
    files/.gitignore \
    test/.gitignore \
    tool/.gitignore
