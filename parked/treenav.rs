use std::{
  ops::{Deref, DerefMut},
  sync::{Arc, RwLock},
};

pub(crate) struct Tree<T> {
  value: Arc<RwLock<T>>,
  children: Vec<Arc<Tree<T>>>,
}

impl<T> Tree<T> {
  pub(crate) fn leaf(t: T) -> Arc<Tree<T>> {
    Arc::new(Tree {
      value: Arc::new(RwLock::new(t)),
      children: Vec::new(),
    })
  }

  pub(crate) fn node(t: T, children: Vec<Arc<Tree<T>>>) -> Arc<Tree<T>> {
    Arc::new(Tree {
      value: Arc::new(RwLock::new(t)),
      children: children.into_iter().collect(),
    })
  }
}

pub(crate) struct TreeNav<T> {
  roots: Vec<Arc<Tree<T>>>,
  stack: Vec<(Vec<Arc<Tree<T>>>, usize)>,
  cursor: usize,
}

impl<T> TreeNav<T> {
  pub(crate) fn new(t: Vec<Arc<Tree<T>>>) -> TreeNav<T> {
    TreeNav {
      roots: t,
      stack: Vec::new(),
      cursor: 0,
    }
  }

  pub(crate) fn get(&self) -> impl Deref<Target = T> + '_ {
    self.roots[self.cursor].value.read().unwrap()
  }

  pub(crate) fn get_mut(&self) -> impl DerefMut<Target = T> + '_ {
    self.roots[self.cursor].value.write().unwrap()
  }

  pub(crate) fn has_children(&self) -> bool {
    !self.roots[self.cursor].children.is_empty()
  }

  pub(crate) fn enter(&mut self) {
    if self.has_children() {
      self.stack.push((self.roots.clone(), self.cursor));
      self.roots = self.roots[self.cursor].children.clone();
      self.cursor = 0;
    }
  }

  pub(crate) fn exit(&mut self) {
    if let Some((tree, cursor)) = self.stack.pop() {
      self.roots = tree;
      self.cursor = cursor;
    }
  }

  pub(crate) fn next(&mut self) {
    self.cursor = usize::min(self.roots.len() - 1, self.cursor + 1);
  }

  pub(crate) fn prev(&mut self) {
    self.cursor = self.cursor.saturating_sub(1);
  }

  pub(crate) fn iter(&self) -> impl Iterator<Item = (impl Deref<Target = T> + '_, bool)> {
    self
      .roots
      .iter()
      .enumerate()
      .map(move |(i, v)| (v.value.read().unwrap(), i == self.cursor))
  }

  pub(crate) fn iter_mut(&self) -> impl Iterator<Item = (impl DerefMut<Target = T> + '_, bool)> {
    self
      .roots
      .iter()
      .enumerate()
      .map(move |(i, v)| (v.value.write().unwrap(), i == self.cursor))
  }
}
