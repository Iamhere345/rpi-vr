# rpi-vr
collection of software for operating a DIY raspberry pi based VR headset

## How to run
 - connect to raspberry pi via vnc and start the python script in `rpi/`
 - on host pc do `cargo run` in `pc/`
 - open obs and click on `start virtual camera`
 - run the python script in `video/`
 - open a web browser on the phone, type in the computers ip addr on port `3000` (ip and port should be shown in the output of `web.py`)
 - slot the phone into the headset

Also don't forget to setup a controller to use.
