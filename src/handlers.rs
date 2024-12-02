use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::db::Database;
use crate::models::{District, FullAdminData, Province, Regency, Village};

pub async fn get_all_provinces(db: web::Data<Database>) -> impl Responder {
    match db.get_all_provinces() {
        Ok(provinces) => HttpResponse::Ok().json(provinces),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch provinces: {}", e)
        })),
    }
}

pub async fn get_one_province(db: web::Data<Database>, path: web::Path<i64>) -> impl Responder {
    match db.get_one_province(path.into_inner()) {
        Ok(Some(province)) => HttpResponse::Ok().json(province),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "Province not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch province: {}", e)
        })),
    }
}

pub async fn get_one_regency(db: web::Data<Database>, path: web::Path<i64>) -> impl Responder {
    match db.get_one_regency(path.into_inner()) {
        Ok(Some(regency)) => HttpResponse::Ok().json(regency),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "Regency not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch regency: {}", e)
        })),
    }
}

pub async fn get_one_district(db: web::Data<Database>, path: web::Path<i64>) -> impl Responder {
    match db.get_one_district(path.into_inner()) {
        Ok(Some(district)) => HttpResponse::Ok().json(district),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "District not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch district: {}", e)
        })),
    }
}

pub async fn get_one_village(db: web::Data<Database>, path: web::Path<i64>) -> impl Responder {
    match db.get_one_village(path.into_inner()) {
        Ok(Some(village)) => HttpResponse::Ok().json(village),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "Village not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch village: {}", e)
        })),
    }
}

pub async fn create_province(db: web::Data<Database>, province: web::Json<Province>) -> impl Responder {
    match db.create_province(&province.into_inner()) {
        Ok(id) => HttpResponse::Created().json(json!({
            "id": id
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to create province: {}", e)
        })),
    }
}

pub async fn create_regency(db: web::Data<Database>, regency: web::Json<Regency>) -> impl Responder {
    match db.create_regency(&regency.into_inner()) {
        Ok(id) => HttpResponse::Created().json(json!({
            "id": id
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to create regency: {}", e)
        })),
    }
}

pub async fn create_district(db: web::Data<Database>, district: web::Json<District>) -> impl Responder {
    match db.create_district(&district.into_inner()) {
        Ok(id) => HttpResponse::Created().json(json!({
            "id": id
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to create district: {}", e)
        })),
    }
}

pub async fn create_village(db: web::Data<Database>, village: web::Json<Village>) -> impl Responder {
    match db.create_village(&village.into_inner()) {
        Ok(id) => HttpResponse::Created().json(json!({
            "id": id
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to create village: {}", e)
        })),
    }
}

pub async fn update_province(db: web::Data<Database>, path: web::Path<i64>, province: web::Json<Province>) -> impl Responder {
    match db.update_province(path.into_inner(), &province.into_inner()) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to update province: {}", e)
        })),
    }
}

pub async fn update_regency(db: web::Data<Database>, path: web::Path<i64>, regency: web::Json<Regency>) -> impl Responder {
    match db.update_regency(path.into_inner(), &regency.into_inner()) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to update regency: {}", e)
        })),
    }
}

pub async fn update_district(db: web::Data<Database>, path: web::Path<i64>, district: web::Json<District>) -> impl Responder {
    match db.update_district(path.into_inner(), &district.into_inner()) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to update district: {}", e)
        })),
    }
}

pub async fn update_village(db: web::Data<Database>, path: web::Path<i64>, village: web::Json<Village>) -> impl Responder {
    match db.update_village(path.into_inner(), &village.into_inner()) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to update village: {}", e)
        })),
    }
}

pub async fn delete_province(db: web::Data<Database>, path: web::Path<i64>) -> impl Responder {
    match db.delete_province(path.into_inner()) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to delete province: {}", e)
        })),
    }
}

pub async fn delete_regency(db: web::Data<Database>, path: web::Path<i64>) -> impl Responder {
    match db.delete_regency(path.into_inner()) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to delete regency: {}", e)
        })),
    }
}

pub async fn delete_district(db: web::Data<Database>, path: web::Path<i64>) -> impl Responder {
    match db.delete_district(path.into_inner()) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to delete district: {}", e)
        })),
    }
}

pub async fn delete_village(db: web::Data<Database>, path: web::Path<i64>) -> impl Responder {
    match db.delete_village(path.into_inner()) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to delete village: {}", e)
        })),
    }
}

pub async fn get_regencies_by_province(
    db: web::Data<Database>,
    province_id: web::Path<i64>,
) -> impl Responder {
    match db.get_regencies_by_province(province_id.into_inner()) {
        Ok(regencies) => HttpResponse::Ok().json(regencies),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch regencies: {}", e)
        })),
    }
}

// Get Districts by Regency ID
pub async fn get_districts_by_regency(
    db: web::Data<Database>,
    regency_id: web::Path<i64>,
) -> impl Responder {
    match db.get_districts_by_regency(regency_id.into_inner()) {
        Ok(districts) => HttpResponse::Ok().json(districts),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch districts: {}", e)
        })),
    }
}

// Get Villages by District ID
pub async fn get_villages_by_district(
    db: web::Data<Database>,
    district_id: web::Path<i64>,
) -> impl Responder {
    match db.get_villages_by_district(district_id.into_inner()) {
        Ok(villages) => HttpResponse::Ok().json(villages),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch villages: {}", e)
        })),
    }
}

// Get Full Admin Data by Village ID
pub async fn get_full_admin_data(
    db: web::Data<Database>,
    village_id: web::Path<i64>,
) -> impl Responder {
    match db.get_full_admin_data(village_id.into_inner()) {
        Ok(Some(full_data)) => HttpResponse::Ok().json(full_data),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "Village not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch admin data: {}", e)
        })),
    }
}
