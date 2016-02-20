use aster::AstBuilder;
use syntax::ast;
use syntax::codemap::Span;
use syntax::ptr::P;

macro_rules! span_err {
    ($span:expr, $msg:expr) => {
        return Err(($span, $msg.to_owned()))
    };
    ($span:expr, $msg:expr, $($arg:tt)+) => {
        return Err(($span, format!($msg, $($arg)+)))
    };
}

#[derive(Debug, Clone)]
struct Method {
    attrs: Vec<ast::Attribute>,
    ident: ast::Ident,
    sig: ast::MethodSig,
    block: P<ast::Block>,
    span: Span,
}

#[derive(Debug, Clone)]
struct ImplData {
    generics: ast::Generics,
    trait_ref: ast::TraitRef,
    trait_name: String,
    ty: P<ast::Ty>,
    methods: Vec<Method>,
}

impl ImplData {
    fn new(generics: ast::Generics, trait_ref: ast::TraitRef, ty: P<ast::Ty>, items: Vec<ast::ImplItem>)
        -> Result<ImplData, (Span, String)>
    {
        let trait_name = match trait_ref.path.segments.last() {
            Some(segment) => segment.identifier.name.as_str().to_string(),
            None => span_err!(trait_ref.path.span, "Need a valid path for trait name"),
        };

        let mut methods = vec![];
        for item in items.into_iter() {
            let method = match item.node {
                ast::ImplItemKind::Method(sig, block) => Method {
                    attrs: item.attrs,
                    ident: item.ident,
                    sig: sig,
                    block: block,
                    span: item.span,
                },
                _ => span_err!(item.span, "Expected a method, got {:?}", item.node),
            };
            methods.push(method);
        }

        Ok(ImplData {
            generics: generics,
            trait_ref: trait_ref,
            trait_name: trait_name,
            ty: ty,
            methods: methods,
        })
    }

    fn trait_name(&self) -> &str {
        &self.trait_name
    }
}

pub struct Extender {
    data: ImplData,
    vis: ast::Visibility,
    span: Span,
}

impl Extender {
    pub fn new(item: ast::Item) -> Result<Extender, (Span, String)> {
        let data = match item.node {
            ast::ItemKind::Impl(_, _, generics, Some(trait_ref), ty, items)
                => try!(ImplData::new(generics, trait_ref, ty, items)),
            _ => span_err!(item.span, "Expected an `impl` item, found: {:?}", item.node),
        };

        Ok(Extender {
            data: data,
            vis: item.vis,
            span: item.span,
        })
    }

    pub fn extend(&self, items: &mut Vec<P<ast::Item>>) {
        // Create trait
        let mut trait_items = vec![];
        for method in self.data.methods.iter() {
            let trait_item = AstBuilder::new().trait_item(method.ident).span(method.span)
                                              .with_attrs(method.attrs.clone())
                                              .build_item(ast::TraitItemKind::Method(method.sig.clone(), None));
            trait_items.push(trait_item);
        }

        let mut builder = AstBuilder::new().item().span(self.span);
        if self.vis == ast::Visibility::Public {
            builder = builder.pub_();
        }

        let item = builder.trait_(self.data.trait_name())
                          .with_generics(self.data.generics.clone())
                          .with_items(trait_items)
                          .build();
        items.push(item);

        // Impl this trait to the specified type
        let mut impl_items = vec![];
        for method in self.data.methods.iter() {
            let impl_item = AstBuilder::new().impl_item(method.ident).span(method.span)
                                             .with_attrs(method.attrs.clone())
                                             .build_item(ast::ImplItemKind::Method(method.sig.clone(), method.block.clone()));
            impl_items.push(impl_item);
        }

        let item = AstBuilder::new().item().span(self.span).impl_()
                                    .with_generics(self.data.generics.clone())
                                    .with_items(impl_items)
                                    .with_trait(self.data.trait_ref.clone())
                                    .build_ty(self.data.ty.clone());
        items.push(item);
    }
}
