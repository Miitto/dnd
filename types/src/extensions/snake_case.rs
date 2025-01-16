pub trait SnakeCase {
    fn to_snake_case(&self) -> String;
}

impl SnakeCase for &str {
    fn to_snake_case(&self) -> String {
        let mut snake = String::new();
        let mut first = true;

        let mut should_add_underscore = |snake: &str| {
            if first {
                first = false;
                return false;
            }

            if snake.chars().last().filter(|c| c == &'_').is_some() {
                return false;
            }

            true
        };

        for c in self.chars() {
            if c.is_whitespace() {
                if should_add_underscore(snake.as_str()) {
                    snake.push('_');
                }
            } else if c.is_uppercase() {
                if should_add_underscore(snake.as_str()) {
                    snake.push('_');
                }
                snake.push(c.to_lowercase().next().unwrap());
            } else {
                snake.push(c);
            }
        }

        snake
    }
}

impl SnakeCase for String {
    fn to_snake_case(&self) -> String {
        self.as_str().to_snake_case()
    }
}
