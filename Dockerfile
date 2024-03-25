FROM  --platform=linux/386 quay.io/pypa/manylinux2014_i686
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-host i686-unknown-linux-gnu --default-toolchain nightly -y
RUN source /root/.cargo/env && rustup toolchain install nightly --allow-downgrade --profile minimal --component clippy
CMD source /root/.cargo/env && cargo build --release --target i686-unknown-linux-gnu