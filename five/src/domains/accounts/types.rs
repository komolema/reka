use crate::shared::domain::types::banking::BankAccount;
use crate::shared::domain::types::Vehicle;
use crate::shared::domain::types::location::AddressType;
use chrono::{Date, Local};


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
    pub date_of_birth: Date<Local>
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
#[serde(tag = "type", content = "content")]
pub enum BuyerType {
    Individual(IndividualBuyer),
    Business(BusinessBuyer)
}

pub struct IndividualBuyer {
    pub personal_information: PersonalInformation
}

pub struct BusinessBuyer{
    pub business_information: BusinessInformation
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum SellerType {
    Individual(IndividualSeller),
    Business(BusinessSeller)
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
pub enum CourierType {
    Individual(IndividualCourier),
    Business(BusinessCourier),
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

pub struct Session{}
pub struct SessionDate{}
pub struct SessionCallback{}