use geometric_server::{AreaCircleMessage, AreaSquareMessage, GeometricClientImpl};

#[derive(Debug)]
pub enum Shape {
    Square(f32),
    Circle(f32),
}

impl Shape {
    pub async fn call_server(&self, client: &GeometricClientImpl) -> f32 {
        println!("Calling server with {self:?}");
        let res = match self {
            Shape::Square(a) => {
                let message = AreaSquareMessage { base: *a };
                client.area_square(message).await.unwrap()
            }
            Shape::Circle(a) => {
                let message = AreaCircleMessage { radius: *a };
                client.area_circle(message).await.unwrap()
            }
        }
        .value;
        println!("Result: {res}");
        res
    }
}
