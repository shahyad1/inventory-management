use std::io;
use colored::Colorize;
enum ItemStatus {
    InStock,
    OutOfStock,
    Discontinued,
}

struct Item {
    name: String,
    quantity: i32,
    price: f64,
    status: ItemStatus,
}

impl Item {
 
    fn new(name: &str, quantity: i32, price: f64) -> Item {
        let status = if quantity > 0 {
            ItemStatus::InStock
        } else {
            ItemStatus::OutOfStock
        };
        Item {
            name: name.to_string(),
            quantity,
            price,
            status
        }
    }
    fn remove(&mut self) {
        self.status = ItemStatus::Discontinued;
        self.quantity = 0;
        println!("{} has been removed!", self.name.red().bold());
    }

    fn update_quantity(&mut self, quantity: i32) {
        self.quantity = quantity;
        self.status = if self.quantity > 0 {
            ItemStatus::InStock
        } else {
            ItemStatus::OutOfStock
        };
    }
    fn uapdate_price(&mut self, price: f64){
        self.price=price;
    }


    fn sell(&mut self, amount: i32) {
        if amount > 0 && self.quantity >= amount {
            self.quantity -= amount;
            self.status = if self.quantity > 0 {
                ItemStatus::InStock
            } else {
                ItemStatus::OutOfStock
            };
            println!("{} {} sold", amount, self.name);
        } else {
            println!("{}","Not enough".red().bold());
        }
    }


    fn display_info(&self) {
        let status = match self.status {
            ItemStatus::InStock => "In Stock".green(),
            ItemStatus::OutOfStock => "Out of Stock".red(),
            ItemStatus::Discontinued => "Discontinued".red(),
        };
        println!("Name : {} , Quantity : {} , Price : {} , Status : {} ", self.name, self.quantity, self.price, status);
    }


    fn search_item(&self, query: &str) -> bool {
        let words: Vec<&str> = self.name.split_whitespace().collect();
        for i in words {
            if i == query {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!();
    println!("                        ~~~~~~~~~~~~~~~~~~~~~~~~>{}<~~~~~~~~~~~~~~~~~~~~~~~~","Welcome to our store!".white().on_blue().bold());
    let mut save: Vec<Item> = Vec::new();

    loop {
        println!();
        println!("{}","Select a number from the list below:".green());
        println!("{}","1.Add product".yellow());
        println!("{}","2.Remove product".yellow());
        println!("{}","3.Product quantity update".yellow());
        println!("{}","4.Product price update".yellow());
        println!("{}","5.Sales of products and inventory reduction".yellow());
        println!("{}","6.Display product information".yellow());
        println!("{}","7.Search and filter products".yellow());
        println!("{}","8.Exit".red());

        let mut my_str = String::new();
        io::stdin().read_line(&mut my_str).unwrap();
        let n: i32 = my_str.trim().parse().unwrap();

        match n {

            1=> {
                println!("{}","enter your name product:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let name = my_str.trim();

                println!("{}","enter your quantity product:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let quantity: i32 = my_str.trim().parse().unwrap();

                println!("{}","enter your price product:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let price: f64 = my_str.trim().parse().unwrap();

                let item = Item::new(name, quantity, price);
                save.push(item);
            }
            2=> {
                println!("{}","enter product name to remove:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let name = my_str.trim();

                let mut found = false;
                for i in save.iter_mut() {
                    if i.name == name {
                        i.remove();
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("{}","Product not found!".red().bold());
                }
            }


            3=> {
                println!("{}","enter your name product to update:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let name = my_str.trim();

                let mut found = false;
                for i in save.iter_mut() {
                    if i.name == name {
                        println!("{}","enter your new quantity product to update:".white());
                        let mut my_str = String::new();
                        io::stdin().read_line(&mut my_str).unwrap();
                        let quantity: i32 = my_str.trim().parse().unwrap();

                        i.update_quantity(quantity);
                        println!("{}","Quantity updated!".green().bold());
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("{}","Product not found!".red().bold());
                }
            }
            4=>{
                println!("{}","enter your name product to update:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let name = my_str.trim();

                let mut found = false;
                for i in save.iter_mut() {
                    if i.name == name {
                        println!("{}","enter your new price product to update:".white());
                        let mut my_str = String::new();
                        io::stdin().read_line(&mut my_str).unwrap();
                        let price: f64 = my_str.trim().parse().unwrap();

                        i.uapdate_price(price);
                        println!("{}","Price updated!".green().bold());
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("{}","Product not found!".red().bold());
                }
            }

            5=> {
                println!("{}","enter product name:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let name = my_str.trim();

                let mut found = false;
                for i in save.iter_mut() {
                    if i.name == name {
                        println!("{}","enter quantity to sell:".white());
                        let mut my_str = String::new();
                        io::stdin().read_line(&mut my_str).unwrap();
                        let quantity: i32 = my_str.trim().parse().unwrap();
                        i.sell(quantity);
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("{}","Product not found".red().bold());
                }
            }

             6=>  {
                if save.is_empty() {
                    println!("{}","No products!".red().bold());
                } else {
                    for i in &save {
                        i.display_info();
                    }
                }
            }

            7=> {
                println!("{}","enter search item:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let search = my_str.trim();

                let mut found = false;
                for i in save.iter() {
                    if i.search_item(search) {
                        i.display_info();
                        found = true;
                    }
                }
                if !found {
                    println!("{}","Product not found!".red().bold());
                }
            }

            8=> {
                println!("                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~>{}<~~~~~~~~~~~~~~~~~~~~~~~~~~~","have good time!".white().on_blue().bold());
                break;
            }
            _ => println!("{}","Wrong!!".red().bold()),
        }
    }
}