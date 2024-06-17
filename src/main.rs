mod utils;

use utils::{mask_vect, suggested_demand, Combination};

fn main() {
    let sku1_store1 = Combination {
        sku: 1,
        store_id: 1,
        min_exhibition: 2,
        historic_sales: vec![1, 0, 0, 1, 0, 3, 5, 0, 1, 0, 0, 0, 2],
        out_stock: vec![
            true, true, true, true, true, true, false, true, false, false, false, false,
            false,
        ],
        anomaly_event: vec![false, false, false, false, false, false],
    };

    for i in 0..10_000_000 {
    // mask historic sales without stock
        let valid_sales = mask_vect(&sku1_store1.historic_sales, &sku1_store1.out_stock);

        let suggested_demand1 = suggested_demand(&valid_sales);
        
        if i % 100_000 == 0 {
            //println!("suggested demmand: {suggested_demand1}");
            println!("{i}");
        }
    }
}
