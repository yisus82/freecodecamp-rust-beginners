pub(crate) fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap_or_else(|| "".to_string())
}

#[derive(Debug)]
pub(crate) struct Args {
    pub(crate) image1: String,
    pub(crate) image2: String,
    pub(crate) output: String,
}

impl Args {
    pub(crate) fn new() -> Self {
        Self {
            image1: get_nth_arg(1),
            image2: get_nth_arg(2),
            output: get_nth_arg(3),
        }
    }
}
