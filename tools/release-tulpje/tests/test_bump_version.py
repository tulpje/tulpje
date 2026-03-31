import pytest
from release_tulpje import version_bump
from semver import Version

TEST_DATA = [
    ("0.1.0", "0.2.0", False, True, None),
    ("1.0.0", "2.0.0", False, True, None),
    # feature bumps patch if 0.x, minor if 1.x
    ("0.1.0", "0.1.1", True, False, None),
    ("1.0.0", "1.1.0", True, False, None),
    # regular always bumps patch
    ("0.1.0", "0.1.1", False, False, None),
    ("1.0.0", "1.0.1", False, False, None),
    # normal -> prerelease bumps relevant parts and adds prerelease
    ("0.1.0", "0.1.1-beta.1", False, False, "beta"),
    ("0.1.0", "0.1.1-beta.1", True, False, "beta"),
    ("0.1.0", "0.2.0-beta.1", False, True, "beta"),
    ("1.0.0", "1.0.0-beta.1", False, False, "beta"),
    ("1.0.0", "1.1.0-beta.1", True, False, "beta"),
    ("1.0.0", "2.0.0-beta.1", False, True, "beta"),
    # prerelease -> prerelease without feature or breaking only bumps prerelease tag
    ("0.1.0-beta.1", "0.1.0-beta.2", False, False, "beta"),
    # prerelease -> different prerelease without feature or breaking only changes prerelease tag
    ("0.1.0-beta.1", "0.1.0-rc.1", False, False, "rc"),
    # prerelease to normal only removes prerelease tag
    ("0.1.0-beta.1", "0.1.0", False, False, None),
]


@pytest.mark.parametrize("input,expected,feature,breaking,prerelease", TEST_DATA)
def test_bump_version(input, expected, feature, breaking, prerelease):
    assert (
        str(version_bump(Version.parse(input), feature, breaking, prerelease))
        == expected
    )
