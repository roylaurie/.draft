# Universal API

## asmov / frogopts

### FrogOpts
Parses command-line options (flags), commands, and parameters based on configuration.

#### constructor( )
IN OptionConfig optionConfig  
IN Validator[ ] validators (optional)  
IN Settings settings (optional) 

#### .parse( )
Parses arguments and determines the raw values for options, commands, and parameters.

IN string[ ] arguments  
OUT Result commandline

#### .process( )
Parses arguments and then validates and transforms values for options, commands, and parameters.
Parameter validation is optional.

IN string[ ] arguments  
IN ParameterConfig parameterConfig (optional)  
OUT Result commandline 

### OptionConfig
Provides a schema for all options available, including type constraints and data validation. 

### Validator
Namespaced function that can be referenced by OptionConfig and ParameterConfig schemas. Performs
type and data validation.

### Settings
General configuration for FrogOpts.

### Result
Immutable data from command-line parsing that provides each option, command, and parameter
parsed and/or processed.

## asmov / bullfrog / common

### Module
Provides base-class support for custom modules and operation methods. Platform specific.

#### constructor( )

#### .init( )
IN asmov.frogopts.Result frogopts (optional)  
ASYNC

#### .start( )
ASYNC

#### .stop( )
ASYNC

### BullfrogOpts
Handles parsing a commandline string by configuring FrogOpts according to bullfrog package
configuration for modules, operations, and parameters.

#### constructor( )
Configures FrogOpts using the provided ModulesConfig.

IN ModulesConfig modulesConfig

#### process()
Returns a fully parsed and processed FrogOpts Result based on the package configuration provided.

OUT asmov.frogopts.Result frogopts



