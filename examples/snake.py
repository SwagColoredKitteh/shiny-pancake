import math
import time
import random

ctr = 0
c1 = 1
c2 = 1
c1new = 1
c2new = 1

for i in range(2000):
    colorAngle = ctr * math.pi * 2 / 10
    offset = ctr * math.pi * 2 / 40
    t = (ctr / 50) % 1
    if ctr % 50 == 0:
        c1 = c1new
        c2 = c2new
        c1new = random.random() * 2
        c2new = random.random() * 2
    print("#FRAME_START")
    for i in range(24):
        angle = i / 24 * math.pi
        print("#COLOR", int(math.cos(colorAngle                       + angle) * 127 + 127)
                      , int(math.cos(colorAngle + math.pi * 2 / 3     + angle) * 127 + 127)
                      , int(math.cos(colorAngle + math.pi * 2 * 2 / 3 + angle) * 127 + 127)
                      , 255)
        print("#CIRCLE", 240 + math.cos(offset + angle * (c1 * (1 - t) + c1new * t)) * 180
                       , 240 + math.sin(offset + angle * (c2 * (1 - t) + c2new * t)) * 180, 60)
    time.sleep(33 / 1000)
    ctr += 1
