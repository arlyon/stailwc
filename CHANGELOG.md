# [0.6.0](https://github.com/arlyon/stailwc/compare/0.5.2...0.6.0) (2022-08-26)


### Features

* add border-btlr ([7627f60](https://github.com/arlyon/stailwc/commit/7627f6096e0a8b9b6170f3b097f1891f538c29bf))
* add overflow-x and overflow-y ([52640b5](https://github.com/arlyon/stailwc/commit/52640b518276e9c5432df0ae162975f25297cc51))
* add rotate plugin ([938c4ed](https://github.com/arlyon/stailwc/commit/938c4ed848a019a0e5c6a676ce785024c157b24c))
* add rounded t, b, l, r, tl, tr, bl, br ([3ec0789](https://github.com/arlyon/stailwc/commit/3ec07896267b7695092f3c296c241066956bf141))
* add text decoration ([d4990ed](https://github.com/arlyon/stailwc/commit/d4990edae087d495a8d3d3279924e0fb669b54c0))

## [0.5.2](https://github.com/arlyon/stailwc/compare/0.5.1...0.5.2) (2022-08-25)


### Bug Fixes

* add wasm32-wasi to toolchain ([c5739e5](https://github.com/arlyon/stailwc/commit/c5739e5bb6f754cfafaab2cd286ade7740f17c3a))
* ignore Cargo.lock file ([dd928b1](https://github.com/arlyon/stailwc/commit/dd928b1f81b519644dc9e7654abe325073b8ff64))

## [0.5.1](https://github.com/arlyon/stailwc/compare/0.5.0...0.5.1) (2022-08-25)


### Bug Fixes

* release metadata ([8dd82cb](https://github.com/arlyon/stailwc/commit/8dd82cbdf5d90cb3c0e276719f3019025b6b3b75))

# [0.5.0](https://github.com/arlyon/stailwc/compare/0.4.0...0.5.0) (2022-08-25)


### Bug Fixes

* fix border-box ([e2e5b9c](https://github.com/arlyon/stailwc/commit/e2e5b9cc9e0e758682437aabc5c8f1d44b087952))
* fix z index ([9868a57](https://github.com/arlyon/stailwc/commit/9868a57f19ba7b139be6735529ea20e6c08aa54d))
* justify plugin values ([4555051](https://github.com/arlyon/stailwc/commit/4555051b9d66b72a9788ab3db3007febbe9beb30))
* line height config parsing is incorrect ([820e59c](https://github.com/arlyon/stailwc/commit/820e59c87ea174fab02c51a9354b6b662a0cad4d))
* proper string escaping for object literal idents ([bc51e4b](https://github.com/arlyon/stailwc/commit/bc51e4b9964987bd4d4a7777659012a761e6d8da))
* ring color could never match here ([656fd00](https://github.com/arlyon/stailwc/commit/656fd000604b87a457067e8977b882e7d2745814))
* support different screen configs ([5849805](https://github.com/arlyon/stailwc/commit/584980554c317646ea223009e96ec0c83b9d3a0e))
* use ident when possible ([414842f](https://github.com/arlyon/stailwc/commit/414842f6c9a7b525c6d0ba13260698e023d7c780))


### Features

* add a number of modifiers ([9ef0142](https://github.com/arlyon/stailwc/commit/9ef014267cd5b81c49c166413b8133a0464ce95a))
* add basic divide plugin ([0b6072a](https://github.com/arlyon/stailwc/commit/0b6072a804f63b6d62dfe39db0f37812a459947f))
* add basic space plugin ([37263e7](https://github.com/arlyon/stailwc/commit/37263e79a6f98f46d67c7b1dc60cfe06db59113f))
* add placeholder plugin ([31c674d](https://github.com/arlyon/stailwc/commit/31c674d526c9a2d023209eec9acea0b921b5b06a))
* add text-transform plugin ([64a3663](https://github.com/arlyon/stailwc/commit/64a36635fb42b6d274d87e32f6593bfe13f88ea3))
* dark mode support ([f43f9c0](https://github.com/arlyon/stailwc/commit/f43f9c023f4a7f512037f216687aed2b20968c9c))


### [0.4.0](https://github.com/arlyon/stailwc/compare/0.3.5...0.3.6) (2022-08-25)


### Bug Fixes

* allow optional borderWidth ([97b7233](https://github.com/arlyon/stailwc/commit/97b7233c7fcf7c32e3fe6eccebdc484c1273ec74))
* use proper margin and padding config ([cf5a2d7](https://github.com/arlyon/stailwc/commit/cf5a2d7baedc343b42a13465b83337a69d3a2ca9))


### Features

* add appearance plugin ([7d77cb2](https://github.com/arlyon/stailwc/commit/7d77cb2e1e63cad416e3c4f8a690d56f62118198))
* add backgroundImage to bg plugin ([32f5be5](https://github.com/arlyon/stailwc/commit/32f5be5f85f38ff5da27fc6137139801bf5b1f72))
* add blur plugin ([d840025](https://github.com/arlyon/stailwc/commit/d840025f5863198b36e3b4e0a73f25fcf7aff1e4))
* add from and to for gradients ([6f86b4e](https://github.com/arlyon/stailwc/commit/6f86b4edcebe1e6588fa4e485ae8a10167af3ace))
* add invert plugin ([da6a27f](https://github.com/arlyon/stailwc/commit/da6a27f100797f25ed265c1407a6ed51c4f98f03))
* add max-h plugin ([72a86f1](https://github.com/arlyon/stailwc/commit/72a86f1ff3cbe73edbef5c7150eddde170f6edf8))
* add max-w plugin ([496e0c4](https://github.com/arlyon/stailwc/commit/496e0c42a5aef2348d13a6a2bdcbef0041a03a85))
* add min-w plugin ([82d879d](https://github.com/arlyon/stailwc/commit/82d879d16b1ec8893c6fbf1aeff3bb0b4b68d13a))
* add mix blend ([0563fa6](https://github.com/arlyon/stailwc/commit/0563fa67a5a51de74e687fe035c24fc9f664a6b3))
* add opacity plugin ([dc64ba1](https://github.com/arlyon/stailwc/commit/dc64ba139480693446ad08634cbb7f402e9aa595))
* add order plugin ([d6aa5ca](https://github.com/arlyon/stailwc/commit/d6aa5ca3650fd8b7471b972f9957e500e575439e))
* add outline ([b6e0cf9](https://github.com/arlyon/stailwc/commit/b6e0cf9090bb5f0f5ffdfc347a1a2159e9bcb9f6))
* add ring inset and offeset ([3f641ca](https://github.com/arlyon/stailwc/commit/3f641ca88cb7bc24fbd77438686838a7ab137ee1))
* add ring plugin ([45531b8](https://github.com/arlyon/stailwc/commit/45531b82ca9e1e7932184e649699ce8433c64f93))
* add transform plugin ([9cf40b7](https://github.com/arlyon/stailwc/commit/9cf40b76edd7877c23f09a5e211194ca5bd48573))


### [0.3.5](https://github.com/arlyon/stailwc/compare/0.3.4...0.3.5) (2022-08-25)


### Bug Fixes

* update wasm path ([b7a9350](https://github.com/arlyon/stailwc/commit/b7a93508d757c733d3bab821f4633e5eb825859d))


### [0.3.2](https://github.com/arlyon/stailwc/compare/0.3.1...0.3.2) (2022-08-25)


### Features

* add new install experience ([ee0ec1e](https://github.com/arlyon/stailwc/commit/ee0ec1e1518314d415e98b42b2833c06dd30aab9))
* add tracking plugin ([9d356f9](https://github.com/arlyon/stailwc/commit/9d356f9d39a543998d5f2f6788b2210bda0507c5))


### [0.3.1](https://github.com/arlyon/stailwc/compare/0.3.0...0.3.1) (2022-08-25)


### Bug Fixes

* allow [ and ] in subject (for arbitrary values) ([05b7bae](https://github.com/arlyon/stailwc/commit/05b7bae6618734781d16cde5a380d130d0233584))
* allow arbitrary spacing in tailwind strings ([5d908b9](https://github.com/arlyon/stailwc/commit/5d908b9430914b29a99344c296d94e43001f9e18))
* allow dashes in the modifier ([2847079](https://github.com/arlyon/stailwc/commit/2847079cc5a5942baa11b25cb9705dc6784777c1))
* deny unwrap to prevent panics ([654d562](https://github.com/arlyon/stailwc/commit/654d562b2dbe29307553a354c4ba1f14573e9f63))
* handle parsing issue when subject has a . in it ([c6372d6](https://github.com/arlyon/stailwc/commit/c6372d6c13a1b69f84c9215c2543b07e4a34b82d))
* hidden should map to display: none ([ca7d8a7](https://github.com/arlyon/stailwc/commit/ca7d8a7195508eb7dad7fd8f1baa8da2a5e5f524))
* make display a root plugin ([a3c156a](https://github.com/arlyon/stailwc/commit/a3c156ab37abe46ca41c0ed52ce513f2cd8c4824))
* propagate proper width and height values ([8fb2f4f](https://github.com/arlyon/stailwc/commit/8fb2f4f5519387f742a5beacb8bd9009d96c3e28))
* proper border width handling for border plugin ([6780341](https://github.com/arlyon/stailwc/commit/6780341d21dcb59465d81826edd110b8f9e6b346))
* swap the modifiers and negative statement when parsing ([02adf9a](https://github.com/arlyon/stailwc/commit/02adf9aeede1c169ffea694890f4d47b2f150a4d))


### Features

* add basis plugin ([ef3fa3f](https://github.com/arlyon/stailwc/commit/ef3fa3f64b22adada1d1a3a89e9d06cf9c267f67))
* add flex plugin ([40d86cf](https://github.com/arlyon/stailwc/commit/40d86cf6b8047fb6b50e65c838eb30b4fdc1f47d))
* add focus-within mod ([5aed754](https://github.com/arlyon/stailwc/commit/5aed75454479f1c5f9ed2a2de7d667f315d06448))
* add font weight ([5eba5c8](https://github.com/arlyon/stailwc/commit/5eba5c8f6a79baa2e9b6917ce4593f64ceefb872))
* add gap plugin ([461c8c6](https://github.com/arlyon/stailwc/commit/461c8c653f226be8f737ab583f5d251efab8d7a4))
* add grid plugin ([5e3470e](https://github.com/arlyon/stailwc/commit/5e3470eac9c9fb425b34c0c456e7ceb8e8460a62))
* add grow plugin ([283b3e7](https://github.com/arlyon/stailwc/commit/283b3e7fab30f8d3d455483888e2156811a83ae1))
* add items plugin ([f3b6eb2](https://github.com/arlyon/stailwc/commit/f3b6eb265e3cf2eab34e65665e3d75b43383cbf6))
* add justify plugin ([ec09cca](https://github.com/arlyon/stailwc/commit/ec09ccac42f85862678c1805348c8b0cccf9a50a))
* add overflow plugin ([1fd8e7b](https://github.com/arlyon/stailwc/commit/1fd8e7be4e3c505605547c6d9f3e6460e04197a6))
* add parsing support for arbitrary CSS ([b73b6ea](https://github.com/arlyon/stailwc/commit/b73b6eab643ece00822a336f34c06f4b3076d947))
* add px, py, pt, pb, pl, pr, and m_ plugins ([11e1297](https://github.com/arlyon/stailwc/commit/11e1297a43f37387f11125c3a25879cea72bb5de))
* add screen reader plugin ([2cbbad0](https://github.com/arlyon/stailwc/commit/2cbbad00044d9b15e79113ca08148a64a2b99f6b))
* add shrink plugin ([464473d](https://github.com/arlyon/stailwc/commit/464473d3121620663ab2c048f8d1ee97f866d96d))
* add text centering and fix text plugin ([5752c3d](https://github.com/arlyon/stailwc/commit/5752c3d009baa9bde33e108b5d0fd3d3958bb4b0))
* add translate plugin ([76aebb9](https://github.com/arlyon/stailwc/commit/76aebb9c9cd40355ddd9225fc87112f6390aad0f))


### [0.3.0](https://github.com/arlyon/stailwc/compare/0.2.2...0.3.0) (2022-08-18)


### Features

* add box plugin ([6c23f30](https://github.com/arlyon/stailwc/commit/6c23f309d4a8bde256317ed16eddbb195ce9c6ba))
* add colored shadows ([c84bb95](https://github.com/arlyon/stailwc/commit/c84bb95e11606d2a628eaa20e44b25599e7cebaa))
* add cursor plugin ([a0a81fb](https://github.com/arlyon/stailwc/commit/a0a81fb6515f375c5c4f8ff924b9fa587cfbe346))
* add display plugin ([562e6c9](https://github.com/arlyon/stailwc/commit/562e6c9b0886f19eafd90233c02836ab584f1ee1))
* add font property ([a9f9288](https://github.com/arlyon/stailwc/commit/a9f9288de7ad7763ef3b1f3f4cd6b5a67e207bd3))
* add position and t/r/b/l plugins ([47d3644](https://github.com/arlyon/stailwc/commit/47d364471c24168bc9df76d6a903585db9d1230a))
* add proper literal merging ([c2b3a56](https://github.com/arlyon/stailwc/commit/c2b3a56b1973d0e5fa6728101c74afc4bf86233b))
* add rounded property ([404d986](https://github.com/arlyon/stailwc/commit/404d986cf3c53efb8f9118dcb2016249b99dfe02))
* add scale plugin ([cfd6449](https://github.com/arlyon/stailwc/commit/cfd6449c98b5cf75b6b33975915dfc7d77e33d8e))
* add select plugin ([9a20f4a](https://github.com/arlyon/stailwc/commit/9a20f4a0bf1c982ea83d51587fd7f8a570b5b850))
* add shadow ([c1bf368](https://github.com/arlyon/stailwc/commit/c1bf3684ba5deec46fc917eea6b0ac5e4a9ee3e7))
* add transitions ([00e3f7d](https://github.com/arlyon/stailwc/commit/00e3f7d60ef13709a741b9a7e1267d320d605c5f))
* add visibility and notion of 'root plugins' ([1345373](https://github.com/arlyon/stailwc/commit/134537388e5b5c47b5a1ddade4bb6499393ed4b2))
* add z plugin ([65a0f08](https://github.com/arlyon/stailwc/commit/65a0f0854d44d74c68fdfe74cbc73c80aec42c75))
* support config from tailwind ([13543d7](https://github.com/arlyon/stailwc/commit/13543d7a08d4bf50ff4dc034f1d10f2dbafed238))


### [0.2.2](https://github.com/arlyon/stailwc/compare/0.2.1...0.2.2) (2022-08-18)


### Features

* add attribute merging for groups ([2e31afb](https://github.com/arlyon/stailwc/commit/2e31afb4adbb80f81f6c4c67866cdf070278363b))


### [0.2.1](https://github.com/arlyon/stailwc/compare/0.2.0...0.2.1) (2022-08-18)


### Features

* properly handle existing css attribute ([97ce9cc](https://github.com/arlyon/stailwc/commit/97ce9cc603065b73510026824907eb1dced39208))



# [0.2.0](https://github.com/arlyon/stailwc/compare/0.1.1...0.2.0) (2022-08-18)


### Bug Fixes

* make sure to visit children if we don't intercept ([515e97b](https://github.com/arlyon/stailwc/commit/515e97b1cef21ee3f1c16db8a5aca1e5a945e794))


### Features

* support template tags ([dd5a21](https://github.com/arlyon/stailwc/commit/dd5a21398bae20f830e37349cc26bdae38056160))


# [0.1.1](https://github.com/arlyon/stailwc/compare/0.1.0...0.1.1) (2022-08-18)


### Features

* add hover, media, focus, borders ([0666bbb](https://github.com/arlyon/stailwc/commit/0666bbb05c097ae9e898b21700709091949ed069))
* get first css rendering on the page ([315ac24](https://github.com/arlyon/stailwc/commit/315ac2463571f85edeaf9b4b627e8a20dce41d04))

# 0.1.0

Initial commit!
