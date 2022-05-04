use hex;

use std::io::Write;
use std::io::{BufRead, BufReader};
use std::{env, error, fmt, fs};

use ntrust_native::CRYPTO_ALGNAME;
use ntrust_native::{crypto_kem_dec, crypto_kem_enc, crypto_kem_keypair};
use ntrust_native::{AesState, RNGState};
use ntrust_native::{CRYPTO_BYTES, CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES};

#[derive(Debug)]
struct InvalidFileFormat(String, usize);

impl error::Error for InvalidFileFormat {}

impl fmt::Display for InvalidFileFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file has invalid format at line {}: {}", self.1, self.0)
    }
}

type R = Result<(), Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
struct Testcase {
    count: usize,
    seed: [u8; 48],
    pk: [u8; CRYPTO_PUBLICKEYBYTES],
    sk: [u8; CRYPTO_SECRETKEYBYTES],
    ct: [u8; CRYPTO_CIPHERTEXTBYTES],
    ss: [u8; CRYPTO_BYTES],
}

fn is_zero(x: &[u8]) -> bool {
    if x.is_empty() {
        true
    } else {
        x[0] == 0 && is_zero(&x[1..])
    }
}

impl Testcase {
    fn new() -> Testcase {
        Testcase {
            count: 0,
            seed: [0u8; 48],
            pk: [0u8; CRYPTO_PUBLICKEYBYTES],
            sk: [0u8; CRYPTO_SECRETKEYBYTES],
            ct: [0u8; CRYPTO_CIPHERTEXTBYTES],
            ss: [0u8; CRYPTO_BYTES],
        }
    }

    fn with_seed(count: usize, seed: &[u8; 48]) -> Testcase {
        Testcase {
            count: count,
            seed: *seed,
            pk: [0u8; CRYPTO_PUBLICKEYBYTES],
            sk: [0u8; CRYPTO_SECRETKEYBYTES],
            ct: [0u8; CRYPTO_CIPHERTEXTBYTES],
            ss: [0u8; CRYPTO_BYTES],
        }
    }

    fn write_to_file(&self, fd: &mut fs::File) -> R {
        let repr_bytes = |bytes: &[u8]| -> String {
            if is_zero(&bytes) {
                "".to_string()
            } else {
                hex::encode_upper(bytes)
            }
        };

        writeln!(fd, "count = {}", self.count)?;
        writeln!(fd, "seed = {}", hex::encode_upper(self.seed))?;
        writeln!(fd, "pk = {}", repr_bytes(&self.pk).as_str())?;
        writeln!(fd, "sk = {}", repr_bytes(&self.sk).as_str())?;
        writeln!(fd, "ct = {}", repr_bytes(&self.ct).as_str())?;
        writeln!(fd, "ss = {}\n", repr_bytes(&self.ss).as_str())?;

        Ok(())
    }

    /// Parse one line of a `.rsp` file. Returns true if data in the
    /// expected format has been successfully stored in `self`.
    /// Returns false, if the line is empty (acts as record separator).
    fn read_line(&mut self, line: &str, lineno: usize) -> Result<bool, Box<dyn error::Error>> {
        let err = |msg: &str| -> Result<bool, Box<dyn error::Error>> {
            Err(Box::new(InvalidFileFormat(msg.to_string(), lineno)))
        };

        if line.starts_with('#') {
            return Ok(true);
        }
        if line.trim() == "" {
            return Ok(false);
        }

        let mut fields = line.split("=");
        let name = match fields.nth(0) {
            Some(n) => n.trim(),
            None => return err("could not split key with '=' assignment operator"),
        };
        let value = match fields.nth(0) {
            Some(v) => v.trim(),
            None => return err("could not split value with '=' assignment operator"),
        };

        match name {
            "count" => self.count = value.parse::<usize>()?,
            "seed" => hex::decode_to_slice(value, &mut self.seed as &mut [u8])?,
            "pk" => hex::decode_to_slice(value, &mut self.pk as &mut [u8])?,
            "sk" => hex::decode_to_slice(value, &mut self.sk as &mut [u8])?,
            "ct" => hex::decode_to_slice(value, &mut self.ct as &mut [u8])?,
            "ss" => hex::decode_to_slice(value, &mut self.ss as &mut [u8])?,
            _ => return err(&format!("assignment to unknown key '{}'", name)),
        };

        Ok(true)
    }

    fn read_from_file(&mut self, reader: &mut BufReader<fs::File>) -> R {
        for (lineno, line) in reader.lines().enumerate() {
            if !self.read_line(&line?, lineno)? {
                return Ok(());
            }
        }

        Ok(())
    }
}

impl Eq for Testcase {}

impl fmt::Display for Testcase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE it requires a new struct with multiple implementations
        //   to abstract Testcase.write_to_file(…) for stdout AND files.
        //   As a result, I decided to duplicate the code.
        let repr_bytes = |bytes: &[u8]| -> String {
            if is_zero(&bytes) {
                "".to_string()
            } else {
                hex::encode_upper(bytes)
            }
        };

        writeln!(f, "count = {}", self.count)?;
        writeln!(f, "seed = {}", hex::encode_upper(self.seed))?;
        writeln!(f, "pk = {}", repr_bytes(&self.pk).as_str())?;
        writeln!(f, "sk = {}", repr_bytes(&self.sk).as_str())?;
        writeln!(f, "ct = {}", repr_bytes(&self.ct).as_str())?;
        writeln!(f, "ss = {}\n", repr_bytes(&self.ss).as_str())
    }
}

fn create_request_file(filepath: &str, rng: &mut impl RNGState) -> R {
    let mut fd = fs::File::create(filepath)?;

    // initialize RNG
    let mut entropy_input = [0u8; 48];
    for i in 0..48 {
        entropy_input[i] = i as u8;
    }
    rng.randombytes_init(entropy_input);

    // create 100 testcase seeds
    for t in 0..100 {
        let mut tc = Testcase::new();
        tc.count = t;
        rng.randombytes(&mut tc.seed)?;

        tc.write_to_file(&mut fd)?;
    }

    Ok(())
}

fn create_response_file(filepath: &str, rng: &mut impl RNGState) -> R {
    let mut fd = fs::File::create(filepath)?;
    writeln!(&mut fd, "# {}\n", CRYPTO_ALGNAME)?;

    // initialize RNG
    let mut entropy_input = [0u8; 48];
    for i in 0..48 {
        entropy_input[i] = i as u8;
    }
    rng.randombytes_init(entropy_input);

    // create 100 testcase seeds
    for t in 0..100 {
        let mut tc = Testcase::new();
        tc.count = t;
        rng.randombytes(&mut tc.seed)?;

        let mut tc_rng = AesState::new();
        tc_rng.randombytes_init(tc.seed);

        crypto_kem_keypair(&mut tc.pk, &mut tc.sk, &mut tc_rng)?;
        crypto_kem_enc(&mut tc.ct, &mut tc.ss, &tc.pk, &mut tc_rng)?;
        let mut ss = [0u8; CRYPTO_BYTES];
        crypto_kem_dec(&mut ss, &tc.ct, &tc.sk)?;

        assert_eq!(tc.ss, ss);
        tc.write_to_file(&mut fd)?;
    }

    Ok(())
}

fn verify(filepath: &str) -> R {
    let fd = fs::File::open(filepath)?;
    let mut reader = BufReader::new(fd);
    let mut rng = AesState::new();

    // first record in a response file is empty (e.g. “# ntruhps2048509\n”)
    // hence, skip it
    let mut expected = Testcase::new();
    expected.read_from_file(&mut reader)?;

    // create 100 testcase seeds
    for t in 0..100 {
        let mut expected = Testcase::new();
        expected.read_from_file(&mut reader)?;

        rng.randombytes_init(expected.seed);

        let mut actual = Testcase::with_seed(t, &expected.seed);
        crypto_kem_keypair(&mut actual.pk, &mut actual.sk, &mut rng)?;
        crypto_kem_enc(&mut actual.ct, &mut actual.ss, &actual.pk, &mut rng)?;
        crypto_kem_dec(&mut actual.ss, &actual.ct, &actual.sk)?;

        //assert_eq!(expected, actual);
        assert_eq!(
            expected.seed, actual.seed,
            "seeds of testcase {} don't match",
            expected.count
        );
        assert_eq!(
            expected.pk, actual.pk,
            "public keys of testcase {} don't match",
            expected.count
        );
        assert_eq!(
            expected.sk, actual.sk,
            "secret keys of testcase {} don't match",
            expected.count
        );
        assert_eq!(
            expected.ct, actual.ct,
            "ciphertexts of testcase {} don't match",
            expected.count
        );
        assert_eq!(
            expected.ss, actual.ss,
            "shared secrets of testcase {} don't match",
            expected.count
        );
    }

    Ok(())
}

fn main() -> R {
    let mut args = env::args();
    match args.len() {
        1 => {
            eprintln!("usage: ./PQCgenKAT_kem <request:filepath> <response:filepath>");
            eprintln!("  generate a request and response file\n");
            eprintln!("usage: ./PQCgenKAT_kem <response:filepath>");
            eprintln!("  verify the given response file\n");
            panic!("wrong number of arguments");
        }

        2 => {
            args.next().unwrap();
            let rsp_file = args.next().unwrap();
            verify(&rsp_file)?;

            println!("Verification successful.");
        }

        3 => {
            args.next().unwrap();
            let req_file = args.next().unwrap();
            let rsp_file = args.next().unwrap();

            create_request_file(&req_file, &mut AesState::new())?;
            create_response_file(&rsp_file, &mut AesState::new())?;

            println!("request and response file created.");
        }

        _ => panic!("unexpected number of arguments!"),
    }

    Ok(())
}
