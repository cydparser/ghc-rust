use syn::{Expr, ExprLit, Lit, Variant};

/// Produce an `Iterator` over the Variant's integer discriminants.
pub fn variant_discriminants<'a, I>(variants: I) -> impl Iterator<Item = (isize, &'a Variant)>
where
    I: IntoIterator<Item = &'a Variant>,
{
    let mut next_disc: isize = 0;

    variants.into_iter().map(move |v| {
        let disc = match &v.discriminant {
            Some((
                _,
                Expr::Lit(ExprLit {
                    lit: Lit::Int(int), ..
                }),
            )) => int.base10_parse().unwrap(),
            _ => next_disc,
        };
        next_disc += 1;

        (disc, v)
    })
}
