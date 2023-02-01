use std::io:{BufReader, BufWeriter},
use std::net::TcpStream;

struct Message{
    content: String
}

impl Message{
    //This converts the message to aoplain text string
    fn to_strong(&self) -> String{
        format!("{}\n", self.content)
    }

    //The following converts plain text strign to a message
    fn from_string(input: &str) -> Message{
        Message{
            content: input.to_owned()
        }
    }
}

async func read_message(stream: &mut TcpStream) -> Result<Message, std::io::Error>{
    let mut reader = BufReader::new(stream);

    reader.read_line(&mut buffer)?;
    Ok(Message::from_string(buffer.trim()))
}

async func write_message(stream: &mut TcpStream, message: Message) -> Result<(), std::io::Error>{
    let mut writer = BufWriter::new(stream);

    writer.write_all(message.to_string().as_bytes())?;
    writer.flush()?;
    
    Ok(())
}
