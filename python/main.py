#!/usr/bin/env python3

ADD, REMOVE, SUBSTITUTE, IGNORE = "A", "R", "S", "I"
TRACE_CACHE = False

if TRACE_CACHE:
    def TraceCache(distance_cache, actions_cache):
        PrintCache(distance_cache, actions_cache)
else:
    def TraceCache(*args):
        pass


def PrintCache(distance_cache, actions_cache):
    n1, n2 = len(distance_cache), len(distance_cache[0])

    for i in range(n1):
        out_str = ""
        for j in range(n2):
            if distance_cache[i][j] == None: out_str += " - "
            else: out_str += f"{distance_cache[i][j]:>4}"

            if actions_cache[i][j] == None: out_str += "(-)"
            else: out_str += f"({actions_cache[i][j]})"

        print(out_str)
    print("")

def TraceActions(source, destination, actions_cache):
    path = []
    n1, n2 = len(source), len(destination)

    while n1 > 0 or n2 > 0:
        action = actions_cache[n1][n2]
        if action == IGNORE:
            n1, n2 = n1-1, n2-1
            path = [(action, source[n1])] + path
        elif action == ADD:
            n2 -= 1
            path = [(action, destination[n2])] + path
        elif action == REMOVE:
            n1 -= 1
            path = [(action, source[n1])] + path
        elif action == SUBSTITUTE:
            n1, n2 = n1-1, n2-1
            path = [(action, source[n1], destination[n2])] + path
        else:
            n1, n2 = -1, -1
            return "UNTRACEABLE"

    return path

# Bottom-Up + TraceCache + BackTrack
def Levenshtein(source, destination):
    distance_cache = [[None for i in range(len(destination) + 1)] for j in range(len(source) + 1)]
    actions_cache = [[None for i in range(len(destination) + 1)] for j in range(len(source) + 1)]

    distance_cache[0][0] = 0
    actions_cache[0][0] = IGNORE

    for n1 in range(1, len(source) + 1): 
        distance_cache[n1][0] = n1
        actions_cache[n1][0] = REMOVE
        TraceCache(distance_cache, actions_cache)

    for n2 in range(1, len(destination) + 1): 
        distance_cache[0][n2] = n2
        actions_cache[0][n2] = ADD
        TraceCache(distance_cache, actions_cache)

    for n1 in range(1, len(source) + 1):
        for n2 in range(1, len(destination) + 1):
            if source[n1 - 1] == destination[n2 - 1]:
                distance_cache[n1][n2] = distance_cache[n1 - 1][n2 - 1]
                actions_cache[n1][n2] = IGNORE
                TraceCache(distance_cache, actions_cache)
                continue

            add_char = 1 + distance_cache[n1][n2 - 1]
            remove_char = 1 + distance_cache[n1 - 1][n2]
            substitute_char = 1 + distance_cache[n1 - 1][n2 - 1]

            distance_cache[n1][n2] = min(add_char, remove_char, substitute_char)
            
            if distance_cache[n1][n2] == add_char: 
                actions_cache[n1][n2] = ADD
            elif distance_cache[n1][n2] == remove_char: 
                actions_cache[n1][n2] = REMOVE
            else: 
                actions_cache[n1][n2] = SUBSTITUTE

            TraceCache(distance_cache, actions_cache)

    PrintCache(distance_cache, actions_cache)

    trace = TraceActions(source, destination, actions_cache)
    print(trace)

    return distance_cache[len(source)][len(destination)]

def test(string1, string2, ans = -1):
    res = Levenshtein(string1, string2)

    print("Source:".ljust(24, ' '), string1)
    print("Destination:".ljust(24, ' '), string2)
    print("Levenshtein Distance:".ljust(24, ' '), res)
    print("")

    if (ans != -1): assert res == ans

test("add", "add", 0)
test("add", "dady", 2)
test("add", "a", 2)
test("add", "dab", 3)
test("hello", "helio", 1)
test("d", "daddy", 4)
test("daddy", "d", 4)
test("", "daddy", 5)
test("daddy", "", 5)
test("sdvsdvavdsvadsvasdvasvasdvsavdsvasdvasdvsdvavd", "vdasvsdvasdvdvasdvsavsvavasdvdvadv")

test("crates", "conjugate")

