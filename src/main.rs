#[allow(dead_code)]

// this first implementation is a rough/naive one
// it doesnt forecast, but gives a 'suggested' value
// for each sku, store 'Combination'


struct Combination {
    sku: u32,
    store_id: u32,
    min_exhibition: u32,
    historic_sales: Vec<u32>,
    out_stock: Vec<bool>,
    anomaly_event: Vec<bool> // like cyber or other promotions
}

fn sum(vect: &Vec<u32>) ->u32 {
    let mut vect_sum: u32 = 0;

    for v in vect.iter() {
        vect_sum = vect_sum + v; 
    }
    vect_sum
}

fn mean(vect: &Vec<u32>) -> f32 {
    let vect_length = vect.len() as u32;
    let vect_sum= sum(&vect);

    vect_sum as f32 / vect_length as f32
}

fn std(vect: &Vec<u32>, n: u8) -> f32 {
    let vect_length = vect.len() as u32;
    let vect_mean = mean(vect);

    let variance = vect.into_iter().fold(
            0.0,
            |acc, v| (vect_mean - *v as f32 ).powi(2) + acc
        );
    
    let variance = variance / (vect_length - 1) as f32;

    variance.sqrt() * n as f32
}

fn main() {
    
    let sku1_store1 = Combination{
        sku: 1,
        store_id: 1,
        min_exhibition: 2,
        historic_sales: vec![1, 0, 0, 1, 0, 3],
        out_stock: vec![false, false, false, false, true, false],
        anomaly_event: vec![false, false, false, false, false, false],
    };

    // mask historic sales without stock
    let valid_sales: Vec<u32> = sku1_store1.historic_sales
        .iter()
        .zip(sku1_store1.out_stock.iter())
        .filter_map(|(v, b)| if !*b {Some(*v)} else {None})
        .collect();

    let mean_sales = mean(&valid_sales);

    let sqrt_sales = std(&valid_sales, 2); 
    println!("mean sales: {mean_sales}");
    println!("sqrt sales: {sqrt_sales}");

}
