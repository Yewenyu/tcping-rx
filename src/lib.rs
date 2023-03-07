use std::{string, net::TcpStream, time::{SystemTime, UNIX_EPOCH, Duration}, io::{Write, Read}, collections::HashMap, sync::mpsc::{self, channel}, thread};



fn tcp_pings(addrs: &[String],send_byte:bool,timeout:u64,max_count:i32,handle:& dyn Fn(String,i32)->bool){
   

    
    let (maxtx, maxrx) = mpsc::channel();
    let mut current = 0;
    let addrs = addrs.to_vec();
    let (tx,rx) = mpsc::channel();
    thread::spawn(move||{
        for _addr in addrs{
            let maxtx = maxtx.clone();
            let addr = _addr.to_string();
            let tx = tx.clone();
            thread::spawn(move || {
                let result = tcp_ping(addr.clone(), send_byte, timeout);
                _ = tx.send((addr,result));
                _ = maxtx.send(-1)
            });
            current += 1;
            if max_count > 0{
                if current > max_count{
                    if let Ok(r) = maxrx.recv() {
                        current += r;
                    }
                }
            }
        }
    });

    while let Ok(r) = rx.recv() {
        let stop = handle(r.0,r.1);
        if stop{
            break;
        }
    }
    
   
}

fn tcp_ping(addr:String,send_byte:bool,timeout:u64) -> i32{

    let start = current_time();
    let result = TcpStream::connect(addr);
    match result {
        Ok(mut stream) => {
            if send_byte == false{
                let offset = current_time() - start;
                return offset.try_into().unwrap();
            }
            let mut buf : [u8;64] = [0;64];
            _ = stream.write(&buf);
            _ = stream.set_read_timeout(Some(Duration::from_secs(timeout)));
            let result = stream.read(&mut buf);
            if let Ok(size) = result{
                if size == 64{
                    let offset = current_time() - start;
                    return offset.try_into().unwrap();
                }
            }
        },
        Err(_) =>{}
    }
    return -1;
}
fn current_time() -> u128{
    return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
}