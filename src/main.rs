use external_csgo_cheat::memory::Memory;

fn main() {
    let a = "csgo.exe";
    let mem = Memory::new(&a);
    
    let local_player_offset = 0xDEA964;
    let flags_offset = 0x104;
    let force_jump_offset = 0x52BBC7C;

    let client = mem.get_module_adress("client.dll");

    loop {
        let local_player = mem.read(client+0xDEA964+0x100);
        println!("{}", local_player);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
}
