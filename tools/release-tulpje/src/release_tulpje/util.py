from pathlib import PurePath
import subprocess
from typing import Optional
import os

from termcolor import colored


def find_file_upwards(path: PurePath, name: str) -> Optional[PurePath]:
    full_path = path.joinpath(name)
    if os.path.isfile(full_path):
        return full_path
    else:
        if os.path.dirname(path) == path:
            return None
        else:
            return find_file_upwards(path.parent, name)


def process_run(*args, **kwargs) -> str:
    """run a process and print it's output if it errors, otherwise return output"""
    output_on_error = kwargs.pop("output_on_error", True)
    try:
        return subprocess.check_output(*args, **kwargs)
    except subprocess.CalledProcessError as e:
        if output_on_error:
            print(e.output)
        raise e


def filename_match(filename: PurePath, matchlist: set[str]) -> bool:
    def match_entry(filename: PurePath, entry: str) -> bool:
        if entry.startswith("!"):
            return filename.match(entry[1:], case_sensitive=True) or filename.match(
                f"*/{entry[1:]}", case_sensitive=True
            )
        else:
            return filename.match(entry, case_sensitive=True) or filename.match(
                f"*/{entry}", case_sensitive=True
            )

    return any(match_entry(filename, entry) for entry in matchlist)


def dry_print(execute: bool, text: str) -> None:
    print(text if execute else text + colored(" (dry-run)", attrs=["bold"]))


def check_output_dry(title: Optional[str], execute: bool, *args, **kwargs):
    if title is not None:
        print(title)

    print(
        "      ",
        ("" if execute else colored("(dry-run)", "grey", attrs=["bold"])),
        colored(f"> {' '.join(args[0])}", "grey"),
    )

    if execute:
        return process_run(*args, **kwargs)
    else:
        return ""
