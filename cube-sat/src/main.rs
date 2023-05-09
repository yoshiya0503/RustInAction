#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String
}

struct GroundStation;

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat{id: sat_id}
    }

    fn send(&self, mailbox: &mut MailBox, msg: Message) {
        mailbox.post(msg)
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl MailBox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg)
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message>{
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
enum StatusMessage {
    OK,
}

fn check_status(sat: CubeSat) -> StatusMessage {
    println!("check .....{:?}", sat.id);
    StatusMessage::OK
}


fn main() {
    let mut mail = MailBox{ messages: vec![]};
    let base = GroundStation{};
    let sat_ids = vec![1, 2, 3, 4];

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message{ to: sat_id, content: String::from("hi")};
        base.send(&mut mail, msg);
    }

    let sat_ids = vec![1, 2, 3, 4];

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}", msg)
    }


    /*
    let sat_a = CubeSat{id: 0};
    let sat_b = CubeSat{id: 1};
    let sat_c = CubeSat{id: 2};

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?} b: {:?} c: {:?}", a_status, b_status, c_status);

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?} b: {:?} c: {:?}", a_status, b_status, c_status);
    */
}
