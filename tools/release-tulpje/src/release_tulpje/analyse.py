import json
import dataclasses
from dataclasses import dataclass
from pathlib import Path
from typing import Self


@dataclass
class AnalysisResult:
    skip_slow: bool

    @classmethod
    def load(cls, path: Path) -> Self:
        with open(path, "rt") as f:
            return cls(**json.load(f))

    def save(self, path: Path):
        with open(path, "wt") as f:
            json.dump(dataclasses.asdict(self), f)
