<a name="0.1"></a>
## 0.1 (2017-11-09)


#### Bug Fixes

* **Corner:**
  *  Fix problem with orientations ([02fd2e4d](https://github.com/Vallium/rubik-rs/commit/02fd2e4d641b4ae6ca923fb0721d39f255d38b37))
  *  Fix problem with F and B moves ([d49c024a](https://github.com/Vallium/rubik-rs/commit/d49c024aab597c1a5e84c714764de25d5c887a65))
  *  Fix inverted corners on R and L moves ([f8ec7226](https://github.com/Vallium/rubik-rs/commit/f8ec7226b82c1d99a886faf56c43bc7be74905b8))
  *  fix permutations of corner (ip) ([8ba1aafc](https://github.com/Vallium/rubik-rs/commit/8ba1aafc1cde1df843a9028dcc39acae7d769167))
  *  Ordinate the names to fix orientation ([dccf57e5](https://github.com/Vallium/rubik-rs/commit/dccf57e5dac0da6d2cfd5ff15ec92437a6eea0a7))
* **Corners:**  Change corner map to an array ([e7c1819f](https://github.com/Vallium/rubik-rs/commit/e7c1819f99f8c1bf4d6838d7fa98c746f6569d56))
* **Cube:**  Fix display of the cube at state ([83d79007](https://github.com/Vallium/rubik-rs/commit/83d7900776f1b8d7b95cf9e43b61a4b5fd96907f))
* **Edge:**  Fix edges permutation and orientation ([5980facb](https://github.com/Vallium/rubik-rs/commit/5980facb88aade34c613cb5173c09994f0a0638c))
* **Edges:**  Change edges map to an array ([10e71d5f](https://github.com/Vallium/rubik-rs/commit/10e71d5fd9df900fd99e670e6cddaf69ec17cb64))

#### Features

* **Corner:**  Implement moves of corners ([ac857c47](https://github.com/Vallium/rubik-rs/commit/ac857c475c1a9ab4bf711b51b824a3f47f9f4b21))
* **Cube:**
  *  Add method displaying colored cube at the state ([e40cd42c](https://github.com/Vallium/rubik-rs/commit/e40cd42c5f98ee36bd4ea89be79460479960cb6d))
  *  Add Edges and Corners orientation ([d1c98e3d](https://github.com/Vallium/rubik-rs/commit/d1c98e3d4cfe4fd07736298d74143223d227fb15))
  *  Add a new Cube implementation ([efc63f73](https://github.com/Vallium/rubik-rs/commit/efc63f737125aa0dfe00eb298fc8ffc5be83cdb4))
  *  Add Cube module ([5a99c2ad](https://github.com/Vallium/rubik-rs/commit/5a99c2ad5ac1adddd52f12f8b90f7e12f4b656d8))
* **Edge:**  Implement moves of edges ([6fc0cfd2](https://github.com/Vallium/rubik-rs/commit/6fc0cfd2e28b0e030aba7b2c45b716251220d9be))
* **Main:**  Add differents moves of rubik's cube, stock them in a smallvec ([271756c6](https://github.com/Vallium/rubik-rs/commit/271756c658c9728a16c93301ef0fd462dde7532d))

