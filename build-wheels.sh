#!/bin/bash
set -ex

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly  -y
#curl https://www.python.org/ftp/python/3.7.3/Python-3.7.3.tgz | tar xz && \
#  cd Python-3.7.3 && ./configure --enable-optimizations --prefix=$HOME/.py37 && make altinstall && cd .. && \
#  rm -rf Python-3.7.3 && ln $HOME/.py37/bin/python3.7 $HOME/.py37/bin/python && ln $HOME/.py37/bin/pip3.7 $HOME/.py37/bin/pip

export PATH="$HOME/.cargo/bin:$PATH"

for PYBIN in /opt/rh/rh-python36/root/usr/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"
    sudo "${PYBIN}/pip" install -U pip setuptools wheel==0.31.1 setuptools-rust auditwheel
    "${PYBIN}/python" setup.py bdist_wheel
done

mkdir wheelhouse
export PATH="/opt/rh/rh-python36/root/usr/bin:$PATH"
for whl in dist/*.whl; do
    auditwheel repair "$whl" -w wheelhouse/
done
