use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub type PinID = usize;
pub struct BLIFInfo {
    pub filename: String,
    pub net_list: HashMap<String, Vec<PinID>>,
    pub n_pin: usize,
}

impl BLIFInfo {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("cannot read file");
        let filename = Path::new(path)
            .file_name()
            .expect("cannot extract file name");
        let filename = filename
            .to_str()
            .expect("cannot cast os_str to str")
            .to_string();

        let pins: Vec<_> = content
            .lines()
            .filter(|line| line.starts_with(".names"))
            .collect();
        let mut net_list = HashMap::new();
        for (pin_id, pin) in pins.iter().enumerate() {
            let nets = pin.split_whitespace().skip(1);
            for net in nets {
                net_list
                    .entry(net.to_string())
                    .or_insert_with(Vec::new)
                    .push(pin_id);
            }
        }

        BLIFInfo {
            filename,
            net_list,
            n_pin: pins.len(),
        }
    }

    #[allow(dead_code)]
    pub fn digest(&self) {
        println!("n_pin = {}", self.n_pin);
        println!("n_net = {}", self.net_list.len());
        for (n_name, pin_ids) in self.net_list.iter().take(5) {
            println!("{}: {:?}", n_name, pin_ids);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digest() {
        // panic!("yes");
        let filename = "./apex1.blif";
        let blif = BLIFInfo::from_file(filename);
        blif.digest();
    }
}
