#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let (width, height) = (30u32, 50u32);
    print_area(area_values(width, height));

    let rectangle = (31u32, 51u32);
    print_area(area_tuple(rectangle));

    // allowed once primitive values are copied to one variable to another
    print_area(area_tuple(rectangle));

    let rect = Rectangle{
        width: 32,
        height: 52,
    };
    // Even if area_struct parameter was without the reference,
    // due ownership rect will be invalidated
    //print_area(area_struct(rect));
    print_area(area_struct(&rect));
}

fn area_struct(value : &Rectangle) -> u32 {
    println!("This is the rect values: {:#?}", value);
    value.width * value.height
}

fn area_tuple(tuple: (u32, u32)) -> u32 {
    tuple.0 * tuple.1
}

fn area_values(width : u32, height : u32) -> u32 {
    width * height
}

fn print_area(value : u32) {
    println!("The area of the rectangle is {} square pixels.", value);
}
