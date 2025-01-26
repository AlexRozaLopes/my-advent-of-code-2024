use std::collections::HashMap;

pub fn bridge_repair(input: &str,is_part_two: bool) -> usize {
    let equations= create_equation(input);
    let valid_equations: Vec<usize> = valid_equations(equations,is_part_two);
    valid_equations.iter().sum()
}

fn valid_equations(input: HashMap<usize, Vec<usize>>,is_part_two: bool) -> Vec<usize> {
    input.iter().filter_map(|(key,value)| {
        if key.clone() == value.iter().sum::<usize>() || key.clone() == value.clone().iter().fold(1, |acc, v| acc * v) || sum_and_mult(key.clone(),value.clone(),is_part_two) {
            Some(key.clone())
        }else {
            None
        }
    }).collect()
}

fn sum_and_mult(result: usize, numbers: Vec<usize>, is_part_two: bool) -> bool {
    let n = numbers.len();  // Tamanho da sequência (não subtrair 1 aqui)
    let mut simbolos = vec!['+', '*'];  // Operadores básicos
    if is_part_two {
        simbolos.push('|');  // Adiciona o operador '|' se for a parte dois
    }

    let mut combinacoes = Vec::new();

    // Gerar todas as combinações de operadores com repetição
    let num_operadores = simbolos.len();  // Agora temos 2 ou 3 operadores
    for i in 0..(num_operadores.pow((n - 1) as u32) as usize) { // num_operadores^(n-1) combinações
        let mut combinacao = Vec::new();
        for j in 0..(n - 1) { // Usamos n-1 aqui porque estamos colocando n-1 operadores
            let op = simbolos[(i / num_operadores.pow(j as u32) % num_operadores) as usize]; // Calcula o operador
            combinacao.push(op);
        }
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
                '|' => {
                    // Concatenar os números como strings e depois converter de volta para usize
                    let concatenado = format!("{}{}", resultado, num);
                    concatenado.parse::<usize>().unwrap() // Converte para usize após a concatenação
                },
                _ => unreachable!(),  // Este ramo nunca será atingido devido ao conjunto de operadores definidos
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

    assert_eq!(bridge_repair(&input,false), 3749);
}

#[test]
pub fn test_bridge_repair_part_two() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    assert_eq!(bridge_repair(&input,true), 11387);
}