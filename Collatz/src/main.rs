fn collatz_leg(mut n: i32)->u32{
    let mut len:u32=0;
    while n >1 {
        if n % 2 ==0{
            n=n/2;
        }
        else{
            n=3*n+1;
        }
        len+=1;
    }
    return len;
}


fn main() {
    let resultado=collatz_leg(20);
    println!("tamanho={resultado}");
}
