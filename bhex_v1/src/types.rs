#[derive(Debug, Clone, PartialEq)]

pub enum AST {
    Psi(Box<AST>),
    Mahlo(Box<AST>),
    Add(Box<AST>, Box<AST>),
    Zero,
}

impl AST {
    pub fn one() -> Self {
        Self::Psi(Self::Zero.to_box())
    }
    pub fn omega() -> Self {
        Self::Psi(Self::one().to_box())
    }
    pub fn aleph() -> Self {
        Self::Psi(Self::mahlo().to_box())
    }
    pub fn mahlo() -> Self {
        Self::Mahlo(Self::Zero.to_box())
    }
    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }
    pub fn _to_json(&self) -> String {
        match self {
            Self::Zero => "0".to_string(),
            Self::Add(a, b) => format!(
                r#"{{ type: "add", a: {}, b: {} }}"#,
                a._to_json(),
                b._to_json()
            ),
            Self::Psi(a) => format!(
                r#"{{ type: "psi", a: {} }}"#,
                a._to_json(),
            ),
            Self::Mahlo(a) => format!(
                r#"{{ type: "mahlo", a: {} }}"#,
                a._to_json(),
            ),
        }
    }
    pub fn to_string(&self) -> String {
        if let Some(n) = self.to_int() {
            format!("{}", n)
        } else {
            match self {
                Self::Zero => "0".to_string(),
                Self::Add(l, r) => format!("{} + {}", l.to_string(), r.to_string()),
                Self::Psi(a) => {
                    if self == &AST::one() {
                        "1".to_string()
                    } else if self == &AST::omega() {
                        "w".to_string()
                    } else if self == &AST::aleph() {
                        "O".to_string()
                    } else {
                        format!("p({})", a.to_string())
                    }
                }
                Self::Mahlo(a) => {
                    if a == &AST::one().to_box() {
                        "M".to_string()
                    } else {
                        format!("M({})", a.to_string())}
                }
            }
        }
    }
    pub fn is_successor(&self) -> bool {
        if let Self::Add(_, r) = self {
            **r == AST::one() || r.is_successor()
        } else {
            false
        }
    }
    pub fn is_non_zero(&self) -> bool {
        if let Self::Add(l, _) = self {
            **l == AST::one()
        } else if self == &AST::one() {
            true
        } else {
            false
        }
    }
    pub fn from_int(n: u32) -> Self {
        match n {
            0 => AST::Zero,
            1 => AST::one(),
            n => AST::Add(AST::one().to_box(), Self::from_int(n - 1).to_box()),
        }
    }
    pub fn to_int(&self) -> Option<u32> {
        if let Self::Add(l, r) = self {
            Some(l.to_int()? + r.to_int()?)
        } else if self == &AST::one() {
            Some(1)
        } else if self == &AST::Zero {
            Some(0)
        } else {
            None
        }
    }
}
