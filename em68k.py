#!/usr/bin/python3

import subprocess
import argparse

TGTMAP = {"lib": 0, "tests": 1}
BUILDCOMMANDS = {
    "build": {
        1: ["cargo", "build"],
        0: ["vasmm68k_mot", "-L", "tests/opcode_tests.lst", "-Fbin", "-o", "tests/opcode_tests.bin", "tests/opcode_tests.asm"],
    },
    "run": {
        1: ["cargo", "run", "--"],
        0: ['cargo', 'test', '--test', 'tests', '--'],
    },
}

parser = argparse.ArgumentParser(description="Build tool for em68k.")
subparsers = parser.add_subparsers(dest="subcommand", description="", required=True)
build_parser = subparsers.add_parser("build", help="Build component <cmp>")
build_parser.add_argument("cmp", choices=("lib", "tests", "all"), nargs="?", default="all")
build_parser.add_argument("--debug", action="store_true", help="Attach debugger")
run_parser = subparsers.add_parser("run", help="Run component <cmp>")
run_parser.add_argument("cmp", choices=("lib", "tests", "all"), nargs="?", default="all")
run_parser.add_argument("--debug", action="store_true", help="Attach debugger")

args = parser.parse_args()

buildcommands = []

def construct_command(args):
    cmd = BUILDCOMMANDS[args.subcommand][(TGTMAP[args.cmp] + 1) % 2]
    if args.debug:
        cmd.append("--debug")
    return cmd

if args.cmp != "lib":
    buildcommands.append(construct_command(args))
if args.cmp != "tests":
    buildcommands.append(construct_command(args))

for cmd in buildcommands:
    subprocess.run(cmd)

