#!/usr/bin/python3

import random
import sys

HALTBAR = 1
ABGELAUFEN = 2

def draw() -> [int]:
    result = [0,0,0,0,0,0]

    for i in range(6):
        r = random.random()
        if r < 0.1:
            result[i] = ABGELAUFEN
        else:
            result[i] = HALTBAR

    return result


def mc_sim(n: int) -> int:
    num_cases_of_interest = 0
    for i in range(n):
        exp = draw()
        num_abgelaufen = 0
        num_haltbar = 0
        for x in exp:
            if x == ABGELAUFEN:
                num_abgelaufen += 1
            if x == HALTBAR:
                num_haltbar += 1
        if num_haltbar == 6:
            num_cases_of_interest += 1
#        print (exp, "abgelaufen: ", num_abgelaufen, "haltbar: ", num_haltbar)
    return num_cases_of_interest



if __name__ == "__main__":
    num_experiments = int(sys.argv[1])
    cases = mc_sim(num_experiments)
    print(cases, num_experiments-cases, num_experiments, float(cases)/float(num_experiments))


