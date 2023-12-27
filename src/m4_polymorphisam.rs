use std::str::FromStr;

/// This module is about demonstate polymorphisam; i.e same function use diffent methods
/// using traits and genrices
/// Problem : To get_ethers_data(address) data, we need to pass address, which could be &str or Address
/// In order to hanlde these two we are creating traits that implements both &str and Address
use ethers::types::Address;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(&self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum Address String"),
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_ethers_data<T: EthereumAddress>(address: T) -> Address {
    let converted_address = address.convert_address().unwrap();
    converted_address
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_poly() {
       let address = Address::from_str("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap();

        let new_addr = get_ethers_data(address);
        assert_eq!(new_addr, address);

        let new_addr = get_ethers_data("0x388C818CA8B9251b393131C08a736A67ccB19297");
        assert_eq!(new_addr, Address::from_str("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap());
    }
}
