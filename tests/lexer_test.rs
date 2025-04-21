use std::process::Command;

#[cfg(test)]
mod book_tests {
    use super::*;

    #[test]
    fn run_ch1_tests() {
        // Run the Python script
        let output = Command::new("sh")
            .arg("-c")
            .arg("./tests/tests_from_book/test_compiler target/debug/nimc --chapter 1 --stage lex")
            .output()
            .expect("Failed to execute Python script");

        // Print exactly what Python printed (preserve newlines/formatting)
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));

        assert!(
            output.status.success(),
            "Python tests failed with exit code: {}",
            output.status
        );

        //TODO: add parse checks
        // Optionally, check the output of the Python tests
        println!("Chap 1 tests completed: {:?}", output);
    }
}
