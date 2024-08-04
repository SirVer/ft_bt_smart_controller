# Program your Fischertechnik BT Smart Controller via MQTT

Upon gifting The Smart Robots Pro set to my kids, I was a bit dissapointed that
the only way of programming it was through the graphical tool provided by
Fischertechnik. I wanted to be able to also do it in Python, hence I spent a few
hours to make this possible. This repo is the result of this work.

The idea is to bridge the data from the bluetooth controller to MQTT, which in
turn can then be read by any programming language (including Python). This
allows for programming in any language, Python included.

## Quickstart

1. Install mosquitto and run it on localhost.
2. Build and run `ftbtc`. I might make releases at some point, for now you have
   to build it yourself.
   1. Install Rust as described at https://rustup.rs/
   2. cargo run --release 
3. Click the pairing button on the BT Controller.
4. Run the example [Python program](py/main.py). 


# Acknowledgments

This work is not supported or tied to Fischertechnik whatsoever, but I am very
grateful for their awesome products that give so much joy and teach so many
valuable skills.

As usual, this work is based on prior work to which I am very grateful. Very
useful was especially the work from Till harbaum (@harbaum) and I learned a lot from these links:

- https://github.com/harbaum/WebBTSmart/blob/master/control.html
- https://github.com/harbaum/RPI-BLE-toy-control

