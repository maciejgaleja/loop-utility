pub fn split_args(args: &[String]) -> (Vec<String>, Vec<String>) {
    let mut set0 = Vec::new();
    let mut set1 = Vec::new();

    let mut separator_occured: bool = false;
    for arg in args {
        if arg == "--" {
            separator_occured = true;
        } else {
            if separator_occured {
                set1.push(arg.clone());
            } else {
                set0.push(arg.clone());
            }
        }
    }
    (set0, set1)
}
