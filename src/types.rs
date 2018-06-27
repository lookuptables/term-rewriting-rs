use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::iter;

use super::pretty::Pretty;

/// Represents a place in a [`Term`].
///
/// [`Term`]: enum.Term.html
pub type Place = Vec<usize>;

/// A symbol for an unspecified term. Only carries meaning alongside a [`Signature`].
///
/// To construct a `Variable`, use [`Signature::new_var`]
///
/// [`Signature`]: struct.Signature.html
/// [`Signature::new_var`]: struct.Signature.html#method.new_var
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Variable {
    pub(crate) id: usize,
}
impl Variable {
    /// Returns a `Variable`'s name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::Signature;
    /// let mut sig = Signature::default();
    /// let var = sig.new_var(Some("Z".to_string()));
    /// assert_eq!(var.name(&sig), Some("Z"));
    /// ```
    pub fn name(self, sig: &Signature) -> Option<&str> {
        let opt = &sig.variables[self.id];
        opt.as_ref().map(|s| s.as_str())
    }
    /// Returns a human-readable, string representation of a `Variable`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::Signature;
    /// let mut sig = Signature::default();
    /// let var = sig.new_var(Some("Z".to_string()));
    /// assert_eq!(var.display(&sig), "Z_");
    /// ```
    pub fn display(self, sig: &Signature) -> String {
        if let Some(ref name) = sig.variables[self.id] {
            format!("{}_", name)
        } else {
            format!("var{}_", self.id)
        }
    }
}

/// A symbol with fixed arity. Only carries meaning alongside a [`Signature`].
///
/// To construct an `Operator`, use [`Signature::new_op`].
///
/// [`Signature`]: struct.Signature.html
/// [`Signature::new_op`]: struct.Signature.html#method.new_op
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Operator {
    pub(crate) id: usize,
}
impl Operator {
    /// Returns an `Operator`'s arity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::Signature;
    /// let mut sig = Signature::default();
    /// let op = sig.new_op(2, Some("Z".to_string()));
    /// assert_eq!(op.arity(&sig), 2);
    /// ```
    pub fn arity(self, sig: &Signature) -> u32 {
        sig.operators[self.id].0
    }
    /// Returns an `Operator`'s name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::Signature;
    /// let mut sig = Signature::default();
    /// let op = sig.new_op(2, Some("Z".to_string()));
    /// assert_eq!(op.name(&sig), Some("Z"));
    /// ```
    pub fn name(self, sig: &Signature) -> Option<&str> {
        let opt = &sig.operators[self.id].1;
        opt.as_ref().map(|s| s.as_str())
    }
    /// Returns a human-readable, string representation of an `Operator`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::Signature;
    /// let mut sig = Signature::default();
    /// let op = sig.new_op(2, Some("Z".to_string()));
    /// assert_eq!(op.display(&sig), "Z");
    /// ```
    pub fn display(self, sig: &Signature) -> String {
        if let (_, Some(ref name)) = sig.operators[self.id] {
            name.clone()
        } else {
            format!("op{}", self.id)
        }
    }
}

/// `Atom`s are the parts of a [`TRS`] that are not constructed from smaller parts: [`Variable`]s and [`Operators`].
///
/// [`TRS`]: struct.TRS.html
/// [`Variable`]: struct.Variable.html
/// [`Operator`]: struct.Operator.hmtl
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Atom {
    Variable(Variable),
    Operator(Operator),
}

impl From<Variable> for Atom {
    fn from(var: Variable) -> Atom {
        Atom::Variable(var)
    }
}

impl From<Operator> for Atom {
    fn from(op: Operator) -> Atom {
        Atom::Operator(op)
    }
}

/// Records a universe of symbols.
///
/// Use [`Signature::default`] for a blank signature, or [`Signature::new`] to initialize a
/// signature with given operators.
///
/// [`Signature::default`]: #method.default
/// [`Signature::new`]: #method.new
#[derive(Clone, Debug)]
pub struct Signature {
    /// Stores the (arity, name) for every operator.
    pub(crate) operators: Vec<(u32, Option<String>)>,
    /// Stores the name for every variable.
    pub(crate) variables: Vec<Option<String>>,
}
impl Signature {
    /// Construct a signature with the given operators.
    ///
    /// Each operator is specified in the form of `(arity, Some(name))` or
    /// `(arity, None)`, where `arity` is the number of arguments a term takes
    /// (for example, an `arity` of 0 gives a "constant" operator). A `name` for
    /// the operator is unnecessary, but may be supplied for more readable
    /// formatting.
    ///
    /// The returned vector of [`Operator`]s corresponds to the supplied spec.
    ///
    /// [`Operator`]: struct.Operator.html
    ///
    /// # Examples
    ///
    ///
    /// ```
    /// # use term_rewriting::{Signature};
    /// // Constructs a new signature with the operators ".","S","K' from the vector
    /// let (mut sig,ops) = Signature:: new(vec![
    ///     (2, Some(".".to_string())),
    ///     (0, Some("S".to_string())),
    ///     (0, Some("K".to_string())),
    /// ]);
    /// let (mut sig2,ops2) = Signature:: new(vec![
    ///     (2, Some(".".to_string())),
    ///     (0, Some("S".to_string())),
    ///     (0, Some("K".to_string())),
    /// ]);
    /// let (mut sig3,ops3) = Signature:: new(vec![
    ///     (2, Some(".".to_string())),
    ///     (0, Some("S".to_string())),
    /// ]);
    ///
    /// assert_eq!(sig,sig2);
    /// assert_ne!(sig,sig3);
    ///```
    pub fn new(operator_spec: Vec<(u32, Option<String>)>) -> (Signature, Vec<Operator>) {
        let variables = Vec::new();
        let sig = Signature {
            operators: operator_spec,
            variables,
        };
        let ops = sig.operators();
        (sig, ops)
    }
    /// Returns every [`Operator`] known to the signature, in the order they were created.
    ///
    /// [`Operator`]: struct.Operator.html
    ///
    /// # Examples
    ///
    ///
    /// ```
    /// # use term_rewriting::{Signature};
    /// // Constructs a new signature with the operators ".","S","K' from the vector
    /// let (mut sig,ops) = Signature:: new(vec![
    ///     (2, Some(".".to_string())),
    ///     (0, Some("S".to_string())),
    ///     (0, Some("K".to_string())),
    /// ]);
    /// // Returns the operators in the signature above
    /// let example_ops = sig.operators();
    ///
    /// assert_eq!(ops,example_ops);
    ///```
    pub fn operators(&self) -> Vec<Operator> {
        (0..self.operators.len())
            .map(|id| Operator { id })
            .collect()
    }
    /// Returns every [`Variable`] known to the signature, in the order they were created.
    ///
    /// [`Variable`]: struct.Variable.html
    ///
    /// # Examples
    ///
    ///
    /// ```
    /// # use term_rewriting::{Signature};
    /// // Constructs a new signature with the variables ".","S","K' from the vector
    /// let (mut sig,ops) = Signature:: new(vec![
    ///     (2, Some(".".to_string())),
    ///     (0, Some("S".to_string())),
    ///     (0, Some("K".to_string())),
    /// ]);
    ///
    /// // Returns the variables in the signature above
    /// let vars = sig.variables();
    /// let vars2 = sig.variables();
    ///
    /// let newVar = sig.new_var(Some("A".to_string()));
    /// let vars3 = sig.variables();
    ///
    /// assert_eq!(vars,vars2);
    /// assert_ne!(vars,vars3);
    ///```
    pub fn variables(&self) -> Vec<Variable> {
        (0..self.variables.len())
            .map(|id| Variable { id })
            .collect()
    }
    /// Returns every [`Atom`] known to the signature.
    ///
    /// [`Atom`]: enum.Atom.html
    pub fn atoms(&self) -> Vec<Atom> {
        let vars = self.variables().into_iter().map(Atom::Variable);
        let ops = self.operators().into_iter().map(Atom::Operator);
        vars.chain(ops).collect()
    }
    /// Create a new [`Operator`] distinct from all existing operators.
    ///
    /// [`Operator`]: struct.Operator.html
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::{Signature};
    /// let mut sig = Signature::default();
    /// let a = sig.new_op(1, Some(".".to_string()));
    /// let s = sig.new_op(2, Some("S".to_string()));
    /// let s2 = sig.new_op(2, Some("S".to_string()));
    ///
    /// assert_ne!(a,s);
    /// assert_ne!(a,s2);
    /// assert_ne!(s,s2);
    /// ```
    pub fn new_op(&mut self, arity: u32, name: Option<String>) -> Operator {
        let id = self.operators.len();
        self.operators.push((arity, name));
        Operator { id }
    }
    /// Create a new [`Variable`] distinct from all existing variables.
    ///
    /// [`Variable`]: struct.Variable.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::{Signature};
    /// let mut sig = Signature::default();
    /// let z = sig.new_var(Some("Z".to_string()));
    /// let z2 = sig.new_var(Some("Z".to_string()));
    ///
    /// assert_ne!(z,z2);
    /// ```
    pub fn new_var(&mut self, name: Option<String>) -> Variable {
        let id = self.variables.len();
        self.variables.push(name);
        Variable { id }
    }
    /// Merge two [`Signature`]s. All terms, contexts, rules, and TRSs associated
    /// with the `other` signature should be `reified` using methods provided
    /// by the returned [`SignatureChange`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::{MergeStrategy, Signature, parse_term, parse_trs};
    /// let (mut sig1, _ops) = Signature::new(vec![
    ///     (2, Some(".".to_string())),
    ///     (0, Some("S".to_string())),
    ///     (0, Some("K".to_string())),
    /// ]);
    /// let mut sig2 = sig1.clone();
    ///
    /// let s1 = "S K K";
    /// let term = parse_term(&mut sig1, s1).unwrap();
    /// let s2 = "S x_ y_ z_ = (x_ z_) (y_ z_);";
    /// let trs = parse_trs(&mut sig2, s2).unwrap();
    ///
    /// let sigchange = sig1.merge(sig2, MergeStrategy::SameOperators);
    /// // we only reify terms/rules/TRSs associated with sig2
    /// let trs = sigchange.reify_trs(trs);
    /// // now term and rule both exist with symbols according to sig1.
    /// ```
    /// [`Signature`]: struct.Signature.html
    /// [`SignatureChange`]: struct.SignatureChange.html
    pub fn merge(&mut self, mut other: Signature, strategy: MergeStrategy) -> SignatureChange {
        let delta_op = match strategy {
            MergeStrategy::SameOperators => 0,
            MergeStrategy::OperatorsByArityAndName => {
                let old_len = self.operators.len();
                for op_spec in other.operators {
                    if !self.operators.contains(&op_spec) {
                        self.operators.push(op_spec)
                    }
                }
                old_len
            }
            MergeStrategy::DistinctOperators => {
                let old_len = self.operators.len();
                self.operators.append(&mut other.operators);
                old_len
            }
        };
        let delta_var = self.variables.len();
        self.variables.append(&mut other.variables);
        SignatureChange {
            delta_op,
            delta_var,
        }
    }
}
/// Creates a new Signature without any Operators or Variables
///
/// # Usage
///
/// ```
/// # use term_rewriting::{Signature};
/// let mut sig = Signature::default();
/// let (mut sig2,ops) = Signature::new(vec![]);
/// let (mut sig3,ops) = Signature::new(vec![
///     (2, Some(".".to_string())),
///     (0, Some("S".to_string())),
///     (0, Some("K".to_string())),
///     ]);
/// assert_eq!(sig,sig2);
/// assert_ne!(sig,sig3);
/// ```
impl Default for Signature {
    fn default() -> Signature {
        Signature {
            operators: Vec::new(),
            variables: Vec::new(),
        }
    }
}
/// Compares two Signatures to test if they should be considered equal
///
/// # Examples
///
/// ```
/// # use term_rewriting::{Signature};
/// let mut sig = Signature::default();
/// let mut sig2 = Signature::default();
///
/// assert!(sig.eq(&sig2));
/// ```
impl PartialEq for Signature {
    fn eq(&self, other: &Signature) -> bool {
        self.variables.len() == other.variables.len()
            && self.operators.len() == other.operators.len()
            && self.operators
                .iter()
                .zip(&other.operators)
                .all(|(&(arity1, _), &(arity2, _))| arity1 == arity2)
    }
}

/// Specifies how to merge two signatures.
/// See [`Signature::merge`].
///
/// [`Signature::merge`]: struct.Signature.html#method.merge
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MergeStrategy {
    /// Operators won't be added to the signature:
    /// this must mean all operators were added in the same order for both
    /// signatures.
    SameOperators,
    /// Operators are added to the signature only when there is no existing
    /// operator with the same arity and name.
    OperatorsByArityAndName,
    /// Operators are always added distinctly:
    /// no operators associated with the first signature will every equate to
    /// those associated with the second signature.
    DistinctOperators,
}

/// Allows terms/rules/TRSs to be reified for use with another signature.
/// See [`Signature::merge`].
///
/// [`Signature::merge`]: struct.Signature.html#method.merge
pub struct SignatureChange {
    delta_op: usize,
    delta_var: usize,
}
impl SignatureChange {
    /// Reifies term for use with another signature
    /// See [`SignatureChange`]
    ///
    /// [`SignatureChange`]: struct.SignatureChange
    pub fn reify_term(&self, term: Term) -> Term {
        match term {
            Term::Variable(Variable { id }) => {
                let id = id + self.delta_var;
                Term::Variable(Variable { id })
            }
            Term::Application {
                op: Operator { id },
                args,
            } => {
                let id = id + self.delta_op;
                Term::Application {
                    op: Operator { id },
                    args: args.into_iter().map(|t| self.reify_term(t)).collect(),
                }
            }
        }
    }
    /// Reifies context for use with another signature
    /// See [`SignatureChange`]
    ///
    /// [`SignatureChange`]: struct.SignatureChange
    pub fn reify_context(&self, context: Context) -> Context {
        match context {
            Context::Hole => Context::Hole,
            Context::Variable(Variable { id }) => {
                let id = id + self.delta_var;
                Context::Variable(Variable { id })
            }
            Context::Application {
                op: Operator { id },
                args,
            } => {
                let id = id + self.delta_op;
                Context::Application {
                    op: Operator { id },
                    args: args.into_iter().map(|t| self.reify_context(t)).collect(),
                }
            }
        }
    }
    /// Reifies rule for use with another signature
    /// See [`SignatureChange`]
    ///
    /// [`SignatureChange`]: struct.SignatureChange
    pub fn reify_rule(&self, rule: Rule) -> Rule {
        let Rule { lhs, rhs } = rule;
        let lhs = self.reify_term(lhs);
        let rhs = rhs.into_iter().map(|t| self.reify_term(t)).collect();
        Rule { lhs, rhs }
    }
    /// Reifies TRS for use with another signature
    /// See [`SignatureChange`]
    ///
    /// [`SignatureChange`]: struct.SignatureChange
    pub fn reify_trs(&self, trs: TRS) -> TRS {
        let rules = trs.rules.into_iter().map(|r| self.reify_rule(r)).collect();
        TRS { rules }
    }
}

/// A way of signifying what type of unification is being performed
#[derive(PartialEq, Eq)]
enum Unification {
    Match,
    Unify,
}

/// A first-order context: a [`Term`] that may have [`Hole`]s; a sort of [`Term`] template.
///
/// [`Term`]: enum.Term.html
/// [`Hole`]: enum.Context.html#variant.Hole
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Context {
    /// An empty place in the `Context`.
    Hole,
    /// A concrete but unspecified `Context` (e.g. `x`, `y`)
    Variable(Variable),
    /// An [`Operator`] applied to zero or more `Context`s (e.g. (`f(x, y)`, `g()`)
    ///
    /// [`Operator`]: struct.Operator.html
    Application { op: Operator, args: Vec<Context> },
}
impl Context {
    /// A serialized representation of the Context.
    pub fn display(&self, sig: &Signature) -> String {
        match self {
            Context::Hole => "[!]".to_string(),
            Context::Variable(v) => v.display(sig),
            Context::Application { op, args } => {
                let op_str = op.display(sig);
                if args.is_empty() {
                    op_str
                } else {
                    let args_str = args.iter().map(|arg| arg.display(sig)).join(" ");
                    format!("{}({})", op_str, args_str)
                }
            }
        }
    }
    /// A human-readable serialization of the Context.
    pub fn pretty(&self, sig: &Signature) -> String {
        Pretty::pretty(self, sig)
    }
    /// Every [`Atom`] used in `self`.
    ///
    /// [`Atom`]: enum.Atom.html
    pub fn atoms(&self) -> Vec<Atom> {
        let vars = self.variables().into_iter().map(Atom::Variable);
        let ops = self.operators().into_iter().map(Atom::Operator);
        vars.chain(ops).collect()
    }
    /// Every [`Variable`] used in `self`.
    ///
    /// [`Variable`]: struct.Variable.html
    pub fn variables(&self) -> Vec<Variable> {
        match *self {
            Context::Hole => vec![],
            Context::Variable(v) => vec![v],
            Context::Application { ref args, .. } => {
                args.iter().flat_map(Context::variables).unique().collect()
            }
        }
    }
    /// Every [`Operator`] used in `self`.
    ///
    /// [`Operator`]: struct.Operator.html
    pub fn operators(&self) -> Vec<Operator> {
        if let Context::Application { op, ref args } = *self {
            args.iter()
                .flat_map(Context::operators)
                .chain(iter::once(op))
                .unique()
                .collect()
        } else {
            vec![]
        }
    }
    /// A list of the [`Place`]s in `self` that are holes.
    ///
    /// [`Place`]: type.Place.html
    pub fn holes(&self) -> Vec<Place> {
        self.subcontexts()
            .into_iter()
            .filter_map(|(c, p)| {
                if let Context::Hole = *c {
                    Some(p)
                } else {
                    None
                }
            })
            .collect()
    }
    /// The head of the `Context`.
    pub fn head(&self) -> Option<Atom> {
        match self {
            Context::Hole => None,
            Context::Variable(v) => Some(Atom::Variable(*v)),
            Context::Application { op, .. } => Some(Atom::Operator(*op)),
        }
    }
    /// The args of the `Context`.
    pub fn args(&self) -> Vec<Context> {
        if let Context::Application { args, .. } = self {
            args.clone()
        } else {
            vec![]
        }
    }
    /// Every subcontext and its place, starting with the original context itself.
    pub fn subcontexts(&self) -> Vec<(&Context, Place)> {
        if let Context::Application { ref args, .. } = *self {
            let here = iter::once((self, vec![]));
            let subcontexts = args.iter().enumerate().flat_map(|(i, arg)| {
                arg.subcontexts()
                    .into_iter()
                    .zip(iter::repeat(i))
                    .map(|((t, p), i)| {
                        let mut a = vec![i];
                        a.extend(p);
                        (t, a)
                    })
            });
            here.chain(subcontexts).collect()
        } else {
            vec![(self, vec![])]
        }
    }
    /// The number of distinct [`Place`]s in `self`.
    ///
    /// [`Place`]: type.Place.html
    pub fn size(&self) -> usize {
        self.subcontexts().len()
    }
    /// Get the subcontext at the given [`Place`], or `None` if the place does not exist.
    ///
    /// [`Place`]: type.Place.html
    #[cfg_attr(feature = "cargo-clippy", allow(ptr_arg))]
    pub fn at(&self, place: &[usize]) -> Option<&Context> {
        self.at_helper(&*place)
    }
    fn at_helper(&self, place: &[usize]) -> Option<&Context> {
        if place.is_empty() {
            return Some(self);
        }
        match *self {
            Context::Application { ref args, .. } if place[0] <= args.len() => {
                args[place[0]].at_helper(&place[1..].to_vec())
            }
            _ => None,
        }
    }
    /// Create a copy of `self` where the `Context` at the given [`Place`] has been replaced with
    /// `subcontext`.
    ///
    /// [`Place`]: type.Place.html
    pub fn replace(&self, place: &[usize], subcontext: Context) -> Option<Context> {
        self.replace_helper(&*place, subcontext)
    }
    fn replace_helper(&self, place: &[usize], subcontext: Context) -> Option<Context> {
        if place.is_empty() {
            Some(subcontext)
        } else {
            match *self {
                Context::Application { op, ref args } if place[0] <= args.len() => {
                    if let Some(context) =
                        args[place[0]].replace_helper(&place[1..].to_vec(), subcontext)
                    {
                        let mut new_args = args.clone();
                        new_args.remove(place[0]);
                        new_args.insert(place[0], context);
                        Some(Context::Application { op, args: new_args })
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }
    /// Translate `self` into a [`Term`], if possible.
    ///
    /// [`Term`]: enum.Term.html
    pub fn to_term(&self) -> Result<Term, ()> {
        match *self {
            Context::Hole => Err(()),
            Context::Variable(v) => Ok(Term::Variable(v)),
            Context::Application { op, ref args } => {
                let mut mapped_args = vec![];
                for arg in args {
                    mapped_args.push(arg.to_term()?);
                }
                Ok(Term::Application {
                    op,
                    args: mapped_args,
                })
            }
        }
    }
}
impl From<Term> for Context {
    fn from(t: Term) -> Context {
        match t {
            Term::Variable(v) => Context::Variable(v),
            Term::Application { op, args } => {
                let args = args.into_iter().map(Context::from).collect();
                Context::Application { op, args }
            }
        }
    }
}

/// A first-order term: either a [`Variable`] or an application of an [`Operator`].
///
/// [`Variable`]: struct.Variable.html
/// [`Operator`]: struct.Operator.html
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Term {
    /// A concrete but unspecified `Term` (e.g. `x`, `y`)
    Variable(Variable),
    /// An [`Operator`] applied to zero or more `Term`s (e.g. (`f(x, y)`, `g()`)
    ///
    /// [`Operator`]: struct.Operator.html
    Application { op: Operator, args: Vec<Term> },
}
impl Term {
    /// A serialized representation of the Term.
    pub fn display(&self, sig: &Signature) -> String {
        match self {
            Term::Variable(v) => v.display(sig),
            Term::Application { op, args } => {
                let op_str = op.display(sig);
                if args.is_empty() {
                    op_str
                } else {
                    let args_str = args.iter().map(|arg| arg.display(sig)).join(" ");
                    format!("{}({})", op_str, args_str)
                }
            }
        }
    }
    /// A human-readable serialization of the Term.
    pub fn pretty(&self, sig: &Signature) -> String {
        Pretty::pretty(self, sig)
    }
    /// Every [`Atom`] used in the term.
    ///
    /// [`Atom`]: enum.Atom.html
    pub fn atoms(&self) -> Vec<Atom> {
        let vars = self.variables().into_iter().map(Atom::Variable);
        let ops = self.operators().into_iter().map(Atom::Operator);
        vars.chain(ops).collect()
    }
    /// Every [`Variable`] used in the term.
    ///
    /// [`Variable`]: struct.Variable.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::{Signature, parse_term,Term};
    /// let mut sig = Signature::default();
    /// let a = sig.new_op(2, Some(".".to_string()));
    /// let s = sig.new_op(0, Some("S".to_string()));
    /// let k = sig.new_op(0, Some("K".to_string()));
    ///
    /// let t = parse_term(&mut sig, "S K y_ z_").expect("parse of S K y_ z_");
    /// let t2 = parse_term(&mut sig, "S K y_ z_").expect("parse of S K y_ z_");
    /// let t3 = parse_term(&mut sig, "S K y_").expect("parse of S K y_");
    ///
    /// let vars = t.variables();
    /// let vars2 = t2.variables();
    /// let vars3 = t3.variables();
    ///
    /// assert_ne!(vars,vars2);
    /// assert_ne!(vars2,vars3);
    /// assert_ne!(vars,vars3);
    /// ```
    pub fn variables(&self) -> Vec<Variable> {
        match *self {
            Term::Variable(v) => vec![v],
            Term::Application { ref args, .. } => {
                args.iter().flat_map(Term::variables).unique().collect()
            }
        }
    }
    /// Every [`Operator`] used in the term.
    ///
    /// [`Operator`]: struct.Operator.html
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::{Signature, parse_term,Term};
    /// let mut sig = Signature::default();
    /// let a = sig.new_op(2, Some(".".to_string()));
    /// let s = sig.new_op(0, Some("S".to_string()));
    /// let k = sig.new_op(0, Some("K".to_string()));
    ///
    /// let t = parse_term(&mut sig, "S K y_ z_").expect("parse of S K y_ z_");
    /// let t2 = parse_term(&mut sig, "S K y_ z_").expect("parse of S K y_ z_");
    /// let t3 = parse_term(&mut sig, "S K y_").expect("parse of S K y_");
    ///
    /// let ops = t.operators();
    /// let ops2 = t2.operators();
    /// let ops3 = t3.operators();
    ///
    /// assert_eq!(ops,ops2);
    /// assert_eq!(ops2,ops3);
    /// ```
    pub fn operators(&self) -> Vec<Operator> {
        match *self {
            Term::Variable(_) => vec![],
            Term::Application { op, ref args } => args.iter()
                .flat_map(Term::operators)
                .chain(iter::once(op))
                .unique()
                .collect(),
        }
    }
    /// The head of the Term.
    pub fn head(&self) -> Atom {
        match self {
            Term::Variable(v) => Atom::Variable(*v),
            Term::Application { op, .. } => Atom::Operator(*op),
        }
    }
    /// The head of the Term.
    pub fn args(&self) -> Vec<Term> {
        match self {
            Term::Variable(_) => vec![],
            Term::Application { args, .. } => args.clone(),
        }
    }
    /// Every subterm and its place, starting with the original term itself.
    ///
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::{Signature, parse_term,Term};
    /// let mut sig = Signature::default();
    /// let a = sig.new_op(2, Some(".".to_string()));
    /// let s = sig.new_op(0, Some("S".to_string()));
    /// let k = sig.new_op(0, Some("K".to_string()));
    ///
    /// let t = parse_term(&mut sig, "S K y_ z_").expect("parse of S K y_ z_");
    ///
    /// t.subterms();
    /// ```
    pub fn subterms(&self) -> Vec<(&Term, Place)> {
        match *self {
            Term::Variable(_) => vec![(self, vec![])],
            Term::Application { ref args, .. } => {
                let here = iter::once((self, vec![]));
                let subterms = args.iter().enumerate().flat_map(|(i, arg)| {
                    arg.subterms()
                        .into_iter()
                        .zip(iter::repeat(i))
                        .map(|((t, p), i)| {
                            let mut a = vec![i];
                            a.extend(p);
                            (t, a)
                        })
                });
                here.chain(subterms).collect()
            }
        }
    }
    /// The number of distinct places in the term
    pub fn size(&self) -> usize {
        self.subterms().len()
    }
    /// Get the subterm at the given [`Place`], or `None` if the place does not exist in the term.
    ///
    /// [`Place`]: type.Place.html
    #[cfg_attr(feature = "cargo-clippy", allow(ptr_arg))]
    pub fn at(&self, place: &[usize]) -> Option<&Term> {
        self.at_helper(&*place)
    }
    fn at_helper(&self, place: &[usize]) -> Option<&Term> {
        if place.is_empty() {
            Some(self)
        } else {
            match *self {
                Term::Variable(_) => None,
                Term::Application { ref args, .. } => if place[0] <= args.len() {
                    args[place[0]].at_helper(&place[1..].to_vec())
                } else {
                    None
                },
            }
        }
    }
    /// Create a copy of `self` where the term at the given [`Place`] has been replaced with
    /// `subterm`.
    ///
    /// [`Place`]: type.Place.html
    #[cfg_attr(feature = "cargo-clippy", allow(ptr_arg))]
    pub fn replace(&self, place: &[usize], subterm: Term) -> Option<Term> {
        self.replace_helper(&*place, subterm)
    }
    fn replace_helper(&self, place: &[usize], subterm: Term) -> Option<Term> {
        if place.is_empty() {
            Some(subterm)
        } else {
            match *self {
                Term::Application { op, ref args } if place[0] <= args.len() => {
                    if let Some(term) = args[place[0]].replace_helper(&place[1..].to_vec(), subterm)
                    {
                        let mut new_args = args.clone();
                        new_args.remove(place[0]);
                        new_args.insert(place[0], term);
                        Some(Term::Application { op, args: new_args })
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }
    /// Given a mapping from [`Variable`]s to [`Term`]s, perform a substitution.
    ///
    /// [`Variable`]: struct.Variable.html
    /// [`Term`]: enum.Term.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::{Signature, parse_term,Term};
    /// # use std::collections::{HashMap};
    /// let (mut sig,ops) = Signature:: new(vec![
    ///     (2, Some(".".to_string())),
    ///     (0, Some("S".to_string())),
    ///     (0, Some("K".to_string())),
    /// ]);
    ///
    /// let term_before = parse_term(&mut sig, "S K y_ z_").expect("parse of S K y_ z_");
    /// let s_term = parse_term(&mut sig, "S").expect("parse of S");
    /// let k_term = parse_term(&mut sig, "K").expect("parse of K");
    ///
    /// let vars = sig.variables();
    /// let y = vars[0];
    /// let z = vars[1];
    ///
    /// let mut sub = HashMap::new();
    /// sub.insert(y,s_term);
    /// sub.insert(z,k_term);
    ///
    /// let term_after = parse_term(&mut sig, "S K S K").expect("parse of S K S K");
    /// let subbed_term = term_before.substitute(&sub);
    ///
    /// assert_eq!(term_after, subbed_term);
    /// ```

    pub fn substitute(&self, sub: &HashMap<Variable, Term>) -> Term {
        match *self {
            Term::Variable(ref v) => sub.get(v).unwrap_or(self).clone(),
            Term::Application { op, ref args } => Term::Application {
                op,
                args: args.iter().map(|t| t.substitute(sub)).collect(),
            },
        }
    }
    /// Take a vector of pairs of terms and perform a substitution on each term.
    fn constraint_substitute(
        cs: &[(Term, Term)],
        sub: &HashMap<Variable, Term>,
    ) -> Vec<(Term, Term)> {
        cs.iter()
            .map(|&(ref s, ref t)| (s.substitute(sub), t.substitute(sub)))
            .collect()
    }
    /// Compose two substitutions.
    fn compose(
        sub1: Option<HashMap<Variable, Term>>,
        sub2: Option<HashMap<Variable, Term>>,
    ) -> Option<HashMap<Variable, Term>> {
        match (sub1, sub2) {
            (Some(mut s1), Some(s2)) => {
                for (k, v) in s2 {
                    let v = v.substitute(&s1);
                    s1.insert(k, v);
                }
                Some(s1)
            }
            _ => None,
        }
    }
    /// Compute the [alpha equivalence] for two terms.
    ///
    /// [alpha equivalence]: https://en.wikipedia.org/wiki/Lambda_calculus#Alpha_equivalence
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::{Signature, parse_term,Term};
    /// let mut sig = Signature::default();
    /// let a = sig.new_op(2, Some(".".to_string()));
    /// let s = sig.new_op(0, Some("S".to_string()));
    /// let k = sig.new_op(0, Some("K".to_string()));
    ///
    /// let t = parse_term(&mut sig, "S K y_ z_").expect("parse of S K y_ z_");
    /// let t2 = parse_term(&mut sig, "S K y_ z_").expect("parse of S K y_ z_");
    /// let t3 = parse_term(&mut sig, "S K y_").expect("parse of S K y_");
    ///
    /// let alpha_1 = Term::alpha(&t,&t2);
    /// let alpha_2 = Term::alpha(&t,&t3);
    /// let alpha_3 = Term::alpha(&t2,&t3);
    /// assert_ne!(alpha_1,alpha_2);
    /// assert_eq!(alpha_2,alpha_3);
    /// ```
    pub fn alpha(t1: &Term, t2: &Term) -> Option<HashMap<Variable, Term>> {
        if Term::pmatch(vec![(t2.clone(), t1.clone())]).is_some() {
            Term::pmatch(vec![(t1.clone(), t2.clone())])
        } else {
            None
        }
    }
    /// Returns whether two terms are shape equivalent
    /// # Examples
    ///
    /// ```
    /// # use term_rewriting::{Signature, parse_term,Term};
    /// let mut sig = Signature::default();
    /// let a = sig.new_op(2, Some(".".to_string()));
    /// let s = sig.new_op(0, Some("S".to_string()));
    /// let k = sig.new_op(0, Some("K".to_string()));
    ///
    /// let t = parse_term(&mut sig, "S K y_ z_").expect("parse of S K y_ z_");
    /// let t2 = parse_term(&mut sig, "S K y_ z_").expect("parse of S K y_ z_");
    /// let t3 = parse_term(&mut sig, "S K y_").expect("parse of S K y_");
    ///
    /// assert!(Term::shape_equivalent(&t,&t2));
    /// assert!(! Term::shape_equivalent(&t,&t3));
    /// ```
    pub fn shape_equivalent(t1: &Term, t2: &Term) -> bool {
        let mut vmap = HashMap::new();
        let mut omap = HashMap::new();
        Term::se_helper(t1, t2, &mut vmap, &mut omap)
    }
    fn se_helper(
        t1: &Term,
        t2: &Term,
        vmap: &mut HashMap<Variable, Variable>,
        omap: &mut HashMap<Operator, Operator>,
    ) -> bool {
        match (t1, t2) {
            (&Term::Variable(v1), &Term::Variable(v2)) => v2 == *vmap.entry(v1).or_insert(v2),
            (
                &Term::Application {
                    op: op1,
                    args: ref args1,
                },
                &Term::Application {
                    op: op2,
                    args: ref args2,
                },
            ) => {
                op2 == *omap.entry(op1).or_insert(op2)
                    && args1
                        .into_iter()
                        .zip(args2)
                        .all(|(a1, a2)| Term::se_helper(a1, a2, vmap, omap))
            }
            _ => false,
        }
    }
    /// Given a vector of contraints, return a substitution which satisfies the constrants.
    /// If the constraints are not satisfiable, `None` is retuned. Constraints are in the form of
    /// patterns, where substitutions are only considered for variables in the first term of each
    /// pair.
    pub fn pmatch(cs: Vec<(Term, Term)>) -> Option<HashMap<Variable, Term>> {
        Term::unify_internal(cs, Unification::Match)
    }
    /// Given a vector of contraints, return a substitution which satisfies the constrants.
    /// If the constraints are not satisfiable, `None` is retuned.
    pub fn unify(cs: Vec<(Term, Term)>) -> Option<HashMap<Variable, Term>> {
        Term::unify_internal(cs, Unification::Unify)
    }
    /// the internal implementation of unify and match.
    fn unify_internal(
        mut cs: Vec<(Term, Term)>,
        utype: Unification,
    ) -> Option<HashMap<Variable, Term>> {
        let c = cs.pop();
        match c {
            None => Some(HashMap::new()),
            Some((ref s, ref t)) if s == t => Term::unify_internal(cs, utype),
            Some((
                Term::Application {
                    op: h1,
                    args: ref a1,
                },
                Term::Application {
                    op: h2,
                    args: ref a2,
                },
            )) if h1 == h2 =>
            {
                cs.append(&mut a1.clone().into_iter().zip(a2.clone().into_iter()).collect());
                Term::unify_internal(cs, utype)
            }
            Some((Term::Variable(var), ref t)) if !t.variables().contains(&&var) => {
                let mut st = HashMap::new();
                st.insert(var, t.clone());
                let mut cs = Term::constraint_substitute(&cs, &st);
                Term::compose(Term::unify_internal(cs, utype), Some(st))
            }
            Some((ref s, Term::Variable(var)))
                if !s.variables().contains(&&var) && utype != Unification::Match =>
            {
                let mut ts = HashMap::new();
                ts.insert(var, s.clone());
                let mut cs = Term::constraint_substitute(&cs, &ts);
                Term::compose(Term::unify_internal(cs, utype), Some(ts))
            }
            _ => None,
        }
    }
}

/// A rewrite rule equating a left-hand-side [`Term`] with one or more
/// right-hand-side [`Term`]s.
///
/// [`Term`]: enum.Term.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub lhs: Term,
    pub rhs: Vec<Term>,
}
impl Rule {
    /// A serialized representation of the Rule.
    pub fn display(&self, sig: &Signature) -> String {
        let lhs_str = self.lhs.display(sig);
        let rhs_str = self.rhs.iter().map(|rhs| rhs.display(sig)).join(" | ");
        format!("{} = {}", lhs_str, rhs_str)
    }
    /// A human-readable serialization of the Rule.
    pub fn pretty(&self, sig: &Signature) -> String {
        let lhs_str = self.lhs.pretty(sig);
        let rhs_str = self.rhs.iter().map(|rhs| rhs.pretty(sig)).join(" | ");
        format!("{} = {}", lhs_str, rhs_str)
    }
    /// Return the total number of subterms across all terms in the rule.
    pub fn size(&self) -> usize {
        self.lhs.size() + self.rhs.iter().map(Term::size).sum::<usize>()
    }
    /// Return the number of RHSs in the rule
    pub fn len(&self) -> usize {
        self.rhs.len()
    }
    /// Is the rule empty?
    pub fn is_empty(&self) -> bool {
        self.rhs.is_empty()
    }
    /// Give the lone RHS, if it exists
    pub fn rhs(&self) -> Option<Term> {
        if self.rhs.len() == 1 {
            Some(self.rhs[0].clone())
        } else {
            None
        }
    }
    /// Return a list of the clauses in the `Rule`.
    pub fn clauses(&self) -> Vec<Rule> {
        self.rhs
            .iter()
            .map(|rhs| Rule::new(self.lhs.clone(), vec![rhs.clone()]).unwrap())
            .collect()
    }
    /// logic ensuring that the `lhs` and `rhs` are compatible.
    fn is_valid(lhs: &Term, rhs: &[Term]) -> bool {
        // the lhs must be an application
        if let Term::Application { .. } = *lhs {
            // variables(rhs) must be a subset of variables(lhs)
            let lhs_vars: HashSet<_> = lhs.variables().into_iter().collect();
            let rhs_vars: HashSet<_> = rhs.iter().flat_map(Term::variables).collect();
            rhs_vars.is_subset(&lhs_vars)
        } else {
            false
        }
    }
    /// Construct a rewrite rule from a left-hand-side (LHS) [`Term`] with one
    /// or more right-hand-side (RHS) [`Term`]s. Returns `None` if the rule is
    /// not valid.
    ///
    /// Valid rules meet two conditions:
    ///
    /// 1. `lhs` is an [`Application`]. This prevents a single rule from
    ///    matching all possible terms
    /// 2. A [`Term`] in `rhs` can only use a [`Variable`] if it appears in
    ///    `lhs`. This prevents rewrites from inventing arbitrary terms.
    ///
    /// [`Term`]: enum.Term.html
    /// [`Application`]: enum.Term.html#variant.Application
    /// [`Variable`]: struct.Variable.html
    pub fn new(lhs: Term, rhs: Vec<Term>) -> Option<Rule> {
        if Rule::is_valid(&lhs, &rhs) {
            Some(Rule { lhs, rhs })
        } else {
            None
        }
    }
    /// Add a clause to the rule from a term.
    pub fn add(&mut self, t: Term) {
        let self_vars = self.lhs.variables();
        if t.variables().iter().all(|x| self_vars.contains(x)) {
            self.rhs.push(t)
        }
    }
    /// Add clauses to the rule from another rule.
    pub fn merge(&mut self, r: &Rule) {
        if let Some(s) = Term::alpha(&self.lhs, &r.lhs) {
            for rhs in r.rhs.clone() {
                self.rhs.push(rhs.substitute(&s));
            }
        }
    }
    /// Discard clauses from the rule.
    pub fn discard(&mut self, r: &Rule) -> bool {
        if let Some(sub) = Term::alpha(&self.lhs, &r.lhs) {
            let terms = r.rhs
                .iter()
                .map(|rhs| rhs.substitute(&sub))
                .collect::<Vec<Term>>();
            self.rhs.retain(|x| !terms.contains(x));
            true
        } else {
            false
        }
    }
    /// Check whether the rule contains certain clauses.
    pub fn contains(&self, r: &Rule) -> Option<HashMap<Variable, Term>> {
        if let Some(sub) = Term::alpha(&self.lhs, &r.lhs) {
            if r.rhs
                .iter()
                .all(|rhs| self.rhs.contains(&rhs.substitute(&sub)))
            {
                Some(sub)
            } else {
                None
            }
        } else {
            None
        }
    }
    /// Get all the Variables in the Rule.
    pub fn variables(&self) -> Vec<Variable> {
        self.lhs.variables()
    }
    /// Get all the Operators in the Rule.
    pub fn operators(&self) -> Vec<Operator> {
        let lhs = self.lhs.operators().into_iter();
        let rhs = self.rhs.iter().flat_map(Term::operators);
        lhs.chain(rhs).unique().collect()
    }
    /// Get all the subterms and places in a Rule.
    pub fn subterms(&self) -> Vec<(usize, &Term, Place)> {
        let lhs = self.lhs.subterms().into_iter().map(|(t, p)| (0, t, p));
        let rhs = self.rhs.iter().enumerate().flat_map(|(i, rhs)| {
            iter::repeat(i + 1)
                .zip(rhs.subterms())
                .into_iter()
                .map(|(i, (t, p))| (i, t, p))
        });
        lhs.chain(rhs).collect()
    }
    /// Get a specific subterm in a Rule
    pub fn at(&self, p: &[usize]) -> Option<&Term> {
        if p[0] == 0 {
            self.lhs.at(&p[1..].to_vec())
        } else {
            self.rhs[p[1]].at(&p[2..].to_vec())
        }
    }
    /// Replace one subterm with another in a Rule
    pub fn replace(&self, place: &[usize], subterm: Term) -> Option<Rule> {
        if place[0] == 0 {
            if let Some(lhs) = self.lhs.replace(&place[1..].to_vec(), subterm) {
                Rule::new(lhs, self.rhs.clone())
            } else {
                None
            }
        } else if let Some(an_rhs) = self.rhs[place[1]].replace(&place[2..].to_vec(), subterm) {
            let mut rhs = self.rhs.clone();
            rhs.remove(place[1]);
            rhs.insert(place[1], an_rhs);
            Rule::new(self.lhs.clone(), rhs)
        } else {
            None
        }
    }
    /// Match one rule against another.
    pub fn pmatch(r1: Rule, r2: Rule) -> Option<HashMap<Variable, Term>> {
        let cs = iter::once((r1.lhs, r2.lhs)).chain(r1.rhs.into_iter().zip(r2.rhs));
        Term::pmatch(cs.collect())
    }
    /// Unify two rules.
    pub fn unify(r1: Rule, r2: Rule) -> Option<HashMap<Variable, Term>> {
        let cs = iter::once((r1.lhs, r2.lhs)).chain(r1.rhs.into_iter().zip(r2.rhs));
        Term::unify(cs.collect())
    }
    /// Compute the alpha equivalence between two rules.
    pub fn alpha(r1: &Rule, r2: &Rule) -> Option<HashMap<Variable, Term>> {
        if Rule::pmatch(r2.clone(), r1.clone()).is_some() {
            Rule::pmatch(r1.clone(), r2.clone())
        } else {
            None
        }
    }
    /// Substitute through a rule
    pub fn substitute(&self, sub: &HashMap<Variable, Term>) -> Rule {
        Rule::new(
            self.lhs.substitute(sub),
            self.rhs.iter().map(|rhs| rhs.substitute(sub)).collect(),
        ).unwrap()
    }
}

/// A [`Rule`] with holes; a sort of [`Rule`] template.
///
/// [`Context`]: enum.Context.html
/// [`Rule`]: struct.Rule.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuleContext {
    pub lhs: Context,
    pub rhs: Vec<Context>,
}

/// A first-order term rewriting system.
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct TRS {
    pub rules: Vec<Rule>,
}
impl TRS {
    /// Constructs a term rewriting system from a list of rules.
    pub fn new(rules: Vec<Rule>) -> TRS {
        TRS { rules }
    }
    /// The number of rules in the TRS.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Are there any rules in the TRS?
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Return the number of total number of subterms across all rules.
    pub fn size(&self) -> usize {
        self.rules.iter().map(Rule::size).sum()
    }
    /// Remove the rule at index `i`.
    pub fn remove(&mut self, i: usize) {
        self.rules.remove(i);
    }
    /// Delete `r` if it exists in `self`.
    pub fn delete(&mut self, r: &Rule) {
        for rule in &mut self.rules {
            if rule.discard(&r) {
                break;
            }
        }
        self.rules.retain(|rule| !rule.is_empty());
    }
    /// Get `r` if it exists in `self`.
    pub fn get(&self, r: &Rule) -> Option<(usize, Rule)> {
        for (i, rule) in self.rules.iter().enumerate() {
            if let Some(sub) = rule.contains(r) {
                return Some((i, r.substitute(&sub)));
            }
        }
        None
    }
    /// Put `r` at index `i` in `self`.
    pub fn set(&mut self, i: usize, r: Rule) {
        if let Some(idx) = self.rules
            .iter()
            .position(|rule| Term::alpha(&rule.lhs, &r.lhs).is_some())
        {
            self.rules[idx].merge(&r);
            let rule = self.rules.remove(idx);
            self.rules.insert(i, rule);
        } else {
            self.rules.insert(i, r);
        }
    }
    /// Put `r` in the TRS as the first Rule.
    pub fn push(&mut self, r: Rule) {
        self.set(0, r)
    }
    /// Push a series of rules into the TRS.
    pub fn pushes(&mut self, rs: Vec<Rule>) {
        for r in rs {
            self.push(r);
        }
    }
    /// A serialized representation of the TRS.
    pub fn display(&self, sig: &Signature) -> String {
        self.rules
            .iter()
            .map(|r| format!("{};", r.display(sig)))
            .join("\n")
    }
    /// A human-readable serialization of the TRS.
    pub fn pretty(&self, sig: &Signature) -> String {
        self.rules
            .iter()
            .map(|r| format!("{};", r.pretty(sig)))
            .join("\n")
    }
    /// All the clauses in the TRS.
    pub fn clauses(&self) -> Vec<Rule> {
        self.rules.iter().flat_map(Rule::clauses).collect()
    }
    /// Move rule `i` to index `j`
    pub fn move_rule(&mut self, i: usize, j: usize) -> bool {
        if self.rules.len() > max(i, j) {
            let rule = self.rules.remove(i);
            self.rules.insert(j, rule);
            true
        } else {
            false
        }
    }
    /// Replace Rule `r1` with Rule `r2`
    pub fn replace(&mut self, r1: &Rule, r2: Rule) {
        if let Some((idx, mut rule)) = self.get(r1) {
            rule.discard(r1);
            self.push(r2);
            if rule.is_empty() {
                self.remove(idx);
            }
        }
    }
    /// Do two TRSs unify?
    pub fn unifies(trs1: TRS, trs2: TRS) -> bool {
        trs1.len() == trs2.len()
            && trs1.rules
                .into_iter()
                .zip(trs2.rules)
                .all(|(r1, r2)| Rule::unify(r1, r2).is_some())
    }
    /// Does one TRS match another?
    pub fn pmatches(trs1: TRS, trs2: TRS) -> bool {
        trs1.len() == trs2.len()
            && trs1.rules
                .into_iter()
                .zip(trs2.rules)
                .all(|(r1, r2)| Rule::pmatch(r1, r2).is_some())
    }
    /// Are two TRSs alpha equivalent?
    pub fn alphas(trs1: &TRS, trs2: &TRS) -> bool {
        TRS::pmatches(trs2.clone(), trs1.clone()) && TRS::pmatches(trs1.clone(), trs2.clone())
    }
    /// All the operators in the TRS
    pub fn operators(&self) -> Vec<Operator> {
        self.rules
            .iter()
            .flat_map(Rule::operators)
            .unique()
            .collect()
    }
    // Return rewrites modifying the entire term, if possible, else None.
    fn rewrite_head(&self, term: &Term) -> Option<Vec<Term>> {
        for rule in &self.rules {
            if let Some(ref sub) = Term::pmatch(vec![(rule.lhs.clone(), term.clone())]) {
                return Some(rule.rhs.iter().map(|x| x.substitute(sub)).collect());
            }
        }
        None
    }
    // Return rewrites modifying subterms, if possible, else None.
    fn rewrite_args(&self, term: &Term) -> Option<Vec<Term>> {
        if let Term::Application { op, ref args } = *term {
            for (i, arg) in args.iter().enumerate() {
                if let Some(v) = self.rewrite(arg) {
                    let res = v.iter()
                        .map(|x| {
                            let mut args = args.clone();
                            args[i] = x.clone();
                            Term::Application { op, args }
                        })
                        .collect();
                    return Some(res);
                }
            }
            None
        } else {
            None
        }
    }
    /// Perform a single rewrite step using a normal-order (leftmost-outermost)
    /// rewrite strategy.
    pub fn rewrite(&self, term: &Term) -> Option<Vec<Term>> {
        match *term {
            Term::Variable(_) => None,
            ref app => self.rewrite_head(app).or_else(|| self.rewrite_args(app)),
        }
    }
    // TODO: pub fn make_deterministic
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_eq() {
        let mut sig = Signature::default();

        let v1 = sig.new_var(Some("blah".to_string()));
        let v2 = sig.new_var(None);
        let v3 = Variable { id: 0 };

        assert_eq!(v1, v1);
        assert_ne!(v1, v2);
        assert_eq!(v1, v3);
    }
}
