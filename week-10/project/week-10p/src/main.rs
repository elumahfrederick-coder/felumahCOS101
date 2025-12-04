struct LaptopStock {
    hp_price: u32,
    ibm_price: u32,
    toshiba_price: u32,
    dell_price: u32,
}

impl LaptopStock {
    fn total_cost_for_three_each(&self) -> u32 {
        3 * (self.hp_price + self.ibm_price + self.toshiba_price + self.dell_price)
    }
}

fn main() {
    let stock = LaptopStock {
        hp_price: 650_000,
        ibm_price: 755_000,
        toshiba_price: 555_000,
        dell_price: 850_000,
    };

    let total = stock.total_cost_for_three_each();
    println!("Total cost for buying 3 of each brand is: {}", total);
}

