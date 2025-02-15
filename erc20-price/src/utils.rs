use lazy_static;
use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref TOKENS: HashMap<&'static str, &'static str> = {
        let token_mapping: HashMap<&str, &str> = HashMap::from([
            ("USD", "0x0000000000000000000000000000000000000348"),
            ("CRO", "0xa0b73e1ff0b80914ab6fe0444e65848c4c34450b"),
            ("STMX", "0xbe9375c6a420d2eeb258962efb95551a5b722803"),
            ("SRM", "0x476c5e26a75bd202a9683ffd34359c0cc15be0ff"),
            ("ETH", "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
            ("BAND", "0xba11d00c5f74255f56a5e366f4f77f5a186d7f55"),
            ("ERN", "0xbbc2ae13b23d715c30720f079fcd9b4a74093505"),
            ("COVER", "0x4688a8b1f292fdab17e9a90c8bc379dc1dbd8713"),
            ("BAT", "0x0d8775f648430679a709e98d2b0cb6250d2887ef"),
            ("REN", "0x408e41876cccdc0f92210600ef50372656052a38"),
            ("WING", "0xcb3df3108635932d912632ef7132d03ecfc39080"),
            ("LON", "0x0000000000095413afc295d19edeb1ad7b71c952"),
            ("FRAX", "0x853d955acef822db058eb8505911ed77f175b99e"),
            ("LRC", "0xbbbbca6a901c926f240b89eacb641d8aec7aeafd"),
            ("GRT", "0xc944e90c64b2c07662a292be6244bdf05cda44a7"),
            ("COMP", "0xc00e94cb662c3520282e6f5717214004a7f26888"),
            ("HUSD", "0xdf574c24545e5ffecb9a659c229253d4111d87e1"),
            ("BNT", "0x1f573d6fb3f13d689ff844b4ce37794d79a7ff1c"),
            ("DNT", "0x0abdace70d3790235af448c88547603b945604ea"),
            ("OKB", "0x75231f58b43240c9718dd58b4967c5114342a86c"),
            ("ADX", "0xade00c28244d5ce17d72e40330b1c318cd12b7c3"),
            ("MKR", "0x9f8f72aa9304c8b593d555f12ef6589cc3a579a2"),
            ("ENJ", "0xf629cbd94d3791c9250152bd8dfbdf380e2a3b9c"),
            ("TRU", "0x4c19596f5aaff459fa38b0f7ed92f11ae6543784"),
            ("ZRX", "0xe41d2489571d322189246dafa5ebde1f4699f498"),
            ("FTM", "0x4e15361fd6b4bb609fa63c81a2be19d873717870"),
            ("RARI", "0xfca59cd816ab1ead66534d82bc21e7515ce441cf"),
            ("LINK", "0x514910771af9ca656af840dff83e8264ecf986ca"),
            ("OGN", "0x8207c1ffc5b6804f6024322ccf34f29c3541ae26"),
            ("SAND", "0x3845badade8e6dff049820680d1f14bd3903a5d0"),
            ("TUSD", "0x0000000000085d4780b73119b644ae5ecd22b376"),
            ("USDT", "0xdac17f958d2ee523a2206206994597c13d831ec7"),
            ("PAX", "0x8e870d67f660d95d5be530380d0ec0bd388289e1"),
            ("PERP", "0xbc396689893d065f41bc2c6ecbee5e0085233447"),
            ("DIGG", "0x798d1be841a82a273720ce31c822c61a67a601c3"),
            ("BTC", "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599"),
            ("RUNE", "0x3155ba85d5f96b2d030a4966af206230e46849cb"),
            ("AMPL", "0xd46ba6d942050d489dbd938a2c909a5d5039a161"),
            ("RAMP", "0x33d0568941c0c64ff7e0fb4fba0b11bd37deed9f"),
            ("RAI", "0x03ab458634910aad20ef5f1c8ee96f1d6ac54919"),
            ("RLC", "0x607f4c5bb672230e8672085532f7e901544a7375"),
            ("LDO", "0x5a98fcbea516cf06857215779fd812ca3bef1b32"),
            ("AAVE", "0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9"),
            ("UNI", "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984"),
            ("OMG", "0xd26114cd6ee289accf82350c8d8487fedb8a0c07"),
            ("BADGER", "0x3472a5a71965499acd81997a54bba8d852c6e53d"),
            ("BUSD", "0x4fabb145d64652a948d72533023f6e7a623c7c53"),
            ("KNC", "0xdd974d5c2e2928dea5f71b9825b8b646686bd200"),
            ("FXS", "0x3432b6a60d23ca0dfca7761b7ab56459d9c964d0"),
            ("1INCH", "0x111111111117dc0aa78b770fa6a738034120c302"),
            ("CEL", "0xaaaebe6fe48e54f431b0c390cfaf0b017d09d42d"),
            ("DAI", "0x6b175474e89094c44da98b954eedeac495271d0f"),
            ("SNX", "0xc011a73ee8576fb46f5e1c5751ca3b9fe0af2a6f"),
            ("FEI", "0x956f47f50a910163d8bf957cf5846d573e7f87ca"),
            ("USDN", "0x674c6ad92fd080e4004b2312b45f796a192d27a0"),
            ("MATIC", "0x7d1afa7b718fb893db30a3abc0cfc608aacfebb0"),
            ("YFI", "0x0bc529c00c6401aef6d220be8c6ea1667f6ad93e"),
            ("OCEAN", "0x967da4048cd07ab37855c090aaf366e4ce1b9f48"),
            ("ANKR", "0x8290333cef9e6d528dd5618fb97a76f268f3edd4"),
            ("CREAM", "0x2ba592f78db6436527729929aaf6c908497cb200"),
            ("MANA", "0x0f5d2fb29fb7d3cfee444a200298f468908cc942"),
            ("TRIBE", "0xc7283b66eb1eb5fb86327f08e1b5816b0720212b"),
            ("AMP", "0xff20817765cb7f73d4bde2e66e067e58d11095c2"),
            ("ALPHA", "0xa1faa113cbe53436df28ff0aee54275c13b40975"),
            ("CRV", "0xd533a949740bb3306d119cc777fa900ba034cd52"),
            ("BZRX", "0x56d811088235f11c8920698a204a5010a788f4b3"),
            ("ANT", "0xa117000000f279d81a1d3cc75430faa017fa5a2e"),
            ("USDC", "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
            ("WOO", "0x4691937a7508860f876c9c0a2a617e7d9e945d4b"),
            ("MTA", "0xa3bed4e1c75d00fa6f4e5e6922db7261b5e9acd2"),
            ("VSP", "0x1b40183efb4dd766f11bda7a7c3ad8982e998421"),
            ("PAXG", "0x45804880de22913dafe09f4980848ece6ecbaf78"),
            ("NMR", "0x1776e1f26f98b1a5df9cd347953a26dd3cb46671"),
            ("GNO", "0x6810e776880c02933d47db1b9fc05908e5386b96"),
            ("AUCTION", "0xa9b1eb5908cfc3cdf91f9b8b3a74108598009096"),
            ("HEGIC", "0x584bc13c7d411c00c01a62e8019472de68768430"),
            ("TRY", "0xc12ecee46ed65d970ee5c899fcc7ae133aff9b03"),
            ("AKRO", "0x8ab7404063ec4dbcfd4598215992dc3f8ec853d7"),
            ("BAL", "0xba100000625a3754423978a60c9317c58a424e3d"),
            ("SUSHI", "0x6b3595068778dd592e39a122f4f5a5cf09c90fe2"),
            ("REP", "0x221657776846890989a759ba2973e427dff5c9bb"),
            ("MLN", "0xec67005c4e498ec7f55e092bd1d35cbc47c91892"),
            ("HT", "0x6f259637dcd74c767781e37bc6133cd6a68aa161"),
            ("RCN", "0xf970b8e36e23f7fc3fd752eea86f8be8d83375a6"),
            ("FTT", "0x50d1c9771902476076ecfc8b2a83ad6b9355a4c9"),
            ("SXP", "0x8ce9137d39326ad0cd6491fb5cc0cba0e089b6a9"),
            ("NU", "0x4fe83213d56308330ec302a8bd641f1d0113a4cc"),
            ("STAKE", "0x0ae055097c6d159879521c384f1d2123d1f195e6"),
            ("UST", "0xa47c8bf37f92abed4a126bda807a7b7498661acd"),
            ("sDEFI", "0xe1afe1fd76fd88f78cbf599ea1846231b8ba3b6b"),
            ("INJ", "0xe28b3b32b6c345a34ff64674606124dd5aceca30"),
            ("YFII", "0xa1d0e215a23d7030842fc67ce582a6afa3ccab83"),
            ("sUSD", "0x57ab1ec28d129707052df4df418d58a2d46d5f51"),
            ("ORN", "0x0258f474786ddfd37abce6df6bbb1dd5dfc4434a"),
            ("FRONT", "0xf8c3527cc04340b208c854e985240c02f7b7793f"),
            ("RGT", "0xd291e7a03283640fdc51b121ac401383a46cc623"),
            ("WOM", "0xbd356a39bff2cada8e9248532dd879147221cf76"),
            ("OXT", "0x4575f41308ec1483f3d399aa9a2826d74da13deb"),
            ("BOND", "0x0391d2021f89dc339f60fff84546ea23e337750f"),
            ("KP3R", "0x1ceb5cb57c4d4e2b2433641b95dd330a33185a44"),
            ("SFI", "0xb753428af26e81097e7fd17f40c88aaa3e04902c"),
            ("DIA", "0x84ca8bc7997272c7cfb4d0cd3d55cd942b3c9419"),
            ("UMA", "0x04fa0d235c4abf4bcf4787af4cf447de572ef828"),
            ("SWAP", "0xcc4304a31d09258b0029ea7fe63d032f52e44efe"),
            ("DPI", "0x1494ca1f11d487c2bbe4543e90080aeba4ba3c2b"),
            ("SUSD", "0x57ab1ec28d129707052df4df418d58a2d46d5f51"),
            ("WNXM", "0x0d438f3b5175bebc262bf23753c1e53d03432bde"),
            ("ALCX", "0xdbdb4d16eda451d0503b854cf79d55697f90c8df"),
            ("LUSD", "0x5f98805a4e8be255a32880fdec7f6728c6568ba0"),
            ("GHST", "0x3f382dbd960e3a9bbceae22651e88158d2791550"),
            ("FARM", "0xa0246c9032bc3a600820415ae600c6388619a14d"),
            ("STETH", "0xae7ab96520de3a18e5e111b5eaab095312d7fe84"),
            ("OHM", "0x383518188c0c6d7730d91b2c03a03c837814a899"),
            ("ZCN", "0xb9ef770b6a5e12e45983c5d80545258aa38f3b78"),
            ("OM", "0x3593d125a4f7849a1b059e64f4517a86dd60c95d"),
            ("USDK", "0x1c48f86ae57291f7686349f12601910bd8d470bb"),
            ("ACH", "0xed04915c23f00a313a544955524eb7dbd823143d"),
            ("USDP", "0x8e870d67f660d95d5be530380d0ec0bd388289e1"),
            ("ATA", "0xa2120b9e674d3fc3875f415a7df52e382f141225"),
            ("GUSD", "0x056fd409e1d7a124bd7017459dfea2f387b6d5cd"),
            ("AUDIO", "0x18aaa7115705e8be94bffebde57af9bfc265b998"),
            ("MASK", "0x69af81e73a73b40adf4f3d4223cd9b1ece623074"),
            ("FET", "0xaea46a60368a7bd060eec7df8cba43b7ef41ad85"),
            ("AXS", "0xbb0e17ef65f82ab018d8edd776e8dd940327b28b"),
            ("EURT", "0xc581b735a1688071a1746c968e0798d642ede491"),
            ("CTSI", "0x491604c0fdf08347dd1fa4ee062a822a5dd06b5d"),
            ("XSUSHI", "0x8798249c2e607446efb7ad49ec89dd1865ff4272"),
            ("LUNA", "0xd2877702675e6ceb975b4a1dff9fb7baf4c91ea9"),
            ("ALUSD", "0xbc6da0fe9ad5f3b0d58160288917aa56653660e9"),
            ("CELR", "0x4f9254c83eb525f9fcf346490bbb3ed28a81c667"),
            ("BORING", "0xbc19712feb3a26080ebf6f2f7849b417fdd792ca"),
            ("GTC", "0xde30da39c46104798bb5aa3fe8b9e0e1f348163f"),
            ("YGG", "0x25f8087ead173b73d6e8b84329989a8eea16cf73"),
            ("DODO", "0x43dfc4159d86f3a37a5a4b3d4580b888ad7d4ddd"),
            ("FLOKI", "0x43f11c02439e2736800433b4594994bd43cd066d"),
            ("SPELL", "0x090185f2135308bad17527004364ebcc2d37e5f6"),
            ("DYDX", "0x92d6c1e31e14520e676a687f0a93788b716beff5"),
            ("DATA", "0x33d63ba1e57e54779f7ddaeaa7109349344cf5f1"),
            ("FORTH", "0x77fba179c79de5b7653f68b5039af940ada60ce0"),
            ("XCN", "0xa2cd3d43c775978a96bdbf12d733d5a1ed94fb18"),
        ]);

        token_mapping
    };
}
