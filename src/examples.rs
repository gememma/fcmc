use crate::fmc::{FmcClosure, FmcTerm};
use crate::kam::{Closure, State};
use crate::lambdaterm::LambdaTerm;
use crate::pam::PState;
use crate::sam::SLambdaTerm::{Pop, Push, Seq, Skip, Variable};
use crate::sam::{SClosure, SLambdaTerm, SState};

impl LambdaTerm {
    /// (\x. \y. x) (\a. \b. a) (\a. \b. b)
    pub fn term1() -> Self {
        LambdaTerm::Apply {
            t1: box LambdaTerm::Apply {
                t1: box LambdaTerm::Lambda {
                    arg: "x".to_string(),
                    body: box LambdaTerm::Lambda {
                        arg: "y".to_string(),
                        body: box LambdaTerm::new_var("x"),
                    },
                },
                t2: box LambdaTerm::new_bool(true),
            },
            t2: box LambdaTerm::new_bool(false),
        }
    }

    /// (\b. (\a. \x. (\y. a) x b) (\a. \b. a)) (\z. z) (\a. \b. b)
    pub fn term2() -> Self {
        LambdaTerm::Apply {
            t1: box LambdaTerm::Apply {
                t1: box LambdaTerm::Lambda {
                    arg: "b".to_string(),
                    body: box LambdaTerm::Apply {
                        t1: box LambdaTerm::Lambda {
                            arg: "a".to_string(),
                            body: box LambdaTerm::Lambda {
                                arg: "x".to_string(),
                                body: box LambdaTerm::Apply {
                                    t1: box LambdaTerm::Apply {
                                        t1: box LambdaTerm::Lambda {
                                            arg: "y".to_string(),
                                            body: box LambdaTerm::new_var("a"),
                                        },
                                        t2: box LambdaTerm::new_var("x"),
                                    },
                                    t2: box LambdaTerm::new_var("b"),
                                },
                            },
                        },
                        t2: box LambdaTerm::new_bool(true),
                    },
                },
                t2: box LambdaTerm::Lambda {
                    arg: "z".to_string(),
                    body: box LambdaTerm::new_var("z"),
                },
            },
            t2: box LambdaTerm::new_bool(false),
        }
    }

    /// \a. \x. (\y. a) x b
    pub fn example5() -> Box<Self> {
        box LambdaTerm::Lambda {
            arg: "a".to_string(),
            body: box LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box LambdaTerm::Apply {
                    t1: box LambdaTerm::Apply {
                        t1: box LambdaTerm::Lambda {
                            arg: "y".to_string(),
                            body: box LambdaTerm::new_var("a"),
                        },
                        t2: box LambdaTerm::new_var("x"),
                    },
                    t2: box LambdaTerm::new_var("b"),
                },
            },
        }
    }

    /// \a. \x. (\y. a) x z
    pub fn example6() -> Box<Self> {
        box LambdaTerm::Lambda {
            arg: "a".to_string(),
            body: box LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box LambdaTerm::Apply {
                    t1: box LambdaTerm::Apply {
                        t1: box LambdaTerm::Lambda {
                            arg: "y".to_string(),
                            body: box LambdaTerm::new_var("a"),
                        },
                        t2: box LambdaTerm::new_var("x"),
                    },
                    t2: box LambdaTerm::new_var("z"),
                },
            },
        }
    }

    /// \c. \a. ((\a. c) a) (\f. \x. x)
    pub fn example7() -> Box<Self> {
        box LambdaTerm::Lambda {
            arg: "c".to_string(),
            body: box LambdaTerm::Lambda {
                arg: "a".to_string(),
                body: box LambdaTerm::Apply {
                    t1: box LambdaTerm::Apply {
                        t1: box LambdaTerm::Lambda {
                            arg: "a".to_string(),
                            body: box LambdaTerm::new_var("c"),
                        },
                        t2: box LambdaTerm::new_var("a"),
                    },
                    t2: box LambdaTerm::new_num(0),
                },
            },
        }
    }
}

impl PState {
    /// (\x. \y. x, [\a. \b. a, \a. \b. b, *])
    pub fn state1() -> Self {
        PState::new(
            LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
            },
            vec![LambdaTerm::new_bool(false), LambdaTerm::new_bool(true)],
        )
    }

    /// (\x. \y. x,[*])
    pub fn state2() -> Self {
        PState::new(
            LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
            },
            vec![],
        )
    }

    /// (y, [\x. \y. x, *])
    pub fn state3() -> Self {
        PState::new(
            LambdaTerm::new_var("y"),
            vec![LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
            }],
        )
    }
}

impl Closure {
    /// Closure from [`state2()`][State::state2()]
    pub fn closure2() -> Self {
        Closure::new(
            LambdaTerm::Apply {
                t1: box LambdaTerm::Lambda {
                    arg: "x".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
                t2: box LambdaTerm::new_var("y"),
            },
            vec![(
                "y".to_string(),
                Closure::new(
                    LambdaTerm::Lambda {
                        arg: "z".to_string(),
                        body: box LambdaTerm::new_var("z"),
                    },
                    vec![],
                ),
            )],
        )
    }

    /// Closure from [`state4()`][State::state4()]
    pub fn closure4() -> Self {
        Closure::new(
            LambdaTerm::Lambda {
                arg: "y".to_string(),
                body: box LambdaTerm::new_var("x"),
            },
            vec![],
        )
    }
}

impl State {
    /// ((\x. x) y,[("y",\z. z,[])],[])
    pub fn state2() -> Self {
        State::new(Closure::closure2(), vec![])
    }

    /// (\y. x,[],[(z,[("z",\a. b,[("b",c,[])])])])
    pub fn state4() -> Self {
        State::new(
            Closure::closure4(),
            vec![Closure::new(
                LambdaTerm::new_var("z"),
                vec![(
                    "z".to_string(),
                    Closure::new(
                        LambdaTerm::Lambda {
                            arg: "a".to_string(),
                            body: box LambdaTerm::new_var("b"),
                        },
                        vec![(
                            "b".to_string(),
                            Closure::new(LambdaTerm::new_var("b"), vec![]),
                        )],
                    ),
                )],
            )],
        )
    }
}

impl SLambdaTerm {
    /// <x>.[x].[x]
    pub fn term1() -> Self {
        Pop {
            arg: "x".to_string(),
            next: box Push {
                term: box Variable {
                    name: "x".to_string(),
                },
                next: box Push {
                    term: box Variable {
                        name: "x".to_string(),
                    },
                    next: box Skip,
                },
            },
        }
    }

    /// <x>.<y>
    pub fn term2() -> Self {
        Pop {
            arg: "x".to_string(),
            next: box Pop {
                arg: "y".to_string(),
                next: box Skip,
            },
        }
    }

    ///[<x>. [x]]. <f>. f; f; f
    pub fn term3() -> Self {
        Push {
            term: box Pop {
                arg: "x".to_string(),
                next: box Push {
                    term: box Variable {
                        name: "x".to_string(),
                    },
                    next: box Skip,
                },
            },
            next: box Pop {
                arg: "f".to_string(),
                next: box Seq {
                    term: box Variable {
                        name: "f".to_string(),
                    },
                    next: box Seq {
                        term: box Variable {
                            name: "f".to_string(),
                        },
                        next: box Seq {
                            term: box Variable {
                                name: "f".to_string(),
                            },
                            next: box Skip,
                        },
                    },
                },
            },
        }
    }

    /// [[y].[x]].<z>.z.z
    pub fn term4() -> Self {
        Push {
            term: box Push {
                term: box Variable {
                    name: "y".to_string(),
                },
                next: box Push {
                    term: box Variable {
                        name: "x".to_string(),
                    },
                    next: box Skip,
                },
            },
            next: box Pop {
                arg: "z".to_string(),
                next: box Seq {
                    term: box Variable {
                        name: "z".to_string(),
                    },
                    next: box Seq {
                        term: box Variable {
                            name: "z".to_string(),
                        },
                        next: box Skip,
                    },
                },
            },
        }
    }
}

impl SClosure {
    /// ([<x>. [x]]. <f>. f; f; f, [("y", [<x>. [x]]. <f>. f; f; f, [])])
    pub fn closure1() -> Self {
        SClosure::new(
            SLambdaTerm::term3(),
            vec![("y".to_string(), SClosure::new(SLambdaTerm::term3(), vec![]))],
        )
    }

    /// (<x>.<y>, ["y", <x>.[x].[x], []])
    pub fn closure2() -> Self {
        SClosure::new(
            SLambdaTerm::term2(),
            vec![("y".to_string(), SClosure::new(SLambdaTerm::term1(), vec![]))],
        )
    }
}

impl SState {
    /// (([<x>. [x]]. <f>. f; f; f, [("y", [<x>. [x]]. <f>. f; f; f, [])]), [(<x>.<y>, ["y", <x>.[x].[x], []]), ([<x>. [x]]. <f>. f; f; f, [("y", [<x>. [x]]. <f>. f; f; f, [])])], [])
    pub fn state1() -> Self {
        SState::new(
            SClosure::closure1(),
            vec![SClosure::closure2(), SClosure::closure1()],
            vec![],
        )
    }

    ///
    pub fn state2() -> Self {
        SState::new(
            SClosure::new(Skip, vec![]),
            vec![SClosure::closure2(), SClosure::closure1()],
            vec![],
        )
    }
}

impl FmcTerm {
    pub fn term1() -> Self {
        FmcTerm::new_seq(
            FmcTerm::new_push(FmcTerm::new_variable("x"), "a".to_string(), FmcTerm::Skip),
            FmcTerm::new_pop("a".to_string(), "y", FmcTerm::Skip),
        )
    }
}

impl FmcClosure {
    pub fn closure1() -> Self {
        FmcClosure::new(FmcTerm::term1(), vec![])
    }
}
