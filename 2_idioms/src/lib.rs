use std::{borrow::Cow, collections::HashMap};

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub price: u32,
    pub name: String,
}
impl Product {
    pub fn new<S: Into<String>>(title: S, price: u32) -> Product {
        Product {
            name: title.into(),
            price,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Coin {
    One,
    Two,
    Five,
    Ten,
    Twenty,
    Fifty,
}

impl Coin {
    pub fn value(&self) -> u32 {
        match self {
            Coin::One => 1,
            Coin::Two => 2,
            Coin::Five => 5,
            Coin::Ten => 10,
            Coin::Twenty => 20,
            Coin::Fifty => 50,
        }
    }
}

// Coin and amount of such coins
pub struct Money {
    pub coins: HashMap<Coin, u32>,
}

impl Money {
    pub fn new() -> Money {
        Money {
            coins: HashMap::new(),
        }
    }

    pub fn calc_amount(&self) -> u32 {
        let mut sum = 0;
        for (key, am) in &self.coins {
            sum += key.value() * am
        }
        sum
    }
}

pub trait VendingMachine {
    type VError;

    fn give_change(&mut self, change: &u32) -> Result<(), VendingError>;

    fn give_product(&mut self, product: &Product) -> Result<(), Self::VError>;

    // give money back if purchase was cancelled
    fn money_back(amount: &Money);

    /// selling product to the customer
    ///
    /// # Arguments
    ///
    /// * `product` - The product selected by customer
    /// * `money` - Amount of money customer entered
    ///
    /// # Returns
    ///
    /// Error: no change or VM run out of such product or not enough money entered
    /// Ok (), when the change and the product were given out
    fn sell_product(&mut self, product: &Product, money: &Money) -> Result<(), VendingError>;

    // one type of coins can be loaded per operation
    fn load_change_supply(&mut self, coin: Coin, quantity: u32);

    // load specific product to the vending machine
    fn load_product(
        &mut self,
        product_kind_and_quantity: ProductKindAndQuantity,
    ) -> Result<(), Self::VError>;
}

// Product and quantity of this product in Vending Machine
pub struct ProductKindAndQuantity {
    product_kind: Product,
    quantity: usize,
}
impl ProductKindAndQuantity {
    pub fn new(kind: Product, quantity: usize) -> ProductKindAndQuantity {
        ProductKindAndQuantity {
            product_kind: kind,
            quantity,
        }
    }
}
pub struct VMSweets {
    title: String,
    // capacity of products is limited
    product_capacity: usize,
    change_supply: HashMap<Coin, u32>,
    products: Vec<ProductKindAndQuantity>, // Product and quantity of this product in Vending Machine
}

impl VMSweets {
    pub fn new<S: Into<String>>(title: S, product_capacity: usize) -> VMSweets {
        VMSweets {
            title: title.into(),
            product_capacity,
            change_supply: HashMap::new(),
            products: Vec::new(),
        }
    }

    pub fn have_product(&self, product: &Product) -> bool {
        self.products
            .iter()
            .any(|x| x.product_kind == *product && x.quantity > 0)
    }

    pub fn calc_change_supply_amount(&self) -> u32 {
        Money {
            coins: self.change_supply.clone(),
        }
        .calc_amount()
    }

    // Getters:
    pub fn get_product_capacity(&self) -> usize {
        self.product_capacity
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_change_supply(&mut self) -> &mut HashMap<Coin, u32> {
        &mut self.change_supply
    }

    pub fn get_loaded_products(&mut self) -> &mut Vec<ProductKindAndQuantity> {
        &mut self.products
    }

    // how much different items of all products loaded in total (in order to check if total capacity of VM is not exceeded)
    pub fn get_amount_of_loaded_products(&self) -> usize {
        let res = self
            .products
            .iter()
            .map(|x| {
                let &ProductKindAndQuantity {
                    product_kind: _,
                    quantity: qntt,
                } = &x;
                qntt
            })
            .sum();
        res
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum VendingError {
    ChangeSupplyIsNotEnough(Cow<'static, str>),
    RunOutOfProduct(Cow<'static, str>),
    NotEnoughCapacity,
    NotEnoughChange,
    NotEnoughMoney,
    Unknown,
}

impl VendingMachine for VMSweets {
    type VError = VendingError;

    fn give_change(&mut self, change: &u32) -> Result<(), VendingError> {
        if *change == 0 {
            return Ok(());
        }

        let mut remaining_change = *change;
        let mut coins_to_give = HashMap::new();

        // Sort coins in descending order to use largest coins first
        let coin_order = [
            Coin::Fifty,
            Coin::Twenty,
            Coin::Ten,
            Coin::Five,
            Coin::Two,
            Coin::One,
        ];

        // First pass: calculate if we can give change
        let mut temp_supply = self.change_supply.clone();

        for coin in coin_order.iter() {
            let coin_value = coin.value();

            if remaining_change >= coin_value {
                if let Some(&available_count) = temp_supply.get(coin) {
                    if available_count > 0 {
                        let max_coins_needed = remaining_change / coin_value;
                        let coins_to_use = available_count.min(max_coins_needed);

                        if coins_to_use > 0 {
                            coins_to_give.insert(*coin, coins_to_use);
                            *temp_supply.get_mut(coin).unwrap() -= coins_to_use;
                            remaining_change -= coin_value * coins_to_use;
                        }
                    }
                }
            }

            if remaining_change == 0 {
                break;
            }
        }

        if remaining_change > 0 {
            return Err(VendingError::ChangeSupplyIsNotEnough(
                "Not enough coins to give exact change".into(),
            ));
        }

        // Second pass: actually deduct coins from supply
        for (coin, count) in coins_to_give {
            *self.change_supply.get_mut(&coin).unwrap() -= count;
        }

        Ok(())
    }

    fn money_back(money: &Money) {
        println!(
            "Money back, amount: {}, please don't forget the money",
            money.calc_amount()
        )
    }

    fn sell_product(&mut self, product: &Product, money: &Money) -> Result<(), VendingError> {
        // if we have such product
        if self.have_product(product) {
            // if enough money entered
            if &product.price <= &money.calc_amount() {
                if &product.price == &money.calc_amount() {
                    // put each type of coins of customers `money` to it's place in change supply in VM
                    for (coin, amnt) in money.coins.clone() {
                        self.load_change_supply(coin, amnt);
                    }
                    // possible errors have already been processed, so we ignore the resulting value
                    let _ = self.give_product(product);
                    Ok(())
                }
                // we need to give change
                else {
                    // give change (if enough change supply, else - cancel purchase)
                    let change_to_give_out = &money.calc_amount() - &product.price;
                    let res = self.give_change(&change_to_give_out);
                    match res {
                        Ok(()) => {
                            // put each type of coins of customers `money` to it's place in change supply in VM
                            for (coin, amnt) in money.coins.clone() {
                                self.load_change_supply(coin, amnt);
                            }

                            // give product
                            let _ = self.give_product(product);
                            Ok(())
                        }
                        Err(_) => {
                            // cancel purchase due to lack of change in Vending Machine
                            VMSweets::money_back(money);
                            Err(VendingError::NotEnoughChange)
                        }
                    }
                }
            }
            // amount entered by customer is not enough.
            else {
                println!("we are giving money back, not enough money");
                VMSweets::money_back(money);
                Err(VendingError::NotEnoughMoney)
            }
        } else {
            Err(VendingError::RunOutOfProduct(
                format!("Product '{}' is out of stock", product.name).into(),
            ))
        }
    }

    fn load_change_supply(&mut self, coin: Coin, quantity: u32) {
        let change_supply: &mut HashMap<Coin, u32> = self.get_change_supply();

        change_supply
            .entry(coin)
            .and_modify(|counter| *counter += quantity)
            .or_insert(quantity);
    }

    fn load_product(
        &mut self,
        product_kind_and_quantity: ProductKindAndQuantity,
    ) -> Result<(), VendingError> {
        let ProductKindAndQuantity {
            product_kind,
            quantity,
        } = &product_kind_and_quantity;
        let capacity = self.get_product_capacity();

        let total_products: usize = self.products.iter().map(|pr| pr.quantity).sum();

        if total_products + quantity <= capacity {
            // find this product in vector of products. If it exists, increase it's quantity
            // if does not exists, push to the vec of products new type of product
            if let Some(existing_product) = self
                .products
                .iter_mut()
                .find(|pr| pr.product_kind == *product_kind)
            {
                existing_product.quantity += quantity;
            } else {
                // push new kind of product to the vector of products
                self.products.push(product_kind_and_quantity);
            }
            Ok(()) //add product
        } else {
            Err(VendingError::NotEnoughCapacity)
        }
    }

    fn give_product(&mut self, product: &Product) -> Result<(), VendingError> {
        if self.have_product(&product) {
            for prdct in self.products.iter_mut() {
                if prdct.product_kind == *product {
                    prdct.quantity -= 1
                }
            }
            Ok(())
        } else {
            Err(VendingError::NotEnoughCapacity)
        }
    }
}
