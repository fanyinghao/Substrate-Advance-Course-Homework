[package]
name = 'node-template-runtime'
version = '4.0.0-dev'
description = 'A fresh FRAME-based Substrate runtime, ready for hacking.'
authors = ['Substrate DevHub <https://github.com/substrate-developer-hub>']
homepage = 'https://substrate.io/'
edition = '2021'
license = 'Unlicense'
publish = false
repository = 'https://github.com/substrate-developer-hub/substrate-node-template/'

[package.metadata.docs.rs]
targets = ['x86_64-unknown-linux-gnu']


[dependencies.pallet-template]
default-features = false
path = '../pallets/template'
version = '4.0.0-dev'

[dependencies.pallet-poe]
default-features = false
path = '../pallets/poe'
version = '4.0.0-dev'


[dependencies.pallet-kitties]
default-features = false
path = '../pallets/kitties'
version = '4.0.0-dev'

[build-dependencies.substrate-wasm-builder]
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '5.0.0-dev'

[dependencies.codec]
default-features = false
features = ['derive']
package = 'parity-scale-codec'
version = '2.0.0'

[dependencies.frame-benchmarking]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
optional = true
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.frame-executive]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.frame-support]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.frame-system]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.frame-system-benchmarking]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
optional = true
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.frame-system-rpc-runtime-api]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.hex-literal]
optional = true
version = '0.3.1'

[dependencies.pallet-aura]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.pallet-balances]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.pallet-grandpa]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.pallet-randomness-collective-flip]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.pallet-sudo]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.pallet-timestamp]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.pallet-transaction-payment]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.pallet-transaction-payment-rpc-runtime-api]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.scale-info]
default-features = false
features = ['derive']
version = '1.0'



[dependencies.sp-api]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.sp-block-builder]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.sp-consensus-aura]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '0.10.0-dev'

[dependencies.sp-core]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.sp-inherents]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.sp-offchain]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.sp-runtime]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.sp-session]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.sp-std]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.sp-transaction-pool]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[dependencies.sp-version]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'devhub/latest'
version = '4.0.0-dev'

[features]
default = ['std']
runtime-benchmarks = [
    'frame-benchmarking',
    'frame-support/runtime-benchmarks',
    'frame-system-benchmarking',
    'frame-system/runtime-benchmarks',
    'hex-literal',
    'pallet-balances/runtime-benchmarks',
    'pallet-template/runtime-benchmarks',
    'pallet-timestamp/runtime-benchmarks',
    'sp-runtime/runtime-benchmarks',
]
std = [
    'codec/std',
    'scale-info/std',
    'frame-executive/std',
    'frame-support/std',
    'frame-system-rpc-runtime-api/std',
    'frame-system/std',
    'pallet-aura/std',
    'pallet-balances/std',
    'pallet-grandpa/std',
    'pallet-randomness-collective-flip/std',
    'pallet-sudo/std',
    'pallet-template/std',
    'pallet-poe/std',
    'pallet-kitties/std',
    'pallet-timestamp/std',
    'pallet-transaction-payment-rpc-runtime-api/std',
    'pallet-transaction-payment/std',
    'sp-api/std',
    'sp-block-builder/std',
    'sp-consensus-aura/std',
    'sp-core/std',
    'sp-inherents/std',
    'sp-offchain/std',
    'sp-runtime/std',
    'sp-session/std',
    'sp-std/std',
    'sp-transaction-pool/std',
    'sp-version/std',
]