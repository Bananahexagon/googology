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
    pub fn to_json(self) -> String {
        match self {
            Self::Zero => "0".to_string(),
            Self::Add(l, r) => format!(
                r#"{{ type: "add", l: {}, r: {} }}"#,
                l.to_json(),
                r.to_json()
            ),
            Self::Psi(l, r) => format!(
                r#"{{ type: "psi", l: {}, r: {} }}"#,
                l.to_json(),
                r.to_json()
            ),
        }
    }
    pub fn _to_string(self) -> String {
        match self {
            Self::Zero => "0".to_string(),
            Self::Add(l, r) => format!("{} + {}", l.to_json(), r.to_json()),
            Self::Psi(l, r) => format!("p({}, {})", l.to_json(), r.to_json()),
        }
    }
    pub fn is_successor(&self) -> bool {
        if let Self::Add(_, r) = self {
            **r == AST::Zero || r.is_successor()
        } else {
            false
        }
    }
}
