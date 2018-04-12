#[cfg(windows)] extern crate winapi;
use std::io::Error;
#[cfg(windows)]
use winapi::um::tlhelp32::{Process32First, Process32Next, LPPROCESSENTRY32, CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32};
#[cfg(windows)]
use winapi::um::winnt::HANDLE;
#[cfg(windows)]
use winapi::um::handleapi::INVALID_HANDLE_VALUE;

struct ProcessInformation {
    pid: u32,
    name: String,
}

impl ProcessInformation {
    fn new(_pid: u32, _name: String) -> ProcessInformation {
        ProcessInformation { pid : _pid, name : _name }
    }
}

#[cfg(windows)]
struct ProcessInformationIterator {
    process_information: ProcessInformation,
    index: usize,
    process_snapshot: HANDLE,
    process_entry: PROCESSENTRY32,

}

#[cfg(windows)]
impl ProcessInformationIterator {
    fn new() -> ProcessInformationIterator {
        let h_process_snapshot: HANDLE = unsafe {
            CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS ,0)
        };
        if h_process_snapshot == INVALID_HANDLE_VALUE {
            panic!("Invalid handle value");
        }
        println!("Got process snapshot handle, moving on...");
        unsafe {
            let mut pe: PROCESSENTRY32 = std::mem::zeroed();
            let a = std::mem::size_of::<PROCESSENTRY32>();
            let lppe:LPPROCESSENTRY32 = & mut pe;
            (*lppe).dwSize = a as u32;
            let res = Process32First(h_process_snapshot, lppe);
            if res == 0 {
                panic!("Can't get process list");
            }
            let pid : u32 = (* lppe).th32ProcessID;
            let process_name:String  = (*lppe).szExeFile.into_iter().map(|c| { *c as u8 as char }).collect();
            ProcessInformationIterator{ process_information: ProcessInformation::new(pid, process_name), index: 0, process_snapshot: h_process_snapshot, process_entry: pe }

        }

    }
}

#[cfg(windows)]
impl Iterator for ProcessInformationIterator {
    type Item = ProcessInformation;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.index = self.index + 1;
        if self.index == 1 {
            return Some(ProcessInformation::new(self.process_information.pid, self.process_information.name.clone()));
        }
        unsafe {
            let mut pe = self.process_entry;
            let lppe = & mut pe;
            (*lppe).szExeFile = std::mem::zeroed();
            let res = Process32Next(self.process_snapshot, lppe);
            if res != 1 {
                None
            } else {

                let pid: u32 = (*lppe).th32ProcessID;
                let process_name: String = (*lppe).szExeFile.into_iter().map(|c| { *c as u8 as char }).collect();
                Some(ProcessInformation::new(pid, process_name))
            }
        }
    }
}

#[cfg(windows)]
fn print_message() -> Result<i32, Error> {

    // let mut pi = ProcessInformationIterator::new().into_iter();
    for process_information in ProcessInformationIterator::new() {
        println!("{}: {}", process_information.pid, process_information.name);
    }


    Ok(0)

}
#[cfg(not(windows))]
fn print_message() -> Result<(), Error> {
    println!("only works on Windows");
    Ok(())
}
fn main() {
    print_message().unwrap();
}