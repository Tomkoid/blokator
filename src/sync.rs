use crate::actions::Colors;
use crate::error::check_http_error;
use crate::tor::if_onion_link;
use crate::Args;
use crate::{get_data_dir, read::read_file_to_string, write::write_to_file};
use std::{path::Path, process::exit};

// Returns true if error
pub fn sync(repo: &str, args: &Args) -> bool {
    let colors = Colors::new();

    let mut client = reqwest::blocking::ClientBuilder::new();
    let tor_proxy = format!("socks5h://{}:{}", args.tor_bind_address, args.tor_port);

    if args.tor {
        client = client.proxy(reqwest::Proxy::all(tor_proxy).unwrap())
    } else if if_onion_link(repo.to_string()) {
        client = client.proxy(reqwest::Proxy::all(tor_proxy).unwrap());
    }

    let response = client.build().unwrap().get(repo).send();

    let local_hosts = format!("{}/hosts", get_data_dir());

    let response = match response {
        Ok(s) => match s.text() {
            Ok(resp) => resp,
            Err(e) => {
                println!(
                    "\n{}error:{} Failed to decode response: {}",
                    colors.bold_red, colors.reset, e
                );
                exit(1);
            }
        },
        Err(e) => {
            if e.is_timeout() {
                println!(
                    "\n{}error:{} Connection failed. (Check your internet connection): {}",
                    colors.bold_red, colors.reset, e,
                );
                exit(1)
            } else if e.is_connect() {
                println!(
                    "\n{}error:{} Couldn't connect to the server. Please check your internet connection: {}",
                    colors.bold_red,
                    colors.reset,
                    e
                );
                exit(1)
            } else {
                println!(
                    "\n{}error:{} Error occurred: {}",
                    colors.bold_red, colors.reset, e,
                );
                exit(1)
            }
        }
    };

    let error = check_http_error(&response);

    if Path::new(&local_hosts).exists() {
        write_to_file(
            &local_hosts,
            read_file_to_string(&local_hosts).unwrap() + &response + "\n\n",
        );
    } else {
        write_to_file(&local_hosts, response);
    }

    error
}
