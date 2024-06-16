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

fn mean(vect: &Vec<u32>) -> f32 {
    let vect_length = vect.len() as u32;
    let vect_sum= vect.iter().sum::<u32>();

    vect_sum as f32 / vect_length as f32
}

fn std(vect: &Vec<u32>, n: u8) -> f32 {
    let vect_length = vect.len() as u32;
    let vect_mean = mean(vect);

    let variance = vect.iter().fold(
            0.0,
            |acc, v| (vect_mean - *v as f32 ).powi(2) + acc
        );
    
    let variance = variance / (vect_length - 1) as f32;

    variance.sqrt() * n as f32
}

fn mask_vect(vect: Vec<u32>, mask: Vec<bool>) -> Vec<u32> {
    vect.iter()
    .zip(mask.iter())
    .filter_map(|(v, b)| if !*b {Some(*v)} else {None})
    .collect()
}

fn suggested_demand(masked_sales: &Vec<u32>) -> u32 {
    let mean_sales = mean(masked_sales);
    let std_sales = std(masked_sales, 1);

    (mean_sales + std_sales).ceil() as u32
}

fn main() {
    
    let sku1_store1 = Combination{
        sku: 1,
        store_id: 1,
        min_exhibition: 2,
        historic_sales: vec![1, 0, 0, 1, 0, 3, 5, 0, 1, 0, 0, 0, 2],
        out_stock: vec![
            false, 
            false, 
            false, 
            false, 
            true, 
            false, 
            false, 
            false,
            false,
            false,
            false,
            false,
            false
        ],
        anomaly_event: vec![false, false, false, false, false, false],
    };

    // mask historic sales without stock
    let valid_sales= mask_vect(
        sku1_store1.historic_sales, 
        sku1_store1.out_stock
    );

    let mean_sales = mean(&valid_sales);
    let sqrt_sales = std(&valid_sales, 1); 
    let suggested_demand1 = suggested_demand(&valid_sales);

    println!("mean sales: {mean_sales}");
    println!("sqrt sales: {sqrt_sales}");
    println!("suggested demmand: {suggested_demand1}");

}
