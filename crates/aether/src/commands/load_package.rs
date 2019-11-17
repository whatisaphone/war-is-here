use crate::{darksiders1::gfc, hooks::ON_POST_UPDATE_QUEUE};
use std::slice;

pub fn run(command: &str) -> Result<(), &'static str> {
    let args = match parse(command) {
        Ok(args) => args,
        Err(()) => return Err("parse error"),
    };
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || go(&args)));
    Ok(())
}

fn parse(command: &str) -> Result<Args, ()> {
    let mut words = command.split_ascii_whitespace();
    words.next().ok_or(())?;
    let package_name = words.next().ok_or(())?.to_string();
    Ok(Args { package_name })
}

struct Args {
    package_name: String,
}

fn go(args: &Args) {
    let resource_manager = <gfc::Singleton<gfc::ResourceManager>>::get_instance();
    let package_name = gfc::HString::from_str(&args.package_name);
    let package_id = resource_manager.get_package_id(&package_name);
    println!("loading package {:?} ({})", args.package_name, package_id);
    let package_id = resource_manager.get_permanent_id(package_id);
    resource_manager.load_packages(slice::from_ref(&package_id), false);
}
