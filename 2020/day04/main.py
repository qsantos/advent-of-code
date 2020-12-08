import re
from typing import Dict, Iterator, List

Passport = Dict[str, str]


def read_passports() -> Iterator[Passport]:
    with open('input') as f:
        data = f.read()
    passports = data.split('\n\n')
    for passport in passports:
        fields = passport.split()
        yield {
            k: v
            for field in fields
            for k, v in [field.split(':')]
        }


def puzzle1(passports: List[Passport]) -> None:
    print(sum(
        {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'} <= set(passport)
        for passport in passports
    ))


def puzzle2(passports: List[Passport]) -> None:
    n_valid_passports = 0
    for passport in passports:
        try:
            if not 1920 <= int(passport['byr']) <= 2002:
                continue
            if not 2010 <= int(passport['iyr']) <= 2020:
                continue
            if not 2020 <= int(passport['eyr']) <= 2030:
                continue
            hgt = passport['hgt']
            if not (
                (hgt.endswith('cm') and 150 <= int(hgt[:-2]) <= 193)
                or (hgt.endswith('in') and 59 <= int(hgt[:-2]) <= 76)
            ):
                continue
            if not re.fullmatch(r'#[0-9a-f]{6}', passport['hcl']):
                continue
            if passport['ecl'] not in {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'}:
                continue
            if not re.fullmatch(r'[0-9]{9}', passport['pid']):
                continue
        except (KeyError, ValueError):
            continue
        n_valid_passports += 1
    print(n_valid_passports)


def main() -> None:
    passports = list(read_passports())
    puzzle1(passports)
    puzzle2(passports)


if __name__ == '__main__':
    main()
