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
    Off = 0x00,
    On = 0x20,
    Dim = 0x98,
    Bright = 0x88,
    AllOff = 0x80,
    AllOn = 0x91,
    LampsOff = 0x84,
    LampsOn = 0x94,
}

pub type CM17ACommand = [u8; 5];

pub fn make_command(house: HouseCode, device: Device, command: Command) -> CM17ACommand {
    let dev = device as u16;
    let device_high = (dev >> 8) as u8;
    let device_low = dev as u8;
    [
        // http://kbase.x10.com/wiki/CM17A_Protocol
        0xd5, // HEADER
        0xaa, // HEADER
        house as u8 | device_high,
        device_low | command as u8,
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
        return Err(io::Error::last_os_error())
    }
    standby_wait();
    Ok(())
}

fn logical1(fd: libc::c_int) -> io::Result<()> {
    let out = libc::TIOCM_RTS;
    let res = unsafe { libc::ioctl(fd, libc::TIOCMBIS, &out )};
    if res == -1 {
        return Err(io::Error::last_os_error())
    }
    standby_wait();
    Ok(())
}

fn logical0(fd: libc::c_int) -> io::Result<()> {
    let out = libc::TIOCM_DTR;
    let res = unsafe { libc::ioctl(fd, libc::TIOCMBIS, &out )};
    if res == -1 {
        return Err(io::Error::last_os_error())
    }
    standby_wait();
    Ok(())
}

fn reset(fd: libc::c_int) -> io::Result<()> {
    let out: libc::c_int = 0x000;
    let res = unsafe { libc::ioctl(fd, libc::TIOCMBIS, &out )};
    if res == -1 {
        return Err(io::Error::last_os_error())
    }
    standby_wait();
    Ok(())
}

fn standby_wait() {
    let standby_delay = time::Duration::new(0, 500000);
    wait(standby_delay);
}

fn wait(dur: time::Duration) {
    thread::sleep(dur);
}

pub fn send_command(cmd: CM17ACommand, fd: libc::c_int) -> io::Result<()> {
    reset(fd)?;
    standby(fd)?;
    for byte in cmd.into_iter() {
        for shift in 1..8 as u8 {
            match (byte << shift) & 0x1 {
                1 => logical1(fd)?,
                0 => logical0(fd)?,
                x => panic!(format!("The developer did something wrong. Byte {} shifted {} & 0x1 is {}", byte, shift, x))
            }
            standby(fd)?;
        }
    }
    reset(fd)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn test_make_command() {
        let expected1: CM17ACommand = [0xd5, 0xaa, 0x94, 0x00, 0xad];
        let actual1 = make_command(HouseCode::F, Device::Device9, Command::Off);
        assert_eq!(actual1, expected1)
    }

    #[test]
    #[ignore] // requires root and a serial port with a firecracker
    fn test_send_command() {
        let command: CM17ACommand = [0xd5, 0xaa, 0x94, 0x00, 0xad];
        let port = open_port(
            ffi::CString::new("/dev/ttyS0").unwrap()
        ).unwrap();
        send_command(command, port).unwrap()
    }
}
