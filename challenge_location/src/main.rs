enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl Location {
    fn display(&self){
        match self {
        Location::Unknown => {println!("The location is unknown.");}
        Location::Anonymous => {println!("The location is anonymous.")}
        Location::Known(x,y) => {println!("The location is at coordinates {},{}.",x,y)}
    }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
