mod logger;

use std::path::Path;
use std::io::{self, Write, Read};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    store: bool,

    #[arg(short, long)]
    list: bool,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    decode: bool,
}

struct DBManager <'db>{
    db_path: &'db str,
    read_only: bool,
}

impl <'db> DBManager<'db > {
    pub fn open(&self) -> PickleDb {
        if Self::check_if_exists(&self.db_path) {
            if self.read_only {
                log! ("Opening read_only session");
                return PickleDb::load_read_only(&self.db_path, SerializationMethod::Cbor).unwrap();
            } else {
                log! ("Opening session");
                return PickleDb::load(&self.db_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Cbor).unwrap();
            };
        } else {
            log! ("Creating new session");
            return PickleDb::new(&self.db_path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Cbor);
        };
    }

    fn check_if_exists(db_path: &str) ->bool {
        return Path::new(db_path).exists()
    }
}

pub fn deduplicate(val: &Vec<u8>, db: &mut PickleDb) {
    let keys_to_remove: Vec<_> = db.iter().filter_map(|kv|{
        if kv.get_key() != "id" {
            if &kv.get_value::<Vec<u8>>().unwrap() == val {
                log! ("found match {}", kv.get_value::<String>().unwrap());
                Some(kv.get_key().to_string())
            } else {
                None
            }
        } else {
            None
        }
    })
    .collect();

    for key in keys_to_remove {
        log! ("removing keys {}", key);
        let _ = db.rem(&key);
    }
}

pub fn store(db_path: &str) {
    let mut buffer: Vec<u8> = Vec::new();
    let _ = io::stdin().read_to_end(&mut buffer);

    match str::from_utf8(&buffer) {
        Ok(valid) => {
            if valid.trim().is_empty(){
                return
            }
        }
        Err(error) => {
            log! ("Encountered Errror {}", error);
        }

    }

    if buffer.len() > 5_usize*10_usize.pow(6) {
        return
    }

    let mut db = DBManager{db_path: db_path, read_only: false}.open();

    let id;
    match db.get::<u32>("id") {
        Some(v) => {
            id = v + 1;
        }
        None => {
            id = 0;
        }
    }
    log! ("curr id: {}", id);
    db.set("id", &id).unwrap();

    deduplicate(&buffer, &mut db);
    db.set(&String::from(id.to_string()), &buffer).unwrap();
}

pub fn list(db_path: &str, max_width: usize) {
    let db = DBManager{db_path: db_path, read_only: true}.open();

    let mut entries: Vec<_> = db.iter().filter(|s| s.get_key() != "id").collect();
    entries.sort_by(|a, b| {
        let ai: i16 = a.get_key().parse().unwrap();
        let bi: i16 = b.get_key().parse().unwrap();
        ai.cmp(&bi).reverse()
    });

    let mut lock = io::stdout().lock();
    for kv in entries {
        let mut new_val;
        let format: String;
        let value = kv.get_value::<Vec<u8>>().unwrap();
        match String::from_utf8(value.clone()) {
            Ok(valid) => {
                new_val = valid.split_whitespace().collect::<Vec<_>>().join(" ");
                new_val.truncate(max_width);
                format = format!("{}:{}", kv.get_key(), new_val);
            }
            Err(_error) => {
                new_val = String::from("Image");
                format = format!("{}:{}-{}", kv.get_key(), new_val, value.len());

            }
        }

        writeln! (lock,"{}", format).unwrap();
    }
}

pub fn decode(db_path: &str) -> Result<(), std::io::Error> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let _ = stdin.read_to_string(&mut buffer);

    match buffer.split_once(':') {
        Some((key, _val)) => {
            let db = DBManager{db_path: db_path, read_only: true}.open();
            let mut lock = io::stdout().lock();
            lock.write_all(&db.get::<Vec<u8>>(key).unwrap())
        }
        None => {
            log! ("Error, this case impossible, bug in code!");
            Ok(())
        }
    }
}



fn main() {
    let db_path = "/opt/hosting/cliphist/cliphist/target/cliphist.db";
    let preview_width = 100;
    let args = Args::parse();

    if args.verbose {
        logger::enable_logging()
    }

    if args.store {
        let _ = store(db_path);
    }
    if args.list {
        let _ = list(db_path, preview_width);
    }
    if args.decode {
        let _ = decode(db_path);
    }
}
