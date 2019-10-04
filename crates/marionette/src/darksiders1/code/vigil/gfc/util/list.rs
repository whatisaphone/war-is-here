use darksiders1_sys::target;

#[repr(C)]
pub struct List<'a, T> {
    list: &'a List__ListNode<'a, T>,
    tail: &'a List__ListNode<'a, T>,
    size: i32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct List__ListNode<'a, T> {
    next: Option<&'a List__ListNode<'a, T>>,
    value: T,
}

impl<'a, T> List<'a, T> {
    pub unsafe fn from_ptr(inner: *mut target::List_gfc__AutoRef_gfc__WorldObject___) -> &'a Self {
        &*(inner as *const _)
    }

    pub fn iter(&'a self) -> List__Iterator<'a, T> {
        List__Iterator::new(self)
    }
}

impl<'a, T> IntoIterator for &'a List<'a, T> {
    type Item = &'a T;
    type IntoIter = List__Iterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        List__Iterator::new(self)
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct List__Iterator<'a, T> {
    node: Option<&'a List__ListNode<'a, T>>,
    prv: Option<&'a List__ListNode<'a, T>>,
    list: &'a List<'a, T>,
}

impl<'a, T> List__Iterator<'a, T> {
    pub fn new(l: &'a List<'a, T>) -> Self {
        Self {
            node: Some(l.list),
            prv: None,
            list: l,
        }
    }
}

impl<'a, T> Iterator for List__Iterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        let result = &node.value;
        self.prv = Some(node);
        self.node = node.next;
        Some(result)
    }
}
