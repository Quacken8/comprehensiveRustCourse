pub fn luhn(cc_number: &str) -> bool {
    // remove spaces
    let stringInp = cc_number.replace(" ", "");
    // check if it is a number
    if !stringInp.chars().all(char::is_numeric) {
        return false;
    }
    if stringInp.len() < 2 {
        return false;
    }

    // prepare array of ints from string
    let mut intVec: Vec<u8> = Vec::new();
    for c in stringInp.chars() {
        intVec.push(c.to_digit(10).unwrap() as u8);
    }

    // reverse vector
    intVec.reverse();
    
    // forcycle the array, read each digit, double it. If result two digit, sum digits
    for i in 0..intVec.len() {
        if i % 2 == 1 {
            let mut double = 2 * intVec[i];
            if double > 9 {
                double = double % 10 + 1;
            }
            intVec[i] = double;
        }
    }

    // sum array
    let mut totalSum = 0;
    for digit in intVec {
        totalSum += digit;
    }

    // if last digit is 0, valid
    if totalSum % 10 == 0 {
        return true;
    }
    return false;
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
