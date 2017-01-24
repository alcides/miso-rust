#[cfg(any(target_os="macos", not(unix)))]
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


#[cfg(all(not(target_os="macos"), unix))]
pub mod energy {
    extern crate x86;
    use energy::x86::shared::msr::{MSR_PKG_ENERGY_STATUS, rdmsr};
    
    use std::sync::{Arc, Mutex, mpsc};
    use std::{thread, time};

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
    
    pub fn start_recording() -> EnergyRecording {        
        
        let interval = time::Duration::from_millis(100);        
        
        let e = Arc::new(Mutex::new(0));
        
        let ie = e.clone();
        
        let (tx, rx) = mpsc::channel();
        
        let t = thread::spawn(move || {
            let mut previous_energy = unsafe { rdmsr(MSR_PKG_ENERGY_STATUS) };
            
            loop {
                thread::sleep(interval);
                
                let current_energy = unsafe { rdmsr(MSR_PKG_ENERGY_STATUS) };
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