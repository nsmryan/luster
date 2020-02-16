use std::process::Command;
use structopt::StructOpt;


#[derive(StructOpt)]
#[structopt(about="Set screen brightness in Windows.
With no arguments luster will print the current AC (wall power) brightness and the DC (battery) brightness")]
struct Opts {
    #[structopt(help="Brightness setting")]
    percent: Option<u8>,
}


fn find_guid(search_string: &str, query: &str, skip: usize) -> String {
    let mut guid = None;
    for line in search_string.lines() {
        if line.contains(query) {
            guid = line.trim().split(" ").skip(skip).next();
            break;
        }
    }
    let guid = guid.expect("Could not find GUID");

    return guid.to_string();
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::from_args();

    let power_query_output =
        Command::new("powercfg")
                .args(&["/q"])
                .output()
                .expect("failed to run powercfg query");

    let power_query =
        std::str::from_utf8(&power_query_output.stdout)
                 .expect("Invalid output from powercfg");

    let scheme_guid = find_guid(power_query, "Power Scheme GUID", 3);
    let subgroup_guid = find_guid(power_query, "(Display)", 2);
    let setting_guid = find_guid(power_query, "(Display brightness)", 3);
     

    if let Some(percent) = opts.percent {
        println!("Brightness: {}", percent);

        let value = format!("{}", percent);

        Command::new("powercfg")
                .args(&["-SetDcValueIndex", &scheme_guid, &subgroup_guid, &setting_guid, &value])
                .output()
                .expect("Failure setting AC brigtness value");

        Command::new("powercfg")
                .args(&["-SetAcValueIndex", &scheme_guid, &subgroup_guid, &setting_guid, &value])
                .output()
                .expect("Failure setting AC brigtness value");
         

        Command::new("powercfg")
                .args(&["-S", &scheme_guid])
                .output()
                .expect("failed to apply updated power scheme");
    } else {
        let mut bright_lines = power_query.lines().skip_while( |line| {
            !line.contains("(Display brightness)")
        }).skip(5);

        let ac_line = bright_lines.next().expect("Did not found AC line");
        let dc_line = bright_lines.next().expect("Did not found DC line");

        let ac_str = ac_line.trim().split(" ").skip(5).next().expect("Did not find AC power setting");
        let dc_str = dc_line.trim().split(" ").skip(5).next().expect("Did not find DC power setting");

        let ac_str = &ac_str[2..];
        let dc_str = &dc_str[2..];

        println!("AC {}", usize::from_str_radix(&ac_str[2..], 16).expect("Could not parse AC string"));
        println!("DC {}", usize::from_str_radix(&dc_str[2..], 16).expect("Could not parse DC string"));
    }

    return Ok(());
}
