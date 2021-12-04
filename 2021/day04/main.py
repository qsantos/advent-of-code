from copy import deepcopy
from typing import List, Tuple

Board = List[List[int]]


def load_data(filename: str) -> Tuple[List[int], List[Board]]:
    boards = []
    with open(filename) as f:
        line = next(f)
        numbers = [int(x) for x in line.strip().split(',')]
        for _ in f:  # skips empty line
            boards.append([
                [int(x) for x in next(f).strip().split()]
                for _ in range(5)
            ])
    return numbers, boards


def board_wins(board: Board) -> bool:
    for i in range(5):
        if all(board[i][j] < 0 for j in range(5)):
            return True
    for j in range(5):
        if all(board[i][j] < 0 for i in range(5)):
            return True
    return False


def board_score(board: Board) -> int:
    return sum(
        board[i][j]
        for i in range(5)
        for j in range(5)
        if board[i][j] >= 0
    )


def first_winning_board(numbers: List[int], boards: List[Board]) -> Tuple[int, int]:
    boards = deepcopy(boards)
    for number in numbers:
        for board in boards:
            for i in range(5):
                for j in range(5):
                    if board[i][j] == number:
                        board[i][j] = -1
            if board_wins(board):
                return board_score(board), number
    assert False


def last_winning_board(numbers: List[int], boards: List[Board]) -> Tuple[int, int]:
    boards = deepcopy(boards)
    last_score = None
    last_number = None
    for number in numbers:
        remaining_boards = []
        for board in boards:
            for i in range(5):
                for j in range(5):
                    if board[i][j] == number:
                        board[i][j] = -1
            if board_wins(board):
                last_score = board_score(board)
                last_number = number
            else:
                remaining_boards.append(board)
        boards = remaining_boards
    assert last_score is not None
    assert last_number is not None
    return last_score, last_number


assert first_winning_board(*load_data('example')) == (188, 24)
assert first_winning_board(*load_data('input')) == (1137, 5)

assert last_winning_board(*load_data('example')) == (148, 13)
assert last_winning_board(*load_data('input')) == (430, 49)
