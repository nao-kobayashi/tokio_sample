use std::net::UdpSocket;
use std::thread;

fn main() {
    let mut handles = Vec::new();

    for i in 0..10000 {
        handles.push(thread::spawn(move || {
            let own_addr = format!("127.0.0.1:{}", 10000 + i);
            let udp = UdpSocket::bind(own_addr).unwrap();

            for h in 0..100 {
                let mut buf = vec![0; 256];
                let send_s = format!("test {}-{}", i, h);
                let _ = udp.send_to(send_s.as_bytes(), "127.0.0.1:9000").unwrap();
                let size = udp.recv(&mut buf).unwrap();
                let echo_s = String::from_utf8(buf[..size].to_vec());
                // println!("{:?}", echo_s);
            }
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
}
