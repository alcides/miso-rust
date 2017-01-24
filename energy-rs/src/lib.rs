/*#[cfg(any(target_os="macos", not(unix)))]
pub mod energy {
    
    pub struct EnergyRecording {
    }
    
    impl EnergyRecording {
        pub fn stop_recording(self) -> Option<u64> {
            None
        }
    }
    
    pub fn start_recording() -> EnergyRecording {
        EnergyRecording {}
    }
}


#[cfg(all(not(target_os="macos"), unix))]*/
pub mod energy {
    
    use std::sync::{Arc, Mutex, mpsc};
    use std::{thread, time};
    use std::fs::File;
    use std::os::unix::io::{AsRawFd, RawFd};
    use std::mem;
    
    extern crate nix;
    use energy::nix::sys::uio::pread;

    pub struct EnergyRecording {
        energy_total: Arc<Mutex<u64>>,
        thread: thread::JoinHandle<()>,
        tx: mpsc::Sender<()>
    }
    
    impl EnergyRecording {
        pub fn stop_recording(self) -> Option<u64> {
            let _ = self.tx.send(());
            let _ = self.thread.join();
            let r = self.energy_total.lock().unwrap();
            Some(*r)
        }
    }
    
    fn open_msr() -> RawFd {
        let k = File::open("/dev/cpu/0/msr");
        match k {
            Ok(k) => {
                return k.as_raw_fd();
            },
            _ => {
                panic!("No MSR");
            }
        }
    }
    
    fn read_msr(raw: RawFd, r : i64) -> u64 {
        
        let mut buf = [0u8;8];
        match pread(raw, &mut buf, r) {
            Ok(_)  => {},
            _ => {
                panic!("Error in pread");
            }
        }
        let r = unsafe { mem::transmute::<[u8; 8], u64>(buf) };
        println!("Found: {}", r);
        r
    }
    
    pub fn start_recording() -> EnergyRecording {        
        
        let interval = time::Duration::from_millis(100);        
        
        let e = Arc::new(Mutex::new(0));
        
        let ie = e.clone();
        
        let (tx, rx) = mpsc::channel();
        
        let t = thread::spawn(move || {
            
            let file = open_msr();
            let mut previous_energy = read_msr(file, 0x639);
            
            loop {
                thread::sleep(interval);
                
                let current_energy = read_msr(file, 0x639);
                let diff = current_energy - previous_energy;
                previous_energy = current_energy;
                
                let mut energy_rec = ie.lock().unwrap();
                *energy_rec += diff;
                if rx.try_recv().is_ok() {
                    break;
                }
            }
        });
        
        EnergyRecording {
            energy_total: e,
            thread: t,
            tx: tx
        }
    }

}