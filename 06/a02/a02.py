#!/usr/bin/python3

import random
import sys


WHITE = 1
BLACK = 2

def draw() -> [int]:
    result = [0,0,0]
    num_white = 3
    num_black = 6

    for i in range(3):
        r = random.random()
        if r < float(num_white)/(num_white+num_black):
            result[i] = WHITE
            num_white -= 1
        else:
            result[i] = BLACK
            num_black -= 1

    
#    # draw 2, probabilities change
#    r = random.random()
#    if result[0] == WHITE:
#        if r < 2./8.:
#            result[1] = WHITE
#        else:
#            result[1] = BLACK
#    if result[0] == BLACK:
#        if r < 3./8.:
#            result[1] = WHITE
#        else:
#            result[1] = BLACK
    return result


def mc_sim(n: int) -> int:
    num_cases_of_interest = 0
    for i in range(n):
        exp = draw()
        if exp[0] == WHITE and exp[2] == WHITE:
            num_cases_of_interest += 1
    return num_cases_of_interest



if __name__ == "__main__":
    num_experiments = int(sys.argv[1])
    cases = mc_sim(num_experiments)
    print(cases, num_experiments, float(cases)/float(num_experiments))


