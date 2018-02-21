<a name="0.2"></a>
## 0.2 (2018-02-21)


#### Features

* **Coordinate:**  Add feature to dump the pruning tables in cache file ([02d13cf2](https://github.com/Vallium/rubik-rs/commit/02d13cf2acd4909b5cabc484c46f563f80c332f2))
* **Coordinate Level:**
  *  Add reading of pruning tables from file if exists ([e28bb875](https://github.com/Vallium/rubik-rs/commit/e28bb875cf284c524697b4edab7b1594e754e0e6))
  *  Implement method to read pruning from file ([7d257000](https://github.com/Vallium/rubik-rs/commit/7d257000e361ed149356e0fea392c13a45824838))
  *  Implement pruning for `flip` ([e4b53a6c](https://github.com/Vallium/rubik-rs/commit/e4b53a6c49af44761c48fc5a4e1d0d29a3838da1))
  *  Implement pruning for `twist` ([99084711](https://github.com/Vallium/rubik-rs/commit/990847116944f3e885819b00d9378e80502a6dce))
  *  Implement pruning for `ur_to_df_parity` ([8c9e064c](https://github.com/Vallium/rubik-rs/commit/8c9e064c9f9df5511dc50f989c359ddc0a4b5657))
  *  Implement pruning for `urf_to_dlf_parity` ([a4663cde](https://github.com/Vallium/rubik-rs/commit/a4663cded4a604e3bf11d2ad7df9a91e600aaac0))
  *  Implement pruning for `merge_ur_to_ul_and_ub_to_df` ([176fc822](https://github.com/Vallium/rubik-rs/commit/176fc82295474c95673c3aad8312792e5d3a29a2))
  *  Implement pruning for `ur_to_df_move` ([f1707402](https://github.com/Vallium/rubik-rs/commit/f17074024854205e5df9c87a0fbe4e072aedcd2a))
  *  Implement pruning for `ur_to_df_move` ([9b3d6a19](https://github.com/Vallium/rubik-rs/commit/9b3d6a19bc00a370059f6c1bcf6b25e628d9aa34))
  *  Implement pruning for `ub_to_df_move` ([1b27c793](https://github.com/Vallium/rubik-rs/commit/1b27c793f18f8beca1bdab16036049f28094dca1))
  *  Implement pruning for `ur_to_ul_move` ([630d4ae4](https://github.com/Vallium/rubik-rs/commit/630d4ae481f3f3df9f819a1dfa270c6876e92997))
  *  Implement pruning for `urf_to_dlf_move` ([b3b04d74](https://github.com/Vallium/rubik-rs/commit/b3b04d74bbecafeae96c635fb98964b9806ab643))
  *  Implement pruning for `fr_to_br move` ([6c4ec23d](https://github.com/Vallium/rubik-rs/commit/6c4ec23d8f91295321244e5b8df0dcd0335f1acc))
  *  Implement pruning for `flip move` ([eea8d7c6](https://github.com/Vallium/rubik-rs/commit/eea8d7c6532f75ca6d3ad5e343630147800d0e21))
  *  Implement the coordinate constructor and all `Cube` method needed to ([d6c61400](https://github.com/Vallium/rubik-rs/commit/d6c61400931e8af475125e25e216d09f686daf9e))
* **Corner:**  Implement moves of corners ([ac857c47](https://github.com/Vallium/rubik-rs/commit/ac857c475c1a9ab4bf711b51b824a3f47f9f4b21))
* **Cube:**
  *  Implement Cube struct ([a2c06eff](https://github.com/Vallium/rubik-rs/commit/a2c06eff89e1f64e616fb19b35f6d38f98a887b8))
  *  Change the way how the Moves works ([950b4a05](https://github.com/Vallium/rubik-rs/commit/950b4a05f8876aeef38063d1afea7bf70804c698))
  *  Impl cnk for cubie cube level ([e76ef9eb](https://github.com/Vallium/rubik-rs/commit/e76ef9eb2539721d039152ffc35c8e91baa4f236))
  *  Impl `default` trait for Cube ([45093a69](https://github.com/Vallium/rubik-rs/commit/45093a69f269ae92187dde14d02e68f5459489f0))
  *  Impl `from_shuffle_sequence` scrambling the cube ([f2a92876](https://github.com/Vallium/rubik-rs/commit/f2a9287641aff6286ad39fc51885f59f365629b1))
  *  Impl double moves ([509b57eb](https://github.com/Vallium/rubik-rs/commit/509b57eb1ae174f8099e24c933f698276710e066))
  *  Implement `is_solved()` returns `bool` ([32d46ffd](https://github.com/Vallium/rubik-rs/commit/32d46ffda89376a55d4f4b8f0bfb199d28a7095f))
  *  Add method displaying colored cube at the state ([e40cd42c](https://github.com/Vallium/rubik-rs/commit/e40cd42c5f98ee36bd4ea89be79460479960cb6d))
  *  Add Edges and Corners orientation ([d1c98e3d](https://github.com/Vallium/rubik-rs/commit/d1c98e3d4cfe4fd07736298d74143223d227fb15))
  *  Add a new Cube implementation ([efc63f73](https://github.com/Vallium/rubik-rs/commit/efc63f737125aa0dfe00eb298fc8ffc5be83cdb4))
  *  Add Cube module ([5a99c2ad](https://github.com/Vallium/rubik-rs/commit/5a99c2ad5ac1adddd52f12f8b90f7e12f4b656d8))
* **CubieLvl:**  Change the implementation of the Cubie cube level ([9241325a](https://github.com/Vallium/rubik-rs/commit/9241325a91e2b1a528eb5eb1fdddabf16b3092f2))
* **Edge:**  Implement moves of edges ([6fc0cfd2](https://github.com/Vallium/rubik-rs/commit/6fc0cfd2e28b0e030aba7b2c45b716251220d9be))
* **Main:**  Add differents moves of rubik's cube, stock them in a smallvec ([271756c6](https://github.com/Vallium/rubik-rs/commit/271756c658c9728a16c93301ef0fd462dde7532d))
* **Move:**  Implemente a method returns Move from usize ([56eeea7d](https://github.com/Vallium/rubik-rs/commit/56eeea7d9ff4c7a8686101b3f669ef9c9f8d4da7))
* **Pruning:**  Generalize the type can be dump to a file ([12039761](https://github.com/Vallium/rubik-rs/commit/120397613f2bb13b229fef71d4b2dd4ad9eb7a20))
* **Solver:**  Impl solver (ip) ([6cf55415](https://github.com/Vallium/rubik-rs/commit/6cf554154fa7761707574da82044ed163370beff))

#### Bug Fixes

* **Coordinate Level:**  Remove benches for pruning ([8ea4840d](https://github.com/Vallium/rubik-rs/commit/8ea4840d486628d0df11120328eae1502b03a125))
* **Corner:**
  *  Fix problem with orientations ([02fd2e4d](https://github.com/Vallium/rubik-rs/commit/02fd2e4d641b4ae6ca923fb0721d39f255d38b37))
  *  Fix problem with F and B moves ([d49c024a](https://github.com/Vallium/rubik-rs/commit/d49c024aab597c1a5e84c714764de25d5c887a65))
  *  Fix inverted corners on R and L moves ([f8ec7226](https://github.com/Vallium/rubik-rs/commit/f8ec7226b82c1d99a886faf56c43bc7be74905b8))
  *  fix permutations of corner (ip) ([8ba1aafc](https://github.com/Vallium/rubik-rs/commit/8ba1aafc1cde1df843a9028dcc39acae7d769167))
  *  Ordinate the names to fix orientation ([dccf57e5](https://github.com/Vallium/rubik-rs/commit/dccf57e5dac0da6d2cfd5ff15ec92437a6eea0a7))
* **Corners:**  Change corner map to an array ([e7c1819f](https://github.com/Vallium/rubik-rs/commit/e7c1819f99f8c1bf4d6838d7fa98c746f6569d56))
* **Cube:**
  *  Change the name of `Cube` struct in `Cubie` ([4ed6ce9a](https://github.com/Vallium/rubik-rs/commit/4ed6ce9a08c891f5b74c96dd40dea8c62e6dbb6b))
  *  Fix display of the cube at state ([83d79007](https://github.com/Vallium/rubik-rs/commit/83d7900776f1b8d7b95cf9e43b61a4b5fd96907f))
* **Dump file:**
  *  Extern dumpin file out of calculation ([24da0c74](https://github.com/Vallium/rubik-rs/commit/24da0c7455a130415210fd90a4307a09541d359d))
  *  Fix the permissions of created cache files to read-only ([fac5978e](https://github.com/Vallium/rubik-rs/commit/fac5978e555106a84b0d04c8f71e427c44d8b36b))
  *  Fix the permissions of created cache files to read-only ([0223927e](https://github.com/Vallium/rubik-rs/commit/0223927e89f838f87518ea8d24db5924402ad5ee))
  *  Move the creation of cache folder in one method ([f708b1df](https://github.com/Vallium/rubik-rs/commit/f708b1df6834010ac864cd9877e3a5c142d16898))
* **Edge:**  Fix edges permutation and orientation ([5980facb](https://github.com/Vallium/rubik-rs/commit/5980facb88aade34c613cb5173c09994f0a0638c))
* **Edges:**  Change edges map to an array ([10e71d5f](https://github.com/Vallium/rubik-rs/commit/10e71d5fd9df900fd99e670e6cddaf69ec17cb64))
* **Pruning tables:**  Store the prunings in Boxes to put them on the heap ([a86c9dd7](https://github.com/Vallium/rubik-rs/commit/a86c9dd72b7f26bf6a87f41ecd2bc201a29b1219))
* **Prunning tables:**  Change the way the tables are read ([ad968e6d](https://github.com/Vallium/rubik-rs/commit/ad968e6d79a90c2c9763786d8dac3d58ab1248e7))



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
