# REST API Wilayah Administrasi Indonesia

Ini adalah sebuah REST API yang menyediakan data wilayah administrasi Indonesia. Database SQLite ini sudah terisi dengan data wilayah Indonesia yang bersumber dari github [guzfirdaus/Wilayah-Administrasi-Indonesia](https://github.com/guzfirdaus/Wilayah-Administrasi-Indonesia).

## Teknologi yang digunakan
1. [Rust](https://www.rust-lang.org/) (Bahasa pemrograman)
2. [Actix Web](https://actix.rs/) (Framework Web)
3. [SQLite](https://www.sqlite.org/) (Database)
4. [Docker](https://www.docker.com/) (Kontainerisasi)

## Fitur
1. List data provinsi
2. List data kota berdasarkan ID provinsi
3. List data kecamatan berdasarkan ID kota
4. List data desa berdasarkan ID kecamatan
5. Data full administrasi berdasarkan ID desa. Contoh:
```json
{
	"province_code": "32",
	"province_name": "JAWA BARAT",
	"city_code": "32.04",
	"city_name": "KAB. BANDUNG",
	"region_code": "32.04.08",
	"region_name": "Bojongsoang",
	"village_code": "32.04.08.2002",
	"village_name": "Bojongsoang"
}
```
## Penggunaan
Untuk menggunakan REST API ini, Anda perlu menggunakan [Postman](https://www.postman.com/) atau [Insomnia](https://insomnia.rest/) untuk mengakses endpoint tersebut.

### Penggunaan dengan Insomnia
1. Buka Insomnia
2. Klik `Import`
3. Pilih file `insomnia_wilayah_indonesia.json` lalu klik `Scan`, lalu klik `Import`

### Docker

#### Build image
```bash
docker build -t wilayah-service .
```

#### Menjalankan image
```bash
docker run -p 8880:8080 --init --rm --name wilayah-service -v /rust-wilayah-service/data:/data wilayah-service
```
Ganti volume `/rust-wilayah-service/data` dengan lokasi dimana database wilayah disimpan. pada linux/mac, Anda bisa menggunakan `pwd` untuk mendapatkan lokasi tersebut.

#### Compose
```yaml
services:
  wilayah-service:
    image: wilayah-service
    ports:
      - "8880:8080"
    volumes:
      - ./data:/data
```

Aplikasi microservice ini bisa dijalankan sendiri, namun akan lebih jika dipasangkan dengan aplikasi lain dan menggunakan API gateway seperti [Krakend](https://www.krakend.io/) atau [Apache APISIX](https://apisix.apache.org/).
