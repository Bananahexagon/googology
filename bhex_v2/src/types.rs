#[derive(Debug, Clone, PartialEq)]

pub enum AST {
    Psi(Box<AST>, Box<AST>),
    Add(Box<AST>, Box<AST>),
    Zero,
}

impl AST {
    pub fn one() -> AST {
        AST::Psi(AST::Zero.to_box(), AST::Zero.to_box())
    }
    pub fn omega() -> AST {
        AST::Psi(AST::Zero.to_box(), AST::one().to_box())
    }
    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }
    pub fn _to_json(&self) -> String {
        match self {
            Self::Zero => "0".to_string(),
            Self::Add(l, r) => format!(
                r#"{{ type: "add", l: {}, r: {} }}"#,
                l._to_json(),
                r._to_json()
            ),
            Self::Psi(l, r) => format!(
                r#"{{ type: "psi", l: {}, r: {} }}"#,
                l._to_json(),
                r._to_json()
            ),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::Zero => "0".to_string(),
            Self::Add(l, r) => format!("{} + {}", l.to_string(), r.to_string()),
            Self::Psi(l, r) => {
                if self == &AST::one() {
                    "1".to_string()
                } else if self == &AST::omega() {
                    "w".to_string()
                } else {
                    format!("p({}, {})", l.to_string(), r.to_string())
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
        if let Self::Add(l, r) = self {
            **l == AST::one() || r.is_non_zero()
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
}
