#!/usr/bin/python3

augenzahlen = [1,2,3,4,5,6]
#augenzahlen = [1,2]

guenstige_faelle = 0

def check(a,b,c,d):
#    if a == b and b== c and c==d:
#        return True
#    if a == b and b == c:
#        return True
#    if a == c and c == d:
#        return True
#    if a == b and b == d:
#        return True
#    if b==c and c==d:
#        return True
    if a == b and c == d:
        return True
    if a == c and b == d:
        return True
    if a == d and b == c:
        return True


for a in augenzahlen:
    for b in augenzahlen:
        for c in augenzahlen:
            for d in augenzahlen:
                if check(a,b,c,d):
                    guenstige_faelle += 1
                    string = ('\033[32m %d %d %d %d \033[0m')%(a,b,c,d)
                    #print(a,b,c,d)
                    #print(a,b,c,d)
                    print(string)
#               else:
#                   print(a,b,c,d)

print (guenstige_faelle)




