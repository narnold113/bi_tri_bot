mod get_arbs;

fn main() {
    // println!("Hello, world!");

    println!("{:#?}", get_arbs::get_arbs().unwrap());

    // let content = reqwest::get("https://api.binance.com/api/v3/avgPrice?symbol=BTCUSDT").unwrap().text();
    // println!("body = {:?}", content);
}