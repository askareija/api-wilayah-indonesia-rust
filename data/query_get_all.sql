SELECT 
    provinces.name AS nama_provinsi,
    provinces.code AS kode_provinsi,
    regencies.name AS nama_kota,
    regencies.code AS kode_kota,
    districts.name AS nama_kecamatan,
    districts.code AS kode_kecamatan,
    villages.name AS nama_desa,
    villages.code AS kode_desa
FROM 
    villages
INNER JOIN districts ON villages.district_id = districts.id
INNER JOIN regencies ON districts.regency_id = regencies.id
INNER JOIN provinces ON regencies.province_id = provinces.id
WHERE villages.id IN (3204082001, 3204082002) OR villages.code IN ('32.04.08.2001', '32.04.08.2002');
