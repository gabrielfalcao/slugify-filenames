#!/usr/bin/env python3
import os
import math
import sys
import time
import click
import json
import random
import string
from pathlib import Path
from functools import reduce
from itertools import chain

here = Path(__file__).parent.absolute()

adjective = [
    "admiring",
    "adoring",
    "agitated",
    "amazing",
    "angry",
    "awesome",
    "backstabbing",
    "berserk",
    "big",
    "boring",
    "clever",
    "cocky",
    "compassionate",
    "condescending",
    "cranky",
    "desperate",
    "determined",
    "distracted",
    "dreamy",
    "drunk",
    "ecstatic",
    "elated",
    "elegant",
    "evil",
    "fervent",
    "focused",
    "furious",
    "gigantic",
    "gloomy",
    "goofy",
    "grave",
    "happy",
    "high",
    "hopeful",
    "hungry",
    "insane",
    "jolly",
    "jovial",
    "kickass",
    "lonely",
    "loving",
    "mad",
    "modest",
    "naughty",
    "nostalgic",
    "pensive",
    "prickly",
    "reverent",
    "romantic",
    "sad",
    "serene",
    "sharp",
    "sick",
    "silly",
    "sleepy",
    "small",
    "stoic",
    "stupefied",
    "suspicious",
    "tender",
    "thirsty",
    "tiny",
    "trusting",
]
noun = [
    "albattani",
    "allen",
    "almeida",
    "archimedes",
    "ardinghelli",
    "aryabhata",
    "austin",
    "babbage",
    "banach",
    "bardeen",
    "bartik",
    "bell",
    "bhabha",
    "bhaskara",
    "blackwell",
    "bohr",
    "booth",
    "borg",
    "bose",
    "boyd",
    "brahmagupta",
    "brattain",
    "brown",
    "carson",
    "chandrasekhar",
    "colden",
    "cori",
    "cray",
    "curie",
    "darwin",
    "davinci",
    "dijkstra",
    "dubinsky",
    "easley",
    "einstein",
    "elion",
    "engelbart",
    "euclid",
    "euler",
    "fermat",
    "fermi",
    "feynman",
    "franklin",
    "galileo",
    "gates",
    "goldberg",
    "goldstine",
    "goodall",
    "hamilton",
    "hawking",
    "heisenberg",
    "hodgkin",
    "hoover",
    "hopper",
    "hugle",
    "hypatia",
    "jang",
    "jennings",
    "jepsen",
    "joliot",
    "jones",
    "kalam",
    "kare",
    "keller",
    "khorana",
    "kilby",
    "kirch",
    "knuth",
    "kowalevski",
    "lalande",
    "lamarr",
    "leakey",
    "leavitt",
    "lichterman",
    "liskov",
    "lovelace",
    "lumiere",
    "mahavira",
    "mayer",
    "mccarthy",
    "mcclintock",
    "mclean",
    "mcnulty",
    "meitner",
    "meninsky",
    "mestorf",
    "mirzakhani",
    "morse",
    "newton",
    "nobel",
    "noether",
    "northcutt",
    "noyce",
    "panini",
    "pare",
    "pasteur",
    "payne",
    "perlman",
    "pike",
    "poincare",
    "poitras",
    "ptolemy",
    "raman",
    "ramanujan",
    "ride",
    "ritchie",
    "roentgen",
    "rosalind",
    "saha",
    "sammet",
    "shaw",
    "shockley",
    "sinoussi",
    "snyder",
    "spence",
    "stallman",
    "swanson",
    "swartz",
    "swirles",
    "tesla",
    "thompson",
    "torvalds",
    "turing",
    "varahamihira",
    "visvesvaraya",
    "wescoff",
    "williams",
    "wilson",
    "wing",
    "wozniak",
    "wnoun",
    "yalow",
    "yonath",
]
name_pools = [adjective, noun]
transformations = [str.capitalize, str.lower, str.capitalize, str.capitalize]
extensions = ["mp4", "mkv", "wav", "png", "tar.gz"]


@click.command
@click.option("-c", "--file-count", type=int, default=-1)
def main(file_count):
    if file_count < 2:
        sys.stderr.write(f"overriding file-count < 2\n")
        file_count = random.randint(13, 37)

    folders = []
    for index in range(file_count):
        name = gen_name()
        folders.append(name)

    filenames = []
    for index in range(file_count):
        name = gen_name()
        filenames.append(name)

    leaves = []
    for index in range(math.ceil(file_count * 1.37)):
        name = gen_name(hidden=((index > 0 and index % 2) == 0))
        leaves.append(name)

    written = []
    ctx = dict(folders=folders, filenames=filenames, leaves=leaves)
    for folder in folders:
        for name in filenames:
            written.append(write_dummy(folder, name, **ctx))
    for leaf in leaves:
        written.append(write_dummy(here, name, **ctx))

    print(f"wrote {len(written)} paths")


def write_dummy(folder, filename, **ctx):
    if not isinstance(folder, (str, Path)):
        ty = type(folder).__name__
        dbg = repr(folder)
        raise TypeError(
            f"folder argument should be a path or string, but got {ty} instead: {dbg}"
        )

    if not isinstance(filename, (str, Path)):
        ty = type(filename).__name__
        dbg = repr(filename)
        raise TypeError(
            f"filename argument should be a path or string, but got {ty} instead: {dbg}"
        )

    append_extension = bool(random.randint(0, 1))
    parent = here.joinpath(folder)
    parent.mkdir(parents=True, exist_ok=True)
    if append_extension:
        path = parent.joinpath(
            "{filename}.{extension}".format(
                filename=filename, extension=random.choice(extensions)
            )
        )
    else:
        path = parent.joinpath(filename)

    with path.open("w") as fd:
        data = dict(
            filename=str(filename),
            path=str(path),
            time=int(time.time()),
            pid=os.getpid(),
            ppid=os.getppid(),
        )
        data.update(ctx)

        fd.write(json.dumps(data))

    return path


def clear_term():
    sys.stderr.write("\x1b[2J\x1b[3J\x1b[H")


def gen_words(spaces: int = None):
    if not isinstance(spaces, int) or spaces < 3:
        spaces = random.randint(3, 7)
    words = [random.choice(random.choice(name_pools)) for _ in range(1, spaces)]
    return [random.choice(transformations)(word) for word in words]


def gen_name(spaces: int = None, hidden: bool = None):
    if not isinstance(spaces, int) or spaces < 3:
        spaces = random.randint(3, 7)
    if hidden is None:
        hidden = bool(spaces % 2 == 0)

    words = gen_words(spaces)
    name = " ".join(words)
    if hidden:
        name = name.lstrip()
        return f".{name}"

    return name


if __name__ == "__main__":
    main()
