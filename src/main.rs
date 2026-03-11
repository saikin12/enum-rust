fn main() {
    // let arah=Arah::timur;
    // match arah {
    //     Arah::utara=>println!("bergerak ke utara"),
    //     Arah::selatan=>println!("bergerak ke selatan"),
    //     Arah::timur=>println!("bergerak ke timur"),
    //     Arah::barat=>println!("bergerak ke barat"),
       
    // }
    //  println!("{}", deskripsi_status(StatusHTTP::NotFound ));
    //  let arah2=Arah::timur;
    //  if let arah ::timur=arah2{
    //     println!("kembali pulang");
    //  }


     let bentuk1=Bentuk::Lingkaran(5.0);
     let bentuk2=Bentuk:: PersegiPanjang(4.0, 5.0);

     println!("Luas Lingkarang : {:.2}", hitung_luas(&bentuk1) );



}


enum Arah{
    timur,
    barat,
    selatan,
    utara,
}

enum StatusHTTP {
     Ok,
     NotFound,
     InternalServerError,
     Unauthorized,
}

enum Bentuk {
    Lingkaran(f64),
    Persegi(f64),
    PersegiPanjang(f64, f64),
    Segitiga(f64, f64, f64),
}

fn hitung_luas(bentuk: &Bentuk) -> f64{
    match bentuk{
        Bentuk::Lingkaran(r) => std::f64::consts::PI * r * r,
        Bentuk::Persegi(s) => s * s,
        Bentuk::PersegiPanjang(l,t)=> l * t,
        Bentuk::Segitiga(a,b,c) =>{
            let s=(a+b+c) / 2.0;
            (s *(s-a) * (s-b) * (s -c)).sqrt()
        }
    }

}

fn deskripsi_status(status:StatusHTTP)->&'static str{
    match status{
        StatusHTTP::Ok =>"200 - berhasil",
        StatusHTTP::NotFound =>"404 - tidak ditemukan",
        StatusHTTP::InternalServerError => "500 error server",
        StatusHTTP::Unauthorized => "401 -tidak ditemukan",


    }
}