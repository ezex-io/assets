import os
import yaml
import argparse
from pathlib import Path
from jinja2 import Environment, FileSystemLoader, TemplateSyntaxError

# Paths
GEN_DIR = "gen/golang"
TEMPLATE_DIR = "generator/templates"
TEMPLATE_FILE = "go.tmpl"

# Ensure output directory exists
os.makedirs(GEN_DIR, exist_ok=True)

# Load Jinja2 environment and template
env = Environment(loader=FileSystemLoader(TEMPLATE_DIR), trim_blocks=True, lstrip_blocks=True)

try:
    template = env.get_template(TEMPLATE_FILE)
except TemplateSyntaxError as e:
    print(f"❌ Jinja2 Template Syntax Error: {e.message} on line {e.lineno}")
    exit(1)

def read_yaml(file_path):
    """Read a YAML file and return its content as a dictionary."""
    with open(file_path, "r", encoding="utf-8") as f:
        return yaml.safe_load(f)

def encode_svg(file_path):
    """Read an SVG file and return its raw content."""
    if not os.path.exists(file_path):
        return ""
    with open(file_path, "r", encoding="utf-8") as f:
        return f.read()

def process_chain(chain_path):
    """Process a blockchain directory and generate a Go file."""
    chain_name = chain_path.stem  # Get the chain name from the path
    print(f"🔍 Processing {chain_name}...")

    # Read blockchain info.yml
    chain_info_path = chain_path / "info.yml"
    if not chain_info_path.exists():
        print(f"❌ Error: {chain_info_path} not found!")
        return

    chain_info = read_yaml(chain_info_path)

    # Ensure links are parsed correctly
    links = chain_info.get("links", [])
    if not isinstance(links, list):
        print(f"⚠️ Warning: Links should be a list in {chain_info_path}, got {type(links)} instead.")
        links = []  # Default to an empty list

    # Read blockchain logo
    chain_logo = encode_svg(chain_path / "logo.svg")

    # Process assets
    assets = []
    assets_dir = chain_path / "assets"
    if assets_dir.exists():
        for asset_id in os.listdir(assets_dir):
            asset_path = assets_dir / asset_id
            asset_info_path = asset_path / "info.yml"
            if not asset_info_path.exists():
                print(f"⚠️ Warning: {asset_info_path} not found, skipping asset {asset_id}")
                continue

            asset_info = read_yaml(asset_info_path)
            asset_icon = encode_svg(asset_path / "icon.svg")

            assets.append({
                "StructName": f"{chain_name.capitalize()}{asset_info['symbol'].upper()}Asset",
                "ID": asset_info["id"],
                "Name": asset_info["name"],
                "Symbol": asset_info["symbol"],
                "Type": asset_info.get("type", ""),
                "Description": asset_info.get("description", ""),
                "Website": asset_info.get("website", ""),
                "Explorer": asset_info.get("explorer", ""),
                "Decimals": asset_info.get("decimals", 0),
                "Status": asset_info.get("status", ""),
                "Icon": asset_icon
            })

    # Generate the Go file
    try:
        go_code = template.render(
            StructName=f"{chain_name.capitalize()}Blockchain",
            FactoryFunc=chain_name.capitalize(),
            Name=chain_info["name"],
            Description=chain_info["description"],
            Website=chain_info["website"],
            Explorer=chain_info["explorer"],
            Symbol=chain_info["symbol"],
            Decimals=chain_info["decimals"],
            Links=links,  # ✅ Pass the properly parsed links
            Assets=assets,
            Logo=chain_logo
        )
    except Exception as e:
        print(f"❌ Jinja2 Rendering Error: {e}")
        return

    # Write to file
    output_file = Path(GEN_DIR) / f"{chain_name}.gen.go"
    with open(output_file, "w", encoding="utf-8") as f:
        f.write(go_code)

    print(f"✅ Generated {output_file}")

def main():
    """Main function to process a specific chain passed as an argument."""
    parser = argparse.ArgumentParser(description="Generate Go files for a specific blockchain")
    parser.add_argument("--chain", type=str, required=True, help="Path to the blockchain directory (e.g., chains/bitcoin)")

    args = parser.parse_args()
    chain_path = Path(args.chain)

    if not chain_path.exists() or not chain_path.is_dir():
        print(f"❌ Error: Chain directory '{args.chain}' does not exist!")
        return

    process_chain(chain_path)

if __name__ == "__main__":
    main()
