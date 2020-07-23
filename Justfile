all: build strip deploy

build:
    cross build --target armv7-unknown-linux-gnueabihf

build_release:
    cross build --release --target armv7-unknown-linux-gnueabihf

strip:
    arm-none-eabi-strip target/armv7-unknown-linux-gnueabihf/debug/lctr

strip_release:
    arm-none-eabi-strip target/armv7-unknown-linux-gnueabihf/release/lctr

deploy:
    scp target/armv7-unknown-linux-gnueabihf/debug/lctr orangepizero@orangepizero:bin/

deploy_release:
    scp target/armv7-unknown-linux-gnueabihf/release/lctr orangepizero@orangepizero:bin/
