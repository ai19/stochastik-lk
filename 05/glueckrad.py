#!/usr/bin/python3

import random

def get_draw():
    draw = [0,0,0]
    for i in range(3):
        r = random.random()
        #print(i, r)
        if r<0.25:
            draw[i] = 1
        if 0.25<=r and r<0.5:
            draw[i] = 2
        if 0.5<=r and r<0.75:
            draw[i] = 3
        if 0.75<=r:
            draw[i] = 4
    return draw


def eval_draw(draw) -> bool:
    result = draw[0] == draw[2] and draw[0] != draw[1]
    return result


hit_counter = 0

runs = 1000000

for i in range(runs):
    draw = get_draw()
    print(draw, eval_draw(draw))
    if eval_draw(draw):
        hit_counter += 1

hit_ratio = float(hit_counter)/runs

print(hit_counter, hit_ratio, 3./16., hit_ratio-3./16., hit_ratio-3./8.)

