#!/bin/bash
set -e -x

for PYBIN in /opt/python/cp3[891]*/bin; do
    "${PYBIN}/pip" install cffi maturin
    "${PYBIN}/maturin" build -i "${PYBIN}/python" --release
done

for wheel in target/wheels/*.whl; do
    auditwheel repair "${wheel}"
done
