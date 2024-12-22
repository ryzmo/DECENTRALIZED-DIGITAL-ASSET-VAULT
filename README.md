# Decentralized Digital Asset Vault

Decentralized Digital Asset Vault adalah aplikasi backend berbasis Rust yang berjalan di atas **Internet Computer Protocol (ICP)**. Aplikasi ini berguna untuk pengguna mengelola aset digital mereka secara terdesentralisasi dengan keamanan tinggi.

## Fitur

1. **Tambah Aset (****`add_asset`****)**:

   - Aset dienkripsi dan hanya dapat diakses oleh pemilik atau pengguna yang diberikan akses.

2. **Berikan Akses (****`share_asset`****)**:

   - Pemilik dapat memberikan izin akses ke aset mereka kepada pengguna lain.

3. **Lihat Detail Aset (****`get_asset`****)**:

   - Pengguna yang memiliki izin dapat melihat detail aset yang dibagikan.

4. **Riwayat Akses (****`get_access_log`****)**:

   - Pemilik dapat melihat riwayat siapa saja yang mengakses aset mereka.

## Manfaat

- **Privasi Data**:
  - Data dienkripsi sehingga aman dari akses tidak sah.
- **Transparansi**:
  - Semua akses dicatat sehingga dapat diaudit kapan saja.
- **Desentralisasi**:
  - Berjalan di atas ICP sehingga tidak tergantung pada server pusat.
- **Fleksibilitas**:
  - Mendukung berbagai jenis data digital.

---

## Persyaratan Sistem

Pastikan Anda memiliki perangkat dengan spesifikasi berikut:

- Sistem Operasi: Windows, macOS, atau Linux
- **Node.js** versi 16 atau lebih baru
- **Rust** versi terbaru
- **DFX SDK** versi 0.18.0 atau lebih baru

---

## Instalasi dan Pengaturan

### 1. **Instal Dependensi**

Ikuti langkah-langkah berikut untuk menginstal semua alat yang diperlukan:

#### a. Instal Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

#### b. Instal DFX SDK

```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
dfx --version
```

#### c. Instal Node.js

Gunakan [Node.js](https://nodejs.org/) versi 16 atau lebih baru:

```bash
sudo apt install nodejs npm   # Linux
brew install node             # macOS
winget install -e --id OpenJS.NodeJS   # Windows
node --version
```

### 2. **Kloning Repository**

Kloning proyek ini ke komputer Anda:

```bash
git clone https://github.com/ryzmo/DECENTRALIZED-DIGITAL-ASSET-VAULT
cd icp-101-rust-boilerplate
```

### 3. **Konfigurasi Proyek**

Pastikan file `dfx.json` dan `Cargo.toml` sudah terkonfigurasi dengan benar. Struktur direktori proyek adalah sebagai berikut:

```
icp-101-rust-boilerplate/
├── dfx.json
├── Cargo.toml
├── src/
│   ├── icp_rust_boilerplate_backend/
│   │   ├── Cargo.toml
│   │   ├── icp_rust_boilerplate_backend.did
│   │   ├── src/
│   │       ├── lib.rs
```

---

## Langkah Menjalankan Proyek

### 1. **Mulai DFX**

Jalankan DFX lokal:

```bash
dfx start --clean
```

### 2. **Bangun Proyek**

Bangun canister dengan perintah berikut:

```bash
dfx build
```

### 3. **Deploy Canister**

Deploy aplikasi ke DFX lokal:

```bash
dfx deploy
```

### 4. **Gunakan Antarmuka Candid**

Buka antarmuka Candid UI di browser untuk menguji API:

```bash
http://localhost:4943
```

Masukkan nama canister Anda dan panggil fungsi seperti `add_asset`, `share_asset`, `get_asset`, atau `get_access_log`.

---

## Cara Penggunaan

### Tambah Aset

Panggil `add_asset`:

- Parameter: `(owner, name, content)`
- Contoh:
  ```
  owner: "user1"
  name: "Document1"
  content: [1, 2, 3, 4]
  ```
- Output: ID aset (contoh: `1`)

### Berikan Akses

Panggil `share_asset`:

- Parameter: `(asset_id, owner, recipient)`
- Contoh:
  ```
  asset_id: 1
  owner: "user1"
  recipient: "user2"
  ```

### Lihat Detail Aset

Panggil `get_asset`:

- Parameter: `(asset_id, user_id)`
- Contoh:
  ```
  asset_id: 1
  user_id: "user2"
  ```

### Lihat Riwayat Akses

Panggil `get_access_log`:

- Parameter: `(asset_id, owner)`
- Contoh:
  ```
  asset_id: 1
  owner: "user1"
  ```

---

## Troubleshooting

### Masalah Umum

1. **Error: ****`BorrowMutError`**

   - Penyebab: Peminjaman ganda pada `RefCell`.
   - Solusi: Pastikan tidak ada peminjaman ganda dalam kode.

2. **Error: ****`Cannot start replica`**

   - Penyebab: DFX sudah berjalan.
   - Solusi: Jalankan `dfx stop` sebelum memulai ulang.

3. **Error: ****`Canister build failed`**

   - Penyebab: Masalah dengan `Cargo.toml` atau dependensi.
   - Solusi: Periksa konfigurasi dan jalankan `cargo check`.

---

## Informasi Tambahan

- Dokumentasi Rust: [Rust-lang](https://www.rust-lang.org/)
- Dokumentasi ICP: [Internet Computer](https://internetcomputer.org/)
- Forum Diskusi: [DFINITY Forum](https://forum.dfinity.org/)

---

## Lisensi

Proyek ini dilisensikan di bawah [MIT License](LICENSE).

