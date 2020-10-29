use fast_async_mutex::mutex::Mutex;
pub use http_types::{Request, Response, Result, Url};
use path_tree::PathTree;

pub struct Handler {
    routes: Mutex<PathTree<&'static str>>,
}

impl Handler {
    pub fn new() -> Self {
        let mut r = PathTree::new();
        r.insert("/_plugins", "plugins");
        Handler {
            routes: Mutex::new(r),
        }
    }

    pub async fn handle_request(&self, req: Request) -> Result<Response> {
        let routes = self.routes.lock().await;
        match routes.find(req.url().path()) {
            Some((plugin, _)) => println!("Matched plugin {}", plugin),
            None => println!("No plugin matched"),
        }
        Ok(().into())
    }
}
