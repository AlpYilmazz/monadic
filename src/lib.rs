
pub mod input;
pub mod render;

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn it_works() {
        let sprite = render::Sprite::load("res/ball.txt");
        println!("{}", sprite);

        let mut display_buffer = render::DisplayBuffer::init(20, 20);

        println!("{}", display_buffer);

        display_buffer.render(render::Coord(2, 2), &sprite);

        println!("{}", display_buffer);
    }
}
