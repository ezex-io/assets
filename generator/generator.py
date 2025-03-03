import os
import yaml
import argparse
from pathlib import Path
from jinja2 import Environment, FileSystemLoader, TemplateSyntaxError

# Paths
GEN_GOLANG_DIR = "gen/golang"
GEN_RUST_DIR = "gen/rust/src"
TEMPLATE_DIR = "generator/templates"
GO_TEMPLATE_FILE = "go.tmpl"
RUST_TEMPLATE_FILE = "rust.tmpl"

# Ensure output directories exist
os.makedirs(GEN_GOLANG_DIR, exist_ok=True)
os.makedirs(GEN_RUST_DIR, exist_ok=True)

# Load Jinja2 environment
env = Environment(loader=FileSystemLoader(TEMPLATE_DIR), trim_blocks=True, lstrip_blocks=True)

# Load Templates
try:
    go_template = env.get_template(GO_TEMPLATE_FILE)
    rust_template = env.get_template(RUST_TEMPLATE_FILE)
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

def escape_svg(svg: str) -> str:
    """Escape SVG content for Rust raw string literals."""
    return svg.replace('"', '\\"')  # Escape double quotes

def process_chain(chain_path, language):
    """Process a blockchain directory and generate a file in the specified language."""
    chain_name = chain_path.stem  # Extract blockchain name from directory
    print(f"🔍 Processing {chain_name} for {language}...")

    # Read blockchain info.yml
    chain_info_path = chain_path / "info.yml"
    if not chain_info_path.exists():
        print(f"❌ Error: {chain_info_path} not found!")
        return

    chain_info = read_yaml(chain_info_path)

    # Read blockchain logo and escape for Rust if needed
    chain_logo = encode_svg(chain_path / "logo.svg")
    if language == "rust":
        chain_logo = escape_svg(chain_logo)

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
            if language == "rust":
                asset_icon = escape_svg(asset_icon)

            assets.append({
                "StructName": f"{chain_name.capitalize()}{asset_info['symbol'].upper()}Asset",
                "ID": asset_info["id"],
                "Name": asset_info["name"],
                "Symbol": asset_info["symbol"],
                "Type": asset_info.get("type", ""),
                "AssetType": asset_info.get("asset_type", ""),
                "Description": asset_info.get("description", ""),
                "Website": asset_info.get("website", ""),
                "Explorer": asset_info.get("explorer", ""),
                "Decimals": asset_info.get("decimals", 0),
                "Status": asset_info.get("status", ""),
                "Icon": asset_icon  # ✅ Escaped if Rust
            })

    # Select template based on language
    if language == "go":
        template = go_template
        output_dir = GEN_GOLANG_DIR
        file_extension = ".gen.go"
    elif language == "rust":
        template = rust_template
        output_dir = GEN_RUST_DIR
        file_extension = ".gen.rs"
    else:
        print(f"❌ Error: Unsupported language '{language}'")
        return

    # Generate the code
    try:
        generated_code = template.render(
            StructName=f"{chain_name.capitalize()}Blockchain",
            FactoryFunc=chain_name.capitalize(),
            Name=chain_info["name"],
            Description=chain_info["description"],
            Website=chain_info["website"],
            Explorer=chain_info["explorer"],
            Symbol=chain_info["symbol"],
            Decimals=chain_info["decimals"],
            Links=chain_info.get("links", []),
            Assets=assets,
            Logo=chain_logo  # ✅ Now the SVG is escaped for Rust if needed
        )
    except Exception as e:
        print(f"❌ Jinja2 Rendering Error: {e}")
        return

    # Write to file
    output_file = Path(output_dir) / f"{chain_name}{file_extension}"
    with open(output_file, "w", encoding="utf-8") as f:
        f.write(generated_code)

    print(f"✅ Generated {output_file}")

def main():
    """Main function to process a specific chain passed as an argument."""
    parser = argparse.ArgumentParser(description="Generate Go or Rust files for a specific blockchain")
    parser.add_argument("--chain", type=str, required=True, help="Path to the blockchain directory (e.g., chains/bitcoin)")
    parser.add_argument("--lang", type=str, choices=["go", "rust"], required=True, help="Target language (go or rust)")

    args = parser.parse_args()
    chain_path = Path(args.chain)

    if not chain_path.exists() or not chain_path.is_dir():
        print(f"❌ Error: Chain directory '{args.chain}' does not exist!")
        return

    process_chain(chain_path, args.lang)

if __name__ == "__main__":
    main()
