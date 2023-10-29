use super::Encodable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RleEncoder {
    input: Vec<u8>,
}


impl RleEncoder {
    pub fn new(input: &Vec<u8>) -> Self {
        RleEncoder { input: input.clone() }
    }

    pub fn encode(&self) -> Vec<RleValue> {
        if self.input.is_empty() {
            return Vec::new();
        }

        let mut encoded_tuples = Vec::new();
        let mut current_value = self.input[0];
        let mut current_count = 1;

        for value in self.input[1..].iter() {
            if *value == current_value {
                current_count += 1;
                if current_count == 255 {
                    encoded_tuples.push(RleValue { value: current_value, count: 255 });
                    current_count = 0;
                }
            } else {
                encoded_tuples.push(RleValue { value: current_value, count: current_count });
                current_value = *value;
                current_count = 1;
            }
        }

        encoded_tuples.push(RleValue { value: current_value, count: current_count });
        encoded_tuples
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RleValue {
    value: u8,
    count: u8,
}

impl RleValue {
    pub fn to_string(&self) -> String {
        format!("{}{}", self.count, self.value)
    }
}
