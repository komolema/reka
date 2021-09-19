pub struct Currency{}
pub struct Seller{}
pub struct PersonalIndentifyInformation{}
pub struct BusinessDetails{}
pub struct Buyer{}
pub struct Payment{}
pub enum PaymentType{
    Crypto,
    Fiat
}
pub enum DeliveryType{}
pub struct Location{}
pub enum LocationType{}
pub struct Coordinates{}
pub struct Country{}
pub struct Province{}
pub struct State{}
pub struct Town{}
pub struct City{}
pub struct Address{}
pub struct Street{}
pub struct Date{}
pub struct Time{}
pub struct Order{}
pub enum OrderType{}
pub enum BuyerType{}
pub struct Crypto{}
pub enum CryptoType{}
pub struct Fiat{}
pub enum FiatType{}
pub struct Token{}
pub enum TokenType{
    CryptoToken,
    FiatToken
}

pub enum CryptoToken{
    CardanoToken,
    BitcoinToken,
    LiteCoinToken,
}

pub enum FiatToken{
    Rand,
    USDollar,
    Pula
}

pub struct Vehicle{}
pub enum VehicleType{}
pub struct VehicleRegistration{}
pub struct VehicleDimensions{}
pub struct VehicleCapacity{}
pub enum VehicleStatus{}
pub enum DriverStatus{}
pub enum VehicleCapacityStatus{}
pub enum VehicleRouteStatus{}

pub struct StoreUser{}
pub struct Route{}
pub struct Scooter{}

pub struct Settlement{}
pub enum SettlementType{
    FiatEFT,
    FiatPaymentGateway,
    FiatCrypto,
    Crypto
}

pub struct DelieryFee{}
pub struct ValueAddedTax{}
pub struct Price{}
pub enum TaxType{
    VAT,
    Service
}

pub struct MenuItem{}
pub struct Menu{}
pub struct DateOfBirth{}
pub struct Surname{}
pub struct Name{}
pub struct Email{}
pub enum StoreType{}
pub struct Store{}
pub struct CourierUser{}
pub struct Driver{}
pub struct Fees{}
pub struct Restaurant{}
pub struct Food{}
pub enum FoodPreparationStatus{}
pub enum OrderStatus{}
pub enum DeliveryStatus{}
pub struct Distance{}
pub struct Notify{}
pub struct StoreOwner{}
pub struct Merchant{}

pub struct BuyerCache{}
pub struct SellerCache{}
pub struct CourierCache{}
pub struct RegionalConfigCache{}
pub struct CountryConfigCache{}