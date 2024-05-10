pub mod imapconnections {
    use imap::{types, Client};
    extern crate imap;
    extern crate native_tls;
    use crate::types::{ImapDumper, Search};
    use native_tls::TlsStream;
    use std::net::TcpStream;
    use imap::Session;
    fn BuildTls<F: AsRef<str>>(
        host: F,
        port: u16,
    ) -> Result<Client<TlsStream<TcpStream>>, ImapDumper> {
        let tls = native_tls::TlsConnector::builder().build().unwrap();
        let client = imap::connect((host.as_ref(), port), host.as_ref(), &tls);
        match client {
            Ok(native) => Ok(native),
            Err(_) => Err(ImapDumper::ErrorBuilder("Error ao buildar".to_string())),
        }
    }

    fn login_username_password(
        tlsConnect: Client<TlsStream<TcpStream>>,
        username: String,
        password: String,
    ) -> Result<Session<TlsStream<TcpStream>>, ImapDumper>{

        let tls = tlsConnect.login(username, password);
        match  tls
         {
                Ok(connect) => Ok(connect),
                Err(_) => Err(ImapDumper::Authenticationfailed("Login incorreto".to_string()))
        }

    }

    fn session_search<O: AsRef<str>>(mut senssion: Session<TlsStream<TcpStream>> , search: O ) -> Search{
        let MailBox_Session = senssion.select("INBOX").unwrap();
        let count_number_msg = MailBox_Session.exists ; // numeros de mensagens
        if count_number_msg < 3{
            return Search::PoucasMensagens;
        }
        let messages = senssion.fetch("1:*", "RFC822").unwrap();
        for message in messages.iter() {
            let body = message.body().unwrap_or("oxfe".as_bytes());
            let body_str = std::str::from_utf8(body)
                .unwrap_or("Error ao ler a utf-8");
        
            // pesquisar o search 

            if body_str.to_ascii_lowercase().contains(&search.as_ref().to_lowercase()) {
                return  Search::Encontrou(search.as_ref().to_string());
            }
            
        }
        Search::NotEncontrou
    }

    pub fn consult_search<T: AsRef<str>>(host: T , port: u16 , username: String , password: String, search: T) -> Result<Search, ImapDumper>{
        let build = BuildTls(host, port)?;
        let login_return_session = login_username_password(build, username, password)?;
        let Session = session_search(login_return_session, search.as_ref());

        Ok(Session)

    }

}
