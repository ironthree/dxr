#!/usr/bin/python3

# This script is used to verify that all combinations of enabled / disabled features
# compile, produce no warnings, and pass tests.

import os
import subprocess as sp


def powerset(elements):
    if not elements:
        return [[]]
    return powerset(elements[1:]) + [[elements[0]] + x for x in powerset(elements[1:])]


def check(package: str, feature_list: list[str]):
    allcombos = powerset(feature_list)
    allcombos.remove([])

    features = [["--all-features"], ["--no-default-features"]]
    features += [["--no-default-features", "--features", ",".join(features)] for features in allcombos]

    abort = False
    for n, featureset in enumerate(features):
        for command in ["check", "clippy", "test"]:
            print(f">> [{package}] [{n:04d}/{len(features):04d}] cargo {command}", " ".join(featureset))

            # cargo test --all-targets skips doctests
            targets = [] if command == "test" else ["--all-targets"]
            ret = sp.run(["cargo", command, "--package", package] + targets + featureset)

            try:
                ret.check_returncode()

            except sp.CalledProcessError:
                abort = True

            if abort:
                break

        if abort:
            break


def main():
    os.environ["QUICKCHECK_TESTS"] = "100000"

    check("dxr", ["derive", "multicall", "i8", "nil"])
    check("dxr_derive", [])
    check("dxr_client", ["default", "multicall", "reqwest", "default-tls", "native-tls", "rustls-tls"])
    check("dxr_server", ["default", "multicall", "axum"])
    check("dxr_tests", [])


if __name__ == "__main__":
    main()
