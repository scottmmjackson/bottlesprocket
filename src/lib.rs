extern crate libc;

use std::{io, ffi, thread, time};

pub enum HouseCode {
    A = 0x06 << 4,
    B = 0x07 << 4,
    C = 0x04 << 4,
    D = 0x05 << 4,
    E = 0x08 << 4,
    F = 0x09 << 4,
    G = 0x0a << 4,
    H = 0x0b << 4,
    I = 0x0e << 4,
    J = 0x0f << 4,
    K = 0x0c << 4,
    L = 0x0d << 4,
    M = 0x00 << 4,
    N = 0x01 << 4,
    O = 0x02 << 4,
    P = 0x03 << 4,
}

pub enum Device {
    Device1 = 0x0000,
    Device2 = 0x0010,
    Device3 = 0x0008,
    Device4 = 0x0018,
    Device5 = 0x0040,
    Device6 = 0x0050,
    Device7 = 0x0048,
    Device8 = 0x0058,
    Device9 = 0x0400,
    Device10 = 0x0410,
    Device11 = 0x0408,
    Device12 = 0x0418,
    Device13 = 0x0440,
    Device14 = 0x0450,
    Device15 = 0x0448,
    Device16 = 0x0458,
}

pub enum Command {
    On = 0x00,
    Off = 0x20,
    Dim = 0x98,
    Bright = 0x88,
    AllOff = 0x80,
    AllOn = 0x91,
    LampsOff = 0x84,
    LampsOn = 0x94,
}

pub type CM17ACommand = [u8; 5];

pub fn make_command(house: HouseCode, device: Option<Device>, command: Command) -> CM17ACommand {
    if device.is_none() && match command {
        Command::On | Command::Off => true,
        _ => false
    } {
        panic!("Can't send a device-specific command to a non-existent device");
    }
    let dev = match command {
        Command::On | Command::Off => device.unwrap() as u16,
        _ => 0 as u16
    };
    let device_high = (dev >> 8) as u8;
    let device_low = dev as u8;
    let cmd = command as u8;
    [
        // http://kbase.x10.com/wiki/CM17A_Protocol
        0xd5, // HEADER
        0xaa, // HEADER
        house as u8 | device_high,
        device_low | cmd,
        0xad, // FOOTER
    ]
}

pub fn open_port(portname: ffi::CString) -> io::Result<libc::c_int> {
    let port = portname.as_ptr();
    match unsafe { libc::open(port, libc::O_RDONLY | libc::O_NONBLOCK) } {
        -1 => Err(io::Error::last_os_error()),
        x => Ok(x)
    }
}

fn standby(fd: libc::c_int) -> io::Result<()> {
    let out = libc::TIOCM_RTS | libc::TIOCM_DTR; // RTS 1 DTR 1 = standby signal
    let res = unsafe { libc::ioctl(fd, libc::TIOCMBIS, &out) };
    if res == -1 {
        return Err(io::Error::last_os_error());
    }
    standby_wait();
    Ok(())
}

fn logical1(fd: libc::c_int) -> io::Result<()> {
    let out = libc::TIOCM_DTR;
    let res = unsafe { libc::ioctl(fd, libc::TIOCMBIC, &out) };
    if res == -1 {
        return Err(io::Error::last_os_error());
    }
    standby_wait();
    Ok(())
}

fn logical0(fd: libc::c_int) -> io::Result<()> {
    let out = libc::TIOCM_RTS;
    let res = unsafe { libc::ioctl(fd, libc::TIOCMBIC, &out) };
    if res == -1 {
        return Err(io::Error::last_os_error());
    }
    standby_wait();
    Ok(())
}

fn reset(fd: libc::c_int) -> io::Result<()> {
    let out: libc::c_int = 0x000;
    let res = unsafe { libc::ioctl(fd, libc::TIOCMBIS, &out) };
    if res == -1 {
        return Err(io::Error::last_os_error());
    }
    standby_wait();
    Ok(())
}

fn standby_wait() {
    let standby_delay = time::Duration::new(0, 1_400_000);
    wait(standby_delay);
}

fn command_wait() {
    let command_delay = time::Duration::from_millis(350); // 350 ms
    wait(command_delay);
}

fn wait(dur: time::Duration) {
    thread::sleep(dur);
}

pub fn send_command(cmd: CM17ACommand, fd: libc::c_int) -> io::Result<()> {
    reset(fd)?;
    standby(fd)?;
    command_wait();
    for byte in cmd.into_iter() {
        for shift in 1..9 as u32 {
            match (byte.rotate_left(shift)) & 0x1 {
                1 => logical1(fd)?,
                0 => logical0(fd)?,
                x => panic!("The developer did something wrong. Byte {byte} shifted {shift} & 0x1 is {x}")
            }
            standby(fd)?;
        }
    }
    command_wait();
    reset(fd)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_make_command() {
        let expected1: CM17ACommand = [0xd5, 0xaa, 0x94, 0x00, 0xad];
        let actual1 = make_command(HouseCode::F, Some(Device::Device9), Command::On);
        assert_eq!(actual1, expected1);
        let expected2: CM17ACommand = [0xd5, 0xaa, 0xf0, 0x00, 0xad];
        let actual2 = make_command(HouseCode::J, Some(Device::Device1), Command::On);
        assert_eq!(actual2, expected2);
        let expected3: CM17ACommand = [0xd5, 0xaa, 0x20, 0x91, 0xad];
        let actual3 = make_command(HouseCode::O, None, Command::AllOn);
        assert_eq!(actual3, expected3);
        let expected4: CM17ACommand = [0xd5, 0xaa, 0x20, 0x84, 0xad];
        let actual4 = make_command(HouseCode::O, None, Command::LampsOff);
        assert_eq!(actual4, expected4);
        let expected5: CM17ACommand = [0xd5, 0xaa, 0x50, 0x98, 0xad];
        let actual5 = make_command(HouseCode::D, None, Command::Dim);
        assert_eq!(actual5, expected5)
    }

    #[test]
    #[ignore] // requires root and a serial port with a firecracker, `cargo test -- --ignored`
    fn test_send_command() {
        let command: CM17ACommand = [0xd5, 0xaa, 0x94, 0x00, 0xad];
        let port = open_port(
            ffi::CString::new("/dev/ttyS0").unwrap()
        ).unwrap(); // panic on unwrap: permission denied (if not root)
        send_command(command, port).unwrap() // panic on unwrap: I/O error (if no cm17a)
    }
}
