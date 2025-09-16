use std::process::Command;

/// Build a command that updates the system packages using `apt`.
pub fn build_system_update_command() -> Command {
    let mut cmd = Command::new("bash");
    cmd.arg("-c").arg("apt update && apt upgrade -y");
    cmd
}

/// Build a command that updates firmware using `fwupdmgr`.
pub fn build_firmware_update_command() -> Command {
    let mut cmd = Command::new("bash");
    cmd.arg("-c").arg("fwupdmgr refresh && fwupdmgr get-updates && fwupdmgr update");
    cmd
}

/// Build a command that updates drivers using `ubuntu-drivers`.
pub fn build_driver_update_command() -> Command {
    let mut cmd = Command::new("ubuntu-drivers");
    cmd.arg("autoinstall");
    cmd
}

/// Build a command that removes a driver package via `apt remove`.
pub fn build_driver_uninstall_command(package: &str) -> Command {
    let mut cmd = Command::new("apt");
    cmd.args(["remove", "-y", package]);
    cmd
}

/// Build a command that shuts the system down immediately.
pub fn build_shutdown_command() -> Command {
    let mut cmd = Command::new("shutdown");
    cmd.arg("now");
    cmd
}

/// Build a command that restarts the system.
pub fn build_restart_command() -> Command {
    Command::new("reboot")
}

/// Build a command that logs off a user.
pub fn build_logoff_command(user: &str) -> Command {
    let mut cmd = Command::new("pkill");
    cmd.args(["-KILL", "-u", user]);
    cmd
}

/// Build a command that switches to another user using `loginctl`.
pub fn build_switch_user_command(user: &str) -> Command {
    let mut cmd = Command::new("loginctl");
    cmd.args(["switch-user", user]);
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shutdown_command_is_correct() {
        let cmd = build_shutdown_command();
        assert_eq!(cmd.get_program(), "shutdown");
        let args: Vec<_> = cmd.get_args().collect();
        assert_eq!(args, ["now"]);
    }

    #[test]
    fn driver_uninstall_command_contains_package() {
        let cmd = build_driver_uninstall_command("nvidia-driver");
        assert_eq!(cmd.get_program(), "apt");
        let args: Vec<_> = cmd.get_args().collect();
        assert_eq!(args, ["remove", "-y", "nvidia-driver"]);
    }
}
