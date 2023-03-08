mod tcping;
use tcping::*;

pub fn start_tcp_ping(addrs: &[String],send_byte:bool,timeout:u64,max_count:i32,handle:& dyn Fn(String,i32)->bool){
    tcp_pings(addrs, send_byte, timeout, max_count, handle);
}