import os
import yaml
from pathlib import Path
from jinja2 import Environment, FileSystemLoader, TemplateSyntaxError

# Paths
CHAINS_DIR = "blockchains"
GEN_GOLANG_DIR = "codegen/chains-go"
GEN_RUST_DIR = "codegen/chains-rs/src"
GEN_TYPESCRIPT_DIR = "codegen/chains-ts/src"
TEMPLATE_DIR = "tools/generator/templates"
GO_TEMPLATE_FILE = "go.tmpl"
RUST_TEMPLATE_FILE = "rust.tmpl"
TS_TEMPLATE_FILE = "ts.tmpl"

# Supported languages
LANGUAGES = ["go", "rust", "ts"]

# Ensure output directories exist
os.makedirs(GEN_GOLANG_DIR, exist_ok=True)
os.makedirs(GEN_RUST_DIR, exist_ok=True)
os.makedirs(GEN_TYPESCRIPT_DIR, exist_ok=True)

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
    return svg.replace('"', '\\"')

def update_lib_rs():
    """Automatically updates lib.rs to include all generated blockchains."""
    rust_src_dir = Path(GEN_RUST_DIR)
    lib_rs_path = rust_src_dir / "lib.rs"
    blockchain_files = [f.stem for f in rust_src_dir.glob("*.rs") if f.stem not in ["lib", "types", "blockchain", "asset"]]

    content = """// Code generated automatically. DO NOT EDIT.

pub mod types;
pub mod blockchain;
pub mod asset;

""" + "\n".join([f"pub mod {blockchain};" for blockchain in blockchain_files]) + "\n"

    with open(lib_rs_path, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"✅ Updated {lib_rs_path}")

def update_meta_files(language):
    """Automatically updates meta files for the specified language."""
    go_output_file = Path(GEN_GOLANG_DIR) / "meta.go"
    rust_output_file = Path(GEN_RUST_DIR) / "meta.rs"
    ts_output_file = Path(GEN_TYPESCRIPT_DIR) / "meta.ts"
    index_ts_file = Path(GEN_TYPESCRIPT_DIR) / "index.ts"

    if language == "go":
        blockchain_files = [f.stem.replace(".gen", "") for f in Path(GEN_GOLANG_DIR).glob("*.gen.go")]
        content = """// Code generated automatically. DO NOT EDIT.

package chains

var Blockchains = map[string]Blockchain{
""" + "\n".join([f'    "{chain}": {chain.capitalize()}(),' for chain in blockchain_files]) + "\n}\n"
        output_file = go_output_file

    elif language == "rust":
        blockchain_files = [f.stem for f in Path(GEN_RUST_DIR).glob("*.rs") if f.stem not in ["lib", "types", "blockchain", "asset", "meta"]]
        content = """// Code generated automatically. DO NOT EDIT.

use std::collections::HashMap;
use std::sync::Arc;
use crate::blockchain::Blockchain;
""" + "\n".join([f'use crate::{chain}::{chain.capitalize()}Blockchain;' for chain in blockchain_files]) + """

pub fn get_blockchains() -> HashMap<String, Arc<dyn Blockchain>> {
    let mut map: HashMap<String, Arc<dyn Blockchain>> = HashMap::new();
""" + "\n".join([f'    map.insert("{chain}".to_string(), Arc::new({chain.capitalize()}Blockchain {{}}));' for chain in blockchain_files]) + """
    map
}
"""
        output_file = rust_output_file

    elif language == "ts":
        blockchain_files = [f.stem for f in Path(GEN_TYPESCRIPT_DIR).glob("*.ts") if f.stem not in ["blockchain", "asset", "meta", "index", "types"]]
        types_ts_exists = (Path(GEN_TYPESCRIPT_DIR) / "types.ts").exists()

        # meta.ts
        meta_content = """// Code generated automatically. DO NOT EDIT.

import { Blockchain } from "./blockchain";
""" + ("""import { TypesBlockchain } from "./types";
""" if types_ts_exists else "") + "\n".join([f'import {{ {chain.capitalize()}Blockchain }} from "./{chain}";' for chain in blockchain_files]) + """

export const Blockchains: Record<string, Blockchain> = {
""" + "\n".join([f'    "{chain}": new {chain.capitalize()}Blockchain(),' for chain in blockchain_files]) + """
};
"""
        with open(ts_output_file, "w", encoding="utf-8") as f:
            f.write(meta_content)
        print(f"✅ Updated {ts_output_file}")

        # index.ts
        index_content = """// Code generated automatically. DO NOT EDIT.

import { Blockchain } from "./blockchain";
import { Blockchains } from "./meta";
""" + ("""import { TypesBlockchain } from "./types";
""" if types_ts_exists else "") + "\n".join([f'import {{ {chain.capitalize()}Blockchain }} from "./{chain}";' for chain in blockchain_files]) + """

export {
    Blockchains,
    Blockchain,
""" + ("    TypesBlockchain,\n" if types_ts_exists else "") + "\n".join([f'    {chain.capitalize()}Blockchain,' for chain in blockchain_files]) + """
};
"""
        with open(index_ts_file, "w", encoding="utf-8") as f:
            f.write(index_content)
        print(f"✅ Updated {index_ts_file}")
        return

    with open(output_file, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"✅ Updated {output_file}")

def process_chain(chain_path, language):
    """Process a blockchain directory and generate a file for the specified language."""
    chain_name = chain_path.stem
    print(f"🔍 Processing {chain_name} for {language}...")

    chain_info_path = chain_path / "info.yml"
    if not chain_info_path.exists():
        print(f"❌ Error: {chain_info_path} not found!")
        return

    chain_info = read_yaml(chain_info_path)
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
                "BIP44CoinType": asset_info.get("bip44_coin_type", 0),
                "Description": asset_info.get("description", ""),
                "Website": asset_info.get("website", ""),
                "Explorer": asset_info.get("explorer", ""),
                "Decimals": asset_info.get("decimals", 0),
                "Status": asset_info.get("status", ""),
                "Icon": asset_icon
            })

    template_map = {
        "go": {"template": GO_TEMPLATE_FILE, "dir": GEN_GOLANG_DIR, "ext": ".gen.go"},
        "rust": {"template": RUST_TEMPLATE_FILE, "dir": GEN_RUST_DIR, "ext": ".rs"},
        "ts": {"template": TS_TEMPLATE_FILE, "dir": GEN_TYPESCRIPT_DIR, "ext": ".ts"},
    }

    env = Environment(loader=FileSystemLoader(TEMPLATE_DIR), trim_blocks=True, lstrip_blocks=True)
    try:
        template = env.get_template(template_map[language]["template"])
    except TemplateSyntaxError as e:
        print(f"❌ Jinja2 Template Syntax Error: {e.message} on line {e.lineno}")
        return

    output_dir = template_map[language]["dir"]
    file_extension = template_map[language]["ext"]
    output_file = Path(output_dir) / f"{chain_name}{file_extension}"

    try:
        generated_code = template.render(
            StructName=f"{chain_name.capitalize()}Blockchain",
            FactoryFunc=chain_name.capitalize(),
            Name=chain_info.get("name", ""),
            Description=chain_info.get("description", ""),
            Website=chain_info.get("website", ""),
            Explorer=chain_info.get("explorer", ""),
            Links=chain_info.get("links", []),
            Assets=assets,
            Logo=chain_logo
        )
    except Exception as e:
        print(f"❌ Jinja2 Rendering Error: {e}")
        return

    with open(output_file, "w", encoding="utf-8") as f:
        f.write(generated_code)
    print(f"✅ Generated {output_file}")

def main():
    """Process all chains in the chains directory for all languages."""
    chains_path = Path(CHAINS_DIR)
    if not chains_path.exists() or not chains_path.is_dir():
        print(f"❌ Error: Chains directory '{CHAINS_DIR}' does not exist!")
        return

    for chain_path in chains_path.iterdir():
        if chain_path.is_dir():
            for language in LANGUAGES:
                process_chain(chain_path, language)

    # Update meta files for all languages
    for language in LANGUAGES:
        if language == "rust":
            update_lib_rs()
        update_meta_files(language)

if __name__ == "__main__":
    main()