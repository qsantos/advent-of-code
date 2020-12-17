from typing import Dict, List, Tuple

Rules = Dict[str, List[Tuple[int, int]]]
Ticket = List[int]


def read_input() -> Tuple[Rules, Ticket, List[Ticket]]:
    with open('input') as f:
        data = f.read()
    str_rules, str_my_ticket, str_tickets = data.strip().split('\n\n')

    rules = {}
    for str_rule in str_rules.split('\n'):
        field, str_clauses = str_rule.split(': ')
        clauses = []
        for str_clause in str_clauses.split(' or '):
            lo, hi = [int(x) for x in str_clause.split('-')]
            clauses.append((lo, hi))
        rules[field] = clauses

    my_ticket = [int(x) for x in str_my_ticket.split('\n')[1].split(',')]

    tickets = [
        [int(x) for x in str_ticket.split(',')]
        for str_ticket in str_tickets.split('\n')[1:]
    ]

    return rules, my_ticket, tickets


def puzzle1(rules: Rules, tickets: List[Ticket]) -> None:
    print(sum(
        value
        for ticket in tickets
        for value in ticket
        if not any(
            lo <= value <= hi
            for clauses in rules.values()
            for lo, hi in clauses
        )
    ))


def find_index_of_field(rules: Rules, tickets: List[Ticket]) -> Dict[str, int]:
    valid_tickets = [
        ticket
        for ticket in tickets
        if all(
            any(
                lo <= value <= hi
                for clauses in rules.values()
                for lo, hi in clauses
            )
            for value in ticket
        )
    ]

    n_fields = len(rules)

    field_candidates = {}
    for i in range(n_fields):
        candidates = set()
        for field in rules:
            if all(
                any(
                    lo <= ticket[i] <= hi
                    for lo, hi in rules[field]
                )
                for ticket in valid_tickets
            ):
                candidates.add(field)
        field_candidates[i] = candidates

    changed = True
    while changed:
        changed = False
        for i in range(n_fields):
            candidates = field_candidates[i]
            if len(candidates) == 1:
                field, = candidates
                for j in range(n_fields):
                    if j != i and field in field_candidates[j]:
                        field_candidates[j].remove(field)
                        changed = True

    for i in range(n_fields):
        assert len(field_candidates[i]) == 1

    return {
        next(iter(field_candidates[i])): i
        for i in range(n_fields)
    }


def puzzle2(rules: Rules, tickets: List[Ticket], my_ticket: Ticket) -> None:
    index_of_field = find_index_of_field(rules, tickets)
    p = 1
    for field in index_of_field:
        if field.startswith('departure'):
            p *= my_ticket[index_of_field[field]]
    print(p)


def main() -> None:
    rules, my_ticket, tickets = read_input()
    puzzle1(rules, tickets)
    puzzle2(rules, tickets, my_ticket)


if __name__ == '__main__':
    main()
