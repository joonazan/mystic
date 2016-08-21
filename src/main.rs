mod power;
mod map;

fn main() {
}

type Sacrifices = [u8; 4];

struct Player {
	power: power::Power,
	built: Buildings,
	cult_points: [u8; 4],
}

struct Buildings {
	stronghold: bool,
	temples: u8,
	trading_posts: u8,
	dwellings: u8,
}