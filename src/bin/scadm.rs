use clap::{arg, ArgAction, Command};
use scadm_core::commands;
use tokio::main;

macro_rules! print_help {
  () => {
    std::process::Command::new(std::env::current_exe().unwrap())
      .arg("-h")
      .spawn()
      .unwrap()
  };
}

// This #[tokio::main]
// See https://www.reddit.com/r/rust/comments/vj2ghz/comment/idhgdxa/?utm_source=share&utm_medium=web2x&context=3
#[main]
async fn main() {
  let args = Command::new("scadm")
    .version("0.1.0-alpha1")
    .args(&[])
    .help_template(indoc::indoc! {"
		{before-help}
		{bin} ({version}) - {usage}

		OPTIONEN:
		{options}
		{positionals}

		BEFEHLE:
		{subcommands}
		"})
    .subcommand(
      Command::new("start").args(&[
        arg!(-f --force "Starten erzwingen, auch wenn eine PID-Datei \
vorliegt. Falls ein prozess mit der in der PID-datei gebenen PID existiert, \
wird dieser mit signal 9 beendet")
        .action(ArgAction::SetTrue)
        .id("force"),
      ]),
    )
    .subcommand(
      Command::new("stop").args(&[
        arg!(-f --force "Prozess nach maximal 60 sek. mit signal 9 beenden")
          .action(ArgAction::SetTrue)
          .id("force"),
      ]),
    )
    .get_matches();

  println!("Args: {:#?}", args);

  match args.subcommand() {
    Some(("start", start_args)) => {
      commands::start::start(*start_args.get_one::<bool>("force").unwrap())
        .await
    }
    Some(("stop", stop_args)) => {
      commands::stop::stop(*stop_args.get_one::<bool>("force").unwrap()).await
    }
    Some(_) => {
      eprintln!("Unbekannter Befehl");
      print_help!();
    }
    None => {
      eprintln!("Bitte wählen sie einen Unterbefehl aus");
      print_help!();
    }
  }
}
