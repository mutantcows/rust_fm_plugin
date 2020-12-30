#[cfg(test)]
mod test {
    use std::io::prelude::*;
    use std::net::TcpListener;
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
        let output = Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg("test_plugin.fmp12")
            .current_dir("./tests")
            .output()
            .expect("couldn't open test db");
        assert_eq!(output.status.to_string(), "exit code: 0");

        let output = Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg("fmp://$/test_plugin?script=test")
            .output()
            .expect("failed open fm");
        assert_eq!(output.status.to_string(), "exit code: 0");
        let result = std::str::from_utf8(&output.stdout).unwrap();
        assert_eq!(result, "");

        let result = filemaker_listener().unwrap();
        assert_eq!(result, "1");
    }

    fn filemaker_listener() -> Result<String, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:12346")?;

        let mut buffer: Vec<u8> = Vec::new();
        for result in listener.incoming() {
            let mut stream = result?;
            stream.read_to_end(&mut buffer).unwrap();
            break;
        }

        Ok(std::str::from_utf8(&buffer)?.to_owned())
    }
}
