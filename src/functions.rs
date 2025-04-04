pub fn kramer(a: &Vec<Vec<f32>>, b: &Vec<f32>) -> (f32, f32, f32) {
    println!("=====================");
    println!("Метод Крамера:");
    println!("=====================");
    
    let det_main = det(&a);
    
    if det_main.abs() < f32::EPSILON {
        panic!("Система не має унікального розв'язку або має безліч розв'язків");
    }

    println!("\nОсновний детермінант: {}", det_main);
    println!("---------------------");

    let mut a_1 = a.clone();
    for i in 0..3 {
        a_1[i][0] = b[i];
    }
    let det_a_1 = det(&a_1);
    println!("\nМатриця з заміненим першим стовпцем на b:");
    for row in &a_1 {
        println!("{:?}", row);
    }
    println!("Детермінант для a_1: {}", det_a_1);
    println!("---------------------");

    let mut a_2 = a.clone();
    for i in 0..3 {
        a_2[i][1] = b[i];
    }
    let det_a_2 = det(&a_2);
    println!("\nМатриця з заміненим другим стовпцем на b:");
    for row in &a_2 {
        println!("{:?}", row);
    }
    println!("Детермінант для a_2: {}", det_a_2);
    println!("---------------------");

    let mut a_3 = a.clone();
    for i in 0..3 {
        a_3[i][2] = b[i];
    }
    let det_a_3 = det(&a_3);
    println!("\nМатриця з заміненим третім стовпцем на b:");
    for row in &a_3 {
        println!("{:?}", row);
    }
    println!("Детермінант для a_3: {}", det_a_3);
    println!("---------------------");

    let x1 = det_a_1 / det_main;
    let x2 = det_a_2 / det_main;
    let x3 = det_a_3 / det_main;

    println!("\nЗнайдені розв'язки:");
    println!("x1 = {}", x1);
    println!("x2 = {}", x2);
    println!("x3 = {}", x3);
    println!("=====================");
    (x1, x2, x3)
}

pub fn obern_matr(a: &Vec<Vec<f32>>, b: &Vec<f32>) -> (f32, f32, f32) {
    println!("=====================");
    println!("Метод оберненої матриці:");
    println!("=====================");

    let det_main = det(a);

    if det_main.abs() < f32::EPSILON {
        panic!("Система не має унікального розв'язку (детермінант дорівнює нулю)");
    }

    println!("\nДетермінант основної матриці: {}", det_main);
    println!("---------------------");

    let mut cofactors = vec![vec![0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
            cofactors[i][j] = sign * minor(a, i, j);
        }
    }

    println!("\nМатриця алгебраїчних доповнень:");
    for row in &cofactors {
        println!("{:?}", row);
    }
    println!("---------------------");

    let mut adjugate = vec![vec![0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            adjugate[i][j] = cofactors[j][i];
        }
    }

    println!("\nТранспонована матриця алгебраїчних доповнень:");
    for row in &adjugate {
        println!("{:?}", row);
    }
    println!("---------------------");

    let mut inverse = vec![vec![0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            inverse[i][j] = adjugate[i][j] / det_main;
        }
    }

    println!("\nОбернена матриця:");
    for row in &inverse {
        println!("{:?}", row);
    }
    println!("---------------------");

    let mut result = vec![0.0; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[i] += inverse[i][j] * b[j];
        }
    }

    println!("\nЗнайдені розв'язки:");
    println!("x1 = {}", result[0]);
    println!("x2 = {}", result[1]);
    println!("x3 = {}", result[2]);
    println!("=====================");
    (result[0], result[1], result[2])
}

fn minor(a: &Vec<Vec<f32>>, row: usize, col: usize) -> f32 {
    let mut m = Vec::new();

    for i in 0..3 {
        if i == row { 
            continue; 
        }
        let mut row_vals = Vec::new();
        for j in 0..3 {
            if j == col { 
                continue; 
            }
            row_vals.push(a[i][j]);
        }
        m.push(row_vals);
    }

    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

pub fn det(a: &Vec<Vec<f32>>) -> f32 {
    let det_value = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
        - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);

    det_value
}
