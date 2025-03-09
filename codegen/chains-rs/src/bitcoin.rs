/*
MIT License

Copyright (c) 2025 ezeX

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

// Code generated automatically. DO NOT EDIT.

use crate::types::{AssetType, Link};
use crate::blockchain::Blockchain;
use crate::asset::Asset;

/// Bitcoin Blockchain Implementation
pub struct BitcoinBlockchain;

const LOGO_SVG: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
<!-- Creator: CorelDRAW 2019 (64-Bit) -->
<svg xmlns=\"http://www.w3.org/2000/svg\" xml:space=\"preserve\" width=\"100%\" height=\"100%\" version=\"1.1\" shape-rendering=\"geometricPrecision\" text-rendering=\"geometricPrecision\" image-rendering=\"optimizeQuality\" fill-rule=\"evenodd\" clip-rule=\"evenodd\"
viewBox=\"0 0 4091.27 4091.73\"
 xmlns:xlink=\"http://www.w3.org/1999/xlink\"
 xmlns:xodm=\"http://www.corel.com/coreldraw/odm/2003\">
 <g id=\"Layer_x0020_1\">
  <metadata id=\"CorelCorpID_0Corel-Layer\"/>
  <g id=\"_1421344023328\">
   <path fill=\"#F7931A\" fill-rule=\"nonzero\" d=\"M4030.06 2540.77c-273.24,1096.01 -1383.32,1763.02 -2479.46,1489.71 -1095.68,-273.24 -1762.69,-1383.39 -1489.33,-2479.31 273.12,-1096.13 1383.2,-1763.19 2479,-1489.95 1096.06,273.24 1763.03,1383.51 1489.76,2479.57l0.02 -0.02z\"/>
   <path fill=\"white\" fill-rule=\"nonzero\" d=\"M2947.77 1754.38c40.72,-272.26 -166.56,-418.61 -450,-516.24l91.95 -368.8 -224.5 -55.94 -89.51 359.09c-59.02,-14.72 -119.63,-28.59 -179.87,-42.34l90.16 -361.46 -224.36 -55.94 -92 368.68c-48.84,-11.12 -96.81,-22.11 -143.35,-33.69l0.26 -1.16 -309.59 -77.31 -59.72 239.78c0,0 166.56,38.18 163.05,40.53 90.91,22.69 107.35,82.87 104.62,130.57l-104.74 420.15c6.26,1.59 14.38,3.89 23.34,7.49 -7.49,-1.86 -15.46,-3.89 -23.73,-5.87l-146.81 588.57c-11.11,27.62 -39.31,69.07 -102.87,53.33 2.25,3.26 -163.17,-40.72 -163.17,-40.72l-111.46 256.98 292.15 72.83c54.35,13.63 107.61,27.89 160.06,41.3l-92.9 373.03 224.24 55.94 92 -369.07c61.26,16.63 120.71,31.97 178.91,46.43l-91.69 367.33 224.51 55.94 92.89 -372.33c382.82,72.45 670.67,43.24 791.83,-303.02 97.63,-278.78 -4.86,-439.58 -206.26,-544.44 146.69,-33.83 257.18,-130.31 286.64,-329.61l-0.07 -0.05zm-512.93 719.26c-69.38,278.78 -538.76,128.08 -690.94,90.29l123.28 -494.2c152.17,37.99 640.17,113.17 567.67,403.91zm69.43 -723.3c-63.29,253.58 -453.96,124.75 -580.69,93.16l111.77 -448.21c126.73,31.59 534.85,90.55 468.94,355.05l-0.02 0z\"/>
  </g>
 </g>
</svg>
";

impl Blockchain for BitcoinBlockchain {
    fn name(&self) -> &str {
        "Bitcoin"
    }

    fn description(&self) -> &str {
        "Bitcoin is a cryptocurrency and worldwide payment system. It is the first decentralized digital currency, as the system works without a central bank or single administrator.
"
    }

    fn website(&self) -> &str {
        "https://bitcoin.org"
    }

    fn explorer(&self) -> &str {
        "https://blockchain.info"
    }

    fn links(&self) -> Vec<Link> {
        vec![
            Link { name: "github".to_string(), url: "https://github.com/bitcoin".to_string() },
            Link { name: "twitter".to_string(), url: "https://twitter.com/Bitcoin".to_string() },
            Link { name: "reddit".to_string(), url: "https://reddit.com/r/Bitcoin".to_string() },
            Link { name: "whitepaper".to_string(), url: "https://bitcoin.org/bitcoin.pdf".to_string() },
        ]
    }

    fn assets(&self) -> Vec<Box<dyn Asset>> {
        vec![
            Box::new(BitcoinBTCAsset),
        ]
    }

    fn asset(&self, id: &str) -> Option<Box<dyn Asset>> {
        match id {
            "bitcoin_btc" => Some(Box::new(BitcoinBTCAsset)),
            _ => None,
        }
    }

    fn logo(&self) -> &str {
            LOGO_SVG
    }
}

/// Bitcoin Asset Implementation
pub struct BitcoinBTCAsset;

const ICON_BITCOINBTCASSET: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
<!-- Creator: CorelDRAW 2019 (64-Bit) -->
<svg xmlns=\"http://www.w3.org/2000/svg\" xml:space=\"preserve\" width=\"100%\" height=\"100%\" version=\"1.1\" shape-rendering=\"geometricPrecision\" text-rendering=\"geometricPrecision\" image-rendering=\"optimizeQuality\" fill-rule=\"evenodd\" clip-rule=\"evenodd\"
viewBox=\"0 0 4091.27 4091.73\"
 xmlns:xlink=\"http://www.w3.org/1999/xlink\"
 xmlns:xodm=\"http://www.corel.com/coreldraw/odm/2003\">
 <g id=\"Layer_x0020_1\">
  <metadata id=\"CorelCorpID_0Corel-Layer\"/>
  <g id=\"_1421344023328\">
   <path fill=\"#F7931A\" fill-rule=\"nonzero\" d=\"M4030.06 2540.77c-273.24,1096.01 -1383.32,1763.02 -2479.46,1489.71 -1095.68,-273.24 -1762.69,-1383.39 -1489.33,-2479.31 273.12,-1096.13 1383.2,-1763.19 2479,-1489.95 1096.06,273.24 1763.03,1383.51 1489.76,2479.57l0.02 -0.02z\"/>
   <path fill=\"white\" fill-rule=\"nonzero\" d=\"M2947.77 1754.38c40.72,-272.26 -166.56,-418.61 -450,-516.24l91.95 -368.8 -224.5 -55.94 -89.51 359.09c-59.02,-14.72 -119.63,-28.59 -179.87,-42.34l90.16 -361.46 -224.36 -55.94 -92 368.68c-48.84,-11.12 -96.81,-22.11 -143.35,-33.69l0.26 -1.16 -309.59 -77.31 -59.72 239.78c0,0 166.56,38.18 163.05,40.53 90.91,22.69 107.35,82.87 104.62,130.57l-104.74 420.15c6.26,1.59 14.38,3.89 23.34,7.49 -7.49,-1.86 -15.46,-3.89 -23.73,-5.87l-146.81 588.57c-11.11,27.62 -39.31,69.07 -102.87,53.33 2.25,3.26 -163.17,-40.72 -163.17,-40.72l-111.46 256.98 292.15 72.83c54.35,13.63 107.61,27.89 160.06,41.3l-92.9 373.03 224.24 55.94 92 -369.07c61.26,16.63 120.71,31.97 178.91,46.43l-91.69 367.33 224.51 55.94 92.89 -372.33c382.82,72.45 670.67,43.24 791.83,-303.02 97.63,-278.78 -4.86,-439.58 -206.26,-544.44 146.69,-33.83 257.18,-130.31 286.64,-329.61l-0.07 -0.05zm-512.93 719.26c-69.38,278.78 -538.76,128.08 -690.94,90.29l123.28 -494.2c152.17,37.99 640.17,113.17 567.67,403.91zm69.43 -723.3c-63.29,253.58 -453.96,124.75 -580.69,93.16l111.77 -448.21c126.73,31.59 534.85,90.55 468.94,355.05l-0.02 0z\"/>
  </g>
 </g>
</svg>
";

impl Asset for BitcoinBTCAsset {
    fn id(&self) -> &str {
        "bitcoin_btc"
    }

    fn name(&self) -> &str {
        "Bitcoin"
    }

    fn address(&self) -> &str {
        ""
    }

    fn symbol(&self) -> &str {
        "BTC"
    }

    fn asset_type(&self) -> AssetType {
        "coin"
    }

    fn asset_kind(&self) -> &str {
        "NATIVE"
    }

    fn bip44_coin_type(&self) -> isize {
        0
    }

    fn description(&self) -> &str {
        "Bitcoin is a cryptocurrency and worldwide payment system. It is the first decentralized digital currency, as the system works without a central bank or single administrator.
"
    }

    fn website(&self) -> &str {
        "https://bitcoin.org"
    }

    fn explorer(&self) -> &str {
        "https://blockchain.info"
    }

    fn decimals(&self) -> u32 {
        8
    }

    fn icon(&self) -> &str {
            ICON_BITCOINBTCASSET
    }
}
