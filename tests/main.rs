#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 5);
    }
}

trait DockerCommand {
    fn execute(&self);
}

struct PSCommand {
    args: String,
}

impl DockerCommand for PSCommand {
    fn execute(&self) {
        println!("{}", self.args);
    }
}

#[test]
fn m() {
    let p: DockerCommand = PSCommand {
        args: "a".to_string()
    };
    p.execute();
}