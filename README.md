hal-simplicity -- a Simplicity-enabled extension of hal -- python interface
===========================================================================

This is a fork of Blockstreams [hal-simplicity](https://github.com/BlockstreamResearch/hal-simplicity)
which in turn is a fork of Steven Roose's [hal-elements](https://github.com/stevenroose/hal-elements/)
which in turn is an extension of his Bitcoin tool [hal](https://github.com/stevenroose/hal).

This project is a research study to create a convinient python interface to hal-simplicity

# Installation

First build the complete rust project
```bash
cargo build
```
and you should find the binary `hal-simplicity` (`hal-simplicity.exe` on windows) in the target folder `debug` or `release`.

Now create a python environment ...
```bash
python -m venv .venv
```
... and activate it
```bash
. .venv/bin/activate # linux/mac 
. .venv/Scripts/activate # windows (git-bash)
```

Let poetry add missing libraries with
```bash
poetry update
```

Now build the debug wheel (to be found in folder debug/wheel)
```bash
maturin build --manifest-path hal_simplicity_py/Cargo.toml
```
or the release build  (to be found in folder relöease/wheel)
```bash
maturin build --manifest-path hal_simplicity_py/Cargo.toml --release
```

A quick install (i.e. a forced reinstall) of the newly compiled wheel could look like this, assuming at least one whl file exists.
```bash
poetry run pip install --force-reinstall "$(ls -t target/wheels/hal_simplicity_py*.whl | head -n 1)"
```

If you want to do all at once you may test this `poe` command (make sure the `venv` is activated)
```bash
poetry run poe build-wheel
```
# test
try running 
```bash
 poetry run python -m python.main simplicity info e4fba0509b4df120e1d320451f14172c46476646daf8d0d6da80e84c986cc5e073f80ed4dcf0210284187248126ac8e671544245742660022ae160c5e14b09ec0c2a17584bf5c548c85961c02b6efc010c03109ad2420c3f00140b16ab91cd75dcbc1e84ea7a320719cbfc6dc95e5194f9eca996d55a7b2d768c511e2a310e1806240a1241b70a35627302ef7da851f75a1f471748121a2b6978930a58ccaee2309401bd1b6e9fcbb0018601881a80e12071190284906e2a37159c2a162cdba0e67e0aad66c82658ec0c7f2a5a2cc38c3f61a892acd0da3a133ff9ead668873dc60c0310b5b0730445fea038d226980c2e6f7e4be9e895848d1fd97f2100db43004cb4eaddefc50601885c078170e6f13a1848e019ef88de2e7a3c1561d1828b3be0f290def9feebf54da94249472c0c0312050920fc8238dc861438a059b630e6ef256702d23cf92f32979f4fcd9ff3909cf7b32538aafb0e3a23ec40079b1d130c03103785c207e4c8201c5a072580e4e0
```

```json
{
  "jets": "core",
  "commit_base64": "5PugUJtN8SDh0yBFHxQXLEZHZkba+NDW2oDoTJhsxeBz+A7U3PAhAoQYckgSasjmcVRCRXQmYAIq4WDF4UsJ7AwqF1hL9cVIyFlhwCtu/AEMAxCa0kIMPwAUCxarkc113LwehOp6MgcZy/xtyV5RlPnsqZbVWnstdoxRHioxDhgGJAoSQbcKNWJzAu99qFH3Wh9HF0gSGitpeJMKWMyu4jCUAb0bbp/LsAGGAYgagOEgcRkChJBuKjcVnCoWLNug5n4KrWbIJljsDH8qWizDjD9hqJKs0No6Ez/56tZohz3GDAMQtbBzBEX+oDjSJpgMLm9+S+nolYSNH9l/IQDbQwBMtOrd78UGAYhcB4Fw5vE6GEjgGe+I3i56PBVh0YKLO+DykN75/uv1TalCSUcsDAMSBQkg/II43IYUOKBZtjDm7yVnAtI8+S8yl59PzZ/zkJz3syU4qvsOOiPsQAebHRMMAxA3hcIH5MggHFoHJYDk4A==",
  "commit_decode": "(witness  & iden); (((unit; const 0xbe241c3a6408a3e282e588c8ecc8db5f1a1adb501d09930d98bc0e7f01da9b9e ) & iden); (((IOH; ((((false & unit); assertl drop jet_sha_256_ctx_8_init ) & iden); ((((false & (OH & IH)); assertl drop jet_sha_256_ctx_8_add_32 ) & iden); ((false & OH); assertl drop jet_sha_256_ctx_8_finalize )))) & iden); ((((false & ((false & (OH & IOH)); assertl drop jet_eq_256 )); assertl drop jet_verify ) & ((((false & unit); assertl drop jet_sig_all_hash ) & iden); ((false & ((IIIOH & OH) & witness )); assertl drop jet_bip_0340_verify ))); IH)))",
  "type_arrow": "1 → 1",
  "cmr": "7fd424f70498ef2fb6dd05ffbb7368dc796e6c47f24404e0b1ff138cfce89a7a",
  "liquid_address_unconf": "ex1pyuvwaqedernfdc7c6qf7r67en3szas6s0sdegzq3jxduhj4mhles29dz23",
  "liquid_testnet_address_unconf": "tex1pyuvwaqedernfdc7c6qf7r67en3szas6s0sdegzq3jxduhj4mhlestul9m7",
  "is_redeem": false
}
```
