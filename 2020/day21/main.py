from itertools import permutations
from typing import Dict, List, Set, Tuple

Food = Tuple[Set[str], Set[str]]


def read_foods(filename: str) -> List[Food]:
    foods = []
    with open(filename) as f:
        for line in f:
            ingredients_str, allergens_str = line.rstrip(')\n').split(' (contains ')
            ingredients = set(ingredients_str.split())
            allergens = set(allergens_str.split(', '))
            foods.append((ingredients, allergens))
    return foods


def allergen_free_ingredients(foods: List[Food]) -> Set[str]:
    foods_of_ingredient: Dict[str, Set[int]] = {
        ingredient: set()
        for ingredients, _ in foods
        for ingredient in ingredients
    }
    foods_of_allergen: Dict[str, Set[int]] = {
        allergen: set()
        for _, allergens in foods
        for allergen in allergens
    }
    for i, (ingredients, allergens) in enumerate(foods):
        for ingredient in ingredients:
            foods_of_ingredient[ingredient].add(i)
        for allergen in allergens:
            foods_of_allergen[allergen].add(i)

    return {
        ingredient
        for ingredient, foods in foods_of_ingredient.items()
        if all(
            # the elements of other_foods are associated with the associated
            # allergen but some not include ingredient, so ingredient cannot
            # contain the associated allergen
            not other_foods <= foods
            for other_foods in foods_of_allergen.values()
        )
    }


def puzzle1(foods: List[Food]) -> int:
    return sum(
        len(allergen_free_ingredients(foods) & ingredients)
        for ingredients, allergens in foods
    )


def find_allergen_of_ingredient(foods: List[Food]) -> Dict[str, str]:
    ingredients_with_no_allergens = allergen_free_ingredients(foods)

    sorted_ingredients = sorted({
        ingredient
        for ingredients, _ in foods
        for ingredient in ingredients
        if ingredient not in ingredients_with_no_allergens
    })
    allergens = {
        allergen
        for _, allergens in foods
        for allergen in allergens
    }
    assert len(sorted_ingredients) == len(allergens)
    for sorted_allergens in permutations(allergens):
        mapping = dict(zip(sorted_allergens, sorted_ingredients))
        for ingredients, allergens in foods:
            for allergen in allergens:
                try:
                    expected_ingredient = mapping[allergen]
                except KeyError:
                    continue
                if expected_ingredient not in ingredients:
                    break
            else:
                continue
            break
        else:
            return dict(zip(sorted_ingredients, sorted_allergens))
    assert False


def puzzle2(foods: List[Food]) -> str:
    allergen_of_ingredient = find_allergen_of_ingredient(foods)
    return ','.join(sorted(allergen_of_ingredient.keys(), key=allergen_of_ingredient.__getitem__))


def main() -> None:
    example = read_foods('example')
    input = read_foods('input')

    assert puzzle1(example) == 5
    assert puzzle1(input) == 2798

    assert puzzle2(example) == 'mxmxvkd,sqjhc,fvjkl'
    assert puzzle2(input) == 'gbt,rpj,vdxb,dtb,bqmhk,vqzbq,zqjm,nhjrzzj'


if __name__ == '__main__':
    main()
