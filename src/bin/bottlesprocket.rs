extern crate clap;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate bottlesprocket;

use bottlesprocket::make_command;
use bottlesprocket::HouseCode;
use bottlesprocket::Device;
use bottlesprocket::Command;
use bottlesprocket::send_command;
use bottlesprocket::open_port;
use bottlesprocket::CM17ACommand;

fn main() {
    let matches = clap::App::new("bottlesprocket")
        .about("bottlesprocket is a CM17A 'Firecracker' serial port command line tool.")
        .version("0.0.1")
        .author("Scott Jackson <scottmmjackson@gmail.com>")
        .arg(clap::Arg::with_name("house")
            .short("h")
            .long("house")
            .value_name("HOUSE")
            .required(true)
            .help("House Code to use")
            .possible_values(&[
                "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P"
            ])
        )
        .arg(clap::Arg::with_name("device")
            .short("d")
            .long("device")
            .value_name("DEVICE")
            .help("Device index to use")
            .long_help("Required for ON and OFF commands")
            .possible_values(&[
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16"
            ])
        )
        .arg(clap::Arg::with_name("serial")
            .short("s")
            .long("serial")
            .value_name("SERIAL_PORT")
            .help("Serial port where the CM17A is connected")
            .default_value("/dev/ttyS0")
        )
        .arg(clap::Arg::with_name("command")
            .short("c")
            .long("command")
            .value_name("COMMAND")
            .required(true)
            .help("Command to send to CM17A")
            .possible_values(&[
                "OFF", "ON", "DIM", "BRIGHT",
                "ALL_ON", "ALL_OFF", "LAMPS_ON", "LAMPS_OFF"
            ])
            .long_help("There are eight valid commands that can be sent:\n\
            OFF - Turn the identified device off\n\
            ON - Turn the identified device on\n\
            DIM - Send a DIM command. See Dimming below.\n\
            BRIGHT - Send a BRIGHT command. See Dimming below.\n\
            ALL_ON - Turn on all devices at the specified house code\n\
            ALL_OFF - Turn off all devices at the specified house code\n\
            LAMPS_ON - Turn on all lamps at the specified house code\n\
            LAMPS_OFF - Turn off all lamps at the specified house code\n\
            \n\
            Dimming: To dim devices, you must send ON to the target device and then\n\
                DIM or BRIGHT to the target house code one time for every 5% increase\n\
                or decrease in brightness you wish\n")
            .requires_ifs(&[
                ("OFF", "device"),
                ("ON", "device"),
            ])
        )
        .get_matches();
    env_logger::init().unwrap();
    let command: CM17ACommand = make_command(
        match matches.value_of("house") {
            Some("A") => HouseCode::A,
            Some("B") => HouseCode::B,
            Some("C") => HouseCode::C,
            Some("D") => HouseCode::D,
            Some("E") => HouseCode::E,
            Some("F") => HouseCode::F,
            Some("G") => HouseCode::G,
            Some("H") => HouseCode::H,
            Some("I") => HouseCode::I,
            Some("J") => HouseCode::J,
            Some("K") => HouseCode::K,
            Some("L") => HouseCode::L,
            Some("M") => HouseCode::M,
            Some("N") => HouseCode::N,
            Some("O") => HouseCode::O,
            Some("P") => HouseCode::P,
            _ => panic!(),
        },
        match matches.value_of("device") {
            Some("1") => Some(Device::Device1),
            Some("2") => Some(Device::Device2),
            Some("3") => Some(Device::Device3),
            Some("4") => Some(Device::Device4),
            Some("5") => Some(Device::Device5),
            Some("6") => Some(Device::Device6),
            Some("7") => Some(Device::Device7),
            Some("8") => Some(Device::Device8),
            Some("9") => Some(Device::Device9),
            Some("10") => Some(Device::Device10),
            Some("11") => Some(Device::Device11),
            Some("12") => Some(Device::Device12),
            Some("13") => Some(Device::Device13),
            Some("14") => Some(Device::Device14),
            Some("15") => Some(Device::Device15),
            Some("16") => Some(Device::Device16),
            _ => None,
        },
        match matches.value_of("command") {
            Some("OFF") => Command::Off,
            Some("ON") => Command::On,
            Some("DIM") => Command::Dim,
            Some("BRIGHT") => Command::Bright,
            Some("ALL_ON") => Command::AllOn,
            Some("ALL_OFF") => Command::AllOff,
            Some("LAMPS_ON") => Command::LampsOn,
            Some("LAMPS_OFF") => Command::LampsOff,
            _ => panic!(),
        },
    );
    debug!(
        "Command: {:X} {:X} {:X} {:X} {:X}",
        command[0],
        command[1],
        command[2],
        command[3],
        command[4]
    );
    let portname = std::ffi::CString::new(
        matches.value_of("serial").unwrap()
    ).unwrap();
    let port = open_port(portname)
        .unwrap_or_else(|e| {
            println!("Can't open port: {}", e);
            std::process::exit(1)
        });
    send_command(command, port)
        .unwrap_or_else(|e| {
            println!("Error sending command: {}", e);
            std::process::exit(1)
        });
    println!("Command was successfully sent to CM17A");
}