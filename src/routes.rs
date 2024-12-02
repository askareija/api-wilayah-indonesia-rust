use crate::handlers;
use actix_web::web;

pub fn configure_province_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/provinces")
            .route("", web::get().to(handlers::get_all_provinces))
            .route("", web::post().to(handlers::create_province))
            .route("/{id}", web::get().to(handlers::get_one_province))
            .route("/{id}", web::put().to(handlers::update_province))
            .route("/{id}", web::delete().to(handlers::delete_province))
            .route(
                "/{province_id}/regencies",
                web::get().to(handlers::get_regencies_by_province),
            ),
    );
}

pub fn configure_regency_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/regencies")
            .route(
                "/{city_id}/districts",
                web::get().to(handlers::get_districts_by_regency),
            )
            .route("/{id}", web::get().to(handlers::get_one_regency)),
    );
}

pub fn configure_district_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/districts")
            .route(
                "/{region_id}/villages",
                web::get().to(handlers::get_villages_by_district),
            )
            .route("/{id}", web::get().to(handlers::get_one_district)),
    );
}

pub fn configure_village_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/villages")
            .route(
                "/{village_id}/details",
                web::get().to(handlers::get_full_admin_data),
            )
            .route("/{id}", web::get().to(handlers::get_one_village)),
    );
}
