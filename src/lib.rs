use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct Argin {
    pub flags: Vec<String>,
    pub values: HashMap<String, String>,
    pub pos_arg: Vec<String>,
}

impl Argin {
    pub fn new() -> Argin {
        return Argin {
            flags: Vec::new(),
            values: HashMap::new(),
            pos_arg: Vec::new(),
        };
    }

    pub fn add_flag(&mut self, flag: &str) {
        self.flags.push(flag.to_string());
    }

    pub fn add_value(&mut self, name: &str) {
        self.values.insert(name.to_string(), String::new());
    }

    pub fn add_positional_arg(&mut self) {
        self.pos_arg.push(String::new());
    }

    pub fn parse(&self) -> Argin {
        let mut args = Argin::new();
        let buffer = env::args().collect::<Vec<String>>();

        let mut pos_arg_idx = 0;
        let mut index = 0;
        while index < buffer.len() {
            let arg = &buffer[index];
            if self.flags.contains(&arg) {
                args.flags.push(arg.clone());
            } else if self.values.get(arg).is_some() {
                if index + 1 < buffer.len() {
                    args.values.insert(arg.clone(), buffer[index + 1].clone());
                    index += 1;
                }
            } else {
                if pos_arg_idx != self.pos_arg.len() {
                    args.pos_arg.push(arg.clone());
                    pos_arg_idx += 1;
                }
            }
            index += 1;
        }
        return args;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arg() {
        let mut argin = Argin::new();
        argin.add_flag("-rgb");
        argin.add_value("-name");
        argin.add_positional_arg();
        println!("argin: {:?}", argin.parse());
        assert_eq!(4, 4);
    }
}
