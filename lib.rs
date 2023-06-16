#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod food_delivery {
    use ink::prelude::{
        string::String,
        vec::Vec,
    };
    use ink::storage::Mapping;

    // Define identifier type
    pub type FoodId = u64;
    pub type OrderId = u64;
    pub type DeliveryId = u64;
    pub type CustomerId = u64;
    pub type RestaurantId = u64;
    pub type CourierId = u64;

    // Event when a customer orders food.
    #[ink(event)]
    pub struct SubmitOrderEvent {
        order_id: OrderId,
        food_id: FoodId,
        restaurant_id: RestaurantId,
        customer_id: CustomerId,
        delivery_address: String,
        phone_number: String,
    }
    
    // Event when a customer confirms delivery.
    #[ink(event)]
    pub struct AcceptDeliveryEvent {
        delivery_id: DeliveryId,
        order_id: OrderId,
    }

    // Event when a restaurant adds food.
    #[ink(event)]
    pub struct AddFoodEvent {
        food_id: FoodId,
        food_name: String,
        restaurant_id: RestaurantId,
        description: String,
        price: Balance,
        eta: u64,
    }

    // Event when a restaurant updates food information.
    #[ink(event)]
    pub struct UpdateFoodEvent {
        food_id: FoodId,
        food_name: String,
        description: String,
        price: Balance,
        eta: u64,
    }
    
    // Event when a restaurant confirms order repuested customer.
    #[ink(event)]
    pub struct ConfirmOrderEvent {
        order_id: OrderId,
        eta: u64,
    }

    // Event when a restaurant requests the delivery.
    #[ink(event)]
    pub struct DeliverOrderEvent {
        order_id: OrderId,
        restaurant_id: RestaurantId,
        customer_id: CustomerId,
        delivery_address: String,
    }

    // Event when a courier picks up the delivery.
    #[ink(event)]
    pub struct PickupDeliveryEvent {
        delivery_id: DeliveryId,
    }

    // Event when a manager add new courier.
    #[ink(event)]
    pub struct AddCourierEvent {
        courier_id: CourierId,
        courier_name: String,
        courier_address: String,
        phone_number: String,
    }

    // Event when a manager add new restaurant.
    #[ink(event)]
    pub struct AddRestaurantEvent {
        restaurant_id: RestaurantId,
        restaurant_name: String,
        restaurant_address: String,
        phone_number: String,
    }

    // Enum that order status.
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

    // Enum that delivery status.
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

    // Customer information structure.
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

    // Restaurant information structure.
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

    // Courier information structure.
    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Courier {
        pub courier_account: AccountId,
        pub courier_name: String,
        pub courier_address: String,
        pub phone_number: String,
    }

    // Food information structure.
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

    // Order information structure.
    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Order {
        pub food_id: FoodId,
        pub restaurant_id: RestaurantId,
        pub customer_id: CustomerId,
        pub courier_id: CourierId,
        pub delivery_address: String,
        pub status: OrderStatus,
        pub timestamp: Timestamp,
        pub price: Balance,
        pub eta: u64,
    }

    // Delivery informaiton structure.
    #[derive(scale::Decode, scale::Encode, Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Delivery {
        pub order_id: OrderId,
        pub restaurant_id: RestaurantId,
        pub customer_id: CustomerId,
        pub courier_id: CourierId,
        pub delivery_address: String,
        pub status: DeliveryStatus,
        pub timestamp: Timestamp,
    }

    // Main Data Structure.
    #[ink(storage)]
    pub struct FoodDelivery {
        // Variable that stores contract's manager account.
        pub manager: AccountId,
        // Variable that stores last food identifier.
        pub food_id: u64,
        // Variable that stores last order identifier.
        pub order_id: u64,
        // Variable that stores last delivery identifier.
        pub delivery_id: u64,
        // Variable that stores last customer identifier.
        pub customer_id: u64,
        // Variable taht stores last restaurant identifier.
        pub restaurant_id: u64,
        // Variable that stores last courier identifier.
        pub courier_id: u64,
        // Variable that stores customer data.
        pub customers: Mapping<CustomerId, Customer>,
        // Variable that stores restaurant data.
        pub restaurants: Mapping<RestaurantId, Restaurant>,
        // Variable that stores courier data.
        pub couriers: Mapping<CourierId, Courier>,
        // Variable that stores food data.
        pub food_data: Mapping<FoodId, Food>,
        // Variable that stores order data.
        pub order_data: Mapping<OrderId, Order>,
        // Variable that stores delivery data.
        pub delivery_data: Mapping<DeliveryId, Delivery>,
        // Variable that stores food identifiers posted by the restaurant.
        pub restaurant_food_data: Mapping<RestaurantId, Vec<FoodId>>,
        // Variable that stores order identifiers placed in a restaurant.
        pub restaurant_order_data: Mapping<RestaurantId, Vec<OrderId>>,
        // Variable that stores delivery identifiers requested by the restaurant.
        pub restaurant_delivery_data: Mapping<RestaurantId, Vec<DeliveryId>>,
        // Variable that stores order identifiers requested by the customer.
        pub customer_order_data: Mapping<CustomerId, Vec<OrderId>>,
        // Variable that stores delivery indentifiers.
        pub customer_delivery_data: Mapping<CustomerId, Vec<DeliveryId>>,
        // Variable that stores delivery ordered to the couriers.
        pub courier_delivery_data: Mapping<CourierId, Vec<DeliveryId>>,
        // Variable that stores customer account and customer identifier mapping.
        pub customer_account_id: Mapping<AccountId, CustomerId>,
        // Variable that stores restaurant account and restaurant identifier mapping.
        pub restaurant_account_id: Mapping<AccountId, RestaurantId>,
        // Variable that stores courier account and courier identifier mapping.
        pub courier_account_id: Mapping<AccountId, CourierId>,
    }

    impl FoodDelivery {
        // Constructor that initializes the data.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                manager: Self::env().caller(),
                food_id: 1,
                order_id: 1,
                delivery_id: 1,
                customer_id: 1,
                restaurant_id: 1,
                courier_id: 1,
                customers: Mapping::default(),
                restaurants: Mapping::default(),
                couriers: Mapping::default(),
                food_data: Mapping::default(),
                order_data: Mapping::default(),
                delivery_data: Mapping::default(),
                restaurant_food_data: Mapping::default(),
                restaurant_order_data: Mapping::default(),
                restaurant_delivery_data: Mapping::default(),
                customer_order_data: Mapping::default(),
                customer_delivery_data: Mapping::default(),
                courier_delivery_data: Mapping::default(),
                customer_account_id: Mapping::default(),
                restaurant_account_id: Mapping::default(),
                courier_account_id: Mapping::default(),
            }
        }

        // Customer's function.
        // Function that add new customer.
        #[ink(message)]
        pub fn add_customer(
            &mut self,
            customer_name: String,
            customer_address: String,
            phone_number: String
        ) -> Customer {
            let customer_account = Self::env().caller();
            assert!(!self.customer_account_id.contains(&customer_account), "Customer already exists");
            assert!(customer_name.len() > 0, "Customer name must not be empty");
            assert!(customer_address.len() > 0, "Customer address must not be empty");
            assert!(phone_number.len() > 0, "Phone number must not be empty");
            let customer = Customer {
                customer_account,
                customer_name,
                customer_address,
                phone_number,
            };
            let customer_id = self.customer_id;
            self.customer_id += 1;
            self.customers.insert(&customer_id, &customer);
            self.customer_account_id.insert(&customer_account, &customer_id);
            customer
        }
        
        // Customer's function.
        // Function taht request an order.
        #[ink(message, payable)]
        pub fn submit_order(
            &mut self, 
            food_id: FoodId,
            delivery_address: String,
        ) -> Order {
            let customer_account = Self::env().caller();
            assert!(self.customer_account_id.contains(&customer_account), "The current caller is not the customer AccountId");
            assert!(self.food_data.contains(&food_id), "Food does not exist");
            assert!(delivery_address.len() > 0, "Delivery address must not be empty");
            let customer_id = self.customer_account_id.get(&customer_account).unwrap();
            let restaurant_id = self.food_data.get(&food_id).unwrap().restaurant_id;
            let courier_id = 0;
            let price = Self::env().transferred_value();
            assert!(self.food_data.get(&food_id).unwrap().price == price, "You must pay same of price");
            let eta = 0;
            let timestamp = Self::env().block_timestamp();
            let status = OrderStatus::OrderSubmitted;
            let order = Order {
                food_id,
                restaurant_id,
                customer_id,
                courier_id,
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
        
        // Customer's function.
        // Function that confirm a delivery.
        #[ink(message)]
        pub fn confirm_delivery(
            &mut self,
            delivery_id: DeliveryId,
        ) -> bool {
            let customer_account = Self::env().caller();
            assert!(self.customer_account_id.contains(&customer_account), "The current caller is not the customer AccountId");
            let customer_id = self.customer_account_id.get(&customer_account).unwrap();
            assert!(self.delivery_data.contains(&delivery_id), "Delivery does not exist");
            let order_id = self.delivery_data.get(&delivery_id).unwrap().order_id;
            assert!(self.order_data.get(&order_id).unwrap().status == OrderStatus::OrderDelivered, "This order is not Delivered");
            assert!(self.order_data.get(&order_id).unwrap().customer_id == customer_id, "This caller is not customer of this order");
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
            
            let courier_amount = _price / 10;
            let restaurant_amount = _price - courier_amount;
            let courier_account = self.couriers.get(&self.delivery_data.get(&delivery_id).unwrap().courier_id).unwrap().courier_account;
            let restaurant_account = self.restaurants.get(&self.delivery_data.get(&delivery_id).unwrap().restaurant_id).unwrap().restaurant_account;
            if Self::env().transfer(courier_account, courier_amount).is_err() {
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
        // Function that add new food with information.
        #[ink(message)]
        pub fn add_food(
            &mut self,
            food_name: String,
            description: String,
            price: Balance,
            eta: u64,
        ) -> Food {
            let restaurant_account = Self::env().caller();
            assert!(self.restaurant_account_id.contains(&restaurant_account), "The current caller is not the restaurant AccountId");
            assert!(food_name.len() > 0, "Food name must not be empty");
            assert!(description.len() > 0, "Food description must not be empty");
            let restaurant_id = self.restaurant_account_id.get(&restaurant_account).unwrap();
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
        
        // Restaurant's function.
        // Function that update the food inforamtion using food_id.
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
            assert!(self.restaurant_account_id.contains(&restaurant_account), "The current caller is not the restauarnat caller");
            let restaurant_id = self.restaurant_account_id.get(&restaurant_account).unwrap();
            assert!(self.food_data.contains(&food_id), "Food does not exist");
            assert!(self.food_data.get(&food_id).unwrap().restaurant_id == restaurant_id, "This caller is not owner of this food");
            assert!(food_name.len() > 0, "Food name must not be empty");
            assert!(description.len() > 0, "Food description must not be empty");
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
    
        // Restaurant's function.
        // Function that confirm the order requested by customer.
        #[ink(message)]
        pub fn confirm_order(
            &mut self,
            order_id: OrderId,
        ) -> bool {
            let restaurant_account = Self::env().caller();
            assert!(self.restaurant_account_id.contains(&restaurant_account), "The current caller is not the restaurant AccountId");
            let restaurant_id = self.restaurant_account_id.get(&restaurant_account).unwrap();
            assert!(self.order_data.contains(&order_id), "Order does not exist");
            let food_id = self.order_data.get(&order_id).unwrap().food_id;
            assert!(self.food_data.get(&food_id).unwrap().restaurant_id == restaurant_id, "This caller is not restaurant of this order");
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
    
        // Restaurant's function.
        // Function that request a delivery.
        #[ink(message)]
        pub fn deliver_order(
            &mut self,
            order_id: OrderId,
        ) -> Delivery {
            let restaurant_account = Self::env().caller();
            assert!(self.restaurant_account_id.contains(&restaurant_account), "The current caller is not the restaurant AccountId");
            let restaurant_id = self.restaurant_account_id.get(&restaurant_account).unwrap();
            assert!(self.order_data.contains(&order_id), "Order does not exist");
            assert!(self.order_data.get(&order_id).unwrap().status == OrderStatus::OrderConfirmed, "This order is not confirmed");
            let food_id = self.order_data.get(&order_id).unwrap().food_id;
            assert!(self.food_data.get(&food_id).unwrap().restaurant_id == restaurant_id, "This caller is not restaurant of this order");
            let mut order = self.order_data.get(&order_id).unwrap();
            let status = OrderStatus::WaitingDeliver;
            order.status = status;
            self.order_data.insert(&order_id, &order);
            let delivery_id = self.delivery_id;
            self.delivery_id += 1;
            let restaurant_id = self.order_data.get(&order_id).unwrap().restaurant_id;
            let customer_id = self.order_data.get(&order_id).unwrap().customer_id;
            let courier_id = 0;
            let delivery_address = self.order_data.get(&order_id).unwrap().delivery_address;
            let status = DeliveryStatus::Waiting;
            let timestamp = Self::env().block_timestamp();
            let delivery = Delivery {
                order_id,
                restaurant_id,
                customer_id,
                courier_id,
                delivery_address,
                status,
                timestamp,
            };
            self.delivery_data.insert(&delivery_id, &delivery);
            let mut restaurant_delivery_vec = self.restaurant_delivery_data.get(&restaurant_id).unwrap_or(Vec::new());
            restaurant_delivery_vec.push(delivery_id);
            self.restaurant_delivery_data.insert(&restaurant_id, &restaurant_delivery_vec);
            let mut customer_delivery_vec = self.customer_delivery_data.get(&customer_id).unwrap_or(Vec::new());
            customer_delivery_vec.push(delivery_id);
            self.customer_delivery_data.insert(&customer_id, &customer_delivery_vec);
            let mut courier_delivery_vec = self.courier_delivery_data.get(&courier_id).unwrap_or(Vec::new());
            courier_delivery_vec.push(delivery_id);
            self.courier_delivery_data.insert(&courier_id, &courier_delivery_vec);
            let delivery_address = self.order_data.get(&order_id).unwrap().delivery_address;
            Self::env().emit_event(DeliverOrderEvent {
                order_id,
                restaurant_id,
                customer_id,
                delivery_address,
            });
            delivery
        }
        
        // Courier's function.
        // Function that pick up the delivery requested by restaurant.
        #[ink(message)]
        pub fn pickup_delivery(
            &mut self,
            delivery_id: DeliveryId,
        ) -> bool {
            let caller = Self::env().caller();
            assert!(self.courier_account_id.contains(&caller), "The current caller is not courier AccountId");
            assert!(self.delivery_data.contains(&delivery_id), "Delivery does not exist");
            assert!(self.delivery_data.get(&delivery_id).unwrap().status == DeliveryStatus::Waiting, "This delivery is already picked up");
            let mut delivery = self.delivery_data.get(&delivery_id).unwrap();
            let courier_id = self.courier_account_id.get(&caller).unwrap();
            let status = DeliveryStatus::PickUp;
            delivery.courier_id = courier_id;
            delivery.status = status;
            self.delivery_data.insert(&delivery_id, &delivery);
            let order_id = self.delivery_data.get(&delivery_id).unwrap().order_id;
            let order_status = OrderStatus::OrderDelivered;
            let mut order = self.order_data.get(&order_id).unwrap();
            order.status = order_status;
            order.courier_id = courier_id;
            self.order_data.insert(&order_id, &order);
            Self::env().emit_event(PickupDeliveryEvent{delivery_id});
            true
        }
        
        // Manager's function.
        // Function that add new restaurant.
        #[ink(message)]
        pub fn add_restaurant(
            &mut self,
            restaurant_account: AccountId,
            restaurant_name: String,
            restaurant_address: String,
            phone_number: String,
        ) -> Restaurant {
            let caller = Self::env().caller();
            assert!(caller == self.manager, "The caller is not the manager AccountId");
            assert!(!self.restaurant_account_id.contains(&restaurant_account), "Restaurant already exists");
            assert!(restaurant_name.len() > 0, "Restaurant name must not be empty");
            assert!(restaurant_address.len() > 0, "Restaurant adddress must not be empty");
            assert!(phone_number.len() > 0, "Phone number must not be empty");
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
    
        // Manager's function.
        // Function that add new Courier.
        #[ink(message)]
        pub fn add_courier(
            &mut self,
            courier_account: AccountId,
            courier_name: String,
            courier_address: String,
            phone_number: String,
        ) -> Courier {
            let caller = Self::env().caller();
            assert!(caller == self.manager, "The current caller is not the manager AccountId");
            assert!(!self.courier_account_id.contains(&courier_account), "Courier already exists");
            assert!(courier_name.len() > 0, "Courier name must not be empty");
            assert!(courier_address.len() > 0, "Courier adddress must not be empty");
            assert!(phone_number.len() > 0, "Phone number must not be empty");
            let courier_id = self.courier_id;
            self.courier_id += 1;
            let courier = Courier {
                courier_account,
                courier_name,
                courier_address,
                phone_number,
            };
            self.couriers.insert(&courier_id, &courier);
            self.courier_account_id.insert(&courier_account, &courier_id);
            let courier_name = self.couriers.get(&courier_id).unwrap().courier_name;
            let courier_address = self.couriers.get(&courier_id).unwrap().courier_address;
            let phone_number = self.couriers.get(&courier_id).unwrap().phone_number;
            Self::env().emit_event(AddCourierEvent {
                courier_id,
                courier_name,
                courier_address,
                phone_number,
            });
            courier
        }
    
        // Manager's function.
        // Function that change manager's account.
        #[ink(message)]
        pub fn change_manager(
            &mut self,
            new_account: AccountId,
        ) -> bool {
            let caller = Self::env().caller();
            assert!(caller == self.manager, "The current caller is not the manager AccountId");
            self.manager = new_account;
            true
        }
        
        // Function that get eta deadline using order identifier.
        #[ink(message)]
        pub fn get_eta(&self, order_id: OrderId) -> u64 {
            assert!(self.order_data.contains(&order_id), "Order does not exist");
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

        // Function that get order information using order identifier.
        #[ink(message)]
        pub fn get_order_from_id(&self, order_id: OrderId) -> Order {
            assert!(self.order_data.contains(&order_id), "Order does not exist");
            self.order_data.get(&order_id).unwrap()
        }
    
        // Function that get all order information placed in a restaurant.
        #[ink(message)]
        pub fn get_order_from_restaurant(&self, restaurant_id: RestaurantId) -> Vec<Order> {
            assert!(self.restaurants.contains(&restaurant_id), "Restaurant does not exist");
            let order_data = self.restaurant_order_data.get(&restaurant_id).unwrap_or(Vec::new());
            let mut order_vec = Vec::new();
            for i in order_data.iter() {
                order_vec.push(self.order_data.get(&i).unwrap());
            }
            order_vec       
        }
        
        // Function that get all order information placed by customers.
        #[ink(message)]
        pub fn get_order_from_customer(&self, customer_id: CustomerId) -> Vec<Order> {
            assert!(self.customers.contains(&customer_id), "Restaurant does not exist");
            let order_data = self.customer_order_data.get(&customer_id).unwrap_or(Vec::new());
            let mut order_vec = Vec::new();
            for i in order_data.iter() {
                order_vec.push(self.order_data.get(&i).unwrap());
            }
            order_vec  
        }
        
        // Function that get all orders from A to B.
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
        
        // Function that get food information using food identifier.
        #[ink(message)]
        pub fn get_food_from_id(&self, food_id: FoodId) -> Food {
            assert!(self.food_data.contains(&food_id), "Order does not exist");
            self.food_data.get(&food_id).unwrap()
        }
        
        // Function that get all food information posted by the restaurant.
        #[ink(message)]
        pub fn get_food_from_restaurant(&self, restaurant_id: RestaurantId) -> Vec<Food> {
            assert!(self.restaurants.contains(&restaurant_id), "Restaurant does not exist");
            let food_data = self.restaurant_food_data.get(&restaurant_id).unwrap_or(Vec::new());
            let mut food_vec = Vec::new();
            for i in food_data.iter() {
                food_vec.push(self.food_data.get(&i).unwrap());
            }
            food_vec        
        }
    
        // Function that get all food information from A to B.
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
    
        // Function that get delivery information using delivery identifier.
        #[ink(message)]
        pub fn get_delivery_from_id(&self, delivery_id: DeliveryId) -> Delivery {
            assert!(self.delivery_data.contains(&delivery_id), "Order does not exist");
            self.delivery_data.get(&delivery_id).unwrap()
        }
        
        // Function that get all delivery information ordered form the forwarder.
        #[ink(message)]
        pub fn get_delivery_from_courier(&self, courier_id: CourierId) -> Vec<Delivery> {
            assert!(self.couriers.contains(&courier_id), "Courier does not exist");
            let delivery_data = self.courier_delivery_data.get(&courier_id).unwrap_or(Vec::new());
            let mut delivery_vec = Vec::new();
            for i in delivery_data.iter() {
                delivery_vec.push(self.delivery_data.get(&i).unwrap());
            }
            delivery_vec
        }

        // Function that get all delivery information requested by restaurant.
        #[ink(message)]
        pub fn get_delivery_from_restaurant(&self, restaurant_id: RestaurantId) -> Vec<Delivery> {
            assert!(self.restaurants.contains(&restaurant_id), "Restaurant does not exist");
            let delivery_data = self.restaurant_delivery_data.get(&restaurant_id).unwrap_or(Vec::new());
            let mut deliver_vec = Vec::new();
            for i in delivery_data.iter() {
                deliver_vec.push(self.delivery_data.get(&i).unwrap());
            }
            deliver_vec     
        }
        
        // Function taht get all delivery information delivered to customer.
        #[ink(message)]
        pub fn get_delivery_from_customer(&self, customer_id: CustomerId) -> Vec<Delivery> {
            assert!(self.customers.contains(&customer_id), "Customer does not exist");
            let delivery_data = self.customer_delivery_data.get(&customer_id).unwrap_or(Vec::new());
            let mut delivery_vec = Vec::new();
            for i in delivery_data.iter() {
                delivery_vec.push(self.delivery_data.get(&i).unwrap());
            }
            delivery_vec       
        }

        // Function that get all delivery information.
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

        // Function that get all restaurant information.
        #[ink(message)]
        pub fn get_restaurant_all(&self, from: u64, to:u64) -> Vec<Restaurant> {
            let mut restaurant_vec: Vec<Restaurant> = Vec::new();
            if to < self.delivery_id {
                for i in from..to {
                    restaurant_vec.push(self.restaurants.get(&i).unwrap());
                }
            } else {
                for i in from..self.delivery_id {
                    restaurant_vec.push(self.restaurants.get(&i).unwrap());
                }
            }
            restaurant_vec
        }
    }
}