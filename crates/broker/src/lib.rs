//! Fail-closed seed. Process separation alone is NOT a sandbox.
#[derive(Debug, PartialEq, Eq)]
pub struct BrokerDisabled;
pub fn authorize_execution(_action: &str) -> Result<(), BrokerDisabled> {
    Err(BrokerDisabled)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deny_every_sample_action() {
        for action in [
            "",
            "shell.exec",
            "fs.read",
            "network.fetch",
            "deploy",
            "approve",
            "unknown",
        ] {
            assert_eq!(authorize_execution(action), Err(BrokerDisabled));
        }
    }
}
