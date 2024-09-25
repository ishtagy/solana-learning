use anyhow::{anyhow, Result};
use std::io; // For error handling

#[derive(Debug, Clone)]
pub struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

#[derive(Debug)]
pub struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: Vec::new(),
        }
    }

    fn add(&mut self, product: &Product) {
        self.products.push(product.clone());
    }

    fn edit(&mut self, index: usize, new_product: Product) -> Result<(), String> {
        if index >= self.products.len() {
            return Err("Index out of bounds".to_string());
        }
        self.products[index] = new_product;
        Ok(())
    }

    fn delete(&mut self, index: usize) -> Result<(), String> {
        if index >= self.products.len() {
            return Err("Index out of bounds".to_string());
        }
        self.products.remove(index);
        Ok(())
    }

    fn list_products(&self) {
        println!("-----Inventory List-----");
        if self.products.is_empty() {
            println!("No products in the inventory.");
        } else {
            for (index, product) in self.products.iter().enumerate() {
                println!(
                    "{}. Product: {}, Description: {}, Price: {:.2}, Quantity: {}",
                    index + 1,
                    product.name,
                    product.description,
                    product.price,
                    product.quantity
                );
            }
        }
        println!("");
    }
}

#[derive(Debug)]
struct SalesTransaction {
    product_name: String,
    quantity: u32,
    sale_price: f64,
}

#[derive(Debug)]
pub struct Sales {
    transactions: Vec<SalesTransaction>,
    total_sales: f64,
    profit: f64,
}

impl Sales {
    fn new() -> Self {
        Sales {
            transactions: Vec::new(),
            total_sales: 0.0,
            profit: 0.0,
        }
    }

    fn record_transaction(
        &mut self,
        product: &mut Product,
        quantity: u32,
        sale_price: f64,
    ) -> Result<(), String> {
        if product.quantity < quantity {
            return Err("Not enough product quantity".to_string());
        }

        if quantity <= 0 {
            return Err("Quantity must be greater than zero".to_string());
        }

        let total_sales = sale_price * quantity as f64;
        let profit = total_sales - product.price * quantity as f64;
        product.quantity -= quantity;

        self.transactions.push(SalesTransaction {
            product_name: product.name.clone(),
            quantity: quantity,
            sale_price: sale_price,
        });
        self.total_sales += total_sales;
        self.profit += profit;

        println!(
            "The total cost of transaction is {}, the profit: {}",
            total_sales, profit
        );

        Ok(())
    }

    fn report(&self) {
        println!("-----Sales Report-----");
        println!("Total sales: {:.2}", self.total_sales);
        println!("Total profit: {:.2}", self.profit);
        println!("Sales transactions: ");
        for (index, transaction) in self.transactions.iter().enumerate() {
            println!(
                "{}. Product: {}, Quantity Sold: {}, Sale Price: {:.2}",
                index + 1,
                transaction.product_name,
                transaction.quantity,
                transaction.sale_price
            );
        }
        println!("");
    }
}

#[derive(Debug)]
struct PurchaseTransaction {
    product_name: String,
    quantity: u32,
    purchase_price: f64,
}

#[derive(Debug)]
pub struct Purchase {
    transactions: Vec<PurchaseTransaction>,
    total_cost: f64,
}

impl Purchase {
    fn new() -> Self {
        Purchase {
            transactions: Vec::new(),
            total_cost: 0.0,
        }
    }

    fn record_transaction(
        &mut self,
        product: &mut Product,
        quantity: u32,
        purchase_price: f64,
    ) -> Result<(), String> {
        if quantity <= 0 {
            return Err("Quantity must be greater than zero".to_string());
        }

        let total_cost = purchase_price * quantity as f64;
        self.total_cost += total_cost;
        product.quantity += quantity;

        self.transactions.push(PurchaseTransaction {
            product_name: product.name.clone(),
            quantity: quantity,
            purchase_price: purchase_price,
        });

        println!("The total cost of purhcase is: {:.2}", total_cost);

        Ok(())
    }

    fn report(&self) {
        println!("-----Purchase Report-----");
        println!("Total cost of Purchases: {:.2}", self.total_cost);
        println!("Purchase transactions: ");
        for (index, transaction) in self.transactions.iter().enumerate() {
            println!(
                "{}. Product: {}, Quantity Purchased: {}, Purchase Price: {:.2}",
                index + 1,
                transaction.product_name,
                transaction.quantity,
                transaction.purchase_price
            );
        }
        println!("");
    }
}

fn main() {
    let mut inventory = Inventory::new();
    let mut sales = Sales::new();
    let mut purchases = Purchase::new();

    //Some Data
    let apple = Product {
        name: String::from("Apple"),
        description: String::from("Green apples"),
        price: 1.1,
        quantity: 50,
    };
    let banana = Product {
        name: String::from("Banana"),
        description: String::from("Yellow"),
        price: 2.0,
        quantity: 70,
    };
    let mango = Product {
        name: String::from("Mango"),
        description: String::from("Tasty"),
        price: 5.0,
        quantity: 30,
    };

    for i in [apple, banana, mango] {
        inventory.add(&i);
    }

    println!("Welcome to the Inventory Management System!");

    // Prompt for password
    loop {
        let input_password =
            input_string("Enter password or type exit to quit the program(Password is 12345): ");
        if input_password == "exit".to_string() {
            return;
        }
        if login(input_password) {
            break;
        } else {
            println!("Incorrect password. Try again.");
        }
    }

    loop {
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Remove Product");
        println!("4. List Products");
        println!("5. Record Sale");
        println!("6. Record Purchase");
        println!("7. Generate Report");
        println!("8. Exit");
        println!("Select option: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        println!("");
        match choice.trim() {
            "1" => {
                let (name, description, price, quantity) = input_product_info();
                inventory.add(&Product {
                    name: name,
                    description: description,
                    price: price,
                    quantity: quantity,
                });
            }
            "2" => {
                let index = input_u32("Enter index of product you want to edit: ");
                if index == 0 {
                    println!("Index can't be zero\n");
                    continue;
                }
                let (name, description, price, quantity) = input_product_info();
                match inventory.edit(
                    (index - 1) as usize,
                    Product {
                        name: name,
                        description: description,
                        price: price,
                        quantity: quantity,
                    },
                ) {
                    Ok(_) => println!("Product edited successfully!"),
                    Err(e) => println!("{}", e),
                }
            }
            "3" => {
                let index = input_u32("Enter index of product you want to delete: ");
                if index == 0 {
                    println!("Index can't be zero\n");
                    continue;
                }
                match inventory.delete((index - 1) as usize) {
                    Ok(_) => println!("Product delete successfully"),
                    Err(e) => println!("{}", e),
                }
            }
            "4" => {
                inventory.list_products();
            }
            "5" => {
                let (index, quantity, sale_price) = input_sale_purchase_info();
                if index == 0 {
                    println!("Index can't be zero\n");
                    continue;
                }
                if index >= inventory.products.len() as u32 {
                    println!("Index out of bounds");
                    continue;
                }
                if let Err(e) = sales.record_transaction(
                    &mut inventory.products[(index - 1) as usize],
                    quantity,
                    sale_price,
                ) {
                    println!("{}", e);
                }
            }
            "6" => {
                let (index, quantity, sale_price) = input_sale_purchase_info();
                if index == 0 {
                    println!("Index can't be zero\n");
                    continue;
                }
                if index >= inventory.products.len() as u32 {
                    println!("Index out of bounds");
                    continue;
                }
                if let Err(e) = purchases.record_transaction(
                    &mut inventory.products[(index - 1) as usize],
                    quantity,
                    sale_price,
                ) {
                    println!("{}", e);
                }
            }

            "7" => {
                inventory.list_products();
                sales.report();
                purchases.report();
            }

            "8" => break,
            _ => println!("Invalid input. Enter number from 1-8"),
        }
    }
}

fn login(password: String) -> bool {
    let correct_password = "12345";
    password == correct_password
}

fn input_product_info() -> (String, String, f64, u32) {
    let name = input_string("Enter name: ");
    let description = input_string("Enter description: ");
    let price = input_float("Enter price: ");
    let quantity = input_u32("Enter quantity");
    (name, description, price, quantity)
}

fn input_sale_purchase_info() -> (u32, u32, f64) {
    let index = input_u32("Enter index of product");
    let quantity = input_u32("Enter quantity: ");
    let price = input_float("Enter price: ");
    (index, quantity, price)
}

fn parse_input<T: std::str::FromStr>(input: &str) -> Result<T> {
    input
        .trim()
        .parse::<T>()
        .map_err(|_| anyhow!("Invalid input"))
}

fn input_string(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn input_float(prompt: &str) -> f64 {
    loop {
        let input = input_string(prompt);
        match parse_input::<f64>(&input) {
            Ok(value) => return value,
            Err(e) => println!("{}", e),
        }
    }
}

fn input_u32(prompt: &str) -> u32 {
    loop {
        let input = input_string(prompt);
        match parse_input::<u32>(&input) {
            Ok(value) => return value,
            Err(e) => println!("{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_product() {
        let mut inventory = Inventory::new();
        let product = Product {
            name: String::from("Product"),
            description: String::from("Desc"),
            price: 10.0,
            quantity: 5,
        };
        inventory.add(&product);
        assert_eq!(inventory.products.len(), 1);
        assert_eq!(inventory.products[0].name, "Product");
    }

    #[test]
    fn test_edit_product() {
        let mut inventory = Inventory::new();
        let product = Product {
            name: String::from("Product"),
            description: String::from("Desc"),
            price: 10.0,
            quantity: 5,
        };

        inventory.add(&product);

        let new_product = Product {
            name: String::from("New Product"),
            description: String::from("new desc"),
            price: 20.0,
            quantity: 10,
        };

        assert!(inventory.edit(0, new_product.clone()).is_ok());
        assert_eq!(inventory.products[0].name, "New Product");
    }

    #[test]
    fn test_delete_product() {
        let mut inventory = Inventory::new();
        let product = Product {
            name: String::from("Product"),
            description: String::from("Description"),
            price: 10.0,
            quantity: 5,
        };
        inventory.add(&product);
        assert!(inventory.delete(0).is_ok());
        assert_eq!(inventory.products.len(), 0);
    }

    #[test]
    fn test_record_sale() {
        let mut inventory = Inventory::new();
        let mut sales = Sales::new();
        let mut product = Product {
            name: String::from("Product"),
            description: String::from("Description"),
            price: 10.0,
            quantity: 5,
        };
        inventory.add(&product);

        let result = sales.record_transaction(&mut product, 2, 15.0);
        assert!(result.is_ok());
        assert_eq!(sales.total_sales, 30.0);
        assert_eq!(sales.profit, 10.0);
        assert_eq!(product.quantity, 3);
    }

    #[test]
    fn test_record_purchase() {
        let mut inventory = Inventory::new();
        let mut purchases = Purchase::new();
        let mut product = Product {
            name: String::from("Product"),
            description: String::from("Description"),
            price: 10.0,
            quantity: 5,
        };
        inventory.add(&product);

        let result = purchases.record_transaction(&mut product, 3, 7.0);
        assert!(result.is_ok());
        assert_eq!(purchases.total_cost, 21.0);
        assert_eq!(product.quantity, 8); // 5 + 3 purchased
    }
}
