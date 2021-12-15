import re

re_lexem = re.compile(r'\(|\)|[0-9]+|[+*]')


def evaluate_left_to_right(expr: str) -> int:
    lexems = re_lexem.finditer(expr)

    def parse_operand() -> int:
        lexem = next(lexems).group()
        if lexem == '(':
            return parse_expr()
        elif lexem in ['+', '-']:
            raise SyntaxError(f'Unexpected token {lexem}')
        else:
            return int(lexem)

    def parse_expr() -> int:
        cur = parse_operand()
        for lexem in lexems:
            operator = lexem.group()
            if operator == ')':
                return cur
            if operator not in ['+', '*']:
                raise SyntaxError(f'Unexpected token {operator}')
            other = parse_operand()
            if operator == '+':
                cur += other
            elif operator == '*':
                cur *= other
            else:
                assert False
        return cur

    return parse_expr()


def evaluate_precedence(expr: str) -> int:
    lexems = re_lexem.findall(expr)[::-1]

    precedence = {
        '+': 2,
        '*': 1,
    }

    def parse_operand() -> int:
        lexem = lexems.pop()
        if lexem == '(':
            ret = parse_expr()
            assert lexems.pop() == ')'
            return ret
        elif lexem in ['+', '-', '*', '/']:
            raise SyntaxError(f'Unexpected token {lexem}')
        else:
            return int(lexem)

    def parse_expr() -> int:
        return parse_expr1(parse_operand(), 0)

    def parse_expr1(lhs: int, min_precedence: int = 0) -> int:
        if precedence[lexems[-1]] < min_precedence:
            return lhs
        while lexems and lexems[-1] in precedence and precedence[lexems[-1]] >= min_precedence:
            op = lexems.pop()
            rhs = parse_operand()
            while lexems and lexems[-1] in precedence and precedence[lexems[-1]] >= precedence[op]:
                rhs = parse_expr1(rhs, precedence[lexems[-1]])
            if op == '+':
                lhs = lhs + rhs
            elif op == '*':
                lhs = lhs * rhs
            else:
                assert False
        return lhs

    return parse_expr()


def puzzle1() -> None:
    with open('input') as f:
        print(sum(
            evaluate_left_to_right(line)
            for line in f
        ))


def puzzle2() -> None:
    with open('input') as f:
        print(sum(
            evaluate_precedence(line)
            for line in f
        ))


def main() -> None:
    puzzle1()
    puzzle2()


if __name__ == '__main__':
    main()
