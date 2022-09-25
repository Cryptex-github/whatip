#![doc = include_str!("../README.md")]

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The IP Address to lookup.
    #[clap(value_parser)]
    ip: String,
    /// Disable ANSI formatting.
    #[clap(long, action)]
    no_ansi: bool,
    /// Output as JSON.
    #[clap(long, action)]
    json: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { ip, mut no_ansi, json } = Args::parse();

    let mode = if json { "json" } else { "line" };

    let url = format!("http://ip-api.com/{}/{}?fields=4259839", mode, ip);

    let info = ureq::get(&url)
        .call()?
        .into_string()?;

    if json {
        println!("{}", info);
        
        std::process::exit(0);
    }

    let mut info = info.split('\n');
    
    if info.next().unwrap() == "fail" {
        println!("Failed to retrieve information about that IP address: {}", info.next().unwrap());
        std::process::exit(1);
    }

    #[cfg(windows)]
    {
        if !no_ansi {
            no_ansi = ansi_term::enable_ansi_support().is_err();
        }
    }

    macro_rules! ansi {
        ($t:literal) => {{
            if no_ansi {
                $t.to_string()
            } else {
                ansi_term::Style::new().bold().paint($t).to_string()
            }
        }};
    }

    let country = info.next().unwrap();
    let country_short = info.next().unwrap();
    let region_short = info.next().unwrap();
    let region = info.next().unwrap();
    let city = info.next().unwrap();
    let zip = info.next().unwrap();
    let lat = info.next().unwrap();
    let lon = info.next().unwrap();
    let timezone = info.next().unwrap();
    let isp = info.next().unwrap();
    let org = info.next().unwrap();
    let asn = info.next().unwrap();
    let asn_name = info.next().unwrap();
    let reverse_dns_name = info.next().unwrap();

    println!(r#"
{}{}
{}{} ({})
{}{} ({})
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
{}{}
        "#,
        ansi!("IP: "),
        ip,
        ansi!("Country: "),
        country,
        country_short,
        ansi!("Region: "),
        region,
        region_short,
        ansi!("City: "),
        city,
        ansi!("Zip Code: "),
        zip,
        ansi!("Latitude: "),
        lat,
        ansi!("Longitude: "),
        lon,
        ansi!("Timezone: "),
        timezone,
        ansi!("ISP: "),
        isp,
        ansi!("Organization: "),
        org,
        ansi!("ASN: "),
        asn,
        ansi!("ASN Name: "),
        asn_name,
        ansi!("Host Name: "),
        reverse_dns_name
    );

    Ok(())
}
