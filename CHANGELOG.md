# [0.16.0](https://github.com/arlyon/stailwc/compare/0.15.0...0.16.0) (2023-05-22)


### Bug Fixes

* add case for SingularBox components ([415afeb](https://github.com/arlyon/stailwc/commit/415afeb2b40f2c5a8e36515eb5c001bf8b521697))
* disable lto (not well supported) ([4af7e7a](https://github.com/arlyon/stailwc/commit/4af7e7afd94bd5a814994c565b4bb66692d20e97))
* don't double-negate negative values ([fc7c0d0](https://github.com/arlyon/stailwc/commit/fc7c0d028fa901d8dec4c7226b20e116bcedd488))
* inline flex ([7af8892](https://github.com/arlyon/stailwc/commit/7af88924fe4c89f66f816d132e5dc1d679babee5))
* prefer multispace over space to better line up with jsx strings ([8be0b60](https://github.com/arlyon/stailwc/commit/8be0b60c9c50df58176b4cdc30f9472cf20b34b9))


### Features

* add a number of peer modifiers ([13b3692](https://github.com/arlyon/stailwc/commit/13b3692578c6d69404423131921b1f806f7ff6d1))
* add accent plugin ([2714ce5](https://github.com/arlyon/stailwc/commit/2714ce5f06cb2fccf735e5a18c844dfc0d953986))
* add alpha to bg plugin ([f15e159](https://github.com/arlyon/stailwc/commit/f15e159b961d2441694de2a2ec35550ff44339ff))
* add alpha to placeholder ([a40c334](https://github.com/arlyon/stailwc/commit/a40c33434df29fd8ab922d453fb657c9419cf7dc))
* add alpha to ring and support opacity and inset ([f3fd0a5](https://github.com/arlyon/stailwc/commit/f3fd0a5d5907d40fa6ecb925115e0f567d5f3fdb))
* add bg attachment plugin support ([5b0ed0b](https://github.com/arlyon/stailwc/commit/5b0ed0b0b25ed6a1e381cb0bbf0cd7f5f5650748))
* add bg opacity plugin support ([ab69a21](https://github.com/arlyon/stailwc/commit/ab69a21cc1181152c002865b4761bcdcf8a3c16b))
* add caret plugin ([b7e22ed](https://github.com/arlyon/stailwc/commit/b7e22ede709950d1b2c1cc4aced1c1e26b0f14b9))
* add columns plugin ([9d81033](https://github.com/arlyon/stailwc/commit/9d81033776b5e8f339ff23aa5218e7c2c6d458e8))
* add decoration plugin ([86d4ada](https://github.com/arlyon/stailwc/commit/86d4ada1de1c3fd974f6630490f93c82625308d5))
* add not-disabled modifier ([9b509a2](https://github.com/arlyon/stailwc/commit/9b509a2a5ce64bb9ccecdd67ffc3ff2230117f8a))
* add outline color ([08a2ea0](https://github.com/arlyon/stailwc/commit/08a2ea0065ef4c172c24ea296f25520fbca03f26))
* add text opacity plugin support ([64d5f32](https://github.com/arlyon/stailwc/commit/64d5f324c591e3ee14cefb1934345379475f4e26))
* add transparency to text plugin ([b71861a](https://github.com/arlyon/stailwc/commit/b71861a2852c5ec339c5ce236c6ce3914a28424c))
* add whitespace-break-spaces support ([a5b122f](https://github.com/arlyon/stailwc/commit/a5b122fc2d22206104fd055566bf341a131eac0a))
* change error handler in test vs in plugin ([5c4fd8f](https://github.com/arlyon/stailwc/commit/5c4fd8f59dfc70c9bc14a3a2ee9a035ae79c0479))
* change error handler in test vs in plugin ([b387b94](https://github.com/arlyon/stailwc/commit/b387b94281aa01448f10d23a94f36c13f1066a3c))
* make opacity accept arbitrary values ([b5fbf02](https://github.com/arlyon/stailwc/commit/b5fbf0259c32e4acea2188331fdd0d1636675771))
* overhaul border plugin ([8f67035](https://github.com/arlyon/stailwc/commit/8f67035862ea3c998941e072884f25cfd43a32a5))
* support border-s and border-e plugins ([22041b3](https://github.com/arlyon/stailwc/commit/22041b36fffd0a9a4985da87aea637bac342ef28))
* support symmetrical border spacing ([a627487](https://github.com/arlyon/stailwc/commit/a6274878c7cab262da82eeee4489217e3d2ab178))

# [0.15.0](https://github.com/arlyon/stailwc/compare/0.14.0...0.15.0) (2023-02-20)


### Bug Fixes

* include d.ts file in npm package ([b3cc831](https://github.com/arlyon/stailwc/commit/b3cc83183a1085c783b79980a19592986e9da30b))
* rename flex-nowrap ([1db8d6d](https://github.com/arlyon/stailwc/commit/1db8d6d0fe2417adc69ddf2c57e4b9a5e6d04f2d))
* rename grid-auto-flow col to column ([b54c3cb](https://github.com/arlyon/stailwc/commit/b54c3cba72ff1490cac774075d80eb802415665b))
* rename iterms-stretch to items-stretch ([95c423d](https://github.com/arlyon/stailwc/commit/95c423d8979ac7f4451d198378087fa5173511a3))


### Features

* allow parser to handle infinitely nested plugin segments ([1b96169](https://github.com/arlyon/stailwc/commit/1b961698faf84694e53cd23b21994303c7e6311e))
* make auto_cols plugin support arbitrary CSS ([f7df759](https://github.com/arlyon/stailwc/commit/f7df7596d9caea8ac38a805ffdb89d895d40cc5c))
* make auto-rows plugin support arbitrary CSS ([41ed335](https://github.com/arlyon/stailwc/commit/41ed335aedc091c7d62e484188f2bd645158c58d))
* make basis plugin support arbitrary CSS ([c7288f8](https://github.com/arlyon/stailwc/commit/c7288f84b2b001869c9c8ef057b06f520cc432d9))
* make col plugin support arbitrary CSS ([c3d7b19](https://github.com/arlyon/stailwc/commit/c3d7b198a5c49006f454a17d017c6c88a5ddd280))
* make flex plugin support arbitrary CSS ([ca48c18](https://github.com/arlyon/stailwc/commit/ca48c1821519bdfdaf6eb3a10eb51edda151358e))
* make grid-col utilities support arbitrary CSS ([f4e038a](https://github.com/arlyon/stailwc/commit/f4e038a0781ef755bd580bf4520a419ebafe04cc))
* make grid-row utilities support arbitrary CSS ([70ff15b](https://github.com/arlyon/stailwc/commit/70ff15bd2796ddc6b20b01b8e9692f4a9e713596))
* make grow plugin support arbitrary css ([acf02bc](https://github.com/arlyon/stailwc/commit/acf02bc431ac0fc02e86d79504a0064e6552a582))
* make order plugin support arbitrary CSS ([dd7656b](https://github.com/arlyon/stailwc/commit/dd7656bf4b5af1aeb9cdd0b75eec87c34dbe3941))
* make row plugin support arbitrary CSS ([70b279c](https://github.com/arlyon/stailwc/commit/70b279cfea3804d6045c4ba5f22ef5ebd8111f81))
* make shrink plugin support arbitrary CSS ([1e20218](https://github.com/arlyon/stailwc/commit/1e2021863c03a6e9e2b1c437807296c993c1c6ee))
* support content subcommands ([41b4d60](https://github.com/arlyon/stailwc/commit/41b4d60bbb86e563e0ea3e1b158b7fac1c293414))

# [0.14.0](https://github.com/arlyon/stailwc/compare/0.13.0...0.14.0) (2023-01-25)


### Features

* add typescript type support ([a57d722](https://github.com/arlyon/stailwc/commit/a57d722c2a50c6d77c49882567decd1119c5699b))

# [0.13.0](https://github.com/arlyon/stailwc/compare/0.12.0...0.13.0) (2023-01-20)


### Features

* add antialiased plugin ([d22886d](https://github.com/arlyon/stailwc/commit/d22886d60e6ba8f6a6546435da716f4c5a807df1))
* add aspect plugin ([2eaecc3](https://github.com/arlyon/stailwc/commit/2eaecc3f339c2aa2430f384b325af4d23cacd5b4))
* add backdrop-blur plugin ([9783916](https://github.com/arlyon/stailwc/commit/978391671e87faae9974f0a1601f62fb2d9922b0))
* add List plugin ([74e2824](https://github.com/arlyon/stailwc/commit/74e28246a7cd8c6a0bae638f7c64117513ae354f))
* add scroll-auto and scroll-smooth ([a5be83e](https://github.com/arlyon/stailwc/commit/a5be83eae8fe18dda2e210d84660859a03be6956))
* add snap ([9c00ec9](https://github.com/arlyon/stailwc/commit/9c00ec958f7a49980b1bf55f0c6e42948e0e3be1))
* add stroke plugin ([4cfbaed](https://github.com/arlyon/stailwc/commit/4cfbaed4f52b287271d9cab938aa7d2c6ec765fe))
* make outline OptionalArbitrary ([0321871](https://github.com/arlyon/stailwc/commit/03218717216df935774dfd3d6a71a2cedc8ebd77))
* make transition delay arbitrary ([f6d7a79](https://github.com/arlyon/stailwc/commit/f6d7a7926e6660e09da52c3f5523ff38be00362f))

# [0.12.0](https://github.com/arlyon/stailwc/compare/0.11.1...0.12.0) (2022-12-27)


### Bug Fixes

* allow the macro to support enums without subcommands ([3ddd529](https://github.com/arlyon/stailwc/commit/3ddd5293e0713abff2c5dc9587654ef17505d56c))
* improve suggestion in text plugin ([497a8b1](https://github.com/arlyon/stailwc/commit/497a8b1c31b996587b28f6093ce209e5db96da61))
* only inject import if actually importing TailwindStyle ([1dadff6](https://github.com/arlyon/stailwc/commit/1dadff6af3a35ea7a0de4bb87e72514c37a922d6))
* revert span offset fix ([9cb7a36](https://github.com/arlyon/stailwc/commit/9cb7a36db397372fdb6f7bff1c216880a797d1bf))


### Features

* add alternative suggestions for array plugins ([1a722ad](https://github.com/arlyon/stailwc/commit/1a722ad064118c0efc78bf62fb8bba34dc086647))
* add alternative suggestions for array_map plugins ([b35e853](https://github.com/arlyon/stailwc/commit/b35e8536bc8a0df1e33883779c803151f15b088a))
* add alternative suggestions for simple_lookup_map plugins ([0137a45](https://github.com/arlyon/stailwc/commit/0137a45e19e425776dd6e52121b6900c95bf2d42))
* add auto plugin ([4c24526](https://github.com/arlyon/stailwc/commit/4c24526c0f9169192c33a916278760b0313de5ca))
* add proper error handling for bad plugin params ([66d6fb1](https://github.com/arlyon/stailwc/commit/66d6fb1b0a121a6a56a52eaac60c18b7baf65cb4))
* add styled-components engine ([79bd086](https://github.com/arlyon/stailwc/commit/79bd086836f1cf881f8d67e0476755ff6cddfa07))
* expose suggestions on swc error handler ([b14872c](https://github.com/arlyon/stailwc/commit/b14872ca26ba60efb2c2b7f01d0dc058e2917a26))
* have simple_lookup plugins suggest alternatives ([62ec998](https://github.com/arlyon/stailwc/commit/62ec9987bcf2468c8d81206598df1d15094a3f29))
* make a bunch of plugins support arbitrary parameters ([21dbe0b](https://github.com/arlyon/stailwc/commit/21dbe0b8074a74d187a408f96e0a273ff23a5a76))
* support full recovery when parsing plugins ([4f78b54](https://github.com/arlyon/stailwc/commit/4f78b54c0af635392a35245f00f8846a54b0e072))
* support style injection for styled-components ([79ba0ff](https://github.com/arlyon/stailwc/commit/79ba0ff15b06dc98fc6af167cf5c9fd7a5f2f284))

## [0.11.1](https://github.com/arlyon/stailwc/compare/0.11.0...0.11.1) (2022-12-05)


### Bug Fixes

* pin toolchain to avoid panics ([5e3ff5f](https://github.com/arlyon/stailwc/commit/5e3ff5f9100c6547b99b6000a6b006773aeaf2dd))

# [0.11.0](https://github.com/arlyon/stailwc/compare/0.10.0...0.11.0) (2022-12-05)


### Bug Fixes

* add unzip feature ([c266fb4](https://github.com/arlyon/stailwc/commit/c266fb437fe3d7cded91d3c7e56c922cb09bc293))
* allow numbers in the modifiers ([8e20a03](https://github.com/arlyon/stailwc/commit/8e20a03a0b4d005cdcbc3ad0aac8d543f04c23fd))


### Features

* add arbitrary support for grid template row and col ([7e44200](https://github.com/arlyon/stailwc/commit/7e44200ff5cdb72eb7be9b2906b4bfda85202a54))
* add arbitrary support to the gap plugin ([6347a8f](https://github.com/arlyon/stailwc/commit/6347a8f16b8ba260ec528917c5d7c984d4ebae73))
* add background position plugin ([50cb275](https://github.com/arlyon/stailwc/commit/50cb275899dde062525bbdde21657aeba1bfce3d))
* add background repeat plugin ([a295300](https://github.com/arlyon/stailwc/commit/a29530058f54879aee6a4602b04512de9b3e1a24))
* add background size plugin ([d1b2300](https://github.com/arlyon/stailwc/commit/d1b230055dac95517884c1f588d1d337a4af0473))
* add content plugin ([687f5a2](https://github.com/arlyon/stailwc/commit/687f5a285692d450b0dc8cbdaf4c8a4758fc6df8))
* add float plugin ([d873b2f](https://github.com/arlyon/stailwc/commit/d873b2f549d3899fdafc4b94e29d98a80e1a69c4))
* add justify items and self plugins ([ae0a938](https://github.com/arlyon/stailwc/commit/ae0a9387bf2e72371b70895633c93643ece11bbc))
* add line clamp plugin ([0c2780f](https://github.com/arlyon/stailwc/commit/0c2780f57e16a24f1eee0f0f8ceb9f38bc8b4774))
* improve parsing and allow arbitrary values for translate ([ae5fe6e](https://github.com/arlyon/stailwc/commit/ae5fe6ea1cff1c2c6dd2c8f61e43bc54e04eb76a))
* support nested tw attrs by using an attribute stack instead ([1848eb3](https://github.com/arlyon/stailwc/commit/1848eb3cdb8e4a225b19dc13a60d230876824499)), closes [#16](https://github.com/arlyon/stailwc/issues/16)

# [0.10.0](https://github.com/arlyon/stailwc/compare/0.9.0...0.10.0) (2022-12-01)


### Bug Fixes

* adjust the parser to handle subcommands better ([217644d](https://github.com/arlyon/stailwc/commit/217644d6b17e981516b4f497bebf8ba4276bc3a2))


### Features

* add align plugin ([3f69b75](https://github.com/arlyon/stailwc/commit/3f69b751e4d1863cfe4d9121ec3b8c1541a2927f))
* add arbitrary w and h ([92dcc97](https://github.com/arlyon/stailwc/commit/92dcc9748790388a8a726007efff27fd10e3fc2b))
* add bg plugin arbitrary css support ([cd8f1a2](https://github.com/arlyon/stailwc/commit/cd8f1a2712a6b564465e84bbdf9870412816c262))
* add border-style plugin ([05d6512](https://github.com/arlyon/stailwc/commit/05d65127d4914520695289c9d337a1c7dec16dc5))
* add col-end plugin ([d874547](https://github.com/arlyon/stailwc/commit/d8745471ff4140ba8ce3238ef31ce7e4dd680c5b))
* add group-focus modifier ([feebe07](https://github.com/arlyon/stailwc/commit/feebe070eb44736d7197b6308c05acfd3659548c))
* add italic plugin ([7bb1acc](https://github.com/arlyon/stailwc/commit/7bb1acc5265409330f748181a552505cf2165083))
* add line-through plugin by improving has_subsegments check ([0d92ef5](https://github.com/arlyon/stailwc/commit/0d92ef5dbd0877ea693c95f715b7769bdcea7e69))
* add origin plugin ([c42759c](https://github.com/arlyon/stailwc/commit/c42759c5b1d5a4f10fad63ae43a47d55f7666b50))
* add row plugin ([73291ca](https://github.com/arlyon/stailwc/commit/73291caffd7a9bef15db9d05c4d1543b50187c09))
* add support for styled components ([744d420](https://github.com/arlyon/stailwc/commit/744d42057b1c813932985efb54f3228f5c0d1340))
* add t, b, l, r, and inset arbitrary css support ([d1469d6](https://github.com/arlyon/stailwc/commit/d1469d628585f2112b8b2ea671778de84b058e46))
* include custom fonts in the reset css ([9011241](https://github.com/arlyon/stailwc/commit/9011241573f63a6a174813c16bfa72c8d73fa8a4))
* make border support arbitrary css ([aa48d03](https://github.com/arlyon/stailwc/commit/aa48d03d4bef0a35dbc21bbdede98676bcaa577c))
* make tracking support arbitrary css ([7eaab04](https://github.com/arlyon/stailwc/commit/7eaab040617126c2521f3da07513826ebe860973))
* support arbitrary CSS plugins ([d7c9331](https://github.com/arlyon/stailwc/commit/d7c933161759153e13e899941bfcfe5fa9b4c587))

# [0.9.0](https://github.com/arlyon/stailwc/compare/0.8.1...0.9.0) (2022-11-05)


### Bug Fixes

* port `flex-1` and friends to the new plugin structure ([92168da](https://github.com/arlyon/stailwc/commit/92168dace386dbff641cace6a15d9d89eb0fbf86))
* port `flex-grow` to the new plugin structure ([8635bcc](https://github.com/arlyon/stailwc/commit/8635bccff4562d40630157e458c4b5c209576b79))
* port `flex-shrink` to the new plugin structure ([6423247](https://github.com/arlyon/stailwc/commit/642324769d13c8a0af7816aa55f3b48ae2f5c15e))
* prevent console log in development by including cooked css ([bb289ef](https://github.com/arlyon/stailwc/commit/bb289ef103fe989fc1682eff519971e9160c595a))


### Features

* add basic non-configurable prose plugin ([d88a744](https://github.com/arlyon/stailwc/commit/d88a744169919d3c32d14be00eebc821305bc5d3))
* unify and expose the strict option ([3af6a0d](https://github.com/arlyon/stailwc/commit/3af6a0da906b4e3b6f4f7006cd3e8720ff19641d))

## [0.8.1](https://github.com/arlyon/stailwc/compare/0.8.0...0.8.1) (2022-11-04)


### Bug Fixes

* release to target new crates ([6e289fc](https://github.com/arlyon/stailwc/commit/6e289fc1c99108b8392c512be3db3e7793156848))

# [0.8.0](https://github.com/arlyon/stailwc/compare/0.7.2...0.8.0) (2022-11-04)


### Bug Fixes

* clean up some more snapshots ([071d463](https://github.com/arlyon/stailwc/commit/071d4639d4b9e1a91e5d7b97c216372ed0122865))
* correct lookup for top, bottom, left, and right plugins ([eec214c](https://github.com/arlyon/stailwc/commit/eec214c1a7ae9705a8af9fb8e8a3170327ae814f))
* get screen dynamically form config ([f5c6f16](https://github.com/arlyon/stailwc/commit/f5c6f16f82f7f2feb455450e2abaf42c3c8dc928))
* parse expressions marked as important with no value ([16c023f](https://github.com/arlyon/stailwc/commit/16c023f722be9b4b7308b1370a7d6859b7238857))
* support standalone `shadow` plugin ([7ca223a](https://github.com/arlyon/stailwc/commit/7ca223a166176f6db7e14d489f4d050aac4f4000))
* use parser for divide plugin and handle `divide` ([7e77e9f](https://github.com/arlyon/stailwc/commit/7e77e9fe09fa7a75bd504e581858110636f4f228))


### Features

* add `self` plugin ([cc93a27](https://github.com/arlyon/stailwc/commit/cc93a27c143d82d21c638c8576848c8a8486be21))
* add additional functions to grid plugin ([b81146c](https://github.com/arlyon/stailwc/commit/b81146c813d4a222e460d1db879f785ecc4d06d2))
* add animate plugin ([6f2eabe](https://github.com/arlyon/stailwc/commit/6f2eabe14ef9bdbf3fe7c956684e1699e96f303b))
* add pointer events plugin ([38b9208](https://github.com/arlyon/stailwc/commit/38b92086dca46c90a136bf0284ff75d885542d33))
* add strict mode ([e85d0d1](https://github.com/arlyon/stailwc/commit/e85d0d145cd4e90bb7b1152cd662e1c60ed122f5))
* add support for tailwind css reset and forms ([82e3fbc](https://github.com/arlyon/stailwc/commit/82e3fbce1c1c7fab8f4d47abc2a9032d83ae8a2e))
* add the fill plugin ([eaf3ef2](https://github.com/arlyon/stailwc/commit/eaf3ef2da560526153a016a2827d430c49f148de))
* add the gap-x and gap-y plugins ([6ea516d](https://github.com/arlyon/stailwc/commit/6ea516d9b57da83c83ad29aa7a3a2843d87eeeb0))
* add the inset plugin ([6d192b5](https://github.com/arlyon/stailwc/commit/6d192b55f8edf5ba7140f354247e459541c239e1))
* add the leading plugin ([4d15fd6](https://github.com/arlyon/stailwc/commit/4d15fd6a1be68a2a02dd65ae84ea2c17c621d71a))
* add the object plugin ([e420c5d](https://github.com/arlyon/stailwc/commit/e420c5d76f7e509f68a2394e0274a5ff56b1a053))
* add the whitespace plugin ([5386680](https://github.com/arlyon/stailwc/commit/53866802ef7ee8cd97fd161a29d71c5a9c571c3f))
* add truncate plugin ([eb143cf](https://github.com/arlyon/stailwc/commit/eb143cfbabc06f42dd5c749c65f99d4e5f82d45b))
* add whole load of new modifiers ([f42e43d](https://github.com/arlyon/stailwc/commit/f42e43d24cd6ddfa2e96833058f5a91dc983d9a5))
* improve spans for error handling ([b3a57cf](https://github.com/arlyon/stailwc/commit/b3a57cf2dad30a948ecf0c9e8a64108818c4f098))
* support border-style in divide plugin ([9c15739](https://github.com/arlyon/stailwc/commit/9c15739426cc8a25532ecf94800b7ce9a6a169ba))
* support transparency in the parser ([2ea75e7](https://github.com/arlyon/stailwc/commit/2ea75e7db07f648af5cad6dcd4c026784d876e62))

## [0.7.2](https://github.com/arlyon/stailwc/compare/0.7.1...0.7.2) (2022-09-24)


### Bug Fixes

* add tailwind as a peer dependency ([e6c3647](https://github.com/arlyon/stailwc/commit/e6c3647667094d5854f4ceae03402fc6624ba5df))
* handle nested merging correctly ([4b100aa](https://github.com/arlyon/stailwc/commit/4b100aa3cfc40741ab0a4959d68b4bc5116356e4))

## [0.7.1](https://github.com/arlyon/stailwc/compare/0.7.0...0.7.1) (2022-08-29)


### Bug Fixes

* pin ast_node version to prevent panics ([ebb4fb3](https://github.com/arlyon/stailwc/commit/ebb4fb3004bc3ba999b9da7c88e7af81293e157f))

# [0.7.0](https://github.com/arlyon/stailwc/compare/0.6.0...0.7.0) (2022-08-28)


### Features

* add col plugin ([5f77625](https://github.com/arlyon/stailwc/commit/5f77625343378354bd45e1cf966fcc4b35a866c2))

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
