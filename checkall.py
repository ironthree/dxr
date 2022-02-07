#!/usr/bin/python3

# This script is used to verify that all combinations of enabled / disabled features
# compile, produce no warnings, and pass tests.

import subprocess as sp


def powerset(elements):
    if not elements:
        return [[]]
    return powerset(elements[1:]) + [[elements[0]] + x for x in powerset(elements[1:])]


FEATURES = [
    "client",
    "derive",
    "server",
    "i8",
    "nil",
    "dxr_derive",
    "reqwest",
    "url",
    "async-trait",
    "axum",
    "tokio",
]


def main():
    allcombos = powerset(FEATURES)
    allcombos.remove([])

    features = [["--all-features"], ["--no-default-features"]]
    features += [["--no-default-features", "--features", ",".join(features)] for features in allcombos]

    for command in ["check", "clippy"]:
        for featureset in features:
            print(f"{command}: features", " ".join(featureset))
            ret = sp.run(["cargo", command, "--all-targets"] + featureset)

            try:
                ret.check_returncode()
            except sp.CalledProcessError:
                break


if __name__ == "__main__":
    main()
