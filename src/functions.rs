pub fn kramer(a: &Vec<Vec<f32>>, b: &Vec<f32>) -> (f32, f32, f32) {
    println!("Метод Крамера:");
    
    let det_main = det(&a);
    
    if det_main.abs() < f32::EPSILON {
        panic!("Система не має унікального розв'язку або має безліч розв'язків");
    }

    println!("Основний детермінант: {}", det_main);

    let mut a_1 = a.clone();
    for i in 0..3 {
        a_1[i][0] = b[i];
    }
    let det_a_1 = det(&a_1);
    println!("\nМатриця з заміненим першим стовпцем на b: {:?}", a_1);
    println!("Детермінант для a_1: {}", det_a_1);

    let mut a_2 = a.clone();
    for i in 0..3 {
        a_2[i][1] = b[i];
    }
    let det_a_2 = det(&a_2);
    println!("\nМатриця з заміненим другим стовпцем на b: {:?}", a_2);
    println!("Детермінант для a_2: {}", det_a_2);

    let mut a_3 = a.clone();
    for i in 0..3 {
        a_3[i][2] = b[i];
    }
    let det_a_3 = det(&a_3);
    println!("\nМатриця з заміненим третім стовпцем на b: {:?}", a_3);
    println!("Детермінант для a_3: {}", det_a_3);

    let x1 = det_a_1 / det_main;
    let x2 = det_a_2 / det_main;
    let x3 = det_a_3 / det_main;

    println!("\nЗнайдені розв'язки:");
    println!("x1 = {}", x1);
    println!("x2 = {}", x2);
    println!("x3 = {}", x3);

    (x1, x2, x3)
}

pub fn obern_matr(a: &Vec<Vec<f32>>, b: &Vec<f32>) -> (f32, f32, f32) {

}



pub fn det(a: &Vec<Vec<f32>>) -> f32 {
    let det_value = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
        - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);

    det_value
}
