fn main() -> std::process::ExitCode {
    pulse_app::console::main(std::env::args_os().skip(1))
}
