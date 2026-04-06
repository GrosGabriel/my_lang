use crate::typechecker::Type;


#[derive(Debug, Clone)]
pub enum Term {
    //lambda calcul
    Var(String),
    Abs{ var : String, ty: Type,body : Box<Term>},
    App(Box<Term>, Box<Term>),

    //booleans
    True,
    False,
    And(Box<Term>, Box<Term>),
    Or(Box<Term>, Box<Term>),
    Not(Box<Term>),

    //control-flow
    Ite{
        cond : Box<Term>,
        if_true : Box<Term>,
        if_false : Box<Term>,
    },


    //integers

    Int(i64),
    Add(Box<Term>, Box<Term>),
    Sub(Box<Term>, Box<Term>),
    Mul(Box<Term>, Box<Term>),
    Div(Box<Term>, Box<Term>),
    Eq(Box<Term>, Box<Term>),
    Greater(Box<Term>, Box<Term>),
    Less(Box<Term>, Box<Term>),


    //let
    Let{
        name: String,
        ty: Type,
        val: Box<Term>,
        body: Box<Term>
    },

    //recursive
    Fix(Box<Term>),

    //pairs
    Pair(Box<Term>, Box<Term>),
    Fst(Box<Term>),
    Snd(Box<Term>),

    // lists
    Nil(Type), // empty list with type annotation
    Cons(Box<Term>, Box<Term>), // cons head tail
    /// In terms of polymorphic types:
    ///   case_list : forall A, forall T, [A] -> T -> (A -> [A] -> T) -> T
    ///   rec_list : forall A, forall T, [A] -> T -> (A -> [A] -> T -> T) -> T
    CaseList { scrutinee: Box<Term>, if_nil: Box<Term>, if_cons: Box<Term> }, // case analysis for lists
    RecList { scrutinee: Box<Term>, if_nil: Box<Term>, if_cons: Box<Term> }, // primitive recursion for lists

    // Sum
    Inl{t: Box<Term>, r_ty: Type},
    Inr{t: Box<Term>, l_ty: Type},
    CaseSum { scrutinee: Box<Term>, inl_case: Box<Term>, inr_case: Box<Term> },
}

use Term::*;


impl Term {

    // Returns wether the term is a value (i.e. cannot be reduced further).
    pub fn is_value(&self) -> bool {
        match self {
            Int(_) => true,
            True | False => true,
            Abs { .. } => true,
            Pair(t1, t2) => t1.is_value() && t2.is_value(),
            Fix(t) => t.is_value(), // Fix is a value if its body is a value
            Nil(_) => true,
            Cons(head, tail) => head.is_value() && tail.is_value(),
            Inl { t, .. } => t.is_value(),
            Inr { t, .. } => t.is_value(),



            _ => false,
        }
    }


    /// Naively substitutes all free occurrences of `var` by `value`.
    /// The substitution must be legal, otherwise the result may be incorrect.
    pub fn subst(&self, var: &str, value: &Term) -> Term {
        match self {
            Var(v) => {
                if v == var {
                    value.clone()
                } else {
                    Var(v.clone())
                }
            }

            Abs { var: v, ty, body } => {
                if v == var {
                    Abs { var : v.clone(), ty: ty.clone(), body : body.clone()}
                } else {
                    Abs { var : v.clone(), ty: ty.clone(), body : Box::new(body.subst(var, value))}
                }
            }

            App(t1, t2) => App(
                Box::new(t1.subst(var, value)),
                Box::new(t2.subst(var, value)),
            ),

            True => True,
            False => False,

            Ite { cond, if_true, if_false } => Ite {
                cond: Box::new(cond.subst(var, value)),
                if_true: Box::new(if_true.subst(var, value)),
                if_false: Box::new(if_false.subst(var, value)),
            },

            Int(n) => Int(*n),

            Add(t1, t2) => Add(
                Box::new(t1.subst(var, value)),
                Box::new(t2.subst(var, value)),
            ),

            Sub(t1, t2) => Sub(
                Box::new(t1.subst(var, value)),
                Box::new(t2.subst(var, value)),
            ),

            Mul(t1, t2) => Mul(
                Box::new(t1.subst(var, value)),
                Box::new(t2.subst(var, value)),
            ),

            Div(t1, t2) => Div(
                Box::new(t1.subst(var, value)),
                Box::new(t2.subst(var, value)),
            ),

            Eq(t1, t2) => Eq(
                Box::new(t1.subst(var, value)),
                Box::new(t2.subst(var, value)),
            ),

            Greater(t1, t2) => Greater(
                Box::new(t1.subst(var, value)),
                Box::new(t2.subst(var, value)),
            ),

            Less(t1, t2) => Less(
                Box::new(t1.subst(var, value)),
                Box::new(t2.subst(var, value)),
            ),

            Let { name, ty, val, body } => Let {
                name: name.clone(),
                ty: ty.clone(),
                val: Box::new(val.subst(var, value)),
                body: if name == var {
                    body.clone()
                } else {
                    Box::new(body.subst(var, value))
                },
            },

            Fix(t) => Fix(Box::new(t.subst(var, value))),
            Pair(t1, t2) => Pair(
                Box::new(t1.subst(var, value)),
                Box::new(t2.subst(var, value)),
            ),

            Fst(t) => Fst(Box::new(t.subst(var, value))),
            Snd(t) => Snd(Box::new(t.subst(var, value))),

            And(t1, t2) => And(
                Box::new(t1.subst(var, value)),
                Box::new(t2.subst(var, value)),
            ),

            Or(t1, t2) => Or(
                Box::new(t1.subst(var, value)),
                Box::new(t2.subst(var, value)),
            ),

            Not(t) => Not(Box::new(t.subst(var, value))),

            Nil(ty) => Nil(ty.clone()),
            Cons(head, tail) => Cons(
                Box::new(head.subst(var, value)),
                Box::new(tail.subst(var, value)),
            ),
            CaseList { scrutinee, if_nil, if_cons } => CaseList {
                scrutinee: Box::new(scrutinee.subst(var, value)),
                if_nil: Box::new(if_nil.subst(var, value)),
                if_cons: Box::new(if_cons.subst(var, value)),
            },
            RecList { scrutinee, if_nil, if_cons } => RecList {
                scrutinee: Box::new(scrutinee.subst(var, value)),
                if_nil: Box::new(if_nil.subst(var, value)),
                if_cons: Box::new(if_cons.subst(var, value)),
            },

            Inl { t, r_ty } => Inl { t: Box::new(t.subst(var, value)), r_ty: r_ty.clone() },
            Inr { t, l_ty } => Inr { t: Box::new(t.subst(var, value)), l_ty: l_ty.clone() },
            CaseSum { scrutinee, inl_case, inr_case } => CaseSum {
                scrutinee: Box::new(scrutinee.subst(var, value)),
                inl_case: Box::new(inl_case.subst(var, value)),
                inr_case: Box::new(inr_case.subst(var, value)),
            },





        }
    }


}