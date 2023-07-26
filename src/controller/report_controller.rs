use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

#[get("/report/daily")]
pub async fn get_daily_report() -> Result<NamedFile, NotFound<String>> {
    let mut last_file: Option<PathBuf> = None;
    let mut last_file_created_time: Option<SystemTime> = None;
    let files = fs::read_dir("report/daily").unwrap();
    files.for_each(|entry| {
        if let Ok(x) = entry {
            if let Ok(y) = x.metadata() {
                if let Ok(z) = y.created() {
                    if last_file.is_none() {
                        last_file = Some(x.path());
                        last_file_created_time = Some(z);
                    } else if z > last_file_created_time.unwrap() {
                        last_file = Some(x.path());
                        last_file_created_time = Some(z);
                    }
                }
            }
        }
    });
    match last_file {
        Some(path) => NamedFile::open(path.display().to_string())
            .await
            .map_err(|e| NotFound(e.to_string())),
        None => Err(NotFound("path not found".to_string())),
    }
}

#[get("/report/weekly")]
pub async fn get_weekly_report() -> Result<NamedFile, NotFound<String>> {
    let mut last_file: Option<PathBuf> = None;
    let mut last_file_created_time: Option<SystemTime> = None;
    let files = fs::read_dir("report/weekly").unwrap();
    files.for_each(|entry| {
        if let Ok(x) = entry {
            if let Ok(y) = x.metadata() {
                if let Ok(z) = y.created() {
                    if last_file.is_none() {
                        last_file = Some(x.path());
                        last_file_created_time = Some(z);
                    } else if z > last_file_created_time.unwrap() {
                        last_file = Some(x.path());
                        last_file_created_time = Some(z);
                    }
                }
            }
        }
    });
    match last_file {
        Some(path) => NamedFile::open(path.display().to_string())
            .await
            .map_err(|e| NotFound(e.to_string())),
        None => Err(NotFound("path not found".to_string())),
    }
}

#[get("/report/monthly")]
pub async fn get_monthly_report() -> Result<NamedFile, NotFound<String>> {
    let mut last_file: Option<PathBuf> = None;
    let mut last_file_created_time: Option<SystemTime> = None;
    let files = fs::read_dir("report/monthly").unwrap();
    files.for_each(|entry| {
        if let Ok(x) = entry {
            if let Ok(y) = x.metadata() {
                if let Ok(z) = y.created() {
                    if last_file.is_none() {
                        last_file = Some(x.path());
                        last_file_created_time = Some(z);
                    } else if z > last_file_created_time.unwrap() {
                        last_file = Some(x.path());
                        last_file_created_time = Some(z);
                    }
                }
            }
        }
    });
    match last_file {
        Some(path) => NamedFile::open(path.display().to_string())
            .await
            .map_err(|e| NotFound(e.to_string())),
        None => Err(NotFound("path not found".to_string())),
    }
}
