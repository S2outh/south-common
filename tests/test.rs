//use south_common::types::{pyro::PyroCommand, *};
//use south_common::definitions;
//use south_common::chell::{ChellValue, ChellDefinition};

//#[test]
//pub fn test_smt() {
//    let mut buf = [0u8; 100];
//    let len = Telecommand::Pyro(PyroCommand::Arm(0)).write(&mut buf).unwrap();
//    println!("bytes: {:?}", &buf[..len]);
//    println!("id: {:?}", definitions::internal_msgs::Telecommand.id());
//
//    let len = Telecommand::Pyro(PyroCommand::Fire(0)).write(&mut buf).unwrap();
//    println!("bytes: {:?}", &buf[..len]);
//    println!("id: {:?}", definitions::internal_msgs::Telecommand.id());
//
//    panic!();
//}
