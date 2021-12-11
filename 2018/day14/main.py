def scores_after_n(n: int) -> str:
    scoreboard = [3, 7]
    pos1 = 0
    pos2 = 1
    while len(scoreboard) < n + 10:
        for d in str(scoreboard[pos1] + scoreboard[pos2]):
            scoreboard.append(int(d))
        pos1 = (pos1 + 1 + scoreboard[pos1]) % len(scoreboard)
        pos2 = (pos2 + 1 + scoreboard[pos2]) % len(scoreboard)
    return ''.join(str(d) for d in scoreboard[n:n + 10])


def n_for_scores(scores: str) -> int:
    scoreboard = [3, 7]
    current_scores = '37'
    pos1 = 0
    pos2 = 1
    n = 2
    while True:
        for d in str(scoreboard[pos1] + scoreboard[pos2]):
            current_scores += d
            if len(current_scores) > len(scores):
                current_scores = current_scores[1:]
                if current_scores == scores:
                    return len(scoreboard) - len(scores) + 1
            scoreboard.append(int(d))
        pos1 = (pos1 + 1 + scoreboard[pos1]) % len(scoreboard)
        pos2 = (pos2 + 1 + scoreboard[pos2]) % len(scoreboard)
        n += 1
    return ''.join(str(d) for d in scoreboard[n:n + 10])


assert scores_after_n(9) == '5158916779'
assert scores_after_n(5) == '0124515891'
assert scores_after_n(18) == '9251071085'
assert scores_after_n(2018) == '5941429882'
assert scores_after_n(920831) == '7121102535'

assert n_for_scores('51589') == 9
assert n_for_scores('01245') == 5
assert n_for_scores('92510') == 18
assert n_for_scores('59414') == 2018
assert n_for_scores('920831') == 20236441
