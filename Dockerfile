FROM rust:1

RUN mkdir mnist_dir && \
    cd mnist_dir && \
    for file in train-images-idx3-ubyte.gz \
        train-labels-idx1-ubyte.gz \
        t10k-images-idx3-ubyte.gz \
        t10k-labels-idx1-ubyte.gz; do \
        curl https://storage.googleapis.com/cvdf-datasets/mnist/$file >$file && \
        gunzip $file; \
    done

ADD . /code
RUN cd /code && \
    cargo build --release && \
    mv target/release/brute-tree-server /usr/bin && \
    mv target/release/brute-tree-worker /usr/bin && \
    rm -r /code

EXPOSE 80

CMD ["brute-tree-server", "--port", "80"]
