use std::slice::Iter;

pub struct Route(Vec<String>);

impl Route {
    pub fn new(string: &str) -> Route {
        let route = string.split('/')
            .filter(|p| p != &"")
            .map(|p| p.to_owned())
            .collect::<Vec<String>>();

        Route(route)
    }

    pub fn iter(&self) -> Iter<'_, String> {
        self.0.iter()
    }

    pub fn to_vec(self) -> Vec<String> {
        self.0
    }
}
