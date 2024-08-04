#!/usr/bin/env python3

"""
This program is for the "getting started" build in the Smart Robotics Pro kit,
i.e. I1 and I2 have a microswitch attached and M1 and M2 have LEDs.

It then tests reaction time by turning the LED one and measure how long it
takes to tap the right button.
"""

import time
import random
from controller import BtSmartController

OPEN = 0xFFFF


def play(read_switch, write_led):
    time.sleep(random.randint(500, 3500) / 1000.0)
    if read_switch() != OPEN:
        print("Cheater. Let go of the button!")
        return
    start = time.perf_counter()
    write_led(-128)
    while read_switch() == OPEN:
        pass
    end = time.perf_counter()
    print(f"Reaction: {int((end - start) * 1000)}ms")
    write_led(0)


if __name__ == "__main__":
    c = BtSmartController()

    while True:
        play(c.read_l1, c.write_m1)
        play(c.read_l2, c.write_m2)
