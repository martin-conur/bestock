#[allow(dead_code)]

// this first implementation is a rough/naive one
// it doesnt forecast, but gives a 'suggested' value
// for each sku, store 'Combination'

pub struct Combination {
    pub sku: u32,
    pub store_id: u32,
    pub min_exhibition: u32,
    pub historic_sales: Vec<u32>,
    pub out_stock: Vec<bool>,
    pub anomaly_event: Vec<bool>, // like cyber or other promotions
}

fn mean(vect: &[u32]) -> f32 {
    let vect_length = vect.len() as u32;
    let vect_sum = vect.iter().sum::<u32>();

    vect_sum as f32 / vect_length as f32
}

fn std(vect: &Vec<u32>, n: u8) -> f32 {
    let vect_length = vect.len() as u32;
    let vect_mean = mean(vect);

    let variance = vect
        .iter()
        .fold(0.0, |acc, v| (vect_mean - *v as f32).powi(2) + acc);

    let variance = variance / (vect_length - 1) as f32;

    variance.sqrt() * n as f32
}

pub fn mask_vect(vect: &Vec<u32>, mask: &Vec<bool>) -> Vec<u32> {
    // this implementation is naive and not optimal
    // because its passes 2 times on the same vector
    // first one, for mean and std calculation
    // and the second one, for imputation depending on vector value
    // when out of stock.

    // 1st pass
    let masked_vector = vect
        .iter()
        .zip(mask.iter())
        .filter_map(|(v, b)| if !*b { Some(*v) } else { None })
        .collect::<Vec<u32>>();

    let mean_masked_vector = mean(&masked_vector);
    let std_masked_vector = std(&masked_vector, 1);

    // 2nd pass
    let updated_masked_vector = vect
        .iter()
        .zip(mask.iter())
        .filter_map(|(v, b)| match (*v, *b) {
            (nv, false) => Some(nv),
            (nv, true) if nv < mean_masked_vector.round() as u32 => {
                Some(mean_masked_vector.round() as u32)
            }
            (nv, true) if nv >= mean_masked_vector.round() as u32 => {
                Some((mean_masked_vector + std_masked_vector).round() as u32)
            }
            (_, true) => None,
        })
        .collect();
    //println!("{:?}", updated_masked_vector);

    updated_masked_vector
}

pub fn suggested_demand(masked_sales: &Vec<u32>) -> u32 {
    let mean_sales = mean(masked_sales);
    let std_sales = std(masked_sales, 1);

    (mean_sales + std_sales).ceil() as u32
}
