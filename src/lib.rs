// lib.rs

// 필요한 모듈 가져오기
use csv::ReaderBuilder;
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;

// 1. Extract 함수
pub fn extract(url: &str, file_path: &str, _timeout: u64) -> Result<String, String> {
    match get(url) {
        Ok(response) => {
            if response.status().is_success() {
                if let Some(parent) = Path::new(file_path).parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create directory: {}", e))?;
                }
                let mut file =
                    File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;
                file.write_all(
                    &response
                        .bytes()
                        .map_err(|e| format!("Failed to read response body: {}", e))?,
                )
                .map_err(|e| format!("Failed to write to file: {}", e))?;
                Ok(file_path.to_string())
            } else {
                Err(format!("Request failed with status: {}", response.status()))
            }
        }
        Err(_) => Err("Failed to fetch the file".to_string()),
    }
}

// 2. Load 함수
pub fn load(dataset: &str) -> Result<String, String> {
    let file = File::open(dataset).map_err(|e| format!("Failed to open CSV file: {}", e))?;
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(file);

    let conn =
        Connection::open("HR_1.db").map_err(|e| format!("Failed to connect to database: {}", e))?;
    conn.execute("DROP TABLE IF EXISTS HR_1", [])
        .map_err(|e| format!("Failed to drop table: {}", e))?;
    conn.execute(
        "CREATE TABLE HR_1 (
            EmployeeNumber INTEGER PRIMARY KEY,
            Age INTEGER,
            Attrition TEXT,
            BusinessTravel TEXT,
            DailyRate INTEGER,
            Department TEXT,
            DistanceFromHome INTEGER,
            Education INTEGER,
            EducationField TEXT,
            EmployeeCount INTEGER
        )",
        [],
    )
    .map_err(|e| format!("Failed to create table: {}", e))?;

    for result in rdr.records() {
        let record = result.map_err(|e| format!("Failed to read record: {}", e))?;
        if record.len() >= 10 {
            let trimmed_row: Vec<&str> = record.iter().take(10).collect();
            conn.execute(
                "INSERT INTO HR_1 
                (EmployeeNumber, Age, Attrition, BusinessTravel, DailyRate, Department, DistanceFromHome, 
                Education, EducationField, EmployeeCount) 
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                params![trimmed_row[0], trimmed_row[1], trimmed_row[2], trimmed_row[3], trimmed_row[4],
                        trimmed_row[5], trimmed_row[6], trimmed_row[7], trimmed_row[8], trimmed_row[9]],
            ).map_err(|e| format!("Failed to insert record: {}", e))?;
        }
    }

    Ok("HR_1.db".to_string())
}

// 3. Query 함수들
pub fn query_create() -> Result<String, String> {
    let conn =
        Connection::open("HR_1.db").map_err(|e| format!("Failed to connect to database: {}", e))?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS HR_1 (
            EmployeeNumber INTEGER UNIQUE,
            Age INTEGER,
            Attrition TEXT,
            BusinessTravel TEXT,
            DailyRate INTEGER,
            Department TEXT,
            DistanceFromHome INTEGER,
            Education INTEGER,
            EducationField TEXT,
            EmployeeCount INTEGER
        )",
        [],
    )
    .map_err(|e| format!("Failed to create table: {}", e))?;

    conn.execute(
        "INSERT OR IGNORE INTO HR_1 
        (EmployeeNumber, Age, Attrition, BusinessTravel, DailyRate, Department, DistanceFromHome, 
        Education, EducationField, EmployeeCount)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            99999,
            30,
            "Yes",
            "Travel_Rarely",
            1100,
            "Sales",
            5,
            3,
            "Life Sciences",
            1
        ],
    )
    .map_err(|e| format!("Failed to insert data: {}", e))?;

    Ok("Create Success".to_string())
}

pub fn query_read() -> Result<String, String> {
    let conn =
        Connection::open("HR_1.db").map_err(|e| format!("Failed to connect to database: {}", e))?;
    let mut stmt = conn
        .prepare("SELECT * FROM HR_1")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i32>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, i32>(6)?,
                row.get::<_, i32>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, i32>(9)?,
            ))
        })
        .map_err(|e| format!("Failed to query data: {}", e))?;

    for row in rows {
        println!(
            "{:?}",
            row.map_err(|e| format!("Failed to read data: {}", e))?
        );
    }

    Ok("Read Success".to_string())
}

pub fn query_update() -> Result<String, String> {
    let conn =
        Connection::open("HR_1.db").map_err(|e| format!("Failed to connect to database: {}", e))?;
    conn.execute(
        "UPDATE HR_1 SET 
            Age = CASE WHEN Age IS NULL OR Age = '' THEN 9999999 ELSE Age END,
            Attrition = CASE WHEN Attrition IS NULL OR Attrition = '' THEN '9999999' ELSE Attrition END,
            BusinessTravel = CASE WHEN BusinessTravel IS NULL OR BusinessTravel = '' THEN '9999999' ELSE BusinessTravel END,
            DailyRate = CASE WHEN DailyRate IS NULL OR DailyRate = '' THEN 9999999 ELSE DailyRate END,
            Department = CASE WHEN Department IS NULL OR Department = '' THEN '9999999' ELSE Department END,
            DistanceFromHome = CASE WHEN DistanceFromHome IS NULL OR DistanceFromHome = '' THEN 9999999 ELSE DistanceFromHome END,
            Education = CASE WHEN Education IS NULL OR Education = '' THEN 9999999 ELSE Education END,
            EducationField = CASE WHEN EducationField IS NULL OR EducationField = '' THEN '9999999' ELSE EducationField END,
            EmployeeCount = CASE WHEN EmployeeCount IS NULL OR EmployeeCount = '' THEN 9999999 ELSE EmployeeCount END,
            EmployeeNumber = CASE WHEN EmployeeNumber IS NULL OR EmployeeNumber = '' THEN 9999999 ELSE EmployeeNumber END",
        []
    ).map_err(|e| format!("Failed to update records: {}", e))?;
    Ok("Update Success".to_string())
}

pub fn query_delete() -> Result<String, String> {
    let conn =
        Connection::open("HR_1.db").map_err(|e| format!("Failed to connect to database: {}", e))?;
    conn.execute(
        "DELETE FROM HR_1 WHERE Age <= 18 OR EmployeeNumber = 99999",
        [],
    )
    .map_err(|e| format!("Failed to delete records: {}", e))?;
    Ok("Delete Success".to_string())
}
