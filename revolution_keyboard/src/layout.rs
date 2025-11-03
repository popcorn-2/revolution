use std::borrow::Cow;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Layout<'s> {
	#[serde(borrow)]
	name: Cow<'s, str>,
	#[serde(borrow)]
	author: Cow<'s, str>,

	#[serde(rename = "key")]
	#[serde(deserialize_with = "vec_to_map")]
	#[serde(borrow)]
	pub(crate) keys: HashMap<u16, Key<'s>>,
	pub(crate) rules: HashMap<RuleTy, Ruleset>,
	#[serde(borrow)]
	pub(crate) deadkeys: HashMap<Cow<'s, str>, DeadkeyTable<'s>>,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RuleTy {
	Alpha,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Ruleset {
	pub(crate) shift: Option<Rule>,
	pub(crate) caps: Option<Rule>,
	pub(crate) alt: Option<Rule>,
	pub(crate) alt_shift: Option<Rule>,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Rule {
	Uppercase,
	Lowercase,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct DeadkeyTable<'s> {
	#[serde(borrow)]
	pub(crate) fallback: Cow<'s, str>,

	#[serde(flatten)]
	#[serde(borrow)]
	pub(crate) keys: HashMap<Cow<'s, str>, Cow<'s, str>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub(crate) enum Key<'s> {
	Action {
		code: u16,
		action: Action,
	},
	Key {
		code: u16,
		#[serde(borrow)]
		base: Option<KeyTy<'s>>,
		#[serde(borrow)]
		shift: Option<KeyTy<'s>>,
		#[serde(borrow)]
		alt: Option<KeyTy<'s>>,
		#[serde(borrow)]
		alt_shift: Option<KeyTy<'s>>,
		#[serde(borrow)]
		caps: Option<KeyTy<'s>>,
	},
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum KeyTy<'s> {
	Normal(#[serde(borrow)] Cow<'s, str>),
	Dead { #[serde(borrow)] dead: Cow<'s, str> },
}

fn vec_to_map<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<HashMap<u16, Key<'de>>, D::Error> {
	let vec = Vec::<Key<'de>>::deserialize(deserializer)?;
	Ok(vec.into_iter().map(|k| {
		let code = match &k {
			Key::Key { code, .. } => *code,
			Key::Action { code, .. } => *code,
		};
		(code, k)
	}).collect())
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Action {
	Escape,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	Backspace,
	Tab,
	Enter,
	Ctrl,
	Shift,
	Cmd,
	Alt,
	Compose,
	Caps,
	ArrowLeft,
	ArrowUp,
	ArrowRight,
	ArrowDown,
	Home,
	End,
	Pgup,
	Pgdown,
}
