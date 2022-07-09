macro_rules! id {
    ($generator:ident[$ident:ident]: $ty:ty) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $ident($ty);

        impl Into<$ty> for $ident {
            fn into(self) -> $ty {
                self.0
            }
        }

        #[derive(Clone, Debug, Default)]
        pub struct $generator {
            next_id: $ty,
        }

        impl $generator {
            pub fn generate(&mut self) -> $ident {
                let id = $ident(self.next_id);
                self.next_id += 1;
                id
            }
        }
    };
}

id!(FunctionIds[FunctionId]: usize);
id!(FunctionSignatureIds[FunctionSignatureId]: usize);
id!(TypeIds[TypeId]: usize);
id!(ModuleIds[ModuleId]: usize);
id!(StructIds[StructId]: usize);
id!(BlockIds[BlockId]: usize);
