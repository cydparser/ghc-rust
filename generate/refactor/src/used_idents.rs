use std::collections::HashSet;
use syn::{Ident, visit};

/// Collects `Ident`s that might need to be imported.
#[derive(Default)]
pub struct UsedIdents(HashSet<Ident>);

impl UsedIdents {
    pub fn idents(self) -> impl Iterator<Item = Ident> {
        self.0.into_iter()
    }

    fn insert(&mut self, ident: &Ident) {
        if !self.0.contains(ident) {
            self.0.insert(ident.clone());
        }
    }

    fn insert_path(&mut self, path: &syn::Path) {
        if path.leading_colon.is_none()
            && let Some(ident) = path.get_ident().or_else(|| {
                path.segments
                    .first()
                    .map(|ps| &ps.ident)
                    .filter(|&ident| ident != "std" && ident != "core")
            })
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
