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

macro_rules! cmd {
  ($n:tt) => {
    Command::new($n)
      .version("0.1.0-alpha1")
      .help_template(indoc::indoc! {"
      \n{before-help}{bin} ({version}) - {usage}

      {about}

      OPTIONEN:
      {options}
      {positionals}

      BEFEHLE:
      {subcommands}
"})
      .before_help("      \x1B[38;2;178;176;255m,\x1b[0m        / _\\ ___| |__  _   _  ___| | ___ _ __\n\
\x1B[38;2;137;136;255m   *******\x1b[0m     \\ \\ / __| '_ \\| | | |/ _ \\ |/ _ \\ '__|\n\
\x1B[38;2;93;91;254m//\x1B[38;2;137;136;255m***   ****\x1b[0m    \\ \\ (__| | | | |_| |  __/ |  __/ |\n\
\x1B[38;2;93;91;254m/\x1B[38;2;178;176;255m,,,,,\x1b[0m         \\__/\\___|_| |_|\\__,_|\\___|_|\\___|_|\n\
\x1B[38;2;178;176;255m   ,,,,,.\x1b[0m         ___\n\
\x1B[38;2;178;176;255m      ,,,,,\x1B[38;2;93;91;254m/\x1b[0m     / __\\___  _ __  _ __   ___  ___| |_\n\
\x1B[38;2;137;136;255m****   ***\x1B[38;2;93;91;254m//\x1b[0m    / /  / _ \\| '_ \\| '_ \\ / _ \\/ __| __|\n\
\x1b[38;2;137;136;255m   *******\x1b[0m     / /__| (_) | | | | | | |  __/ (__| |_\n\
\x1B[38;2;178;176;255m      ‘\x1b[0m        \\____/\\___/|_| |_|_| |_|\\___|\\___|\\__|")
      .mut_arg("help", |h| h.help("Hilfe anzeigen"))
      .disable_help_subcommand(true)
  };
}

// This #[tokio::main]
// See https://www.reddit.com/r/rust/comments/vj2ghz/comment/idhgdxa/?utm_source=share&utm_medium=web2x&context=3
#[main]
async fn main() {
  let args = cmd!("scadm")
    .version("0.1.0-alpha1")
    .args(&[])
    .mut_arg("version", |a| a.help("Versionsnummer anzeigen"))
    .subcommand(
      cmd!("setup")
        .about("Server und benötigte Konfigurationsdateien einrichten"),
    )
    .subcommand(
      cmd!("start").about("Daemon und Server starten").args(&[
        arg!(-f --force "Starten erzwingen, auch wenn eine PID-Datei \
vorliegt. Falls ein prozess mit der in der PID-datei gebenen PID existiert, \
wird dieser mit signal 9 beendet")
        .action(ArgAction::SetTrue)
        .id("force"),
      ]),
    )
    .subcommand(
      cmd!("stop").about("Daemon und Server stoppen").args(&[
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
