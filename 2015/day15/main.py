from typing import Dict, Iterable, List, NamedTuple, Optional


class Ingredient(NamedTuple):
    name: str
    properties: Dict[str, int]


def prod(numbers: Iterable[int]) -> int:
    r = 1
    for x in numbers:
        r *= x
    return r


def read_ingredients(filename: str) -> List[Ingredient]:
    with open(filename) as f:
        ingredients = []
        for line in f:
            ingredient, properties = line.strip().split(': ', 1)
            d = {}
            for property in properties.split(', '):
                name, value = property.split()
                d[name] = int(value)
            ingredients.append(Ingredient(name=ingredient, properties=d))
    return ingredients


scoring_property_names = ('capacity', 'durability', 'flavor', 'texture')
all_property_names = scoring_property_names + ('calories', )


def best_score(ingredients: List[Ingredient], *, target_calories: Optional[int] = None) -> int:
    def aux(i: int, acc_properties: Dict[str, int], max_teaspoons: int) -> int:
        if i == len(ingredients):
            if target_calories is None or acc_properties['calories'] == target_calories:
                return prod(max(0, acc_properties[name]) for name in scoring_property_names)
            else:
                return 0
        ingredient = ingredients[i]
        new_properties = dict(acc_properties)
        best = 0
        for teaspoons in range(max_teaspoons + 1):
            best = max(best, aux(i + 1, new_properties, max_teaspoons - teaspoons))
            for name in all_property_names:
                new_properties[name] += ingredient.properties[name]
        return best
    return aux(0, {name: 0 for name in all_property_names}, 100)


example = read_ingredients('example')
input = read_ingredients('input')

assert best_score(example) == 62842880
assert best_score(input) == 222870

assert best_score(example, target_calories=500) == 57600000
assert best_score(input, target_calories=500) == 117936
