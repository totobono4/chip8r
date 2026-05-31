use rodio::{MixerDeviceSink, Player};
use rodio::source::{SineWave, Source};

const WAVE_FREQUENCY: f32 = 440.0;
const WAVE_AMPLIFICATION: f32 = 0.20;

pub struct Audio {
    _handle: MixerDeviceSink,
    player: Player,
}

impl Audio {
    pub fn new() -> Self {
        let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = Player::connect_new(&handle.mixer());
        let source = SineWave::new(WAVE_FREQUENCY).amplify(WAVE_AMPLIFICATION).repeat_infinite();
        player.append(source.clone());
        player.pause();

        Self {
            _handle: handle,
            player: player,
        }
    }

    pub fn play_sound(&mut self) {
        self.player.play();
    }

    pub fn stop_sound(&mut self) {
        self.player.pause();   
    }
}
