use rhai::{CustomType, Dynamic, Engine, EvalAltResult, Position, TypeBuilder, INT};

// Sanity check to make sure everything compiles

#[derive(Clone, CustomType)]
pub struct Bar(
    #[rhai_type(skip)] rhai::FLOAT,
    INT,
    #[rhai_type(name = "boo", readonly)] String,
    Vec<INT>,
);

#[derive(Clone, Default, CustomType)]
#[rhai_type(name = "MyFoo", extra = Self::build_extra)]
pub struct Foo {
    #[rhai_type(skip)]
    _dummy: rhai::FLOAT,
    #[rhai_type(get = get_bar)]
    pub bar: INT,
    #[rhai_type(name = "boo", readonly)]
    pub(crate) baz: String,
    #[rhai_type(set = Self::set_qux)]
    pub qux: Vec<INT>,
    pub maybe: Option<INT>,
}

impl Foo {
    pub fn set_qux(&mut self, value: Vec<INT>) {
        self.qux = value;
    }

    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder.with_fn("new_foo", Self::default);
    }
}

fn get_bar(_this: &Foo) -> INT {
    42
}

#[test]
fn test() {
    let mut engine = Engine::new();
    engine.build_type::<Foo>().build_type::<Bar>();

    assert_eq!(
        engine
            .eval::<INT>(
                "
                    let foo = new_foo();
                    foo.bar = 42;
                    foo.bar
                "
            )
            .unwrap(),
        42
    );
}
