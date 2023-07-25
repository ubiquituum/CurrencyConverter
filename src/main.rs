const CURRENCIES: Currencies = Currencies{
    USD: CurrencyInfo 
    {
        ratio: 1.0,
        name: "US Dollar",
    },
    INR: CurrencyInfo 
    {
        ratio: 0.012,
        name: "Indian Rupee",
    },
    ILS: CurrencyInfo 
    {
        ratio: 0.27,
        name: "Israeli New Shekel",
    },
};

#[allow(non_snake_case)]
struct Currencies
{
    USD: CurrencyInfo,
    INR: CurrencyInfo,
    ILS: CurrencyInfo,
}


struct CurrencyInfo
{
    ratio: f64,
    name: &'static str,
}

struct CurrencyCount 
{
    count: f64,
    currency: &'static CurrencyInfo,
}



impl CurrencyInfo 
{
    fn count(&'static self, count: f64) -> CurrencyCount
    {
        CurrencyCount { count, currency: self }
    }
}

impl CurrencyCount 
{
    fn print(&self) -> &Self
    {
        println!("Name: {}, Value: {:.5}", self.currency.name, self.count);
        self
    }

    fn convert(&self, target: &'static CurrencyInfo) -> CurrencyCount 
    {
        CurrencyCount 
        {
            count: (self.currency.ratio / target.ratio) * self.count,
            currency: target,
        }
    }
}

fn main() 
{
    CURRENCIES.INR.count(1.0).print().convert(&CURRENCIES.ILS).print();
    CURRENCIES.USD.count(1.0).print().convert(&CURRENCIES.INR).print();
}
