fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args {
    pub img_1: String,
    pub img_2: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        Args {
            img_1: get_nth_arg(1),
            img_2: get_nth_arg(2),
            output: get_nth_arg(3),
        }
    }
}
