I want to create REST API for Indonesian administrative data, like get cities, provinces, etc. I will define the technology stack below:
1. Rust
2. Actix web framework
3. SQLite database

For entities and relations:
1. Provinces (has many cities)
2. Cities (belongs to province, has many regions)
3. Regions (belongs to cities, has many villages)
4. Village (belongs to region)

it has ID, code, name for every entities.

For need specifics, i want to be able to:
1. List All Provinces, Cities, Regions, Village
2. Get by parent id (Get cities by province, get regions by city, get villages by region)
3. Get full administration data (village, region, city, province) by village ID, the result will be:
    3a. Province code
    3b. Province name
    3c. City code
    3d. City name
    3e. Region code
    3f. Region name
    3g. Village code
    3h. Village name
4. Update & delete for every entities

