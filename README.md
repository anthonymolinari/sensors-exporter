# sensors-exporter


#### Build from source:

Install dependencies
```
sudo apt isntall librust-clang-sys-dev \
    lm-sensors \
    librust-libsensors-sys-dev \
    libsensors-dev \
```
Clone repo 
```
git clone https://github.com/anthonymolinari/sensors-exporter
```
Compile
```
cd sensors-exporter && cargo build --release

sudo cp ./targets/release/sensors-exporter /etc/bin/sensors-exporter

```
