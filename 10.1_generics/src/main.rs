fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         // Unable to figure out PartialOrd
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Location<T, U> {
    x: T,
    y: U,
}

impl<T, U> Location<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: Location<V, W>) -> Location<T, W> {
        Location {
            x: self.x,
            y: other.y,
        }
    }
}

impl Location<f32, f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'z'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    println!("============== Generics ================");
    
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q', 'z'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    println!("============== Point ================");
    
    let integer = Point { x: 9, y: 1 };
    let float = Point { x: 1.0, y: 2.1 };
    let home = Location { x: "Street 1", y: 999874 };
    let shopping = Location { x: "Street 2", y: 991174 };

    //println!("Mix: {:?}", home.mixup(shopping));
    
    println!("The x is {}", float.x());
    println!("The x is {}", home.x());

    // println!("Integer distance: {}", integer.distance());
    println!("Float distance: {}", float.distance());
    // println!("Location distance: {}", home.distance());

    //Using generics in Rust doesn't compromises the runtime speed.
    // This is acomplished by Monomorphization, that transcript your
    // generics into concret value-implementation at compiler time.

}