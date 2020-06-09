use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};
use itertools::Itertools;

struct Arg {
    short: Option<char>,
    long: Option<String>,
    inputs: Vec<String>,
    info: String,
}

impl Arg {
    pub fn new(short: Option<char>, long: Option<&str>, inputs: Vec<&str>, info: &str) -> Self {
	Self {
	    short: short,
	    long:   long.map(|s| s.to_string()),
	    inputs: inputs.into_iter().map(|s| s.to_string()).collect(),
	    info: info.to_string(),
	}
    }
}

pub struct Manpage {
    name: String,
    version: String,
    section: u32,
    desc_short: String,
    descriptions: Vec<(String, String)>,
    args: Vec<Arg>,
}

impl Manpage {
    pub fn new(name: &str, version: &str, section: u32) -> Self {
	Self {
	    name: name.to_string(),
	    version: version.to_string(),
	    section,
	    desc_short: String::new(),
	    descriptions: Vec::new(),
	    args: Vec::new(),
	}
    }

    pub fn desc_short(&mut self, desc_short: &str) -> &mut Self {
	self.desc_short = desc_short.to_string();
	self
    }

    pub fn description(&mut self, name: &str, desc: &str) -> &mut Self {
	self.descriptions.push((name.to_string(), desc.to_string()));
	self
    }

    pub fn arg(&mut self, short: Option<char>, long: Option<&str>, inputs: Vec<&str>, info: &str) -> &mut Self {
	self.args.push(Arg::new(short, long, inputs, info));
	self
    }

    pub fn write_to_file(&self, path: PathBuf) {
	let heading = format!(".TH {} {} {}\\-{}",
			      self.name.to_uppercase(),
			      self.section,
			      self.name,
			      self.version);
	let name = format!(".SH NAME\n{} \\- {}", self.name, self.desc_short);

	let description = format!(".SH DESCRIPTION\n{}",
				  self.descriptions.iter().map(|(name, description)| {
				      format!(".B {}\n{}", name, description)
				  }).join("\n.P\n"));
	let (synopsis, options) = self.gen_argstrs();

	let mut usage_file = File::open(concat!(env!("CARGO_MANIFEST_DIR"),
						"/src/usage.1")).unwrap();
	let mut usage = String::new();
	if let Err(err) = usage_file.read_to_string(&mut usage) {
	    panic!("Could not read usage man file {}", err);	
	}
	let mut see_also_file = File::open(concat!(env!("CARGO_MANIFEST_DIR"),
						"/src/see_also.1")).unwrap();
	let mut see_also = String::new();
	if let Err(err) = see_also_file.read_to_string(&mut see_also) {
	    panic!("Could not read see_also man file {}", err);	
	}
	
	let manpage = vec![heading, name, synopsis, description,
			   options, usage, see_also].join("\n");
	match File::create(&path) {
	    Ok(mut file) => {
		if let Err(err) = file.write_all(manpage.as_bytes()) {
		    panic!("Could not write to file '{}': {}", path.to_string_lossy(), err);
		}
	    },
	    Err(err) => panic!("Could not open file '{}' for writing: {}",
			       path.to_string_lossy(), err),
	}
    }

    fn gen_argstrs(&self) -> (String, String) {
	let mut arg_shorts      = Vec::new();
	let mut arg_other_short = Vec::new();
	let mut arg_other_long  = Vec::new();
	let mut arg_other_both  = Vec::new();
	for arg in &self.args {
	    match (arg.short, arg.long.as_ref(), arg.inputs.len()) {
		(Some(_), None, 0)    => arg_shorts.push(arg),
		(Some(_), None, _)    => arg_other_short.push(arg),
		(None, Some(_), _)    => arg_other_long.push(arg),
		(Some(_), Some(_), _) => arg_other_both.push(arg),
		(None, None, _) => panic!("yaml arguement must have some flag"),
	    }
	}

	let synopsis_shorts_str = if arg_shorts.len() == 0 {
	    String::new()
	} else {
	    format!(".RB [ \\-{} ]\n",
		    arg_shorts.into_iter()
		    .map(|arg| arg.short.unwrap()).collect::<String>())
	};
	
	let synopsis_other_short_str =
	    if arg_other_short.len() == 0 {
		String::new()
	    } else {
		format!("{}\n", arg_other_short.iter().map(|syn| {
		    format!(".RB [ \\-{}\n{} ]", syn.short.unwrap(), syn.inputs.iter().map(|name| {
		    format!(".IR {}", name)
		}).join("\n"))
	    }).join("\n"))
    };
	
	let synopsis_other_long_str =
	    if arg_other_long.len() == 0 {
		String::new()
	    } else {
		format!("{}\n", arg_other_long.iter().map(|syn| {
		    if syn.inputs.len() > 0 {
			format!(".RB [ \\-\\-{}\n{} ]", syn.long.as_ref().unwrap(),
				syn.inputs.iter().map(|name| {
				    format!(".IR {}", name)
				}).join("\n"))
		    } else {
			format!(".RB [ \\-\\-{} ]", syn.long.as_ref().unwrap())
		    }
		}).join("\n"))
	    };
	
	let synopsis_other_both_str =
	    if arg_other_both.len() == 0 {
		String::new()
	    } else {
		format!("{}\n", arg_other_both.iter().map(|syn| {
		    if syn.inputs.len() > 0 {
			format!(".RB [ \\-{}|\\-\\-{}\n{} ]", syn.short.unwrap(),
				syn.long.as_ref().unwrap(),
				syn.inputs.iter().map(|name| {
				    format!(".IR {}", name)
				}).join("\n"))
		    } else {
			format!(".RB [ \\-{}|\\-\\-{} ]", syn.short.unwrap(),
				syn.long.as_ref().unwrap())
		    }
		}).join("\n"))
	    };
	

	let synopsis = format!(".SH SYNOPSIS\n\
		 .B {}\n\
		 {}{}{}{}\n\
		 .P\n\
		 .BR dmenu_run \" ...\"",
		self.name,
		synopsis_shorts_str,
		synopsis_other_short_str,
		synopsis_other_long_str,
			       synopsis_other_both_str);


	(synopsis, String::new())
    }
}
