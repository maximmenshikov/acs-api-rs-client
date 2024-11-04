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
            "exit" => {
                break;
            }
            &_ => todo!(),
        }
    }

    outro("Done!")?;

    Ok(())
}
