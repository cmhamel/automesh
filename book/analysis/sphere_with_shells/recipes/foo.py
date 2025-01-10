from pathlib import Path
from typing import NamedTuple

from schema import Schema, And, Or, Use, Optional, SchemaError
import yaml


class Parent(NamedTuple):
    folder: Path
    file: Path


# Define the schema for the 'parent' key
parent_schema = Schema(
    Or(
        {
            "folder": And(str, len),  # "folder" must be a non-empty string
            "file": And(str, len),  # "file" must be a non-empty string
        },
        None,
    )
)

# Define the main schema that includes the "parent" key
main_schema = Schema(
    {
        "parent": parent_schema,
    },
    ignore_extra_keys=True,  # Allow any additional keys
)

# Example data to validate
vd0 = {
    "parent": {
        "folder": "~/autotwin/automesh/book/analysis/sphere_with_shells/recipes",
        "file": "displacement_base.yml",
    }
}

vd1 = {"parent": None}

fa = Path(__file__).parent.joinpath("displacement_sr2_new.yml")
assert fa.is_file()
with open(str(fa), "r") as file:
    da = yaml.safe_load(file)

# Validate the example data
for item in (vd0, vd1, da):
    try:
        validated_data = main_schema.validate(item)
        print(f"Valid: Validated data for {item}:\n\n", validated_data)
    except SchemaError as e:
        print(f"Error: Validation error for {item}:\n\n", e)

# for item in (vd0, vd1, da):
for item in (da,):
    if item.get("parent") is not None:
        print("Parent is not None")
        parent = Parent(
            folder=Path(item["parent"]["folder"]).expanduser(),
            file=Path(item["parent"]["folder"])
            .expanduser()
            .joinpath(item["parent"]["file"]),
        )
        print(f"parent.folder = {parent.folder}")
        print(f"parent.file = {parent.file}")
        breakpoint()
        assert (
            parent.folder.is_dir()
        ), f"Error: folder not found {parent.folder}"
        assert parent.file.is_file(), f"Error: file not found {parent.file}"
    else:
        print("Parent is None")


breakpoint()


# fb = Path(__file__).parent.joinpath("displacement_sr2_new.yml")
# assert fb.is_file()
# with open(str(fb), "r") as file:
#    db = yaml.safe_load(file)

aa = 4
