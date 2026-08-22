import json
import os
import urllib.request
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


DATA_FILES = {
    "pokedex.ts": "https://raw.githubusercontent.com/smogon/pokemon-showdown/refs/heads/master/data/pokedex.ts",
    "pokedex-mini.js": "https://play.pokemonshowdown.com/data/pokedex-mini.js",
}


def relative_path(name):
    basepath = os.path.dirname(__file__)
    return os.path.abspath(os.path.join(basepath, name))


def cache_files():
    for name in DATA_FILES:
        path = relative_path(name)
        if not os.path.isfile(path):
            req = urllib.request.Request(DATA_FILES[name], headers={"User-Agent": "discord_bot"})
            with urllib.request.urlopen(req) as response:
                with open(path, "wb") as file:
                    file.write(response.read())


def remove_comments(line: str) -> str:
    split = line.split("//")
    return split[0]


def handle_base_stats(base_stats: str) -> str:
    # This is ultra hacky :^)
    split = base_stats.split()
    stats = f'"hp": {split[2]} "atk": {split[4]} "def": {split[6]} "spa": {split[8]} "spd": {split[10]} "spe": {split[12]}'

    built = " { " + stats + "},"
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
    return f' "{res}",'


def main():
    cache_files()

    output_data: List[str] = []

    with open(relative_path("pokedex.ts"), "r") as pokedex:
        for i, line in enumerate(pokedex):
            line = remove_comments(line)

            leading_spaces = len(line) - len(line.lstrip())
            indent = " " * (leading_spaces * 2)

            line = line.strip()

            if line == "":
                # Skip empty lines and comment-only lines
                pass
            elif i == 0:
                # The first line should be replaced with '{'
                output_data.append("{\n")
            elif line.startswith("}") or line.startswith("]"):
                # If current line is closing an object, remove trailing comma from the previous line

                previous_line = output_data[-1]
                # Need to remove trailing whitespace before I can strip the trailing comma
                previous_line = previous_line.rstrip().rstrip(",") + "\n"
                output_data[-1] = previous_line

                # Do this here for nicer formatting
                output_data.append(f'{indent}{line}\n')

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
                    new_line = f'{indent}"{key}":{value}\n'

                    output_data.append(new_line)
                else:
                    # I think this can no longer be reached?
                    output_data.append(line)

        # Remove trailing semicolon at the end
        output_data[-1] = output_data[-1].rstrip().rstrip(";") + "\n"

    with open(relative_path("pokedex.json"), "w") as new_file:
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
