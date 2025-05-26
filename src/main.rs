use std::io;
use std::cmp::min;
use std::time::Instant;

 // Jarak antar dua simpul yang tidak terhubung
const INF:i32 = i32::MAX/2;

fn main(){
    // Input pengguna
    let mut input = String::new();

    println!("Jumlah simpul:");
    io::stdin().read_line(&mut input).expect("Gagal membaca masukan.");
    let n: usize = input.trim().parse().expect("Harap masukkan angka.");
    
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    println!("Matriks Ketetanggaan:");
    for i in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).expect("Gagal membaca masukan.");

        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .enumerate()
            .map(|(j,s)| {
                let elmt:i32 = s.parse().expect("Gagal membaca masukan.");
                if i!=j && elmt ==0{
                    INF
                } else{
                    elmt
                }
            })
            .collect();

        if row.len() != n{
            panic!("Matriks ketetanggaan harus berupa matriks persegi.");
        }

        matrix.push(row);
    }
    
    // Pencarian Solusi
    let start = Instant::now();
    let (cost, path) = solve_dp(matrix);
    let duration = start.elapsed();
    println!("----------------- Pemrograman Dinamis -----------------");
    println!("Waktu pencarian: {:.4?}", duration);
    println!("Cost minimum   : {}", cost);
    print!("Rute Akhir     : ");
    print_path(&path);
}

fn solve_dp(matrix: Vec<Vec<i32>>) -> (i32, Vec<usize>){
    // banyak simpul
    let n = matrix.len();
    
    // size menyatakan banyak kombinasi simpul yang sudah dikunjungi
    let size = 1 << n;

    // dp[i][j] mewakili f(j, i)
    // i menyatakan himpunan simpul yang sudah dikunjungi, dinyatakan dalam bentuk bitmap
    // j menyatakan simpul yang terakhir dikunjungi dalam i
    // dp[i][j] menyatakan cost minimum mencapai j setelah mengunjungi seluruh simpul dalam i
    let mut dp = vec![vec![INF; n]; size];
    // parent[i][j] digunakan untuk menyimpan rute akhir/optimal. 
    // i menyatakan himpunan simpul yang sudah dikunjungi, dinyatakan dalam bentuk bitmap
    // j menyatakan simpul yang terakhir dikunjungi dalam i
    // parent[i][j] menyatakan simpul yang dikunjungi sebelum j dalam rute optimal
    let mut parent = vec![vec![None; n]; size];
    
    // titik awal perjalanan, simpul 0.
    dp[1][0] = 0;
    

    // tinjau seluruh kombinasi simpul-simpul yang sudah dikunjungi
    // mask ekivalen dengan himpunan S
    for mask in 1..size{
        // tinjau simpul ke-u sebagai simpul yang dikunjungi terakhir dalam mask
        for u in 0..n{
            // jika u tidak termasuk dalam mask, lewati
            if (mask & (1<<u)) == 0{
                continue;
            }
            // jika tidak, tinjau simpul v yang menjadi 1 langkah sebelum u
            for v in 0..n{
                // jika jika v==u, simpul v tidak bertetangga dengan u, atau v belum pernah dikunjungi, lewati
                if u==v || (mask & (1<<v))==0 || matrix[v][u]==INF{
                    continue;
                }
                // jika tidak, hitung c_vu + f(u, mask - {u})
                let prev_mask = mask^(1<<u);
                let new_cost = dp[prev_mask][v]+matrix[v][u];
                
                // f(v, mask) = min {c_vj + f(j, mask - {j})}, j elemen dari mask
                if new_cost<dp[mask][u]{
                    dp[mask][u] = new_cost;
                    parent[mask][u] = Some(v);
                }
            }
        }
    }
    // 0b11...1, menyatakan bitmap ketika seluruh simpul telah dikunjungi
    let full_mask = (1<<n)-1;
    // deklarasi awal untuk pencatatan rute optimal
    let mut min_cost = INF;
    let mut last_node = None;

    // untuk setiap simpul u selain simpul awal, 
    // tentukan 1 langkah optimal sebelumnya
    for u in 1..n{
        // jika u tidak bisa kembali ke simpul 0, lewati
        if matrix[u][0]==INF{
            continue;
        }
        let cost = dp[full_mask][u]+matrix[u][0];
        // simpan cost menuju simpul tersebut
        if cost<min_cost{
            min_cost = cost;
            last_node = Some(u);
        }
    }

    // lakukan backtrack untuk mendapatkan kembali rute optimal
    let mut path = Vec::new();
    if let Some(mut u) = last_node{
        // deklarasi awal: seluruh simpul dikunjungi, diawali dari simpul 0
        let mut mask = full_mask;
        path.push(0);
        let mut stack = Vec::new();

        // mundur dari langkah terakhir hingga ke langkah awal.
        // catat simpul ke dalam stack
        while let Some(prev)=parent[mask][u]{
            stack.push(u);
            mask = mask^(1<<u);
            u = prev;
        }

        // untuk mendapatkan rute, cukup balikkan (reverse) entri yang ada pada stack
        // masukkan ke dalam path
        while let Some(node) = stack.pop(){
            path.push(node);
        }
        // perjalanan berakhir di simpul 0
        path.push(0);
    }

    (min_cost, path)
}

fn print_path(path: &[usize]){
    let formatted = path.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" -> ");
    println!("{}", formatted);
}