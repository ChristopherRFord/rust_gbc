use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct Cart
{
    file_name  : String,
    rom_size   : u32,
    rom_data   : Vec<u8>
}

impl Cart
{
    pub fn new() -> Self
    {
        Cart
        {
            file_name  : String::new(),
            rom_size   : 0,
            rom_data   : Vec::new()
        }
    }

    pub fn load_cart(&mut self, location : &str) -> io::Result<()>
    {
        let path = Path::new(location);
        let mut file = match File::open(&path)
        {
            Ok(file) => 
            {
                println!("Opened: {}", location);
                file
            },
            Err(e) =>
            {
                eprintln!("Failed to open file '{}': {}", location, e);
                return Err(e);
            }
        };

        self.file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("unknown").to_string();
        self.rom_size  = file.metadata()?.len() as u32;
        self.rom_data.resize(self.rom_size as usize, 0);
        file.read_exact(&mut self.rom_data)?;

        self.print_info();

        Ok(())
    }


    pub fn logo(&self) -> &[u8]
    {
        &self.rom_data[0x104..=0x133]
    }

    pub fn title(&self) -> &[u8]
    {
        &self.rom_data[0x134..0x143]
    }

    pub fn title_str(&self) -> &str
    {
        let raw = self.title();
        std::str::from_utf8(raw).unwrap_or("Invalid UTF-8")
    }

    pub fn cgb(&self) -> bool
    {
        let raw = self.rom_data[0x143];
        match raw
        {
            0x80 | 0xC0 => true,
            _ => false,
        }
    }

    pub fn lic_code(&self) -> u8
    {
        self.rom_data[0x144]
    }

    pub fn lic_code_str(&self) -> &str
    {
        let raw = &self.rom_data[0x144..=0x145];
        let code: &str = std::str::from_utf8(raw).unwrap_or("Invalid UTF-8");

        match code
        {
            "00" => "None",
            "01" => "Nintendo R&D1",
            "08" => "Capcom",
            "13" => "Electronic Arts",
            "18" => "Hudson Soft",
            "19" => "b-ai",
            "20" => "kss",
            "22" => "pow",
            "24" => "PCM Complete",
            "25" => "san-x",
            "28" => "Kemco Japan",
            "29" => "seta",
            "30" => "Viacom",
            "31" => "Nintendo",
            "32" => "Bandai",
            "33" => "Ocean/Acclaim",
            "34" => "Konami",
            "35" => "Hector",
            "37" => "Taito",
            "38" => "Hudson",
            "39" => "Banpresto",
            "41" => "Ubi Soft",
            "42" => "Atlus",
            "44" => "Malibu",
            "46" => "angel",
            "47" => "Bullet-Proof",
            "49" => "irem",
            "50" => "Absolute",
            "51" => "Acclaim",
            "52" => "Activision",
            "53" => "American sammy",
            "54" => "Konami",
            "55" => "Hi tech entertainment",
            "56" => "LJN",
            "57" => "Matchbox",
            "58" => "Mattel",
            "59" => "Milton Bradley",
            "60" => "Titus",
            "61" => "Virgin",
            "64" => "LucasArts",
            "67" => "Ocean",
            "69" => "Electronic Arts",
            "70" => "Infogrames",
            "71" => "Interplay",
            "72" => "Broderbund",
            "73" => "sculptured",
            "75" => "sci",
            "78" => "THQ",
            "79" => "Accolade",
            "80" => "misawa",
            "83" => "lozc",
            "86" => "Tokuma Shoten Intermedia",
            "87" => "Tsukuda Original",
            "91" => "Chunsoft",
            "92" => "Video system",
            "93" => "Ocean/Acclaim",
            "95" => "Varie",
            "96" => "Yonezawa/s’pal",
            "97" => "Kaneko",
            "99" => "Pack in soft",
            "A4" => "Konami (Yu-Gi-Oh!)",
            _ => "Other",
        }
    }

    pub fn sgb(&self) -> bool
    {
        let raw = self.rom_data[0x146];
        match raw
        {
            0x0 => false,
            0x3 => true,
            _ => panic!("Unknown SGB value: {}", raw),
        }
    }

    pub fn rom_type(&self) -> u8
    {
        self.rom_data[0x147]
    }

    pub fn rom_type_str(&self) -> &str
    {
        let raw = self.rom_type();
        
        match raw
        {
            0  => "ROM ONLY",
            1  => "MBC1",
            2  => "MBC1+RAM",
            3  => "MBC1+RAM+BATTERY",
            4  => "0x04 ???",
            5  => "MBC2",
            6  => "MBC2+BATTERY",
            7  => "0x07 ???",
            8  => "ROM+RAM 1",
            9  => "ROM+RAM+BATTERY 1",
            10 =>"0x0A ???",
            11 =>"MMM01",
            12 =>"MMM01+RAM",
            13 =>"MMM01+RAM+BATTERY",
            14 => "0x0E ???",
            15 => "MBC3+TIMER+BATTERY",
            16 => "MBC3+TIMER+RAM+BATTERY 2",
            17 => "MBC3+RAM 2",
            18 => "MBC3+RAM+BATTERY 2",
            19 => "0x14 ???",
            20 => "0x15 ???",
            21 => "0x16 ???",
            22 => "0x17 ???",
            23 => "0x18 ???",
            24 => "MBC5",
            25 => "MBC5+RAM",
            26 => "MBC5+RAM+BATTERY",
            27 => "MBC5+RUMBLE",
            28 => "MBC5+RUMBLE+RAM",
            29 => "MBC5+RUMBLE+RAM+BATTERY",
            30 => "0x1F ???", 
            31 => "MBC6",
            32 => "0x21 ???",
            33 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
            _ => panic!("Unknown ram type: {}", raw),
        }
    }

    pub fn rom_size(&self) -> u8
    {
        self.rom_data[0x148]
    }

    pub fn rom_size_str(&self) -> &str
    {
        let raw = self.rom_size();

        match raw
        {
            0x0 => "32 Kb",
            0x1 => "64 Kb",
            0x2 => "128 Kb",
            0x3 => "256 Kb",
            0x4 => "512 Kb",
            0x5 => "2 Mb",
            0x6 => "4 Mb",
            0x7 => "8 Mb",
            _ => panic!("Unknown rom size: {}", raw),
        }
    }

    pub fn ram_size(&self) -> u8
    {
        self.rom_data[0x149]
    }
    pub fn ram_size_str(&self) -> &str
    {
        let raw = self.ram_size();

        match raw
        {
            0x0 => "None",
            0x1 => "2 Kb",
            0x2 => "8 Kb",
            0x3 => "32 Kb",
            0x4 => "128 Kb",
            _ => panic!("Unknown ram size: {}", raw),
        }
    }

    pub fn destination_code(&self) -> bool
    {
        let raw = self.rom_data[0x14A];
        match raw
        {
            0x0 => true,
            0x1 => false,
            _ => panic!("Unknown destination code: {}", raw),
        }
    }

    pub fn header_checksum(&self) -> u8
    {
        self.rom_data[0x14D]
    }

    pub fn verify_header_checksum(&self) -> bool
    {
        let mut checksum: u8 = 0;
        for b in &self.rom_data[0x134..=0x14C]
        {
            checksum = checksum.wrapping_sub(*b).wrapping_sub(1);
        }

        checksum == self.header_checksum()
    }

    pub fn print_info(&self)
    {
        println!("Cartridge Loaded");
        println!("\tTitle     : {}",        self.title_str());
        println!("\tLIC Code  : {:02X} {}", self.lic_code(), self.lic_code_str());
        println!("\tType      : {:02X} {}", self.rom_type(), self.rom_type_str());
        println!("\tROM Size  : {:02X} {}", self.rom_size(), self.rom_size_str());
        println!("\tRAM Size  : {:02X} {}", self.ram_size(), self.ram_size_str());
        println!("\tChecksum  : {:02X} ({})", self.header_checksum(), self.verify_header_checksum());
    }
}