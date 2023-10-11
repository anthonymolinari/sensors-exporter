# sensors-exporter

![Build](https://github.com/anthonymolinari/sensors-exporter/actions/workflows/build.yml/badge.svg?branch=main)

[![Docker Pulls](https://img.shields.io/docker/pulls/anthonymolinari/sensors-exporter)](https://hub.docker.com/r/anthonymlinari/sensors-exporter)

#### Run with docker
```bash
docker run -d \
    --name sensors-exporter \
    -p 8282:8282 \
    anthonymolinari/sensors-exporter:latest
```

prometheus scrape job:
```yaml
- job_name: 'sensors-exporter'
    metrics_path: /metrics
    static_configs:
      - targets: ['sensors-exporter:8282']
```

#### Build from source:

Install dependencies
```bash
sudo apt isntall librust-clang-sys-dev \
    lm-sensors \
    librust-libsensors-sys-dev \
    libsensors-dev \
```
Clone repo 
```bash
git clone https://github.com/anthonymolinari/sensors-exporter
```
Compile & run
```bash
cd sensors-exporter && cargo build --release 
./targets/release/sensors-exporter
```
