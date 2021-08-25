/// Helper macro to avoid retyping the base domain-like name of our system when creating further
/// string constants from it. When given no parameters, this returns the base domain-like name of
/// the system. When given a string literal parameter it adds `/parameter` to the end.
macro_rules! testsys {
    () => {
        "testsys.bottlerocket.aws"
    };
    ($s:literal) => {
        concat!(testsys!(), "/", $s)
    };
}

// System identifiers
pub const API_VERSION: &str = testsys!("v1");
pub const NAMESPACE: &str = "testsys-bottlerocket-aws";
pub const TESTSYS: &str = testsys!();

// Component names
pub const CONTROLLER: &str = "testsys-controller";
pub const TEST_AGENT: &str = "testsys-test-agent";
pub const TEST_AGENT_SERVICE_ACCOUNT: &str = "testsys-test-agent-account";

// Label keys
pub const LABEL_TEST_NAME: &str = testsys!("test-name");
pub const LABEL_TEST_UID: &str = testsys!("test-uid");

// Environment variables
pub const ENV_TEST_NAME: &str = "TESTSYS_TEST_NAME";

#[test]
fn testsys_constants_macro_test() {
    assert_eq!("testsys.bottlerocket.aws", testsys!());
    assert_eq!("testsys.bottlerocket.aws/v1", API_VERSION);
    assert_eq!("testsys.bottlerocket.aws/foo", testsys!("foo"));
}
