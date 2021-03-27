#!/usr/bin/env python3

"""
# Rust Project Builder Assistant

> Easier huh?

## Description

What I do for you:
+ provide a quick and efficient way to execute complexe commands without having to
  memorized them
+ compile your project in `debug` or `release`
+ execute unit and integration tests
+ choice between quick or complete tests
+ build the documentation
+ publish the crate

## Usage

To build and run the program, tests or examples:

Usage: ./compile.py [OPTIONS]
    -r, --release        build in release, default is debug
    -e, --example [NAME] run the example [NAME]
    -t, --test           launch fast tests
    --all-targets        to be used with -t, runs all tests
"""

from pathlib import Path
from typing import Optional, Any
from dotmap import DotMap
import envtoml
import subprocess
import termcolor
import re
import os
import sys

import click
from pudb import set_trace as bp  # noqa

# Parameters.
TEST_FLAGS = "-- --color always --nocapture "
RUST_BACKTRACE = "full"

# Initialization.
MANIFEST_FILE = Path("Cargo.toml")
MANIFEST = DotMap(envtoml.load(MANIFEST_FILE.open()))
_NAME = MANIFEST.package.name
VERSION = MANIFEST.package.version
CONTEXT_SETTINGS = dict(help_option_names=["-h", "--help"])
REGEX_VERSION = dict()
REGEX_VERSION.update(
    dict.fromkeys(
        ["src/lib.rs", "README.md"],
        {
            "pattern": rf'(\[dependencies\][\r]?\n{_NAME}\s=\s)("[\d.]+")',
            "repl": rf'\g<1>"{VERSION}"',
        },
    )
)


def export_environment_variables() -> None:
    """Export environment variables."""
    if os.environ.get("RUST_BACKTRACE") != RUST_BACKTRACE:
        os.environ["RUST_BACKTRACE"] = RUST_BACKTRACE


def build(command: str, flags: str = "") -> None:
    """Proceed to build with following the given flags."""
    try:
        subprocess.check_call(f"cargo {command} {flags}", shell=True)
    except subprocess.CalledProcessError:
        # The publishing might fail, the error is explained by Cargo.
        sys.exit(1)


def check_release(flags: str = "", release: bool = False) -> str:
    """
    Add to flags if the build is meant to be in release mode.

    The argument `flags` is mutable in the scope of the function.
    """
    if release:
        flags += "--release "
    return flags


def print_version(ctx: click.Context, param: Any, value: bool) -> None:
    """Show version."""
    if not value or ctx.resilient_parsing:
        return
    click.echo(VERSION)
    ctx.exit()


@click.group(context_settings=CONTEXT_SETTINGS, invoke_without_command=True)
@click.pass_context
@click.option(
    "-v",
    "--version",
    is_flag=True,
    callback=print_version,
    expose_value=False,
    is_eager=True,
    help="Show version.",
)
@click.option(
    "-r",
    "--release",
    is_flag=True,
    help="Enable release mode.",
)
def cli(ctx: click.Context, release: bool) -> None:
    """
    Rust Project Builder Assistant.

    By default, compile the binary from the main program in debug mode, and run it. If
    you want to compile in release mode, for the main command or any subcommand, use the
    release option.
    """
    # Ensure that ctx.obj exists.
    ctx.ensure_object(dict)
    # Share default command options.
    ctx.obj["RELEASE"] = release
    # Environment variable.
    export_environment_variables()
    # Default command.
    if ctx.invoked_subcommand is None:
        flags = check_release(release=release)
        build("run", flags)


@cli.command()
@click.pass_context
@click.argument("name", required=False)
def example(ctx: click.Context, name: Optional[str] = None) -> None:
    """Compile and run an example."""
    flags = check_release(release=ctx.obj["RELEASE"])
    flags += "--example "
    if name is not None:
        flags += name
        build("run", flags)
    else:
        # Get list of available examples.
        list_examples = [example.stem for example in Path("examples").glob("*.rs")]
        # Prepare string format.
        example_files = "   " + "\n   ".join(list_examples)
        # Organize error message.
        error_title = termcolor.colored("missing the name of the example", "red")
        raised_error = f"{error_title}.\n\nAvailable examples:\n{example_files}"
        # Raise error.
        raise TypeError(raised_error)


@cli.command()
@click.pass_context
@click.option(
    "-a",
    "--all",
    "all_",
    is_flag=True,
    help="Test all targets: lib, bins, tests, benches, examples.",
)
def tests(ctx: click.Context, all_: bool) -> None:
    """Compile and execute unit and integration tests."""
    flags = check_release(release=ctx.obj["RELEASE"])
    if all_:
        flags += "--all-targets "
    else:
        flags += "--test lib "
    flags += TEST_FLAGS
    build("test", flags)


@cli.command()
@click.pass_context
@click.option(
    "-o",
    "--open",
    "open_",
    is_flag=True,
    help="Open the documentation in your browser after the build.",
)
def doc(ctx: click.Context, open_: bool) -> None:
    """Build documentation."""
    flags = ""
    if open_:
        flags += "--open "
    build("doc", flags)


@cli.command()
@click.pass_context
@click.option(
    "-c",
    "--check",
    is_flag=True,
    help="Check any warning or error before publishing.",
)
def publish(ctx: click.Context, check: bool) -> None:
    """Publish your library on crates.io."""
    flags = ""
    if check:
        flags += "--dry-run "
    build("publish", flags)


@cli.command()
@click.pass_context
def update_versions(ctx: click.Context) -> None:
    """Update versions in all files according to the manifest."""
    for file_to_check, regex in REGEX_VERSION.items():
        path = Path(file_to_check)
        contents = path.read_text()
        # Check result is unique.
        result = re.findall(regex["pattern"], contents)
        if len(result) != 1:
            raise ValueError(
                f"More than one match for the match pattern of {path.name} file"
            )
        # Apply version correction.
        contents = re.sub(regex["pattern"], regex["repl"], contents)
        with path.open("w") as f:
            f.write(contents)


def main() -> None:
    cli(prog_name=__file__)


if __name__ == "__main__":
    main()
