const SMALL_PRIMES: [u64; 12] = [7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

/// Exported function to check if a number is prime
#[no_mangle]
pub extern "C" fn rust_is_prime(n: u64) -> bool {
    if vec![2, 3, 5].contains(&n) {
        return true;
    }
    if (n < 2) | (n % 2 == 0) | (n % 3 == 0) | (n % 5 == 0) {
        return false;
    }

    if n < 49 {
        return true;
    }
    for &p in &SMALL_PRIMES {
        if n % p == 0 {
            return false;
        }
    }

    if n < 2809 {
        return true;
    }

    if n < 65077 {
        // There are only five Euler pseudoprimes
        // with a least prime factor greater than 47
        let euler_pseudoprimes = [8321, 31621, 42799, 49141, 49981];
        let test_value = mod_exp(2, n / 2, n);
        return (test_value == 1 || test_value == n - 1) && !euler_pseudoprimes.contains(&n);
    }

    // Step 2: deterministic Miller-Rabin testing for numbers < 2^64.  See:
    //    https://miller-rabin.appspot.com/
    // for lists.  We have made sure the M-R routine will successfully handle
    // bases larger than n, so we can use the minimal set.
    // In September 2015 deterministic numbers were extended to over 2^81.
    //    https://arxiv.org/pdf/1509.00864.pdf
    //    https://oeis.org/A014233
    if n < 341531 {
        return miller_rabin(n, &[9345883071009581737]);
    }
    if n < 885594169 {
        return miller_rabin(n, &[725270293939359937, 3569819667048198375]);
    }
    if n < 350269456337 {
        return miller_rabin(
            n,
            &[
                4230279247111683200,
                14694767155120705706,
                16641139526367750375,
            ],
        );
    }
    if n < 55245642489451 {
        return miller_rabin(
            n,
            &[
                2,
                141889084524735,
                1199124725622454117,
                11096072698276303650,
            ],
        );
    }
    if n < 7999252175582851 {
        return miller_rabin(
            n,
            &[
                2,
                4130806001517,
                149795463772692060,
                186635894390467037,
                3967304179347715805,
            ],
        );
    }
    if n < 585226005592931977 {
        return miller_rabin(
            n,
            &[
                2,
                123635709730000,
                9233062284813009,
                43835965440333360,
                761179012939631437,
                1263739024124850375,
            ],
        );
    }
    if n < u64::MAX {
        return miller_rabin(n, &[2, 325, 9375, 28178, 450775, 9780504, 1795265022]);
    }

    // For numbers larger than 3317044064679887385961981, additional tests would be needed
    // Here we assume it's not prime for demonstration purposes
    false
}

/// Helper function to perform modular exponentiation
fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result: u128 = 1;
    let modulus_u128 = modulus as u128;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base as u128 % modulus as u128;
        }
        exp >>= 1;
        base = (base as u128 * base as u128 % modulus_u128) as u64;
    }
    result as u64
}

fn miller_rabin(n: u64, bases: &[u64]) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let d = n - 1;
    let mut r = 0;
    let mut d = d;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    for &a in bases {
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut continue_flag = false;
        for _ in 0..r - 1 {
            x = mod_exp(x, 2, n);
            if x == n - 1 {
                continue_flag = true;
                break;
            }
        }
        if !continue_flag {
            return false;
        }
    }
    true
}

// /// Custom implementation of the Sieve of Eratosthenes
// fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
//     let mut primes = vec![true; limit + 1];
//     primes[0] = false;
//     primes[1] = false;

//     for i in 2..=((limit as f64).sqrt() as usize) {
//         if primes[i] {
//             for j in (i * i..=limit).step_by(i) {
//                 primes[j] = false;
//             }
//         }
//     }

//     primes
//         .iter()
//         .enumerate()
//         .filter_map(|(index, &is_prime)| if is_prime { Some(index) } else { None })
//         .collect()
// }
