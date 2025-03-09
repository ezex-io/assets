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

import { Blockchain, Asset, Link } from "./blockchain";

// Binance Smart Chain Assets Implementation
export const BinanceBNBAsset = new Asset({
    id: "binance_bnb",
    name: "Binance",
    address: "",
    symbol: "BNB",
    type: "NATIVE",
    assetType: "coin",
    bip44CoinType: 714,
    website: "https://binance.org",
    explorer: "https://explorer.binance.org/",
    decimals: 18,
    description: `Fast and secure decentralized digital asset exchange. The new crypto currency trading standard is here.
`,
    icon: `<?xml version=\"1.0\" encoding=\"utf-8\"?>
<!-- Generator: Adobe Illustrator 26.0.1, SVG Export Plug-In . SVG Version: 6.00 Build 0)  -->
<svg version=\"1.1\" id=\"Layer_1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" x=\"0px\" y=\"0px\"
	 viewBox=\"0 0 2496 2496\" style=\"enable-background:new 0 0 2496 2496;\" xml:space=\"preserve\">
<g>
	<path style=\"fill-rule:evenodd;clip-rule:evenodd;fill:#F0B90B;\" d=\"M1248,0c689.3,0,1248,558.7,1248,1248s-558.7,1248-1248,1248
		S0,1937.3,0,1248S558.7,0,1248,0L1248,0z\"/>
	<path style=\"fill:#FFFFFF;\" d=\"M685.9,1248l0.9,330l280.4,165v193.2l-444.5-260.7v-524L685.9,1248L685.9,1248z M685.9,918v192.3
		l-163.3-96.6V821.4l163.3-96.6l164.1,96.6L685.9,918L685.9,918z M1084.3,821.4l163.3-96.6l164.1,96.6L1247.6,918L1084.3,821.4
		L1084.3,821.4z\"/>
	<path style=\"fill:#FFFFFF;\" d=\"M803.9,1509.6v-193.2l163.3,96.6v192.3L803.9,1509.6L803.9,1509.6z M1084.3,1812.2l163.3,96.6
		l164.1-96.6v192.3l-164.1,96.6l-163.3-96.6V1812.2L1084.3,1812.2z M1645.9,821.4l163.3-96.6l164.1,96.6v192.3l-164.1,96.6V918
		L1645.9,821.4L1645.9,821.4L1645.9,821.4z M1809.2,1578l0.9-330l163.3-96.6v524l-444.5,260.7v-193.2L1809.2,1578L1809.2,1578
		L1809.2,1578z\"/>
	<polygon style=\"fill:#FFFFFF;\" points=\"1692.1,1509.6 1528.8,1605.3 1528.8,1413 1692.1,1316.4 1692.1,1509.6 	\"/>
	<path style=\"fill:#FFFFFF;\" d=\"M1692.1,986.4l0.9,193.2l-281.2,165v330.8l-163.3,95.7l-163.3-95.7v-330.8l-281.2-165V986.4
		L968,889.8l279.5,165.8l281.2-165.8l164.1,96.6H1692.1L1692.1,986.4z M803.9,656.5l443.7-261.6l444.5,261.6l-163.3,96.6
		l-281.2-165.8L967.2,753.1L803.9,656.5L803.9,656.5z\"/>
</g>
</svg>
`
});

// Binance Smart Chain Assets Implementation
export const BinanceUSDTAsset = new Asset({
    id: "binance_usdt",
    name: "Tether USD",
    address: "0x55d398326f99059fF775485246999027B3197955",
    symbol: "USDT",
    type: "BEP20",
    assetType: "token",
    bip44CoinType: -1,
    website: "https://tether.to",
    explorer: "https://bscscan.com/token/0x55d398326f99059fF775485246999027B3197955",
    decimals: 18,
    description: `Tether gives you the joint benefits of open blockchain technology and traditional currency by converting your cash into a stable digital currency equivalent.
`,
    icon: `<svg id=\"Layer_1\" data-name=\"Layer 1\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 339.43 295.27\"><title>tether-usdt-logo</title><path d=\"M62.15,1.45l-61.89,130a2.52,2.52,0,0,0,.54,2.94L167.95,294.56a2.55,2.55,0,0,0,3.53,0L338.63,134.4a2.52,2.52,0,0,0,.54-2.94l-61.89-130A2.5,2.5,0,0,0,275,0H64.45a2.5,2.5,0,0,0-2.3,1.45h0Z\" style=\"fill:#50af95;fill-rule:evenodd\"/><path d=\"M191.19,144.8v0c-1.2.09-7.4,0.46-21.23,0.46-11,0-18.81-.33-21.55-0.46v0c-42.51-1.87-74.24-9.27-74.24-18.13s31.73-16.25,74.24-18.15v28.91c2.78,0.2,10.74.67,21.74,0.67,13.2,0,19.81-.55,21-0.66v-28.9c42.42,1.89,74.08,9.29,74.08,18.13s-31.65,16.24-74.08,18.12h0Zm0-39.25V79.68h59.2V40.23H89.21V79.68H148.4v25.86c-48.11,2.21-84.29,11.74-84.29,23.16s36.18,20.94,84.29,23.16v82.9h42.78V151.83c48-2.21,84.12-11.73,84.12-23.14s-36.09-20.93-84.12-23.15h0Zm0,0h0Z\" style=\"fill:#fff;fill-rule:evenodd\"/></svg>`
});


// Binance Smart Chain Blockchain Implementation
export const BinanceBlockchain = new Blockchain({
    name: "Binance Smart Chain",
    website: "https://binance.org/",
    explorer: "https://explorer.binance.org/",
    links: [
        new Link({ name: "github", url: "https://github.com/binance-chain/" }),
        new Link({ name: "twitter", url: "https://twitter.com/binance_dex" }),
        new Link({ name: "reddit", url: "https://reddit.com/r/BinanceExchange" }),
        new Link({ name: "whitepaper", url: "https://www.binance.com/resources/ico/Binance_WhitePaper_en.pdf" }),
    ],
    assets: [
        BinanceBNBAsset,
        BinanceUSDTAsset,
    ],
    description: `Fast and secure decentralized digital asset exchange. The new crypto currency trading standard is here.
`,
    logo: `<?xml version=\"1.0\" encoding=\"utf-8\"?>
<!-- Generator: Adobe Illustrator 26.0.1, SVG Export Plug-In . SVG Version: 6.00 Build 0)  -->
<svg version=\"1.1\" id=\"Layer_1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" x=\"0px\" y=\"0px\"
	 viewBox=\"0 0 2496 2496\" style=\"enable-background:new 0 0 2496 2496;\" xml:space=\"preserve\">
<g>
	<path style=\"fill-rule:evenodd;clip-rule:evenodd;fill:#F0B90B;\" d=\"M1248,0c689.3,0,1248,558.7,1248,1248s-558.7,1248-1248,1248
		S0,1937.3,0,1248S558.7,0,1248,0L1248,0z\"/>
	<path style=\"fill:#FFFFFF;\" d=\"M685.9,1248l0.9,330l280.4,165v193.2l-444.5-260.7v-524L685.9,1248L685.9,1248z M685.9,918v192.3
		l-163.3-96.6V821.4l163.3-96.6l164.1,96.6L685.9,918L685.9,918z M1084.3,821.4l163.3-96.6l164.1,96.6L1247.6,918L1084.3,821.4
		L1084.3,821.4z\"/>
	<path style=\"fill:#FFFFFF;\" d=\"M803.9,1509.6v-193.2l163.3,96.6v192.3L803.9,1509.6L803.9,1509.6z M1084.3,1812.2l163.3,96.6
		l164.1-96.6v192.3l-164.1,96.6l-163.3-96.6V1812.2L1084.3,1812.2z M1645.9,821.4l163.3-96.6l164.1,96.6v192.3l-164.1,96.6V918
		L1645.9,821.4L1645.9,821.4L1645.9,821.4z M1809.2,1578l0.9-330l163.3-96.6v524l-444.5,260.7v-193.2L1809.2,1578L1809.2,1578
		L1809.2,1578z\"/>
	<polygon style=\"fill:#FFFFFF;\" points=\"1692.1,1509.6 1528.8,1605.3 1528.8,1413 1692.1,1316.4 1692.1,1509.6 	\"/>
	<path style=\"fill:#FFFFFF;\" d=\"M1692.1,986.4l0.9,193.2l-281.2,165v330.8l-163.3,95.7l-163.3-95.7v-330.8l-281.2-165V986.4
		L968,889.8l279.5,165.8l281.2-165.8l164.1,96.6H1692.1L1692.1,986.4z M803.9,656.5l443.7-261.6l444.5,261.6l-163.3,96.6
		l-281.2-165.8L967.2,753.1L803.9,656.5L803.9,656.5z\"/>
</g>
</svg>
`,
})