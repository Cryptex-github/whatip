use clap::Parser;
use hyper::Client;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The IP Address to lookup.
    #[clap(value_parser)]
    ip: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = Args::parse().ip;

    let runtime = tokio::runtime::Builder::new_current_thread().enable_io().build()?;

    runtime.block_on(async {
        let client = Client::new();

        let url = format!("http://ip-api.com/line/{}?fields=4259839", ip).parse()?;

        let resp = client.get(url).await?;
        let buf = hyper::body::to_bytes(resp).await?.to_vec();
        let info = String::from_utf8_lossy(buf.as_slice()).into_owned();
        let mut info = info.split("\n");
        
        if info.next().unwrap() == "fail" {
            println!("Failed to retrieve information about that IP address: {}", info.next().unwrap());
            std::process::exit(1);
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
IP: {}
Country: {} ({})
Region: {} ({})
City: {}
Zip code: {}
Latitude: {}
Longitude: {}
Timezone: {}
ISP: {}
Organization: {}
ASN: {}
ASN Name: {}
Hostname: {}
            "#,
            ip,
            country,
            country_short,
            region,
            region_short,
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

        Ok(())
    })
}
