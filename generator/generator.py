import os
import yaml
import argparse
from pathlib import Path
from jinja2 import Environment, FileSystemLoader, TemplateSyntaxError

# Paths
GEN_GOLANG_DIR = "gen/golang"
GEN_RUST_DIR = "gen/rust/src"
GEN_TYPESCRIPT_DIR = "gen/typescript/src"
TEMPLATE_DIR = "generator/templates"
GO_TEMPLATE_FILE = "go.tmpl"
RUST_TEMPLATE_FILE = "rust.tmpl"
TS_TEMPLATE_FILE = "ts.tmpl"

# Ensure output directories exist
os.makedirs(GEN_GOLANG_DIR, exist_ok=True)
os.makedirs(GEN_RUST_DIR, exist_ok=True)
os.makedirs(GEN_TYPESCRIPT_DIR, exist_ok=True)

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
    """Automatically updates meta files (Go, Rust, TypeScript)."""
    go_output_file = Path("gen/golang/meta.go")
    rust_output_file = Path("gen/rust/src/meta.rs")
    ts_output_file = Path("gen/typescript/src/meta.ts")
    index_ts_file = Path("gen/typescript/src/index.ts")

    rust_src_dir = Path("gen/rust/src")
    ts_src_dir = Path("gen/typescript/src")

    # ✅ Get all blockchain names
    blockchain_files = [
        f.stem for f in rust_src_dir.glob("*.rs")
        if f.stem not in ["lib", "types", "blockchain", "asset", "meta"]
    ]

    ts_blockchain_files = [
        f.stem for f in ts_src_dir.glob("*.ts")
        if f.stem not in ["blockchain", "asset", "meta", "index", "types"]
    ]

    # ✅ Generate `meta.ts` (TypeScript)
    ts_meta_content = """// Code generated automatically. DO NOT EDIT.

import { Blockchain } from "./blockchain";
import { TypesBlockchain } from "./types";
"""
    ts_meta_content += "\n".join([
        f'import {{ {chain.capitalize()}Blockchain }} from "./{chain}";'
        for chain in ts_blockchain_files
    ]) + "\n\n"

    ts_meta_content += """export const Blockchains: Record<string, Blockchain> = {
"""
    ts_meta_content += "\n".join([
        f'    "{chain}": new {chain.capitalize()}Blockchain(),'
        for chain in ts_blockchain_files
    ])
    ts_meta_content += "\n};\n"

    with open(ts_output_file, "w", encoding="utf-8") as f:
        f.write(ts_meta_content)

    print(f"✅ Updated {ts_output_file}")

    # ✅ Generate `index.ts`
    index_ts_content = """// Code generated automatically. DO NOT EDIT.

import { Blockchain } from "./blockchain";
import { Blockchains } from "./meta";
import { TypesBlockchain } from "./types";
"""
    index_ts_content += "\n".join([
        f'import {{ {chain.capitalize()}Blockchain }} from "./{chain}";'
        for chain in ts_blockchain_files
    ]) + "\n\n"

    index_ts_content += """export {
    Blockchains,
    Blockchain,
    TypesBlockchain,
"""
    index_ts_content += "\n".join([
        f'    {chain.capitalize()}Blockchain,'
        for chain in ts_blockchain_files
    ])
    index_ts_content += "\n};\n"

    with open(index_ts_file, "w", encoding="utf-8") as f:
        f.write(index_ts_content)

    print(f"✅ Updated {index_ts_file}")

def update_meta_files(lang):
    """Automatically updates meta files (Go, Rust, TypeScript) based on the selected language."""

    # ✅ Paths for meta files
    go_output_file = Path("gen/golang/meta.go")
    rust_output_file = Path("gen/rust/src/meta.rs")
    ts_output_file = Path("gen/typescript/src/meta.ts")
    index_ts_file = Path("gen/typescript/src/index.ts")

    rust_src_dir = Path("gen/rust/src")
    ts_src_dir = Path("gen/typescript/src")

    # ✅ Get blockchain names based on the language
    if lang == "rust":
        blockchain_files = [
            f.stem for f in rust_src_dir.glob("*.rs")
            if f.stem not in ["lib", "types", "blockchain", "asset", "meta"]
        ]
    elif lang == "ts":
        blockchain_files = [
            f.stem for f in ts_src_dir.glob("*.ts")
            if f.stem not in ["blockchain", "asset", "meta", "index", "types"]
        ]
    elif lang == "go":
        blockchain_files = [
            f.stem.replace(".gen", "")
            for f in Path("gen/golang").glob("*.gen.go")
        ]
    else:
        print(f"⚠️ Skipping meta file update: Unsupported language '{lang}'")
        return

    # ✅ Update Go meta file
    if lang == "go":
        go_content = """// Code generated automatically. DO NOT EDIT.

package golang

var Blockchains = map[string]Blockchain{
"""
        go_content += "\n".join([
                    f'    "{chain}": {chain.capitalize()}(),'
                    for chain in blockchain_files
                ])
        go_content += "\n}\n"

        with open(go_output_file, "w", encoding="utf-8") as f:
            f.write(go_content)

        print(f"✅ Updated {go_output_file}")

    # ✅ Update Rust meta file
    elif lang == "rust":
        rust_content = """// Code generated automatically. DO NOT EDIT.

use std::collections::HashMap;
use std::sync::Arc;
use crate::blockchain::Blockchain;
"""
        rust_content += "\n".join([
            f'use crate::{chain}::{chain.capitalize()}Blockchain;'
            for chain in blockchain_files
        ]) + "\n\n"

        rust_content += """pub fn get_blockchains() -> HashMap<String, Arc<dyn Blockchain>> {
    let mut map: HashMap<String, Arc<dyn Blockchain>> = HashMap::new();
"""
        rust_content += "\n".join([
            f'    map.insert("{chain}".to_string(), Arc::new({chain.capitalize()}Blockchain {{}}));'
            for chain in blockchain_files
        ])
        rust_content += "\n    map\n}\n"

        with open(rust_output_file, "w", encoding="utf-8") as f:
            f.write(rust_content)

        print(f"✅ Updated {rust_output_file}")

    # ✅ Update TypeScript meta files
    elif lang == "ts":
        types_ts_exists = (ts_src_dir / "types.ts").exists()

        ts_meta_content = """// Code generated automatically. DO NOT EDIT.

import { Blockchain } from "./blockchain";
"""
        if types_ts_exists:
            ts_meta_content += 'import { TypesBlockchain } from "./types";\n'

        ts_meta_content += "\n".join([
            f'import {{ {chain.capitalize()}Blockchain }} from "./{chain}";'
            for chain in blockchain_files
        ]) + "\n\n"

        ts_meta_content += """export const Blockchains: Record<string, Blockchain> = {
"""
        ts_meta_content += "\n".join([
            f'    "{chain}": new {chain.capitalize()}Blockchain(),'
            for chain in blockchain_files
        ])
        ts_meta_content += "\n};\n"

        with open(ts_output_file, "w", encoding="utf-8") as f:
            f.write(ts_meta_content)

        print(f"✅ Updated {ts_output_file}")

        # ✅ Generate `index.ts`
        index_ts_content = """// Code generated automatically. DO NOT EDIT.

import { Blockchain } from "./blockchain";
import { Blockchains } from "./meta";
"""
        if types_ts_exists:
            index_ts_content += 'import { TypesBlockchain } from "./types";\n'

        index_ts_content += "\n".join([
            f'import {{ {chain.capitalize()}Blockchain }} from "./{chain}";'
            for chain in blockchain_files
        ]) + "\n\n"

        index_ts_content += """export {
    Blockchains,
    Blockchain,
"""
        if types_ts_exists:
            index_ts_content += "    TypesBlockchain,\n"

        index_ts_content += "\n".join([
            f'    {chain.capitalize()}Blockchain,'
            for chain in blockchain_files
        ])
        index_ts_content += "\n};\n"

        with open(index_ts_file, "w", encoding="utf-8") as f:
            f.write(index_ts_content)

        print(f"✅ Updated {index_ts_file}")

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
                "StructName": f"{chain_name.capitalize()}{asset_info.get('symbol', '').upper()}Asset",
                "ID": asset_info.get("id", ""),
                "Name": asset_info.get("name", ""),
                "Symbol": asset_info.get("symbol", ""),
                "Type": asset_info.get("type", ""),
                "AssetType": asset_info.get("asset_type", ""),
                "Description": asset_info.get("description", ""),
                "Website": asset_info.get("website", ""),
                "Explorer": asset_info.get("explorer", ""),
                "Decimals": asset_info.get("decimals", 0),
                "Status": asset_info.get("status", ""),
                "Icon": asset_icon
            })

    # ✅ Select output directory and file extension based on language
    template_map = {
        "go": {"template": "go.tmpl", "dir": "gen/golang", "ext": ".gen.go"},
        "rust": {"template": "rust.tmpl", "dir": "gen/rust/src", "ext": ".rs"},
        "ts": {"template": "ts.tmpl", "dir": "gen/typescript/src", "ext": ".ts"},
    }

    if language not in template_map:
        print(f"❌ Error: Unsupported language '{language}'")
        return

    template_file = template_map[language]["template"]
    output_dir = template_map[language]["dir"]
    file_extension = template_map[language]["ext"]

    # ✅ Ensure output directory exists
    os.makedirs(output_dir, exist_ok=True)

    # ✅ Load Jinja2 environment and template
    env = Environment(loader=FileSystemLoader("generator/templates"), trim_blocks=True, lstrip_blocks=True)
    try:
        template = env.get_template(template_file)
    except TemplateSyntaxError as e:
        print(f"❌ Jinja2 Template Syntax Error: {e.message} on line {e.lineno}")
        return

    # ✅ Generate the code
    try:
        generated_code = template.render(
            StructName=f"{chain_name.capitalize()}Blockchain",
            FactoryFunc=chain_name.capitalize(),
            Name=chain_info.get("name", ""),
            Description=chain_info.get("description", ""),
            Website=chain_info.get("website", ""),
            Explorer=chain_info.get("explorer", ""),
            Symbol=chain_info.get("symbol", ""),
            Decimals=chain_info.get("decimals", 0),
            Links=chain_info.get("links", []),
            Assets=assets,
            Logo=chain_logo
        )
    except Exception as e:
        print(f"❌ Jinja2 Rendering Error: {e}")
        return

    # ✅ Write to output file
    output_file = Path(output_dir) / f"{chain_name}{file_extension}"
    with open(output_file, "w", encoding="utf-8") as f:
        f.write(generated_code)

    print(f"✅ Generated {output_file}")

    # ✅ Update lib.rs for Rust
    if language == "rust":
        update_lib_rs()

    # ✅ Update global meta files (Go, Rust, TypeScript)
    update_meta_files(language)

def main():
    """Main function to process a specific chain passed as an argument."""
    parser = argparse.ArgumentParser(description="Generate Go or Rust files for a specific blockchain")
    parser.add_argument("--chain", type=str, required=True, help="Path to the blockchain directory (e.g., chains/bitcoin)")
    parser.add_argument("--lang", type=str, choices=["go", "rust", "ts"], required=True, help="Target language (go, rust, ts)")

    args = parser.parse_args()
    chain_path = Path(args.chain)

    if not chain_path.exists() or not chain_path.is_dir():
        print(f"❌ Error: Chain directory '{args.chain}' does not exist!")
        return

    process_chain(chain_path, args.lang)

if __name__ == "__main__":
    main()
