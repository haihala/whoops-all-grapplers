#!/usr/bin/env python

import png

WIDTH = 2047
HEIGHT = WIDTH  # Easier to debug if it's a square
PERIOD = 25
STRIPE_POWER = 12.0
STRIPE_VALUE = 200.0
BG_VALUE = 0.0


def get_pixel(n):
    mod = n % PERIOD

    # 0-1, 0 at the edge of the period, 1 in the middle
    norm = 1 - 2 * abs(mod / PERIOD - 0.5)

    sharpened = norm ** STRIPE_POWER

    # Lerp
    return int(sharpened * STRIPE_VALUE + (1-sharpened) * BG_VALUE)


img = [
    [
        get_pixel(w) for w in range(WIDTH)
    ]
    for h in range(HEIGHT)
]
with open('pinstripe.png', 'wb') as f:
    w = png.Writer(WIDTH, HEIGHT, greyscale=True)
    w.write(f, img)
