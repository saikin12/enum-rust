fn main() {
    let arah=Arah::utara;
    match arah {
        Arah::utara=>println!("bergerak ke utara"),
        Arah::selatan=>println!("bergerak ke selatan"),
        Arah::timur=>println!("bergerak ke timur"),
        Arah::barat=>println!("bergerak ke barat"),
       
    }
     println!("{}", deskripsi_status(StatusHTTP::NotFound ));

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
fn deskripsi_status(status:StatusHTTP)->&'static str{
    match status{
        StatusHTTP::Ok =>"200 - berhasil",
        StatusHTTP::NotFound =>"404 - tidak ditemukan",
        StatusHTTP::InternalServerError => "500 error server",
        StatusHTTP::Unauthorized => "401 -tidak ditemukan",


    }
}