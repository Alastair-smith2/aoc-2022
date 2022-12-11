#[derive(Debug)]
pub enum OperationType {
    Old,
    Number(u64),
}

#[derive(Debug)]
pub enum Operation {
    Multiplication(OperationType),
    Addition(OperationType),
}

#[derive(Debug)]
pub struct Monkey {
    pub id: u32,
    pub items: Vec<u64>,
    pub operation: Operation,
    pub test_factor: u64,
    pub true_destination: u32,
    pub false_destination: u32,
    pub inspection_count: u64,
}

impl Monkey {
    pub fn get_worry_level_for_item(&self, item: u64) -> u64 {
        match &self.operation {
            Operation::Addition(op_type) => match op_type {
                OperationType::Old => item + item,
                OperationType::Number(val) => item + val,
            },
            Operation::Multiplication(op_type) => match op_type {
                OperationType::Old => item * item,
                OperationType::Number(val) => item * val,
            },
        }
    }

    // monkey id
    pub fn find_monkey_to_throw_to(&self, current_worry_level: u64) -> u32 {
        if (current_worry_level % self.test_factor) == 0 {
            self.true_destination
        } else {
            self.false_destination
        }
    }
}
