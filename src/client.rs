use core::f64;
use json;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::sync::mpsc;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use websockets::WebSocket;



fn main() {
    loop {
        //println!("1");
        let (tx, rx) = mpsc::channel();

        let mut join_handle = vec![];

        for i in 0..5 {
            //println!("2");

            let tx = tx.clone();

            join_handle.push(thread::spawn(move || {

                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    //println!("3");
                    let mut ws =
                        WebSocket::connect("wss://stream.binance.com:9443/ws/btcusdt@trade")
                            .await
                            .expect("problem in ws");

                    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");


                    let mut sum: f64 = 0.0;
                    let mut total: f64 = 0.0;

                    let start_time = Instant::now();
                    let duration_limit = Duration::from_secs(10);

                    // let mut buffer = [0; 512];
                    // let public_key = stream.read(&buffer).unwrap();

                    while Instant::now() - start_time < duration_limit {

                        let frame = ws.receive().await.expect("problem in receiving ws");

                        let data = json::parse(
                            frame
                                .as_text()
                                .expect("something problem with extracting")
                                .0,
                        )
                        .expect("problem in converting json");

                        //println!("5");
                        //println!(" price of btc - {:?}",data["p"]);

                        let btc_price = data["p"].as_str().expect("problem converting as f64");
                        total += 1.0;
                        sum += btc_price.parse::<f64>().unwrap();

                    }

                    let average = sum / total;
                    //println!("---{}", average);

                    //let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, average).expect("failed to encrypt");

                    tx.send(average);
                    stream.write(average.to_string().as_bytes()).unwrap();
                })
            }));
        }
        //let mut avgofavg : f64 = 0.0;
        let mut sum: f64 = 0.0;

        for handle in join_handle {
            handle.join().unwrap();
            let recv = rx.recv().expect("problem in receving");
            //println!("recv ->{} ", recv);
            sum += recv;
        }


        println!("The average USD price of BTC is: {}", sum / 5.0);


        let duration = Duration::from_secs(30); //860390
        thread::sleep(duration);
    }
}
