from typing import Dict, List, Tuple

Dicts = Tuple[Dict[int, str], Dict[str, int]]


def read_moves(filename: str) -> List[str]:
    with open(filename) as f:
        return f.read().strip().split(',')


def repeat_moves(programs: str, moves: List[str], n_repeats: int) -> str:
    seen: Dict[str, int] = {}
    n = 0
    while n < n_repeats:
        if programs in seen:
            cycle = n - seen[programs]
            n_repeats = n + (n_repeats - n) % cycle
        seen[programs] = n
        programs = do_moves(programs, moves)
        n += 1
    return programs


def do_moves(program_string: str, moves: List[str]) -> str:
    programs = list(program_string)
    for move in moves:
        action, params = move[0], move[1:]
        if action == 's':
            n = int(params)
            programs = programs[-n:] + programs[:-n]
            pass
        elif action == 'x':
            a, b = (int(x) for x in params.split('/'))
            programs[a], programs[b] = programs[b], programs[a]
        elif action == 'p':
            pa, pb = params.split('/')
            a, b = programs.index(pa), programs.index(pb)
            programs[a], programs[b] = programs[b], programs[a]
        else:
            assert False, move
    return ''.join(programs)


def make_dicts() -> Dicts:
    programs = {}
    indices = {}
    for i, p in enumerate('abcdefghijklmnop'):
        programs[i] = p
        indices[p] = i
    return programs, indices


def main() -> None:
    input = read_moves('input')

    assert do_moves('abcde', ['s1']) == 'eabcd'
    assert do_moves('eabcd', ['x3/4']) == 'eabdc'
    assert do_moves('eabdc', ['pe/b']) == 'baedc'

    programs = 'abcdefghijklmnop'
    programs = do_moves(programs, input)
    assert programs == 'fnloekigdmpajchb'

    programs = 'abcdefghijklmnop'
    programs = repeat_moves(programs, input, n_repeats=1_000_000_000)
    assert programs == 'amkjepdhifolgncb'


if __name__ == '__main__':
    main()
