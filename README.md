# ntrust-native

A safe pure-rust implementation of the NTRU post-quantum scheme.

* NTRU is a lattice-based key encapsulation mechanism (KEM)
* The implementation is based on the NTRU reference implementation of NIST round 3
* The implementation does not utilize any concurrency techniques (SIMD/threading/…, except maybe auto-vectorization on your CPU)
* It depends on `tiny-keccak` as SHA-3 implementation and `aes` as AES block cipher (used as RNG) implementation
* It passes the 100 testcases of the C reference implementation
* It implements the NTRU-HPS (Hoffstein-Pipher-Silverman) scheme in three variants
* It implements the NTRU-HRSS (Hülsing-Rijneveld-Schanck-Schwabe) scheme in one variant
* The implementation takes between 20 milliseconds (`ntruhps2048509`) and 45 milliseconds (`ntruhps4096821`) to run on a modern computer
* The implementation is constant-time on software instruction level
* The random number generator is based on AES256 in counter mode
* NTRUst is the name of a WebAssembly implementation. Thus, this implementation is called `ntrust-native`.

## Who should use it?

Anyone, how wants to use the NTRU scheme to negotiate a key between two parties.

## How does one use it?

Add this to your `Cargo.toml`:
```toml
[dependencies]
ntrust-native = "1.0"
```

To use a specific NTRU variant, you need to import it with the corresponding feature flag:

```toml
[dependencies]
ntrust-native = { version = "1.0", features = ["ntruhrss701"] }
```


The `simple` example illustrates the API:
```rust
use ntrust_native::AesState;
use ntrust_native::{crypto_kem_dec, crypto_kem_enc, crypto_kem_keypair};
use ntrust_native::{CRYPTO_BYTES, CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES};

fn main() -> Result<(), Box<dyn error::Error>> {
  let mut rng = AesState::new();
  let mut pk = [0u8; CRYPTO_PUBLICKEYBYTES];
  let mut sk = [0u8; CRYPTO_SECRETKEYBYTES];
  let mut ct = [0u8; CRYPTO_CIPHERTEXTBYTES];
  let mut ss_alice = [0u8; CRYPTO_BYTES];
  let mut ss_bob = [0u8; CRYPTO_BYTES];

  crypto_kem_keypair(&mut pk, &mut sk, &mut rng)?;
  crypto_kem_enc(&mut ct, &mut ss_bob, &pk, &mut rng)?;
  crypto_kem_dec(&mut ss_alice, &ct, &sk)?;

  assert_eq!(ss_bob, ss_alice);
}
```

## How does one run it?

This library comes with two examples:

```bash
$ cargo run --example simple
```

The output annotates messages with Alice/Bob to illustrate which data is processed by which party.
The `katkem` example implements the classic request/response file structure which is part of the NIST PQC framework.

```bash
$ cargo run --example katkem PQCkemKAT_935.req PQCkemKAT_935.rsp
$ cargo run --example katkem PQCkemKAT_935.rsp
```

The different variants (`ntruhps2048509`, `ntruhps2048677`, `ntruhps4096821`, `ntruhrss701`) can be enabled through feature flags:

```bash
$ cargo run --example katkem --features ntruhrss701 -- PQCkemKAT_1450.req PQCkemKAT_1450.rsp
```

`ntruhps2048509` is the default variant. You cannot enable two variants simultaneously.

## How fast is it?

All data uses clock cycles as unit.
The rust implementation has the following clock-cycle count characteristics (the smaller the better):

<table>
  <thead>
    <tr><td></td><td>complete KEM</td><td>keypair</td><td>enc</td><td>dec</td></tr>
  </thead><tbody>
    <tr><td>ntruhps2048509</td><td>19,980,855</td><td>14,105,680</td><td>472,909</td><td>1,122,414</td></tr>
    <tr><td>ntruhps2048677</td><td>27,478,939</td><td>24,077,519</td><td>895,930</td><td>2,333,079</td></tr>
    <tr><td>ntruhps4096821</td><td>42,083,125</td><td>36,882,783</td><td>1,487,401</td><td>3,367,818</td></tr>
    <tr><td>ntruhrss701</td><td>32,433,993</td><td>28,506,984</td><td>828,162</td><td>2,919,074</td></tr>
  </tbody>
</table>

The C reference implementation has the following clock-cycle count characteristics (the smaller the better):

<table>
  <thead>
    <tr><td></td><td>complete KEM</td><td>keypair</td><td>enc</td><td>dec</td></tr>
  </thead><tbody>
    <tr><td>ntruhps2048509</td><td>15,912,900</td><td>12,139,200</td><td>811,651</td><td>1,812,650</td></tr>
    <tr><td>ntruhps2048677</td><td>28,911,500</td><td>22,233,600</td><td>1,520,640</td><td>3,668,860</td></tr>
    <tr><td>ntruhps4096821</td><td>41,914,800</td><td>32,138,300</td><td>2,089,350</td><td>5,908,570</td></tr>
    <tr><td>ntruhrss701</td><td>28,966,600</td><td>23,134,700</td><td>1,368,270</td><td>3,462,640</td></tr>
  </tbody>
</table>

The tests were done on a Lenovo Thinkpad x260 (Intel Core i5-6200U CPU @ 2.30GHz). In the case of rust, [criterion 0.3.5](https://crates.io/crates/criterion) has been used as given in `benches/` and in case of C, Google's [benchmark](https://github.com/google/benchmark/blob/v1.6.1/docs/perf_counters.md) with PFM support and disabled CPU frequency scaling. Our summary is that both implementations have comparable runtime. rust is a little bit slower (but uses many copy operations for type safety you could replace with `unsafe {}` code). You can run the benchmark suite yourself with the `bench` subcommand and optionally some variant feature flag:

```bash
$ cargo bench --features ntruhrss701
```

## Where is the source code?

On [github](https://github.com/prokls/ntrust-native).

## What is the content's license?

[MIT License](LICENSE.txt)

## Changelog

* **2022-05-03 version 1.0.0:** public release

## Where can I ask you to fix a bug?

On [github](https://github.com/prokls/ntrust-native/issues).
