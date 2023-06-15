#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod food_delivery {
    use ink::prelude::{
        string::String,
        vec::Vec,
    };
    use ink::storage::Mapping;

    pub type FoodId = u64;
    pub type OrderId = u64;
    pub type DeliveryId = u64;
    pub type CustomerId = u64;
    pub type RestaurantId = u64;
    pub type DeliverId = u64;

    #[ink(event)]
    pub struct SubmitOrderEvent {
        order_id: OrderId,
        food_id: FoodId,
        restaurant_id: RestaurantId,
        customer_id: CustomerId,
        delivery_address: String,
        phone_number: String,
    }

    #[ink(event)]
    pub struct AcceptDeliveryEvent {
        delivery_id: DeliveryId,
        order_id: OrderId,
    }

    #[ink(event)]
    pub struct AddFoodEvent {
        food_id: FoodId,
        food_name: String,
        restaurant_id: RestaurantId,
        description: String,
        price: Balance,
        eta: u64,
    }

    #[ink(event)]
    pub struct UpdateFoodEvent {
        food_id: FoodId,
        food_name: String,
        description: String,
        price: Balance,
        eta: u64,
    }

    #[ink(event)]
    pub struct ConfirmOrderEvent {
        order_id: OrderId,
        eta: u64,
    }

    #[ink(event)]
    pub struct DeliverOrderEvent {
        order_id: OrderId,
        restaurant_id: RestaurantId,
        customer_id: CustomerId,
        delivery_address: String,
    }

    #[ink(event)]
    pub struct PickupDeliveryEvent {
        delivery_id: DeliveryId,
    }

    #[ink(event)]
    pub struct AddDeliverEvent {
        deliver_id: DeliverId,
        deliver_name: String,
        deliver_address: String,
        phone_number: String,
    }

    #[ink(event)]
    pub struct AddRestaurantEvent {
        restaurant_id: RestaurantId,
        restaurant_name: String,
        restaurant_address: String,
        phone_number: String,
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum OrderStatus {
        OrderSubmitted,
        OrderConfirmed,
        WaitingDeliver,
        OrderDelivered,
        DeliveryAcceptted,
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum DeliveryStatus {
        Waiting,
        PickUp,
        Acceptted,
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Customer {
        pub customer_account: AccountId,
        pub customer_name: String,
        pub customer_address: String,
        pub phone_number: String,
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Restaurant {
        pub restaurant_account: AccountId,
        pub restaurant_name: String,
        pub restaurant_address: String,
        pub phone_number: String,
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Deliver {
        pub deliver_account: AccountId,
        pub deliver_name: String,
        pub deliver_address: String,
        pub phone_number: String,
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Food {
        pub food_name: String,
        pub restaurant_id: RestaurantId,
        pub description: String,
        pub price: Balance,
        pub eta: u64,
        pub timestamp: Timestamp,
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Order {
        pub food_id: FoodId,
        pub restaurant_id: RestaurantId,
        pub customer_id: CustomerId,
        pub deliver_id: DeliveryId,
        pub delivery_address: String,
        pub status: OrderStatus,
        pub timestamp: Timestamp,
        pub price: Balance,
        pub eta: u64,
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Delivery {
        pub order_id: OrderId,
        pub restaurant_id: RestaurantId,
        pub customer_id: CustomerId,
        pub deliver_id: DeliverId,
        pub delivery_address: String,
        pub status: DeliveryStatus,
        pub timestamp: Timestamp,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct FoodDelivery {
        pub manager: AccountId,
        pub food_id: u64,
        pub order_id: u64,
        pub delivery_id: u64,
        pub customer_id: u64,
        pub restaurant_id: u64,
        pub deliver_id: u64,
        pub customers: Mapping<CustomerId, Customer>,
        pub restaurants: Mapping<RestaurantId, Restaurant>,
        pub delivers: Mapping<DeliverId, Deliver>,
        pub food_data: Mapping<FoodId, Food>,
        pub order_data: Mapping<OrderId, Order>,
        pub delivery_data: Mapping<DeliveryId, Delivery>,
        pub restaurant_food_data: Mapping<RestaurantId, Vec<FoodId>>,
        pub restaurant_order_data: Mapping<RestaurantId, Vec<OrderId>>,
        pub restaurant_delivery_data: Mapping<RestaurantId, Vec<DeliveryId>>,
        pub customer_order_data: Mapping<CustomerId, Vec<OrderId>>,
        pub customer_delivery_data: Mapping<CustomerId, Vec<DeliveryId>>,
        pub deliver_delivery_data: Mapping<DeliverId, Vec<DeliveryId>>,
        pub customer_whitelist: Vec<AccountId>,
        pub restaurant_whitelist: Vec<AccountId>,
        pub deliver_whitelist: Vec<AccountId>,
        pub customer_account_id: Mapping<AccountId, CustomerId>,
        pub restaurant_account_id: Mapping<AccountId, RestaurantId>,
        pub deliver_account_id: Mapping<AccountId, DeliverId>,
    }

    impl FoodDelivery {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                manager: Self::env().caller(),
                food_id: 1,
                order_id: 1,
                deliver_id: 1,
                customer_id: 1,
                restaurant_id: 1,
                delivery_id: 1,
                customers: Mapping::default(),
                restaurants: Mapping::default(),
                delivers: Mapping::default(),
                food_data: Mapping::default(),
                order_data: Mapping::default(),
                delivery_data: Mapping::default(),
                restaurant_food_data: Mapping::default(),
                restaurant_order_data: Mapping::default(),
                restaurant_delivery_data: Mapping::default(),
                customer_order_data: Mapping::default(),
                customer_delivery_data: Mapping::default(),
                deliver_delivery_data: Mapping::default(),
                customer_whitelist: Vec::new(),
                restaurant_whitelist: Vec::new(),
                deliver_whitelist: Vec::new(),
                customer_account_id: Mapping::default(),
                restaurant_account_id: Mapping::default(),
                deliver_account_id: Mapping::default(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        // Customer's function.
        #[ink(message)]
        pub fn add_customer(
            &mut self,
            customer_name: String,
            customer_address: String,
            phone_number: String
        ) -> Customer {
            let customer_account = Self::env().caller();
            assert!(!self.customer_whitelist.contains(&customer_account), "alread exist customer!");
            let customer = Customer {
                customer_account,
                customer_name,
                customer_address,
                phone_number,
            };
            let customer_id = self.customer_id;
            self.customer_id += 1;
            self.customers.insert(&customer_id, &customer);
            self.customer_whitelist.push(customer_account);
            self.customer_account_id.insert(&customer_account, &customer_id);
            customer
        }
    
        #[ink(message)]
        pub fn submit_order(
            &mut self, 
            food_id: FoodId,
            restaurant_id: RestaurantId,
            delivery_address: String,
        ) -> Order {
            let customer_account = Self::env().caller();
            assert!(self.customer_whitelist.contains(&customer_account), "only customer can submit order!");
            let customer_id = self.customer_account_id.get(&customer_account).unwrap();
            let deliver_id = 0;
            let price = Self::env().transferred_value();
            assert!(self.food_data.get(&food_id).unwrap().price == price, "you must pay same of price!");
            let eta = 0;
            let timestamp = Self::env().block_timestamp();
            let status = OrderStatus::OrderSubmitted;
            let order = Order {
                food_id,
                restaurant_id,
                customer_id,
                deliver_id,
                delivery_address,
                status,
                timestamp,
                price,
                eta,
            };
            let order_id = self.order_id;
            self.order_id += 1;
            self.order_data.insert(&order_id, &order);
            let mut customer_vec = self.customer_order_data.get(&customer_id).unwrap_or(Vec::new());
            customer_vec.push(order_id);
            self.customer_order_data.insert(&customer_id, &customer_vec);
            let mut restaurant_vec = self.restaurant_order_data.get(&restaurant_id).unwrap_or(Vec::new());
            restaurant_vec.push(order_id);
            self.restaurant_order_data.insert(&restaurant_id, &restaurant_vec);
            let delivery_address = self.order_data.get(&order_id).unwrap().delivery_address;
            let customer_id = self.order_data.get(&order_id).unwrap().customer_id;
            let phone_number = self.customers.get(&customer_id).unwrap().phone_number;
            Self::env().emit_event(SubmitOrderEvent {
                order_id,
                food_id,
                restaurant_id,
                customer_id,
                delivery_address,
                phone_number,
            });
            order
        }
    
        #[ink(message)]
        pub fn confrim_delivery(
            &mut self,
            delivery_id: DeliveryId,
        ) -> bool {
            let customer_account = Self::env().caller();
            assert!(self.customer_whitelist.contains(&customer_account), "only customer can submit order!");
            let customer_id = self.customer_account_id.get(&customer_account).unwrap();
            let order_id = self.delivery_data.get(&delivery_id).unwrap().order_id;
            assert!(self.order_data.get(&order_id).unwrap().customer_id == customer_id, "not customer of this order!");
            let mut order = self.order_data.get(&order_id).unwrap();
            let status = OrderStatus::DeliveryAcceptted;
            order.status = status;
            self.order_data.insert(&order_id, &order);
            let mut delivery = self.delivery_data.get(&delivery_id).unwrap();
            let delivery_status = DeliveryStatus::Acceptted;
            delivery.status = delivery_status;
            self.delivery_data.insert(&delivery_id, &delivery);
            let _price = self.order_data.get(&order_id).unwrap().price;
            Self::env().emit_event(AcceptDeliveryEvent {
                order_id,
                delivery_id,
            });
            
            let deliver_amount = _price / 10;
            let restaurant_amount = _price - deliver_amount;
            let deliver_account = self.delivers.get(&self.order_data.get(&order_id).unwrap().deliver_id).unwrap().deliver_account;
            let restaurant_account = self.restaurants.get(&self.order_data.get(&order_id).unwrap().restaurant_id).unwrap().restaurant_account;
            if Self::env().transfer(deliver_account, deliver_amount).is_err() {
                false
            } else {
                if Self::env().transfer(restaurant_account, restaurant_amount).is_err() {
                    false
                } else {
                    true
                }
            }
        }
        
        // Restaurant's function.
        #[ink(message)]
        pub fn add_food(
            &mut self,
            food_name: String,
            description: String,
            price: Balance,
            eta: u64,
        ) -> Food {
            let restaurant_account = Self::env().caller();
            let restaurant_id = self.restaurant_account_id.get(&restaurant_account).unwrap();
            assert!(self.restaurant_whitelist.contains(&restaurant_account), "Only restaurant can add food!");
            let food_id = self.food_id;
            self.food_id += 1;
            let food = Food {
                food_name,
                restaurant_id,
                description,
                price,
                eta,
                timestamp: Self::env().block_timestamp(),
            };
            self.food_data.insert(&food_id, &food);
            let mut food_vec = self.restaurant_food_data.get(&restaurant_id).unwrap_or(Vec::new());
            food_vec.push(food_id);
            self.restaurant_food_data.insert(&restaurant_id, &food_vec);
            let food_name = self.food_data.get(&food_id).unwrap().food_name;
            let description = self.food_data.get(&food_id).unwrap().description;
            Self::env().emit_event(AddFoodEvent {
                food_id,
                food_name,
                restaurant_id,
                description,
                price,
                eta,
            });
            food
        }
    
        #[ink(message)]
        pub fn update_food(
            &mut self,
            food_id: FoodId,
            food_name: String,
            description: String,
            price: Balance,
            eta: u64,
        ) -> Food {
            let restaurant_account = Self::env().caller();
            let restaurant_id = self.restaurant_account_id.get(&restaurant_account).unwrap();
            assert!(self.restaurant_whitelist.contains(&restaurant_account), "Only restaurant can update food!");
            assert!(self.food_data.get(&food_id).unwrap().restaurant_id == restaurant_id, "Not owner of this food!");
            let food = Food {
                food_name,
                restaurant_id,
                description,
                price,
                eta,
                timestamp: Self::env().block_timestamp(),
            };
            self.food_data.insert(&food_id, &food);
            let food_name = self.food_data.get(&food_id).unwrap().food_name;
            let description = self.food_data.get(&food_id).unwrap().description;
            Self::env().emit_event(UpdateFoodEvent {
                food_id,
                food_name,
                description,
                price,
                eta,
            });
            food
        }
    
        #[ink(message)]
        pub fn confirm_order(
            &mut self,
            order_id: OrderId,
        ) -> bool {
            assert!(self.order_data.contains(&order_id), "Order not exist!");
            let restaurant_account = Self::env().caller();
            let restaurant_id = self.restaurant_account_id.get(&restaurant_account).unwrap();
            let food_id = self.order_data.get(&order_id).unwrap().food_id;
            assert!(self.restaurant_whitelist.contains(&restaurant_account), "Only restaurant can confirm order!");
            assert!(self.food_data.get(&food_id).unwrap().restaurant_id == restaurant_id, "Not owner of this order!");
            let mut order = self.order_data.get(&order_id).unwrap();
            let status = OrderStatus::OrderConfirmed;
            order.status = status;
            let food_id = self.order_data.get(&order_id).unwrap().food_id;
            let eta = self.food_data.get(&food_id).unwrap().eta;
            order.eta = eta;
            self.order_data.insert(&order_id, &order);
            Self::env().emit_event(ConfirmOrderEvent {
                order_id,
                eta,
            });
            true
        }
    
        #[ink(message)]
        pub fn deliver_order(
            &mut self,
            order_id: OrderId,
        ) -> Delivery {
            assert!(self.order_data.contains(&order_id), "Order not exist!");
            let restaurant_account = Self::env().caller();
            let restaurant_id = self.restaurant_account_id.get(&restaurant_account).unwrap();
            let food_id = self.order_data.get(&order_id).unwrap().food_id;
            assert!(self.restaurant_whitelist.contains(&restaurant_account), "Only restaurant can confirm order!");
            assert!(self.food_data.get(&food_id).unwrap().restaurant_id == restaurant_id, "Not owner of this order!");
            let mut order = self.order_data.get(&order_id).unwrap();
            let status = OrderStatus::WaitingDeliver;
            order.status = status;
            self.order_data.insert(&order_id, &order);
            let delivery_id = self.delivery_id;
            self.delivery_id += 1;
            let restaurant_id = self.order_data.get(&order_id).unwrap().restaurant_id;
            let customer_id = self.order_data.get(&order_id).unwrap().customer_id;
            let deliver_id = 0;
            let delivery_address = self.order_data.get(&order_id).unwrap().delivery_address;
            let status = DeliveryStatus::Waiting;
            let timestamp = Self::env().block_timestamp();
            let delivery = Delivery {
                order_id,
                restaurant_id,
                customer_id,
                deliver_id,
                delivery_address,
                status,
                timestamp,
            };
            self.delivery_data.insert(&delivery_id, &delivery);
            let mut restaurant_delivery_vec = self.restaurant_delivery_data.get(&restaurant_id).unwrap_or(Vec::new());
            restaurant_delivery_vec.push(delivery_id);
            let mut customer_delivery_vec = self.customer_delivery_data.get(&customer_id).unwrap_or(Vec::new());
            customer_delivery_vec.push(delivery_id);
            let delivery_address = self.order_data.get(&order_id).unwrap().delivery_address;
            Self::env().emit_event(DeliverOrderEvent {
                order_id,
                restaurant_id,
                customer_id,
                delivery_address,
            });
            delivery
        }
        
        // Deliver's function.
        #[ink(message)]
        pub fn pickup_delivery(
            &mut self,
            delivery_id: DeliveryId,
        ) -> bool {
            let caller = Self::env().caller();
            assert!(self.deliver_whitelist.contains(&caller), "only deliver can confirm devliery");
            assert!(self.delivery_data.get(&delivery_id).unwrap().status == DeliveryStatus::Waiting, "this delivery is already picked up!");
            let mut delivery = self.delivery_data.get(&delivery_id).unwrap();
            let status = DeliveryStatus::PickUp;
            delivery.status = status;
            self.delivery_data.insert(&delivery_id, &delivery);
            let order_id = self.delivery_data.get(&delivery_id).unwrap().order_id;
            let order_status = OrderStatus::OrderDelivered;
            let mut order = self.order_data.get(&order_id).unwrap();
            order.status = order_status;
            self.order_data.insert(&order_id, &order);
            Self::env().emit_event(PickupDeliveryEvent{delivery_id});
            true
        }
        
        // Manager's function.
        #[ink(message)]
        pub fn add_restaurant(
            &mut self,
            restaurant_account: AccountId,
            restaurant_name: String,
            restaurant_address: String,
            phone_number: String,
        ) -> Restaurant {
            let caller = Self::env().caller();
            assert!(caller != self.manager, "Only manager can add restaurant!");
            assert!(!self.restaurant_whitelist.contains(&restaurant_account), "already exist restaurant!");
            let restaurant_id = self.restaurant_id;
            self.restaurant_id += 1;
            let restaurant = Restaurant {
                restaurant_account,
                restaurant_name,
                restaurant_address,
                phone_number,
            };
            self.restaurants.insert(&restaurant_id, &restaurant);
            self.restaurant_account_id.insert(&restaurant_account, &restaurant_id);
            self.restaurant_whitelist.push(restaurant_account);
            let restaurant_name = self.restaurants.get(&restaurant_id).unwrap().restaurant_name;
            let restaurant_address = self.restaurants.get(&restaurant_id).unwrap().restaurant_address;
            let phone_number = self.restaurants.get(&restaurant_id).unwrap().phone_number;
            Self::env().emit_event(AddRestaurantEvent {
                restaurant_id,
                restaurant_name,
                restaurant_address,
                phone_number,
            });
            restaurant
        }
    
        #[ink(message)]
        pub fn add_deliver(
            &mut self,
            deliver_account: AccountId,
            deliver_name: String,
            deliver_address: String,
            phone_number: String,
        ) -> Deliver {
            let caller = Self::env().caller();
            assert!(caller == self.manager, "Only manager can add deliver!");
            assert!(!self.deliver_whitelist.contains(&deliver_account), "already exist deliver!");
            let deliver_id = self.deliver_id;
            self.deliver_id += 1;
            let deliver = Deliver {
                deliver_account,
                deliver_name,
                deliver_address,
                phone_number,
            };
            self.delivers.insert(&deliver_id, &deliver);
            self.deliver_account_id.insert(&deliver_account, &deliver_id);
            self.deliver_whitelist.push(deliver_account);
            let deliver_name = self.delivers.get(&deliver_id).unwrap().deliver_name;
            let deliver_address = self.delivers.get(&deliver_id).unwrap().deliver_address;
            let phone_number = self.delivers.get(&deliver_id).unwrap().phone_number;
            Self::env().emit_event(AddDeliverEvent {
                deliver_id,
                deliver_name,
                deliver_address,
                phone_number,
            });
            deliver
        }
    
        #[ink(message)]
        pub fn change_manager(
            &mut self,
            new_account: AccountId,
        ) -> bool {
            let caller = Self::env().caller();
            assert!(caller == self.manager, "Only manager can add deliver!");
            self.manager = new_account;
            true
        }
    
        #[ink(message)]
        pub fn get_eta(&self, order_id: OrderId) -> u64 {
            assert!(self.order_data.contains(&order_id), "Order does not exist!");
            let timestamp = self.order_data.get(&order_id).unwrap().timestamp;
            let cur_timestamp = Self::env().block_timestamp();
            let order_eta = self.order_data.get(&order_id).unwrap().eta;
            let eta = order_eta - (cur_timestamp - timestamp);
            if eta > 0 {
                eta
            } else {
                0
            }
        }
    
        #[ink(message)]
        pub fn get_order_from_id(&self, order_id: OrderId) -> Order {
            assert!(self.order_data.contains(&order_id), "Order does not exist!");
            self.order_data.get(&order_id).unwrap()
        }
    
        #[ink(message)]
        pub fn get_order_from_restaurant(&self, restaurant_id: RestaurantId) -> Vec<Order> {
            assert!(self.restaurants.contains(&restaurant_id), "Restaurant does not exist!");
            let order_data = self.restaurant_order_data.get(&restaurant_id).unwrap_or(Vec::new());
            let mut order_vec = Vec::new();
            for i in order_data.iter() {
                order_vec.push(self.order_data.get(&i).unwrap());
            }
            order_vec       
        }
    
        #[ink(message)]
        pub fn get_order_from_customer(&self, customer_id: CustomerId) -> Vec<Order> {
            assert!(self.customers.contains(&customer_id), "Restaurant does not exist!");
            let order_data = self.customer_order_data.get(&customer_id).unwrap_or(Vec::new());
            let mut order_vec = Vec::new();
            for i in order_data.iter() {
                order_vec.push(self.order_data.get(&i).unwrap());
            }
            order_vec  
        }
    
        #[ink(message)]
        pub fn get_order_all(&self, from: u64, to: u64) -> Vec<Order> {
            let mut order_vec: Vec<Order> = Vec::new();
            if to < self.order_id {
                for i in from..to {
                    order_vec.push(self.order_data.get(&i).unwrap());
                }
            } else {
                for i in from..self.order_id {
                    order_vec.push(self.order_data.get(&i).unwrap());
                }
            }
            order_vec
        }
    
        #[ink(message)]
        pub fn get_food_from_id(&self, food_id: FoodId) -> Food {
            assert!(self.food_data.contains(&food_id), "Order does not exist!");
            self.food_data.get(&food_id).unwrap()
        }
    
        #[ink(message)]
        pub fn get_food_from_restaurant(&self, restaurant_id: RestaurantId) -> Vec<Food> {
            assert!(self.restaurants.contains(&restaurant_id), "Restaurant does not exist!");
            let food_data = self.restaurant_food_data.get(&restaurant_id).unwrap_or(Vec::new());
            let mut food_vec = Vec::new();
            for i in food_data.iter() {
                food_vec.push(self.food_data.get(&i).unwrap());
            }
            food_vec        
        }
    
        #[ink(message)]
        pub fn get_food_all(&self, from: u64, to: u64) -> Vec<Food> {
            let mut food_vec: Vec<Food> = Vec::new();
            if to < self.food_id {
                for i in from..to {
                    food_vec.push(self.food_data.get(&i).unwrap());
                }
            } else {
                for i in from..self.food_id {
                    food_vec.push(self.food_data.get(&i).unwrap());
                }
            }
            food_vec
        }
    
        #[ink(message)]
        pub fn get_delivery_from_id(&self, delivery_id: DeliveryId) -> Delivery {
            assert!(self.delivery_data.contains(&delivery_id), "Order does not exist!");
            self.delivery_data.get(&delivery_id).unwrap()
        }
    
        #[ink(message)]
        pub fn get_delivery_from_deliver(&self, deliver_id: DeliverId) -> Vec<Delivery> {
            assert!(self.delivers.contains(&deliver_id), "Deliver does not exist!");
            let delivery_data = self.deliver_delivery_data.get(&deliver_id).unwrap_or(Vec::new());
            let mut deliver_vec = Vec::new();
            for i in delivery_data.iter() {
                deliver_vec.push(self.delivery_data.get(&i).unwrap());
            }
            deliver_vec
        }

        #[ink(message)]
        pub fn get_delivery_from_restaurant(&self, restaurant_id: RestaurantId) -> Vec<Delivery> {
            assert!(self.restaurants.contains(&restaurant_id), "Restaurant does not exist!");
            let delivery_data = self.restaurant_delivery_data.get(&restaurant_id).unwrap_or(Vec::new());
            let mut deliver_vec = Vec::new();
            for i in delivery_data.iter() {
                deliver_vec.push(self.delivery_data.get(&i).unwrap());
            }
            deliver_vec     
        }

        #[ink(message)]
        pub fn get_delivery_from_customer(&self, customer_id: CustomerId) -> Vec<Delivery> {
            assert!(self.customers.contains(&customer_id), "Customer does not exist!");
            let delivery_data = self.customer_delivery_data.get(&customer_id).unwrap_or(Vec::new());
            let mut deliver_vec = Vec::new();
            for i in delivery_data.iter() {
                deliver_vec.push(self.delivery_data.get(&i).unwrap());
            }
            deliver_vec       
        }

        #[ink(message)]
        pub fn get_delivery_all(&self, from: u64, to: u64) -> Vec<Delivery> {
            let mut delivery_vec: Vec<Delivery> = Vec::new();
            if to < self.delivery_id {
                for i in from..to {
                    delivery_vec.push(self.delivery_data.get(&i).unwrap());
                }
            } else {
                for i in from..self.delivery_id {
                    delivery_vec.push(self.delivery_data.get(&i).unwrap());
                }
            }
            delivery_vec
        }
    }
}