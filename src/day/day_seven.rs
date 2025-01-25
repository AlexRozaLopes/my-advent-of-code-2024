use std::collections::HashMap;

pub fn bridge_repair(input: &str) -> usize {
    let equations= create_equation(input);
    let valid_equations: Vec<usize> = valid_equations(equations);
    valid_equations.iter().sum()
}

fn valid_equations(input: HashMap<usize, Vec<usize>>) -> Vec<usize> {
    input.iter().filter_map(|(key,value)| {
        if key.clone() == value.iter().sum::<usize>() || key.clone() == value.clone().iter().fold(1, |acc, v| acc * v) || sum_and_mult(key.clone(),value.clone()) {
            println!("key: {}, value: {:?}", key, value);
            Some(key.clone())
        }else {
            None
        }
    }).collect()
}

fn sum_and_mult(result: usize, numbers: Vec<usize>) -> bool {
    let n = numbers.len();  // Tamanho da sequência (não subtrair 1 aqui)
    let simbolos = ['+', '*'];

    let mut combinacoes = Vec::new();

    // Gerar todas as combinações de operadores
    for i in 0..(1 << (n - 1)) { // 1 << (n-1) gera 2^(n-1) combinações
        let mut combinacao = Vec::new();
        for j in 0..(n - 1) { // Usamos n-1 aqui porque estamos colocando n-1 operadores
            let bit = (i >> j) & 1; // Extrair o bit correspondente
            combinacao.push(simbolos[bit]);
        }
        println!("{:?}",combinacao);
        combinacoes.push(combinacao);
    }

    // Testar cada combinação de operadores
    for combinacao in combinacoes {
        let mut resultado = numbers[0];

        // Iterar sobre os operadores na combinação
        for (i, &op) in combinacao.iter().enumerate() {
            let num = numbers[i + 1];
            resultado = match op {
                '+' => resultado + num,
                '*' => resultado * num,
                _ => resultado, // Este ramo nunca será atingido devido ao conjunto de operadores definidos
            };
        }

        // Se o resultado for igual ao esperado, retornamos true
        if resultado == result {
            return true;
        }
    }

    false
}

fn create_equation(input: &str) -> HashMap<usize,Vec<usize>> {
    let mut map:HashMap<usize,Vec<usize>> = HashMap::new();
    input.lines().for_each(|line| {
        let vec = line.split(':').collect::<Vec<&str>>();
        map.insert(vec[0].parse::<usize>().unwrap(), vec[1].split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect());
    });

    println!("{:?}", map);
    map
}

#[test]
pub fn test_bridge_repair() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    assert_eq!(bridge_repair(&input), 3749);
}