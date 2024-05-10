use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::env::{self, args};
use imap_search::connect::imapconnections;
use imap_search::types::{ImapDumper, Search};
use std::fs::OpenOptions;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use imap_search::art::art;

#[derive(Debug)]
struct args_include {
    filename: String, 
    numero_threads: i32,
    search: String
    // a saida por default vai ser Ok(valid.txt,erro.txt,loginfailed.txt,search_and_valid.txt)
}


impl args_include{
    fn new(filename: String , numero_threads: i32 , search: String) -> args_include{ 
     
        let ok_lex = PathBuf::from_str(filename.as_str()).unwrap().exists();
        if ok_lex{
            args_include{
                filename, numero_threads, search
            }
        }   
        else {
            eprintln!("Arquivo não existe ");
            exit(1);
        }

    }
}




fn open_salve_file(filename: &str , conteudo: String){
    let mut open_and_salve = OpenOptions::new().create(true).append(true).write(true).open(filename).unwrap();
    open_and_salve.write(format!("{}\n", conteudo).as_bytes());
}





fn main() {


    #[cfg(target_os = "windows")]
    std::process::Command::new("cls").status();
    #[cfg(target_os = "linux")]
    std::process::Command::new("clear").status();
    art();
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Argumentos incompleto");
        eprintln!("./bin nome_do_arquivo_que_deseja_abri numero_de_threads search_o_que_voce_quer_procurar");
        
        exit(2);
    }    
    let ok = args_include::new(args[1].clone(), args[2].clone().parse::<i32>().expect("Erro ao converter o numero de threads"), args[3].to_owned());

    // default é usando o pipe ok
     let _thread_pool = rayon::ThreadPoolBuilder::new() 
        .num_threads( ok.numero_threads as usize)
        .build_global()
        .unwrap();
    
    let openfile = fs::read_to_string(ok.filename) .expect("Erroa oa abrir o arquivo");
    let lines: Vec<String> = openfile.lines().map(|s| s.to_string()).collect();
    let search = ok.search;
   lines.into_par_iter().for_each(|file_line| {
        
       let split_n: Vec<&str> = file_line.split("|").collect();

        if split_n.len() == 4 {
            let search = search.clone().to_string();
            let host_imap = split_n[0].to_string();
            let port = split_n[1].parse::<i32>().unwrap_or(993);
            let username  = split_n[2].to_string();
            let password = split_n[3].to_string();

            let format = format!("{}|{}|{}|{}",&host_imap, port, &username, &password);
            match imapconnections::consult_search(host_imap, port as u16, username, password, search.clone()){
                Ok(login_ok) => {
                    match login_ok {
                        Search::Encontrou(value) =>{
                            open_salve_file("search_valid.txt", format!("{} -> {} ", format, search ));
                        }
                        Search::NotEncontrou => {
                            open_salve_file("valid_not_search.txt", format);
                        }

                        _ => {
                         open_salve_file("PoucasMensagens_valid.txt", format);   
                        }
                    }
                }
                Err(dumper) => {
                    match dumper {
                        ImapDumper::Authenticationfailed(_) => {
                            open_salve_file("loginfailed.txt", format);
                        }

                        ImapDumper::ErrorBuilder(_) => {
                            open_salve_file("error.txt", format);
                        }

                        _ => {
                            // conection ok(Porem eu ja salvo usando o enum 'Search')
                        }
                    }
                    
                }
            }
        }
     

    });
   
  
}
