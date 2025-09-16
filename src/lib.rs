//! Media server tools library.
//! Provides system control commands used by the dashboard.

pub mod system_control;

#[cfg(test)]
mod tests {
    use super::system_control::*;

    #[test]
    fn build_restart_cmd() {
        let cmd = build_restart_command();
        assert_eq!(cmd.get_program(), "reboot");
    }
}
