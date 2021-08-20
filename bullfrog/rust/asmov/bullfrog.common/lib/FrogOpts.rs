#[allow(non_camel_case_types)]
trait iFrogOpts {
    fn parse(&self, arguments: &[&str]);
    fn process(&self, arguments: &[&str], parameter_config: &ParameterConfig);
}

#[allow(non_camel_case_types)]
trait cFrogOpts {
    fn new(settings: &Settings, option_config: &OptionConfig, module_config: &ModuleConfig) -> FrogOpts;
}

struct Settings {

}

struct OptionConfig {

}

struct OptionInputConfig {

}

struct ParameterConfig {

}

struct ParameterInputConfig {
    name: String,
    input_type: String,
    flag: char,
    required: bool,
    default: String,
    input_enum: u32,
}

struct ModuleConfig {

}

struct FrogOpts { }

impl cFrogOpts for FrogOpts {
    fn new(settings: &Settings, option_config: &OptionConfig, module_config: &ModuleConfig) -> FrogOpts {
        todo!()
    }
}

impl iFrogOpts for FrogOpts {
    fn parse(&self, arguments: &[&str]) {
        todo!()
    }

    fn process(&self, arguments: &[&str], parameter_config: &ParameterConfig) {
        todo!()
    }
}
