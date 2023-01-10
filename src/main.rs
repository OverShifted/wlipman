use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    io::{Read, Write}, process::{Command, Stdio}
};

use wl_clipboard_rs::{
    paste::{get_contents, get_mime_types, ClipboardType as PasteClipboardType, MimeType as PasteMimeType, Seat as PasteSeat},
    copy::{Options, Source, MimeType as CopyMimeType, MimeSource, ClipboardType as CopyClipboardType, Seat as CopySeat, clear}
};

// Stores a Vec<u8> for each mime-type
pub type ClipboardRecord = HashMap<String, Vec<u8>>;
pub type ClipboardHistory = Vec<ClipboardRecord>;

fn get_storage_path() -> PathBuf {
    dirs::cache_dir().unwrap().join(Path::new("wlipman.json"))
}

fn load_history() -> anyhow::Result<ClipboardHistory> {
    serde_json::from_str(&std::fs::read_to_string(get_storage_path())?).map_err(|err| err.into())
}

fn dump_history(history: &ClipboardHistory) -> anyhow::Result<()> {
    let dump = serde_json::to_string(history)?;
    std::fs::write(get_storage_path(), &dump)?;

    Ok(())
}

fn read_mime(mime: &str) -> anyhow::Result<Vec<u8>> {
    match get_contents(PasteClipboardType::Regular, PasteSeat::Unspecified, PasteMimeType::Specific(mime)) {
        Ok((mut pipe, _)) => {
            let mut contents = vec![];
            pipe.read_to_end(&mut contents)?;

            Ok(contents)
        },

        Err(err) => Err(err.into())
    }
}

fn create_record() -> anyhow::Result<ClipboardRecord> {
    let mimes = get_mime_types(PasteClipboardType::Regular, PasteSeat::Unspecified)?;
    let mut all_mimes : ClipboardRecord = HashMap::new();

    for mime in mimes.into_iter() {
        if &mime == "SAVE_TARGETS" {
            continue;
        }
        println!("Going for: {}", mime);
        match read_mime(&mime) {
            Ok(contents) => {
                // println!("Inserted: {}", t);
                all_mimes.insert(mime, contents);
            },
            Err(_) => ()
        }
    }

    Ok(all_mimes)
}

fn stringify_record(index: usize, record: &ClipboardRecord) -> String {

    let label = if record.contains_key("text/plain;charset=utf-8".into()) {
        String::from_utf8_lossy(record.get("text/plain;charset=utf-8").unwrap()).into_owned()
    } else if record.contains_key("text/plain".into()) {
        String::from_utf8_lossy(record.get("text/plain").unwrap()).into_owned()
    } else {
        "".into()
    };

    // for (mime, contents) in record.iter() {
    //     if mime != "text/ico" && wl_clipboard_rs::utils::is_text(mime) {
    //         println!("Stringed {}", mime);
    //         return String::from_utf8_lossy(contents).into_owned();
    //     }
    // }

    format!("{}: {}", index, label)
}

fn restore_record(record: ClipboardRecord) -> anyhow::Result<()> {
    clear(CopyClipboardType::Regular, CopySeat::All)?;

    let all_mimes = record.into_iter()
        .map(|(mime, contents)| MimeSource{
            source: Source::Bytes(contents.into()),
            mime_type: CopyMimeType::Specific(mime)
        }).collect();
    
    let mut opts = Options::new();
    opts.foreground(true);
    opts.copy_multi(all_mimes)?;

    Ok(())
}

fn store() -> anyhow::Result<()> {
    let storage_path = get_storage_path();

    // TODO: This assumes that `~/.cache` already exists. Is that fine?
    if !storage_path.exists() {
        std::fs::write(storage_path, "[]")?;
    }

    let mut history = load_history()?;
    history.push(create_record()?);
    dump_history(&history)?;
    
    Ok(())
}

fn pick() -> anyhow::Result<()> {
    let history = load_history()?;

    let rofi_input = history.iter().rev()
        .enumerate().map(|(i, record)| stringify_record(i, &record))
        .collect::<Vec<String>>().join(&"\n").as_bytes().to_owned();

    let mut rofi = Command::new("rofi")
        .args(["-dmenu", "-p", "Pick"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    rofi.stdin.take().unwrap().write_all(&rofi_input)?;

    let stdout = String::from_utf8_lossy(&rofi.wait_with_output()?.stdout).into_owned();
    let index_from_end: usize = stdout.split(":").nth(0).unwrap().parse()?;
    restore_record(history.into_iter().rev().nth(index_from_end).unwrap())?;
    println!("{:?}", index_from_end);

    Ok(())
}

fn help(exec_path: &str) {
    eprintln!("Usage:\n\t{} COMMAND", exec_path);
    eprintln!("\nWhere COMMAND is one of the following:");
    eprintln!("\t-h, --help, help\tShows this message.");
    eprintln!("\tstore\t\t\tAppends current clipboard state to history file.");
    eprintln!("\tpick\t\t\tOpens a rofi dialog to choose a clipboard entry.");
    eprintln!("\tstorage\t\t\tPrints history file's path.");
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        help(&args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "help" | "-h" | "--help" => help(&args[0]),
        "store" => store()?,
        "pick" => pick()?,
        "storage" => println!("{}", get_storage_path().to_str().unwrap()),
        cmd => eprintln!("Unknown command: '{}'.\nRun '{} -h' for help.", cmd, &args[0])
    };

    Ok(())
}
