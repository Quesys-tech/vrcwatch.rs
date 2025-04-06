from pathlib import Path
import tomlkit
import sys

if __name__ == "__main__":
    new_version = sys.argv[1]
    if new_version.startswith("v"):
        new_version = new_version[1:]
    
    project_dir = Path(__file__).parent.parent

    # Chenge the version in Cargo.toml
    cargo_toml_path = project_dir / "Cargo.toml"
    with open(cargo_toml_path, "r") as f:
        cargo_toml = tomlkit.load(f)
    
    pkg_name = cargo_toml["package"]["name"]
    print(f"Updating {pkg_name} version to {new_version}")

    cargo_toml["package"]["version"] = new_version
    with open(cargo_toml_path, "w") as f:
        tomlkit.dump(cargo_toml, f)

    # Change the version in Cago.lock
    cargo_lock_path = project_dir / "Cargo.lock"
    with open(cargo_lock_path, "r") as f:
        cargo_lock = tomlkit.load(f)

    for pkg in cargo_lock["package"]:
        if pkg["name"] == pkg_name:
            pkg["version"] = new_version
            break
    with open(cargo_lock_path, "w") as f:
        tomlkit.dump(cargo_lock, f)
            
