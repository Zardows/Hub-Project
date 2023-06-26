use std::env;

fn print_res(s: &str, cmd: &str, res: f64) {
    println!("Chances to get a {} {}: {:.2}%", s, cmd, res);
}


fn factorial(n: u64) -> f64 {
    let mut res = 1.0;
    for i in 2..=n {
        res *= i as f64;
    }
    res
}

fn coeff(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }

    let mut result: usize = 1;

    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }

    result
}

fn calc_full(first: &str, second: &str, dices: &[i32]) -> f64 {
    let mut keep_first = 5 - (5 - dices.iter().filter(|&x| *x == first.parse::<i32>().unwrap()).count());

    // Nombre de dés à garder pour la paire
    let mut keep_second = 5 - (5 - dices.iter().filter(|&x| *x == second.parse::<i32>().unwrap()).count());

    // Si on a déjà le full, le pourcentage de chance est de 100%
    if keep_first == 3 && keep_second == 2 {
        return 100.0;
    }

    // Si on a déjà au moins le brelan, on garde 3 dés
    if keep_first > 3 {
        keep_first = 3;
    }

    // Si on a déjà au moins la paire, on garde 2 dés
    if keep_second > 2 {
        keep_second = 2;
    }

    // On calcule le nombre de dés à relancer
    let nb = 5 - keep_first - keep_second;

    // Si on relance seulement 1 dé, la probabilité est simple à calculer
    if nb == 1 {
        return 1.0 / 6.0 * 100.0;
    }
    // Si on relance plus d'1 dé, on calcule les coefficients binomiaux pour la paire et le brelan
    else {


        let _pair = coeff(nb, 3 - keep_first);
        let _three = coeff(2 - keep_second, 2 - keep_second);
        let _res = (_pair * _three) as f64 / (6usize.pow(nb as u32) as f64) * 100.0;
        return _res
    }
}

fn calc_straight(_nb: &str, dices: &[i32]) -> f64 {
    let mut sorted_dices = dices.to_vec(); // Copie dices dans un nouveau vecteur
    sorted_dices.sort_by(|a, b| b.cmp(a));
    let mut _dice = 0;
    let mut  _nb_int = _nb.parse::<i32>().unwrap();
    let mut _x = 5;

    while _x > 0 {
        if sorted_dices.contains(&_nb_int) {
            _dice += 1;
        }
        _nb_int -= 1;
        _x -= 1;
    }
    let res = factorial(5 - _dice) / power(6 as f64, (5 - _dice) as usize) * 100.0;
    res
}

fn power(base: f64, exponent: usize) -> f64 {
    let mut result = 1.0;
    for _ in 0..exponent {
        result *= base;
    }
    result
}

fn binomial(n: usize, k: usize) -> f64 {
    let coefficient = coeff(n, k);
    let power1 = power(1.0 / 6.0, k);
     let power2 = power(1.0 - 1.0 / 6.0, n-k);
    
    let result = coefficient as f64 * power1 * power2;
    result
}

fn calc_proba(nb: &str, dices: &[i32], _arg: usize) -> f64 {
    let mut _keep = 5 - (5 - dices.iter().filter(|&&x| x == nb.parse::<i32>().unwrap()).count());
    let mut _res = 0.0;

    if _keep < _arg {
        for k in _arg - _keep..(6 - _keep) {
            _res += binomial(5 - _keep, k);
        }
        _res *= 100.0;
    } else {
        _res = 100.0;
    }

    _res
}


fn get_cmd(args: [&str; 7]) {
    let _exec: Vec<&str> = args[6].split('_').collect();

    let args: Vec<i32> = args.iter().enumerate()
        .filter(|&(i, _)| i != 0 && i != 6)
        .map(|(_, val)| val.parse().unwrap())
        .collect();

    if _exec[0] == "pair" {
        print_res(&_exec[1], "pair", calc_proba(_exec[1], &args, 2));
    }
    if _exec[0] == "three" {
        print_res(&_exec[1], "three-of-a-kind", calc_proba(_exec[1], &args, 3));
    }
    if _exec[0] == "four" {
        print_res(&_exec[1], "four-of-a-kind", calc_proba(_exec[1], &args, 4));
    }
    if _exec[0] == "yams" {
        print_res(&_exec[1], "yams", calc_proba(_exec[1], &args, 5));
    }
    if _exec[0] == "full" {
        print_res(&_exec[1], "full of", calc_full(_exec[1], _exec[2], &args));
    }
    if _exec[0] == "straight" {
        print_res(&_exec[1].to_string(), "straight", calc_straight(_exec[1], &args));
    }
}

fn main() {
    
    //--------------------------Si tu veux tester sur le playground de rust-------------------------------------------------
    let args = ["oui", "1", "2", "3","4", "5","straight_3"];
    
    //-----------------ce que t'es cense faire mais jsp si ca fonctionne-----------------------
    let args: Vec<String> = env::args().collect();

    get_cmd(args);
}
    

