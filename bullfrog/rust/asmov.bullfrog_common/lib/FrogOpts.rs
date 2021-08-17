trait FrogOptsTrait {
    fn parse(args: &[&str]);
    fn process(args: &[&str], parameterCfg: &ParameterConfig);

}

struct ParameterConfig {

}

struct ParameterInputConfig {
    name: &str,
    input_type: &str,
    flag: &char,
    required: &bool,
    default: &str,
    input_enum: &[&str]
}

struct FrogOpts();

impl FrogOptsTrait for FrogOpts {
    fn parse(args: &[str]) {
        for arg in args {

        }
    }
}