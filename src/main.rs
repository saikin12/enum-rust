fn main() {
    let arah=Arah::utara;
    match arah {
        Arah::utara=>println!("bergerak ke utara"),
        Arah::selatan=>println!("bergerak ke selatan"),
        Arah::timur=>println!("bergerak ke timur"),
        Arah::barat=>println!("bergerak ke barat"),

    }
}


enum Arah{
    timur,
    barat,
    selatan,
    utara,
}