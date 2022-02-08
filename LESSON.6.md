# LESSON 6 (pallets/template/weights.rs)


## Weights 

[pallets src code](pallets/template/weights.rs)

```sh
./target/release/node-template benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_template --extrinsic do_something --steps 50 --repeat 20 --raw
```

## Chain Spec 

```sh
./target/release/node-template build-spec --chain local >local.json --raw >local-raw.json
```
