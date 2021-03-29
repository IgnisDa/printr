fn run() {}

#[derive(Debug)]
pub struct Arguments(Vec<String>);

impl Arguments {
    pub fn new(args: Vec<String>) -> Self {
        Arguments(args)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new_being_created() {
        Arguments::new(vec!["hello".to_string(), "hi".to_string()]);
    }
}
