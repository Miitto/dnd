pub trait SnakeCase {
    fn to_snake_case(&self) -> String;
}

impl SnakeCase for &str {
    fn to_snake_case(&self) -> String {
        let mut snake = String::new();
        let mut last = '_';

        for c in self.chars() {
            if c.is_whitespace() {
                if last != '_' {
                    snake.push('_');
                }
            } else if c.is_uppercase() {
                if last != '_' {
                    snake.push('_');
                }
                snake.push(c.to_lowercase().next().unwrap());
            } else {
                snake.push(c);
            }

            last = c;
        }

        snake
    }
}

impl SnakeCase for String {
    fn to_snake_case(&self) -> String {
        self.as_str().to_snake_case()
    }
}
