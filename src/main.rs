#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs:u32,
}

impl TaylorSwiftSong {
    fn display_song_info(self: Self){
                println!("Title: {}", self.title);
                println!("Release: {}", self.release_year);
                println!("Duration: {}", self.duration_secs);
    }
}

fn main() {

    let song = TaylorSwiftSong{
        title: String::from("Blank space"),
        release_year: 2014,
        duration_secs:231
    };

    song.display_song_info();
    
    let mut mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    println!("{:?}", mocha);
    println!("{:#?}", mocha);

    mocha.name = String::from("Caramel Macchiato");
    mocha.price = 6.99;
    mocha.is_hot = true;

    println!("{}, {}, {}", mocha.name, mocha.price, mocha.is_hot);

    println!("{}", mocha.name);

    let favorite_coffe = mocha.name;

    println!("{favorite_coffe}");

    // will not work
    //println!("{}", mocha.name);
    let name = String::from("Latte");
    let mut latte: Coffee = make_coffee(name, 4.99, true);
    drink_coffee(&mut latte);
    //println!("{}", latte.price);
}

fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", (*coffee).name);
    coffee.is_hot = false;
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}
