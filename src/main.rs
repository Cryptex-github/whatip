#![doc = include_str!("../README.md")]

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The IP Address to lookup, defaults to your IP Address.
    #[clap(value_parser)]
    ip: Option<String>,
    /// Disable ANSI formatting.
    #[clap(long, action)]
    no_ansi: bool,
    /// Output as JSON.
    #[clap(long, action)]
    json: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args {
        ip,
        mut no_ansi,
        json,
    } = Args::parse();

    let ip = ip.unwrap_or_else(|| {
        ureq::get("http://ifconfig.me/ip")
            .call()
            .expect("Failed to retrive user's IP address")
            .into_string()
            .expect("Failed to retrive user's IP Address")
    });

    let mode = if json { "json" } else { "line" };

    let url = format!("http://ip-api.com/{}/{}?fields=4259839", mode, ip);

    let info = ureq::get(&url).call()?.into_string()?;

    if json {
        println!("{}", info);

        std::process::exit(0);
    }

    let mut info = info.split('\n');

    if info.next().unwrap() == "fail" {
        println!(
            "Failed to retrieve information about that IP address: {}",
            info.next().unwrap()
        );
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

    macro_rules! stream {
        ($iter:expr, $($item:expr),*) => {{
            $(
                $item = $iter.next().unwrap();
            )*
        }};
    }

    let (
        country,
        country_short,
        region_short,
        region,
        city,
        zip,
        lat,
        lon,
        timezone,
        isp,
        org,
        asn,
        asn_name,
        reverse_dns_name,
    );

    stream!(
        info,
        country,
        country_short,
        region_short,
        region,
        city,
        zip,
        lat,
        lon,
        timezone,
        isp,
        org,
        asn,
        asn_name,
        reverse_dns_name
    );

    println!(
        r#"
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
