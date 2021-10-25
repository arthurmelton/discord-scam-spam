use curl::easy::{Easy, List};
use std::io::Read;
use rand::seq::SliceRandom;
fn main() {
    let urls = vec!["https://discordn.gift/discord/login"];
    loop {
        let email = get_email();
        let password = get_password();
        let url = urls.choose(&mut rand::thread_rng()).unwrap();

        let mut dst = Vec::new();
        let mut easy = Easy::new();
        easy.url(url).unwrap();
        let _output = easy.custom_request("POST");
        let data_pass = ["{\"login\":\"", email.as_str(), "\",\"password\":\"", password.as_str(), "\"}"].join("");
        let mut data = data_pass.as_bytes();
        easy.post_field_size(data.len() as u64).unwrap();
        let mut list = List::new();
        list.append("content-type: application/json").unwrap();
        easy.http_headers(list).unwrap();

        let mut transfer = easy.transfer();
        transfer
                .read_function(|buf| {
                    dst.extend_from_slice(buf);
                    Ok(data.read(buf).unwrap_or(0))
                })
                .unwrap();
        let response = transfer.perform();
        drop(transfer);
        if response.is_ok() && easy.response_code().unwrap() == 200 {
            println!("Url: \"{}\", Email: \"{}\", Password: \"{}\"", url, email, password);
        }else {
            println!("Failed to send to \"{}\" with an error code of {}", url, easy.response_code().unwrap());
        }
    }
}

fn get_email() -> String {
    let chars = vec!['0','1','2','3','4','5','6','7','8','9', 'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z', 'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut email = "".to_string();
    for _ in 0..*vec![5,6,7,8,9,10,11,12,13,14,15].choose(&mut rand::thread_rng()).unwrap() {
        email.push(*chars.choose(&mut rand::thread_rng()).unwrap());
    }
    email.push('@');
    email.push_str(vec!["gmail.com", "yahoo.com", "hotmail.com"].choose(&mut rand::thread_rng()).unwrap());
    return email;
}

fn get_password() -> String {
    let chars = vec!['0','1','2','3','4','5','6','7','8','9', 'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z', 'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut password = "".to_string();
    for _ in 0..*vec![7,8,9,10,11,12,13,14,15].choose(&mut rand::thread_rng()).unwrap() {
        password.push(*chars.choose(&mut rand::thread_rng()).unwrap());
    }
    return password;
}