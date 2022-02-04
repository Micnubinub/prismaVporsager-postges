//extern crate imap;
//extern crate native_tls;
//extern crate openssl;
//
//use std::collections::HashSet;
//use std::net::TcpStream;
//use std::time::SystemTime;
//
//use imap::types::Uid;
//use rustc_serialize::base64::FromBase64;
//
////use crate::PgConnection;
////
////use super::diesel_helper::insert_resident_user_data;
//
//use self::imap::Session;
//use self::imap::types::Fetch;
//use self::imap::types::Seq;
//use self::imap::types::ZeroCopy;
//use self::native_tls::TlsStream;
//
//pub fn refresh_resident_data(conn: &PgConnection) -> Result<String, String> {
//    let start = SystemTime::now();
//    println!("started ");
//    let domain = "imap.gmail.com";
//    let tls = native_tls::TlsConnector::builder().build().unwrap();
//
//    // we pass in the domain twice to check that the server's TLS
//    // certificate is valid for the domain we're connecting to.
//    let client = imap::connect((domain, 993), domain, &tls).unwrap();
//
//    // the client we have here is unauthenticated.
//    // to do anything useful with the e-mails, we need to log in
//    let mut imap_session = match client
//        .login("wipitapp@gmail.com", "ITofficer1_2mihaven")
//        .map_err(|e| e.0) {
//        Ok(sesh) => sesh,
//        Err(err) => {
//            return Err(err.to_string());
//        }
//    };
//
//    // we want to fetch the first email in the INBOX mailbox
////    match imap_session.select("INBOX") {
//    match imap_session.select("[Gmail]/All Mail") {
//        Ok(res) => {}
//        Err(s) => {
//            return Err(s.to_string());
//        }
//    }
//
//    // fetch message number 1 in this mailbox, along with its RFC822 field.
//    // RFC 822 dictates the format of the body of e-mails
//    let messages = imap_session.search("FROM studentliving@mihaven.com.au");
//    let message_list = match messages {
//        Ok(t) => {
//            t
//        }
//        Err(e) => {
//            println!("Failed to get messages");
//            HashSet::new()
//        }
//    };
//
//    let mut nums: Vec<u32> = Vec::new();
//    // Todo delete parsed emails
//    message_list.iter().for_each(|item| {
//        &nums.push(item.clone());
//        match parse_message(&mut imap_session, *item) {
//            Ok(s) => {
//                match upload_data_to_db(s.email, conn) {
//                    Ok(res) => {}
//                    Err(err) => {
//                        err.to_string();
//                    }
//                }
//            }
//            Err(s) => {
//                println!("ERR: {}", s)
//            }
//        }
//    });
//
//    let seq = format!("{:?}", nums).replace(" ", "").replace("[", "").replace("]", "");
//    let seq_clone = seq.clone();
//    match imap_session.mv(seq, "[Gmail]/Trash") {
//        Ok(_) => {
//            println!("moved > {}", seq_clone);
//        }
//        Err(err) => {
//            println!("move failed: {}", err.to_string())
//        }
//    }
//
//    match imap_session.select("[Gmail]/Trash") {
//        Ok(res) => {}
//        Err(s) => {
//            return Err(s.to_string());
//        }
//    }
//
//    let a = "STORE 1:* +FLAGS \\Deleted";
//    println!("{}", a.clone());
//    match imap_session.run_command_and_read_response(a) {
//        Ok(resp) => {
//            println!("flagged: {:?}", resp);
//            match imap_session.expunge() {
//                Ok(res1) => {
//                    println!("deleted folder expunge: {:?}", res1);
//                }
//                Err(e) => {
//                    let _ = imap_session.logout();
//                    return Err(e.to_string());
//                }
//            }
//        }
//        Err(e) => {
//            println!("{}", &e.to_string());
//            let _ = imap_session.logout();
//            return Err(e.to_string());
//        }
//    };
//
//    let _ = imap_session.logout();
//    Ok("done refreshing emails".to_string())
//}
//
//pub struct Email {
//    email: String,
//    uid: Uid,
//}
//
//fn parse_message(imap_session: &mut Session<TlsStream<TcpStream>>, seq: u32) -> Result<Email, String> {
//    let msg = seq.to_string();
//    let messages: ZeroCopy<Vec<Fetch>> = match imap_session.fetch(msg, "(BODY[TEXT] ENVELOPE UID)") {
//        Ok(m) => m,
//        Err(err) => {
//            return Err(format!("failed to parse messages {}", seq));
//        }
//    };
//
//    let message = if let Some(m) = messages.iter().next() {
//        m
//    } else {
//        return Err(format!("Nothing in message number {}", seq));
//    };
//
//    let envelope = match message.envelope() {
//        Some(parsed) => parsed,
//        None => return Err(format!("Invalid envelope {}", seq))
//    };
//
//    let body = match message.text() {
//        Some(bo) => bo,
//        None => {
//            return Err("email had no body".to_string());
//        }
//    };
//
//    let body = body.to_vec().from_base64();
//    let res = match body {
//        Ok(b) => {
//            let email = match String::from_utf8(b) {
//                Ok(res) => {
//                    if res.len() > 8 {
//                        (&res[3..(res.len() - 6)]).to_string()
//                    } else {
//                        return Err("email too short".to_string());
//                    }
//                }
//                Err(e) => {
//                    return Err(e.to_string());
//                }
//            };
//            Ok(Email {
//                email,
//                uid: message.uid.unwrap(),
//            })
//        }
//        Err(err) => {
//            println!("decode err");
//            Err(err.to_string())
//        }
//    };
//    res
//}
//
//fn upload_data_to_db(s: String, conn: &PgConnection) -> Result<String, String> {
//    //Todo
//    let split = s.split(", ").collect::<Vec<&str>>();
////    insert_resident_user_data(split, conn)
//    Ok(String::from(""))
//}
