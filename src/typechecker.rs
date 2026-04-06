use crate::term::Term;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Arrow(Box<Type>, Box<Type>),
    Pair(Box<Type>, Box<Type>),
    List(Box<Type>),
    Sum(Box<Type>, Box<Type>),


}



impl Term {

    // Bool typing rule
    pub fn bool_ty<'t>(&'t self, _ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::True | Term::False => Some(Type::Bool),
            _ => None,
        }
    }

    // Bool and or not rules
    pub fn bool_and_or_not_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::And(t1, t2) | Term::Or(t1, t2) => {
                if t1.infer_type_ctx(ctx)? == Type::Bool && t2.infer_type_ctx(ctx)? == Type::Bool {
                    Some(Type::Bool)
                } else {
                    None
                }
            }
            Term::Not(t) => {
                if t.infer_type_ctx(ctx)? == Type::Bool {
                    Some(Type::Bool)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // Ite Typing rule
    pub fn ite_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Ite { cond, if_true, if_false } => {
                if cond.infer_type_ctx(ctx)? != Type::Bool {
                    None
                } else {
                    let ty = if_true.infer_type_ctx(ctx)?;
                    if ty == if_false.infer_type_ctx(ctx)? {
                        Some(ty)
                    } else {
                        None
                    }
                }
            }
            _ => None,
        }
    }

    // Var typing rule
    // Note the rfind for shadowing
    pub fn var_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Var(name) => ctx.iter().rfind(|(n, _)| n == name).map(|(_, ty)| (*ty).clone()),
            _ => None,
        }
    }

    // App typing rule
    pub fn app_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::App(t1, t2) => {
                let t1_ty = t1.infer_type_ctx(ctx)?;
                let t2_ty = t2.infer_type_ctx(ctx)?;
                match t1_ty {
                    Type::Arrow(arg_ty, ret_ty) if *arg_ty == t2_ty => Some(*ret_ty),
                    _ => None,
                }
            }
            _ => None,
        }
    }
    
    // Abs typing rule
    pub fn abs_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Abs { var, ty, body } => {
                ctx.push((var, ty)); // var : &'t String, ty : &'t Type , don't clone
                let body_ty = body.infer_type_ctx(ctx);
                ctx.pop();
                let body_ty = body_ty?;
                Some(Type::Arrow(Box::new(ty.clone()), Box::new(body_ty)))
            }
            _ => None,
        }
    }


    // Int typing rule 
    pub fn int_ty<'t>(&'t self, _ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Int(_) => Some(Type::Int),
            _ => None,
        }
    }

    // Add Sub Mul Div typing rule 
    pub fn add_sub_mul_div_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Add(t1, t2) | Term::Sub(t1, t2) | Term::Mul(t1, t2) | Term::Div(t1, t2) => {
                if t1.infer_type_ctx(ctx)? == Type::Int && t2.infer_type_ctx(ctx)? == Type::Int {
                    Some(Type::Int)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    // Eq typing rule (only for Int and Bool for now)
    pub fn eq_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Eq(t1, t2) => {
                let t1_ty = t1.infer_type_ctx(ctx)?;
                let t2_ty = t2.infer_type_ctx(ctx)?;
                if t1_ty == t2_ty && matches!(t1_ty, Type::Int | Type::Bool) {
                    Some(Type::Bool)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // Comparison typing rules for integers
    pub fn comparison_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Greater(t1, t2) | Term::Less(t1, t2) => {
                if t1.infer_type_ctx(ctx)? == Type::Int && t2.infer_type_ctx(ctx)? == Type::Int {
                    Some(Type::Bool)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    // Let typing rule
    pub fn let_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Let { name, ty, val, body } => {
                if val.infer_type_ctx(ctx)? != *ty {
                    None
                } else {
                    ctx.push((name, ty)); // name : &'t String, ty : &'t Type , don't clone
                    let body_ty = body.infer_type_ctx(ctx);
                    ctx.pop();
                    body_ty
                }
            }
            _ => None,
        }
    }

    
    // Fix typing rule
    pub fn fix_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Fix(f) => {
                let f_ty = f.infer_type_ctx(ctx)?;
                match f_ty {
                    Type::Arrow(arg_ty, ret_ty) if *arg_ty == *ret_ty => Some(*ret_ty),
                    _ => None,
                }
            }
            _ => None,
        }
    }   


    // Pair typing rule
    pub fn pair_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Pair(t1, t2) => {
                let t1_ty = t1.infer_type_ctx(ctx)?;
                let t2_ty = t2.infer_type_ctx(ctx)?;
                Some(Type::Pair(Box::new(t1_ty), Box::new(t2_ty)))
            }
            _ => None,
        }
    }

    // Fst and Snd typing rules 
    pub fn fst_and_snd_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Fst(t) => {
                let t_ty = t.infer_type_ctx(ctx)?;
                match t_ty {
                    Type::Pair(left_ty, _) => Some(*left_ty),
                    _ => None,
                }
            }
            Term::Snd(t) => {
                let t_ty = t.infer_type_ctx(ctx)?;
                match t_ty {
                    Type::Pair(_, right_ty) => Some(*right_ty),
                    _ => None,
                }
            }
            _ => None,
        }
    }


    // Lists typing rules 
    pub fn list_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            // Nil
            Term::Nil(ty) => Some(Type::List(Box::new(ty.clone()))),

            // Cons
            Term::Cons(head, tail) => {
                let head_ty = head.infer_type_ctx(ctx)?;
                match tail.infer_type_ctx(ctx)? {
                    Type::List(t) if *t == head_ty => Some(Type::List(Box::new(head_ty))),
                    _ => None,
                }
            }
            // CaseList : t1::[T], tn::U, tc::T->[T]->U => U
            Term::CaseList { scrutinee, if_nil, if_cons } => {
                let elem_ty = match scrutinee.infer_type_ctx(ctx)? {
                    Type::List(t) => *t,
                    _ => return None,
                };
                let u = if_nil.infer_type_ctx(ctx)?;
                let expected_cons_ty = Type::Arrow(
                    Box::new(elem_ty.clone()),
                    Box::new(Type::Arrow(
                        Box::new(Type::List(Box::new(elem_ty))),
                        Box::new(u.clone()),
                    )),
                );
                if if_cons.infer_type_ctx(ctx)? == expected_cons_ty {
                    Some(u)
                } else {
                    None
                }
            }
            // RecList : t1::[T], tn::U, tc::T->[T]->U->U => U
            Term::RecList { scrutinee, if_nil, if_cons } => {
                let elem_ty = match scrutinee.infer_type_ctx(ctx)? {
                    Type::List(t) => *t,
                    _ => return None,
                };
                let u = if_nil.infer_type_ctx(ctx)?;
                let expected_cons_ty = Type::Arrow(
                    Box::new(elem_ty.clone()),
                    Box::new(Type::Arrow(
                        Box::new(Type::List(Box::new(elem_ty))),
                        Box::new(Type::Arrow(
                            Box::new(u.clone()),
                            Box::new(u.clone()),
                        )),
                    )),
                );
                if if_cons.infer_type_ctx(ctx)? == expected_cons_ty {
                    Some(u)
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    // Sum typing rules
    pub fn sum_ty<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        match self {
            Term::Inl{t, r_ty} => Some(
                Type::Sum(
                    Box::new( t.infer_type_ctx(ctx)?),
                    Box::new(r_ty.clone()),
                )
            ),
            Term::Inr{t, l_ty} => Some(
                Type::Sum(
                    Box::new( l_ty.clone()),
                    Box::new( t.infer_type_ctx(ctx)?),
                )
            ),
            Term::CaseSum{scrutinee, inl_case, inr_case} => {
                // scrutinee :: T1 + T2
                let (t1, t2) = match scrutinee.infer_type_ctx(ctx)? {
                    Type::Sum(t1, t2) => (*t1, *t2),
                    _ => return None,
                };
                // if_left :: T1 -> T
                let t = match inl_case.infer_type_ctx(ctx)? {
                    Type::Arrow(dom, cod) if *dom == t1 => *cod,
                    _ => return None,
                };
                // if_right :: T2 -> T
                match inr_case.infer_type_ctx(ctx)? {
                    Type::Arrow(dom, cod) if *dom == t2 && *cod == t => Some(t),
                    _ => None,
                }
            },
            _ => None,
        }
    }



    /// Infer type in some context using the hypothetical typing rules
    pub fn infer_type_ctx<'t>(&'t self, ctx: &mut Vec<(&'t str, &'t Type)>) -> Option<Type> {
        self.var_ty(ctx)
            .or_else(|| self.app_ty(ctx))
            .or_else(|| self.abs_ty(ctx))
            .or_else(|| self.bool_ty(ctx)) 
            .or_else(|| self.ite_ty(ctx))
            .or_else(|| self.int_ty(ctx))
            .or_else(|| self.add_sub_mul_div_ty(ctx))
            .or_else(|| self.eq_ty(ctx))
            .or_else(|| self.comparison_ty(ctx))
            .or_else(|| self.let_ty(ctx))
            .or_else(|| self.fix_ty(ctx))
            .or_else(|| self.pair_ty(ctx))
            .or_else(|| self.fst_and_snd_ty(ctx))
            .or_else(|| self.bool_and_or_not_ty(ctx))
            .or_else(|| self.list_ty(ctx))
            .or_else(|| self.sum_ty(ctx))
    }

    /// Infer type in the empty context
    pub fn infer_type(&self) -> Option<Type> {
        let mut ctx = Vec::new();
        self.infer_type_ctx(&mut ctx)
    }
}

