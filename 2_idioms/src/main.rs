use step_2::VendingMachine;
use step_2::*;
fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use step_2::Product;

    use super::*;
    use step_2::VMSweets;
    use step_2::VendingMachine;

    #[test]
    fn test_load_product() {
        let mut vms = VMSweets::new("EatAndGo", 100);
        let twix = &Product::new("Twix", 25);

        let block_of_twix = ProductKindAndQuantity::new(twix.clone(), 50);
        let second_block_of_twix = ProductKindAndQuantity::new(twix.clone(), 15);

        let _ = vms.load_product(block_of_twix);
        let _ = vms.load_product(second_block_of_twix);

        // check getter of total capacity
        let total_capacity = vms.get_product_capacity();
        assert_eq!(total_capacity, 100);

        assert_eq!(twix.name, "Twix");

        // we should have only one kind of products in our machine - Twix
        assert_eq!(vms.get_loaded_products().len(), 1);

        // since we loaded two blocks of Twix, total amount of items should be 65 (50 in first block and 15 in second)
        assert_eq!(vms.get_amount_of_loaded_products(), 65);
    }

    #[test]
    fn test_load_change_supply() {
        let mut vms = VMSweets::new("EatAndGo", 100);

        let _ = vms.load_change_supply(Coin::One, 34);
        let _ = vms.load_change_supply(Coin::One, 6);

        let _ = vms.load_change_supply(Coin::Ten, 22);
        let _ = vms.load_change_supply(Coin::Ten, 8);

        let change_supply = vms.get_change_supply();

        assert_eq!(change_supply.get(&Coin::One), Some(40).as_ref());
        assert_eq!(change_supply.get(&Coin::Ten), Some(30).as_ref());
    }

    #[test]
    // Customer is trying to buy Snickers, but it's out of stock
    fn test_run_out_of_product() {
        let mut vms = VMSweets::new("EatAndGo", 100);
        let twix = &Product::new("Twix", 25);
        let snickers = &Product::new("Snickers", 30);
        let block_of_twix = ProductKindAndQuantity::new(twix.clone(), 50);
        let empty_block_of_snickers = ProductKindAndQuantity::new(snickers.clone(), 0);
        let _ = vms.load_product(block_of_twix);
        let _ = vms.load_product(empty_block_of_snickers);

        // 2 coins (Twenty and Five) customer is entering into the VM
        let money: &mut Money = &mut Money::new();
        money.coins.insert(Coin::Twenty, 1);
        money.coins.insert(Coin::Ten, 1);

        let res = vms.sell_product(snickers, money);
        assert_eq!(
            res,
            Err(VendingError::RunOutOfProduct(std::borrow::Cow::Borrowed(
                "Product 'Snickers' is out of stock"
            )))
        );
    }

    #[test]
    // if customer entered exactly required amount of money
    fn test_sell_product_exact_amount() {
        // price of Twix - 25. Customer will give 20 and 5 in two coins.
        let mut vms = VMSweets::new("EatAndGo", 100);
        let twix = &Product::new("Twix", 25);
        let block_of_twix = ProductKindAndQuantity::new(twix.clone(), 50);
        let _ = vms.load_product(block_of_twix);

        // 2 coins (Twenty and Five) customer is entering into the VM
        let money: &mut Money = &mut Money::new();
        money.coins.insert(Coin::Twenty, 1);
        money.coins.insert(Coin::Five, 1);

        let res = vms.sell_product(twix, money);
        assert_eq!(res, Ok(()));

        //check if we now have only 49 Twixes in the VM after the purchase
        let total_products = vms.get_amount_of_loaded_products();
        assert_eq!(total_products, 49);
    }

    #[test]
    fn test_cancell_sell_product_due_to_no_change() {
        // price of Twix - 25. Customer will give 50 in one coin.
        // change supply in vending machine only 10 Ones, so purchase should be refused due to lack of change
        let mut vms = VMSweets::new("EatAndGo", 100);
        let twix = &Product::new("Twix", 25);
        let block_of_twix = ProductKindAndQuantity::new(twix.clone(), 50);
        let _ = vms.load_product(block_of_twix);

        let _ = vms.load_change_supply(Coin::One, 10);

        let money: &mut Money = &mut Money::new();
        money.coins.insert(Coin::Fifty, 1);

        let res = vms.sell_product(twix, money);
        println!("{:?}", res);
        assert_eq!(res, Err(VendingError::NotEnoughChange));

        // amount of Twixes is the same - 50
    }

    #[test]
    // sell product and give change
    fn test_sell_product_give_change() {
        // price of Twix - 25. Customer will give 50.
        let mut vms = VMSweets::new("EatAndGo", 100);
        let twix = &Product::new("Twix", 25);
        // load 50 Twixes in the VM
        let block_of_twix = ProductKindAndQuantity::new(twix.clone(), 50);
        let _ = vms.load_product(block_of_twix);

        // load change supply into the VM
        let _ = vms.load_change_supply(Coin::One, 10);
        let _ = vms.load_change_supply(Coin::Two, 10);
        let _ = vms.load_change_supply(Coin::Five, 10);
        let _ = vms.load_change_supply(Coin::Ten, 10);
        let _ = vms.load_change_supply(Coin::Twenty, 10);

        let initial_change_supply = vms.calc_change_supply_amount();

        // customer is entering into the VM one coin - 50
        let money: &mut Money = &mut Money::new();
        money.coins.insert(Coin::Fifty, 1);

        let res = vms.sell_product(twix, money);
        assert_eq!(res, Ok(()));

        // check updated change_supply in the VM (it should be 25 more)
        let new_change_supply = vms.calc_change_supply_amount();
        assert_eq!(new_change_supply - initial_change_supply, 25);

        //check if we now have only 49 Twixes in the VM after the purchase
        let total_products = vms.get_amount_of_loaded_products();
        assert_eq!(total_products, 49);
    }
}
