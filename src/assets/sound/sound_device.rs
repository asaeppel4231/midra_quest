use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host};

pub struct SoundDevice {
    host: Host,
    device: Device,
}

/*pub fn test() {
    let snd: SoundDevice;

    snd.host = cpal::default_host();
}
*/
