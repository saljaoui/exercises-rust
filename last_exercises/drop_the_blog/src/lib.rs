use std::cell::{RefCell, Cell};
use std::borrow::Borrow;
use std::borrow::BorrowMut;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Blog {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Blog {
    pub fn new() -> Blog {
        Blog {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new())
        }
    }
    pub fn new_article(&self, body: String) -> (usize, Article) {
        let id = self.new_id();
        (id , Article::new(id, body, self))
    }

    pub fn new_id(&self) -> usize {
        self.states.borrow_mut().push(false);
        self.states.borrow().len() - 1
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }


    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id) {
            panic!("{} is already dropped", id)
        }
        self.states.borrow_mut()[id] = true;
        let num = self.drops.get();
        self.drops.set(num + 1);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Article<'a> {
    id: usize,
    body: String, 
    blog: &'a Blog
}

impl<'a> Article<'a> {
    pub fn new(id: usize, body: String, blog: &'a Blog) -> Article {
        Article {
            id: id,
            body: body, 
            blog: blog
        }
    }

    pub fn discard(self) {
        self.blog.add_drop(self.id);
    }
}