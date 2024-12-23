#!/usr/bin/env python

import png
import perlin_noise

SIDE_LENGHT = 255

noise_generator = perlin_noise.PerlinNoise()

SMALL_SCALE = 80 / SIDE_LENGHT
SMALL_BASE_COLOR = [180, 138, 120]
SMALL_TEXTURE_COLOR = [150, 130, 100]


BIG_SCALE = 20 / SIDE_LENGHT
BIG_INFLUENCE = 0.7
BIG_POWER = 3.0
BIG_TEXTURE_COLOR = [50, 40, 30]


def get_pixel(x: int, y: int):
    small_noise = abs(noise_generator(
        [
            SMALL_SCALE * x,
            SMALL_SCALE * y,
        ],
    ))

    big_noise = (BIG_INFLUENCE * abs(noise_generator(
        [
            BIG_SCALE * x,
            BIG_SCALE * y,
        ],
    ))) ** BIG_POWER

    out = [
        int(
            big_noise * big_texture +
            (1-big_noise) * (
                # The small scale stuff
                small_noise * small_texture + (1-small_noise) * base
            )
        )
        for (big_texture, small_texture, base)
        in zip(BIG_TEXTURE_COLOR, SMALL_TEXTURE_COLOR, SMALL_BASE_COLOR)
    ]

    return out


pixels = [
    [
        val
        for x in range(SIDE_LENGHT)
        for val in get_pixel(x, y)
    ]
    for y in range(SIDE_LENGHT)
]

img = png.from_array(
    pixels,
    mode='RGB',
    info={"width": SIDE_LENGHT, "height": SIDE_LENGHT},
)
img.save('skin.png')
