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

def update_lib_rs():
    """Automatically updates lib.rs to include all generated blockchains."""
    rust_src_dir = Path("gen/rust/src")
    lib_rs_path = rust_src_dir / "lib.rs"

    # Find all blockchain files (*.rs) in src/
    blockchain_files = [f.stem for f in rust_src_dir.glob("*.rs") if f.stem not in ["lib", "types", "blockchain", "asset"]]

    # Generate the new lib.rs content
    content = """// Code generated automatically. DO NOT EDIT.

pub mod types;
pub mod blockchain;
pub mod asset;

""" + "\n".join([f"pub mod {blockchain};" for blockchain in blockchain_files]) + "\n"

    # Write to lib.rs
    with open(lib_rs_path, "w", encoding="utf-8") as f:
        f.write(content)

    print(f"✅ Updated {lib_rs_path}")

def update_meta_files():
    """Automatically updates meta.go and meta.rs to include all generated blockchains."""
    go_output_file = Path("gen/golang/meta.go")
    rust_output_file = Path("gen/rust/src/meta.rs")

    rust_src_dir = Path("gen/rust/src")
    blockchain_files = [f.stem for f in rust_src_dir.glob("*.rs") if f.stem not in ["lib", "types", "blockchain", "asset", "meta"]]

    # ✅ Generate meta.go for Go
    go_content = """// Code generated automatically. DO NOT EDIT.

package golang

var Blockchains = map[string]Blockchain{
"""
    go_content += "\n".join([f'    "{chain}": {chain.capitalize()}(),' for chain in blockchain_files])
    go_content += "\n}\n"

    with open(go_output_file, "w", encoding="utf-8") as f:
        f.write(go_content)

    print(f"✅ Updated {go_output_file}")

    # ✅ Generate meta.rs for Rust
    rust_content = """// Code generated automatically. DO NOT EDIT.

use std::collections::HashMap;
use std::sync::Arc;
use crate::blockchain::Blockchain;

pub fn get_blockchains() -> HashMap<String, Arc<dyn Blockchain>> {
    let mut map: HashMap<String, Arc<dyn Blockchain>> = HashMap::new();
"""
    rust_content += "\n".join([f'    map.insert("{chain}".to_string(), Arc::new({chain.capitalize()}Blockchain));' for chain in blockchain_files])
    rust_content += "\n    map\n}\n"

    with open(rust_output_file, "w", encoding="utf-8") as f:
        f.write(rust_content)

    print(f"✅ Updated {rust_output_file}")

def process_chain(chain_path, language):
    """Process a blockchain directory and generate a file in the specified language."""
    chain_name = chain_path.stem  # Get chain name from directory
    print(f"🔍 Processing {chain_name} for {language}...")

    # Read blockchain info.yml
    chain_info_path = chain_path / "info.yml"
    if not chain_info_path.exists():
        print(f"❌ Error: {chain_info_path} not found!")
        return

    chain_info = read_yaml(chain_info_path)

    # Read and escape blockchain logo SVG
    chain_logo = escape_svg(encode_svg(chain_path / "logo.svg"))

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
            asset_icon = escape_svg(encode_svg(asset_path / "icon.svg"))

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
                "Icon": asset_icon  # ✅ Now properly escaped
            })

    # Select output directory and file extension based on language
    if language == "go":
        template_file = "go.tmpl"
        output_dir = "gen/golang"
        file_extension = ".gen.go"
    elif language == "rust":
        template_file = "rust.tmpl"
        output_dir = "gen/rust/src"
        file_extension = ".rs"
    else:
        print(f"❌ Error: Unsupported language '{language}'")
        return

    # Ensure output directory exists
    os.makedirs(output_dir, exist_ok=True)

    # Load Jinja2 environment and template
    env = Environment(loader=FileSystemLoader("generator/templates"), trim_blocks=True, lstrip_blocks=True)
    try:
        template = env.get_template(template_file)
    except TemplateSyntaxError as e:
        print(f"❌ Jinja2 Template Syntax Error: {e.message} on line {e.lineno}")
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
            Logo=chain_logo  # ✅ Ensuring SVG is correctly escaped for Rust
        )
    except Exception as e:
        print(f"❌ Jinja2 Rendering Error: {e}")
        return

    # Write to output file
    output_file = Path(output_dir) / f"{chain_name}{file_extension}"
    with open(output_file, "w", encoding="utf-8") as f:
        f.write(generated_code)

    print(f"✅ Generated {output_file}")

    # If Rust, update lib.rs
    if language == "rust":
        update_lib_rs()

    update_meta_files()

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
