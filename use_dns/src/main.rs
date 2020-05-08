use std::env;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use trust_dns::rr::record_type::RecordType;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide a name to query!");
        std::process::exit(1);
    }

    let query = format!("{}.", args[1]);

    println!("");
    println!("use default config:");
    let resolver = 
        Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let response = resolver.lookup_ip(query.as_str());
    for ans in response.iter() {
        println!("{:?}", ans);
    }

    println!("");
    println!("use system config:");
    let resolver = 
        Resolver::from_system_conf().unwrap();
    let response = resolver.lookup_ip(query.as_str());
    for ans in response.iter() {
        println!("{:?}", ans);
    }

    println!("");
    println!("use ns:");
    let ns = resolver.lookup(query.as_str(), RecordType::NS);
    for ans in ns.iter() {
        println!("{:?}", ans);
    }

    println!("Hello, world!");
}
