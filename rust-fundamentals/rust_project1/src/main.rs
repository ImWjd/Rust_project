//创建一个枚举值
enum IpAddKind {
    V4,
    V6,    
}

struct IpAddr{
    kind:IpAddKind,
    address:String,
}
fn main(){
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;
    route(four);
    route(six);
    route(IpAddKind::V6);

    let home = IpAddr{
        kind:IpAddKind::V4,
        address:String::from("127.0.0.1"),
    };
    let loopback = IpAddr{
        kind:IpAddKind::V6,
        address:String::from("::1"),
    };


}

fn route(ip_kind:IpAddKind){

}
