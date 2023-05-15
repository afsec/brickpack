use design_scaffold::AppResult;

#[derive(Debug, Default)]
pub struct WebServer {
    config: (),
}

impl WebServer {
    pub async fn new() -> AppResult<Self> {
        Ok(WebServer::default())
    }
    pub async fn run(self) -> AppResult<()> {
        dbg!(self);
        Ok(())
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
