# Configuration

Most of the commands can be customized either providing parameters at run-time or by passing a config file. The config file is specified using the `--config` or `-c' option. 

Config files use [TOML syntax](https://toml.io/). 

By default, rudof uses a [default_config.toml](https://github.com/rudof-project/rudof/blob/master/rudof_lib/src/default_config.toml) file. 

If you want to customize rudof's behaviour you can create your own template making a copy of the previous one.

> Note: we would like to have an automated specification of the different keys/values that can be written in the COnfig file. Meanwhile, the internal structure where the Config is defined is [here](https://github.com/rudof-project/rudof/blob/master/rudof_lib/src/rudof_config.rs).  
