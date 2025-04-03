mod functions;
fn main() {

    let a: Vec<Vec<f32>> = vec![
    vec![0.34, 0.71, 0.63],
    vec![0.71, -0.65, -0.18],
    vec![1.17, -2.35, 0.75],
    ];

    let b: Vec<f32> = vec![2.08, 0.17, 1.28];


    println!("Задана СЛАР:");
    for i in 0..3 {
        for j in 0..3 {
            print!("{} x{}  ", a[i][j],j+1);
        }
        println!("= {}", b[i]);
    }

    let (x1, x2, x3) = functions::kramer(&a, &b);
    println!("Корені системи: x1 = {}, x2 = {}, x3 = {}", x1, x2, x3);
    

}
