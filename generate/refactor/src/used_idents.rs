use std::collections::HashSet;
use std::mem;
use syn::{Ident, UseTree, visit};

/// Collects `Ident`s that might need to be imported.
#[derive(Default)]
pub struct UsedIdents(HashSet<Ident>);

impl UsedIdents {
    pub fn contains(&self, ident: &Ident) -> bool {
        self.0.contains(ident)
    }

    pub fn filter_used(&self, mut item_use: syn::ItemUse) -> Option<syn::ItemUse> {
        self.keep_use_tree(&mut item_use.tree).then_some(item_use)
    }

    fn keep_use_tree(&self, mut use_tree: &mut UseTree) -> bool {
        loop {
            match use_tree {
                UseTree::Path(use_path) => {
                    use_tree = &mut *use_path.tree;
                }
                UseTree::Name(use_name) => {
                    break self.contains(&use_name.ident);
                }
                UseTree::Rename(use_rename) => break self.contains(&use_rename.rename),
                UseTree::Glob(_) => break true,
                UseTree::Group(use_group) => {
                    let mut trees: Vec<UseTree> = Vec::with_capacity(use_group.items.len());

                    for mut tree in mem::take(&mut use_group.items) {
                        if self.keep_use_tree(&mut tree) {
                            trees.push(tree);
                        }
                    }

                    if trees.is_empty() {
                        break false;
                    } else {
                        use_group.items = trees.into_iter().collect();

                        break true;
                    }
                }
            }
        }
    }

    fn insert(&mut self, ident: &Ident) {
        if !self.0.contains(ident) {
            self.0.insert(ident.clone());
        }
    }

    fn insert_path(&mut self, path: &syn::Path) {
        if path.leading_colon.is_none()
            && let Some(ident) = path.segments.first().map(|ps| &ps.ident)
        {
            self.insert(ident);
        }
    }
}

impl<'ast> visit::Visit<'ast> for UsedIdents {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        self.insert(&i.method);

        visit::visit_expr_method_call(self, i);
    }

    fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
        self.insert_path(&i.path);
    }

    fn visit_expr_struct(&mut self, i: &'ast syn::ExprStruct) {
        self.insert_path(&i.path);

        visit::visit_expr_struct(self, i);
    }

    fn visit_item_macro(&mut self, _: &'ast syn::ItemMacro) {
        // No-op
    }

    fn visit_item_use(&mut self, _: &'ast syn::ItemUse) {
        // No-op
    }

    fn visit_pat_struct(&mut self, i: &'ast syn::PatStruct) {
        self.insert_path(&i.path);

        visit::visit_pat_struct(self, i);
    }
    fn visit_pat_tuple_struct(&mut self, i: &'ast syn::PatTupleStruct) {
        self.insert_path(&i.path);

        visit::visit_pat_tuple_struct(self, i);
    }

    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        self.insert_path(&i.path);
    }
}
