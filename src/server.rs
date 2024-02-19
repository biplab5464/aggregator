use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    // Read data from the client
    match stream.read(&mut buffer){
        Ok(_) => {
            
            // let mut rng = rand::thread_rng();
            // let bits = 2048;
            // let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
            // let pub_key = RsaPublicKey::from(&priv_key);
            
            // // send public key to client
            // stream.write(pub_key).expect("Failed to send public key");

            let recv: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);

            let avg_price = recv.to_string(); //.parse().expect("expeted f64");

            //let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");

            println!("{}",avg_price)
            // let mut recv_message =0;
            // let mut sum: f64 = 0.0;

            // recv_message += 1;
            // sum += avg_price;
            // if recv_message == 5{
            //     println!("The average USD price of BTC is: {}",sum/5.0);
            //     sum =0.0;
            //     recv_message =0;
            // }

        }
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream)
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
