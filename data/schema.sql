CREATE TABLE provinces (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE regencies (
    id INTEGER PRIMARY KEY,
    province_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (province_id) REFERENCES provinces(id)
);

CREATE TABLE districts (
    id INTEGER PRIMARY KEY,
    regency_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (regency_id) REFERENCES regencies(id)
);

CREATE TABLE villages (
    id INTEGER PRIMARY KEY,
    district_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (district_id) REFERENCES districts(id)
);

CREATE VIRTUAL TABLE villages_fts USING fts5(id, name, content=villages, content_rowid=id);

INSERT INTO villages_fts(rowid, name) SELECT id, name FROM villages;