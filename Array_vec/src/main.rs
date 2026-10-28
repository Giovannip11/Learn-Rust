fn demostracao_array() {
    let notas: [f64; 5] = [9.5, 9.0, 8.0, 7.0, 6.0];

    let zeros: [i32; 10] = [0; 10];

    let primeira_nota = notas[0];
    println!("Primeira nota: {}", primeira_nota);

    for (i, nota) in notas.iter().enumerate() {
        println!("Aluno {}: {:.1}", i + 1, nota);
    }
    println!("Total de alunos: {}", notas.len());

    let tres_primeiras = &notas[0..3];
    println!("3 primeiras notas: {:?}", tres_primeiras);

    match notas {
        [primeiro, .., ultimo] => {
            println!("Primeira: {}, Última: {}", primeiro, ultimo);
        }
    }
    let soma: f64 = notas.iter().sum();
    let media = soma / notas.len() as f64;
    println!("Média da turma: {:.2}", media);

    println!("Zeros: {:?}", zeros);
}

fn main() {
    println!("Hello, world!");
}
