#[derive(Debug)]

enum Message { //defining enum
    Quit, //defing varient 1 with perform nothing
    Write(String), //defining varient 2 with data type string 
    Move{x: i32, y: i32},
    change_color(u32,u32,u32)
}
impl Message {
    fn call(&self){
        println!("{:?}",self);
    }
}
fn main() {
    let msg_1 = Message::Write(String::from("hay jani dakha"));
    let msg_2 = Message::change_color(12,52,43);

    msg_1.call();
    msg_2.call();

}
