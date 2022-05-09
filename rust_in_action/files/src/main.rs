use rand::Rng;

fn one_in(n: u32) -> bool {
    rand::thread_rng().gen_weighted_bool(n)
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        // make a copy here, as .append() will shrink the input
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        // ensures sufficient space to fit the incoming data and minimizes allocations when data is inserted byte-by-byte
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        if one_in(10) {
            let err_msg = String::from("Cannot read file");
            return Err(err_msg);
        }
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    if one_in(10) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    if one_in(10) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = Vec::new();
    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    // convert Vec<u8> to String
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
}
