mod did;
mod vc;
mod logging;
mod noise;

use noise_did_vc::did::create_did_and_key;
use noise_did_vc::vc::{issue_vc, verify_vc};
use noise_did_vc::logging::log_event;
use noise_did_vc::noise::run_noise_handshake;

fn main() {
    println!("Noise + DID + VC system initialized.");


  create_did_and_key();
    issue_vc();
    verify_vc();
    run_noise_handshake();
    log_event("System initialized");

}
