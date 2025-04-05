from pathlib import Path
import tomlkit
import sys

if __name__ == "__main__":
    new_version = sys.argv[1]
    if new_version.startswith("v"):
        new_version = new_version[1:]
    
    project_dir = Path(__file__).parent.parent
    cargo_toml_path = project_dir / "Cargo.toml"
    cargo_toml = tomlkit.load(cargo_toml_path)
    
    pkg_name = cargo_toml["package"]["name"]
    print(f"Updating {pkg_name} version to {new_version}")

    cargo_toml["package"]["version"] = new_version
    with open(cargo_toml_path, "w") as f:
        tomlkit.dump(cargo_toml, f)
