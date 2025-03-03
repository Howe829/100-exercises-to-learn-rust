// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        Order::product_name_checker(&product_name);
        Order::quantity_checker(quantity);
        Order::unit_price_checker(unit_price);
        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    fn product_name_checker(product_name: &str) {
        if product_name.len() == 0 {
            panic!("ProductName cannot be empty");
        } else if product_name.len() > 300 {
            panic!("ProductName cannot be larger than 300 bytes");
        }
    }
    fn quantity_checker(quantity: u32) {
        if quantity <= 0 {
            panic!("Quantity must greater than zero");
        }
    }
    fn unit_price_checker(unit_price: u32) {
        if unit_price <= 0 {
            panic!("UnitPrice must greater than zero");
        }
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
    
    pub fn set_product_name(&mut self, title:String){
        self.product_name = title
    }
    
    pub fn set_quantity(&mut self, quantity: u32){
        self.quantity = quantity
    }
    
    pub fn set_unit_price(& mut self, unit_price:u32){
        self.unit_price = unit_price
    }
}
