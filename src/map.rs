type Map = [[Hex]];

pub enum Hex {
	Empty(Terrain),
	Built(Building, u8),
}

pub enum Terrain {
	Mountains,
	Wasteland,
	Desert,
	Plains,
	Swamp,
	Lakes,
	Forest,
	River,
}

pub enum Building {
	Stronghold,
	Temple,
	TradingPost,
	Dwelling,
}