use external_csgo_cheat::memory::Memory;
use winapi::shared::minwindef::LPVOID;

fn main() {
    let app = "csgo.exe";
    let mem = Memory::new(app);

    let local_player_offset = 0xA46B9C;
    let health = 0x230;

    let base = mem.get_module_adress("server.dll");

    loop {
        let local_player_pointer: usize = mem.read(base + local_player_offset);
        let local_player: usize = mem.read(local_player_pointer + health);
        unsafe { mem.write(local_player_pointer + health, 124 as LPVOID) };
        println!("{}", local_player);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
