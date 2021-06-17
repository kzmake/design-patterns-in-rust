use rand::seq::SliceRandom;

pub struct Garapon<T> {
    balls: Vec<T>,
}

impl<T> Garapon<T> {
    pub fn new() -> Garapon<T> {
        Garapon { balls: vec![] }
    }

    pub fn push(&mut self, v: T) {
        self.balls.push(v);
    }
}

impl<T> Iterator for Garapon<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // NOTE: ガラポンって毎回シャッフルされるじゃん？そのシャッフル
        let mut rng = rand::thread_rng();
        self.balls.shuffle(&mut rng);

        return self.balls.pop();
    }
}
