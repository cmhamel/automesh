"""This module provides utility functions."""

from pathlib import Path
from typing import Dict, Tuple

import yaml


def yml_to_dict(
    *,
    yml_path_file: Path,
    yml_schema_version: str,
    required_keys: Tuple[str, ...],
) -> Dict:
    """Given a valid Path to a yml input file, read it in and
    return the result as a dictionary.

    This function converts a yml file into a dictionary, while also
    assuring the yml file has the correct version number and correct
    key(s) present in the file.

    yml_path_file (Path): The fully pathed location to the input file.
    yml_schema_version (str): The version of the schema used for the yml input.
        The semantic versioning x.y.z format is used.
    required_keys (tuple[str,...]): The key(s) that must be in the yml file for
        conversion to a dictionary to occur.
    """

    # Compared to the lower() method, the casefold() method is stronger.
    # It will convert more characters into lower case, and will find more
    # matches on comparison of two strings that are both are converted
    # using the casefold() method.
    file_type = yml_path_file.suffix.casefold()

    supported_types = (".yaml", ".yml")

    if file_type not in supported_types:
        raise TypeError("Only file types .yaml, and .yml are supported.")

    try:
        with open(yml_path_file, mode="r", encoding="utf-8") as stream:
            # See deprecation warning for plain yaml.load(input) at
            # https://github.com/yaml/pyyaml/wiki/PyYAML-yaml.load(input)-Deprecation
            db = yaml.load(stream, Loader=yaml.SafeLoader)
    except yaml.YAMLError as error:
        print(f"Error with YAML file: {error}")
        # print(f"Could not open: {self.self.path_file_in}")
        print(f"Could not open or decode: {yml_path_file}")
        # raise yaml.YAMLError
        raise OSError from error

    # If "version" key does not exist in the dictionary, then return 0.0.0 to
    # indicate no version specified; this strategy avoids return of the None
    # type, which is prohibits logic comparisons to strings in the code that
    # below in this module.
    version_specified = db.get("yml_schema_version", "0.0.0")
    version_implemented = yml_schema_version

    if version_specified != version_implemented:
        msg = f"Version mismatch: specified was {version_specified}, "
        msg += f"implemented is {version_implemented}"
        raise ValueError(msg)
    else:
        # check keys found in input file against required keys
        found_keys = tuple(db.keys())
        keys_exist = tuple(map(lambda x: x in found_keys, required_keys))
        has_required_keys = all(keys_exist)
        if not has_required_keys:
            msg = f"Input files must have these keys defined: {required_keys}"
            raise KeyError(msg)
    return db
