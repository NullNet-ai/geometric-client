use geometric_server::GeometricClientImpl;

#[derive(Debug)]
pub enum Shape {
    Square(u64),
    Circle(u64),
}

impl Shape {
    pub async fn call_server(&self, client: &GeometricClientImpl) -> f32 {
        println!("Calling server from {self:?}");
        let res = match self {
            Shape::Square(a) => client.area_square(*a).await.unwrap() as f32,
            Shape::Circle(a) => client.area_circle(*a).await.unwrap(),
        };
        println!("Result: {res}");
        res
    }
}
