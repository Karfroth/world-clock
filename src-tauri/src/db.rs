use directories::ProjectDirs;
use jammdb::DB;
use uuid::Uuid;

fn get_db() -> Result<DB, jammdb::Error> {
    let dir = ProjectDirs::from("com", "karfkim",  "world clock").unwrap();
    let db_path = dir.data_dir().to_owned().join("db");
    let _ = std::fs::create_dir_all(db_path.clone());
    let db_path_str = format!("{}/my-database.db", db_path.to_str().unwrap());
    println!("Trying to open: {}", db_path_str);
    DB::open(db_path_str)
}

pub fn put(id: String, tz: String) -> Result<(), jammdb::Error> {
    let db = get_db()?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket("tzs")?;
    bucket.put(id, tz)?;
    tx.commit()
}

pub fn get(id: String) -> Vec<String> {
    let db = get_db().unwrap();
    let tx = db.tx(false).unwrap();
    let bucket = tx.get_bucket("tzs").unwrap();
    let tz = bucket.get(id);
    tz.map(|x| String::from_utf8(x.kv().value().to_owned()).unwrap()).into_iter().collect::<Vec<String>>()
}

pub fn get_or_create_tzs() -> Vec<String> {
    let db = get_db().unwrap();
    let tx = db.tx(true).unwrap();
    let bucket = tx.get_or_create_bucket("tzs").unwrap();
    let keys = bucket.kv_pairs().map(|kv| String::from_utf8(kv.key().to_owned()).unwrap()).collect::<Vec<String>>();
    
    let mut count = 0;
    let ks = if keys.len() == 4 {
        keys
    } else {
        let ks = (0..(4-keys.len())).map(|_x| {
            let id = Uuid::new_v4();
            let _ = bucket.put(id.to_string(), iana_time_zone::get_timezone().unwrap().to_string());
            count += 1;
            id.to_string()
        }).collect::<Vec<String>>();
        ks
    };
    let _ = tx.commit();
    ks
}