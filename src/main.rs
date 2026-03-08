#[derive(Debug)]

struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            hard_drive_capacity,
            memory,
        }
    }

    fn update_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn update_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}

impl TaylorSwiftSong {
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }
}

impl TaylorSwiftSong {
    fn display_song_info(self: &Self) {
        println!("Title: {}", self.title);
        println!("Release: {}", self.release_year);
        println!("Duration: {}", self.duration_secs);
        println!("Years since release: {}", self.years_since_release())
    }

    fn double_length(&mut self) {
        self.duration_secs = self.duration_secs * 2;
        println!("{:?}", self);
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        2026 - self.release_year
    }
}

fn main() {
    let mut computer = Computer::new(String::from("M3 max"), 64, 2);
    computer.update_cpu(String::from("M4 max"));
    computer.update_memory(128);

    computer
        .update_cpu(String::from("M5 max"))
        .update_memory(256)
        .upgrade_hard_drive_capacity(4);

    let mut blank_space = TaylorSwiftSong::new(String::from("Blank space"), 2014, 231);

    let all_too_well = TaylorSwiftSong {
        title: String::from("All Too Well"),
        release_year: 2015,
        duration_secs: 225,
    };

    blank_space.display_song_info();
    blank_space.double_length();
    let result = blank_space.is_longer_than(&all_too_well);
    println!("{}", result);

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
