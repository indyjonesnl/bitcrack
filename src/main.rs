extern crate bitcoin;
extern crate rayon;
extern crate serde;
extern crate serde_json;

mod address;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use address::BitcoinAddress;
use bitcoin::secp256k1::Secp256k1;
use minreq::Response;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::process::ExitCode;

#[derive(Serialize, Deserialize, Debug)]
struct ReservationRequest {
    worker_name: String,
    amount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Found {
    reservation_id: u32,
    number: String,
    private_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Reservation {
    id: u32,
    target: String,
    start: String,
    end: String,
}

fn main() -> ExitCode {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        let reservation_request: ReservationRequest = ReservationRequest {
            amount: 20_000_000,
            worker_name: "macbook".to_string(),
        };
        let reservation_response: Response = minreq::post("http://localhost:8080/next")
            .with_json(&reservation_request)
            .expect("Failed to serialize reservation request.")
            .send()
            .expect("Failed to send reservation request to server.");
        println!("{}", reservation_response.as_str().expect("Failed to get response as string."));

        if reservation_response.status_code == 500 {
            print!("Got status 500 from server after requesting new reservation.");
            return ExitCode::FAILURE;
        }

        let reservation: Reservation = reservation_response
            .json()
            .expect("Failed to unpack JSON resonse into Reservation");
        let secp: Secp256k1<bitcoin::secp256k1::All> = Secp256k1::new();

        let rayon_pool: rayon::ThreadPool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .expect("Failed to create thread pool");

        let start: u128 = u128::from_str_radix(&reservation.start, 16).unwrap();
        let end: u128 = u128::from_str_radix(&reservation.end, 16).unwrap() + 1;

        println!(
            "Searching for address {} between {} and {}.",
            reservation.target, start, end
        );

        let bitcoin_address: Option<BitcoinAddress> = rayon_pool.install(|| {
            (start..end)
                .into_par_iter()
                .map(|i: u128| BitcoinAddress::new(&secp, &format!("{:0>64X}", i), true))
                .find_first(|address: &BitcoinAddress| {
                    address
                        .address
                        .as_ref()
                        .expect("Failed to get base58 address from bitcoin keypair")
                        .to_string()
                        .eq(&reservation.target)
                })
        });

        if bitcoin_address.is_some() {
            let address: BitcoinAddress =
                bitcoin_address.expect("Failed to get bitcoin keypair from result");
            println!(
                "Found address: {}",
                address
                    .address
                    .expect("Failed to get base58 address from bitcoin keypair")
                    .to_string()
            );
            println!("Private key: {}", address.private_key.to_wif());
            let found: Found = Found {
                number: address.hex,
                reservation_id: reservation.id,
                private_key: address.private_key.to_wif(),
            };
            let found_response: Response = minreq::post("http://localhost:8080/found")
                .with_json(&found)
                .expect("Failed to serialize found request.")
                .send()
                .expect("Failed to send reservation request to server.");
            println!("Got status {} from server.", found_response.status_code);
        } else {
            println!("Did not find the address within this keyspace.");
            let not_found_response = minreq::post(format!(
                "http://localhost:8080/not-found/{}",
                reservation.id,
            ))
            .send()
            .expect("Failed to post not found request to server.");
            println!(
                "Done sending not found request to server. Server responded with status {}.",
                not_found_response.status_code
            );
        }
    }

    return ExitCode::SUCCESS;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", "1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH")] // 1
    #[case("3", "1CUNEBjYrCn2y1SdiUMohaKUi4wpP326Lb")] // 2
    #[case("7", "19ZewH8Kk1PDbSNdJ97FP4EiCjTRaZMZQA")] // 3
    #[case("8", "1EhqbyUMvvs7BfL8goY6qcPbD6YKfPqb7e")] // 4
    #[case("15", "1E6NuFjCi27W5zoXg8TRdcSRq84zJeBW3k")] // 5
    #[case("31", "1PitScNLyp2HCygzadCh7FveTnfmpPbfp8")] // 6
    #[case("4C", "1McVt1vMtCC7yn5b9wgX1833yCcLXzueeC")] // 7
    #[case("E0", "1M92tSqNmQLYw33fuBvjmeadirh1ysMBxK")] // 8
    #[case("1D3", "1CQFwcjw1dwhtkVWBttNLDtqL7ivBonGPV")] // 9
    #[case("202", "1LeBZP5QCwwgXRtmVUvTVrraqPUokyLHqe")] // 10
    #[case("483", "1PgQVLmst3Z314JrQn5TNiys8Hc38TcXJu")] // 11
    #[case("A7B", "1DBaumZxUkM4qMQRt2LVWyFJq5kDtSZQot")] // 12
    #[case("1460", "1Pie8JkxBT6MGPz9Nvi3fsPkr2D8q3GBc1")] // 13
    #[case("2930", "1ErZWg5cFCe4Vw5BzgfzB74VNLaXEiEkhk")] // 14
    #[case("68F3", "1QCbW9HWnwQWiQqVo5exhAnmfqKRrCRsvW")] // 15
    #[case("C936", "1BDyrQ6WoF8VN3g9SAS1iKZcPzFfnDVieY")] // 16
    #[case("1764F", "1HduPEXZRdG26SUT5Yk83mLkPyjnZuJ7Bm")] // 17
    #[case("3080D", "1GnNTmTVLZiqQfLbAdp9DVdicEnB5GoERE")] // 18
    #[case("5749F", "1NWmZRpHH4XSPwsW6dsS3nrNWfL1yrJj4w")] // 19
    #[case("D2C55", "1HsMJxNiV7TLxmoF6uJNkydxPFDog4NQum")] // 20
    #[case("1BA534", "14oFNXucftsHiUMY8uctg6N487riuyXs4h")] // 21
    #[case("2DE40F", "1CfZWK1QTQE3eS9qn61dQjV89KDjZzfNcv")] // 22
    #[case("556E52", "1L2GM8eE7mJWLdo3HZS6su1832NX2txaac")] // 23
    #[case("DC2A04", "1rSnXMr63jdCuegJFuidJqWxUPV7AtUf7")] // 24
    #[case("1FA5EE5", "15JhYXn6Mx3oF4Y7PcTAv2wVVAuCFFQNiP")] // 25
    #[case("340326E", "1JVnST957hGztonaWK6FougdtjxzHzRMMg")] // 26
    #[case("6AC3875", "128z5d7nN7PkCuX5qoA4Ys6pmxUYnEy86k")] // 27
    #[case("D916CE8", "12jbtzBb54r97TCwW3G1gCFoumpckRAPdY")] // 28
    #[case("17E2551E", "19EEC52krRUK1RkUAEZmQdjTyHT7Gp1TYT")] // 29
    #[case("3D94CD64", "1LHtnpd8nU5VHEMkG2TMYYNUjjLc992bps")] // 30
    #[case("7D4FE747", "1LhE6sCTuGae42Axu1L1ZB7L96yi9irEBE")] // 31
    #[case("B862A62E", "1FRoHA9xewq7DjrZ1psWJVeTer8gHRqEvR")] // 32
    #[case("1A96CA8D8", "187swFMjz1G54ycVU56B7jZFHFTNVQFDiu")] // 33
    #[case("34A65911D", "1PWABE7oUahG2AFFQhhvViQovnCr4rEv7Q")] // 34
    #[case("4AED21170", "1PWCx5fovoEaoBowAvF5k91m2Xat9bMgwb")] // 35
    #[case("9DE820A7C", "1Be2UF9NLfyLFbtm3TCbmuocc9N1Kduci1")] // 36
    #[case("1757756A93", "14iXhn8bGajVWegZHJ18vJLHhntcpL4dex")] // 37
    #[case("22382FACD0", "1HBtApAFA9B2YZw3G2YKSMCtb3dVnjuNe2")] // 38
    #[case("4B5F8303E9", "122AJhKLEfkFBaGAd84pLp1kfE7xK3GdT8")] // 39
    #[case("E9AE4933D6", "1EeAxcprB2PpCnr34VfZdFrkUWuxyiNEFv")] // 40
    #[case("153869ACC5B", "1L5sU9qvJeuwQUdt4y1eiLmquFxKjtHr3E")] // 41
    #[case("2A221C58D8F", "1E32GPWgDyeyQac4aJxm9HVoLrrEYPnM4N")] // 42
    #[case("6BD3B27C591", "1PiFuqGpG8yGM5v6rNHWS3TjsG6awgEGA1")] // 43
    #[case("E02B35A358F", "1CkR2uS7LmFwc3T2jV8C1BhWb5mQaoxedF")] // 44
    #[case("122FCA143C05", "1NtiLNGegHWE3Mp9g2JPkgx6wUg4TW7bbk")] // 45
    #[case("2EC18388D544", "1F3JRMWudBaj48EhwcHDdpeuy2jwACNxjP")] // 46
    #[case("6CD610B53CBA", "1Pd8VvT49sHKsmqrQiP61RsVwmXCZ6ay7Z")] // 47
    #[case("ADE6D7CE3B9B", "1DFYhaB2J9q1LLZJWKTnscPWos9VBqDHzv")] // 48
    #[case("174176B015F4D", "12CiUhYVTTH33w3SPUBqcpMoqnApAV4WCF")] // 49
    #[case("22BD43C2E9354", "1MEzite4ReNuWaL5Ds17ePKt2dCxWEofwk")] // 50
    #[case("75070A1A009D4", "1NpnQyZ7x24ud82b7WiRNvPm6N8bqGQnaS")] // 51
    #[case("EFAE164CB9E3C", "15z9c9sVpu6fwNiK7dMAFgMYSK4GqsGZim")] // 52
    #[case("180788E47E326C", "15K1YKJMiJ4fpesTVUcByoz334rHmknxmT")] // 53
    #[case("236FB6D5AD1F43", "1KYUv7nSvXx4642TKeuC2SNdTk326uUpFy")] // 54
    #[case("6ABE1F9B67E114", "1LzhS3k3e9Ub8i2W1V8xQFdB8n2MYCHPCa")] // 55
    #[case("9D18B63AC4FFDF", "17aPYR1m6pVAacXg1PTDDU7XafvK1dxvhi")] // 56
    #[case("1EB25C90795D61C", "15c9mPGLku1HuW9LRtBf4jcHVpBUt8txKz")] // 57
    #[case("2C675B852189A21", "1Dn8NF8qDyyfHMktmuoQLGyjWmZXgvosXf")] // 58
    #[case("7496CBB87CAB44F", "1HAX2n9Uruu9YDt4cqRgYcvtGvZj1rbUyt")] // 59
    #[case("FC07A1825367BBE", "1Kn5h2qpgw9mWE5jKpk8PP4qvvJ1QVy8su")] // 60
    #[case("13C96A3742F64906", "1AVJKwzs9AskraJLGHAZPiaZcrpDr1U6AB")] // 61
    #[case("363D541EB611ABEE", "1Me6EfpwZK5kQziBwBfvLiHjaPGxCKLoJi")] // 62
    #[case("7CCE5EFDACCF6808", "1NpYjtLira16LfGbGwZJ5JbDPh3ai9bjf4")] // 63
    #[case("F7051F27B09112D4", "16jY7qLJnxb7CHZyqBP8qca9d51gAjyXQN")] // 64
    #[case("1A838B13505B26867", "18ZMbwUFLMHoZBbfpCjUJQTCMCbktshgpe")] // 65
    #[case("349B84B6431A6C4EF1", "19YZECXj3SxEZMoUeJ1yiPsw8xANe7M7QR")] // 70
    #[case("4C5CE114686A1336E07", "1J36UjUByGroXcCvmj13U6uwaVv9caEeAt")] // 75
    #[case("EA1A5C66DCC11B5AD180", "1BCf6rHUW6m3iH2ptsvnjgLruAiPQQepLe")] // 80
    #[case("11720C4F018D51B8CEBBA8", "1Kh22PvXERd2xpTQk3ur6pPEqFeckCJfAr")] // 85
    #[case("2CE00BB2136A445C71E85BF", "1L12FHH2FHjvTviyanuiFVfmzCy46RRATU")] // 90
    #[case("527A792B183C7F64A0E8B1F4", "19eVSDuizydXxhohGh8Ki9WY9KsHdSwoQC")] // 95
    #[case("AF55FC59C335C8EC67ED24826", "1KCgMv8fo2TPBpddVi9jqmMmcne9uSNJ5F")] // 100
    #[case("16F14FC2054CD87EE6396B33DF3", "1CMjscKB3QW7SDyQ4c3C3DEUHiHRhiZVib")] // 105
    #[case("35C0D7234DF7DEB0F20CF7062444", "12JzYkkN76xkwvcPT6AWKZtGX6w2LAgsJg")] // 110
    #[case("60F4D11574F5DEEE49961D9609AC6", "1NLbHuJebVwUZ1XqDjsAyfTRUPwDQbemfv")] // 115
                                                                                   // #[case("", "")]
    fn test_first_challenge(#[case] input: String, #[case] expected: String) {
        let secp: Secp256k1<bitcoin::secp256k1::All> = Secp256k1::new();
        assert_eq!(
            expected,
            BitcoinAddress::new(&secp, &format!("{:0>64}", input), true)
                .address
                .expect("Failed to get base58 address from bitcoin keypair")
                .to_string()
        );
    }

    #[test]
    fn test_int_to_hex() {
        assert_eq!(
            "00000000000000000000000000000000000000000000000000000000075BCD15",
            format!("{:0>64X}", 123456789)
        );
    }

    #[test]
    fn test_string_pad() {
        assert_eq!(
            "00000000000000000000000000000000000000000000000000000000000001D3",
            format!("{:0>64}", "1D3")
        );
    }
}
