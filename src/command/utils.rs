use std::process::Command;

pub fn run_command(command: String) -> Result<(), String> {
    let (exec, args) = if let Some((exec, args)) = command.split_once(" ") {
        (exec, args)
    } else {
        (command.as_str(), "")
    };
    Command::new(exec)
        .args(args.split(" "))
        .output()
        .map_err(|e|{format!("{}", e)})
        .map(|_o|{})
}

