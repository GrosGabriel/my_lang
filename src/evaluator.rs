use crate::term::Term;

use Term::*;


impl Term {


    // AppAbs rule
    pub fn app_abs(&self) -> Option<Term> {
        match self {
            App(t1, t2) if t2.is_value() => match &**t1 {
                Abs { var, ty: _, body } => {
                    Some(body.clone().subst(var, t2))
                }
                _ => None,
            },
            _ => None,
        }
    }

    // App1 rule
    pub fn app1(&self) -> Option<Term> {
        match self {
            App(t1, t2) => Some(App(Box::new(t1.step_cbv()?), t2.clone())),
            _ => None,
        }
    }


    // App2 rule
    pub fn app2(&self) -> Option<Term> {
        match self {
            App(t1, t2) if t1.is_value() => Some(App(t1.clone(), Box::new(t2.step_cbv()?))),
            _ => None,
        }
    }

    // Ite1 rule
    pub fn ite1(&self) -> Option<Term> {
        match self {
            Ite {
                cond,
                if_true,
                if_false,
            } => Some(Ite {
                cond: Box::new(cond.step_cbv()?),
                if_true: if_true.clone(),
                if_false: if_false.clone(),
            }),
            _ => None,
        }
    }

    // IteTrue and IteFalse rules
    pub fn ite(&self) -> Option<Term> {
        match self {
            Ite {
                cond,
                if_true,
                if_false,
            } => match **cond {
                True => Some(if_true.as_ref().clone()),
                False => Some(if_false.as_ref().clone()),
                _ => None,
            },
            _ => None,
        }
    }

    // Arithmetic cmp1 rule
    pub fn arithmetic_cmp1(&self) -> Option<Term> {
        match self {
            Add(t1, t2) => Some(Add(Box::new(t1.step_cbv()?), t2.clone())),
            Sub(t1, t2) => Some(Sub(Box::new(t1.step_cbv()?), t2.clone())),
            Mul(t1, t2) => Some(Mul(Box::new(t1.step_cbv()?), t2.clone())),
            Div(t1, t2) => Some(Div(Box::new(t1.step_cbv()?), t2.clone())),
            Eq(t1, t2) => Some(Eq(Box::new(t1.step_cbv()?), t2.clone())),
            Greater(t1, t2) => Some(Greater(Box::new(t1.step_cbv()?), t2.clone())),
            Less(t1, t2) => Some(Less(Box::new(t1.step_cbv()?), t2.clone())),
            _ => None,
        }
    }

    // Arithmetic cmp2 rule
    pub fn arithmetic_cmp2(&self) -> Option<Term> {
        match self {
            Add(t1, t2) if t1.is_value() => Some(Add(t1.clone(), Box::new(t2.step_cbv()?))),
            Sub(t1, t2) if t1.is_value() => Some(Sub(t1.clone(), Box::new(t2.step_cbv()?))),
            Mul(t1, t2) if t1.is_value() => Some(Mul(t1.clone(), Box::new(t2.step_cbv()?))),
            Div(t1, t2) if t1.is_value() => Some(Div(t1.clone(), Box::new(t2.step_cbv()?))),
            Eq(t1, t2) if t1.is_value() => Some(Eq(t1.clone(), Box::new(t2.step_cbv()?))),
            Greater(t1, t2) if t1.is_value() => Some(Greater(t1.clone(), Box::new(t2.step_cbv()?))),
            Less(t1, t2) if t1.is_value() => Some(Less(t1.clone(), Box::new(t2.step_cbv()?))),
            _ => None,
        }
    }


    // Arithemic evaluation rules 
    pub fn arithmetic_eval(&self) -> Option<Term> {
        match self {
            Add(t1, t2) => match (&**t1, &**t2) {
                (Int(n1), Int(n2)) => Some(Int(n1 + n2)),
                _ => None,
            },
            Sub(t1, t2) => match (&**t1, &**t2) {
                (Int(n1), Int(n2)) => Some(Int(n1 - n2)),
                _ => None,
            },
            Mul(t1, t2) => match (&**t1, &**t2) {
                (Int(n1), Int(n2)) => Some(Int(n1 * n2)),
                _ => None,
            },
            Div(t1, t2) => match (&**t1, &**t2) {
                (Int(_), Int(0)) => Some(Int(0)), // prevent division by zero
                (Int(n1), Int(n2)) => Some(Int(n1 / n2)),
                _ => None,
            },
            Eq(t1, t2) => match (&**t1, &**t2) {
                (Int(n1), Int(n2)) => Some(if n1 == n2 { True } else { False }),
                (True, True) => Some(True),
                (False, False) => Some(True),
                (True, False) | (False, True) => Some(False),
                _ => None,
            },
            Greater(t1, t2) => match (&**t1, &**t2) {
                (Int(n1), Int(n2)) => Some(if n1 > n2 { True } else { False }),
                _ => None,
            },
            Less(t1, t2) => match (&**t1, &**t2) {
                (Int(n1), Int(n2)) => Some(if n1 < n2 { True } else { False }),
                _ => None,
            },
            
            _ => None,
        }
    }

    // Let1 rule : Reduce val until it's a value (here it is reduced 1 time)
    pub fn let1(&self) -> Option<Term> {
        match self {
            Let { name, ty, val, body } => Some(Let {
                name: name.clone(),
                ty: ty.clone(),
                val: Box::new(val.step_cbv()?),
                body: body.clone(),
            }),
            _ => None,
        }
    }

    // Let2 rule : If val is a value, substitute it in the body
    pub fn let2(&self) -> Option<Term> {
        match self {
            Let { name, ty, val, body } if val.is_value() => Some(body.clone().subst(name, val)),
            _ => None,
        }
    }

    // Fix rule : Reduce fix f to f (fix f), only when applied to something
    pub fn fix_step(&self) -> Option<Term> {
        match self {
            App(f, arg) => match &**f {
                Fix(g) => Some(App(
                    Box::new(App(g.clone(), Box::new(Fix(g.clone())))),
                    arg.clone(),
                )),
                _ => None,
            },
            _ => None,
        }
    }
    
    // Pair rule : Reduce left term until it's a value, then reduce right term until it's a value
    pub fn pair_step(&self) -> Option<Term> {
        match self {
            Pair(a, b) if !a.is_value() => Some(Pair(Box::new(a.step_cbv()?), b.clone())),
            Pair(a, b) if !b.is_value() => Some(Pair(a.clone(), Box::new(b.step_cbv()?))),
            _ => None,
        }
    }


    // Fst and Snd rules : If the pair is a value, return the corresponding term
    pub fn fst_snd_step(&self) -> Option<Term> {
        match self {
            Fst(p) if !p.is_value() => Some(Fst(Box::new(p.step_cbv()?))),
            Fst(p) => match p.as_ref() {
                Pair(a, _) => Some(*a.clone()),
                _ => None,
            },
            Snd(p) if !p.is_value() => Some(Snd(Box::new(p.step_cbv()?))),
            Snd(p) => match p.as_ref() {
                Pair(_, b) => Some(*b.clone()),
                _ => None,
            },
            _ => None,
        }
    }
    // Boolean operations rules
    pub fn bool_ops_step(&self) -> Option<Term> {
        match self {
            And(t1, t2) if !t1.is_value() => Some(And(Box::new(t1.step_cbv()?), t2.clone())),
            And(t1, t2) if !t2.is_value() => Some(And(t1.clone(), Box::new(t2.step_cbv()?))),
            And(t1, t2) => match (&**t1, &**t2) {
                (True, True) => Some(True),
                (False, _) | (_, False) => Some(False),
                _ => None,
            },
            Or(t1, t2) if !t1.is_value() => Some(Or(Box::new(t1.step_cbv()?), t2.clone())),
            Or(t1, t2) if !t2.is_value() => Some(Or(t1.clone(), Box::new(t2.step_cbv()?))),
            Or(t1, t2) => match (&**t1, &**t2) {
                (True, _) | (_, True) => Some(True),
                (False, False) => Some(False),
                _ => None,
            },
            Not(t) if !t.is_value() => Some(Not(Box::new(t.step_cbv()?))),
            Not(t) => match &**t {
                True => Some(False),
                False => Some(True),
                _ => None,
            },
            _ => None,
        }
    }


    // Lists operations rules
    pub fn list_steps(&self) -> Option<Term> {
        match self {
            // Cons1, Cons2
            Cons(head, tail) if !head.is_value() => {
                Some(Cons(Box::new(head.step_cbv()?), tail.clone()))
            }
            Cons(head, tail) if head.is_value() => {
                Some(Cons(head.clone(), Box::new(tail.step_cbv()?)))
            }

            // CaseList1, CaseListNil, CaseListCons
            CaseList { scrutinee, if_nil, if_cons } => match scrutinee.as_ref() {
                Nil(_) => Some(*if_nil.clone()),
                Cons(vh, vt) if vh.is_value() && vt.is_value() => Some(App(
                    Box::new(App(if_cons.clone(), vh.clone())),
                    vt.clone(),
                )),
                _ => Some(CaseList {
                    scrutinee: Box::new(scrutinee.step_cbv()?),
                    if_nil: if_nil.clone(),
                    if_cons: if_cons.clone(),
                }),
            },

            // RecList1, RecListNil, RecListCons
            RecList { scrutinee, if_nil, if_cons } => match scrutinee.as_ref() {
                Nil(_) => Some(*if_nil.clone()),
                Cons(vh, vt) if vh.is_value() && vt.is_value() => Some(App(
                    Box::new(App(
                        Box::new(App(if_cons.clone(), vh.clone())),
                        vt.clone(),
                    )),
                    Box::new(RecList {
                        scrutinee: vt.clone(),
                        if_nil: if_nil.clone(),
                        if_cons: if_cons.clone(),
                    }),
                )),
                _ => Some(RecList {
                    scrutinee: Box::new(scrutinee.step_cbv()?),
                    if_nil: if_nil.clone(),
                    if_cons: if_cons.clone(),
                }),
            },

            _ => None,
        }
    }


    // Sum rules
    pub fn sum_steps(&self) -> Option<Term> {
        match self {
            // Inl1, Inr1
            Inl { t, r_ty } if !t.is_value() => Some(Inl {
                t: Box::new(t.step_cbv()?),
                r_ty: r_ty.clone(),
            }),
            Inr { t, l_ty } if !t.is_value() => Some(Inr {
                t: Box::new(t.step_cbv()?),
                l_ty: l_ty.clone(),
            }),
            // CaseSum1, CaseSumL, CaseSumR
            CaseSum { scrutinee, inl_case, inr_case } => match scrutinee.as_ref() {
                Inl { t, .. } if t.is_value() => Some(App(inl_case.clone(), t.clone())),
                Inr { t, .. } if t.is_value() => Some(App(inr_case.clone(), t.clone())),
                _ => Some(CaseSum {
                    scrutinee: Box::new(scrutinee.step_cbv()?),
                    inl_case: inl_case.clone(),
                    inr_case: inr_case.clone(),
                }),
            },
            _ => None,
        }
    }


    /// Does a call-by-value reduction step returning None if no reduction rule applies.
    pub fn step_cbv(&self) -> Option<Term> {
        self.app1()
            .or_else(|| self.app2())
            .or_else(|| self.app_abs())
            .or_else(|| self.ite1())
            .or_else(|| self.ite())
            .or_else(|| self.let1())
            .or_else(|| self.let2())
            .or_else(|| self.fix_step())
            .or_else(|| self.pair_step())
            .or_else(|| self.fst_snd_step())
            .or_else(|| self.arithmetic_cmp1()) // check if left term can be reduced
            .or_else(|| self.arithmetic_cmp2()) // check if right term can be reduced (only if left is a value)
            .or_else(|| self.arithmetic_eval()) // check if both terms are values and can be evaluated ORDER IS IMPORTANT BETWEEN THESE 3 RULES
            .or_else(|| self.bool_ops_step()) // check if boolean operations can be reduced
            .or_else(|| self.list_steps()) // check if list operations can be reduced
            .or_else(|| self.sum_steps()) // check if sum operations can be reduced
    }


    /// Does any number of CBV steps.
    /// Returns the final term for which no reduction could be made.
    pub fn multistep_cbv(mut self) -> Self {
        while let Some(t) = self.step_cbv() {
            self = t;
        }
        self
    }

}