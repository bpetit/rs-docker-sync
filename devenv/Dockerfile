FROM rustlang/rust:nightly

RUN set -ex;                                                                    \
    apt-get update;                                                             \
    apt-get install -q -y --no-install-recommends                               \
        apt-transport-https                                                     \
        sudo                                                                    \
        git                                                                     \
        vim                                                                     \
        curl                                                                    \
	build-essential								\
        libssl1.0-dev                                                           \
        ;                                                                       \
    apt-get autoremove -q -y;                                                   \
    apt-get autoclean -q -y
