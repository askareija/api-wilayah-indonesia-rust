use rusqlite::OptionalExtension;
use rusqlite::{params, Connection, OpenFlags, Result};
use std::sync::{Arc, Mutex};
use thiserror::Error;

use crate::models::{District, FullAdminData, Province, Regency, Village};

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    SQLiteError(#[from] rusqlite::Error),
    #[error("No data found")]
    NotFound,
}

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, DatabaseError> {
        // Use thread-safe open flags
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_FULL_MUTEX,
        )?;

        let conn = Arc::new(Mutex::new(conn));

        Ok(Database { conn })
    }

    // SECTION: Provinces
    pub fn get_all_provinces(&self) -> Result<Vec<Province>, DatabaseError> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare("SELECT id, code, name FROM provinces")?;
        let province_iter = stmt.query_map([], |row| {
            Ok(Province {
                id: row.get(0)?,
                code: row.get(1)?,
                name: row.get(2)?,
            })
        })?;

        let mut provinces = Vec::new();
        for province in province_iter {
            provinces.push(province?);
        }
        Ok(provinces)
    }

    pub fn get_one_province(&self, province_id: i64) -> Result<Option<Province>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, code, name FROM provinces WHERE id = ?1")?;

        let province = stmt
            .query_row(params![province_id], |row| {
                Ok(Province {
                    id: row.get(0)?,
                    code: row.get(1)?,
                    name: row.get(2)?,
                })
            })
            .optional()?;

        Ok(province)
    }

    pub fn create_province(&self, province: &Province) -> Result<i64, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO provinces (code, name) VALUES (?1, ?2)",
            params![province.code, province.name],
        )?;
        Ok(conn.last_insert_rowid())
    }

    // Update an existing province
    pub fn update_province(&self, id: i64, province: &Province) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();

        // Check if the province exists before updating
        let affected_rows = conn.execute(
            "UPDATE provinces SET code = ?1, name = ?2 WHERE id = ?3",
            params![province.code, province.name, id],
        )?;

        // If no rows were affected, it means the province doesn't exist
        if affected_rows == 0 {
            return Err(DatabaseError::NotFound);
        }

        Ok(())
    }

    // Delete a province
    pub fn delete_province(&self, id: i64) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();

        // Check if the province exists and can be deleted
        let affected_rows = conn.execute("DELETE FROM provinces WHERE id = ?1", params![id])?;

        // If no rows were affected, it means the province doesn't exist
        if affected_rows == 0 {
            return Err(DatabaseError::NotFound);
        }

        Ok(())
    }
    // !SECTION: Provinces

    // SECTION: Regencies
    pub fn get_regencies_by_province(
        &self,
        province_id: i64,
    ) -> Result<Vec<Regency>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, code, name, province_id FROM regencies WHERE province_id = ?1")?;

        let regency_iter = stmt.query_map(params![province_id], |row| {
            Ok(Regency {
                id: row.get(0)?,
                code: row.get(1)?,
                name: row.get(2)?,
                province_id: row.get(3)?,
            })
        })?;

        let mut regencies = Vec::new();
        for regency in regency_iter {
            regencies.push(regency?);
        }
        Ok(regencies)
    }

    pub fn get_one_regency(&self, regency_id: i64) -> Result<Option<Regency>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, code, name, province_id FROM regencies WHERE id = ?1")?;

        let regency = stmt
            .query_row(params![regency_id], |row| {
                Ok(Regency {
                    id: row.get(0)?,
                    code: row.get(1)?,
                    name: row.get(2)?,
                    province_id: row.get(3)?,
                })
            })
            .optional()?;

        Ok(regency)
    }

    pub fn create_regency(&self, regency: &Regency) -> Result<i64, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO regencies (code, name, province_id) VALUES (?1, ?2, ?3)",
            params![regency.code, regency.name, regency.province_id],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn update_regency(&self, id: i64, regency: &Regency) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();

        // Check if the regency exists before updating
        let affected_rows = conn.execute(
            "UPDATE regencies SET code = ?1, name = ?2, province_id = ?3 WHERE id = ?4",
            params![regency.code, regency.name, regency.province_id, id],
        )?;

        // If no rows were affected, it means the regency doesn't exist
        if affected_rows == 0 {
            return Err(DatabaseError::NotFound);
        }

        Ok(())
    }

    pub fn delete_regency(&self, id: i64) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();

        // Check if the regency exists and can be deleted
        let affected_rows = conn.execute("DELETE FROM regencies WHERE id = ?1", params![id])?;

        // If no rows were affected, it means the regency doesn't exist
        if affected_rows == 0 {
            return Err(DatabaseError::NotFound);
        }

        Ok(())
    }
    // !SECTION: Regencies

    // SECTION: Districts
    pub fn get_districts_by_regency(
        &self,
        regency_id: i64,
    ) -> Result<Vec<District>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT id, code, name, regency_id FROM districts WHERE regency_id = ?1")?;

        let district_iter = stmt.query_map(params![regency_id], |row| {
            Ok(District {
                id: row.get(0)?,
                code: row.get(1)?,
                name: row.get(2)?,
                regency_id: row.get(3)?,
            })
        })?;

        let mut districts = Vec::new();
        for district in district_iter {
            districts.push(district?);
        }
        Ok(districts)
    }

    pub fn get_one_district(&self, district_id: i64) -> Result<Option<District>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, code, name, regency_id FROM districts WHERE id = ?1")?;

        let district = stmt
            .query_row(params![district_id], |row| {
                Ok(District {
                    id: row.get(0)?,
                    code: row.get(1)?,
                    name: row.get(2)?,
                    regency_id: row.get(3)?,
                })
            })
            .optional()?;

        Ok(district)
    }

    pub fn create_district(&self, district: &District) -> Result<i64, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO districts (code, name, regency_id) VALUES (?1, ?2, ?3)",
            params![district.code, district.name, district.regency_id],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn update_district(&self, id: i64, district: &District) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();

        // Check if the district exists before updating
        let affected_rows = conn.execute(
            "UPDATE districts SET code = ?1, name = ?2, regency_id = ?3 WHERE id = ?4",
            params![district.code, district.name, district.regency_id, id],
        )?;

        // If no rows were affected, it means the district doesn't exist
        if affected_rows == 0 {
            return Err(DatabaseError::NotFound);
        }

        Ok(())
    }

    pub fn delete_district(&self, id: i64) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();

        // Check if the district exists and can be deleted
        let affected_rows = conn.execute("DELETE FROM districts WHERE id = ?1", params![id])?;

        // If no rows were affected, it means the district doesn't exist
        if affected_rows == 0 {
            return Err(DatabaseError::NotFound);
        }

        Ok(())
    }
    // !SECTION: Districts

    // SECTION: Villages
    pub fn get_villages_by_district(
        &self,
        district_id: i64,
    ) -> Result<Vec<Village>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, code, name, district_id FROM villages WHERE district_id = ?1")?;

        let village_iter = stmt.query_map(params![district_id], |row| {
            Ok(Village {
                id: row.get(0)?,
                code: row.get(1)?,
                name: row.get(2)?,
                district_id: row.get(3)?,
            })
        })?;

        let mut villages = Vec::new();
        for village in village_iter {
            villages.push(village?);
        }
        Ok(villages)
    }

    pub fn get_one_village(&self, village_id: i64) -> Result<Option<Village>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, code, name, district_id FROM villages WHERE id = ?1")?;

        let village = stmt
            .query_row(params![village_id], |row| {
                Ok(Village {
                    id: row.get(0)?,
                    code: row.get(1)?,
                    name: row.get(2)?,
                    district_id: row.get(3)?,
                })
            })
            .optional()?;

        Ok(village)
    }

    pub fn create_village(&self, village: &Village) -> Result<i64, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO villages (code, name, district_id) VALUES (?1, ?2, ?3)",
            params![village.code, village.name, village.district_id],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn update_village(&self, id: i64, village: &Village) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();

        // Check if the village exists before updating
        let affected_rows = conn.execute(
            "UPDATE villages SET code = ?1, name = ?2, district_id = ?3 WHERE id = ?4",
            params![village.code, village.name, village.district_id, id],
        )?;

        // If no rows were affected, it means the village doesn't exist
        if affected_rows == 0 {
            return Err(DatabaseError::NotFound);
        }

        Ok(())
    }

    pub fn delete_village(&self, id: i64) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();

        // Check if the village exists and can be deleted
        let affected_rows = conn.execute("DELETE FROM villages WHERE id = ?1", params![id])?;

        // If no rows were affected, it means the village doesn't exist
        if affected_rows == 0 {
            return Err(DatabaseError::NotFound);
        }

        Ok(())
    }

    pub fn get_full_admin_data(
        &self,
        village_id: i64,
    ) -> Result<Option<FullAdminData>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "
            SELECT 
                p.code as province_code, 
                p.name as province_name, 
                r.code as regency_code, 
                r.name as regency_name, 
                d.code as district_code, 
                d.name as district_name, 
                v.code as village_code, 
                v.name as village_name
            FROM villages v
            JOIN districts d ON v.district_id = d.id
            JOIN regencies r ON d.regency_id = r.id
            JOIN provinces p ON r.province_id = p.id
            WHERE v.id = ?1
        ",
        )?;

        let full_data = stmt
            .query_row(params![village_id], |row| {
                Ok(FullAdminData {
                    province_code: row.get(0)?,
                    province_name: row.get(1)?,
                    city_code: row.get(2)?,
                    city_name: row.get(3)?,
                    region_code: row.get(4)?,
                    region_name: row.get(5)?,
                    village_code: row.get(6)?,
                    village_name: row.get(7)?,
                })
            })
            .optional()?;

        Ok(full_data)
    }
    // !SECTION: Villages
}

// Implement Clone to allow easy sharing between threads
impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            conn: Arc::clone(&self.conn),
        }
    }
}
