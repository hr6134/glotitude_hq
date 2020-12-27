FROM rust:1.48

RUN apt-get update && apt-get install -y supervisor telnet net-tools
RUN mkdir -p /var/log/supervisor
RUN mkdir -p /etc/ctchi

WORKDIR /usr/src/glotitude_hq
COPY . .
COPY conf/supervisord.conf /etc/supervisor/supervisord.conf
COPY conf/conf.txt /etc/ctchi/conf.txt

RUN rustup default nightly
RUN cargo install --path .

EXPOSE 8080

CMD ["/usr/bin/supervisord"]
