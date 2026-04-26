import json
from typing import List

SKIPPED_FIELDS = [
    "abilities",
    "genderRatio",
    "requiredItem",
    "eggGroups",
    "mother",
    "battleOnly",
    "canHatch",
    "gen",
    "requiredAbility",
    "requiredItems",
    "requiredMove",
    "requiredTeraType",
]


def remove_comments(line: str) -> str:
    split = line.split("//")
    return split[0]


def handle_base_stats(base_stats: str) -> str:
    # This is ultra hacky :^)
    split = base_stats.split()
    stats = f'"hp": {split[2]} "atk": {split[4]} "def": {split[6]} "spa": {split[8]} "spd": {split[10]} "spe": {split[12]}'

    built = "{ " + stats + "},\n"
    return built


def handle_tags(line: str) -> str:
    # Need to remove the newline and trailing comma for this to parse
    tags = json.loads(line.rstrip()[:-1])
    # Possible values here are
    # Paradox
    # Sub-Legendary
    # Restricted Legendary
    # Mythical
    # Ultra Beast

    # I think this can only have a length of 1?
    tag: str = tags[0].lower()
    res = "normal"
    if tag == "sub-legendary":
        res = "legendary"
    elif tag == "restricted legendary":
        res = "legendary"
    elif tag == "mythical":
        res = "mythical"
    return f' "{res}",\n'


def main():
    output_data: List[str] = []
    current_line = 0

    with open("pokedex.ts", "r") as pokedex:
        for i, line in enumerate(pokedex):
            line = remove_comments(line)

            leading_spaces = len(line) - len(line.lstrip())
            leading_spaces *= 2

            # If current line is closing an object, remove trailing comma from the previous line
            if line.strip().startswith("}") and current_line > 0:
                previous_line = output_data[current_line - 1]
                previous_line = previous_line.rstrip().rstrip(
                    ","
                )  # Need to remove trailing whitespace before I can strip the trailing comma
                output_data[current_line - 1] = f"{previous_line}\n"

                # Do this here for nicer formatting
                current_line += 1
                output_data.append((" " * leading_spaces) + "},\n")

            else:
                # If the current line begins with a key, need to add double quotes to the key
                try_split_on_key = line.split(":", maxsplit=1)
                if len(try_split_on_key) == 2:
                    key = try_split_on_key[0].strip()
                    value = try_split_on_key[1]

                    # We want to skip some fields
                    if key in SKIPPED_FIELDS:
                        continue

                    if key == "baseStats":
                        value = handle_base_stats(value)
                    elif key == "tags":
                        key = "rarity"
                        value = handle_tags(value)

                    # Re-write the line with double quotes around the key
                    new_line = f'"{key}":{value}'
                    new_line = (" " * leading_spaces) + new_line

                    current_line += 1
                    output_data.append(new_line)
                else:
                    # I think this can no longer be reached?
                    current_line += 1
                    output_data.append(line)

    with open("pokedex.json", "w") as new_file:
        for line in output_data:
            new_file.write(line)


if __name__ == "__main__":
    main()


# TODO: REMOVE ME AS THIS IS LITERAL CODE AND NOT A COMMENT
"""
- Collect all possible keys to see what I need or don't need as not all pokemon in the pokedex have all of the keys
- Need to convert pokemon with `tags` of "Sub-Legendary" and "Restricted Legendary" to just "Legendary"
- Remove the final "," of the last closing } ???
- Forms sometimes leave trailing commas that would be nice to automatically remove
"""

# TODO:
# "rarity" is my new classification thing

"""
'baseForme',
 'baseSpecies',
 'baseStats',
 'canGigantamax',
 'cannotDynamax',
 'changesFrom',
 'color',
 'cosmeticFormes',
 'evoCondition',
 'evoItem',
 'evoLevel',
 'evoMove',
 'evoRegion',
 'evoType',
 'evos',
 'forme',
 'formeOrder',
 'gender',
 'heightm',
 'isCosmeticForme',
 'maxHP',
 'name',
 'num',  # can be negative
 'otherFormes',
 'prevo',
 'tags',
 'types',
 'weightkg'
 """
