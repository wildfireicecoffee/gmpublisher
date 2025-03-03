// https://github.com/garrynewman/bootil/blob/beb4cec8ad29533965491b767b177dc549e62d23/src/3rdParty/globber.cpp
// https://github.com/Facepunch/gmad/blob/master/include/AddonWhiteList.h

macro_rules! nul_str {
	( $str:literal ) => {
		concat!($str, "\0")
	};
}

pub const DEFAULT_IGNORE: &'static [&'static str] = &[
	nul_str!(".git/*"),
	nul_str!("*.psd"),
	nul_str!("*.pdn"),
	nul_str!("*.xcf"),
	nul_str!("*.svn"),
	nul_str!(".gitignore"),
	nul_str!(".vscode/*"),
	nul_str!(".github/*"),
	nul_str!(".editorconfig"),
	nul_str!("README.md"),
	nul_str!("README.txt"),
	nul_str!("readme.txt"),
	nul_str!("addon.json"),
	nul_str!("addon.txt"),
	nul_str!("addon.jpg"),
];

const ADDON_WHITELIST: &'static [&'static str] = &[
	nul_str!("lua/*.lua"),
	nul_str!("scenes/*.vcd"),
	nul_str!("particles/*.pcf"),
	nul_str!("resource/fonts/*.ttf"),
	nul_str!("scripts/vehicles/*.txt"),
	nul_str!("resource/localization/*/*.properties"),
	nul_str!("maps/*.bsp"),
	nul_str!("maps/*.nav"),
	nul_str!("maps/*.ain"),
	nul_str!("maps/thumb/*.png"),
	nul_str!("sound/*.wav"),
	nul_str!("sound/*.mp3"),
	nul_str!("sound/*.ogg"),
	nul_str!("materials/*.vmt"),
	nul_str!("materials/*.vtf"),
	nul_str!("materials/*.png"),
	nul_str!("materials/*.jpg"),
	nul_str!("materials/*.jpeg"),
	nul_str!("models/*.mdl"),
	nul_str!("models/*.vtx"),
	nul_str!("models/*.phy"),
	nul_str!("models/*.ani"),
	nul_str!("models/*.vvd"),
	nul_str!("gamemodes/*/*.txt"),
	nul_str!("gamemodes/*/*.fgd"),
	nul_str!("gamemodes/*/logo.png"),
	nul_str!("gamemodes/*/icon24.png"),
	nul_str!("gamemodes/*/gamemode/*.lua"),
	nul_str!("gamemodes/*/entities/effects/*.lua"),
	nul_str!("gamemodes/*/entities/weapons/*.lua"),
	nul_str!("gamemodes/*/entities/entities/*.lua"),
	nul_str!("gamemodes/*/backgrounds/*.png"),
	nul_str!("gamemodes/*/backgrounds/*.jpg"),
	nul_str!("gamemodes/*/backgrounds/*.jpeg"),
	nul_str!("gamemodes/*/content/models/*.mdl"),
	nul_str!("gamemodes/*/content/models/*.vtx"),
	nul_str!("gamemodes/*/content/models/*.phy"),
	nul_str!("gamemodes/*/content/models/*.ani"),
	nul_str!("gamemodes/*/content/models/*.vvd"),
	nul_str!("gamemodes/*/content/materials/*.vmt"),
	nul_str!("gamemodes/*/content/materials/*.vtf"),
	nul_str!("gamemodes/*/content/materials/*.png"),
	nul_str!("gamemodes/*/content/materials/*.jpg"),
	nul_str!("gamemodes/*/content/materials/*.jpeg"),
	nul_str!("gamemodes/*/content/scenes/*.vcd"),
	nul_str!("gamemodes/*/content/particles/*.pcf"),
	nul_str!("gamemodes/*/content/resource/fonts/*.ttf"),
	nul_str!("gamemodes/*/content/scripts/vehicles/*.txt"),
	nul_str!("gamemodes/*/content/resource/localization/*/*.properties"),
	nul_str!("gamemodes/*/content/maps/*.bsp"),
	nul_str!("gamemodes/*/content/maps/*.nav"),
	nul_str!("gamemodes/*/content/maps/*.ain"),
	nul_str!("gamemodes/*/content/maps/thumb/*.png"),
	nul_str!("gamemodes/*/content/sound/*.wav"),
	nul_str!("gamemodes/*/content/sound/*.mp3"),
	nul_str!("gamemodes/*/content/sound/*.ogg"),
];

const WILD_BYTE: u8 = '*' as u8;
const QUESTION_BYTE: u8 = '?' as u8;

pub unsafe fn globber(_wild: &str, _str: &str) -> bool {
	let mut cp: *const u8 = 0 as u8 as *const u8;
	let mut mp: *const u8 = 0 as u8 as *const u8;

	let mut wild = _wild.as_ptr();
	let mut str = _str.as_ptr();

	while *str != 0 && *wild != WILD_BYTE {
		if *wild != *str && *wild != QUESTION_BYTE {
			return false;
		}
		wild = wild.add(1);
		str = str.add(1);
	}

	while *str != 0 {
		if *wild == WILD_BYTE {
			wild = wild.add(1);
			if *wild == 0 {
				return true;
			}
			mp = wild;
			cp = str.add(1);
		} else if *wild == *str || *wild == QUESTION_BYTE {
			wild = wild.add(1);
			str = str.add(1);
		} else {
			wild = mp;
			cp = cp.add(1);
			str = cp;
		}
	}

	while *wild == WILD_BYTE {
		wild = wild.add(1);
	}
	*wild == 0
}

pub fn check<S: Into<String> + Clone>(str: &S) -> bool {
	let mut string = str.clone().into();
	string.push('\0');

	let str = string.as_str();
	for glob in ADDON_WHITELIST {
		if unsafe { globber(glob, str) } {
			return true;
		}
	}

	false
}

pub fn filter_default_ignored<S: Into<String> + Clone>(str: &S) -> bool {
	let mut string = str.clone().into();
	string.push('\0');

	let str = string.as_str();
	for glob in DEFAULT_IGNORE {
		if unsafe { globber(glob, str) } {
			return false;
		}
	}

	true
}

pub fn is_ignored<S: Into<String> + Clone>(str: &S, ignore: &[String]) -> bool {
	if ignore.is_empty() {
		return false;
	}

	let mut string = str.clone().into();
	string.push('\0');

	let str = string.as_str();
	for glob in ignore {
		debug_assert!(glob.ends_with('\0'));
		if unsafe { globber(glob, str) } {
			return true;
		}
	}

	false
}

#[test]
pub fn test_whitelist() {
	let good: &'static [&'static str] = &[
		"lua/test.lua",
		"lua/lol/test.lua",
		"lua/lua/testing.lua",
		"gamemodes/test/something.txt",
		"gamemodes/test/content/sound/lol.wav",
		"materials/lol.jpeg",
	];

	let bad: &'static [&'static str] = &[
		"test.lua",
		"lua/test.exe",
		"lua/lol/test.exe",
		"gamemodes/test",
		"gamemodes/test/something",
		"gamemodes/test/something/something.exe",
		"gamemodes/test/content/sound/lol.vvv",
		"materials/lol.vvv",
	];

	for good in good {
		assert!(check(&*good));
	}

	for good in ADDON_WHITELIST {
		assert!(check(&good.replace('*', "test")));
	}

	for bad in bad {
		assert!(!check(&*bad));
	}
}

#[test]
pub fn test_ignore() {
	let ignored: &'static [&'static str] = &[
		".git/index",
		".git/info/exclude",
		".git/logs/head",
		".git/logs/refs/heads/4.0.0",
		".git/logs/refs/heads/master",
		".git/logs/refs/remotes/origin/4.0.0",
		".git/logs/refs/remotes/origin/cracker",
		".git/logs/refs/remotes/origin/cracker-no-minigames",
		".git/logs/refs/remotes/origin/master",
		".git/objects/00/007c75922055623f4177467fd50a7d573c2c86",
		"blah.psd",
		"some/location/blah.psd",
		"some/blah/blah.pdn",
		"hi.xcf",
		"addon.jpg",
		"addon.json"
	];

	for ignored in ignored {
		assert!(!filter_default_ignored(&*ignored));
	}

	let default_ignore: Vec<String> = DEFAULT_IGNORE.iter().cloned().map(|x| x.to_string()).collect();
	for ignored in ignored {
		assert!(is_ignored(&*ignored, &default_ignore));
	}

	assert!(is_ignored(&"lol.txt".to_string(), &["lol.txt\0".to_string()]));
	assert!(!is_ignored(&"lol.txt".to_string(), &[]));
}
