# TantanganTSP_13523066
> Tugas Tantangan IF 2211 Strategi Algoritma

> Disusun oleh Muhammad Ghifary Komara Putra - 13523066

### Deskripsi Singkat

Repositori ini berisi program penyelesaian persoalan TSP dengan pendekatan pemrograman dinamis menggunakan bahasa Rust. Pengguna dapat memasukkan matriks ketetanggaan suatu graf dan program akan memberikan rute optimal untuk perjalanan dari dan ke simpul 0, biaya total yang dibutuhkan, dan lama waktu pencarian solusi.

### Requirement, Instalasi, dan Panduan Penggunaan

Untuk menjalankan program, silakan ikuti langkah-langkah berikut:
1. Lakukan instalasi Rust. (Program dikembangkan dengan Rust versi 1.87.0. Silakan unduh versi yang kompatibel)
2. Clone repositori ini pada perangkat Anda.
3. Jalankan perintah berikut dalam root directory:
```
cargo build
cargo run
```
4. Contoh masukan:

 ```
Jumlah simpul:
4
Matriks Ketetanggaan:
0 10 15 20
5 0 9 10
6 13 0 12
8 8 9 0
```

5. Selamat menjalankan program!

### Tangkapan Layar Hasil Pengujian
> Catatan: Tangkapan layar tersedia pula dalam folder test

1. Matriks 4x4

![Screenshot 2025-05-27 045504](https://github.com/user-attachments/assets/daa085a8-c4d8-42f4-8553-505e29a66a32)

2. Matriks 5x5

![Screenshot 2025-05-27 045517](https://github.com/user-attachments/assets/bcb63fe8-da3a-4cbe-a625-bca4d26890a6)

3. Matriks 6x6

![Screenshot 2025-05-27 045533](https://github.com/user-attachments/assets/a3eab43f-65bd-4d44-9f6d-2d692744c3ad)

4. Matriks 7x7

![Screenshot 2025-05-27 045548](https://github.com/user-attachments/assets/0a0270f1-e068-4a09-a4cc-182788f187f9)

5. Matriks 8x8

![Screenshot 2025-05-27 045925](https://github.com/user-attachments/assets/bb005665-bfac-4cce-b02b-63e8a946e723)
