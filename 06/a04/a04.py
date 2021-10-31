#!/usr/bin/python3

import random
import sys

DICHT = 1
UNDICHT = 2

def draw() -> [int]:
    result = [0,0,0,0]

    for i in range(4):
        r = random.random()
        if r < 0.08:
            result[i] = UNDICHT
        else:
            result[i] = DICHT

    return result


def mc_sim(n: int) -> int:
    num_cases_of_interest = 0
    for i in range(n):
        exp = draw()
        num_dicht = 0
        num_undicht = 0
        for x in exp:
            if x == DICHT:
                num_dicht += 1
            if x == UNDICHT:
                num_undicht += 1
        if num_dicht == 4:
            num_cases_of_interest += 1
#        print (exp, "abgelaufen: ", num_abgelaufen, "haltbar: ", num_haltbar)
    return num_cases_of_interest



if __name__ == "__main__":
    num_experiments = int(sys.argv[1])
    cases = mc_sim(num_experiments)
    print(cases, num_experiments-cases, num_experiments, float(cases)/float(num_experiments))


