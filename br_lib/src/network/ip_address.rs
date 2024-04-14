use crate::std_lib::std_result::StdResult;

#[derive(Debug, PartialEq)]
pub enum IpAddrParseError {
    InvalidFormat,
    InvalidValue,
}

#[derive(Debug, PartialEq)]
pub enum IpAddrState {
    Start,
    Digit,
    Dot,
    Colon,
    End,
}

pub fn parse_address(input: &str) -> StdResult<[u8; 16]> {
    if let Ok(ipv4_addr) = parse_ipv4_addr(input) {
        let mut ipv6_addr = [0; 16];
        ipv6_addr[10] = 0xFF;
        ipv6_addr[11] = 0xFF;
        ipv6_addr[12..].copy_from_slice(&ipv4_addr);

        return Ok(ipv6_addr);
    } else if let Ok(ipv6_addr) = parse_ipv6_addr(input) {
        return Ok(ipv6_addr);
    }

    Err("invalid_ip_address")?
}

fn parse_ipv4_addr(input: &str) -> StdResult<[u8; 4]> {
    let mut state = IpAddrState::Start;
    let mut digit_count = 0;
    let mut octet_count = 0;
    let mut octet_value: u16 = 0;
    let mut address: [u8; 4] = [0; 4];

    for c in input.chars() {
        match state {
            IpAddrState::Start => match c {
                '0'..='9' => {
                    state = IpAddrState::Digit;
                    digit_count = 1;
                    octet_value = c.to_digit(10).unwrap() as u16;
                }
                _ => Err("invalid_ip_address")?,
            },
            IpAddrState::Digit => match c {
                '0'..='9' => {
                    digit_count += 1;
                    if digit_count > 3 {
                        Err("invalid_ip_address")?;
                    }
                    octet_value = octet_value * 10 + c.to_digit(10).unwrap() as u16;
                    if octet_value > 255 {
                        Err("invalid_ip_address")?;
                    }
                }
                '.' => {
                    state = IpAddrState::Dot;
                    digit_count = 0;
                    address[octet_count] = octet_value as u8;
                    octet_count += 1;
                    if octet_count > 3 {
                        Err("invalid_ip_address")?;
                    }
                }
                _ => Err("invalid_ip_address")?,
            },
            IpAddrState::Dot => match c {
                '0'..='9' => {
                    state = IpAddrState::Digit;
                    digit_count = 1;
                    octet_value = c.to_digit(10).unwrap() as u16;
                }
                _ => Err("invalid_ip_address")?,
            },
            IpAddrState::End => Err("invalid_ip_address")?,
            _ => Err("invalid_ip_address")?,
        }
    }

    if octet_count != 3 {
        Err("invalid_ip_address")?;
    }

    address[octet_count] = octet_value as u8;
    Ok(address)
}

fn set_zero(address: &mut [u8; 16], from: usize) {
    address[from] = 0;
    address[from + 1] = 0;
}

fn parse_ipv6_addr(input: &str) -> StdResult<[u8; 16]> {
    let mut state = IpAddrState::Start;
    let mut digit_count = 0;
    let mut colon_count = 0;
    let mut address = [0; 16];
    let mut current_digit = String::new();

    for c in input.chars() {
        match state {
            IpAddrState::Start => match c {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    state = IpAddrState::Digit;
                    current_digit.push(c);
                    digit_count = 1;
                }

                ':' => {
                    state = IpAddrState::Colon;
                    colon_count = 1;

                    set_zero(&mut address, 0);
                }
                _ => Err("invalid_ip_address")?,
            },
            IpAddrState::Digit => match c {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    digit_count += 1;
                    if digit_count > 4 {
                        Err("invalid_ip_address")?;
                    }
                    current_digit.push(c);
                }

                ':' => {
                    state = IpAddrState::Colon;
                    if colon_count + 1 > 7 {
                        Err("invalid_ip_address")?;
                    }
                    if let Ok(value) = u16::from_str_radix(&current_digit, 16) {
                        let from = colon_count * 2;
                        set_from_slice(&mut address, from, &value.to_be_bytes());
                    } else {
                        Err("invalid_ip_address")?;
                    }

                    colon_count += 1;
                    current_digit.clear();
                }
                _ => Err("invalid_ip_address")?,
            },
            IpAddrState::Colon => match c {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    state = IpAddrState::Digit;
                    current_digit.push(c);
                    digit_count = 1;
                }

                ':' => {
                    if colon_count + 1 > 7 {
                        Err("invalid_ip_address")?;
                    }
                    let from = colon_count * 2;
                    set_zero(&mut address, from);

                    colon_count += 1;
                }

                _ => Err("invalid_ip_address")?,
            },
            IpAddrState::End => Err("invalid_ip_address")?,
            _ => Err("invalid_ip_address")?,
        }
    }

    if colon_count != 7 {
        Err("invalid_ip_address")?;
    }

    if let Ok(value) = u16::from_str_radix(&current_digit, 16) {
        let from = colon_count * 2;
        set_from_slice(&mut address, from, &value.to_be_bytes());
    } else {
        let from = colon_count * 2;
        set_zero(&mut address, from);
    }

    Ok(address)
}

fn set_from_slice(address: &mut [u8; 16], from: usize, value: &[u8; 2]) {
    let to = from + 2;
    address[from..to].copy_from_slice(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_ipv4_address_0() {
        valid_address("0.0.0.0", [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, 0, 0, 0, 0]);
    }

    #[test]
    fn valid_ipv4_address_255() {
        valid_address(
            "255.255.255.255",
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        );
    }

    #[test]
    fn valid_ipv4_address_1() {
        valid_address(
            "192.168.0.1",
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, 192, 168, 0, 1],
        );
    }

    #[test]
    fn valid_ipv4_address_2() {
        valid_address(
            "255.255.255.0",
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, 255, 255, 255, 0],
        );
    }

    #[test]
    fn valid_ipv4_address_3() {
        valid_address("10.0.0.1", [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, 10, 0, 0, 1]);
    }

    #[test]
    fn valid_ipv4_address_4() {
        valid_address("127.0.0.1", [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, 127, 0, 0, 1]);
    }

    #[test]
    fn invalid_ipv4_address_1() {
        invalid_address("abc.def.ghi.jkl");
    }

    #[test]
    fn invalid_ipv4_address_2() {
        invalid_address("256.256.256.256");
    }

    #[test]
    fn invalid_ipv4_address_3() {
        invalid_address("192.168.1");
    }

    #[test]
    fn invalid_ipv4_address_4() {
        invalid_address("192.168.1.1.1");
    }

    #[test]
    fn invalid_ipv4_address_5() {
        invalid_address("192..1.1");
    }

    #[test]
    fn invalid_ipv4_address_6() {
        invalid_address("192.168.1.0.0");
    }

    #[test]
    fn empty_string() {
        invalid_address("");
    }

    #[test]
    fn valid_ipv6_address_0() {
        valid_address("0:0:0:0:0:0:0:0", [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn valid_ipv6_address_1() {
        valid_address(":::::::", [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn valid_ipv6_address_2() {
        valid_address(
            "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
            [32, 1, 13, 184, 133, 163, 0, 0, 0, 0, 138, 46, 3, 112, 115, 52],
        );
    }

    #[test]
    fn invalid_ipv6_address_1() {
        invalid_address("2001:0db8:85a3::8a2e:0370:7334");
    }

    #[test]
    fn invalid_ipv6_address_2() {
        invalid_address("2001:0db8:85a3:::0370:7334");
    }

    #[test]
    fn values_beyond_ipv6() {
        invalid_address("2001:0db8:85a3:0000:0000:8a2e:0370:7334:");
    }

    fn valid_address(input: &str, expected: [u8; 16]) {
        let value = parse_address(input);
        assert_eq!(value.expect("value"), expected);
    }

    fn invalid_address(input: &str) {
        let value = parse_address(input);
        assert_eq!("invalid_ip_address", value.expect_err("Err").to_string());
    }
}
