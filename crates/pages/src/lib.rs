use std::collections::HashMap;
use renderer::window::Window;

pub trait Page {
    fn name(&self) -> String;
    fn get_window(&self) -> Window;
}

#[derive(PartialEq, Eq, Hash)]
pub struct PageId(pub u32);

pub struct PageManager {
    pub pages: HashMap<PageId, Box<dyn Page>>,
    pub current_page: PageId,
    pub current_id: PageId,
}

impl storage::Node for PageManager {}

impl PageManager {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
            // Default page is the first one
            current_page: PageId(0),
            current_id: PageId(0),
        }
    }

    pub fn new_page(&mut self, page: impl Page + 'static) {
        self.pages.insert(PageId(self.current_id.0 + 1), Box::new(page));
    }

    pub fn change_page(&mut self, page_id: u32) {
        self.current_page = PageId(page_id);
    } 
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
       assert_eq!(4, 4);
    }
}
