use acs_api_rs::acs_type::AcsType;
use cliclack::{input, intro, log, outro, select};
use acs_api_rs::connection::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = AcsConnection::new(AcsType::GenieAcs, "".to_string());

    intro("ACS API Client")?;

    loop {
        let selected = select("Choose action")
            .item("create", "Create connection", "")

            .item("get_devices", "Get devices", "")
            .item("get_props", "Get properties", "")
            .item("exit", "Exit", "")
            .interact()?;
        match selected {
            "create" => {
                let addr : String = input("Address")
                    .placeholder("localhost:7557")
                    .validate(|input: &String| {
                        if input.is_empty() {
                            Err("Please enter valid address")
                        } else {
                            Ok(())
                        }
                    })
                    .interact()?;
                conn = AcsConnection::new(AcsType::GenieAcs, addr.clone());
            }
            "get_devices" => {
                let result = conn.list_devices();
                if result.is_ok() {
                    let devices = result.unwrap();
                    log::info(format!("Devices: {:?}", devices))?;
                } else {
                    log::error(format!("Bad response: {}", result.unwrap_err()))?;
                }
            }
            "get_props" => {
                let id : String = input("ID")
                    .placeholder("some id")
                    .validate(|input: &String| {
                        if input.is_empty() {
                            Err("Please enter valid address")
                        } else {
                            Ok(())
                        }
                    })
                    .interact()?;
                let mut v = Vec::new();
                v.push("Device".to_string());
                let result = conn.get_parameter_values(id, v);
                if result.is_ok() {
                    let res = result.unwrap();
                    log::info(format!("Result: {:?}", res))?;
                } else {
                    log::error(format!("Bad response: {}", result.unwrap_err()))?;
                }
            }
            "exit" => {
                break;
            }
            &_ => todo!(),
        }
    }

    outro("Done!")?;

    Ok(())
}
