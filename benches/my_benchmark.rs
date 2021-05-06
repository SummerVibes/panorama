use criterion::{black_box, criterion_group, criterion_main, Criterion};

use actix_web::client::Client;
use actix_web::dev::Url;
use panorama::device::DeviceType::Phone;
use criterion::async_executor::FuturesExecutor;
// #[bench]
// fn bench(b: &mut Bencher) {
//     let mut node = Node::create(Phone);
//     node.start().is_ok().then(|| info!("node started"));
//     //create a server
//     let device = node.device.clone();
//     let srv = HttpServer::new(move ||{
//         App::new().configure(|cfg| {device.get_service(cfg)})
//     }).bind(format!("{}:{}",node.addr.to_string(),HTTP_SERVICE_PORT)).unwrap().run();
//     // create another thread to handle request
//     thread::spawn(|| block_on(async{
//         info!("http server started");
//         srv.await.unwrap();
//     }));
//     let client = Client::default();
//     b.iter(|| block_on(async{
//         let res = client.get(String::from("http://192.168.88.99:8080/take_photo")).send().await;;
//         match res {
//             Ok(mut resp) => {
//                 let str = resp.body().await.unwrap();
//                 let str = String::from_utf8(str.to_vec()).unwrap();
//                 println!("From {}, Response: {:?}",url, str);
//             }
//             Err(err) => {
//                 println!("Execute failed: {:?}", err);
//             }
//         }
//     }));
//     // b.iter(|| );
// }
async fn send(url: String){
    let client = Client::default();
    let res = client.get(url.clone()).send().await;
    match res {
        Ok(mut resp) => {
            let str = resp.body().await.unwrap();
            let str = String::from_utf8(str.to_vec()).unwrap();
            println!("Response: {:?}",str);
        }
        Err(err) => {
            println!("Execute failed: {:?}", err);
        }
    }
}
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("panorama", |b| b.to_async(FuturesExecutor).iter(|| send(black_box(String::from("http://192.168.88.99:8080/take_photo")))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);