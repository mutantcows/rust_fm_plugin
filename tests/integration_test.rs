#[cfg(test)]
mod test {
    use std::process::Command;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            let output = Command::new("cargo")
                .arg("post")
                .arg("build")
                .arg("--release")
                .current_dir("./tests/test_plugin")
                .output()
                .expect("failed to build test");

            let status = output.status;
            assert_eq!(status.to_string(), "exit code: 0");
        });
    }
    #[test]
    fn test_pluggo() {
        initialize();
        Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg("test_plugin.fmp12")
            .current_dir("./tests")
            .output()
            .expect("couldn't open test db");

        let output = Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg("fmp://$/test_plugin?script=test")
            .output()
            .expect("failed open fm");
        let status = output.status;
        assert_eq!(status.to_string(), "exit code: 0");
        let result = std::str::from_utf8(&output.stdout).unwrap();
        assert_eq!(result, "");
    }
}
