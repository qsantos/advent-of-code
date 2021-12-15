from typing import List, Tuple


def read_hands(filename: str) -> Tuple[List[int], List[int]]:
    player1 = []
    player2 = []
    with open(filename) as f:
        next(f)  # "Player 1:"
        for line in f:
            if line == '\n':
                break
            player1.append(int(line.strip()))
        next(f)  # "Player 2:"
        for line in f:
            player2.append(int(line.strip()))
        return player1, player2


def hand_score(hand: List[int]) -> int:
    return sum(
        (len(hand) - i) * v
        for i, v in enumerate(hand)
    )


def play_combat(player1: List[int], player2: List[int], log: bool) -> int:
    player1 = list(player1)
    player2 = list(player2)
    round_number = 0
    while player1 and player2:
        if log:
            round_number += 1
            print(f'-- Round {round_number} --')
            print("Player 1's deck:", ', '.join(str(v) for v in player1))
            print("Player 2's deck:", ', '.join(str(v) for v in player2))
        a, b = player1.pop(0), player2.pop(0)
        if log:
            print('Player 1 plays:', a)
            print('Player 2 plays:', b)
        if a > b:
            if log:
                print('Player 1 wins the round!\n')
            player1 += [a, b]
        else:
            if log:
                print('Player 2 wins the round!\n')
            player2 += [b, a]
    if log:
        print()
        print('== Post-game results ==')
        print("Player 1's deck:", ', '.join(str(v) for v in player1))
        print("Player 2's deck:", ', '.join(str(v) for v in player2))
    return hand_score(player1 or player2)


def play_recursive_combat(player1: List[int], player2: List[int], log: bool) -> int:
    game_counter = 0

    def aux(player1: List[int], player2: List[int]) -> Tuple[bool, List[int], List[int]]:
        nonlocal game_counter
        game_counter += 1
        game_number = game_counter
        if log:
            print(f'=== Game {game_number} ===')
        seen = set()
        round_number = 0
        while player1 and player2:
            state = (tuple(player1), tuple(player2))
            if state in seen:
                return True, player1, player2
            seen.add(state)
            if log:
                round_number += 1
                print()
                print(f'-- Round {round_number} (Game {game_number}) --')
                print("Player 1's deck:", ', '.join(str(v) for v in player1))
                print("Player 2's deck:", ', '.join(str(v) for v in player2))
            a, b = player1.pop(0), player2.pop(0)
            if log:
                print('Player 1 plays:', a)
                print('Player 2 plays:', b)
            if len(player1) >= a and len(player2) >= b:
                if log:
                    print('Playing a sub-game to determine the winner...')
                    print()
                player1_wins, _, _ = aux(player1[:a], player2[:b])
                if log:
                    print()
                    print(f'...anyway, back to game {game_number}.')
            else:
                player1_wins = a > b
            if player1_wins:
                if log:
                    print(f'Player 1 wins round {round_number} of game {game_number}!')
                player1 += [a, b]
            else:
                if log:
                    print(f'Player 2 wins round {round_number} of game {game_number}!')
                player2 += [b, a]
        if player1:
            if log:
                print(f'The winner of game {game_number} is player 1!')
            return True, player1, player2
        else:
            if log:
                print(f'The winner of game {game_number} is player 2!')
            return False, player1, player2
    player1_wins, player1, player2 = aux(list(player1), list(player2))
    if log:
        print()
        print()
        print('== Post-game results ==')
        print("Player 1's deck:", ', '.join(str(v) for v in player1))
        print("Player 2's deck:", ', '.join(str(v) for v in player2))
    return hand_score(player1 or player2)


example = read_hands('example')
input = read_hands('input')

assert play_combat(*example, False) == 306
assert play_combat(*input, False) == 35818

assert play_recursive_combat(*example, False) == 291
assert play_recursive_combat(*input, False) == 34771
