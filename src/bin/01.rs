use anyhow::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01"; // Atualize para o dia correspondente
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1 3
2 3
3 3
3 4
3 5
4 9
"; // Entrada de teste

fn parse_input<R: BufRead>(reader: R) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Erro ao converter para i32"))
            .collect();
        if parts.len() == 2 {
            left_list.push(parts[0]);
            right_list.push(parts[1]);
        }
    }

    Ok((left_list, right_list))
}

fn calculate_total_distance(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>) -> i32 {
    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn calculate_similarity_score(left_list: &[i32], right_list: &[i32]) -> i32 {
    let mut right_count = HashMap::new();

    for &num in right_list {
        *right_count.entry(num).or_insert(0) += 1;
    }

    left_list
        .iter()
        .map(|&num| num * right_count.get(&num).unwrap_or(&0))
        .sum()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let (mut left_list, mut right_list) = parse_input(reader)?;
        Ok(calculate_total_distance(&mut left_list, &mut right_list))
    }

    // Validar com o input de teste
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    // Processar o arquivo de entrada real
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let (left_list, right_list) = parse_input(reader)?;
        Ok(calculate_similarity_score(&left_list, &right_list))
    }

    // Validar com o input de teste
    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    // Processar o arquivo de entrada real
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
