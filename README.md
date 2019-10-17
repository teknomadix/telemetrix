
Make sure `gpsd` is running with a detected device.

    cargo run

Snap can be built on Raspberry Pi arm64 environment with

    snapcraft --destructive-mode

Install built snap artifact with

    sudo snap install --devmode --dangerous <build artifact>.snap
