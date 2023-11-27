#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(price) = s
            .products
            .iter()
            .find(|(name, _)| name == &ele)
            .map(|(_, price)| *price)
        {
            self.items.push((ele, price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // Sort the items by price
        self.items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Initialize accumulators for total price and total discount
        let mut total_price = 0.0;

        // Calculate the total price
        for (_, price) in &self.items {
            total_price += *price;
        }

        // Calculate the number of "free" items
        let num_free_items = self.items.len() / 3;

        // Calculate the total discount by summing the cheapest items
        let total_discount: f32 = self.items[0..num_free_items]
            .iter()
            .map(|(_, price)| *price)
            .sum();

        // Calculate the discount factor
        let discount_factor = total_discount / total_price;

        // Generate the receipt with discounted prices
        self.receipt = self
            .items
            .iter()
            .map(|(_, price)| {
                let discounted_price = price - (price * discount_factor);
                (discounted_price * 100.0).round() / 100.0 // Round to 2 decimal places
            })
            .collect();

        // Sort the receipt by price
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.receipt.clone()
    }
}
