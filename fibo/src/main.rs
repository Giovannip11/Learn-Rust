fn main() {
    //Fiboncacci recursiva.
    fn fibo(x: u32) ->u32 {
        if x<2 {
            return x;
        }
            return fibo(x-1)+ fibo(x-2);       
        }

        let resultado=fibo(20);
        println!("Resultado:{resultado}")
}

