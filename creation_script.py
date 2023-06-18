#!/usr/bin/env python
import os
import toml

# Path to the Cargo.toml file
cargo_toml_path = 'Cargo.toml'
import subprocess

try:
    with open(cargo_toml_path, 'r') as cargo_file:
        cargo_data = cargo_file.read()

    # Parse the TOML data
    cargo_toml = toml.loads(cargo_data)

    # Get the list of members
    members = cargo_toml['workspace']['members']

    # Create a folder for the scripts

    # Create a shell script for each member
    for member in members:
        shortened_member = member.replace('something-',"")
        script_content = f'cd ./{member}\ncargo run -q "$@"'
        script_path = f'{shortened_member}.sh'

        # Write the shell script
        with open(script_path, 'w') as script_file:
            script_file.write(script_content)

        # Make the shell script executable
        os.chmod(script_path, 0o755)
        gitignore_path = '.gitignore'
        with open(gitignore_path, 'a') as gitignore_file:
            gitignore_file.write(f'\n{script_path}')
except FileNotFoundError:
    print(f"File '{cargo_toml_path}' not found.")
except toml.TomlDecodeError as e:
    print(f"Error decoding TOML file: {e}")
except Exception as e:
    print(f"An error occurred: {e}")
