pub struct Currency{}
pub struct Payment{}
pub enum CurrencyType{}
pub enum PaymentType{}
pub struct Crypto{}
pub enum CryptoType{}
pub struct Fiat{}
pub struct FiatType{}
pub struct Token{}
pub enum TokenPlatformType{
    InternalToken,
    CardanoToken,
    EthereumToken
}
pub struct Bank{}
pub struct PaymentGateway{}
pub struct Settlement{}
pub enum SettlementType{}
pub struct Refund{}
pub enum RefundStatus{
    NotStarted,
    InProgress,
    Failed,
    Completed
}
pub enum SettlementStrategy{
    Fiat,
    Bitcoin,
    Litecoin,
    Cardano
}
pub struct Reconciliation{}
pub enum ReconStatus{}
pub struct BankAccount{}
pub struct FiatWallet{}
pub struct BitcoinWallet{}
pub struct LitecoinWallet{}
pub struct CardanoWallet{}
pub struct EthereumWallet{}
pub struct CustomCardanoTokenWallet{}
pub struct CustomEthereumTokenWallet{}
pub enum WalletType{
    Fiat,
    Bitcoin,
    Litecoin,
    Cardano,
    Ethereum,
    CustomCardanoToken,
    CustomEthereumToken
}