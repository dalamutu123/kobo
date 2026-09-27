// GIVEN: do not edit. Read it anyway; what you write has to fit it.

use std::collections::HashMap;

use crate::value::Value;

/// Kobo's scope stack: one map per scope, in a plain `Vec`. Entering a block pushes a
/// map and leaving pops it; a name is looked up innermost first, which is what makes
/// shadowing work. A `Vec` is enough only because Kobo has no closures (spec section 4.3).
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
    /// Where each active call's scopes begin. A function sees its own scopes and the
    /// globals, nothing between (spec section 4.3): the caller's locals are still on the
    /// stack, and the frame base is what hides them.
    frames: Vec<usize>,
}

impl Environment {
    /// One scope, the global one, which is never popped.
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
            frames: Vec::new(),
        }
    }

    /// Enter a call. The caller's scopes stay hidden until `pop_frame`.
    pub fn push_frame(&mut self) {
        self.frames.push(self.scopes.len());
        self.scopes.push(HashMap::new());
    }

    /// Leave a call, discarding every scope it pushed, including any left open by a
    /// `return` from inside a block.
    pub fn pop_frame(&mut self) {
        if let Some(base) = self.frames.pop() {
            self.scopes.truncate(base);
        }
    }

    /// How many calls are active. The interpreter caps it (spec section 5.1).
    pub fn frame_depth(&self) -> usize {
        self.frames.len()
    }

    /// The lowest scope the running code may see; zero at the top level.
    fn frame_base(&self) -> usize {
        *self.frames.last().unwrap_or(&0)
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Bind `name` in the innermost scope. A binding of the same name in that scope is
    /// replaced (spec section 4.2); one in an enclosing scope is shadowed.
    pub fn declare(&mut self, name: &str, value: Value) {
        self.innermost().insert(name.to_string(), value);
    }

    /// Read a local the resolver placed. `depth` counts scopes out from the innermost; no
    /// search is needed, because the resolver walked these same scopes.
    pub fn get_at(&self, depth: usize, name: &str) -> Option<&Value> {
        let i = self.scopes.len().checked_sub(depth + 1)?;
        self.scopes[i].get(name)
    }

    /// Assign to a local the resolver placed. `false` only if the binding is gone, which
    /// resolution makes impossible.
    pub fn assign_at(&mut self, depth: usize, name: &str, value: Value) -> bool {
        match self.scopes.len().checked_sub(depth + 1) {
            Some(i) => match self.scopes[i].get_mut(name) {
                Some(slot) => {
                    *slot = value;
                    true
                }
                None => false,
            },
            None => false,
        }
    }

    /// Read a global. Still a lookup by name: a global may be used before its declaration
    /// runs, which is what makes mutual recursion work.
    pub fn get_global(&self, name: &str) -> Option<&Value> {
        self.scopes[0].get(name)
    }

    /// Assign to a global. `false` if it was never declared: only `var` creates one.
    pub fn assign_global(&mut self, name: &str, value: Value) -> bool {
        match self.scopes[0].get_mut(name) {
            Some(slot) => {
                *slot = value;
                true
            }
            None => false,
        }
    }

    /// Search the current frame's scopes innermost first, then the globals. For
    /// `evaluate` written before the resolver; unused once `resolver.rs` is.
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&Value> {
        let base = self.frame_base();
        for scope in self.scopes[base..].iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        if base > 0 {
            return self.scopes[0].get(name);
        }
        None
    }

    /// Assign by search, like `get`. `false` if the name was never declared.
    #[allow(dead_code)]
    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        let base = self.frame_base();
        for scope in self.scopes[base..].iter_mut().rev() {
            if let Some(slot) = scope.get_mut(name) {
                *slot = value;
                return true;
            }
        }
        if base > 0 {
            if let Some(slot) = self.scopes[0].get_mut(name) {
                *slot = value;
                return true;
            }
        }
        false
    }

    fn innermost(&mut self) -> &mut HashMap<String, Value> {
        self.scopes
            .last_mut()
            .expect("the global scope is never popped")
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}
