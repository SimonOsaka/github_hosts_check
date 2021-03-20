use cmd_lib::{run_cmd, run_fun, spawn_with_output, use_builtin_cmd, CmdResult};

fn main() -> CmdResult {
    println!("Hello, world!");
    // cmd_lib::set_debug(true);
    let ip_vec: Vec<&str> = vec![
        "13.229.188.59",
        "52.74.223.119",
        "13.250.177.223",
        "203.208.39.99",
        "15.164.81.167",
        "140.82.121.3",
        "140.82.114.4",
        "140.82.121.4",
        "13.114.40.48",
        "52.69.186.44",
        "140.82.113.4",
        "140.82.112.4",
        "52.192.72.89",
        "192.30.255.113",
    ];

    let mut ip_success = vec![];
    for i in 0..3 {
        for ip in &ip_vec {
            if run_cmd! {
            echo "check ip:$ip start.";
            // ping $ip -c 3;
            curl -H "Host:github.com" --connect-timeout 3 r"https://"$ip -k -I | grep "HTTP/2 200";
            echo "check ip:$ip end.";
        }.is_err() {
                println!("ip:{} could not resolve host", ip);
                continue;
            };

            if i == 2 {
                ip_success.push(ip);
            }
        }
    }
    println!("{} ip hosts:{:?}", ip_success.len(), ip_success);
    let len = ip_success.len();
    for n in 0..len {
        println!("{} github.com", ip_success[n]);
    }
    Ok(())
}
