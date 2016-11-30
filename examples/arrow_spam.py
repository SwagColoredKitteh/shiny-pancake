import math

WIDTH  = 640
HEIGHT = 480
COUNT  = 60
FRAMES = 400

print("#STROKE_COLOR", 255, 255, 255, 255)

stepsize = (HEIGHT - 40) / COUNT

for f in range(FRAMES):
    print("#FRAME_START")
    for i in range(COUNT):
        height = 20 + i * stepsize
        print("#STROKE_WIDTH", 1)
        print("#ARROW", WIDTH / 2
                      , height
                      , WIDTH / 2 + math.sin(((0.2 * i) / COUNT + f / COUNT) * math.pi * 2) * (WIDTH - 40) / 2
                      , height)
