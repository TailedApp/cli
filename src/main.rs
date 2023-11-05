use signalrs_client::SignalRClient;
use linemux::MuxedLines;
use std::process;
use nanoid::nanoid;
use qrcode::QrCode;

mod colors;
mod rules;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Tailed (v{})", env!("CARGO_PKG_VERSION"));
    println!();

    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let id: String = nanoid!(22);
    let url = format!("https://{}/{}", args.host, id);

    let rules: Vec<rules::Rule>;

    if args.custom_rule_set.is_some() {
        rules = rules::parse_rules(&args.custom_rule_set.unwrap());
    }
    else if args.rule_set.is_some() {
        rules = rules::get_standard_rules(&args.rule_set.unwrap()).unwrap();
    }
    else {
        rules = Vec::new();
    }

    let code = QrCode::new(url.clone()).unwrap();
    let string = code.render()
        .light_color(' ')
        .dark_color('\u{2588}')
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string);

    println!();
    println!("{url}");
    println!("Tailing: {}", &args.file.to_string_lossy().to_owned());
    println!();
    println!("Press Ctrl+C to exit..");

    ctrlc::set_handler(move || {
        println!();
        println!("Stopping Tailed..");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let client = SignalRClient::builder(args.host)
        .use_port(443)
        .use_hub("api/tail")
        .build()
        .await?;

    let mut lines = MuxedLines::new()?;
    lines.add_file(args.file.clone()).await?;

    while let Ok(Some(line)) = lines.next_line().await {
        let mut content = format!("{}\n", &line.line());

        content = rules::apply_rules(&rules, content);

        client
            .method("SendData")
            .arg(id.clone())?
            .arg(content)?
            .send()
            .await?;
        
    }

    return Ok(());
}

const HELP: &str = 
"USAGE:
  tailed FILE [OPTIONS]

FLAGS:
  -h, --help               Prints help information

ARGS:
  FILE                     The path to the file to tail. Required.

OPTIONS:
  --rule-set STRING        The optional colorization rule set to use.
                           [included: serilog].
  --custom-rule-set PATH   Optional path to a JSON file specifying
                           colorization rules.
  --host DOMAIN            Optional override of the Tailed server hostname.
                           [default: tailed.live]
                           
For further information, visit https://docs.tailed.live .

";

#[derive(Debug)]
struct AppArgs {
    file: std::path::PathBuf,
    rule_set: Option<String>,
    custom_rule_set: Option<std::path::PathBuf>,
    host: String
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let default_host = String::from("tailed.live");

    let args = AppArgs {
        rule_set: pargs.opt_value_from_str("--rule-set")?,
        custom_rule_set: pargs.opt_value_from_str("--custom-rule-set")?,
        host: pargs.value_from_str("--host").unwrap_or(default_host.clone()),
        file: pargs.free_from_str()?
    }; 

    if args.file.to_string_lossy().to_owned().starts_with("--") {
        println!("Either no filename was provided for tailing, or it was provided in the wrong position in the arguments.");
        std::process::exit(1);
    }
    Ok(args)
}