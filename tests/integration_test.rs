#[cfg(test)]
mod test {
    use std::process::Command;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            fm_plugin::post_build::bundle_plugin().unwrap();
        });
    }
    #[test]
    fn test_pluggo() {
        initialize();
        let output = Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg("fmp://$/plugin_test?script=test")
            .output()
            .expect("failed open fm");
        let status = output.status;
        assert_eq!(status.to_string(), "exit code: 0");
        let result = std::str::from_utf8(&output.stdout).unwrap();
        assert_eq!(result, "");
    }
}
