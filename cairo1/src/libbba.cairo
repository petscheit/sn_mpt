use alexandria_merkle_tree::storage_proof::{verify_mpt_proof, verify_leaf_update, EdgeNodeImpl, BinaryNodeImpl, BinaryNode, EdgeNode, TrieNode, Membership};

#[derive(Drop)]
pub struct LeafUpdate {
    key: felt252,
    proof_pre: Array<TrieNode>,
    proof_post: Array<TrieNode>,
}

#[derive(Drop)]
pub struct BatchUpdate {
    pre_root: felt252,
    post_root: felt252,
    leaf_updates: Array<LeafUpdate>,
}

fn main() -> () {
    let batch_update = BatchUpdate {
        pre_root: 0x03e97495d6a6903e46827229f0d877252017e74cab4299cf4e04c62f05698be7,
        post_root: 0x058f39559468ca8ee738705192acb062404b2d2313aa0b7bef833d27f02ae189,
        leaf_updates: array![
            LeafUpdate {
                key: 0x032f49a3936ace2ffb2f8e8d7469c9f21d3f8cc5b66effa19f1499f42f4bf153,
                proof_pre: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x0409b9d8a73186ed41f08f8765490d44629aaca959e3adfd07abdbd93805433a,0x00ba9f6908e92e93c517ed33ed2357a9e3c6c63ca96b062dc56e1fefbffed7cb)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x03ca60f2c6b91902020131595985f05b39370037ea0c804db73da73aa96b2251,0x04f73b2d813c6e87ce8db928e93739c636ad07be89896680676fdd8c36fab066)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x062c47db72a1e05f72392d6210f1970016e8d2246f69daa8eb187f54dfd8493c,0x07ee900dd73523e61e8148c85d7e959533ba64bc9b70ec7dad177fe4c9c34a2f)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x033f532d3d8c21907d69fbdf2611e0864091b36eeeae66b5f2145aeb056f5ff9,0x043fe62081d3eb2adc6eaf61538fafe3c8877e9fdf29fd0f206cc86dfd2aad68)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02ceea4e5f0ad94b8075c5e051d923df762311c8f2e2bfc8ff18170f71f7aadd,0x035a5105fad382b271ce02e3f01acab56c897fd85ab1510695dc3923d49c183f)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02c6f403772832575ad9e97bcf549b2d5af0363dfeae001c8dc19c7121d79957,0x076825400322bab3619d611928ab073db4d9598d5130ab53d19214ac36061276)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06b3db1b0a100855676ddb035864f8e6dd4403062fee0571d881f3e90be4761e,0x023a9c4660f23962ab2cbcc99b1b0117df4a2a8d747dd1d51af32e8fdf4c7109)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04c0fdee1e8bdc4df84573af603ec76f8625948a79a8ed97ed03e338dbaa2f28,0x016b138fd79139ff3a3a569158fe3082f5c5a374e3dfd025e7c2dc92aaf0f38b)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000000000000000000000000000000000000000000000000000000000000001d, 0x061c442e20dfcdf4c7ccd53816c8dc0bd86607e9dcc01906dda55fa38a4e851f, 5)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04ce107b9b1b69560f527f2550430c721c4ae698b8050c19d39c283eb02cf193,0x03749feaa9e859980e7305ff5e60956c3116f82683ae485f4f3739ad24495fe3)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000009a3936ace2ffb2f8e8d7469c9f21d3f8cc5b66effa19f1499f42f4bf153, 0x04ec7825a22b6f59e33c148706a3967a8af2f1ece46c7d7f4631dec7f04e809a, 237))
                ],
                proof_post: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x02a6dd7c91e8ca5cd37751ad89e557f41a9edf435e785f75d5b80bbf6c8bc90e,0x00ba9f6908e92e93c517ed33ed2357a9e3c6c63ca96b062dc56e1fefbffed7cb)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x03ca60f2c6b91902020131595985f05b39370037ea0c804db73da73aa96b2251,0x064674c2dd30c5f2926097ef966e181873d84524131b887a80e47d72d6c12f42)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x062c47db72a1e05f72392d6210f1970016e8d2246f69daa8eb187f54dfd8493c,0x02e702a5bbbc754e081fabf4a861cd27a9d6b539a1d3c592cd369e1912e34d29)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05b75817dec8605340113064f708de0c5b0532468c20e7ba1938c1c03dbfcd7d,0x043fe62081d3eb2adc6eaf61538fafe3c8877e9fdf29fd0f206cc86dfd2aad68)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x03aea683a0a22845fff7378fbe1391d14444de98c7defa20dc52ff5e1a54acaa,0x035a5105fad382b271ce02e3f01acab56c897fd85ab1510695dc3923d49c183f)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02c6f403772832575ad9e97bcf549b2d5af0363dfeae001c8dc19c7121d79957,0x00e7940abffc1c901945e4b96996461c3828deddeadaa68255061e3c6f91814b)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04890c170550c4ad50e3a87363c27222f8cd18f596eda2cc7b53026a883f8657,0x023a9c4660f23962ab2cbcc99b1b0117df4a2a8d747dd1d51af32e8fdf4c7109)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04c0fdee1e8bdc4df84573af603ec76f8625948a79a8ed97ed03e338dbaa2f28,0x051f5c445df3fd97daf5505250f31d2db0b4988914aac9db8aed8e30c319ab47)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000000000000000000000000000000000000000000000000000000000000001d, 0x070662fea41f28014507d264048516cd96e8af3a8067ead8ad0a3f7b8b568847, 5)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06efe632b2b7085be00084e7dec4e26ff196c6b9363c86f7db4c0d9147fb9400,0x03749feaa9e859980e7305ff5e60956c3116f82683ae485f4f3739ad24495fe3)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000009a3936ace2ffb2f8e8d7469c9f21d3f8cc5b66effa19f1499f42f4bf153, 0x00abefc6c03ee69df3c75d8a1f1bb8588e793d079dc0def5acac7f1c4d8f4ccd, 237))
                ],
            },
            LeafUpdate {
                key: 0x013f4dc663eab8f74784ca02a762a3a23fe9a3465fb78a66164d3d71980faa5a,
                proof_pre: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x02a6dd7c91e8ca5cd37751ad89e557f41a9edf435e785f75d5b80bbf6c8bc90e,0x00ba9f6908e92e93c517ed33ed2357a9e3c6c63ca96b062dc56e1fefbffed7cb)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x03ca60f2c6b91902020131595985f05b39370037ea0c804db73da73aa96b2251,0x064674c2dd30c5f2926097ef966e181873d84524131b887a80e47d72d6c12f42)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07760fd3b509208156525dc9b2a8c716870f9f0f79257a77b9b1398370aaefa1,0x0377c81860ea695baa640c6bce67f387a1570a1e064ed31f17b383dfd603fe54)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0274d6bc96f4fc1f2ee7174edf957ad07f49bcf388f8c2ff26e160ee1ba8f15a,0x01e500a318b71bc3c76a4e4e9472acce4afe0977651be66f93bf7bc7f8ebba1c)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02d6acecdbff8ef5ebc4d88944a6da8f5111e7a52ec5cb384d1af73e4f156a6e,0x07bb6f8aa839a88429ffe52756a063743629aa867306d0c1e67b9dbb7e2821a6)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07e44c09cfbb9bdbd20be900a661b1c763e32013552339f0817bcf4cf6396da9,0x06bfa1ea0801e47ca5011b4b1673545e1e38436a14c4a613ea54dfd8c9fd5cc3)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06dc6ab0a2723eb6d3ca7dc2fd26aed5ccf7ac7adbb28edd439756cad7259c9c,0x0386f25455e473453c892918eb96a063996646a3e4dcfb1c9e49862ee2972a46)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x044b9d3631a7e6f14f6bc5b1f218cf1c068e596b357580eca54e546cd15af252,0x07ec4f750f0b1af475b9b57bc66f3b4e23ad25ec77054e76326d27fc2eeea6e4)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x00074dc663eab8f74784ca02a762a3a23fe9a3465fb78a66164d3d71980faa5a, 0x035ef6885d1d8a8ea06e6fc7914139ba6c09106f96f07a5aeb794210a851f46c, 243))
                ],
                proof_post: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x071e49914c7c7df7cec59413de5d6789ec46db1c3e1177fac684c2b9bc518403,0x00ba9f6908e92e93c517ed33ed2357a9e3c6c63ca96b062dc56e1fefbffed7cb)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06aaf25cab52e360406330014ec1265ac4d9c964dc7d3130da38640c480719eb,0x064674c2dd30c5f2926097ef966e181873d84524131b887a80e47d72d6c12f42)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07760fd3b509208156525dc9b2a8c716870f9f0f79257a77b9b1398370aaefa1,0x020c8465a71caeba7c9be6c151686adbbf0fb17f35d38d12cd6e0daef1aac57e)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0271aca01d8ef9bb50c96801dcb08db792943315097e770efdefaa0bc321724c,0x01e500a318b71bc3c76a4e4e9472acce4afe0977651be66f93bf7bc7f8ebba1c)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x00980ec400d3a26613b2b4828616c5604b51442ce064b7f33a75bae4f32a2830,0x07bb6f8aa839a88429ffe52756a063743629aa867306d0c1e67b9dbb7e2821a6)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07e44c09cfbb9bdbd20be900a661b1c763e32013552339f0817bcf4cf6396da9,0x058045311c1ad8a0b5267361a1154d52ae825a22e6200f9a409ca0fbf517a7bc)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06dc6ab0a2723eb6d3ca7dc2fd26aed5ccf7ac7adbb28edd439756cad7259c9c,0x040c626461b4765990681844fe3e7b6b81d76d9ea286c7e0d729a274a64226f6)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x044b9d3631a7e6f14f6bc5b1f218cf1c068e596b357580eca54e546cd15af252,0x013d8925d08237a2befcb0efa2e58d96840ddf5d8cd314ae082c6061d30b0bcc)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x00074dc663eab8f74784ca02a762a3a23fe9a3465fb78a66164d3d71980faa5a, 0x030fbfcfc78c857b8044bbcb2f224a3dab329dc25e1589d031d0853fda52cf2f, 243))
                ],
            },
            LeafUpdate {
                key: 0x072ac597606f3c69e0f8d3c5414bb6c59a91a5251ae624c1df16e398a8f1aa3d,
                proof_pre: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x071e49914c7c7df7cec59413de5d6789ec46db1c3e1177fac684c2b9bc518403,0x00ba9f6908e92e93c517ed33ed2357a9e3c6c63ca96b062dc56e1fefbffed7cb)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05434ca9d413623081ae034f58282c5f80b72df4c0a3aa1053bffa5ea3498fd3,0x007bed2299b21a196b5c86e63b05ec50ef0819185fc2a730eb43809467da65a2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01c6242c11c3338ae744b4a67d18a9463182c30d6a5ce98edf5ac51b574ffa53,0x0064d4218a69e8e4ac6a6d3117ef47f8e22e70db46eeea8a8fb7e45dfdb92e18)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02d95f768fb62f0f7131e3f9e3514b54bdcd5ff4b42ee09930a137c7d589d786,0x03d38c203bd3fc6c8f28f96047c0cfebc3aacb0e15a7c58100e86fa125742c89)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0360e005362e357013be9882c63a75506deb0c850165328088648c489503328b,0x01c29c81466496fd77c61140ce9508aab371a38ba999c09caa34b8e616a84e2f)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x00ddf2968a00d805c8cae49ec95ab4fb429f4cb038c1ab32b94ca2e0f4058e66,0x057e26db0726472d99201f58835069c3a107ccc9026b33351ad4de2468ed4f2a)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0603da71b1d94e0689b06b44369529531d3643eb0e7ddfb352e694af27c4dd26,0x004d7f2c2713c017a4d0b3307952292ea1ad6035ac122f0cb409acd0a5a4994f)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02faf11a915b3e3e52a09b02bd5b9661cff6416e2a4faff9bbf2ab04eba2b8a9,0x028bcdf1ab82389ef7d929eab6724ac080dff7d884bea8b03a62550dbb066829)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0002c597606f3c69e0f8d3c5414bb6c59a91a5251ae624c1df16e398a8f1aa3d, 0x06f249ad242927b4840dd33693b7248e9211d65b344f4bb7aea332115ffa2bf0, 243))
                ],
                proof_post: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x071e49914c7c7df7cec59413de5d6789ec46db1c3e1177fac684c2b9bc518403,0x07afad7ef9e5748fa1df84452e99955c5d77032f7dbd6cd2fe4f1c0b89cea210)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05434ca9d413623081ae034f58282c5f80b72df4c0a3aa1053bffa5ea3498fd3,0x0295a571f0bad984d2702a3f858f1ef1cd9ac4455c30ba7719738e7c193e7fde)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01c6242c11c3338ae744b4a67d18a9463182c30d6a5ce98edf5ac51b574ffa53,0x07cbd644bd522d32c3900fdb457d37622d1ee08057a05f1124383dc29dea0118)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0461d40b2ccaa02e386060d530dc38e9ca67e7fa2780725a2ad6717ce3214bd7,0x03d38c203bd3fc6c8f28f96047c0cfebc3aacb0e15a7c58100e86fa125742c89)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x03d5488debe6a7e59796e1df4926037eee4728d8bf74a55089bf81b150232abe,0x01c29c81466496fd77c61140ce9508aab371a38ba999c09caa34b8e616a84e2f)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x00ddf2968a00d805c8cae49ec95ab4fb429f4cb038c1ab32b94ca2e0f4058e66,0x02eca9043adc837ae0a7cf19036b0f152b8c623474e989c2449ad117a682e347)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x064c2e335bf5d5dc94afed4df4651a21eacf55e28977a862a37da6c55da6cf02,0x004d7f2c2713c017a4d0b3307952292ea1ad6035ac122f0cb409acd0a5a4994f)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02faf11a915b3e3e52a09b02bd5b9661cff6416e2a4faff9bbf2ab04eba2b8a9,0x0685bf371be13e695d045266d00eb6bf7867ca22cf5e122801bd5648c3bb5c9c)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0002c597606f3c69e0f8d3c5414bb6c59a91a5251ae624c1df16e398a8f1aa3d, 0x07226f73aeee285271efc0cc4206179251515134d7f639867677c793f93d2225, 243))
                ],
            },
            LeafUpdate {
                key: 0x014e7ef518b10c591d90db9e48b74dccf23bad35ea596618f729dc937693c2bf,
                proof_pre: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x071e49914c7c7df7cec59413de5d6789ec46db1c3e1177fac684c2b9bc518403,0x07afad7ef9e5748fa1df84452e99955c5d77032f7dbd6cd2fe4f1c0b89cea210)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06aaf25cab52e360406330014ec1265ac4d9c964dc7d3130da38640c480719eb,0x064674c2dd30c5f2926097ef966e181873d84524131b887a80e47d72d6c12f42)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07760fd3b509208156525dc9b2a8c716870f9f0f79257a77b9b1398370aaefa1,0x020c8465a71caeba7c9be6c151686adbbf0fb17f35d38d12cd6e0daef1aac57e)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0271aca01d8ef9bb50c96801dcb08db792943315097e770efdefaa0bc321724c,0x01e500a318b71bc3c76a4e4e9472acce4afe0977651be66f93bf7bc7f8ebba1c)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x00980ec400d3a26613b2b4828616c5604b51442ce064b7f33a75bae4f32a2830,0x07bb6f8aa839a88429ffe52756a063743629aa867306d0c1e67b9dbb7e2821a6)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0360a3aa80fb012183c99dfb78b95b33ca60c6edb56124a96a981acaf93a19e5,0x07703ed0324cdcc79b5742a206ee386700b876d71aca985ed4222e7f2cc792fc)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04940bd6bad33922377d16fbf84f802814cb2411cad2a48eb10963138f3f34c6,0x03cb27cdd23707f76136f79d6368dcf31a7bfc2c17093194ce69275058b00f37)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05dec7258c5a299b6c7077f27eba7f3a74a900fdd4b3975321e4e49485870ff0,0x0494ee51b2efc70b48d83501f7f18f2335c18ae38dd3553dc0063144ff3781d8)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x00067ef518b10c591d90db9e48b74dccf23bad35ea596618f729dc937693c2bf, 0x0793be95fc1104d143558d530d77fc322290d5272d5d48e135659282ea6c1265, 243))
                ],
                proof_post: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x0756b36d8ee4fc8a502fd56c88b755a5e1fc9c1def7fe6181def197b646709d4,0x07afad7ef9e5748fa1df84452e99955c5d77032f7dbd6cd2fe4f1c0b89cea210)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01c49ab7636185827b096a62aeb79ca39a04b1dee6140569ee33ae30d94db692,0x064674c2dd30c5f2926097ef966e181873d84524131b887a80e47d72d6c12f42)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07760fd3b509208156525dc9b2a8c716870f9f0f79257a77b9b1398370aaefa1,0x050b1535fae79dd01c5b6ddee4e644735722a929acc48a608c3b7b176661f0a3)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05b1cd4cab750f8bc7908ab882f68502b7e27b62c038c82b30eea693275c4619,0x01e500a318b71bc3c76a4e4e9472acce4afe0977651be66f93bf7bc7f8ebba1c)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x00980ec400d3a26613b2b4828616c5604b51442ce064b7f33a75bae4f32a2830,0x02bfe33a37b958f07997e986814b9d5ab38892d6fe542678527c76e80ca02335)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0145eb4b05a267a38b659eba113ef86ee375a31b772622d789596adb7a3f4bb1,0x07703ed0324cdcc79b5742a206ee386700b876d71aca985ed4222e7f2cc792fc)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x018e4c1f5adcf81e089ff00f84b71459fcc23e06f89813d60bfa97b65db6ee20,0x03cb27cdd23707f76136f79d6368dcf31a7bfc2c17093194ce69275058b00f37)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05dec7258c5a299b6c7077f27eba7f3a74a900fdd4b3975321e4e49485870ff0,0x062b023656b90659b64f847d441677d293caea14fc9098a0eeb25039306fc1ab)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x00067ef518b10c591d90db9e48b74dccf23bad35ea596618f729dc937693c2bf, 0x0759496ec4b7c9ff91f8487621656f362d9730ed87b0402537e9fbd7687c8da6, 243))
                ],
            },
            LeafUpdate {
                key: 0x03e74c5ac6f1b59c0b5533bad6fcb840e4d5fcfb8bc04f6f8ddf724c2cdf4e90,
                proof_pre: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x0756b36d8ee4fc8a502fd56c88b755a5e1fc9c1def7fe6181def197b646709d4,0x07afad7ef9e5748fa1df84452e99955c5d77032f7dbd6cd2fe4f1c0b89cea210)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01c49ab7636185827b096a62aeb79ca39a04b1dee6140569ee33ae30d94db692,0x064674c2dd30c5f2926097ef966e181873d84524131b887a80e47d72d6c12f42)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x062c47db72a1e05f72392d6210f1970016e8d2246f69daa8eb187f54dfd8493c,0x02e702a5bbbc754e081fabf4a861cd27a9d6b539a1d3c592cd369e1912e34d29)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05b75817dec8605340113064f708de0c5b0532468c20e7ba1938c1c03dbfcd7d,0x043fe62081d3eb2adc6eaf61538fafe3c8877e9fdf29fd0f206cc86dfd2aad68)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01e445f4eb34369b32c36a80a1e0e8267bd77a2ed444c2bc623fea48272ad75f,0x0631d6c1e4774699c809443cf386bcbc6916a1b6bdb0531bc36dc0192d6d4df5)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0016a0f0d84baf66645b3b3c3f0e144f743f592f755e22fed6d987e50955f9ec,0x037efbd468d78b94a957fe0cb82c630202c84c4701b0cd6f8032f05465c63292)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02f0feee673baf7251cd8b05f38dc6a999cb6a4f75c76a1f831611c69df93396,0x01e1fd2ea55ddab789d90005961d194dab7d9f7d0277f14f390de5dd8c976b6e)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06f31a9b1b9c5b8697d946a80eb72c198770c30a878a28bb22447ff3ef9235e1,0x0655fd0528b379ef1e7e05ebcfa7a42eaba8cbd19ba8240c201c39d2ad9f8dea)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000001, 0x0106a3fbbb955ba54794c21b911abc7b118136be5114da0c0762f2b18d7ff705, 1)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x019afe0d3ab4c4f6db053a112563671de09dc5b115b471405a76f214aa036c45,0x0277c2a876bfd40c73200146d4bd95553e86f11211ed22ce6f5cafe5cf4cbcb9)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x00014c5ac6f1b59c0b5533bad6fcb840e4d5fcfb8bc04f6f8ddf724c2cdf4e90, 0x0499879f19536ba0c36ab8224e22c3d96406dde314c04c0dd797637d6122b9fc, 241))
                ],
                proof_post: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x001e98e684d48585db21cd549f6b5f4e8a55d58b3d3d4c6686703525e4203d00,0x07afad7ef9e5748fa1df84452e99955c5d77032f7dbd6cd2fe4f1c0b89cea210)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01c49ab7636185827b096a62aeb79ca39a04b1dee6140569ee33ae30d94db692,0x071104e55aa2e112c7e01506e4ce176ad9ff5cb2cb67ad8f8520292e1c3f41d4)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x062c47db72a1e05f72392d6210f1970016e8d2246f69daa8eb187f54dfd8493c,0x06785ea90bc05fbe37dd5f890e66c7e6de3e4938a8e8e88d966515e10657d405)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05b75817dec8605340113064f708de0c5b0532468c20e7ba1938c1c03dbfcd7d,0x027ae8e82080245dc0d1463aaac326c2f8e87576c6dff719fd07078beb2dbea1)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01e445f4eb34369b32c36a80a1e0e8267bd77a2ed444c2bc623fea48272ad75f,0x0458f3c06e05d1abd9b2cbcfd625c30de6f13aabffe3d5475c8e0ef47c61350b)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0016a0f0d84baf66645b3b3c3f0e144f743f592f755e22fed6d987e50955f9ec,0x05f5976a01d13c5a4c6fa2a60c261adda96fbea8bf9bf0479b7b7b0f7785c043)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01c53c67c8324df5672d1714545f37083d7df04d549371805515fa2cc798f0c6,0x01e1fd2ea55ddab789d90005961d194dab7d9f7d0277f14f390de5dd8c976b6e)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x03443d6d95f0354d571ad05dba978a518ab3d2adc1af62a15b039771e62071d7,0x0655fd0528b379ef1e7e05ebcfa7a42eaba8cbd19ba8240c201c39d2ad9f8dea)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000001, 0x05435061ebea16ffb2050f5143f4bcccffe8871cf03f3d562ba34cafd8adaaab, 1)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x019afe0d3ab4c4f6db053a112563671de09dc5b115b471405a76f214aa036c45,0x02edb6bb34fbcb68727f4ed1875f99d848c8d9bbdf780b6b54bd990187c5de77)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x00014c5ac6f1b59c0b5533bad6fcb840e4d5fcfb8bc04f6f8ddf724c2cdf4e90, 0x06b991e40e326549bea20148d8e11b37677664c3a6773116a5627c721663b70d, 241))
                ],
            },
            LeafUpdate {
                key: 0x027f73e6c94fa8249ec9f2f4eec607acc97fa632c9e8fb6c49437e62390d9860,
                proof_pre: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x001e98e684d48585db21cd549f6b5f4e8a55d58b3d3d4c6686703525e4203d00,0x07afad7ef9e5748fa1df84452e99955c5d77032f7dbd6cd2fe4f1c0b89cea210)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01c49ab7636185827b096a62aeb79ca39a04b1dee6140569ee33ae30d94db692,0x071104e55aa2e112c7e01506e4ce176ad9ff5cb2cb67ad8f8520292e1c3f41d4)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x062c47db72a1e05f72392d6210f1970016e8d2246f69daa8eb187f54dfd8493c,0x06785ea90bc05fbe37dd5f890e66c7e6de3e4938a8e8e88d966515e10657d405)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x045985b15cbedaa14e73131b939490be7680b9ec7a566940dcafc63510b98667,0x05025c2eda6862225a883a743eb85a7b9ec58db8b99a10347afaf3cad0fc363e)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07d5cf9d86e6b2e9e0b5a467cd0168ff7e584d2d92d9aa5ba5fd4305b25544f4,0x00c4e631b03e80dddc7c8bf3c09fb040726e29065b02d128a958ce130481e7a8)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000001, 0x039503b1f5ba21d3c89c16146a7dd52e3000fe42596ddc6500648fe965c5c956, 1)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x071dbdf23dc054bb3e6117a66038a43f1be5afce077d4a8b614c8b2050f92909,0x04c9ec4534bb23691287896552ec9f665bb2f0e92f6e54b338a4a0c6b29a372b)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01c263f78ab468b5a9e745b535c94dae31bb848424fa0523f45e95a3a3319af9,0x05297a668cd8485a546c171da92dc28d92814cd98f1507169cc5685aaad2ad3d)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07d8a7e9f56637d8f661bd63a1e25703e7db750e34ccfef09a8cb177b0355a2c,0x02e83bd011332ad820a56be1d014333153ade55d0bb65c1b6a5c8348ff90da7f)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000373e6c94fa8249ec9f2f4eec607acc97fa632c9e8fb6c49437e62390d9860, 0x01ee74936a80bc9a247c3b193a307da2505e35ea7f37eea07f90f6b973c4f472, 242))
                ],
                proof_post: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x04ebd5bcbd212a64a6935a58e2b60895a51b399e63e2c642346f89057050ddcc,0x07afad7ef9e5748fa1df84452e99955c5d77032f7dbd6cd2fe4f1c0b89cea210)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01c49ab7636185827b096a62aeb79ca39a04b1dee6140569ee33ae30d94db692,0x0649792412e5a3f46d61eeee678ea18566546b57e85ceb5eb8d0cd23cf6658e2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x013269ed8d8f3caaad4a59264cb9e08cd7cecec4e7b0f628aa78117ad51b488b,0x06785ea90bc05fbe37dd5f890e66c7e6de3e4938a8e8e88d966515e10657d405)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x068bebb4ef2635b170c0b68a734878611f181a1694ea4655a68c02b1c10ae4f5,0x05025c2eda6862225a883a743eb85a7b9ec58db8b99a10347afaf3cad0fc363e)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07d5cf9d86e6b2e9e0b5a467cd0168ff7e584d2d92d9aa5ba5fd4305b25544f4,0x054eacd7500726538669c2c5b808518b75d0a77adf590ecfb9c6f3d23f4b03c2)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000001, 0x0146a1c29f0dd41c4e340bb6cd033f440798ec4edb6f9557da59b4fc3da92f78, 1)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x071dbdf23dc054bb3e6117a66038a43f1be5afce077d4a8b614c8b2050f92909,0x07338b71e07b5c7d28c354eb6beace33ed03cb2d797ca587ecb97ba99eeb3a2e)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01c263f78ab468b5a9e745b535c94dae31bb848424fa0523f45e95a3a3319af9,0x07af299cf809dd28a661d2e1e0a4a8f4da95f824aa3963c7699fec619c285815)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07d8a7e9f56637d8f661bd63a1e25703e7db750e34ccfef09a8cb177b0355a2c,0x00047c49f44aead54c126d6a43ca087a16e27075f4320ebe10342ccb1e067913)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000373e6c94fa8249ec9f2f4eec607acc97fa632c9e8fb6c49437e62390d9860, 0x03a6d33d424e9f721a55b0cb2bf6df1d6b238d38b8e5d8ea1a18a57c627918cb, 242))
                ],
            },
            LeafUpdate {
                key: 0x056336e107e48a10985acbe5050f915a9bbad6470eb2ab797ccb93c00a525f7d,
                proof_pre: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x04ebd5bcbd212a64a6935a58e2b60895a51b399e63e2c642346f89057050ddcc,0x07afad7ef9e5748fa1df84452e99955c5d77032f7dbd6cd2fe4f1c0b89cea210)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05434ca9d413623081ae034f58282c5f80b72df4c0a3aa1053bffa5ea3498fd3,0x0295a571f0bad984d2702a3f858f1ef1cd9ac4455c30ba7719738e7c193e7fde)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05e2ae485981857ab0061aee2e2a82f5cfa69b35dd86919ecd909604d141e9e9,0x0002ad28ca01bb59bd21475c963e0237dbaf1aa198355738c6c4cba0f9ad1021)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07c10cfab9369397e8a9d4cf69e551fd58bf93975c8bc8da2ae29706cbfea287,0x06cc33e97e72aaab6534c3055b0e975e8ad63f9f38c4486d172da8e7ae47b3f0)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02945ee160c70e51a707f7b4cf82127348b44542538c3c7c135b8b861560e5ec,0x0618a9f0cd1f1f15805aa18dc2b1c13a9485e57a39c27b37906e3c2b7a846c86)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x066982841b391b455750e18cd77dc8c073d547ba9dc5c0c95b401363ddb6c00e,0x04d4258156c9f6e15ad374a2f5a5dfeab175b916152032bcd955f652997c1daa)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05715613e52621dc77234c85cb79112da87f877383ec9e541c3f9344e24413a8,0x002e3e1a15f2cafbced8f1126bc8526da4e545029d97c924815eecaa505e3c83)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000000, 0x01892bb1c95b2a422349e4058f3172405688954787ec80070735f8131529932a, 1)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x071ac50a2a7ede77c3e2a4a0f7998af78794d6fa67b1a7f5b3501f39acf1a2ff,0x040a4b131c9b36d6920452d92b3cf375b1001e035ff87edc71654ba4fdf2609d)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000336e107e48a10985acbe5050f915a9bbad6470eb2ab797ccb93c00a525f7d, 0x07b5a3416a8465d474f40371c6ca61fd54d7644abc5ffd38055ee575294c1dba, 242))
                ],
                proof_post: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x04ebd5bcbd212a64a6935a58e2b60895a51b399e63e2c642346f89057050ddcc,0x0275ab10e9207d5c549cf31e042f5ea69a3ed36a3be100e8376bf2af1c9659c2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04abf2001ba2214b0a55dc19ddcc12516d1bda81d17e121ade01cc4731cf95ce,0x0295a571f0bad984d2702a3f858f1ef1cd9ac4455c30ba7719738e7c193e7fde)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05e2ae485981857ab0061aee2e2a82f5cfa69b35dd86919ecd909604d141e9e9,0x032fc43ac55618d2d09b635dd2c22fbf42162d4e4dae95db0b67b985b406f83a)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x03ea532c8de27aaba0f59a2337afc6557d4099a259636c22024befd64b19861c,0x06cc33e97e72aaab6534c3055b0e975e8ad63f9f38c4486d172da8e7ae47b3f0)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02945ee160c70e51a707f7b4cf82127348b44542538c3c7c135b8b861560e5ec,0x05cf99e3f685d45ca1501b6044f102c77de782cadc5d5531e217b7ad1442d3fe)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x066982841b391b455750e18cd77dc8c073d547ba9dc5c0c95b401363ddb6c00e,0x02babc1f36532691e06299cd75cedc9b363b0fe53d2704013bb3cac80f6a848a)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x062677810c6f3ad8f20c8f6b8acc231a6216738b07ad688cf6d1845f9466d42b,0x002e3e1a15f2cafbced8f1126bc8526da4e545029d97c924815eecaa505e3c83)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000000, 0x059a06b0e6f8143d7c050982b21812dd237c33cf89e88fa6160d871311fb3bda, 1)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06770b6ffe2a81e1dba4fe5deb11522fd883068caf4562d926565ccb56416586,0x040a4b131c9b36d6920452d92b3cf375b1001e035ff87edc71654ba4fdf2609d)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000336e107e48a10985acbe5050f915a9bbad6470eb2ab797ccb93c00a525f7d, 0x06f6e03ac3323f5a6b71a077d91d7c048e512e2f934b3fecd2ab766d40fa0241, 242))
                ],
            },
            LeafUpdate {
                key: 0x0092d2279ee29c8f093ed406d6c7c30cf238285fe71193a08174bfa0a7a171d2,
                proof_pre: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x04ebd5bcbd212a64a6935a58e2b60895a51b399e63e2c642346f89057050ddcc,0x0275ab10e9207d5c549cf31e042f5ea69a3ed36a3be100e8376bf2af1c9659c2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01c49ab7636185827b096a62aeb79ca39a04b1dee6140569ee33ae30d94db692,0x0649792412e5a3f46d61eeee678ea18566546b57e85ceb5eb8d0cd23cf6658e2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x07760fd3b509208156525dc9b2a8c716870f9f0f79257a77b9b1398370aaefa1,0x050b1535fae79dd01c5b6ddee4e644735722a929acc48a608c3b7b176661f0a3)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04bf64b568efd601638640d3b1b99bfc98f10957df05af6af949810dab25b27f,0x0209bad859f8a8280462488d69443dcc02b396d2d42e95f1b8b5d63d60515456)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02895db89f9509dffe52046e37bf0d34c9b821830ace715bfdc72169a4dddfa0,0x030ee94f5968f4840bbf1187df6213816621523b17e2826e585b469f11904777)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x02cd17873754630a9ede9ef659729c04df53f131a30b780f1982ed4aee65f90e,0x07ff7111f29f63b959f4b2f6a5a4b1ff4936c54f80c893882c3f4fc52e66ba43)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x00d6b1faa7f581a0ce6b91690da438536426e6064d4dcf9d8e359b85f6738510,0x056f597667c578094c660b057b07f19dfa336e38257fa93be69850f94f7bf9d1)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000000, 0x00556f0cdad9beb0380498cc13e694b58e821d321faa766a54e1bca476837897, 2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01d22a55cfd0823f3daf575b55456ef56ddcc2e387cf364ce8ae74c7ace70227,0x049eac6344cd9c52b00e2b844a92ab3c586e18ef9fd502a5097aa540d014326d)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000d2279ee29c8f093ed406d6c7c30cf238285fe71193a08174bfa0a7a171d2, 0x026081d5a80ccd9159a9dd5222b172801310ddeb8bafa9df0686247a548ecf53, 241))
                ],
                proof_post: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x00ac97143fdceb90478866c3e1e259049b6caa6033194bce5b7a282d131578bd,0x0275ab10e9207d5c549cf31e042f5ea69a3ed36a3be100e8376bf2af1c9659c2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x043a24acccb2f4d8afe38b8c5ef7be26b02a197233b8ffe8131464d090330589,0x0649792412e5a3f46d61eeee678ea18566546b57e85ceb5eb8d0cd23cf6658e2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x00d415a868a3d07fa14035939410a96f3518bba483cd9c3f75f58b20f4abe27d,0x050b1535fae79dd01c5b6ddee4e644735722a929acc48a608c3b7b176661f0a3)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04bf64b568efd601638640d3b1b99bfc98f10957df05af6af949810dab25b27f,0x07f64f5881cd4c38d8ac19379703bce5a1c16b1e7bd63b55d7062c7993576882)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06d81c4a0a33ccd1d29c6fd0904a9544090354dcd85785a80c1829b670224ead,0x030ee94f5968f4840bbf1187df6213816621523b17e2826e585b469f11904777)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01464f6055748740942c1bc488b9f7a8c739f252a73133d4e9c72d67db0711d6,0x07ff7111f29f63b959f4b2f6a5a4b1ff4936c54f80c893882c3f4fc52e66ba43)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x00d6b1faa7f581a0ce6b91690da438536426e6064d4dcf9d8e359b85f6738510,0x038b138352b77057ce905ef7a3fc86a7be6198117bcc5263a67c5ad1b69e3e25)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000000, 0x07b2318bc74fd273110929790cfafa9821e4852c499f4135cf7fe66bf515e2a9, 2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x01d22a55cfd0823f3daf575b55456ef56ddcc2e387cf364ce8ae74c7ace70227,0x076924f10894643b682105314c128a52834a6bea500ae820379f6c70a340ec11)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000d2279ee29c8f093ed406d6c7c30cf238285fe71193a08174bfa0a7a171d2, 0x042e3c1d0e5e373f174c09da541a2b09642a84b1cd57dc93822b5b396aaa9544, 241))
                ],
            },
            LeafUpdate {
                key: 0x04c159a003f615764e97abe97f0dd8e9c6969a31784167286b803de1952e8ab1,
                proof_pre: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x00ac97143fdceb90478866c3e1e259049b6caa6033194bce5b7a282d131578bd,0x0275ab10e9207d5c549cf31e042f5ea69a3ed36a3be100e8376bf2af1c9659c2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04abf2001ba2214b0a55dc19ddcc12516d1bda81d17e121ade01cc4731cf95ce,0x0295a571f0bad984d2702a3f858f1ef1cd9ac4455c30ba7719738e7c193e7fde)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05e2ae485981857ab0061aee2e2a82f5cfa69b35dd86919ecd909604d141e9e9,0x032fc43ac55618d2d09b635dd2c22fbf42162d4e4dae95db0b67b985b406f83a)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04279a2f00bd6cea790c95be5a0ddd28d766f6f63f0f06bd731142a4cd391dfb,0x018cd404c8f5a6c585cfe578622cb8db65b6ea891a4e46390f459a283d517859)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x03a0f92869d8b88363d68732724e45893d8926506c34ce8fdb174f31b10f2a2e,0x00b8a4fc5ab1a6ab0495dbed05c93c4ea98ad91045790942de2afee5816617f9)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x045d34cb72fe0245d1e209da6b13a35fad9c03346c66e8898f78ed9e7bdf8f79,0x002b0a3d806dab7d41fe5e49c1f6bd2e160d1a21819ac8c987d9d5cd67b2dbb6)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x030b02d5bc1c9798bfe0bba65659a3899dbfdc7eeb0d479f5837ad81f7feb67f,0x072400420380a2764d85eeccdc5dd1307cf5d13503dfe6cdf9e346e53980a8f2)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000000, 0x0539c907cd44280a6c4dbaaa6d2de12a636b9e36d7b3031a18e70e2350b26fbb, 1)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05d807b1473e760f57677904f999d946f19474cbe70a53ca0d812ca1e17de087,0x0641503b093bab815208db3e3878a37be9d3e9f6e83963018217d9be4e5616da)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000159a003f615764e97abe97f0dd8e9c6969a31784167286b803de1952e8ab1, 0x01b7eac954fac28deadce691fee40b6d889b474ede80de1014499e2cb6fac44b, 242))
                ],
                proof_post: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x00ac97143fdceb90478866c3e1e259049b6caa6033194bce5b7a282d131578bd,0x02084491bf92f4286c97d807b9800fc367773b00e1e0dfd58a9fc52cab986cd6)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x006d6527ba75c8b47f930ab35f8c5e19929f909970707f1b213c30c7b5f5e4d6,0x0295a571f0bad984d2702a3f858f1ef1cd9ac4455c30ba7719738e7c193e7fde)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x075f2b8cb6f917e299228e5aa6b38757cee943347510073f0dddf2bdb0ce574a,0x032fc43ac55618d2d09b635dd2c22fbf42162d4e4dae95db0b67b985b406f83a)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04279a2f00bd6cea790c95be5a0ddd28d766f6f63f0f06bd731142a4cd391dfb,0x03c830091bb6a448a9f0118fe9fc5c34706f1246d74a1757abd3f75b3b93866b)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x03a0f92869d8b88363d68732724e45893d8926506c34ce8fdb174f31b10f2a2e,0x01b1c47f1b822652fd175b42d10d874ae35d03c3fd7a0afc20534370e3fb542d)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0729b8b512bf4e10f3a932ce30c524f84b284890ba3ea62d4f8e5bd2b625c80e,0x002b0a3d806dab7d41fe5e49c1f6bd2e160d1a21819ac8c987d9d5cd67b2dbb6)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x05f3616c7db32874272d14dae3d09f039f015200baf905ec5b8da54aac2c6434,0x072400420380a2764d85eeccdc5dd1307cf5d13503dfe6cdf9e346e53980a8f2)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000000, 0x033c62a99f4e37031c3a10b7687ed382314b0bb6cbf90cc5a34b94f531052bf5, 1)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0022bfd661aa3b775057b5674e2a4c6a8b84c7dce025b3c252529b263061a2cf,0x0641503b093bab815208db3e3878a37be9d3e9f6e83963018217d9be4e5616da)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000159a003f615764e97abe97f0dd8e9c6969a31784167286b803de1952e8ab1, 0x067ff789287d37792d8ae09b4ef48af604a6f327bea6cdaa29048ea39a249a6b, 242))
                ],
            },
            LeafUpdate {
                key: 0x006756e62cedb9ec5bf198eb6a6263fd966d647d8940271badafd15f743bc607,
                proof_pre: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x00ac97143fdceb90478866c3e1e259049b6caa6033194bce5b7a282d131578bd,0x02084491bf92f4286c97d807b9800fc367773b00e1e0dfd58a9fc52cab986cd6)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x043a24acccb2f4d8afe38b8c5ef7be26b02a197233b8ffe8131464d090330589,0x0649792412e5a3f46d61eeee678ea18566546b57e85ceb5eb8d0cd23cf6658e2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x00d415a868a3d07fa14035939410a96f3518bba483cd9c3f75f58b20f4abe27d,0x050b1535fae79dd01c5b6ddee4e644735722a929acc48a608c3b7b176661f0a3)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04bf64b568efd601638640d3b1b99bfc98f10957df05af6af949810dab25b27f,0x07f64f5881cd4c38d8ac19379703bce5a1c16b1e7bd63b55d7062c7993576882)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06508f7671b133fca3589aaef6615fc074581e5d7aa2ada69e2a1181ae9805d9,0x00aa90ac7ca0a5657c7787f6da1d8fd933f827f9bf234483bceaea5d8c8f6dbe)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x075782922383e0759a4aa274fd3c656ceb7c612549c8af3a980b4faaf6bdeda6,0x006ff324ac065239b9eba703a2b4cfcb5ad7e149db2c5ebd4293045e3131381c)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x03d179e173a249f0bca5ec92bfe1f018ccd86ea1568a9300452635f4519a8fbb,0x07f41362c719162e87f2b91bba1e42b657e7874784d7108bc8fafa20c7a250d7)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0657542c169b620cf63e7ecd14895db942275c7f31833ed8c341dd8a05fe98ec,0x00682b334811b7f55f0c2c3b77fc742133ff61bba6ff41ebf2a80b5d3d60199e)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000000000000000000000000000000000000000000000000000000000000000e, 0x07e8174167c6b35ac39699668e8165b1520c6791e75ce401409f4de34ec1ffc6, 4)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04f9f6a0890f91079cd8d3ce08c6b5c1c277ed3ad7040e474d68a47e273e68e4,0x04f9f8fc0acf7fd53ca549dfd8c34f80a629d5e3d9bb974d00116c7162cd036f)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000016e62cedb9ec5bf198eb6a6263fd966d647d8940271badafd15f743bc607, 0x0037245dac0922c04b163b91abd56a901ae209acccf309baca1be30ccf09f91c, 238))
                ],
                proof_post: array![
                    TrieNode::Binary(BinaryNodeImpl::new(0x072a6e15683fb0af95afaad2cfcff44f58f6673d85a8746fc02a66eab0ebe332,0x02084491bf92f4286c97d807b9800fc367773b00e1e0dfd58a9fc52cab986cd6)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06277cca152c1c1880ddf5d95fed1101cdc45760c38c5be0bd37794dbe26aef9,0x0649792412e5a3f46d61eeee678ea18566546b57e85ceb5eb8d0cd23cf6658e2)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0493ee0f946c744dacd53f3ad58a58ce955540acb96c263945cc219928c031a4,0x050b1535fae79dd01c5b6ddee4e644735722a929acc48a608c3b7b176661f0a3)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x046a7d63bc8bd683ff014217fdb4b1d27447d6457c924941ca1cf8dd928733ad,0x07f64f5881cd4c38d8ac19379703bce5a1c16b1e7bd63b55d7062c7993576882)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x06508f7671b133fca3589aaef6615fc074581e5d7aa2ada69e2a1181ae9805d9,0x054e6abfc6790d3f8b5f2be25887764641001f271566f84df78f9282a31dade6)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x075782922383e0759a4aa274fd3c656ceb7c612549c8af3a980b4faaf6bdeda6,0x06e835d66495e32688d2efe80c54f1b16558b0b5b02c9fbb6b316d0cb7b41d27)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x0760f9955f136ed30a1066f97eac894195c48582cc8a498ae4a5aa89426ad24a,0x07f41362c719162e87f2b91bba1e42b657e7874784d7108bc8fafa20c7a250d7)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x037d97f24d037bc33566ce9895561c9a2015b98774e114ef459f9f16dde0eb89,0x00682b334811b7f55f0c2c3b77fc742133ff61bba6ff41ebf2a80b5d3d60199e)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000000000000000000000000000000000000000000000000000000000000000e, 0x0798fa2b45275f1e01a6e2772fbe4f00f14ed7a58dcc83dd08acdefc02d28770, 4)),
                    TrieNode::Binary(BinaryNodeImpl::new(0x04f9f6a0890f91079cd8d3ce08c6b5c1c277ed3ad7040e474d68a47e273e68e4,0x05e2c70baa71dc3a2d6a2dd265024c5092e90cf75a8aa4d50f986b05508e05e8)),
                    TrieNode::Edge(EdgeNodeImpl::new(0x000016e62cedb9ec5bf198eb6a6263fd966d647d8940271badafd15f743bc607, 0x00fa4e2c03100cd5cc78d8e7542f93cc563d6f6e81198dc0b71f17fab5d27745, 238))
                ],
            },
        ]
    };
    let computed_root = update_tree(batch_update.pre_root, batch_update.leaf_updates);
    assert(computed_root == batch_update.post_root, 'Verfication failed!');
}

pub fn update_tree(root: felt252, mut node_proofs: Array<LeafUpdate>) -> felt252 {
    let mut new_root = root;

    loop {
        match node_proofs.pop_front() {
            Option::Some(proof) => {
                match verify_leaf_update(new_root, proof.key, proof.proof_pre, proof.proof_post) {
                    Option::Some((root, _)) => { new_root = root; },
                    Option::None => { assert(1 == 0, 'invalid leaf update') }
                }
            },
            Option::None => { break (); }
        }
    };
    return new_root;
}

            
#[cfg(test)]
mod tests {
    use super::{LeafUpdate, update_tree};
    use alexandria_merkle_tree::storage_proof::{EdgeNodeImpl, BinaryNodeImpl, Membership, BinaryNode, EdgeNode, TrieNode, verify_mpt_proof};


    #[test]
    fn update_test() {
        let root: felt252 = 0x04946c7636b878064ddaab68a34c440f7961b854934e9b7347d981f9f5f768f2;
        let key: felt252 = 0x008b6cf864557c0a7beac7637c50ef1035602ad49bd7005dcaa5a18e2d8ec848;
        let proof: LeafUpdate = LeafUpdate{
            key: key,
            proof_pre: array![
                TrieNode::Binary(BinaryNodeImpl::new(0x01f387675be9603ca64faee950fc41c70c4e8c534fb9ac6d235bf7ef732fdbb6,0x01bdac12073dd4161c28740f654bdd4162aad1e8c0588989f7d05977fc5ef41e)),
                TrieNode::Binary(BinaryNodeImpl::new(0x07cddc1ceb86f6e1187cd2d76f14e54fc98c0b26817febbcdf62ee02b5ae4b2a,0x025c7b135c096bfccf748e7c653c58c17868a1f0d8fd388b16e8533460da6558)),
                TrieNode::Binary(BinaryNodeImpl::new(0x07a68c7086fd19839b8c3949f166ff04b79431c177933768d4beb9c4289f1aef,0x05a6b807d4c79bc17a973171b3173803298237e18ddcfbca64f4887f01398207)),
                TrieNode::Binary(BinaryNodeImpl::new(0x006e6b797dd63db4bae624f72a21da383897819fd00c42d1a854bc8145adb6dc,0x007287cb5e4eb045be77f5d1b3f65005d1575f957532fd5278174561ec9094f0)),
                TrieNode::Binary(BinaryNodeImpl::new(0x078de8cff84d9c0d9185556c88aa8bfb7e2bc96f7f8b46c5a302e541c87dea81,0x01e890645ff0edd6a9bcd780c13f7f6d849aaf74b5b4bbaf5aba87b6c02460aa)),
                TrieNode::Binary(BinaryNodeImpl::new(0x042cd49dec469d13fba40a7bd94251c2ac1b4c3e496b7d148dccfde067d517ee,0x04320769e7113054ee20e812eb1f43086692e493e6d80ada3342c0788d6f390a)),
                TrieNode::Binary(BinaryNodeImpl::new(0x0719ca46679d3bd711d02b4443be40805894caea3aa8927f5cb55c6315c2b6f6,0x051e0b01f8875c39b55a59c02ecdc85e5448827dc4012baaadd530b4b6d8c55f)),
                TrieNode::Binary(BinaryNodeImpl::new(0x04933b5c205d0161817486e6d6b59da1034d3629f58d71cfd5c75872fcbf9b1b,0x046a498d1e3e6c6de427bd6b9f924de07ff77229a0a27f7cc87b6b2d55c0908a)),
                TrieNode::Binary(BinaryNodeImpl::new(0x04efbe5545b48f074fa979cf4530adca86a306fb26d03e194757934437c0ae89,0x032c523fbf915ac197377cdf8aceb64a5856a118fe22d71bf9a4f1113c27ecd4)),
                TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000000, 0x000b93d5d8fee7730b679ccfd3aff58c89867f2b83067780ab3e95e7ac0ce3a5, 1))
            ],
            proof_post: array![
                TrieNode::Binary(BinaryNodeImpl::new(0x033121a6bec83dd586c0b17124a9ffe0d1d5a6e883c38553caa4e189c356c38d,0x01bdac12073dd4161c28740f654bdd4162aad1e8c0588989f7d05977fc5ef41e)),
                TrieNode::Binary(BinaryNodeImpl::new(0x060dc0058ed64c0c7713fb66ea70a1c43023d242145e2d771db8dbc3f53149c7,0x025c7b135c096bfccf748e7c653c58c17868a1f0d8fd388b16e8533460da6558)),
                TrieNode::Binary(BinaryNodeImpl::new(0x0310c415d87024f7df81bf241f52a01a5d731dfebe5538036edbc8e53ce0d8d3,0x05a6b807d4c79bc17a973171b3173803298237e18ddcfbca64f4887f01398207)),
                TrieNode::Binary(BinaryNodeImpl::new(0x006e6b797dd63db4bae624f72a21da383897819fd00c42d1a854bc8145adb6dc,0x022051767faa6bdbd772667c9cfa53576ea170f327e07ab04cbdeb81b3e20b78)),
                TrieNode::Binary(BinaryNodeImpl::new(0x017e6a1a970009adfb199df2a7265f663d898bfad2d38f00d517cc46c5016912,0x01e890645ff0edd6a9bcd780c13f7f6d849aaf74b5b4bbaf5aba87b6c02460aa)),
                TrieNode::Binary(BinaryNodeImpl::new(0x0102b8eb55b6270659696551d2f80d36f4169b1628a51bae46fdc4c626853e8a,0x04320769e7113054ee20e812eb1f43086692e493e6d80ada3342c0788d6f390a)),
                TrieNode::Binary(BinaryNodeImpl::new(0x0424fc25974187c6c27dbaa91d10803c9f527cbdc33f81a0944b04e10609a90e,0x051e0b01f8875c39b55a59c02ecdc85e5448827dc4012baaadd530b4b6d8c55f)),
                TrieNode::Binary(BinaryNodeImpl::new(0x04933b5c205d0161817486e6d6b59da1034d3629f58d71cfd5c75872fcbf9b1b,0x02cc111cdecb102b96587471a6ba3df3b847bf5480d61e908a22175940514e0e)),
                TrieNode::Binary(BinaryNodeImpl::new(0x06d664dc734b626afbaad9d7fdee65d9704fccfa276bc8b49c39b53d11774ab6,0x032c523fbf915ac197377cdf8aceb64a5856a118fe22d71bf9a4f1113c27ecd4)),
                TrieNode::Binary(BinaryNodeImpl::new(0x000b93d5d8fee7730b679ccfd3aff58c89867f2b83067780ab3e95e7ac0ce3a5,0x07e89300e3a408a22d04bd988a828a20f37727665a0db717dd4e8fcf8000c711)),
                TrieNode::Edge(EdgeNodeImpl::new(0x00016cf864557c0a7beac7637c50ef1035602ad49bd7005dcaa5a18e2d8ec848, 0x04b3dccd37b5e769990d4d52deb17899408cafb1b5bfb5ac977d79e64b9aa9b7, 241))
            ]
        };
        let expected_root: felt252 = 0x0724e99d15daabe91301b0383092c68f907464c84a18ea65bec255a3eb871ec1;
        
        assert(update_tree(root, array![proof]) == expected_root, 'it works!');
    }

    #[test]
    fn verify_mpt_proof_correctly() {
        // Test Inclusion 
        let root = 0x04946c7636b878064ddaab68a34c440f7961b854934e9b7347d981f9f5f768f2;
        let key = 0x068ba2a188dd231112c1cb5aaa5d18be6d84f6c8683e5c3a6638dee83e727acc;
        let proof = array![
            TrieNode::Binary(BinaryNodeImpl::new(0x01f387675be9603ca64faee950fc41c70c4e8c534fb9ac6d235bf7ef732fdbb6,0x01bdac12073dd4161c28740f654bdd4162aad1e8c0588989f7d05977fc5ef41e)),
            TrieNode::Binary(BinaryNodeImpl::new(0x00f618893b917332d0f527205f94ab7bd8ab20da5b60be66f22b90d08100aba4,0x0001e996b4969876b0d24e9c9446f9aa9efef959462e2a9e800945942d4d0b63)),
            TrieNode::Binary(BinaryNodeImpl::new(0x07cf400d095212c51ce3320971de539643d8eabaffb4c2a4369e222f3606f03a,0x0321dba462885b2a1969701b2c9efebb1407a0c1108fe27da6f54db7808721d2)),
            TrieNode::Binary(BinaryNodeImpl::new(0x02051cd969f864999bb8849bc81b7f9269da475cfe3917572ee45019c1848bbf,0x06a08b555530572d8ef951bcc258775dd8f9a16491b5ef9ce1d6487469b09cd3)),
            TrieNode::Binary(BinaryNodeImpl::new(0x0484d955456ffe836bc76ca68ed0346f8000d49b1d4390752bdcf463fb9ff8df,0x075bbd3fefcbc80a4bfa9f7966fa922ef1e8b06296c74fdb027fccf86e1f8653)),
            TrieNode::Binary(BinaryNodeImpl::new(0x03356b189a13b373a5b314df50ea23890ae30238f2874f5c0f6c43d48c478a9b,0x052bc30d623a2e5250bba867a6caad82fa0b4c7fae8bb1c14322d5a254dd6a95)),
            TrieNode::Binary(BinaryNodeImpl::new(0x01098d6468cf066f501660a61a8d75190a3210fedc18c847be02c195d6722188,0x04d779eab03f4c8ee3c6e1bbdf7ca0cd17118dbde0d466d802d7712f0b65e4ba)),
            TrieNode::Binary(BinaryNodeImpl::new(0x0564dc25df5ea2d3c2ce68eec7295b65377b7ce36d5ade782f863391f6a442b3,0x077693e2d55eca151f494e6dec1cf291b0c62cc03b0f5c9d84b044737345fd04)),
            TrieNode::Binary(BinaryNodeImpl::new(0x01c96fd8ff1ef78091004a87badb3640cfb7ce0254f01bcad717ab0d81f9649f,0x0346c845e8efd9360317e8b4b80e2969082ae4a6c3425fe84791df12b4c68fa6)),
            TrieNode::Binary(BinaryNodeImpl::new(0x00a3857a5d171d57bcaac627c2b79eda657ae37ad97837c8e056c2edeb802495,0x00fd425d1e7f42d5c25501a3ce65c57520e1c45b459794b99c304a33315065eb)),
            TrieNode::Edge(EdgeNodeImpl::new(0x01a2a188dd231112c1cb5aaa5d18be6d84f6c8683e5c3a6638dee83e727acc, 0x02c25f304f5a12b4497a481a363a32a40d5891e4360dbd9de61f0117703f1f36, 241))
        ];

        let res = verify_mpt_proof(root, key, proof);
        assert(res == Option::Some(Membership::Included(1248050996070649414396845444796915005109664324995260892124683528808398004022)), 'it works!');

        // Test Inclusion Multiple Edge Nodes
        let root = 0x04946c7636b878064ddaab68a34c440f7961b854934e9b7347d981f9f5f768f2;
        let key = 0x0759f32296ec292b2b4fdf4ce2ab51314bedb9be3456b18689279274c62004c1;
        let proof = array![
            TrieNode::Binary(BinaryNodeImpl::new(0x01f387675be9603ca64faee950fc41c70c4e8c534fb9ac6d235bf7ef732fdbb6,0x01bdac12073dd4161c28740f654bdd4162aad1e8c0588989f7d05977fc5ef41e)),
            TrieNode::Binary(BinaryNodeImpl::new(0x00f618893b917332d0f527205f94ab7bd8ab20da5b60be66f22b90d08100aba4,0x0001e996b4969876b0d24e9c9446f9aa9efef959462e2a9e800945942d4d0b63)),
            TrieNode::Binary(BinaryNodeImpl::new(0x07cf400d095212c51ce3320971de539643d8eabaffb4c2a4369e222f3606f03a,0x0321dba462885b2a1969701b2c9efebb1407a0c1108fe27da6f54db7808721d2)),
            TrieNode::Binary(BinaryNodeImpl::new(0x04a2a67938a30148e2122d6fcf1f4d2a8a3d0aae210561172ddd464883ce7b21,0x0260a7f8ec1bcc0daaf25fd848a99971aa7671a80bc70f7807383948d42c92a6)),
            TrieNode::Binary(BinaryNodeImpl::new(0x028cc7e12e96accc03f46be90dfdfd71a8784554ad141bebf945d94eff9491ec,0x073b2437cd6a589e8d71239256386f2058b1f47335ec3fbf7e0f494d3a76ef00)),
            TrieNode::Binary(BinaryNodeImpl::new(0x03687a62fa62b5f7591834e9f7f9a9719d9509c1d17ed62d0f2a59c36903c84a,0x030423db6cab22bac5b3a76669268df1400a33ad09a6fed3231b726592c5ff64)),
            TrieNode::Binary(BinaryNodeImpl::new(0x0657f678f1ab4cdbd1ee8c8155bfdb8829072c8d7a6075623942f16dcaab9356,0x069568988fe00ebacebe7e5a04841e7e7d16fd96d9d4691fc40826f205ad216e)),
            TrieNode::Binary(BinaryNodeImpl::new(0x05f10727387bfbda71818a4c020c3621c39e133f2281aef38107375d3ee552a4,0x04f57665e066d0977f1866f648a39f74dfc8f6cb523a74044a04676e29a9ae5e)),
            TrieNode::Binary(BinaryNodeImpl::new(0x05a306e9abc76fb78fcfb687badf05505c233ee7c4f29a130273fe4e6e7c6076,0x0575d889875d899cd31bdc5b6485c39f79df834fbfb2180f2761071cb6844ccd)),
            TrieNode::Binary(BinaryNodeImpl::new(0x006fbb6151a9792d92068ab775d981caf944e1426d042965bcdc46a22b4b7f02,0x03506e89928b685d30b4e7fd04ccdf2e1d7fa598226f8340feb39814f52c3671)),
            TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000003, 0x05e8a8c69651f5f47e0384d0f71dba57a660c02a57d4a318b6161031d2fb6639, 2)),
            TrieNode::Binary(BinaryNodeImpl::new(0x03937a6283972d485392da517301b2c3af89685ca0b96433050814172e097f76,0x0420b6820c32deb7e26098f351319c08697dc0180cda346779e502834d760ea4)),
            TrieNode::Edge(EdgeNodeImpl::new(0x0000332296ec292b2b4fdf4ce2ab51314bedb9be3456b18689279274c62004c1, 0x07e704b4c5faed6238a37b62b80a45d3488916aa0fa842a813f5a04b4661cbb8, 238))
        ];

        let res = verify_mpt_proof(root, key, proof);
        assert(res == Option::Some(Membership::Included(3574364092672205702516898695783452043330964574546146246602102132808683211704)), 'it works!');

        // No Inclusion
        let root = 0x04946c7636b878064ddaab68a34c440f7961b854934e9b7347d981f9f5f768f2;
        let key = 0x07f772b4a9855e925d6e25370386cae7feb0738bfaa760b55f354ff68a016bbb;
        let proof = array![
            TrieNode::Binary(BinaryNodeImpl::new(0x01f387675be9603ca64faee950fc41c70c4e8c534fb9ac6d235bf7ef732fdbb6,0x01bdac12073dd4161c28740f654bdd4162aad1e8c0588989f7d05977fc5ef41e)),
            TrieNode::Binary(BinaryNodeImpl::new(0x00f618893b917332d0f527205f94ab7bd8ab20da5b60be66f22b90d08100aba4,0x0001e996b4969876b0d24e9c9446f9aa9efef959462e2a9e800945942d4d0b63)),
            TrieNode::Binary(BinaryNodeImpl::new(0x07cf400d095212c51ce3320971de539643d8eabaffb4c2a4369e222f3606f03a,0x0321dba462885b2a1969701b2c9efebb1407a0c1108fe27da6f54db7808721d2)),
            TrieNode::Binary(BinaryNodeImpl::new(0x04a2a67938a30148e2122d6fcf1f4d2a8a3d0aae210561172ddd464883ce7b21,0x0260a7f8ec1bcc0daaf25fd848a99971aa7671a80bc70f7807383948d42c92a6)),
            TrieNode::Binary(BinaryNodeImpl::new(0x03491251a53e9c5bec49cc660d56ca385e73083d6718e29fb97e4807c9e30313,0x06ceaa10a3aaa88c2e8854580d6b07e830962b09347ce347b46cc72dfa6489d1)),
            TrieNode::Binary(BinaryNodeImpl::new(0x00f8bf8528760fe6b0222695ede29ffae141c56cdb213d4e1466e14ce4e8276d,0x00929c3f7cef440f97eede9e24d14b5e0a63a66cae78a5382012fd7bfe9f3504)),
            TrieNode::Binary(BinaryNodeImpl::new(0x062d74b8320ace1044838eb51482fda8665312c073bb7bcbaa9dfe269ea02e66,0x05ff3df449b7b21b776cace2b50fa55661c6392123c648570999845352eb9fd4)),
            TrieNode::Binary(BinaryNodeImpl::new(0x006272dff0610fc4002f1c938bd7881bb64d3f6faec0a6bf5c70ca7e3621f07c,0x01076040d2fcf662ef1ea965be7861879c7bba9ef042cb7c3805071d23d02524)),
            TrieNode::Binary(BinaryNodeImpl::new(0x03f9b70ed8775c6a82de52454a0afb07d411c723b5a2b8c8112a8cb0231df612,0x0105b7061c0ff9ec6b3b4a214f25860efe841215ce3ae8a48b9ecdbe78afb2de)),
            TrieNode::Binary(BinaryNodeImpl::new(0x0700308f65ad92429798078e94796ed3d527ab5b5abb23f8fe7d01668a1be5f1,0x0703d8c398ce5658cb691fda4f4128c8f2d0dfcc54e3c52527056d367fb8b17c)),
            TrieNode::Edge(EdgeNodeImpl::new(0x0000000000000000000000000000000000000000000000000000000000000000, 0x06c757f42b0fedf1235795910d9fc31a473a229a5001f536240ff7e9e7af6669, 3))
        ];

        let res = verify_mpt_proof(root, key, proof);
        assert(res == Option::Some(Membership::NotIncluded), 'it works!');

        // Invalid Proof
        let root = 0x04946c7636b878064ddaab68a34c440f7961b854934e9b7347d981f9f5f768f2;
        let key = 0x04bd06380495cdd0b2ad24159d4dadf3b814a4dcb3c7f1c0320e7ab557701a08;
        let proof = array![
            TrieNode::Binary(BinaryNodeImpl::new(0x01f387675be9603ca64faee950fc41c70c4e8c534fb9ac6d235bf7ef732fdbb6,0x01bdac12073dd4161c28740f654bdd4162aad1e8c0588989f7d05977fc5ef41e)),
            TrieNode::Binary(BinaryNodeImpl::new(0x00f618893b917332d0f527205f94ab7bd8ab20da5b60be66f22b90d08100aba4,0x0001e996b4969876b0d24e9c9446f9aa9efef959462e2a9e800945942d4d0b63)),
            TrieNode::Binary(BinaryNodeImpl::new(0x024e0353771899f4914cfae5584fbab7818193956d3d4e13fa161213b8718ae5,0x05329ab53b7a3b59928c6cc431a907a9298e763af1305fb0e6c06a1b1bca5aa5)),
            TrieNode::Binary(BinaryNodeImpl::new(0x04d32327e2a5c33de295638d7431a01ad683806ec5d5c4baa1e49f6fb124b67e,0x013fdd24d9e6098fdf0a764125c55ffc9cc157da1a5dbfc7363f9d16392cfc79)),
            TrieNode::Binary(BinaryNodeImpl::new(0x01ac26f9d089d784d8fc0314c4a3af515d0926f6060de406ed13be62a6a57658,0x027315f79eba55ebdb20733066944c3f85a87df49abbafb617eda4dbbdfe0a2b)),
            TrieNode::Binary(BinaryNodeImpl::new(0x013346bf653869e6dfb9c50c629067c8ebbf12455cfa1ed1c4a801bce268d284,0x0119ec6adb71dfaffbcadae6cc1a0155d730624b94d79ade925fda284cebfed2)),
            TrieNode::Binary(BinaryNodeImpl::new(0x01cfdeaa5ff1dfe8f44df46cfbc7e188cc6b7911eb422ba9fc143734e76d4bc3,0x07f2a1f0e546b809358a83369136a66f0f5940e4f179b835298e77f948a7f2d2)),
            TrieNode::Binary(BinaryNodeImpl::new(0x0775482a0487931679742d488960e72876b5c4a962aca71db2bf71ae3129392a,0x02e8754f617d378d094b3d9f52d7620a13ec80294068d4060deb5393dd9c8583)),
            TrieNode::Binary(BinaryNodeImpl::new(0x02940f0ba8b001aafece7edac2e6c1e7d9f5f26ef05e1b2d73401c409d90b03b,0x057568b881608861226a10f327ae42970354afbb2280902befe0c952f93a9098)),
            TrieNode::Edge(EdgeNodeImpl::new(0x00, 0x023f85f2501db36234e6c5a8ce5b75c8c97a4871a1f6b71e7c5687826f59ce6a, 1)),
            TrieNode::Binary(BinaryNodeImpl::new(0x07252c9f96510e5728a7fadb3a280c1d60289f4c971afbad67c5859d745170c0,0x034a56bd7abceb632b85e6bd4b26acccc9c8125dfb93fcbdbc221881cfac75cf)),
            TrieNode::Binary(BinaryNodeImpl::new(0x01ddca4a2b72a4c62b2781bf9a33888d7a3b3e298c147797440e2fa69e6cbf33,0x025af085398a5e229bf586fd900bcd192d1d0d987217baafc74f95b698fe6151)),
            TrieNode::Edge(EdgeNodeImpl::new(0x06380495cdd0b1ad24159d4dadf3b814a4dcb3c7f1c0320e7ab557701a08, 0x04f1a9ead7881ae6068230eb986ed7deeaa78f3d0547630e8bfb4650fd96e46f, 239))
        ];

        let res = verify_mpt_proof(root, key, proof);
        assert(res == Option::None, 'it works!');

        // Invalid key
        let root = 0x04946c7636b878064ddaab68a34c440f7961b854934e9b7347d981f9f5f768f2;
        let key = 0x03bd06380495cdd0b2ad24159d4dadf3b814a4dcb3c7f1c0320e7ab557701a08;
        let proof = array![
            TrieNode::Binary(BinaryNodeImpl::new(0x01f387675be9603ca64faee950fc41c70c4e8c534fb9ac6d235bf7ef732fdbb6,0x01bdac12073dd4161c28740f654bdd4162aad1e8c0588989f7d05977fc5ef41e)),
            TrieNode::Binary(BinaryNodeImpl::new(0x00f618893b917332d0f527205f94ab7bd8ab20da5b60be66f22b90d08100aba4,0x0001e996b4969876b0d24e9c9446f9aa9efef959462e2a9e800945942d4d0b63)),
            TrieNode::Binary(BinaryNodeImpl::new(0x024e0353771899f4914cfae5584fbab7818193956d3d4e13fa161213b8718ae5,0x05329ab53b7a3b59928c6cc431a907a9298e763af1305fb0e6c06a1b1bca5aa5)),
            TrieNode::Binary(BinaryNodeImpl::new(0x04d32327e2a5c33de295638d7431a01ad683806ec5d5c4baa1e49f6fb124b67e,0x013fdd24d9e6098fdf0a764125c55ffc9cc157da1a5dbfc7363f9d16392cfc79)),
            TrieNode::Binary(BinaryNodeImpl::new(0x01ac26f9d089d784d8fc0314c4a3af515d0926f6060de406ed13be62a6a57658,0x027315f79eba55ebdb20733066944c3f85a87df49abbafb617eda4dbbdfe0a2b)),
            TrieNode::Binary(BinaryNodeImpl::new(0x013346bf653869e6dfb9c50c629067c8ebbf12455cfa1ed1c4a801bce268d284,0x0119ec6adb71dfaffbcadae6cc1a0155d730624b94d79ade925fda284cebfed2)),
            TrieNode::Binary(BinaryNodeImpl::new(0x01cfdeaa5ff1dfe8f44df46cfbc7e188cc6b7911eb422ba9fc143734e76d4bc3,0x07f2a1f0e546b809358a83369136a66f0f5940e4f179b835298e77f948a7f2d2)),
            TrieNode::Binary(BinaryNodeImpl::new(0x0775482a0487931679742d488960e72876b5c4a962aca71db2bf71ae3129392a,0x02e8754f617d378d094b3d9f52d7620a13ec80294068d4060deb5393dd9c8583)),
            TrieNode::Binary(BinaryNodeImpl::new(0x02940f0ba8b001aafece7edac2e6c1e7d9f5f26ef05e1b2d73401c409d90b03b,0x057568b881608861226a10f327ae42970354afbb2280902befe0c952f93a9098)),
            TrieNode::Edge(EdgeNodeImpl::new(0x00, 0x023f85f2501db36234e6c5a8ce5b75c8c97a4871a1f6b71e7c5687826f59ce6a, 1)),
            TrieNode::Binary(BinaryNodeImpl::new(0x07252c9f96510e5728a7fadb3a280c1d60289f4c971afbad67c5859d745170c0,0x034a56bd7abceb632b85e6bd4b26acccc9c8125dfb93fcbdbc221881cfac75cf)),
            TrieNode::Binary(BinaryNodeImpl::new(0x01ddca4a2b72a4c62b2781bf9a33888d7a3b3e298c147797440e2fa69e6cbf33,0x025af085398a5e229bf586fd900bcd192d1d0d987217baafc74f95b698fe6151)),
            TrieNode::Edge(EdgeNodeImpl::new(0x06380495cdd0b2ad24159d4dadf3b814a4dcb3c7f1c0320e7ab557701a08, 0x04f1a9ead7881ae6068230eb986ed7deeaa78f3d0547630e8bfb4650fd96e46f, 239))
        ];

        let res = verify_mpt_proof(root, key, proof);
        assert(res == Option::None, 'it works!');

    }
}

