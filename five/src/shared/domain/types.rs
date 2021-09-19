extern crate serde;
extern crate serde_json;

use crate::shared::domain::types::{banking::BankAccount, location::AddressType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullName {
    pub name: String,
    pub surname: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Email {
    pub value: String,
}

//Personal Identifying Information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersonalInformation {
    pub fullname: FullName,
    pub address: AddressType,
    pub cellphone: String,
    pub email: Email,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BusinessInformation {
    pub name: String,
    pub trading_address: AddressType,
    pub cellphone: String,
    pub email: Email,
}

// Users of system
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Buyer {
    pub personal_information: PersonalInformation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum Seller {
    INDIVIDUAL(IndividualSeller),
    BUSINESS(BusinessSeller),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndividualSeller {
    pub personal_information: PersonalInformation,
    pub account: BankAccount,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BusinessSeller {
    pub business_information: BusinessInformation,
    pub account: BankAccount,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum Courier {
    INDIVIDUAL(IndividualCourier),
    BUSINESS(BusinessCourier),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndividualCourier {
    pub personal_information: PersonalInformation,
    pub vehicle: Vehicle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BusinessCourier {
    pub vehicles: Vec<Vehicle>,
}

//Delivery
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum Vehicle {
    Taxi,
    Minibus,
    Van,
    MotorBike,
    Bicycle,
    Sedan,
    HatchBack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Taxi {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Minibus {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MotorBike {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bicycle {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sedan {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hatchback {}

//Banking
pub mod banking {
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "type", content = "content")]
    pub enum BankAccount {
        SavingsAccount(SavingsAccount),
        ChequeAccount(ChequeAccount),
        BusinessChequeAccount(BusinessChequeAccount),
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SavingsAccount {}

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ChequeAccount {}

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct BusinessChequeAccount {}
}

//Payment
pub mod payment {
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "type", content = "content")]
    pub enum PaymentMethod {
        CreditCard(CreditCardPayment),
        EFT(EFTPayment),
        SnapScan(SnapScanPayment),
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CreditCardPayment {}

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct EFTPayment {}

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SnapScanPayment {}

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Payment {
        payment_method: PaymentMethod,
        paid: bool,
    }
}

//Orders
pub mod orders {
    use crate::shared::domain::types::{
        payment::Payment, products::Product, Buyer, Seller,
    };

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Order {
        seller: Seller,
        buyer: Buyer,
        payment: Payment,
        products: Vec<Product>,
    }
}

//Products
pub mod products {
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "type", content = "content")]
    pub enum Product {}
}

//Location

pub mod location {

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Address {
        pub address_line_1: String,
        pub address_line_2: String,
        pub suburb: String,
        pub city: City,
        pub province: Province,
        pub coordinates: Coordinates,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct City {
        pub name: String,
        pub short_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Town {
        pub name: String,
        pub short_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Province {
        pub name: String,
        pub short_name: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Coordinates {
        pub longitude: f32,
        pub latitude: f32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Country{}
}
