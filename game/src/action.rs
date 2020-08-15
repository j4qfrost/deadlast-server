#[derive(Copy, Clone, Default, Debug, PartialEq, Deserialize, Serialize)]
pub struct Action<S, T>(S, String, pub T);

impl<S, T> Action<S, T> {
	pub fn new(actor: S, name: String, thing: T) -> Self {
		Self(actor, name, thing)
	}
}