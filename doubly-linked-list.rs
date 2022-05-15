use std::{marker::PhantomData, ptr};

mod pre_implemented;

pub struct Node<T> {
    data: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    curr: *mut Node<T>,
}

pub struct Iter<'a, T> {
    curr: *mut Node<T>,
    _marker: PhantomData<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let head_ptr: *mut _ = self.head;
        Cursor {
            list: self,
            curr: head_ptr,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let tail_ptr: *mut _ = self.tail;
        Cursor {
            list: self,
            curr: tail_ptr,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            curr: self.head,
            _marker: PhantomData,
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.curr.is_null() {
            return None;
        }

        unsafe { Some(&mut (*self.curr).data) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if self.curr.is_null() || (*self.curr).next.is_null() {
                return None;
            }

            self.curr = (*self.curr).next;
            Some(&mut (*self.curr).data)
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            if self.curr.is_null() || (*self.curr).prev.is_null() {
                return None;
            }

            self.curr = (*self.curr).prev;
            Some(&mut (*self.curr).data)
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let curr_ptr = self.curr;

        unsafe {
            let next_node_ptr = (*curr_ptr).next;
            let prev_node_ptr = (*curr_ptr).prev;

            if !next_node_ptr.is_null() && prev_node_ptr.is_null() {
                // Head
                (*next_node_ptr).prev = ptr::null_mut();
                self.list.head = next_node_ptr;
                self.curr = next_node_ptr;
            } else if next_node_ptr.is_null() && !prev_node_ptr.is_null() {
                // Tail
                (*prev_node_ptr).next = ptr::null_mut();
                self.list.tail = prev_node_ptr;
                self.curr = prev_node_ptr;
            } else if !next_node_ptr.is_null() && !prev_node_ptr.is_null() {
                // Middle
                (*prev_node_ptr).next = next_node_ptr;
                (*next_node_ptr).prev = prev_node_ptr;
                self.curr = next_node_ptr;
            } else {
                // Only one
                self.curr = ptr::null_mut();
                self.list.head = ptr::null_mut();
                self.list.tail = ptr::null_mut();
            }

            self.list.len -= 1;

            let data = std::ptr::read(&(*curr_ptr).data);
            drop(Box::from_raw(curr_ptr));

            Some(data)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let new_node = Box::new(Node {
            data: element,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        });

        let new_node_ptr: *mut _ = Box::into_raw(new_node);

        if !self.curr.is_null() {
            unsafe {
                (*new_node_ptr).prev = self.curr;
                let next_node_ptr = (*self.curr).next;

                if !next_node_ptr.is_null() {
                    (*new_node_ptr).next = next_node_ptr;
                    (*next_node_ptr).prev = new_node_ptr;
                } else {
                    self.list.tail = new_node_ptr;
                }

                (*self.curr).next = new_node_ptr;
            }
        } else {
            self.list.head = new_node_ptr;
            self.list.tail = new_node_ptr;
            self.curr = new_node_ptr;
        }

        self.list.len += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        let new_node = Box::new(Node {
            data: element,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        });

        let new_node_ptr: *mut _ = Box::into_raw(new_node);

        if !self.curr.is_null() {
            unsafe {
                (*new_node_ptr).next = self.curr;
                let prev_node_ptr = (*self.curr).prev;

                if !prev_node_ptr.is_null() {
                    (*new_node_ptr).prev = prev_node_ptr;
                    (*prev_node_ptr).next = new_node_ptr;
                } else {
                    self.list.head = new_node_ptr;
                }

                (*self.curr).prev = new_node_ptr;
            }
        } else {
            self.list.head = new_node_ptr;
            self.list.tail = new_node_ptr;
            self.curr = new_node_ptr;
        }

        self.list.len += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            if self.curr.is_null() {
                return None;
            }

            let data = &(*self.curr).data;
            self.curr = (*self.curr).next;
            Some(data)
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr_ptr = self.head;

        while !curr_ptr.is_null() {
            unsafe {
                let next_ptr = (*curr_ptr).next;
                drop(Box::from_raw(curr_ptr));
                curr_ptr = next_ptr;
            }
        }
    }
}
