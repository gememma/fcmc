mod tests {
    use crate::fcmc::{FcmcProgramState, FcmcTerm};
    use crate::parser::FcmcTermParser;

    #[test]
    fn term1() {
        let parser = FcmcTermParser::new();
        let parsed: FcmcTerm = parser.parse("{[[x]~out]~a}.~a<y>.y").expect("");
        assert_eq!(parsed, FcmcTerm::term1());
    }

    #[test]
    fn term2() {
        let parser = FcmcTermParser::new();
        let parsed: FcmcTerm = parser.parse("[a<x>.x]b.[[z]out]a.b<f>.f").expect("");
        assert_eq!(
            FcmcProgramState::run(parsed),
            FcmcProgramState::run(FcmcTerm::term2())
        );
    }

    #[test]
    fn term3() {
        let parser = FcmcTermParser::new();
        let parsed: FcmcTerm = parser
            .parse("[x]~a.({~a<y>.[[y]~out]~b.[*]~t1}.({~b<z>.z;[*]~t2}.~t1<n>.~t2<m>.*))")
            .expect("");
        assert_eq!(
            FcmcProgramState::run(parsed),
            FcmcProgramState::run(FcmcTerm::term3())
        );
    }
}
