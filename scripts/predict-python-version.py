import argparse
import json
import re
import subprocess


STABLE_TAG = re.compile(
    r"refs/tags/((?:python-)?v((?:0|[1-9][0-9]*)(?:\.(?:0|[1-9][0-9]*)){2}))(\^\{\})?"
)


def git(*args):
    return subprocess.check_output(["git", *args], text=True)


def latest_release(repo):
    refs = git(
        "ls-remote",
        "--tags",
        f"https://github.com/{repo}.git",
        "refs/tags/v*",
        "refs/tags/python-v*",
    )
    releases = {}
    for line in refs.splitlines():
        commit, ref = line.split("\t", 1)
        match = STABLE_TAG.fullmatch(ref)
        if not match:
            continue
        tag, version_text, peeled = match.groups()
        version = tuple(map(int, version_text.split(".")))
        # Prefer the shared tag when both tag names exist for the same version.
        release_key = (version, tag.startswith("v"))
        if peeled or release_key not in releases:
            releases[release_key] = (version, tag, commit)
    if not releases:
        return None
    return releases[max(releases)]


def has_changes_since(commit):
    if git("rev-parse", "HEAD").strip() == commit:
        return bool(git("status", "--porcelain"))
    ancestry = subprocess.run(["git", "merge-base", "--is-ancestor", commit, "HEAD"])
    if ancestry.returncode:
        raise SystemExit("Fetch and check out a branch that contains the latest release tag.")
    return True


def package_version():
    metadata = subprocess.check_output(
        ["cargo", "metadata", "--locked", "--no-deps", "--format-version", "1"], text=True
    )
    return json.loads(metadata)["packages"][0]["version"]


def predict(repo, bump):
    release = latest_release(repo)
    if release is None:
        version = package_version()
        if bump == "auto":
            return version
        base = tuple(map(int, re.split(r"[-+]", version, maxsplit=1)[0].split(".")))
    else:
        base, tag, commit = release
        if bump == "auto" and not has_changes_since(commit):
            return f"No changes since {tag}"

    major, minor, patch = base
    if bump == "major":
        return f"{major + 1}.0.0"
    if bump == "minor":
        return f"{major}.{minor + 1}.0"
    return f"{major}.{minor}.{patch + 1}"


def main():
    parser = argparse.ArgumentParser(description="Preview the next Python package version.")
    parser.add_argument("--repo", required=True, help="GitHub repository in owner/name form")
    parser.add_argument("--bump", choices=("auto", "patch", "minor", "major"), default="auto")
    args = parser.parse_args()
    try:
        print(predict(args.repo, args.bump))
    except subprocess.CalledProcessError as error:
        raise SystemExit(error.returncode) from error


if __name__ == "__main__":
    main()
