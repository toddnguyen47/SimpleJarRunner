pub trait ICommand {
  fn get_command() -> String;
}

pub struct WindowsCommand;

impl ICommand for WindowsCommand {
  fn get_command() -> String {
    return String::from("javaw.exe");
  }
}

pub struct LinuxMacCommand;

impl ICommand for LinuxMacCommand {
  fn get_command() -> String {
    return String::from("java");
  }
}

pub struct CommandFactory;

impl CommandFactory {
  pub fn get_java_command() -> String {
    if cfg!(target_os = "windows") {
      return WindowsCommand::get_command();
    } else {
      return LinuxMacCommand::get_command();
    }
  }
}
