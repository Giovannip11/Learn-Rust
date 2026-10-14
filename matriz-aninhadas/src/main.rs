fn transpose(matrix: [[i32;3]; 3]) -> [[i32; 3];3]{
    let mut resultado = [[0;3];3];
    for i in 0..3{
        for j in 0..3{
            resultado[j][i] = matrix[i][j];
        }
    }
    return resultado;
}

fn main() {
    let matrix = [
        [101,102,103],
        [201,202,203],
        [301,302,303]
    ];

    println!("Original:");
    for row in matrix{
        println!("{row:?}");
    }

    let transposta=transpose(matrix);

    println!("\nTransposta");
    for row in transposta{
        println!("{row:?}");
    }
}
